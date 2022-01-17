use std::process::exit;
use dbus::arg::{Arg, ArgType, Get, Iter};
use dbus::Signature;
use crate::gui::dbus::get_config;
use lazy_static::lazy_static;

lazy_static! {
    pub static ref FINDEX_CONFIG: FindexConfig = {
        match get_config() {
            Ok(c) => c,
            Err(e) => {
                println!("[Error] Failed to load config: {}", e);
                exit(1);
            }
        }
    };
}

#[derive(Debug, Clone)]
pub struct FindexConfig {
    pub default_window_width: i32,
    pub min_content_height: i32,
    pub max_content_height: i32,
    pub max_name_fuzz_result_score: f64,
    pub max_command_fuzz_result_score: f64,
    pub max_fuzz_distance: i32,
    pub decorate_window: bool,
    pub close_window_on_losing_focus: bool,
    pub query_placeholder: String,
    pub icon_size: i32,
    pub custom_backend_loader_path: String,
}

impl Arg for FindexConfig {
    const ARG_TYPE: ArgType = ArgType::Struct;

    fn signature() -> Signature<'static> {
        Signature::new("(iiiddibbsiss)").unwrap()
    }
}

impl<'a> Get<'a> for FindexConfig {
    fn get(i: &mut Iter<'a>) -> Option<Self> {
        let cfg_tuple: (i32, i32, i32, f64, f64, i32, bool, bool, String, i32, String, String) = i.read().unwrap();

        Some(FindexConfig {
            default_window_width: cfg_tuple.0,
            min_content_height: cfg_tuple.1,
            max_content_height: cfg_tuple.2,
            max_name_fuzz_result_score: cfg_tuple.3,
            max_command_fuzz_result_score: cfg_tuple.4,
            max_fuzz_distance: cfg_tuple.5,
            decorate_window: cfg_tuple.6,
            close_window_on_losing_focus: cfg_tuple.7,
            query_placeholder: cfg_tuple.8,
            icon_size: cfg_tuple.9,
            custom_backend_loader_path: cfg_tuple.10,
        })
    }
}