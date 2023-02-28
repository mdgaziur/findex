#[cfg(feature = "findex_internals")]
pub mod findex_internal;

use abi_stable::std_types::*;

/// This struct is used to represent results by plugins and internal code of Findex.
#[derive(Clone)]
#[repr(C)]
pub struct FResult {
    /// Name of the result
    pub name: RString,
    /// Optional description of the result
    pub desc: ROption<RString>,
    /// The command to execute when the user presses enter
    pub cmd: ApplicationCommand,
    /// The icon of the result
    pub icon: RString,
    /// Score of the result. This will be used to sort multiple results
    pub score: isize,
}

#[derive(Clone)]
#[repr(C)]
pub enum ApplicationCommand {
    /// Exact command to execute
    Command(RString),
    /// AppId of GIO AppInfo
    Id(RString),
}

/// This macro is used to define a Findex plugin.
///
/// Example usage:
/// ```rust
/// use findex_plugin::{define_plugin, FResult};
/// use abi_stable::std_types::*;
///
/// fn init(config: &RHashMap<RString, RString>) -> RResult<(), RString>  {
///     // Set up your plugin using the config if necessary
///     // Return RErr if something went wrong
///
///     // Returning this indicates that the plugin initalization is successful
///     ROk(())
/// }
///
/// fn handle_query(query: RStr) -> RVec<FResult> {
///     let mut result = vec![];
///
///     /* Do stuff here */
///
///     RVec::from(result)
/// }
///
/// define_plugin!("prefix!", init, handle_query);
/// ```
///
/// Refer to the `README.md` of this crate for more detailed explanation
#[macro_export]
macro_rules! define_plugin {
    ($prefix:literal, $init_function:ident, $query_handler:ident) => {
        #[no_mangle]
        #[used]
        pub static FINDEX_PLUGIN_PREFIX: &'static str = $prefix;

        #[no_mangle]
        extern "C" fn findex_plugin_init(
            config: &RHashMap<RString, RString>,
        ) -> RResult<(), RString> {
            $init_function(config)
        }

        #[no_mangle]
        extern "C" fn findex_plugin_query_handler(query: RStr) -> RVec<FResult> {
            $query_handler(query)
        }
    };
}
