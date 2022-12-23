use abi_stable::std_types::*;
use gtk::gio::AppInfo as GIOAppInfo;
use parking_lot::Mutex;
use std::collections::HashMap;

use findex_plugin::FResult;
use gtk::prelude::*;
use lazy_static::lazy_static;

pub type AppInfo = FResult;

lazy_static! {
    pub static ref APPS_LIST: Mutex<Vec<AppInfo>> = Mutex::new(Vec::new());
}

fn appresult_from(app_info: &GIOAppInfo) -> AppInfo {
    let cmd = app_info.commandline().unwrap();
    let icon = match app_info.icon() {
        Some(icon) => RString::from(IconExt::to_string(&icon).unwrap().to_string()),
        None => RString::from("application-other"),
    };

    AppInfo {
        name: RString::from(app_info.name().to_string()),
        desc: ROption::from(app_info.description().map(|d| RString::from(d.to_string()))),
        cmd: RString::from(cmd.to_str().unwrap().to_string()),
        icon,
        score: 0,
    }
}

pub fn update_apps_list() {
    let parameter_regex = regex::Regex::new("%.").unwrap();

    let list = GIOAppInfo::all()
        .into_iter()
        .filter(|appinfo| appinfo.commandline().is_some())
        .map(|appinfo| (appinfo.name().to_string(), appresult_from(&appinfo)))
        .collect::<HashMap<String, AppInfo>>()
        .iter()
        .map(|value| value.1.clone())
        .map(|mut appinfo| {
            appinfo.cmd = RString::from(parameter_regex.replace(&appinfo.cmd, ""));

            appinfo
        })
        .collect();

    *APPS_LIST.lock() = list;
}
