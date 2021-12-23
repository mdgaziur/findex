use crate::daemon::paths::config::get_config;
use dbus::blocking::Connection;
use dbus::message::MatchRule;
use dbus::{arg, Message};
use dbus_crossroads::Crossroads;
use std::process::exit;

pub fn init_daemon() {
    let con = Connection::new_session().expect("[Error] Failed to create new D-Bus session");
    con.request_name("org.findex.daemon", true, true, false)
        .expect("[Error] Failed to request name on D-Bus");
    let proxy = con.with_proxy(
        "org.freedesktop.DBus",
        "/",
        std::time::Duration::from_millis(1000),
    );

    let mut crossroads = Crossroads::new();

    let get_config_method = get_config(&mut crossroads);
    crossroads.insert("/get_config", &[get_config_method], ());

    println!("[Info] Serving clients...");

    use dbus::channel::MatchingReceiver;
    con.start_receive(
        dbus::message::MatchRule::new_method_call(),
        Box::new(move |msg, conn| {
            crossroads.handle_message(msg, conn).unwrap();
            true
        }),
    );

    loop {
        let r: (String,) = proxy
            .method_call(
                "org.freedesktop.DBus",
                "GetNameOwner",
                ("org.findex.daemon",),
            )
            .unwrap();
        if con.unique_name().to_string() != r.0 {
            println!("[Info] Daemon replaced with owner: {}", r.0);
            break;
        }
        con.process(std::time::Duration::from_millis(1000))
            .expect("[Error] Failed to process connection");
    }
}
