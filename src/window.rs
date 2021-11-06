use crate::css::load_css;
use crate::query::init_query;
use crate::search_result::init_search_result;
use gtk::gdk::EventMask;
use gtk::prelude::*;
use gtk::{
    Application, ApplicationWindow, BoxBuilder, Orientation, ScrolledWindow, WindowPosition,
};

pub fn init_window(app: &Application) {
    let win = ApplicationWindow::builder()
        .application(app)
        .window_position(WindowPosition::CenterAlways)
        .resizable(false)
        .default_width(700)
        .decorated(false)
        .events(EventMask::FOCUS_CHANGE_MASK)
        .build();

    win.style_context().add_class("findex-window");

    win.connect_focus_out_event(|win, _event| {
        win.close();
        Inhibit(true)
    });
    win.connect_key_press_event(|win, ev| {
        if ev.keyval().name().unwrap() == "Escape" {
            win.close();
            return Inhibit(true);
        }

        Inhibit(false)
    });

    let container = BoxBuilder::new()
        .orientation(Orientation::Vertical)
        .parent(&win)
        .build();

    let search_box = init_query();
    let scw = ScrolledWindow::builder()
        .max_content_height(400)
        .propagate_natural_height(true)
        .build();
    scw.add(&init_search_result());

    container.add(&search_box);
    container.add(&scw);

    // add css provider
    gtk::StyleContext::add_provider_for_screen(
        &win.screen().unwrap(),
        &load_css(),
        gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
    );

    win.show_all();
}
