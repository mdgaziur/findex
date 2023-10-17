use crate::app_list::{AppInfo, APPS_LIST};
use crate::config::FINDEX_CONFIG;
use crate::gui::result_list::result_list_clear;
use crate::gui::result_list_row::result_list_row;
use abi_stable::std_types::*;
use findex_plugin::findex_internal::KeyboardShortcut;
use gtk::builders::BoxBuilder;
use gtk::gdk::EventKey;
use gtk::prelude::*;
use gtk::{Container, Entry, ListBox, Orientation};
use std::cmp::min;
use sublime_fuzzy::{best_match, format_simple};

pub fn searchbox_new(parent: &impl IsA<Container>, result_list: ListBox) -> Entry {
    let container = BoxBuilder::new()
        .orientation(Orientation::Horizontal)
        .expand(true)
        .parent(parent)
        .build();
    container
        .style_context()
        .add_class("findex-query-container");

    let entry = Entry::builder()
        .placeholder_text(&FINDEX_CONFIG.query_placeholder)
        .parent(&container)
        .has_focus(true)
        .can_focus(true)
        .is_focus(true)
        .editable(true)
        .sensitive(true)
        .expand(true)
        .build();

    entry.connect_changed(move |entry| on_text_changed(entry, &result_list));
    entry.connect_key_press_event(on_key_pressed);
    entry.style_context().add_class("findex-query");

    entry
}

fn on_key_pressed(entry: &Entry, eventkey: &EventKey) -> Inhibit {
    let keyboard_shortcut = KeyboardShortcut::from_eventkey(eventkey);

    // Check if any plugin has registered keyboard shortcut
    for plugin in FINDEX_CONFIG.plugin_definitions.values() {
        if plugin.keyboard_shortcut.as_ref() == Some(&keyboard_shortcut) {
            entry.set_text(&format!("{} ", plugin.prefix.as_str()));
            entry.select_region(-1, -1);
            return Inhibit(true);
        }
    }

    Inhibit(false)
}

fn on_text_changed(entry: &Entry, result_list: &ListBox) {
    let text = entry.text();
    let mut matches: Vec<AppInfo> = Vec::new();
    result_list_clear(result_list);

    if let Some(plugin) = FINDEX_CONFIG
        .plugin_definitions
        .get(text.split_ascii_whitespace().next().unwrap_or(""))
    {
        let query = text.split_ascii_whitespace().collect::<Vec<_>>()[1..].join(" ");
        matches = unsafe { plugin.plugin_query_handler(RStr::from(query.as_str())) }.to_vec();
    } else {
        let apps = APPS_LIST.lock();

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

                    app.name = RString::from(formatted_name);
                    app.score = match_.score();
                    matches.push(app);
                }
            }
        }
    }
    matches.sort_by(|l, r| r.score.cmp(&l.score));

    let result_count = min(FINDEX_CONFIG.result_size, matches.len());
    for (app_idx, app) in matches.iter().take(result_count).enumerate() {
        result_list_row(
            result_list,
            &app.icon,
            &app.name.replace('&', "&amp;"),
            app.desc.as_deref(),
            &app.cmd,
            if app_idx < 10 { Some(app_idx) } else { None },
        );
    }

    let parent = result_list.parent().unwrap().parent().unwrap();
    if result_list.children().is_empty() {
        parent.hide();
    } else {
        parent.show();
    }
    result_list.show_all();
    result_list.select_row(result_list.row_at_index(0).as_ref());
    entry.grab_focus_without_selecting();
}
