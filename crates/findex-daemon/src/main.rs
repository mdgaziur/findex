use daemonize::Daemonize;
use inotify::{Inotify, WatchMask};
use nix::libc::{pid_t, time_t};
use nix::time::ClockId;
use shellexpand::tilde;
use std::fs::{create_dir, File};
use std::path::Path;
use std::process::Command;
use std::time::Duration;
use subprocess::{ExitStatus, Popen, PopenConfig, Redirection};

fn getpid(name: &str) -> Option<pid_t> {
    let output = match Command::new("pidof").arg(name).output() {
        Ok(o) => o,
        Err(e) => {
            eprintln!("[ERROR] Failed to run `pidof findex findex-daemon`: {e}");
            return None;
        }
    };

    let pid_s = String::from_utf8(output.stdout)
        .unwrap()
        .split('\n')
        .take(1)
        .map(|s| s.to_string())
        .collect::<String>();

    if pid_s.is_empty() {
        None
    } else {
        Some(pid_s.parse().unwrap())
    }
}

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

    let mut inotify = Inotify::init().expect("Failed to init inotify");
    let watch_mask = WatchMask::CREATE | WatchMask::MODIFY | WatchMask::MOVE | WatchMask::DELETE;
    inotify
        .add_watch(&*tilde("~/.config/findex/settings.toml"), watch_mask)
        .expect("Failed to watch `~/.config/findex/settings.toml`");
    inotify
        .add_watch(&*tilde("~/.config/findex/style.css"), watch_mask)
        .expect("Failed to watch `~/.config/findex/style.css`");

    let findex_output = File::create(&*tilde(&format!(
        "~/.findex-logs/findex-{current_time}.log"
    )))
    .expect("Failed to create file to store findex output");

    let mut findex_process = spawn_findex(findex_output.try_clone().unwrap());
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
        if let Ok(mut events) = inotify.read_events(&mut buf) {
            if events.next().is_some() {
                println!("[INFO] configs changed, restarting findex");
                if findex_process.poll().is_none() {
                    findex_process.kill().unwrap();
                }
                findex_process = spawn_findex(findex_output.try_clone().unwrap());
            }
        }
    }
}

fn main() {
    if let Some(pid) = getpid("findex") {
        eprintln!("[ERROR] Findex is already running with pid: {pid}");
        eprintln!("[ERROR] Help: You may want to kill with `killall findex`");

        return;
    }
    if let Some(pid) = getpid("findex-daemon") {
        if pid != nix::unistd::getpid().as_raw() {
            eprintln!("[ERROR] Findex daemon is already running with pid: {pid}");
            eprintln!("[ERROR] Help: You may want to kill with `killall findex-daemon`");

            return;
        }
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
