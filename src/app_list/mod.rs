use std::collections::HashMap;
use gtk::gio::AppInfo as GIOAppInfo;
use parking_lot::Mutex;

use gtk::prelude::*;
use lazy_static::lazy_static;

lazy_static! {
    pub static ref APPS_LIST: Mutex<Vec<AppInfo>> = Mutex::new(Vec::new());
}

#[derive(Clone)]
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

pub fn update_apps_list() {
    let parameter_regex = regex::Regex::new("%.")
        .unwrap();

    let list = GIOAppInfo::all()
        .into_iter()
        .filter(|appinfo| appinfo.commandline().is_some())
        .map(|appinfo| (appinfo.name().to_string(), AppInfo::from(&appinfo)))
        .collect::<HashMap<String, AppInfo>>()
        .iter()
        .map(|value| value.1.clone())
        .map(|mut appinfo| {
            appinfo.cmd = parameter_regex.replace(&appinfo.cmd, "").to_string();

            appinfo
        })
        .collect();

    *APPS_LIST.lock() = list;
}
