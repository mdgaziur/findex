use gtk::prelude::*;
use gtk::{Image, Label, ListBox, BoxBuilder, Orientation, ListBoxRow, IconTheme, IconLookupFlags};
use gtk::gdk_pixbuf::{Colorspace, Pixbuf};
use gtk::pango::EllipsizeMode;

#[derive(Debug, Clone)]
pub struct ScoredApp {
    pub total_score: f64,
    pub name: String,
    pub exec: String,
    pub icon: String
}

#[derive(Debug, Clone)]
pub struct AppInfo {
    pub name: String,
    pub exec: String,
    pub icon: String,
}

pub fn add_app_to_listbox(list_box: &ListBox, app: &AppInfo) {
    add_scored_app_to_listbox(list_box, &ScoredApp {
        total_score: 0 as f64,
        icon: app.icon.clone(),
        name: app.name.clone(),
        exec: app.exec.clone(),
    });
}

pub fn add_scored_app_to_listbox(list_box: &ListBox, app: &ScoredApp) {
    let icon = get_icon(&app.icon);

    let image = Image::builder().pixbuf(&icon).build();
    image.style_context().add_class("findex-result-icon");

    let name = Label::new(Some(&app.name));
    name.style_context().add_class("findex-result-app-name");

    let command = Label::new(Some(&app.exec));
    command.set_xalign(0f32);
    command.set_max_width_chars(1);
    command.set_hexpand(true);
    command.set_ellipsize(EllipsizeMode::End);
    command.style_context().add_class("findex-result-command");

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

pub fn get_entries(dir: &str) -> Vec<AppInfo> {
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