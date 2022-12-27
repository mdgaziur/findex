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
    let config_path = xdg::BaseDirectories::new()
        .expect("Failed to get base directories")
        .create_config_directory("findex")
        .expect("Failed to create config dir");
    let css_path_0 = config_path.join("style.css");
    let css_path_1 = "/opt/findex/style.css";
    let css = CssProvider::default().unwrap();

    let mut file = std::path::Path::new(&css_path_0);
    if !file.exists() {
        eprintln!(
            "[WARN] Stylesheet wasn't found in user's home directory. Falling backing to default one."
        );

        file = std::path::Path::new(css_path_1);
        if file.exists() {
            std::fs::copy(css_path_1, &css_path_0).unwrap();
        } else {
            eprintln!("[WARN] Couldn't find any stylesheet");
            return Ok(css);
        }
    }

    css.load_from_path(css_path_0.to_str().unwrap())?;

    Ok(css)
}
