use dbus_crossroads::{Crossroads, IfaceToken};
use crate::SHOW_GUI;

pub fn show_gui(crossroads: &mut Crossroads) -> IfaceToken<()> {
    crossroads.register("org.findex.daemon.gui", |builder| {
        builder.method("show_gui", (), (), move |_, (), ()| {
            let mut show_gui = SHOW_GUI.lock().unwrap();
            *show_gui = !*show_gui;
            Ok(())
        });
    })
}
