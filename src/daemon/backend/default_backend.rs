use freedesktop_entry_parser::Entry;
use crate::daemon::backend::{AppInfo, Backend};
use crate::daemon::config::FINDEX_CONFIG;
use crate::daemon::db::DB;
use fuse_rust::Fuse;
use inotify::{EventMask, Inotify, WatchDescriptor, WatchMask};

pub struct DefaultBackend {}

impl Backend for DefaultBackend {
    fn new(_lib_path: Option<&str>) -> Result<Self, String> {
        std::thread::spawn(watch_desktop_files);

        Ok(DefaultBackend {})
    }

    fn process_result(&mut self, query: &str) -> Vec<AppInfo> {
        let apps = DB.get_data(true).unwrap_or_else(|e| {
            println!("[Warning] Failed to get app list from inmemory db: {:?}", e);
            return vec![];
        });

        // iterate and do fuzzy search
        let mut filtered_apps = Vec::new();
        let fuse = Fuse {
            distance: FINDEX_CONFIG.max_fuzz_distance,
            ..Default::default()
        };

        for app in apps {
            let score_result_name = fuse.search_text_in_string(query, &app.name);
            let score_result_exec = fuse.search_text_in_string(query, &app.exec);
            let mut do_not_push = true;
            let mut total_score = 0f64;

            if let Some(result) = score_result_name {
                if result.score <= FINDEX_CONFIG.max_name_fuzz_result_score {
                    total_score += result.score;
                    do_not_push = false;
                }
            }
            if let Some(result) = score_result_exec {
                if result.score <= FINDEX_CONFIG.max_command_fuzz_result_score {
                    total_score += result.score;
                    do_not_push = false;
                }
            }

            if !do_not_push {
                filtered_apps.push(app.to_appinfo(total_score));
            }
        }
        filtered_apps.sort_by(|l, r| l.total_score.partial_cmp(&r.total_score).unwrap());

        filtered_apps
    }

    fn get_all(&mut self) -> Vec<AppInfo> {
        match DB.get_data(true) {
            Ok(list) => list.iter().map(|a| a.to_appinfo(0.0)).collect(),
            Err(_) => Vec::new()
        }
    }
}

fn watch_desktop_files() {
    let mut inotify = Inotify::init().expect("[Error] Error while initializing inotify instance");

    let search_dirs = [
        "/usr/share/applications",
        &shellexpand::tilde("~/.local/share/applications").to_string(),
    ];
    let mut wds = Vec::new();

    for search_dir in search_dirs {
        let path = std::path::Path::new(search_dir);
        if path.is_dir() {
            for file in std::fs::read_dir(path).unwrap() {
                if let Ok(file) = file {
                    update_entry(file.path().to_str().unwrap());
                } else {
                    eprintln!("[Warning] Invalid dir entry: {}", file.err().unwrap());
                }
            }

            let wd = inotify
                .add_watch(
                    search_dir,
                    WatchMask::MODIFY | WatchMask::CREATE | WatchMask::MOVE | WatchMask::DELETE,
                )
                .unwrap();
            wds.push((wd, search_dir));
        } else {
            eprintln!("[Warning] Search dir \"{search_dir}\" does not exist");
        }
    }

    loop {
        let mut buffer = [0; 1024];
        let events = inotify.read_events_blocking(&mut buffer).unwrap();

        for event in events {
            let file_name = event.name.unwrap().to_str().unwrap();
            let file_path = get_path(file_name, &wds, &event.wd).unwrap();

            match event.mask {
                EventMask::DELETE => remove_entry(&file_path),
                EventMask::CREATE => update_entry(&file_path),
                EventMask::MODIFY => update_entry(&file_path),
                EventMask::MOVED_TO => update_entry(&file_path),
                EventMask::MOVED_FROM => remove_entry(&file_path),
                _ => eprintln!("[Warning] Received unexpected event: {:?}", event.mask)
            }
        }
    }
}

fn get_path(file_name: &str, wds: &[(WatchDescriptor, &str)], file_wd: &WatchDescriptor) -> Result<String, ()> {
    for wd in wds {
        if &wd.0 == file_wd {
            return Ok(std::path::Path::new(wd.1).join(file_name).to_str().unwrap().to_string());
        }
    }

    Err(())
}

fn update_entry(file_name: &str) {
    if std::path::Path::new(file_name).extension().unwrap() != "desktop" {
        return;
    }

    let entry = match parse_entry(file_name) {
        Ok(entry) => {
            match crate::daemon::db::DBAppInfo::from_freedesktop_entry(&entry, file_name) {
                Ok(info) => info,
                Err(e) => {
                    eprintln!("[Warning] Error while parsing \"{}\": {}", file_name, e);
                    return;
                }
            }
        },
        Err(e) => {
            eprintln!("{}", e);
            return;
        }
    };

    let mut db = match DB.get_data(true) {
        Ok(db) => db,
        Err(_) => Vec::new(),
    };

    let mut push = true;
    let mut idx = 0;
    while idx < db.len() {
        let app = &db[idx];
        if app.name == entry.name {
            let _ = std::mem::replace(&mut db[idx], entry.clone());
            push = false;
            break;
        }
        idx += 1;
    }

    if push {
        db.push(entry.clone());
    }

    DB.put_data(db, true).unwrap();
}

fn remove_entry(file_name: &str) {
    let mut db = match DB.get_data(true) {
        Ok(db) => db,
        Err(_) => Vec::new(),
    };


    let mut idx = 0;
    while idx < db.len() {
        let app = &db[idx];
        if app.desktop_file == file_name {
            println!("[Info] Removed \"{file_name}\" from db.");
            db.remove(idx);
            break;
        }
        idx += 1;
    }

    DB.put_data(db, true).unwrap();
}

fn parse_entry(file_name: &str) -> Result<Entry, String> {
    Entry::parse_file(file_name)
        .map_err(|e| format!("[Warning] Failed to parse \"{}\": {}", file_name, e))
}