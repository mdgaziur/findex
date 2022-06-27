use gtk::prelude::*;
use gtk::{Container, TreeView};

pub struct ResultList {
    pub tree_view: TreeView,
}

impl ResultList {
    pub fn new(parent: &impl IsA<Container>) -> Self {
        let tree_view = TreeView::builder().expand(true).parent(parent).build();

        tree_view.style_context().add_class("findex-results");

        Self { tree_view }
    }
}
