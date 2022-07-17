use crate::app_list::update_apps_list;
use crate::config::FINDEX_CONFIG;
use crate::gui::dialog::show_dialog;
use crate::gui::GUI;
use gtk::gio::{Cancellable, File, FileMonitorFlags};
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
    if let Err(e) = inotify.add_watch(
        "/usr/share/applications",
        WatchMask::CREATE | WatchMask::DELETE | WatchMask::MODIFY,
    ) {
        eprintln!("[WARN] Failed to watch `/usr/share/applications`: {}", e);
    }
    if let Err(e) = inotify.add_watch(
        shellexpand::tilde("~/.local/share/applications").as_ref(),
        WatchMask::all(),
    ) {
        eprintln!(
            "[WARN] Failed to watch `~/.local/share/applications`: {}",
            e
        );
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
