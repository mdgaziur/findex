use gtk::prelude::*;
use gtk::{ButtonsType, MessageDialog, MessageType};

pub fn show_dialog(title: &str, message: &str, message_type: MessageType) {
    let dialog = MessageDialog::builder()
        .title(title)
        .text(message)
        .message_type(message_type)
        .buttons(ButtonsType::Ok)
        .build();

    dialog.run();
    dialog.close();
}
