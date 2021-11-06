# Findex
Highly customizable finder with high performance. Written in Rust and uses GTK
------
![Screenshot](Screenshot_20211106_111608.png)
------

## Installation
### Automatic
#### Binary
- Clone from `https://aur.archlinux.org/findex.git`
#### From source
- Clone from `https://aur.archlinux.org/findex-git.git`
Now run `makepkg -si`

### Manual
Make a release build using `cargo build --release`  
Copy `target/release/findex` to `/usr/bin/`  
Copy `css/style.css` to `/opt/findex/`  


## Customization
Customization can be done through the stylesheet located in ~/.config/findex/style.css.
You only have to make sure that it's valid for gtk.
