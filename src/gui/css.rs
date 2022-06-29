use gtk::prelude::*;
use gtk::CssProvider;

#[cfg(debug_assertions)]
pub fn load_css() -> Result<CssProvider, gtk::glib::Error> {
    #[cfg(debug_assertions)]
    let css_path = "./css/style.css";

    let css = CssProvider::default().unwrap();
    css.load_from_path(css_path)?;

    Ok(css)
}

#[cfg(not(debug_assertions))]
pub fn load_css() -> Result<CssProvider, gtk::glib::Error> {
    let css_path_0 = shellexpand::tilde("~/.config/findex/style.css");
    let css_path_1 = "/opt/findex/style.css";
    let css = CssProvider::default().unwrap();

    let mut file = std::path::Path::new(css_path_0.as_ref());
    if !file.exists() {
        eprintln!(
            "Stylesheet wasn't found in user's home directory. Falling backing to default one."
        );

        file = std::path::Path::new(css_path_1);
        if file.exists() {
            // copy the file to css path 0
            let dirpath = shellexpand::tilde("~/.config/findex");
            if !std::path::Path::new(dirpath.as_ref()).exists() {
                std::fs::create_dir(dirpath.as_ref()).unwrap();
            }

            std::fs::copy(css_path_1, css_path_0.as_ref()).unwrap();
        } else {
            eprintln!("Error: Couldn't find any stylesheet");
            return Ok(css);
        }
    }

    css.load_from_path(css_path_0.as_ref())?;

    Ok(css)
}
