use crate::gui::GUI;
use std::sync::Mutex;

mod config;
mod gui;

static SHOW_WINDOW: Mutex<bool> = Mutex::new(false);

fn main() {
    println!("[INFO] Starting Findex...");
    gtk::init().expect("Failed to init GTK");

    let mut gui = GUI::new();
    gui.listen_for_hotkey();
    println!("[INFO] listening for hotkey...");

    gtk::main();
}
