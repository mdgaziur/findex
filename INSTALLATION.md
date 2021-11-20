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

## Installation from AUR

From repo: `findex-git`   
Prebuilt: `findex-bin`
