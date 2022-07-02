mod css;
pub mod dialog;
mod result_list;
mod result_list_row;
mod searchbox;

use crate::config::FINDEX_CONFIG;
use crate::gui::css::load_css;
use crate::gui::result_list::{result_list_clear, result_list_new};
use crate::gui::result_list_row::handle_click_or_enter;
use crate::gui::searchbox::searchbox_new;
use crate::{show_dialog, SHOW_WINDOW};
use gtk::builders::BoxBuilder;
use gtk::gdk::{EventKey, Screen};
use gtk::prelude::*;
use gtk::{
    gdk, Adjustment, Entry, ListBox, MessageType, Orientation, ScrolledWindow, Window, WindowType,
};
use keybinder::KeyBinder;

pub struct GUI {
    pub window: Window,
    search_box: Entry,
    result_list: ListBox,
    keybinder: KeyBinder<KeypressHandlerPayload>,
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
            .type_(WindowType::Toplevel)
            .can_focus(true)
            .build();
        window.set_keep_above(true);

        let screen = Screen::default().unwrap();
        let visual = screen.rgba_visual();

        if screen.is_composited() {
            if let Some(visual) = visual {
                window.set_visual(Some(&visual));
            }
        }

        match load_css() {
            Ok(provider) => gtk::StyleContext::add_provider_for_screen(
                &window.screen().unwrap(),
                &provider,
                gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
            ),
            Err(e) => show_dialog(
                "Warning",
                &format!("Failed to load css: {}", e),
                MessageType::Warning,
            ),
        }

        if FINDEX_CONFIG.close_window_on_losing_focus {
            window.connect_focus_out_event(|window, _| {
                Self::hide_window(window);

                Inhibit(false)
            });
        }

        let container = BoxBuilder::new()
            .parent(&window)
            .orientation(Orientation::Vertical)
            .build();
        container.style_context().add_class("findex-container");

        let scrolled_container = ScrolledWindow::builder()
            .max_content_height(FINDEX_CONFIG.max_content_height)
            .propagate_natural_height(true)
            .build();
        scrolled_container.style_context().add_class("findex-results-scroll");

        if FINDEX_CONFIG.min_content_height > 0 {
            scrolled_container.set_min_content_height(FINDEX_CONFIG.min_content_height);
            scrolled_container.set_propagate_natural_height(false);
        }
        let result_list = result_list_new(&scrolled_container);
        let search_box = searchbox_new(&container, result_list.clone());

        container.add(&scrolled_container);
        container.show_all();
        scrolled_container.hide();

        window.connect_key_press_event({
            let entry = search_box.clone();
            let list_box = result_list.clone();
            let scrolled_container = scrolled_container.clone();

            move |window, event| {
                // TODO(mdgaziur): fix this hack
                let entry = entry.clone();
                let list_box = list_box.clone();
                let scrolled_container = scrolled_container.clone();

                keypress_handler(window, entry, scrolled_container, list_box, event)
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
            |_, payload| {
                let mut show_window = SHOW_WINDOW.lock();

                *show_window = true;
                payload.window.present();
                payload
                    .window
                    .present_with_time(keybinder::get_current_event_time());
                payload.search_box.set_text("");
                result_list_clear(&payload.result_list);
                Self::position_window(&payload.window);
            },
            KeypressHandlerPayload {
                window: self.window.clone(),
                result_list: self.result_list.clone(),
                search_box: self.search_box.clone(),
            },
        );
    }

    fn position_window(window: &Window) {
        let display = gdk::Display::default().unwrap();
        let monitor_geo = display.primary_monitor().unwrap().geometry();
        let screen_height = monitor_geo.height() as f32;
        let screen_width = monitor_geo.width() as f32;

        window.move_(
            (screen_width * 0.5 - (window.allocation().width() / 2) as f32) as i32
                + monitor_geo.x(),
            (screen_height * 0.3) as i32 + monitor_geo.y(),
        );
    }

    fn hide_window(window: &Window) {
        window.hide();
        *SHOW_WINDOW.lock() = false;
    }
}

struct KeypressHandlerPayload {
    window: Window,
    result_list: ListBox,
    search_box: Entry,
}

fn keypress_handler(
    window: &Window,
    entry: Entry,
    scrolled_container: ScrolledWindow,
    list_box: ListBox,
    event: &EventKey,
) -> Inhibit {
    let key_name = event.keyval().name().unwrap();

    if key_name == "Escape" {
        GUI::hide_window(window);
        Inhibit(true)
    } else if key_name == "Down" {
        if list_box.selected_row().is_none() {
            list_box.select_row(list_box.row_at_index(0).as_ref());
            list_box.row_at_index(0).map(|row| row.grab_focus());

            Inhibit(true)
        } else if list_box.selected_row().unwrap().index()
            == list_box.children().len().checked_sub(1).unwrap_or(0) as i32
        {
            list_box.unselect_row(&list_box.selected_row().unwrap());
            scrolled_container.set_vadjustment(Some(&Adjustment::builder().value(0f64).build()));
            entry.grab_focus();
            entry.select_region(-1, -1);

            Inhibit(true)
        } else {
            Inhibit(false)
        }
    } else if key_name == "Up" {
        if let Some(row) = list_box.row_at_index(0) {
            if row.is_selected() {
                list_box.unselect_row(&row);
                entry.grab_focus();
                entry.select_region(-1, -1);
            }
        }

        Inhibit(false)
    } else if key_name == "Return" {
        if let Some(row) = list_box.selected_row() {
            handle_click_or_enter(&row);
        }

        Inhibit(true)
    } else {
        if !entry.has_focus() {
            entry.grab_focus();
            entry.select_region(-1, -1);
        }

        Inhibit(false)
    }
}
