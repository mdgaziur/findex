use crate::daemon::backend::{AppInfo, Backend};
use libloading::{Library, Symbol};

pub struct CustomBackend {
    result_func: Symbol<'static, extern "Rust" fn(&str) -> Vec<AppInfo>>
}

impl Backend for CustomBackend {
    fn new(lib_path: Option<&str>) -> Result<Self, String> {
        let lib = Box::leak(Box::new(
            unsafe {
                Library::new(lib_path.unwrap())
                    .map_err(|e| e.to_string())?
            }
        ));

        get_backend_init_func(lib)?()?;
        Ok(CustomBackend {
            result_func: get_backend_process_result_func(lib)?
        })
    }

    fn process_result(&mut self, query: &str) -> Vec<AppInfo> {
        (self.result_func)(query)
    }
}

pub fn get_backend_process_result_func(lib: &'static Library) -> Result<libloading::Symbol<'static, extern "Rust" fn(&str) -> Vec<AppInfo>>, String> {
    unsafe { lib.get(b"process_result").map_err(|e| e.to_string()) }
}

pub fn get_backend_init_func(lib: &'static Library) -> Result<libloading::Symbol<'static, extern "Rust" fn() -> Result<(), String>>, String> {
    unsafe { lib.get(b"init").map_err(|e| e.to_string()) }
}
