use gtk::prelude::*;
use gtk::CssProvider;

pub fn load_css() -> CssProvider {
    let css_path;
    if std::env::var("DEVELOPMENT").is_ok() {
        css_path = String::from("./css/style.css");
    } else {
        css_path = String::from("/opt/findex/style.css");
    }

    let css = CssProvider::default().unwrap();
    css.load_from_path(&css_path).unwrap();

    css
}
