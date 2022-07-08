use crate::config::FINDEX_CONFIG;
use crate::gui::dialog::show_dialog;
use crate::gui::GUI;
use gtk::MessageType;
use parking_lot::Mutex;

mod app_list;
mod config;
mod gui;

fn main() {
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
