use crate::daemon::backend::FindexBackend;
use crate::daemon::config::FINDEX_CONFIG;
use crate::daemon::paths::config::get_config;
use crate::daemon::paths::query::get_result;
use dbus::blocking::Connection;
use dbus_crossroads::Crossroads;
use std::sync::Mutex;

pub fn init_daemon() {
    // lock FINDEX_CONFIG to load configs and then drop it
    Mutex::unlock(FINDEX_CONFIG.lock().unwrap());
    let backend = match FindexBackend::new() {
        Ok(b) => b,
        Err(e) => {
            if let Err(_) = native_dialog::MessageDialog::new()
                .set_type(native_dialog::MessageType::Error)
                .set_text(&format!("Failed to initialize backend:\n{}\nFalling back to default backend", e))
                .show_alert() {
                println!("{}", e);
            }

            FINDEX_CONFIG.lock().unwrap().custom_backend_loader_path = String::new(); // forcing default backend
            FindexBackend::new().unwrap() // initializing default backend should never panic
        }
    };


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
    let get_result_method = get_result(&mut crossroads, backend);
    crossroads.insert("/get_config", &[get_config_method], ());
    crossroads.insert("/get_result", &[get_result_method], ());

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
        let r: Result<(String,), dbus::Error> = proxy.method_call(
            "org.freedesktop.DBus",
            "GetNameOwner",
            ("org.findex.daemon",),
        );

        if let Ok(r) = r {
            if con.unique_name().to_string() != r.0 {
                println!("[Info] Daemon replaced with owner: {}", r.0);
                break;
            }
        }
        con.process(std::time::Duration::from_millis(1000))
            .expect("[Error] Failed to process connection");
    }
}