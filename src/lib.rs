use dbus::arg::{Append, Arg, ArgType, IterAppend};
use dbus::Signature;

#[derive(Debug, Clone)]
pub struct AppInfo {
    pub total_score: f64,
    pub name: String,
    pub exec: String,
    pub icon: String,
}

impl Arg for AppInfo {
    const ARG_TYPE: ArgType = ArgType::Struct;

    fn signature() -> Signature<'static> {
        Signature::new("(dsss)").unwrap()
    }
}

impl Append for AppInfo {
    fn append_by_ref(&self, ia: &mut IterAppend) {
        (self.total_score, &self.name, &self.exec, &self.icon).append(ia);
    }
}