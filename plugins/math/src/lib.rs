use abi_stable::std_types::*;
use findex_plugin::{ApplicationCommand, define_plugin, FResult};

fn init(_: &RHashMap<RString, RString>) -> RResult<(), RString> {
    ROk(())
}

fn handle_query(query: RStr) -> RVec<FResult> {
    let mut context = fend_core::Context::new();

    let Ok(result) = fend_core::evaluate(&format!("@plain_number {}", query), &mut context) else {
        return RVec::new();
    };

    RVec::from(vec![FResult {
        cmd: ApplicationCommand::None,
        icon: RString::from("calc"),
        score: isize::MAX,
        name: RString::from(result.get_main_result()),
        desc: RNone,
    }])
}

define_plugin!("", init, handle_query);