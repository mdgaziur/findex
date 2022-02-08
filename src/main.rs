mod daemon;
mod gui;

use std::sync::Mutex;
use lazy_static::lazy_static;

lazy_static! {
    static ref SHOW_GUI: Mutex<bool> = Mutex::new(false);
}

fn main() {
    daemon::launch_daemon();
}
