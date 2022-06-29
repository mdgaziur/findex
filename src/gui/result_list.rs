use gtk::prelude::*;
use gtk::{Container, Label, ListBox, ListBoxRow};

pub struct ResultList {
    pub list_box: ListBox,
}

impl ResultList {
    pub fn new(parent: &impl IsA<Container>) -> Self {
        let list_box = ListBox::builder().parent(parent).can_focus(false).build();

        list_box.style_context().add_class("findex-results");

        for _ in 0..10 {
            let _ = ListBoxRow::builder()
                .parent(&list_box)
                .child(&Label::new(Some("Text")))
                .build();
        }

        Self { list_box }
    }
}

pub fn is_row_selected(list_box: &ListBox, idx: usize) -> bool {
    let children = list_box.children();

    // TODO(mdgaziur): make it less weird
    if let Some(child) = children.get(idx) {
        if let Some(row) = child.downcast_ref::<ListBoxRow>() {
            if row.is_selected() {
                true
            } else {
                false
            }
        } else {
            false
        }
    } else {
        false
    }
}

pub fn get_row(list_box: &ListBox, idx: usize) -> Option<ListBoxRow> {
    let children = list_box.children();

    if let Some(child) = children.get(idx) {
        child.downcast_ref::<ListBoxRow>().map(|r| r.clone())
    } else {
        None
    }
}
