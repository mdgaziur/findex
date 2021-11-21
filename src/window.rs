use crate::common::{add_app_to_listbox, get_entries};
use crate::css::load_css;
use crate::query::init_query;
use crate::search_result::init_search_result;
use gtk::gdk::{EventMask, Screen};
use gtk::prelude::*;
use gtk::{
    Application, ApplicationWindow, BoxBuilder, ButtonsType, CssProvider, DialogFlags,
    MessageDialog, MessageType, Orientation, ScrolledWindow, WindowPosition,
};

pub fn init_window(app: &Application) {
    let win = ApplicationWindow::builder()
        .application(app)
        .window_position(WindowPosition::CenterAlways)
        .resizable(false)
        .default_width(600)
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

    // add css provider
    let css_provider = load_css();
    if let Err(e) = css_provider {
        show_dialog(&win, &e.to_string(), MessageType::Warning, "Warning");

        // try to load css from /opt/findex/style.css
        let file = "/opt/findex/style.css";
        let file_path = std::path::Path::new(file);

        if file_path.exists() {
            let cssprovider = CssProvider::default().unwrap();
            if let Err(e) = cssprovider.load_from_path(file) {
                show_dialog(
                    &win,
                    &(String::from("Failed to load fallback stylesheet: ") + &e.to_string()),
                    MessageType::Error,
                    "Error",
                );
            } else {
                gtk::StyleContext::add_provider_for_screen(
                    &win.screen().unwrap(),
                    &cssprovider,
                    gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
                );
            }
        }
    } else if let Ok(provider) = css_provider {
        gtk::StyleContext::add_provider_for_screen(
            &win.screen().unwrap(),
            &provider,
            gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
        );
    }

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

    win.show_all();

    for app in &desktop_entries {
        add_app_to_listbox(&list_box, &app);
    }
    if desktop_entries.len() > 0 {
        let first_row = list_box.row_at_index(0).unwrap();
        list_box.select_row(Some(&first_row));
    }
}

fn show_dialog(window: &ApplicationWindow, message: &str, message_type: MessageType, title: &str) {
    let dialog = MessageDialog::new(
        Some(window),
        DialogFlags::empty(),
        message_type,
        ButtonsType::Ok,
        message,
    );

    dialog.set_title(title);
    dialog.run();
    unsafe { dialog.destroy(); }
}
