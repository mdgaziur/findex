use daemonize::Daemonize;
use inotify::{Inotify, WatchMask};
use nix::libc::time_t;
use nix::time::ClockId;
use shellexpand::tilde;
use std::ffi::OsStr;
use std::fs::{create_dir, File};
use std::path::Path;
use std::time::Duration;
use subprocess::{ExitStatus, Popen, PopenConfig, Redirection};
use sysinfo::{ProcessRefreshKind, RefreshKind, System, SystemExt};

fn findex_daemon(current_time: time_t) {
    fn spawn_findex(findex_output: File) -> Popen {
        Popen::create(
            &["findex"],
            PopenConfig {
                stdout: Redirection::File(findex_output),
                ..Default::default()
            },
        )
        .expect("Failed to spawn Findex")
    }

    let findex_output = File::create(&*tilde(&format!(
        "~/.findex-logs/findex-{current_time}.log"
    )))
    .expect("Failed to create file to store findex output");

    let mut findex_process = spawn_findex(findex_output.try_clone().unwrap());

    let mut inotify = Inotify::init().expect("Failed to init inotify");
    let watch_mask = WatchMask::CREATE | WatchMask::MODIFY | WatchMask::MOVE | WatchMask::DELETE;
    inotify
        .watches()
        .add(&*tilde("~/.config/findex/"), watch_mask)
        .expect("Failed to watch `~/.config/findex/`");
    loop {
        if let Ok(Some(exit_status)) = findex_process.wait_timeout(Duration::from_millis(500)) {
            eprint!("[WARN] Findex exited unexpectedly");
            match exit_status {
                ExitStatus::Exited(code) => eprintln!("with exit code: {code}"),
                ExitStatus::Signaled(signal) => eprintln!(" with signal: {signal}"),
                _ => eprintln!(),
            }

            println!("[INFO] Respawning Findex");

            findex_process = spawn_findex(findex_output.try_clone().unwrap());
        }

        let mut buf = [0; 1024];
        match inotify.read_events(&mut buf) {
            Ok(mut events) => {
                if let Some(event) = events.next() {
                    if event.name == Some(OsStr::new("toggle_file")) && events.next().is_none() {
                        continue;
                    }

                    println!("[INFO] configs changed, restarting findex");
                    if findex_process.poll().is_none() {
                        findex_process.kill().unwrap();
                    }
                    findex_process = spawn_findex(findex_output.try_clone().unwrap());
                }
            }
            Err(e) => println!("[WARN] Inotify error: {e}"),
        }
    }
}

fn main() {
    if System::new_with_specifics(RefreshKind::new().with_processes(ProcessRefreshKind::new()))
        .processes_by_exact_name("findex")
        .count()
        > 0
    {
        eprintln!("[ERROR] Findex is already running");
        eprintln!("[ERROR] Help: You may want to kill with `killall findex findex-daemon`");

        return;
    }

    let current_time = nix::time::clock_gettime(ClockId::CLOCK_REALTIME)
        .unwrap()
        .tv_sec();

    if !Path::new(&*tilde("~/.findex-logs")).is_dir() {
        create_dir(&*tilde("~/.findex-logs")).unwrap();
    }

    let logfile = File::create(&*tilde(&format!(
        "~/.findex-logs/findex-daemon-{current_time}.log"
    )))
    .expect("Failed to create file to store logs");

    let daemon = Daemonize::new()
        .stdout(logfile.try_clone().unwrap())
        .stderr(logfile.try_clone().unwrap());

    match daemon.start() {
        Ok(_) => findex_daemon(current_time),
        Err(e) => eprintln!("[ERROR] Failed to start findex daemon: {e}"),
    }
}
