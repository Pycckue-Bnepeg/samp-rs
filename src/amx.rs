use std;
use std::mem::transmute;
use std::ffi::CString;
use types;
use consts::*;
use data::amx_functions;

pub struct AMX {
    amx: *mut types::AMX,
}

impl AMX {
    pub fn new(amx: *mut types::AMX) -> AMX {
        AMX {
            amx,
        }
    }

    pub fn find_public(&self, name: &str) -> (i32, i32) {
        unsafe {
            let find_public: types::FindPublic_t = std::ptr::read(transmute(amx_functions + PLUGIN_AMX_EXPORT_FindPublic));

            let index = -1;
            let c_name = CString::new(name).unwrap();
            let result = find_public(self.amx, c_name.as_ptr(), transmute(&index));

            (result, index)
        }
    }
}