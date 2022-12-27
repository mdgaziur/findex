use findex_plugin::{ApplicationCommand, define_plugin, FResult};
use abi_stable::std_types::*;

fn init(_: &RHashMap<RString, RString>) -> RResult<(), RString>  {
    ROk(())
}

fn handle_query(query: RStr) -> RVec<FResult> {
    if query == "" {
        return RVec::new();
    }

    RVec::from(vec![FResult {
        cmd: ApplicationCommand::Command(RString::from(format!("{query}"))),
        name: RString::from(format!("Run \"{query}\"")),
        desc: RNone,
        score: isize::MAX,
        icon: RString::from("terminal"),
    }])
}

define_plugin!("cmd!", init, handle_query);
