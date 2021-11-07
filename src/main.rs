mod css;
pub mod query;
pub mod search_result;
mod window;

use crate::window::init_window;
use gtk::prelude::*;
use gtk::Application;

fn main() {
    let app = Application::builder().application_id("org.findex").build();

    app.connect_activate(init_window);

    app.run();
}
