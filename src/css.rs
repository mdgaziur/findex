use gtk::prelude::*;
use gtk::CssProvider;

#[cfg(debug_assertions)]
pub fn load_css() -> CssProvider {
    #[cfg(debug_assertions)]
    let css_path = "./css/style.css";

    let css = CssProvider::default().unwrap();
    css.load_from_path(css_path).unwrap();

    css
}

#[cfg(not(debug_assertions))]
pub fn load_css() -> CssProvider {
    let css_path_0 = shellexpand::tilde("~/.config/findex/style.css");
    let css_path_1 = "/opt/findex/style.css";
    let css = CssProvider::default().unwrap();

    let mut file = std::path::Path::new(css_path_0.as_ref());
    if !file.exists() {
        eprintln!("Stylesheet wasn't found in user's home directory. Falling backing to default one.");

        file = std::path::Path::new(css_path_1);
        if file.exists() {
            // copy the file to css path 0
            std::fs::copy(css_path_1, css_path_0.as_ref()).unwrap();
        }
        else {
            eprintln!("Error: Couldn't find any stylesheet");
            return css;
        }
    }

    css.load_from_path(css_path_0.as_ref()).unwrap();
    css
}