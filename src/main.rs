


use crate::config::FINDEX_CONFIG;
use crate::gui::dialog::show_dialog;
use crate::gui::GUI;
use gtk::MessageType;

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

    let mut gui = GUI::new();
    gui.listen_for_hotkey();
    println!("[INFO] listening for hotkey...");

    gtk::main();
}
