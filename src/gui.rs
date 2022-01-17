use std::ops::Deref;
use crate::gui::config::FINDEX_CONFIG;

mod dbus;
mod config;

pub fn init() {
    // deref to make sure it's evaluated
    FINDEX_CONFIG.deref();
}