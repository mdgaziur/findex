use crate::app_list::strip_parameters;
use crate::gui::dialog::show_dialog;
use crate::FINDEX_CONFIG;
use abi_stable::std_types::*;
use findex_plugin::ApplicationCommand;
use gtk::builders::BoxBuilder;
use gtk::gdk_pixbuf::{Colorspace, Pixbuf};
use gtk::gio::{AppLaunchContext, DesktopAppInfo};
use gtk::pango::EllipsizeMode;
use gtk::prelude::*;
use gtk::{
    IconLookupFlags, IconTheme, Image, Justification, Label, ListBox, ListBoxRow, MessageType,
    Orientation,
};
use shlex::split;
use std::process::Command;

pub fn result_list_row(
    listbox: &ListBox,
    app_icon: &str,
    app_name: &str,
    app_desc: ROption<&str>,
    app_cmd: &ApplicationCommand,
    trigger_idx: Option<usize>,
) -> ListBoxRow {
    let box1 = BoxBuilder::new()
        .orientation(Orientation::Horizontal)
        .expand(true)
        .build();
    box1.style_context()
        .add_class("findex-result-icon-container");

    let app_icon = Image::builder()
        .pixbuf(&get_icon(app_icon))
        .parent(&box1)
        .build();
    app_icon.style_context().add_class("findex-result-icon");

    let box2 = BoxBuilder::new()
        .orientation(Orientation::Vertical)
        .parent(&box1)
        .build();
    box2.style_context()
        .add_class("findex-result-info-container");

    if let Some(trigger_idx) = trigger_idx {
        let keyboard_shortcut_label = Label::builder()
            .parent(&box1)
            .use_markup(true)
            .label(&format!("Ctrl+{trigger_idx}"))
            .xalign(1f32)
            .expand(true)
            .build();
        keyboard_shortcut_label
            .style_context()
            .add_class("findex-result-trigger-shortcut");
    }

    let app_name_label = Label::builder()
        .parent(&box2)
        .use_markup(true)
        .label(app_name)
        .justify(Justification::Left)
        .xalign(0f32)
        .expand(true)
        .build();
    app_name_label
        .style_context()
        .add_class("findex-result-app-name");

    if let RSome(app_desc) = app_desc {
        let app_desc_label = Label::builder()
            .label(app_desc)
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

    if *app_cmd != ApplicationCommand::None {
        let app_cmd_label = Label::builder()
            .label(&match app_cmd {
                ApplicationCommand::Command(cmd) => cmd.to_string(),
                ApplicationCommand::Id(id) => strip_parameters(
                    DesktopAppInfo::new(id)
                        .unwrap()
                        .commandline()
                        .unwrap()
                        .to_str()
                        .unwrap(),
                ),
                _ => unreachable!()
            })
            .expand(true)
            .parent(&box2)
            .justify(Justification::Left)
            .xalign(0f32)
            .max_width_chars(1)
            .hexpand(true)
            .ellipsize(EllipsizeMode::End)
            .build();
        app_cmd_label
            .style_context()
            .add_class("findex-result-app-command");
    }

    let row = ListBoxRow::builder().parent(listbox).child(&box1).build();
    row.style_context().add_class("findex-result-row");

    unsafe {
        row.set_data("app-cmd", app_cmd.clone());
    }

    row
}

pub fn handle_enter(row: &ListBoxRow) {
    handle_interaction(row);
}

pub fn handle_interaction(row: &ListBoxRow) {
    let cmd = unsafe { row.data::<ApplicationCommand>("app-cmd").unwrap().as_ref() };

    match cmd {
        ApplicationCommand::Command(cmd) => {
            // Ideally, plugins should provide a None variant for this, but rogue plugins
            // can do anything :)
            if cmd.is_empty() {
                return;
            }

            row.toplevel().unwrap().hide();
            let Some(cmd) = split(cmd) else {
                show_dialog("Error", "Failed to launch application", MessageType::Error);
                return;
            };

            let child = Command::new(&cmd[0]).args(&cmd[1..]).spawn();

            if let Err(e) = child {
                show_dialog(
                    "Error",
                    &format!("Failed to launch application: {e}"),
                    MessageType::Error,
                );
            }
        }
        ApplicationCommand::Id(id) => {
            // Ideally, plugins should provide a None variant for this, but rogue plugins
            // can do anything :)
            if id.is_empty() {
                return;
            }

            row.toplevel().unwrap().hide();
            let Some(desktop_appinfo) = DesktopAppInfo::new(id) else {
                show_dialog("Error", "Failed to launch application", MessageType::Error);
                return;
            };

            if let Err(e) = desktop_appinfo.launch(&[], None::<AppLaunchContext>.as_ref()) {
                show_dialog(
                    "Error",
                    &format!("Failed to launch application: {e}"),
                    MessageType::Error,
                );
            }
        }
        ApplicationCommand::None => {}
    }
}

fn get_icon(icon_name: &str) -> Pixbuf {
    let icon;
    let icon_theme = IconTheme::default().unwrap();

    if let Ok(i) =
        Pixbuf::from_file_at_size(icon_name, FINDEX_CONFIG.icon_size, FINDEX_CONFIG.icon_size)
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
