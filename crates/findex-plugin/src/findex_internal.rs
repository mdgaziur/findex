use crate::FResult;
use abi_stable::std_types::*;
use gtk::gdk::{EventKey, ModifierType};
use gtk::{accelerator_get_label, accelerator_parse};
use libloading::{Library, Symbol};
use serde::de;
use serde::de::{Deserialize, Deserializer, Error as SerdeError};
use serde::ser::{Serialize, Serializer};
use std::fmt::Formatter;

pub struct PluginDefinition {
    pub plugin: Library,
    pub prefix: RString,
    pub keyboard_shortcut: Option<KeyboardShortcut>,
}

impl PluginDefinition {
    /// Calls the initialization function of plugin
    ///
    /// # Safety
    /// User must ensure that the plugin doesn't violate memory safety
    pub unsafe fn plugin_init(&self, config: &RHashMap<RString, RString>) -> RResult<(), RString> {
        self.plugin
            .get::<Symbol<unsafe extern "C" fn(&RHashMap<RString, RString>) -> RResult<(), RString>>>(
                b"findex_plugin_init",
            )
            .unwrap()(config)
    }

    /// Sends query to plugin and retrieves the result
    ///
    /// # Safety
    /// User must ensure that the plugin doesn't violate memory safety
    pub unsafe fn plugin_query_handler(&self, query: RStr) -> RVec<FResult> {
        self.plugin
            .get::<Symbol<unsafe extern "C" fn(RStr) -> RVec<FResult>>>(
                b"findex_plugin_query_handler",
            )
            .unwrap()(query)
    }
}

/// Loads the plugin and checks for necessary functions and symbols
///
/// # Safety
/// User must ensure that the plugin doesn't violate memory safety
pub unsafe fn load_plugin(plugin_path: &str) -> Result<PluginDefinition, String> {
    let plugin = Library::new(plugin_path).map_err(|e| e.to_string())?;
    let prefix = plugin
        .get::<*const &str>(b"FINDEX_PLUGIN_PREFIX")
        .map_err(|e| e.to_string())?;
    let keyboard_shortcut_accel = plugin
        .get::<*const &str>(b"FINDEX_PLUGIN_KEYBOARD_SHORTCUT")
        .map(|accel| **accel)
        .ok();
    let keyboard_shortcut = if let Some(accel) = keyboard_shortcut_accel {
        Some(
            KeyboardShortcut::from_accelerator(accel)
                .ok_or(format!("plugin provides invalid accelerator: {accel}"))?,
        )
    } else {
        None
    };

    // We don't use them right now, but we need this to check whether the necessary functions exist or not
    plugin
        .get::<Symbol<unsafe extern "C" fn(&RHashMap<RString, RString>) -> bool>>(
            b"findex_plugin_init",
        )
        .map_err(|e| e.to_string())?;
    plugin
        .get::<Symbol<unsafe extern "C" fn(RStr) -> RVec<FResult>>>(b"findex_plugin_query_handler")
        .map_err(|e| e.to_string())?;

    Ok(PluginDefinition {
        prefix: RString::from(**prefix),
        keyboard_shortcut,
        plugin,
    })
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct KeyboardShortcut {
    pub keyval: u32,
    pub modifier: ModifierType,
}

impl KeyboardShortcut {
    pub fn from_eventkey(eventkey: &EventKey) -> Self {
        Self {
            keyval: *eventkey.keyval().to_lower(),
            modifier: Self::clean_modifier_type(eventkey.state()),
        }
    }

    pub fn from_accelerator(accelerator: &str) -> Option<Self> {
        let (keycode, modifier) = accelerator_parse(accelerator);
        if keycode == 0 {
            None
        } else {
            Some(KeyboardShortcut {
                keyval: keycode,
                modifier,
            })
        }
    }

    pub fn clean_modifier_type(mut modifier_type: ModifierType) -> ModifierType {
        modifier_type &= ModifierType::MODIFIER_MASK;
        modifier_type.remove(ModifierType::MOD2_MASK);
        modifier_type.remove(ModifierType::LOCK_MASK);
        modifier_type.remove(ModifierType::MOD4_MASK);

        modifier_type
    }
}

impl Serialize for KeyboardShortcut {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&format!(
            "{}",
            accelerator_get_label(self.keyval, self.modifier).unwrap()
        ))
    }
}

impl<'de> Deserialize<'de> for KeyboardShortcut {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_str(KeyboardShortcutVisitor)
    }
}

struct KeyboardShortcutVisitor;

impl<'de> de::Visitor<'de> for KeyboardShortcutVisitor {
    type Value = KeyboardShortcut;

    fn expecting(&self, formatter: &mut Formatter) -> std::fmt::Result {
        formatter.write_str("expected a string containing valid accelerator")
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: SerdeError,
    {
        KeyboardShortcut::from_accelerator(v).ok_or(E::custom("expected a valid accelerator"))
    }
}
