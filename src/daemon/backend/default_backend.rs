use crate::daemon::backend::{AppInfo, Backend};

pub struct DefaultBackend {}

impl Backend for DefaultBackend {
    fn new(_lib_path: Option<&str>) -> Result<Self, String> {
        Ok(DefaultBackend {})
    }

    fn process_result(&mut self, _query: &str) -> Vec<AppInfo> {
        todo!()
    }
}
