use crate::app_list::update_apps_list;
use crate::config::FINDEX_CONFIG;
use crate::gui::dialog::show_dialog;
use crate::gui::GUI;
use gtk::MessageType;
use inotify::{Inotify, WatchMask};
use std::ffi::OsStr;

mod app_list;
mod config;
mod gui;

static FINDEX_VERSION: &str = env!("CARGO_PKG_VERSION");

fn main() {
    if std::env::args().any(|arg| arg == "--about") {
        println!("Findex v{FINDEX_VERSION}");
        println!("Author: MD Gaziur Rahman Noor <mdgaziurrahmannoor@gmail.com>");
        println!("License: GPL3");
        println!("Report issues at: https://github.com/mdgaziur/findex/issues");
        return;
    }

    println!("[INFO] Starting Findex...");
    gtk::init().expect("Failed to init GTK");
    if !FINDEX_CONFIG.error.is_empty() {
        show_dialog("Warning", &FINDEX_CONFIG.error, MessageType::Warning);
    }

    update_apps_list();

    let mut inotify = Inotify::init().expect("Failed to init inotify");
    let watch_mask = WatchMask::CREATE | WatchMask::MODIFY | WatchMask::MOVE | WatchMask::DELETE;
    let base_directories = xdg::BaseDirectories::new().expect("Failed to get base directories");

    for dir in base_directories.get_data_dirs() {
        let watch_dir = dir.join("applications");

        if !watch_dir.exists() {
            continue;
        }
        if let Err(e) = inotify.watches().add(&watch_dir, watch_mask) {
            eprintln!("[WARN] Failed to watch `{}`: {}", watch_dir.display(), e);
        }
    }

    let xdg_data_home = base_directories.get_data_home().join("applications");
    if xdg_data_home.exists() {
        if let Err(e) = inotify.watches().add(&xdg_data_home, watch_mask) {
            eprintln!(
                "[WARN] Failed to watch `{}`: {}",
                xdg_data_home.display(),
                e
            );
        }
    }

    std::thread::spawn(move || {
        loop {
            let mut buffer = [0; 1024];
            if let Ok(events) = inotify.read_events_blocking(&mut buffer) {
                // if we're here this means something changed inside any of the directories
                for event in events {
                    println!(
                        "[INFO] File `{}` was changed",
                        event
                            .name
                            .unwrap_or_else(|| OsStr::new("unavailable"))
                            .to_str()
                            .unwrap_or("file name with invalid unicode chars")
                    );
                }
                update_apps_list();
            }
        }
    });

    let mut gui = GUI::new();
    gui.wait_for_toggle();
    println!("[INFO] listening for hotkey...");

    gtk::main();
}
