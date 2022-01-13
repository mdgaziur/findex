mod backend;
mod config;
mod db;
mod init;
mod paths;

#[cfg(debug_assertions)]
pub fn launch_daemon() {
    init::init_daemon();
}

#[cfg(not(debug_assertions))]
pub fn launch_daemon() {
    use daemonize::Daemonize;
    use std::fs::File;

    let stdout = File::create("/tmp/findex-daemon.out").unwrap();
    let stderr = File::create("/tmp/findex-daemon.err").unwrap();

    let daemon = Daemonize::new().stdout(stdout).stderr(stderr);

    match daemon.start() {
        Ok(()) => init::init_daemon(),
        Err(e) => eprintln!("[Error] Failed to start daemon: {}", e),
    }
}
