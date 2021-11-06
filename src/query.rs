use crate::common::spawn_process;
use gtk::gdk::gdk_pixbuf::Pixbuf;
use gtk::gdk::EventKey;
use gtk::prelude::*;
use gtk::{
    BoxBuilder, Entry, IconLookupFlags, IconTheme, Image, Label, ListBox, ListBoxRow, Orientation,
    ScrolledWindow, Viewport,
};
use std::process::exit;

pub fn init_query() -> Entry {
    let query_box = Entry::builder().name("findex-query").build();
    let mut desktop_entries = get_entries("/usr/share/applications");
    desktop_entries.extend(get_entries(
        shellexpand::tilde("~/.local/share/applications").as_ref(),
    ));

    query_box.style_context().add_class("findex-query");
    query_box.connect_changed({
        let de = desktop_entries.clone();
        move |qb| on_text_changed(qb, &de)
    });
    query_box.connect_key_press_event(on_key_press);

    query_box
}

fn on_key_press(qb: &Entry, ev: &EventKey) -> Inhibit {
    if ev.keyval().name().unwrap() == "Return" {
        let list = get_list_box(qb);
        let first_result = match list.row_at_index(0) {
            Some(res) => res,
            None => exit(0),
        };
        let container_w = &first_result.children()[0];
        let container = container_w.downcast_ref::<gtk::Box>().unwrap();
        let c_widget = &container.children()[2];
        let command = c_widget.downcast_ref::<Label>().unwrap();

        let mut splitted_cmd = shlex::split(&command.text().to_string()).unwrap();
        // strip parameters like %U %F etc
        for idx in 0..splitted_cmd.len() {
            if splitted_cmd[idx].starts_with('%') {
                splitted_cmd.remove(idx);
            }
        }

        spawn_process(&splitted_cmd);

        Inhibit(true)
    } else if ev.keyval().name().unwrap() == "Down" {
        let list_box = get_list_box(qb);
        let first_row = list_box.row_at_index(1);

        if let Some(first_row) = first_row {
            list_box.select_row(Some(&first_row));
            first_row.grab_focus();
        }

        Inhibit(true)
    } else {
        Inhibit(false)
    }
}

fn on_text_changed(qb: &Entry, apps: &[AppInfo]) {
    let text = regex::escape(&qb.text().to_lowercase());
    if text.is_empty() {
        let list_box = get_list_box(qb);
        clear_listbox(&list_box);
        return;
    }

    let regex = regex::Regex::new(&format!(r"^{}", text)).unwrap();

    let list_box = get_list_box(qb);

    clear_listbox(&list_box);

    for app in apps {
        if !regex.is_match(&app.name.to_lowercase()) {
            continue;
        }

        let icon = get_icon(&app.icon);

        let image = Image::builder().pixbuf(&icon).build();
        image.style_context().add_class("findex-result-icon");

        let name = Label::new(Some(&app.name));
        name.style_context().add_class("findex-result-app-name");

        let command = Label::new(Some(&app.exec));

        let container = BoxBuilder::new()
            .orientation(Orientation::Horizontal)
            .build();
        container.pack_start(&image, false, false, 0);
        container.pack_start(&name, false, false, 0);
        container.add(&command);

        let row = ListBoxRow::new();
        row.add(&container);
        row.style_context().add_class("findex-result-row");
        row.show_all();
        row.focus_child();

        list_box.add(&row);
    }

    if let Some(first_row) = list_box.row_at_index(0) {
        list_box.select_row(Some(&first_row));
    }
}

#[derive(Clone)]
struct AppInfo {
    name: String,
    exec: String,
    icon: String,
}
fn get_entries(dir: &str) -> Vec<AppInfo> {
    let apps_dir = std::fs::read_dir(dir).unwrap();
    let mut apps = Vec::new();

    for app in apps_dir {
        let app = app.unwrap();
        let app_path = app.path();
        if app_path.is_dir() {
            continue;
        }
        if app_path.extension().unwrap().to_str().unwrap() != "desktop" {
            continue;
        }

        let desktop_entry = match freedesktop_entry_parser::parse_entry(&app_path) {
            Ok(entry) => entry,
            Err(e) => {
                eprintln!(
                    "Error occurred while parsing desktop entry: {}",
                    e.to_string()
                );
                continue;
            }
        };

        let section = desktop_entry.section("Desktop Entry");
        let name = section.attr("Name").unwrap();
        let icon = section.attr("Icon").unwrap_or("applications-other");
        let exec = match section.attr("Exec") {
            Some(e) => e,
            None => continue,
        };

        apps.push(AppInfo {
            name: name.to_string(),
            icon: icon.to_string(),
            exec: exec.to_string(),
        });
    }

    apps
}

fn get_icon(icon_name: &str) -> Pixbuf {
    let icon;
    let icon_theme = IconTheme::default().unwrap();

    if let Ok(i) = Pixbuf::from_file_at_size(&icon_name, 32, 32) {
        icon = i;
    } else if let Ok(i) = icon_theme.load_icon(
        icon_name,
        32,
        IconLookupFlags::FORCE_SIZE | IconLookupFlags::USE_BUILTIN,
    ) {
        icon = i.unwrap();
    } else {
        icon = icon_theme
            .load_icon(
                "applications-other",
                32,
                IconLookupFlags::FORCE_SIZE | IconLookupFlags::USE_BUILTIN,
            )
            .unwrap()
            .unwrap();
    }

    icon
}

fn clear_listbox(list_box: &ListBox) {
    for child in &list_box.children() {
        list_box.remove(child);
    }
}

fn get_list_box(qb: &Entry) -> ListBox {
    let par: gtk::Box = qb.parent().unwrap().downcast().unwrap();
    let child = &par.children()[1];
    let scw = child.downcast_ref::<ScrolledWindow>().unwrap();
    let scw_child = &scw.children()[0];
    let view_port = scw_child.downcast_ref::<Viewport>().unwrap();
    let v_child = &view_port.children()[0];

    v_child.downcast_ref::<ListBox>().unwrap().clone()
}
