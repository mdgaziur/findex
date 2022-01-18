use crate::daemon::backend::FindexBackend;
use crate::daemon::paths::config::get_config;
use crate::daemon::paths::query::{get_all, get_result};
use dbus::blocking::Connection;
use dbus_crossroads::Crossroads;
use crate::daemon::config::FINDEX_CONFIG;

pub fn init_daemon() {
    let backend = match FindexBackend::new(&FINDEX_CONFIG.custom_backend_loader_path) {
        Ok(b) => b,
        Err(e) => {
            if let Err(_) = native_dialog::MessageDialog::new()
                .set_type(native_dialog::MessageType::Error)
                .set_text(&format!(
                    "Failed to initialize backend:\n{}\nFalling back to default backend",
                    e
                ))
                .show_alert()
            {
                println!("{}", e);
            }

            FindexBackend::new("").unwrap() // initializing default backend should never panic
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
    let get_result_method = get_result(&mut crossroads, backend.clone());
    let get_all_method = get_all(&mut crossroads, backend.clone());
    crossroads.insert("/get_config", &[get_config_method], ());
    crossroads.insert("/get_result", &[get_result_method], ());
    crossroads.insert("/get_all", &[get_all_method], ());

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
