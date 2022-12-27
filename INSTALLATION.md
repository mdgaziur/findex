# Installation
If you are not using Findex directly from `development` branch and using latest release,
check instructions from the `release` branch.

## Supported OS
- Linux

## Requirements
- Gtk3
- libkeybinder3
- Rust v1.66.0 (building from source)

## Automated installation from source
- Run `./installer.sh`

## Manual installation from source
- Set Rust toolchain to stable using: `rustup default stable`
- Compile it using `cargo build --release`
- Make `/opt/findex` directory
- Copy `css/style.css` to `/opt/findex`
- Copy `target/release/findex` to `/usr/bin/`
- Copy `targer/release/findex-daemon` to `/usr/bin/`
- Add `findex-daemon` to autostart/startup applications

## Installation from AUR

From repo: `findex-git`   
Prebuilt: `findex-bin`

After that, add `findex-daemon` to autostart/startup applications

## Notes
- Findex can't bind hotkey in Wayland. Bind hotkey to `echo 1 > ~/.config/findex/toggle_file`
- Window decoration settings may not work in Wayland
