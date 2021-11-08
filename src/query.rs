use gtk::gdk::gdk_pixbuf::{Colorspace, Pixbuf};
use gtk::gdk::EventKey;
use gtk::prelude::*;
use gtk::{
    BoxBuilder, Entry, IconLookupFlags, IconTheme, Image, Label, ListBox, ListBoxRow, Orientation,
    ScrolledWindow, Viewport,
};
use std::process::exit;
use fuse_rust::Fuse;

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
    let key_name = match ev.keyval().name() {
        Some(name) => name,
        None => return Inhibit(false)
    };

    if key_name == "Return" {
        let list = get_list_box(qb);
        let first_result = match list.row_at_index(0) {
            Some(res) => res,
            None => exit(0),
        };
        first_result.emit_activate();

        Inhibit(true)
    } else if key_name == "Down" {
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

    let list_box = get_list_box(qb);

    clear_listbox(&list_box);

    // filter stuff that didn't match
    struct ScoredApp {
        total_score: f64,
        name: String,
        exec: String,
        icon: String
    };
    let mut filtered_apps = Vec::new();
    let mut fuse = Fuse::default();
    fuse.distance = 80;
    for app in apps {
        let score_result_name = fuse.search_text_in_string(&text, &app.name);
        let score_result_exec = fuse.search_text_in_string(&text, &app.exec);
        let mut do_not_push = true;
        let mut total_score = 0f64;

        if let Some(result) = score_result_name {
            if result.score <= 0.4 {
                total_score += result.score;
                do_not_push = false;
            }
        }
        if let Some(result) = score_result_exec {
            if result.score <= 0.4 {
                total_score += result.score;
                do_not_push = false;
            }
        }

        if !do_not_push {
            filtered_apps.push(ScoredApp {
                total_score,
                name: app.name.clone(),
                exec: app.exec.clone(),
                icon: app.icon.clone()
            });
        }
    }
    filtered_apps.sort_by(|l, r| {
       l.total_score.partial_cmp(&r.total_score).unwrap()
    });


    for app in filtered_apps {
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
    let apps_dir = match std::fs::read_dir(dir) {
        Ok(path) => path,
        Err(e) => {
            println!("Could not access: {}, reason: {}", dir, e.to_string());
            return vec![]
        }
    };
    let mut apps = Vec::new();
    let parameter_regex = regex::Regex::new("%.").unwrap();

    for app in apps_dir {
        let app = app.unwrap();
        let app_path = app.path();
        if app_path.is_dir() {
            continue;
        }
        if app_path.extension().unwrap_or_default() != "desktop" {
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

        let name = match section.attr("Name") {
            Some(n) => n,
            None => {
                eprintln!("Error occurred while parsing {}: {}", app_path.display(), "cannot find 'Name' field");
                continue;
            }
        };
        let icon = section.attr("Icon").unwrap_or("applications-other");
        let exec = match section.attr("Exec") {
            Some(e) => {
                parameter_regex.replace_all(e, "")
            },
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
            .or::<Result<Pixbuf,()>>(Ok(Pixbuf::new(Colorspace::Rgb, true, 8, 32, 32)))
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
