use findex_plugin::{define_plugin, FResult};
use abi_stable::std_types::{RHashMap, ROption, RStr, RString, RVec};

fn init(_: &RHashMap<RString, RString>) -> bool {
    true
}

fn handle_query(query: RStr) -> RVec<FResult> {
    if query == "" {
        return RVec::new();
    }

    RVec::from(vec![FResult {
        cmd: RString::from(format!("xdg-open {query}")),
        name: RString::from(format!("Open {query}")),
        desc: ROption::RNone,
        score: isize::MAX,
        icon: RString::from("browser"),
    }])
}

define_plugin!("url!", init, handle_query);
