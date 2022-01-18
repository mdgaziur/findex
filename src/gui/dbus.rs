use std::time::Duration;
use dbus::blocking::Connection;
use findex::AppInfo;
use crate::gui::config::FindexConfig;

pub fn get_config() -> Result<FindexConfig, String> {
    let conn = Connection::new_session()
        .map_err(|e| e.to_string())?;

    let proxy = conn.with_proxy("org.findex.daemon", "/get_config", Duration::from_secs(1));

    let (config,): (FindexConfig,) = proxy.method_call("org.findex.daemon.config", "get_config", ())
        .map_err(|e| e.to_string())?;

    Ok(config)
}

pub fn get_all() -> Result<Vec<AppInfo>, String> {
    let conn = Connection::new_session()
        .map_err(|e| e.to_string())?;

    let proxy = conn.with_proxy("org.findex.daemon", "/get_all", Duration::from_secs(1));

    let (result,): (Vec<AppInfo>,) = proxy.method_call("org.findex.daemon.query", "get_all", ())
        .map_err(|e| e.to_string())?;

    Ok(result)
}

pub fn get_result(query: &str) -> Result<Vec<AppInfo>, String> {
    let conn = Connection::new_session()
        .map_err(|e| e.to_string())?;

    let proxy = conn.with_proxy("org.findex.daemon", "/get_result", Duration::from_secs(1));

    let (result,): (Vec<AppInfo>,) = proxy.method_call("org.findex.daemon.query", "get_result", (query,))
        .map_err(|e| e.to_string())?;

    Ok(result)
}