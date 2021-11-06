use nix::unistd::execvp;
use std::ffi::CString;

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
