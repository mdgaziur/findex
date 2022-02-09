use crate::gui::config::FINDEX_CONFIG;
use gtk::gdk::EventKey;
use gtk::prelude::*;
use gtk::{Entry, ListBox, MessageType, ScrolledWindow, Viewport, Window};
use std::process::exit;
use crate::gui::common::{add_app_to_listbox, show_dialog, clear_listbox};
use crate::gui::dbus::get_result;

pub fn init_query() -> Entry {
    let query_box = Entry::builder().name("findex-query").build();
    query_box.set_placeholder_text(Some(&FINDEX_CONFIG.query_placeholder));
    query_box.style_context().add_class("findex-query");
    query_box.connect_changed(on_text_changed);
    query_box.connect_key_press_event(on_key_press);

    query_box
}

fn on_key_press(qb: &Entry, ev: &EventKey) -> Inhibit {
    let key_name = match ev.keyval().name() {
        Some(name) => name,
        None => return Inhibit(false),
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

fn on_text_changed(qb: &Entry) {
    let text = regex::escape(&qb.text().to_lowercase());
    if text.is_empty() {
        return;
    }

    let list_box = get_list_box(qb);

    clear_listbox(&list_box);
    let window = qb.toplevel().unwrap();
    let window = window.downcast_ref::<Window>().unwrap();
    let filtered_apps = match get_result(&text) {
        Ok(apps) => apps,
        Err(e) => {
            show_dialog(
                window,
                &(String::from("Failed to load fallback stylesheet: ") + &e.to_string()),
                MessageType::Error,
                "Error",
            );
            return;
        }
    };
    for app in filtered_apps {
        add_app_to_listbox(&list_box, &app);
    }

    if let Some(first_row) = list_box.row_at_index(0) {
        list_box.select_row(Some(&first_row));
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