use dbus::blocking::Connection;
use dbus_crossroads::Crossroads;
use crate::daemon::paths::config::get_config;

pub fn init_daemon() {
    let con = Connection::new_session().expect("Failed to create new D-Bus session");
    con.request_name("org.findex.daemon", false, false, true)
        .expect("[Error] Failed to request name on D-Bus");

    let mut crossroads = Crossroads::new();

    let get_config_method = get_config(&mut crossroads);
    crossroads.insert("/get_config", &[get_config_method], ());

    println!("[Info] Serving clients...");
    crossroads.serve(&con)
        .expect("[Error] Error occurred while serving clients");
}
