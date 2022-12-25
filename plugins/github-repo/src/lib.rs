use abi_stable::std_types::*;
use regex::Regex;
use findex_plugin::{ApplicationCommand, define_plugin, FResult};

fn init(_: &RHashMap<RString, RString>) -> RResult<(), RString> {
    ROk(())
}

fn handle_query(query: RStr) -> RVec<FResult> {
    let regex = Regex::new("[a-zA-Z0-9]+/[a-zA-Z0-9]+")
        .unwrap();

    if !regex.is_match(query.as_str()) {
        return RVec::new();
    }

    RVec::from(vec![FResult {
        cmd: ApplicationCommand::Command(RString::from(format!("xdg-open https://github.com/{query}"))),
        icon: RString::from("github"),
        score: isize::MAX,
        name: RString::from("Open github repository"),
        desc: RNone,
    }])
}

define_plugin!("github!", init, handle_query);