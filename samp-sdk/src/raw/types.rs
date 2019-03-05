use super::functions::{AmxCallback, AmxDebug, AmxNative};
use std::os::raw::{c_char, c_int, c_long, c_uchar, c_void};

#[repr(C, packed)]
pub struct AMX {
    pub base: *mut c_uchar,
    pub data: *mut c_uchar,
    pub callback: AmxCallback,
    pub debug: AmxDebug,
    pub cip: i32,
    pub frm: i32,
    pub hea: i32,
    pub hlw: i32,
    pub stk: i32,
    pub stp: i32,
    pub flags: c_int,
    pub usertags: [c_long; 4usize],
    pub userdata: [*mut c_void; 4usize],
    pub error: c_int,
    pub paramcount: c_int,
    pub pri: i32,
    pub alt: i32,
    pub reset_stk: i32,
    pub reset_hea: i32,
    pub sysreq_d: i32,
}

#[repr(C, packed)]
pub struct AMX_NATIVE_INFO {
    pub name: *const c_char,
    pub func: AmxNative,
}

#[repr(C, packed)]
pub struct AMX_FUNCSTUB {
    pub address: u32,
    pub name: [c_char; 20usize],
}

#[repr(C, packed)]
pub struct ANX_FUNCSTUBNT {
    pub address: u32,
    pub nameofs: u32,
}

#[repr(C, packed)]
pub struct AMX_HEADER {
    pub size: i32,
    pub magic: u16,
    pub file_version: c_char,
    pub amx_version: c_char,
    pub flags: i16,
    pub defsize: i16,
    pub cod: i32,
    pub dat: i32,
    pub hea: i32,
    pub stp: i32,
    pub cip: i32,
    pub publics: i32,
    pub natives: i32,
    pub libraries: i32,
    pub pubvars: i32,
    pub tags: i32,
    pub nametable: i32,
}
