use crate::config::FINDEX_CONFIG;
use crate::gui::result_list_row::result_list_row;
use std::cmp::min;

use crate::app_list::{AppInfo, APPS_LIST};
use crate::gui::result_list::result_list_clear;
use gtk::prelude::*;
use gtk::{Container, Entry, ListBox};
use sublime_fuzzy::{best_match, format_simple};

pub fn searchbox_new(parent: &impl IsA<Container>, result_list: ListBox) -> Entry {
    let entry = Entry::builder()
        .placeholder_text(&FINDEX_CONFIG.query_placeholder)
        .parent(parent)
        .has_focus(true)
        .can_focus(true)
        .is_focus(true)
        .editable(true)
        .sensitive(true)
        .build();

    entry.connect_changed(move |entry| on_text_changed(entry, &result_list));
    entry.style_context().add_class("findex-query");

    entry
}

fn on_text_changed(entry: &Entry, result_list: &ListBox) {
    let text = entry.text();
    let apps = APPS_LIST.lock();
    let mut matches: Vec<AppInfo> = Vec::new();
    result_list_clear(result_list);

    for app in &*apps {
        if let Some(match_) = best_match(&text, &app.name) {
            let mut app = app.clone();
            if match_.score() > FINDEX_CONFIG.min_score {
                let formatted_name = format_simple(
                    &match_,
                    &app.name,
                    &format!(
                        "<span color=\"{}\">",
                        FINDEX_CONFIG.name_match_highlight_color
                    ),
                    "</span>",
                );

                app.name = formatted_name;
                app.score = match_.score();
                matches.push(app);
            }
        }
    }
    matches.sort_by(|l, r| r.score.cmp(&l.score));

    let result_count = min(FINDEX_CONFIG.result_size, matches.len());
    for app in matches.iter().take(result_count) {
        result_list_row(
            result_list,
            &app.icon,
            &app.name.replace('&', "&amp;"),
            app.desc.as_deref(),
            &app.cmd,
            &app.id,
        );
    }

    let parent = result_list.parent().unwrap().parent().unwrap();
    if result_list.children().is_empty() {
        parent.hide();
    } else {
        parent.show();
    }
    result_list.show_all();
    if let Some(row) = result_list.row_at_index(0) {
        row.grab_focus()
    }
    result_list.select_row(result_list.row_at_index(0).as_ref());
    entry.grab_focus();
    entry.select_region(-1, -1);
}
