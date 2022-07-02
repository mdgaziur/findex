use gtk::gio::{AppInfo as GIOAppInfo};

use gtk::prelude::*;

pub struct AppInfo {
    pub name: String,
    pub desc: Option<String>,
    pub cmd: String,
    pub icon: String,
    pub score: isize,
    pub id: String,
}

impl From<&GIOAppInfo> for AppInfo {
    fn from(app_info: &GIOAppInfo) -> Self {
        let cmd = app_info.commandline().unwrap();
        let icon = match app_info.icon() {
            Some(icon) => IconExt::to_string(&icon).unwrap().to_string(),
            None => String::from("application-other"),
        };

        Self {
            name: app_info.name().to_string(),
            desc: app_info.description().map(|d| d.to_string()),
            cmd: cmd.to_str().unwrap().to_string(),
            icon,
            score: 0,
            id: app_info.id().unwrap().to_string(),
        }
    }
}
