use crate::config::FINDEX_CONFIG;
use gtk::prelude::*;
use gtk::{Container, Entry};

pub fn searchbox_new(parent: &impl IsA<Container>) -> Entry {
    let entry = Entry::builder()
        .placeholder_text(&FINDEX_CONFIG.query_placeholder)
        .parent(parent)
        .has_focus(true)
        .can_focus(true)
        .is_focus(true)
        .editable(true)
        .sensitive(true)
        .build();

    entry.style_context().add_class("findex-query");

    entry
}
