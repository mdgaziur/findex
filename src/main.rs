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

fn main() {
    if std::env::var("XDG_SESSION_TYPE") != Ok(String::from("x11")) {
        eprintln!("[Error] Only X11 is supported");
        std::process::exit(1);
    }

    println!("[INFO] Starting Findex...");
    gtk::init().expect("Failed to init GTK");
    if !FINDEX_CONFIG.error.is_empty() {
        show_dialog("Warning", &FINDEX_CONFIG.error, MessageType::Warning);
    }

    update_apps_list();

    let mut inotify = Inotify::init().expect("Failed to init inotify");
    let watch_mask = WatchMask::CREATE | WatchMask::MODIFY | WatchMask::MOVE | WatchMask::DELETE;
    let base_directories = xdg::BaseDirectories::new()
        .expect("Failed to get base directories");

    for dir in base_directories.get_data_dirs() {
        let watch_dir = dir.join("applications");

        if !watch_dir.exists() {
            continue;
        }
        if let Err(e) = inotify.add_watch(
            &watch_dir,
            watch_mask,
        ) {
            eprintln!(
                "[WARN] Failed to watch `{}`: {}",
                watch_dir.display(),
                e
            );
        }
    }

    let xdg_data_home = base_directories.get_data_home().join("applications");
    if xdg_data_home.exists() {
        if let Err(e) = inotify.add_watch(
            &xdg_data_home,
            watch_mask,
        ) {
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
            if let Ok(events) = inotify
                .read_events_blocking(&mut buffer) {
                // if we're here this means something changed inside any of the directories
                for event in events {
                    println!(
                        "[Info] File `{}` was changed",
                        event
                            .name
                            .unwrap_or(OsStr::new("unavailable"))
                            .to_str()
                            .unwrap_or("file name with invalid unicode chars")
                    );
                }
                update_apps_list();
            }
        }
    });

    let mut gui = GUI::new();
    gui.listen_for_hotkey();
    println!("[INFO] listening for hotkey...");

    gtk::main();
}
