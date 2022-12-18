# Installation
If you are not using Findex directly from `development` branch and using latest release,
check the `release` branch.

## Supported OS
- Linux

## Requirements
- Gtk3
- libkeybinder3
- Rust v1.56.1(building from source)

## Automated installation from source
- Run `./installer.sh`

## Manual installation from source
- Set Rust toolchain to stable using: `rustup default stable`
- Do a release build using: `cargo build --release`
- Make `/opt/findex` directory
- Copy `css/style.css` to `/opt/findex`
- Copy `target/release/findex` to `/usr/bin/findex`
- Add `findex-daemon` to autostart/startup applications

## Installation from AUR

From repo: `findex-git`   
Prebuilt: `findex-bin`

After that, add `findex-daemon` to autostart/startup applications
