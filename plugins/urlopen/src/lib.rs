use findex_plugin::{define_plugin, FResult};
use abi_stable::std_types::*;

fn init(_: &RHashMap<RString, RString>) -> RResult<(), RString>  {
    ROk(())
}

fn handle_query(query: RStr) -> RVec<FResult> {
    if query == "" {
        return RVec::new();
    }

    RVec::from(vec![FResult {
        cmd: RString::from(format!("xdg-open \"{query}\"")),
        name: RString::from(format!("Open {query}")),
        desc: RNone,
        score: isize::MAX,
        icon: RString::from("browser"),
    }])
}

define_plugin!("url!", init, handle_query);
