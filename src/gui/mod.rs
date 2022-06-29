mod css;
pub mod dialog;
mod result_list;
mod searchbox;

use crate::config::FINDEX_CONFIG;
use crate::gui::css::load_css;
use crate::gui::result_list::{get_row, is_row_selected, ResultList};
use crate::gui::searchbox::SearchBox;
use crate::{show_dialog, SHOW_WINDOW};
use gtk::builders::BoxBuilder;
use gtk::gdk::EventKey;
use gtk::prelude::*;
use gtk::{Entry, ListBox, MessageType, Orientation, Window, WindowPosition, WindowType};
use keybinder::KeyBinder;

pub struct GUI {
    pub window: Window,
    search_box: SearchBox,
    result_list: ResultList,
    keybinder: KeyBinder<Window>,
}

impl GUI {
    pub fn new() -> Self {
        let window = Window::builder()
            .title("Findex")
            .resizable(false)
            .default_width(FINDEX_CONFIG.default_window_width)
            .decorated(FINDEX_CONFIG.decorate_window)
            .skip_taskbar_hint(true)
            .skip_pager_hint(true)
            .deletable(false)
            .window_position(WindowPosition::Center)
            .type_(WindowType::Toplevel)
            .can_focus(true)
            .build();
        window.set_keep_above(true);

        match load_css() {
            Ok(provider) => gtk::StyleContext::add_provider_for_screen(
                &window.screen().unwrap(),
                &provider,
                gtk::STYLE_PROVIDER_PRIORITY_APPLICATION
            ),
            Err(e) => show_dialog(
                "Warning",
                &format!("Failed to load css: {}", e),
                MessageType::Warning,
            ),
        }

        window.connect_focus_out_event(|window, _| {
            Self::hide_window(window);

            Inhibit(false)
        });

        let container = BoxBuilder::new()
            .parent(&window)
            .orientation(Orientation::Vertical)
            .build();
        let search_box = SearchBox::new(&container);
        let result_list = ResultList::new(&container);
        container.show_all();

        window.connect_key_press_event({
            let entry = search_box.entry.clone();
            let list_box = result_list.list_box.clone();

            move |window, event| {
                // TODO(mdgaziur): fix this hack
                let entry = entry.clone();
                let list_box = list_box.clone();

                keypress_handler(window, entry, list_box, event)
            }
        });
        Self {
            window,
            search_box,
            result_list,
            keybinder: KeyBinder::new(true).expect("Keybinder is not supported"),
        }
    }

    pub fn listen_for_hotkey(&mut self) {
        self.keybinder.bind(
            &FINDEX_CONFIG.toggle_key,
            |_, window| {
                let mut show_window = SHOW_WINDOW.lock().unwrap();

                *show_window = true;
                window.present();
                window.present_with_time(keybinder::get_current_event_time());
            },
            self.window.clone(),
        );
    }

    fn hide_window(window: &Window) {
        window.hide();
        *SHOW_WINDOW.lock().unwrap() = false;
    }
}

fn keypress_handler(window: &Window, entry: Entry, list_box: ListBox, event: &EventKey) -> Inhibit {
    let key_name = event.keyval().name().unwrap();

    if key_name == "Escape" {
        GUI::hide_window(window);
        Inhibit(true)
    } else if key_name == "Down" {
        let first_row = get_row(
            &list_box,
            (list_box.selected_row().map(|r| r.index()).unwrap_or(-1) + 1) as usize,
        );
        if let Some(row) = first_row {
            list_box.select_row(Some(&row));
        }

        Inhibit(true)
    } else if key_name == "Up" {
        let first_row = get_row(
            &list_box,
            (list_box.selected_row().map(|r| r.index()).unwrap_or(0) as usize)
                .checked_sub(1)
                .unwrap_or(0),
        );
        if let Some(row) = first_row {
            list_box.select_row(Some(&row));
        }

        Inhibit(true)
    } else {
        Inhibit(false)
    }
}
