/*!
    Raw pointers to logprintf and the list of AMX functions.

    Do **not** use it directly.

    Used in `log!` macro and `amx` module.
*/

use std;
use std::sync::Mutex;
use crate::types::Logprintf_t;
use crate::lazy_static;

lazy_static! {
    pub static ref logprintf: Mutex<Logprintf_t> = unsafe {
        Mutex::new(std::mem::transmute(0usize))
    };
}

pub static mut amx_functions: *const u32 = 0 as *const u32;