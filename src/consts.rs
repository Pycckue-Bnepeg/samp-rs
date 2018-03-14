/*!
AMX Constants.


There is a bunch of raw AMX constants. 

They shouldn't be used in yours plugin code.
*/

pub const SUPPORTS_VERSION: u32 = 512;
pub const SUPPORTS_VERSION_MASK: u32 = 65535;
pub const SUPPORTS_AMX_NATIVES: u32 = 65536;
pub const SUPPORTS_PROCESS_TICK: u32 = 131072;

pub const PLUGIN_DATA_LOGPRINTF: u32 = 0;
pub const PLUGIN_DATA_AMX_EXPORTS: u32 = 16;
pub const PLUGIN_DATA_CALLPUBLIC_FS: u32 = 17;
pub const PLUGIN_DATA_CALLPUBLIC_GM: u32 = 18;

pub const AMX_ERR_NONE: i32 = 0;
pub const AMX_ERR_EXIT: i32 = 1;
pub const AMX_ERR_ASSERT: i32 = 2;
pub const AMX_ERR_STACKERR: i32 = 3;
pub const AMX_ERR_BOUNDS: i32 = 4;
pub const AMX_ERR_MEMACCESS: i32 = 5;
pub const AMX_ERR_INVINSTR: i32 = 6;
pub const AMX_ERR_STACKLOW: i32 = 7;
pub const AMX_ERR_HEAPLOW: i32 = 8;
pub const AMX_ERR_CALLBACK: i32 = 9;
pub const AMX_ERR_NATIVE: i32 = 10;
pub const AMX_ERR_DIVIDE: i32 = 11;
pub const AMX_ERR_SLEEP: i32 = 12;
pub const AMX_ERR_INVSTATE: i32 = 13;
pub const AMX_ERR_MEMORY: i32 = 16;
pub const AMX_ERR_FORMAT: i32 = 17;
pub const AMX_ERR_VERSION: i32 = 18;
pub const AMX_ERR_NOTFOUND: i32 = 19;
pub const AMX_ERR_INDEX: i32 = 20;
pub const AMX_ERR_DEBUG: i32 = 21;
pub const AMX_ERR_INIT: i32 = 22;
pub const AMX_ERR_USERDATA: i32 = 23;
pub const AMX_ERR_INIT_JIT: i32 = 24;
pub const AMX_ERR_PARAMS: i32 = 25;
pub const AMX_ERR_DOMAIN: i32 = 26;
pub const AMX_ERR_GENERAL: i32 = 27;

pub const AMX_FLAG_DEBUG: i32 = 0x02;
pub const AMX_FLAG_COMPACT: i32 = 0x04;
pub const AMX_FLAG_BYTEOPC: i32 = 0x08;
pub const AMX_FLAG_NOCHECKS: i32 = 0x10;
pub const AMX_FLAG_NTVREG: i32 = 0x1000;
pub const AMX_FLAG_JITC: i32 = 0x2000;
pub const AMX_FLAG_BROWSE: i32 = 0x4000;
pub const AMX_FLAG_RELOC: i32 = 0x8000;

pub const AMX_EXEC_MAIN: i32 = -1;
pub const AMX_EXEC_CONT: i32 = -2;

pub enum Exports {
    Align16 = 0,
    Align32 = 1,
    Align64 = 2,
    Allot = 3,
    Callback = 4,
    Cleanup = 5,
    Clone = 6,
    Exec = 7,
    FindNative = 8,
    FindPublic = 9,
    FindPubVar = 10,
    FindTagId = 11,
    Flags = 12,
    GetAddr = 13,
    GetNative = 14,
    GetPublic = 15,
    GetPubVar = 16,
    GetString = 17,
    GetTag = 18,
    GetUserData = 19,
    Init = 20,
    InitJIT = 21,
    MemInfo = 22,
    NameLength = 23,
    NativeInfo = 24,
    NumNatives = 25,
    NumPublics = 26,
    NumPubVars = 27,
    NumTags = 28,
    Push = 29,
    PushArray = 30,
    PushString = 31,
    RaiseError = 32,
    Register = 33,
    Release = 34,
    SetCallback = 35,
    SetDebugHook = 36,
    SetString = 37,
    SetUserData = 38,
    StrLen = 39,
    UTF8Check = 40,
    UTF8Get = 41,
    UTF8Len = 42,
    UTF8Put = 43,
}