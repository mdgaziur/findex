use crate::daemon::config::FINDEX_CONFIG;
use dbus_crossroads::{Crossroads, IfaceToken};

pub fn get_config(crossroads: &mut Crossroads) -> IfaceToken<()> {
    crossroads.register("org.findex.daemon.config", |builder| {
        builder.method("get_config", (), ("config",), move |_, (), ()| {
            let cfg = FINDEX_CONFIG.clone();
            Ok((cfg,))
        });
    })
}
