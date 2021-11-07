use gtk::prelude::*;
use gtk::{Entry, Label, ListBox, ScrolledWindow, Viewport};
use nix::unistd::execvp;
use std::ffi::CString;

pub fn init_search_result() -> ListBox {
    let list_box = ListBox::builder().name("findex-results").build();
    list_box.style_context().add_class("findex-results");

    list_box.connect_key_press_event(|lb, ev| {
        let key_name = match ev.keyval().name() {
            Some(name) => name,
            None => return Inhibit(false)
        };

        if key_name == "Up" {
            let selected_row_index = lb.clone().selected_row().unwrap().index();

            if selected_row_index == 0 {
                // focus to input box
                let par = lb.parent().unwrap().downcast::<Viewport>().unwrap();
                let scw = par.parent().unwrap().downcast::<ScrolledWindow>().unwrap();
                let container = scw.parent().unwrap().downcast::<gtk::Box>().unwrap();
                let child = &container.children()[0];
                let entry = child.downcast_ref::<Entry>().unwrap();

                entry.grab_focus();
                entry.set_position(-1);
                entry.activate();
                return Inhibit(true);
            }
        }
        Inhibit(false)
    });
    list_box.connect_row_activated(|_, lbr| {
        let container_w = &lbr.children()[0];
        let container = container_w.downcast_ref::<gtk::Box>().unwrap();
        let c_widget = &container.children()[2];
        let command = c_widget.downcast_ref::<Label>().unwrap();

        let splitted_cmd = shlex::split(&command.text().to_string()).unwrap();

        spawn_process(&splitted_cmd);
    });

    list_box
}

pub fn spawn_process(cmd: &[String]) {
    let p_name = CString::new(cmd[0].as_bytes()).unwrap();
    execvp(
        &p_name,
        &cmd.iter()
            .map(|s| CString::new(s.as_bytes()).unwrap())
            .collect::<Vec<CString>>(),
    )
        .unwrap();
}

