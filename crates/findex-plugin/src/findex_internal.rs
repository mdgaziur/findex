use crate::FResult;
use abi_stable::std_types::{RHashMap, RResult, RStr, RString, RVec};
use libloading::{Error, Library, Symbol};

pub struct PluginDefinition {
    pub plugin: Library,
    pub prefix: RString,
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
pub unsafe fn load_plugin(plugin_path: &str) -> Result<PluginDefinition, Error> {
    let plugin = libloading::Library::new(plugin_path)?;
    let prefix = plugin.get::<*const &str>(b"FINDEX_PLUGIN_PREFIX")?;

    // We don't use them right now, but we need this to check whether the necessary functions exist or not
    plugin.get::<Symbol<unsafe extern "C" fn(&RHashMap<RString, RString>) -> bool>>(
        b"findex_plugin_init",
    )?;
    plugin.get::<Symbol<unsafe extern "C" fn(RStr) -> RVec<FResult>>>(
        b"findex_plugin_query_handler",
    )?;

    Ok(PluginDefinition {
        prefix: RString::from(**prefix),
        plugin,
    })
}
