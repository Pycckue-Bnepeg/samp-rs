use std;
use std::sync::Mutex;
use types::Logprintf_t;

lazy_static! {
    pub static ref logprintf: Mutex<Logprintf_t> = unsafe {
        Mutex::new(std::mem::transmute(0u32))
    };
}

pub static mut amx_functions: *const u32 = 0 as *const u32;