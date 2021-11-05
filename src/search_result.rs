use gtk::prelude::*;
use gtk::ListBox;

pub fn init_search_result() -> ListBox {
    let list_view = ListBox::builder().name("findex-results").build();
    list_view.style_context().add_class("findex-results");

    list_view
}
