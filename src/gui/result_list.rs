use crate::gui::result_list_row::result_list_row;
use gtk::gio::AppInfo;
use gtk::glib::GString;
use gtk::prelude::*;
use gtk::{ListBox, ScrolledWindow};

pub fn result_list_new(parent: &ScrolledWindow) -> ListBox {
    let list_box = ListBox::builder().parent(parent).can_focus(true).build();

    list_box.style_context().add_class("findex-results");
    list_box.connect_hierarchy_changed({
        let parent = parent.clone();

        move |list_box, _| {
            if list_box.children().len() == 0 {
                parent.hide();
            } else {
                parent.show();
            }
        }
    });

    list_box
}

pub fn result_list_clear(list_box: &ListBox) {
    for child in list_box.children() {
        list_box.remove(&child);
    }
}

pub fn result_list_load_and_sort_apps(list_box: &ListBox) {
    let mut apps = AppInfo::all();
    apps.sort_by_key(|app| app.name());
    if !apps.is_empty() {
        list_box.parent().unwrap().parent().unwrap().show_all();
    } else {
        list_box.parent().unwrap().parent().unwrap().hide();
    }

    for app in apps {
        let app_cmd = match app.commandline() {
            Some(p) => p.display().to_string(),
            None => String::from(""),
        };
        let icon = match app.icon() {
            Some(icon) => IconExt::to_string(&icon).unwrap(),
            None => GString::from("application-other"),
        };

        result_list_row(
            list_box,
            &icon,
            &app.name(),
            app.description(),
            &app_cmd,
            app.commandline().is_some(),
            0,
            1,
        );
    }

    list_box.show_all();
}
