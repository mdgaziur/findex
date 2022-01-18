use crate::daemon::backend::{AppInfo, Backend};
use libloading::{Library, Symbol};

#[derive(Clone)]
pub struct CustomBackend {
    result_func: Symbol<'static, unsafe extern "Rust" fn(&str) -> Vec<AppInfo>>,
    get_all_func: Symbol<'static, unsafe extern "Rust" fn() -> Vec<AppInfo>>,
}

impl Backend for CustomBackend {
    fn new(lib_path: Option<&str>) -> Result<Self, String> {
        let lib = Box::leak(Box::new(unsafe {
            Library::new(lib_path.unwrap()).map_err(|e| e.to_string())?
        }));

        // User must ensure that the custom backend's init function is valid
        unsafe {
            get_backend_init_func(lib)?()?;
        }
        Ok(CustomBackend {
            result_func: get_backend_process_result_func(lib)?,
            get_all_func: get_backend_get_all_func(lib)?
        })
    }

    fn process_result(&mut self, query: &str) -> Vec<AppInfo> {
        // User must ensure that the custom backend's query function is valid
        unsafe {
            (self.result_func)(query)
        }
    }

    fn get_all(&mut self) -> Vec<AppInfo> {
        unsafe {
            (self.get_all_func)()
        }
    }
}

pub fn get_backend_process_result_func(
    lib: &'static Library,
) -> Result<libloading::Symbol<'static, unsafe extern "Rust" fn(&str) -> Vec<AppInfo>>, String> {
    unsafe { lib.get(b"process_result").map_err(|e| e.to_string()) }
}

pub fn get_backend_init_func(
    lib: &'static Library,
) -> Result<libloading::Symbol<'static, unsafe extern "Rust" fn() -> Result<(), String>>, String> {
    unsafe { lib.get(b"init").map_err(|e| e.to_string()) }
}

pub fn get_backend_get_all_func(
    lib: &'static Library,
) -> Result<libloading::Symbol<'static, unsafe extern "Rust" fn() -> Vec<AppInfo>>, String> {
    unsafe { lib.get(b"get_all").map_err(|e| e.to_string()) }
}
