#[cfg(feature = "findex_internals")]
pub mod findex_internal;

use abi_stable::std_types::{ROption, RString};

#[derive(Clone)]
pub struct FResult {
    pub name: RString,
    pub desc: ROption<RString>,
    pub cmd: RString,
    pub icon: RString,
    pub score: isize,
}

#[macro_export]
macro_rules! define_plugin {
    ($prefix:literal, $init_function:ident, $query_handler:ident) => {
        #[no_mangle]
        #[used]
        pub static FINDEX_PLUGIN_PREFIX: &'static str = $prefix;

        #[no_mangle]
        extern "C" fn findex_plugin_init(config: &RHashMap<RString, RString>) -> bool {
            $init_function(config)
        }

        #[no_mangle]
        extern "C" fn findex_plugin_query_handler(query: RStr) -> RVec<FResult> {
            $query_handler(query)
        }
    };
}
