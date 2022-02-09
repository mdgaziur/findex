use crate::gui::config::FINDEX_CONFIG;
use gtk::gdk_pixbuf::{Colorspace, Pixbuf};
use gtk::pango::EllipsizeMode;
use gtk::prelude::*;
use gtk::{Box, ButtonsType, DialogFlags, IconLookupFlags, IconTheme, Image, Label, ListBox, ListBoxRow, MessageDialog, MessageType, Orientation, Window};
use findex::AppInfo;

pub fn add_app_to_listbox(list_box: &ListBox, app: &AppInfo) {
    let icon = get_icon(&app.icon);

    let image = Image::builder().pixbuf(&icon).build();
    image.style_context().add_class("findex-result-icon");

    let name = Label::new(Some(&app.name));
    name.style_context().add_class("findex-result-app-name");

    let command = Label::new(Some(&app.exec));
    command.set_xalign(0f32);
    command.set_max_width_chars(1);
    command.set_hexpand(true);
    command.set_ellipsize(EllipsizeMode::End);
    command.style_context().add_class("findex-result-command");

    let container = Box::builder()
        .orientation(Orientation::Horizontal)
        .build();
    container.pack_start(&image, false, false, 0);
    container.pack_start(&name, false, false, 0);
    container.add(&command);

    let row = ListBoxRow::new();
    row.add(&container);
    row.style_context().add_class("findex-result-row");
    row.show_all();
    row.focus_child();

    list_box.add(&row);
}

fn get_icon(icon_name: &str) -> Pixbuf {
    let icon;
    let icon_theme = IconTheme::default().unwrap();

    if let Ok(i) =
    Pixbuf::from_file_at_size(&icon_name, FINDEX_CONFIG.icon_size, FINDEX_CONFIG.icon_size)
    {
        icon = i;
    } else if let Ok(i) = icon_theme.load_icon(
        icon_name,
        FINDEX_CONFIG.icon_size,
        IconLookupFlags::FORCE_SIZE | IconLookupFlags::USE_BUILTIN,
    ) {
        icon = i.unwrap();
    } else if let Ok(i) = icon_theme.load_icon(
        "applications-other",
        FINDEX_CONFIG.icon_size,
        IconLookupFlags::FORCE_SIZE | IconLookupFlags::USE_BUILTIN,
    ) {
        icon = i.unwrap();
    } else {
        icon = Pixbuf::new(
            Colorspace::Rgb,
            true,
            8,
            FINDEX_CONFIG.icon_size,
            FINDEX_CONFIG.icon_size,
        )
            .unwrap();
    }

    icon
}

pub fn show_dialog<T: IsA<Window>>(window: &T, message: &str, message_type: MessageType, title: &str) {
    let dialog = MessageDialog::new(
        Some(window),
        DialogFlags::empty(),
        message_type,
        ButtonsType::Ok,
        message,
    );

    dialog.set_title(title);
    dialog.run();
    unsafe {
        dialog.destroy();
    }
}

pub fn clear_listbox(list_box: &ListBox) {
    for child in &list_box.children() {
        list_box.remove(child);
    }
}