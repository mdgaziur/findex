use std::ops::Deref;
use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, Box, CssProvider, glib, MessageType, Orientation, ScrolledWindow, WindowPosition};
use gtk::gdk::{EventMask, Screen};
use crate::gui::common::{add_app_to_listbox, clear_listbox, show_dialog};
use crate::gui::config::FINDEX_CONFIG;
use crate::gui::css::load_css;
use crate::gui::query::init_query;
use crate::gui::dbus::get_all;
use crate::gui::search_result::init_search_result;
use crate::{IS_SHOWN, SHOW_GUI};

mod config;
mod css;
mod query;
mod common;
mod search_result;
mod dbus;

pub struct FindexGUI {
    app: Application,
}

impl FindexGUI {
    pub fn init() -> Self {
        // deref to make sure it's evaluated
        let _ = FINDEX_CONFIG.deref();

        let app = Application::builder()
            .application_id("org.findex.gui")
            .build();

        app.connect_activate(Self::window);

        Self {
            app,
        }
    }

    fn window(app: &Application) {
        let mut window = ApplicationWindow::builder()
            .application(app)
            .window_position(WindowPosition::CenterAlways)
            .title("Findex")
            .resizable(false)
            .default_width(FINDEX_CONFIG.default_window_width)
            .decorated(FINDEX_CONFIG.decorate_window);

        if FINDEX_CONFIG.close_window_on_losing_focus {
            window = window
                .events(EventMask::FOCUS_CHANGE_MASK)
                .skip_pager_hint(true)
                .skip_taskbar_hint(true);
        }

        let window = window.build();
        window.set_keep_above(true);
        window.style_context().add_class("findex-window");

        if FINDEX_CONFIG.close_window_on_losing_focus {
            window.connect_focus_out_event(|win, _event| {
                *SHOW_GUI.lock().unwrap() = false;
                *IS_SHOWN.lock().unwrap() = false;
                win.hide();
                Inhibit(true)
            });
        }
        window.connect_key_press_event(|win, event| {
            let key_name = match event.keyval().name() {
                Some(name) => name,
                None => return Inhibit(false)
            };

            if key_name == "Escape" {
                *SHOW_GUI.lock().unwrap() = false;
                *IS_SHOWN.lock().unwrap() = false;
                win.hide();
                return Inhibit(true);
            }

            Inhibit(false)
        });

        let screen = Screen::default().unwrap();
        let visual = screen.rgba_visual();

        if screen.is_composited() {
            if let Some(visual) = visual {
                window.set_visual(Some(&visual));
            }
        }

        match load_css() {
            Ok(provider) => {
                gtk::StyleContext::add_provider_for_screen(
                    &window.screen().unwrap(),
                    &provider,
                    gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
                );
            }
            Err(e) => {
                show_dialog(&window, &e.to_string(), MessageType::Warning, "Warning");

                // try to load css from /opt/findex/style.css
                let file = "/opt/findex/style.css";
                let file_path = std::path::Path::new(file);

                if file_path.exists() {
                    let css_provider = CssProvider::default().unwrap();
                    if let Err(e) = css_provider.load_from_path(file) {
                        show_dialog(
                            &window,
                            &(String::from("Failed to load fallback stylesheet: ") + &e.to_string()),
                            MessageType::Error,
                            "Error",
                        );
                    } else {
                        gtk::StyleContext::add_provider_for_screen(
                            &window.screen().unwrap(),
                            &css_provider,
                            gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
                        );
                    }
                }
            }
        }

        let container = Box::builder()
            .orientation(Orientation::Vertical)
            .parent(&window)
            .build();
        container.style_context().add_class("findex-container");
        let apps = match get_all() {
            Ok(a) => a,
            Err(e) => {
                show_dialog(
                    &window,
                    &(String::from("Failed to get all apps list: ") + &e.to_string()),
                    MessageType::Error,
                    "Error",
                );
                app.quit();
                return;
            }
        };

        let search_box = init_query();
        let list_box = init_search_result();
        let scw = ScrolledWindow::builder()
            .min_content_height(FINDEX_CONFIG.min_content_height)
            .max_content_height(FINDEX_CONFIG.max_content_height)
            .propagate_natural_height(true)
            .build();
        scw.add(&list_box);
        scw.style_context().add_class("findex-results-scroll");

        container.add(&search_box);
        container.add(&scw);

        glib::idle_add({
            let window = fragile::Fragile::new(window.clone());
            let search_box = fragile::Fragile::new(search_box.clone());
            let list_box = fragile::Fragile::new(list_box.clone());
            let apps = apps.clone();
            move || {
                let mut show_gui = SHOW_GUI.lock().unwrap();
                if *show_gui && !*IS_SHOWN.lock().unwrap() {
                    window.get().present();
                    search_box.get().set_text("");
                    clear_listbox(list_box.get());

                    for app in &apps {
                        add_app_to_listbox(list_box.get(), app);
                    }
                    if !apps.is_empty() {
                        let first_row = list_box.get().row_at_index(0).unwrap();
                        list_box.get().select_row(Some(&first_row));
                    }

                    *IS_SHOWN.lock().unwrap() = true;
                } else if !*show_gui && *IS_SHOWN.lock().unwrap() {
                    window.get().hide();
                    *show_gui = false;
                    *IS_SHOWN.lock().unwrap() = false;
                }

                Continue(true)
            }
        });

        window.show_all();
        window.hide();
    }

    pub fn run(&self) {
        self.app.run();
    }
}