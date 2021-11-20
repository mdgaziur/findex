use crate::css::load_css;
use crate::query::init_query;
use crate::search_result::init_search_result;
use gtk::gdk::{EventMask, Screen};
use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, BoxBuilder, Orientation, ScrolledWindow, WindowPosition};
use crate::common::{add_app_to_listbox, get_entries};

pub fn init_window(app: &Application) {
    let win = ApplicationWindow::builder()
        .application(app)
        .window_position(WindowPosition::CenterAlways)
        .resizable(false)
        .default_width(650)
        .skip_pager_hint(true)
        .skip_taskbar_hint(true)
        .decorated(false)
        .events(EventMask::FOCUS_CHANGE_MASK)
        .build();

    win.style_context().add_class("findex-window");

    let screen = Screen::default().unwrap();
    let visual = screen.rgba_visual();

    if screen.is_composited() {
        if let Some(visual) = visual {
            win.set_visual(Some(&visual));
        }
    }

    win.connect_focus_out_event(|win, _event| {
        win.close();
        Inhibit(true)
    });
    win.connect_key_press_event(|win, ev| {
        let key_name = match ev.keyval().name() {
            Some(name) => name,
            None => return Inhibit(false),
        };

        if key_name == "Escape" {
            win.close();
            return Inhibit(true);
        }

        Inhibit(false)
    });

    let container = BoxBuilder::new()
        .orientation(Orientation::Vertical)
        .parent(&win)
        .build();
    container.style_context().add_class("findex-container");

    let mut desktop_entries = get_entries("/usr/share/applications");
    desktop_entries.extend(get_entries(
        shellexpand::tilde("~/.local/share/applications").as_ref(),
    ));

    let search_box = init_query(&desktop_entries);
    let list_box = init_search_result();

    let scw = ScrolledWindow::builder()
        .min_content_height(400)
        .max_content_height(400)
        .propagate_natural_height(true)
        .build();
    scw.add(&list_box);
    scw.style_context().add_class("findex-results-scroll");

    container.add(&search_box);
    container.add(&scw);

    // add css provider
    gtk::StyleContext::add_provider_for_screen(
        &win.screen().unwrap(),
        &load_css(),
        gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
    );

    win.show_all();

    for app in &desktop_entries {
        add_app_to_listbox(&list_box, &app);
    }
}
