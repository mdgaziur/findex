use crate::FINDEX_CONFIG;
use gtk::builders::BoxBuilder;
use gtk::gdk_pixbuf::{Colorspace, Pixbuf};

use gtk::glib::GString;
use gtk::pango::EllipsizeMode;
use gtk::prelude::*;
use gtk::{
    IconLookupFlags, IconTheme, Image, Justification, Label, ListBox, ListBoxRow, Orientation,
};

pub fn result_list_row(
    listbox: &ListBox,
    app_icon: &str,
    app_name: &str,
    app_desc: Option<GString>,
    app_cmd: &str,
    is_cli: bool,
    _highlight_start: usize,
    _highlight_end: usize,
) -> ListBoxRow {
    let box1 = BoxBuilder::new()
        .orientation(Orientation::Horizontal)
        .expand(true)
        .build();

    let app_icon = Image::builder()
        .pixbuf(&get_icon(app_icon))
        .parent(&box1)
        .build();
    app_icon.style_context().add_class("findex-result-icon");

    let box2 = BoxBuilder::new()
        .orientation(Orientation::Vertical)
        .parent(&box1)
        .build();

    let app_name_label = Label::builder()
        .parent(&box2)
        .label(app_name)
        .justify(Justification::Left)
        .xalign(0f32)
        .expand(true)
        .build();
    app_name_label
        .style_context()
        .add_class("findex-result-app-name");

    if let Some(app_desc) = app_desc {
        let app_desc_label = Label::builder()
            .label(&app_desc)
            .expand(true)
            .parent(&box2)
            .justify(Justification::Left)
            .xalign(0f32)
            .max_width_chars(1)
            .hexpand(true)
            .ellipsize(EllipsizeMode::End)
            .build();
        app_desc_label
            .style_context()
            .add_class("findex-result-app-description");
    }

    let row = ListBoxRow::builder().parent(listbox).child(&box1).build();
    row.style_context().add_class("findex-result-row");

    // We know the type of both values
    unsafe {
        row.set_data("app-cmd", app_cmd.to_string());
        row.set_data("is-cli", is_cli);
    }

    row
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
