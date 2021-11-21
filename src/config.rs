use lazy_static::lazy_static;

pub struct FindexConfig {
    default_window_width: i32,
    max_content_height: i32,
    max_result_score: i32,
    max_fuzz_distance: i32
}

fn load_settings() -> FindexConfig {
    #[cfg(debug_assertions)]
    let settings_path = "settings.toml";

    #[cfg(not(debug_assertions))]
    let settings_path = shellexpand::tilde("~/.config/findex/settings.toml");

    let file = std::fs::read_to_string(settings_path);
}

lazy_static! {
    static ref FINDEX_CONFIG: FindexConfig = load_settings();
}