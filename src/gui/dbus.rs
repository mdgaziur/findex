use std::time::Duration;
use dbus::blocking::Connection;
use crate::gui::config::FindexConfig;

pub fn get_config() -> Result<FindexConfig, String> {
    let conn = Connection::new_session()
        .map_err(|e| e.to_string())?;

    let proxy = conn.with_proxy("org.findex.daemon", "/get_config", Duration::from_secs(1));

    let (config,): (FindexConfig,) = proxy.method_call("org.findex.daemon.config", "get_config", ())
        .map_err(|e| e.to_string())?;

    Ok(config)
}