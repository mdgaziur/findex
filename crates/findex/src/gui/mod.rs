mod css;
pub mod dialog;
mod result_list;
mod result_list_row;
mod searchbox;

use crate::config::FINDEX_CONFIG;
use crate::gui::css::load_css;
use crate::gui::result_list::{result_list_clear, result_list_new};
use crate::gui::result_list_row::handle_enter;
use crate::gui::searchbox::searchbox_new;
use crate::show_dialog;
use gtk::builders::BoxBuilder;
use gtk::gdk::{EventKey, EventMask, Screen};
use gtk::prelude::*;
use gtk::{
    gdk, Adjustment, Entry, ListBox, ListBoxRow, MessageType, Orientation, ScrolledWindow, Window,
    WindowType,
};
use keybinder::KeyBinder;

#[allow(clippy::upper_case_acronyms)]
pub struct GUI {
    pub window: Window,
    search_box: Entry,
    result_list: ListBox,
    keybinder: Option<KeyBinder<KeypressHandlerPayload>>,
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
            .events(EventMask::BUTTON_PRESS_MASK)
            .can_focus(true)
            .build();
        window.set_keep_above(true);
        window.style_context().add_class("findex-window");
        window.connect_destroy(|_| gtk::main_quit());

        let screen = Screen::default().unwrap();
        let visual = screen.rgba_visual();
        window.set_visual(visual.as_ref());

        match load_css() {
            Ok(provider) => gtk::StyleContext::add_provider_for_screen(
                &window.screen().unwrap(),
                &provider,
                gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
            ),
            Err(e) => show_dialog(
                "Warning",
                &format!("Failed to load css: {e}"),
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
        scrolled_container
            .style_context()
            .add_class("findex-results-scroll");

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
            let scrolled_container = scrolled_container;

            move |window, event| {
                // TODO(mdgaziur): fix this hack
                let entry = entry.clone();
                let list_box = list_box.clone();
                let scrolled_container = scrolled_container.clone();

                keypress_handler(window, entry, scrolled_container, list_box, event)
            }
        });

        let keybinder = if std::env::var("WAYLAND_DISPLAY").is_err() {
            match KeyBinder::new(true) {
                Ok(instance) => Some(instance),
                Err(_e) => {
                    eprintln!("[ERROR] Keybinder is not supported");
                    std::process::exit(1);
                }
            }
        } else {
            None
        };

        Self {
            keybinder,
            window,
            result_list,
            search_box,
        }
    }

    pub fn wait_for_toggle(&mut self) {
        if let Some(keybinder) = &mut self.keybinder {
            assert!(
                keybinder.bind(
                    &FINDEX_CONFIG.toggle_key,
                    |_, payload| {
                        Self::show_window(
                            &payload.window,
                            &payload.search_box,
                            &payload.result_list,
                        );
                        Self::position_window(&payload.window);
                    },
                    KeypressHandlerPayload {
                        window: self.window.clone(),
                        result_list: self.result_list.clone(),
                        search_box: self.search_box.clone(),
                    },
                ),
                "Failed to bind key"
            );
        } else {
            use gtk::glib::thread_guard::ThreadGuard;
            use inotify::{Inotify, WatchMask};
            use shellexpand::tilde;
            use std::fs::File;
            use std::path::Path;

            let mut inotify = Inotify::init().expect("Failed to init inotify");
            let watch_mask =
                WatchMask::CREATE | WatchMask::MODIFY | WatchMask::MOVE | WatchMask::DELETE;
            let toggle_file = {
                if !Path::new(&*tilde("~/.config/findex/toggle_file")).is_file() {
                    File::create(&*tilde("~/.config/findex/toggle_file"))
                        .expect("Failed to create file that toggles findex window");
                }

                &*tilde("~/.config/findex/toggle_file")
            };
            inotify
                .add_watch(toggle_file, watch_mask)
                .expect("Failed to add toggle file to inotify watch list");
            let (tx, rx) = gdk::glib::MainContext::channel::<()>(gdk::glib::PRIORITY_DEFAULT);

            std::thread::spawn(move || loop {
                let mut buf = [0; 1024];

                if let Ok(mut events) = inotify.read_events_blocking(&mut buf) {
                    if events.next().is_some() {
                        tx.send(()).expect("Error when notifying event");
                    }
                }
            });

            rx.attach(None, {
                let window = ThreadGuard::new(self.window.clone());
                let search_box = ThreadGuard::new(self.search_box.clone());
                let result_list = ThreadGuard::new(self.result_list.clone());
                move |()| {
                    Self::show_window(
                        window.get_ref(),
                        search_box.get_ref(),
                        result_list.get_ref(),
                    );
                    Continue(true)
                }
            });
        }
    }

    fn show_window(window: &Window, search_box: &Entry, result_list: &ListBox) {
        window.present();

        if std::env::var("WAYLAND_DISPLAY").is_err() {
            window.present_with_time(keybinder::get_current_event_time());
        }

        search_box.set_text("");
        result_list_clear(result_list);
        Self::position_window(window);
    }

    fn position_window(window: &Window) {
        let display = gdk::Display::default().unwrap();
        let monitor_geo = display
            .monitor_at_window(&window.window().unwrap())
            .unwrap()
            .geometry();
        let screen_height = monitor_geo.height() as f32;
        let screen_width = monitor_geo.width() as f32;

        window.move_(
            (screen_width * 0.5 - (window.allocation().width() / 2) as f32) as i32,
            (screen_height * 0.3) as i32,
        );
    }

    fn hide_window(window: &Window) {
        window.hide();
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
        if let Some(selected_row) = list_box.selected_row() {
            let row_index = selected_row.index() as usize;

            if row_index == list_box.children().len() - 1 {
                list_box.select_row(list_box.row_at_index(0).as_ref());
                scrolled_container
                    .set_vadjustment(Some(&Adjustment::builder().value(0f64).build()));
                entry.grab_focus();
                entry.select_region(-1, -1);

                Inhibit(true)
            } else if row_index == 0 && list_box.children().len() > 1 {
                list_box.select_row(list_box.row_at_index(1).as_ref());
                if let Some(row) = list_box.row_at_index(1) {
                    row.grab_focus()
                }

                Inhibit(true)
            } else if row_index == 0 && list_box.children().len() == 1 {
                entry.grab_focus();
                entry.select_region(-1, -1);

                Inhibit(true)
            } else {
                Inhibit(false)
            }
        } else {
            list_box.select_row(list_box.row_at_index(0).as_ref());
            if let Some(row) = list_box.row_at_index(0) {
                row.grab_focus()
            }

            Inhibit(true)
        }
    } else if key_name == "Up" {
        if let Some(row) = list_box.row_at_index(0) {
            if row.is_selected() {
                return if list_box.children().len() == 1 {
                    entry.grab_focus();
                    entry.select_region(-1, -1);

                    Inhibit(true)
                } else {
                    let last_row_widget = list_box.children().last().unwrap().clone();
                    let last_row = last_row_widget.downcast_ref::<ListBoxRow>().unwrap();
                    list_box.select_row(Some(last_row));
                    last_row.grab_focus();

                    let adjustment = scrolled_container.vadjustment();
                    adjustment.set_value(scrolled_container.vadjustment().upper());
                    scrolled_container.set_vadjustment(Some(&adjustment));

                    // "Up" button will select the row before last row if not inhibited
                    Inhibit(true)
                };
            } else if let Some(row) = list_box.selected_row() {
                if row.index() == 1 {
                    list_box.unselect_row(&row);
                    list_box.select_row(list_box.row_at_index(0).as_ref());
                    scrolled_container
                        .set_vadjustment(Some(&Adjustment::builder().value(0f64).build()));

                    entry.grab_focus();
                    entry.select_region(-1, -1);
                }
            }
        }

        Inhibit(false)
    } else if key_name == "Return" {
        if let Some(row) = list_box.selected_row() {
            handle_enter(&row);
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
