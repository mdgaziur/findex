use abi_stable::std_types::*;
use findex_plugin::{ApplicationCommand, FResult};
use gtk::gio::AppInfo as GIOAppInfo;
use gtk::prelude::*;
use lazy_static::lazy_static;
use parking_lot::Mutex;
use std::collections::HashMap;

pub type AppInfo = FResult;

lazy_static! {
    pub static ref APPS_LIST: Mutex<Vec<AppInfo>> = Mutex::new(Vec::new());
}

fn appresult_from(app_info: &GIOAppInfo) -> AppInfo {
    let icon = match app_info.icon() {
        Some(icon) => RString::from(IconExt::to_string(&icon).unwrap().to_string()),
        None => RString::from("application-other"),
    };

    AppInfo {
        name: RString::from(app_info.name().to_string()),
        desc: ROption::from(app_info.description().map(|d| RString::from(d.to_string()))),
        cmd: ApplicationCommand::Id(RString::from(app_info.id().unwrap().to_string())),
        icon,
        score: 0,
    }
}

pub fn strip_parameters(cmd: &str) -> String {
    let parameter_regex = regex::Regex::new("%.").unwrap();
    parameter_regex.replace(cmd, "").to_string()
}

pub fn update_apps_list() {
    let list = GIOAppInfo::all()
        .into_iter()
        .filter(|appinfo| appinfo.commandline().is_some())
        .map(|appinfo| (appinfo.name().to_string(), appresult_from(&appinfo)))
        .collect::<HashMap<String, AppInfo>>()
        .iter()
        .map(|value| value.1.clone())
        .collect();

    *APPS_LIST.lock() = list;
}
