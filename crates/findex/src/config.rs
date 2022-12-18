use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};

lazy_static! {
    pub static ref FINDEX_CONFIG: FindexConfig = {
        let settings = load_settings();
        if let Err(e) = settings {
            FindexConfig {
                error: e,
                ..Default::default()
            }
        } else {
            settings.unwrap()
        }
    };
}

#[derive(Serialize, Deserialize)]
#[serde(default)]
pub struct FindexConfig {
    pub default_window_width: i32,
    pub min_content_height: i32,
    pub max_content_height: i32,
    pub name_match_highlight_color: String,
    pub decorate_window: bool,
    pub close_window_on_losing_focus: bool,
    pub query_placeholder: String,
    pub icon_size: i32,
    pub toggle_key: String,
    pub min_score: isize,
    pub result_size: usize,
    #[serde(skip)]
    pub error: String,
}

fn default_placeholder() -> String {
    String::from("Search for applications")
}

impl Default for FindexConfig {
    fn default() -> Self {
        FindexConfig {
            min_content_height: 0,
            max_content_height: 400,
            default_window_width: 600,
            name_match_highlight_color: String::from("orange"),
            decorate_window: false,
            query_placeholder: default_placeholder(),
            close_window_on_losing_focus: true,
            icon_size: 32,
            min_score: 5,
            result_size: 10,
            toggle_key: String::from("<Shift>space"),
            error: String::new(),
        }
    }
}

fn load_settings() -> Result<FindexConfig, String> {
    #[cfg(debug_assertions)]
    let settings_path = String::from("settings.toml");

    #[cfg(not(debug_assertions))]
    let settings_path = shellexpand::tilde("~/.config/findex/settings.toml").to_string();

    #[cfg(not(debug_assertions))]
    let settings_dir = shellexpand::tilde("~/.config/findex").to_string();

    let file = std::path::Path::new(&settings_path);
    if !file.exists() {
        #[cfg(not(debug_assertions))]
        if !std::path::Path::new(&settings_dir).exists() {
            std::fs::create_dir(&settings_dir).unwrap();
        }

        let settings = toml::to_string(&FindexConfig::default()).unwrap();
        std::fs::write(settings_path, settings).unwrap();

        Ok(FindexConfig::default())
    } else {
        let settings = std::fs::read_to_string(settings_path).unwrap();

        let config =
            toml::from_str(&settings).map_err(|e| format!("Error while parsing settings: {e}"))?;

        Ok(config)
    }
}
