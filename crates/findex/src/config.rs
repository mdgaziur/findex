use crate::gui::dialog::show_dialog;
use abi_stable::std_types::*;
use findex_plugin::findex_internal::{load_plugin, KeyboardShortcut, PluginDefinition};
use gtk::MessageType;
use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

lazy_static! {
    pub static ref FINDEX_CONFIG: FindexConfig = {
        let settings = load_settings();
        if let Err(e) = settings {
            FindexConfig {
                error: RString::from(e),
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
    pub name_match_highlight_color: RString,
    pub decorate_window: bool,
    pub close_window_on_losing_focus: bool,
    pub query_placeholder: RString,
    pub icon_size: i32,
    pub toggle_key: RString,
    pub min_score: isize,
    pub result_size: usize,
    pub plugins: HashMap<RString, Plugin>,
    #[serde(skip)]
    pub error: RString,
    /// This should get filled after configuration gets initialized
    #[serde(skip)]
    pub plugin_definitions: HashMap<RString, PluginDefinition>,
}

#[derive(Serialize, Deserialize)]
pub struct Plugin {
    pub prefix: Option<RString>,
    pub keyboard_shortcut: Option<KeyboardShortcut>,
    pub path: RString,
    pub config: RHashMap<RString, RString>,
}

impl Default for FindexConfig {
    fn default() -> Self {
        FindexConfig {
            min_content_height: 0,
            max_content_height: 400,
            default_window_width: 600,
            name_match_highlight_color: RString::from("orange"),
            decorate_window: false,
            query_placeholder: RString::from("Search for applications"),
            close_window_on_losing_focus: true,
            icon_size: 32,
            min_score: 5,
            result_size: 10,
            toggle_key: RString::from("<Shift>space"),
            error: RString::new(),
            plugins: HashMap::new(),
            plugin_definitions: HashMap::new(),
        }
    }
}

fn load_settings() -> Result<FindexConfig, String> {
    #[cfg(debug_assertions)]
    use std::path::PathBuf;

    #[cfg(debug_assertions)]
    let settings_path = PathBuf::from("settings.toml");

    #[cfg(not(debug_assertions))]
    let settings_dir = xdg::BaseDirectories::new()
        .expect("Failed to get XDG base directories")
        .create_config_directory("findex")
        .expect("Failed to create config directory");

    #[cfg(not(debug_assertions))]
    let settings_path = settings_dir.join("settings.toml");

    let file = std::path::Path::new(&settings_path);
    let mut res = if !file.exists() {
        #[cfg(not(debug_assertions))]
        if !std::path::Path::new(&settings_dir).exists() {
            std::fs::create_dir(&settings_dir).unwrap();
        }

        let settings = toml::to_string(&FindexConfig::default()).unwrap();
        std::fs::write(settings_path, settings).unwrap();

        Ok(FindexConfig::default())
    } else {
        let settings = std::fs::read_to_string(&settings_path).unwrap();

        toml::from_str(&settings).map_err(|e| format!("Error while parsing settings: {e}"))
    };

    if let Ok(ref mut config) = res {
        for (name, plugin) in &mut config.plugins {
            let mut plugin_definition =
                match unsafe { load_plugin(&shellexpand::tilde(&plugin.path)) } {
                    Ok(pd) => pd,
                    Err(e) => {
                        show_dialog(
                            "Error",
                            &format!("Failed to load plugin {name}: {e}"),
                            MessageType::Error,
                        );
                        continue;
                    }
                };

            if !plugin.config.contains_key("highlight_color") {
                plugin.config.insert(
                    RString::from("highlight_color"),
                    config.name_match_highlight_color.clone(),
                );
            }
            let init_result = unsafe { plugin_definition.plugin_init(&plugin.config) };
            if let RErr(e) = init_result {
                show_dialog(
                    "Error",
                    &format!("Plugin \"{name}\" failed to initialize: {e}"),
                    MessageType::Error,
                );
                continue;
            }

            if let Some(prefix) = &plugin.prefix {
                plugin_definition.prefix = prefix.clone();
            }

            if plugin.keyboard_shortcut.is_some() {
                plugin_definition.keyboard_shortcut = plugin.keyboard_shortcut;
            }

            config.plugin_definitions.insert(
                plugin
                    .prefix
                    .as_ref()
                    .cloned()
                    .unwrap_or_else(|| plugin_definition.prefix.clone()),
                plugin_definition,
            );
        }
    }

    res
}
