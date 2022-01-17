mod custom_backend;
mod default_backend;

use crate::daemon::config::FINDEX_CONFIG;
use findex::AppInfo;

pub struct FindexBackend {
    backend: Box<dyn Backend>,
}

impl FindexBackend {
    pub fn new() -> Result<Self, String> {
        if FINDEX_CONFIG.custom_backend_loader_path.is_empty() {
            Ok(Self {
                backend: Box::new(default_backend::DefaultBackend::new(None)?),
            })
        } else {
            Ok(Self {
                backend: Box::new(custom_backend::CustomBackend::new(Some(
                    &FINDEX_CONFIG.custom_backend_loader_path,
                ))?),
            })
        }
    }

    pub fn process_query(&mut self, query: &str) -> Vec<AppInfo> {
        self.backend.process_result(query)
    }
}

trait Backend: Send {
    fn new(lib_path: Option<&str>) -> Result<Self, String>
    where
        Self: Sized + Send;
    fn process_result(&mut self, query: &str) -> Vec<AppInfo>;
}
