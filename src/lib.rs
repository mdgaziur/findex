use dbus::arg::{Append, Arg, ArgType, Get, Iter, IterAppend};
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

impl<'a> Get<'a> for AppInfo {
    fn get(i: &mut Iter<'a>) -> Option<Self> {
        let tuple: (f64, String, String, String) = i.read().ok()?;

        Some(Self {
            total_score: tuple.0,
            name: tuple.1,
            exec: tuple.2,
            icon: tuple.3
        })
    }
}

impl Append for AppInfo {
    fn append_by_ref(&self, ia: &mut IterAppend) {
        (self.total_score, &self.name, &self.exec, &self.icon).append(ia);
    }
}