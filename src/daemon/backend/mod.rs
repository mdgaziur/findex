mod custom_backend;
mod default_backend;

use dyn_clone::DynClone;
use findex::AppInfo;

#[derive(Clone)]
pub struct FindexBackend {
    backend: Box<dyn Backend>,
}

impl FindexBackend {
    pub fn new(backend_loader_path: &str) -> Result<Self, String> {
        if backend_loader_path.is_empty() {
            Ok(Self {
                backend: Box::new(default_backend::DefaultBackend::new(None)?),
            })
        } else {
            Ok(Self {
                backend: Box::new(custom_backend::CustomBackend::new(Some(
                    backend_loader_path,
                ))?),
            })
        }
    }

    pub fn process_query(&mut self, query: &str) -> Vec<AppInfo> {
        self.backend.process_result(query)
    }

    pub fn get_all(&mut self) -> Vec<AppInfo> {
        self.backend.get_all()
    }
}

trait Backend: Send + DynClone {
    fn new(lib_path: Option<&str>) -> Result<Self, String>
        where
            Self: Sized + Send + Clone;
    fn process_result(&mut self, query: &str) -> Vec<AppInfo>;
    fn get_all(&mut self) -> Vec<AppInfo>;
}

dyn_clone::clone_trait_object!(Backend);