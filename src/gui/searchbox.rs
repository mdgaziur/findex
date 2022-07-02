use crate::config::FINDEX_CONFIG;
use crate::gui::result_list_row::result_list_row;

use crate::app_list::AppInfo;
use crate::gui::result_list::result_list_clear;
use gtk::gio::AppInfo as GIOAppInfo;
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

    entry.connect_changed(move |entry| on_text_changed(&entry, &result_list));
    entry.style_context().add_class("findex-query");

    entry
}

fn on_text_changed(entry: &Entry, result_list: &ListBox) {
    let text = entry.text();
    let mut apps = GIOAppInfo::all()
        .iter()
        .filter(|app| app.commandline().is_some())
        .map(|app| AppInfo::from(app))
        .collect::<Vec<AppInfo>>();
    let mut matches_: Vec<&AppInfo> = Vec::new();
    result_list_clear(result_list);

    for app in &mut *apps {
        if let Some(match_) = best_match(&text, &app.name) {
            if match_.score() > 5 {
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
                matches_.push(app);
            }
        }
    }
    matches_.sort_by_key(|app| app.score);

    let count = if matches_.len() > 10 {
        10
    } else {
        matches_.len()
    };

    for idx in 0..count {
        let app = &matches_[idx];

        result_list_row(
            result_list,
            &app.icon,
            &app.name.replace("&", "&amp;"),
            app.desc.as_ref().map(|desc| desc.as_str()),
            &app.cmd,
            &app.id,
        );
    }

    let parent = result_list.parent().unwrap().parent().unwrap();
    if result_list.children().len() == 0 {
        parent.hide();
    } else {
        parent.show();
    }
    result_list.show_all();
    result_list.row_at_index(0).map(|row| row.grab_focus());
    result_list.select_row(result_list.row_at_index(0).as_ref());
    entry.grab_focus();
    entry.select_region(-1, -1);
}
