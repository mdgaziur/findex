# Installation

## Supported OS
- Linux

## Requirements
- Gtk3
- Rust v1.56.1(building from source)

## Automated installation from source
- Run `./installer.sh`

## Manual installation from source
- Set Rust toolchain to stable using: `rustup default stable`
- Do a release build using: `cargo build --release`
- Make `/opt/findex` directory
- Copy `css/style.css` to `/opt/findex`
- Copy `target/release/findex` to `/usr/bin/findex`

### If you have systemd
- Copy `findex.service`, `findex-restarter.service` and `findex-restarter.path` to `/etc/systemd/user/`
- Run `systemctl --user enable findex.service`
- Run `systemctl --user enable findex-restarter.path`

### If you don't have systemd
- Add Findex to startup applications

Unfortunately Findex can't restart automatically when configs are changed unless you are using systemd. That's a future plan. Till then,
you'll have to restart it manually.

## Installation from AUR

From repo: `findex-git`   
Prebuilt: `findex-bin`

If you are using systemd, run the following commands:
- `systemctl --user enable findex.service`
- `systemctl --user enable findex-restarter.path`
