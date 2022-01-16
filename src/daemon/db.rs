use freedesktop_entry_parser::Entry;
use lazy_static::lazy_static;
use rustbreak::{deser::Bincode, MemoryDatabase};
use serde::{Deserialize, Serialize};

lazy_static! {
    pub static ref DB: MemoryDatabase<Vec<AppInfo>, Bincode> =
        MemoryDatabase::memory(Vec::new()).unwrap();
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppInfo {
    pub name: String,
    pub exec: String,
    pub icon: String,
}

impl AppInfo {
    pub(crate) fn from_freedesktop_entry(entry: &Entry) -> Result<Self, &str> {
        let parameter_regex = regex::Regex::new("%.").unwrap();
        let section = entry.section("Desktop Entry");
        let name = match section.attr("Name") {
            Some(n) => n.to_string(),
            None => {
                return Err("Cannot find 'Name' field")
            }
        };
        let icon = section.attr("Icon").unwrap_or("applications-other").to_string();
        let exec = match section.attr("Exec") {
            Some(e) => parameter_regex.replace_all(e, "").to_string(),
            None => return Err("Cannot find 'Exec' field"),
        };

        Ok(Self {
            name,
            exec,
            icon
        })
    }
}