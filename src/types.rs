/*!
    List of AMX types such as `Cell`, `AMX_NATIVE_INFO` and signatures of functions.
*/
use std::os::raw::c_void;

pub type Cell = i32;
pub type Ucell = u32;

#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct AMX {
    pub base: *mut u8,
    pub data: *mut u8,
    pub callback: AmxCallback,
    pub debug: AmxDebug,
    pub cip: Cell,
    pub frm: Cell,
    pub hea: Cell,
    pub hlw: Cell,
    pub stk: Cell,
    pub stp: Cell,
    pub flags: i32,
    pub usertags: [i64; 4usize],
    pub userdata: [*mut c_void; 4usize],
    pub error: i32,
    pub paramcount: i32,
    pub pri: Cell,
    pub alt: Cell,
    pub reset_stk: Cell,
    pub reset_hea: Cell,
    pub sysreq_d: Cell,
}

pub type AmxNative = extern "system" fn(*mut AMX, params: *mut Cell) -> Cell;
pub type AmxCallback = extern "system" fn(*mut AMX, index: Cell, result: *mut Cell, params: *mut Cell) -> i32;
pub type AmxDebug = extern "system" fn(*mut AMX) -> i32;

#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct AMX_NATIVE_INFO {
    pub name: *const i8,
    pub func: AmxNative,
}

#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct AMX_FUNCSTUB {
    pub address: Ucell,
    pub name: [i8; 20usize],
}

#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct FUNCSTUBNT {
    pub address: Ucell,
    pub nameofs: u32,
}

#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct AMX_HEADER {
    pub size: i32,
    pub magic: u16,
    pub file_version: i8,
    pub amx_version: i8,
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

pub type Align16 = extern "system" fn(*mut u16) -> *mut u16;
pub type Align32 = extern "system" fn(*mut u32) -> *mut u32;
pub type Allot = extern "system" fn(*mut AMX, i32, *mut Cell, *mut Cell) -> i32;
pub type Callback = extern "system" fn(*mut AMX, Cell, *mut Cell, *mut Cell) -> i32;
pub type Cleanup = extern "system" fn(*mut AMX) -> i32;
pub type Clone = extern "system" fn(*mut AMX, *mut AMX, *mut c_void) -> i32;
pub type Exec = extern "system" fn(*mut AMX, *mut Cell, i32) -> i32;
pub type FindNative = extern "system" fn(*mut AMX, *const i8, *mut i32) -> i32;
pub type FindPublic = extern "system" fn(*mut AMX, *const i8, *mut i32) -> i32;
pub type FindPubVar = extern "system" fn(*mut AMX, *const i8, *mut Cell) -> i32;
pub type FindTagId = extern "system" fn(*mut AMX, Cell, *mut i8) -> i32;
pub type Flags = extern "system" fn(*mut AMX, *mut u16) -> i32;
pub type GetAddr = extern "system" fn(*mut AMX, Cell, *mut *mut Cell) -> i32;
pub type GetNative = extern "system" fn(*mut AMX, i32, *mut i8) -> i32;
pub type GetPublic = extern "system" fn(*mut AMX, i32, *mut i8) -> i32;
pub type GetPubVar = extern "system" fn(*mut AMX, i32, *mut i8, *mut Cell) -> i32;
pub type GetString = extern "system" fn(*mut u8, *const Cell, i32, usize) -> i32;
pub type GetTag = extern "system" fn(*mut AMX, i32, *mut i8, *mut Cell) -> i32;
pub type GetUserData = extern "system" fn(*mut AMX, i64, *mut *mut c_void) -> i32;
pub type Init = extern "system" fn(*mut AMX, *mut c_void) -> i32;
pub type InitJIT = extern "system" fn(*mut AMX, *mut c_void, *mut c_void) -> i32;
pub type MemInfo = extern "system" fn(*mut AMX, *mut i64, *mut i64, *mut i64) -> i32;
pub type NameLength = extern "system" fn(*mut AMX, *mut i32) -> i32;
pub type NativeInfo = extern "system" fn(*const i8, AmxNative) -> *mut AMX_NATIVE_INFO;
pub type NumNatives = extern "system" fn(*mut AMX, *mut i32) -> i32;
pub type NumPublics = extern "system" fn(*mut AMX, *mut i32) -> i32;
pub type NumPubVars = extern "system" fn(*mut AMX, *mut i32) -> i32;
pub type NumTags = extern "system" fn(*mut AMX, *mut i32) -> i32;
pub type Push = extern "system" fn(*mut AMX, Cell) -> i32;
pub type PushArray = extern "system" fn(*mut AMX, *mut Cell, *mut *mut Cell, *const Cell, i32) -> i32;
pub type PushString = extern "system" fn(*mut AMX, *mut Cell, *mut *mut Cell, *const i8, i32, i32) -> i32;
pub type RaiseError = extern "system" fn(*mut AMX, i32) -> i32;
pub type Register = extern "system" fn(*mut AMX, *const AMX_NATIVE_INFO, i32) -> i32;
pub type Release = extern "system" fn(*mut AMX, Cell) -> i32;
pub type SetCallback = extern "system" fn(*mut AMX, AmxCallback) -> i32;
pub type SetDebugHook = extern "system" fn(*mut AMX, AmxDebug) -> i32;
pub type SetString = extern "system" fn(*mut Cell, *const i8, i32, i32, usize) -> i32;
pub type SetUserData = extern "system" fn(*mut AMX, i64, *mut c_void) -> i32;
pub type StrLen = extern "system" fn(*const Cell, *mut i32) -> i32;
pub type UTF8Check = extern "system" fn(*const i8, *mut i32) -> i32;
pub type UTF8Get = extern "system" fn(*const i8, *mut *const i8, *mut Cell) -> i32;
pub type UTF8Len = extern "system" fn(*const Cell, *mut i32) -> i32;
pub type UTF8Put = extern "system" fn(*mut i8, *mut *mut i8, i32, Cell) -> i32;

pub type Logprintf_t = extern "C" fn(*const i8, ...);
