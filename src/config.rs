use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct FindexConfig {
    pub default_window_width: i32,
    pub min_content_height: i32,
    pub max_content_height: i32,
    pub max_name_fuzz_result_score: f64,
    pub max_command_fuzz_result_score: f64,
    pub max_fuzz_distance: i32,
    pub decorate_window: bool,
    pub close_window_on_losing_focus: bool,
    #[serde(default = "default_placeholder")]
    pub query_placeholder: String,
    #[serde(skip)]
    pub error: String, // a nasty hack to check if there's an error while parsing settings.toml
}

fn default_placeholder() -> String {
    String::from("Search for applications")
}

impl FindexConfig {
    pub fn default() -> Self {
        FindexConfig {
            min_content_height: 400,
            max_content_height: 400,
            default_window_width: 600,
            max_fuzz_distance: 80,
            max_name_fuzz_result_score: 0.4,
            max_command_fuzz_result_score: 0.4,
            decorate_window: false,
            query_placeholder: default_placeholder(),
            close_window_on_losing_focus: true,
            error: String::new(),
        }
    }
}

fn load_settings() -> Result<FindexConfig, String> {
    #[cfg(debug_assertions)]
    let settings_path = "settings.toml";

    #[cfg(not(debug_assertions))]
    let settings_path = shellexpand::tilde("~/.config/findex/settings.toml");

    let settings_dir = shellexpand::tilde("~/.config/findex");

    let file = std::path::Path::new(&*settings_path);
    if !file.exists() {
        if !std::path::Path::new(&*settings_dir).exists() {
            std::fs::create_dir(&*settings_dir).unwrap();
        }

        let settings = toml::to_string(&FindexConfig::default()).unwrap();
        std::fs::write(&*settings_path, settings).unwrap();

        Ok(FindexConfig::default())
    } else {
        let settings = std::fs::read_to_string(&*settings_path).unwrap();

        let config = toml::from_str(&settings).map_err(|e| e.to_string())?;
        Ok(config)
    }
}

lazy_static! {
    pub static ref FINDEX_CONFIG: FindexConfig = {
        let settings = load_settings();
        if let Err(e) = settings {
            let mut default_settings = FindexConfig::default();
            default_settings.error = e.to_string();

            default_settings
        } else {
            settings.unwrap()
        }
    };
}
