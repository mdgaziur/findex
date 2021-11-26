use crate::common::{add_scored_app_to_listbox, AppInfo, ScoredApp};
use crate::config::FINDEX_CONFIG;
use fuse_rust::Fuse;
use gtk::gdk::EventKey;
use gtk::prelude::*;
use gtk::{Entry, ListBox, ScrolledWindow, Viewport};
use std::process::exit;

pub fn init_query(entries: &Vec<AppInfo>) -> Entry {
    let query_box = Entry::builder().name("findex-query").build();
    query_box.set_placeholder_text(Some(&FINDEX_CONFIG.query_placeholder));
    query_box.style_context().add_class("findex-query");
    query_box.connect_changed({
        let de = entries.clone();
        move |qb| on_text_changed(qb, &de)
    });
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

fn on_text_changed(qb: &Entry, apps: &[AppInfo]) {
    let text = regex::escape(&qb.text().to_lowercase());
    if text.is_empty() {
        return;
    }

    let list_box = get_list_box(qb);

    clear_listbox(&list_box);

    let mut filtered_apps = Vec::new();
    let mut fuse = Fuse::default();
    fuse.distance = FINDEX_CONFIG.max_fuzz_distance;
    for app in apps {
        let score_result_name = fuse.search_text_in_string(&text, &app.name);
        let score_result_exec = fuse.search_text_in_string(&text, &app.exec);
        let mut do_not_push = true;
        let mut total_score = 0f64;

        if let Some(result) = score_result_name {
            if result.score <= FINDEX_CONFIG.max_name_fuzz_result_score {
                total_score += result.score;
                do_not_push = false;
            }
        }
        if let Some(result) = score_result_exec {
            if result.score <= FINDEX_CONFIG.max_command_fuzz_result_score {
                total_score += result.score;
                do_not_push = false;
            }
        }

        if !do_not_push {
            filtered_apps.push(ScoredApp {
                total_score,
                name: app.name.clone(),
                exec: app.exec.clone(),
                icon: app.icon.clone(),
            });
        }
    }
    filtered_apps.sort_by(|l, r| l.total_score.partial_cmp(&r.total_score).unwrap());

    for app in filtered_apps {
        add_scored_app_to_listbox(&list_box, &app);
    }

    if let Some(first_row) = list_box.row_at_index(0) {
        list_box.select_row(Some(&first_row));
    }
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
