use dbus::arg::{Append, Arg, ArgType, IterAppend};
use dbus::Signature;
use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};
use std::sync::Mutex;

#[derive(Serialize, Deserialize, Clone)]
#[serde(default)]
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
    #[serde(skip)]
    pub error: String,
}

impl Arg for FindexConfig {
    const ARG_TYPE: ArgType = ArgType::Struct;

    fn signature() -> Signature<'static> {
        Signature::new("(iiiddibbsis)").unwrap()
    }
}

impl Append for FindexConfig {
    fn append_by_ref(&self, ia: &mut IterAppend) {
        self.default_window_width.append(ia);
        self.min_content_height.append(ia);
        self.max_content_height.append(ia);
        self.max_name_fuzz_result_score.append(ia);
        self.max_command_fuzz_result_score.append(ia);
        self.max_fuzz_distance.append(ia);
        self.decorate_window.append(ia);
        self.close_window_on_losing_focus.append(ia);
        self.query_placeholder.as_str().append(ia);
        self.icon_size.append(ia);
        self.error.as_str().append(ia);
    }
}

fn default_placeholder() -> String {
    String::from("Search for applications")
}

impl Default for FindexConfig {
    fn default() -> Self {
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
            icon_size: 32,
            error: String::new(),
        }
    }
}

fn load_settings() -> Result<FindexConfig, String> {
    #[cfg(debug_assertions)]
    let settings_path = "settings.toml";

    #[cfg(not(debug_assertions))]
    let settings_path = shellexpand::tilde("~/.config/findex/settings.toml");

    #[cfg(not(debug_assertions))]
    let settings_dir = shellexpand::tilde("~/.config/findex");

    let file = std::path::Path::new(&*settings_path);
    if !file.exists() {
        #[cfg(not(debug_assertions))]
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
    pub static ref FINDEX_CONFIG: Mutex<FindexConfig> = Mutex::new({
        let settings = load_settings();
        if let Err(e) = settings {
            FindexConfig {
                error: e,
                ..Default::default()
            }
        } else {
            settings.unwrap()
        }
    });
}
