use lazy_static::lazy_static;
use rustbreak::{deser::Bincode, MemoryDatabase};
use serde::{Deserialize, Serialize};

lazy_static! {
    static ref DB: MemoryDatabase<Vec<AppInfo>, Bincode> =
        MemoryDatabase::memory(Vec::new()).unwrap();
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppInfo {
    pub name: String,
    pub exec: String,
    pub icon: String,
}
