mod custom_backend;
mod default_backend;

use crate::daemon::config::FINDEX_CONFIG;
use dbus::arg::{Append, Arg, ArgType, IterAppend};
use dbus::Signature;

pub struct FindexBackend {
    backend: Box<dyn Backend>,
}

// FIXME: Not sure if it's really safe
unsafe impl Send for FindexBackend {}

impl FindexBackend {
    pub fn new() -> Result<Self, String> {
        let cfg = FINDEX_CONFIG.lock().unwrap();

        if cfg.custom_backend_loader_path.is_empty() {
            Ok(Self {
                backend: Box::new(default_backend::DefaultBackend::new(None)?),
            })
        } else {
            Ok(Self {
                backend: Box::new(custom_backend::CustomBackend::new(Some(
                    &cfg.custom_backend_loader_path,
                ))?),
            })
        }
    }

    pub fn process_query(&mut self, query: &str) -> Vec<AppInfo> {
        self.backend.process_result(query)
    }
}

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

trait Backend {
    fn new(lib_path: Option<&str>) -> Result<Self, String> where Self: Sized + Send;
    fn process_result(&mut self, query: &str) -> Vec<AppInfo>;
}
