use crate::config::FINDEX_CONFIG;
use crate::gui::dialog::show_dialog;
use crate::gui::GUI;
use gtk::{glib, MessageType};
use parking_lot::Mutex;
use crate::app_list::{app_list_updater, AppInfo, update_app_list};

mod config;
mod gui;
mod app_list;

static SHOW_WINDOW: Mutex<bool> = Mutex::new(false);
static APP_LIST: Mutex<Vec<AppInfo>> = Mutex::new(Vec::new());

fn main() {
    println!("[INFO] Starting Findex...");
    gtk::init().expect("Failed to init GTK");
    if !FINDEX_CONFIG.error.is_empty() {
        show_dialog("Warning", &FINDEX_CONFIG.error, MessageType::Warning);
    }
    update_app_list();
    glib::idle_add(app_list_updater);

    let mut gui = GUI::new();
    gui.listen_for_hotkey();
    println!("[INFO] listening for hotkey...");

    gtk::main();
}
