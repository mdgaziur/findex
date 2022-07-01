use gtk::prelude::*;
use gtk::{ListBox, ScrolledWindow};

pub fn result_list_new(parent: &ScrolledWindow) -> ListBox {
    let list_box = ListBox::builder().parent(parent).can_focus(true).build();

    list_box.style_context().add_class("findex-results");

    list_box
}

pub fn result_list_clear(list_box: &ListBox) {
    for child in list_box.children() {
        list_box.remove(&child);
    }
}
