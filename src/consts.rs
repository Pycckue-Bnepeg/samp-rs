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

pub const AMX_ERR_NONE: u32 = 0;
pub const AMX_ERR_EXIT: u32 = 1;
pub const AMX_ERR_ASSERT: u32 = 2;
pub const AMX_ERR_STACKERR: u32 = 3;
pub const AMX_ERR_BOUNDS: u32 = 4;
pub const AMX_ERR_MEMACCESS: u32 = 5;
pub const AMX_ERR_INVINSTR: u32 = 6;
pub const AMX_ERR_STACKLOW: u32 = 7;
pub const AMX_ERR_HEAPLOW: u32 = 8;
pub const AMX_ERR_CALLBACK: u32 = 9;
pub const AMX_ERR_NATIVE: u32 = 10;
pub const AMX_ERR_DIVIDE: u32 = 11;
pub const AMX_ERR_SLEEP: u32 = 12;
pub const AMX_ERR_INVSTATE: u32 = 13;
pub const AMX_ERR_MEMORY: u32 = 16;
pub const AMX_ERR_FORMAT: u32 = 17;
pub const AMX_ERR_VERSION: u32 = 18;
pub const AMX_ERR_NOTFOUND: u32 = 19;
pub const AMX_ERR_INDEX: u32 = 20;
pub const AMX_ERR_DEBUG: u32 = 21;
pub const AMX_ERR_INIT: u32 = 22;
pub const AMX_ERR_USERDATA: u32 = 23;
pub const AMX_ERR_INIT_JIT: u32 = 24;
pub const AMX_ERR_PARAMS: u32 = 25;
pub const AMX_ERR_DOMAIN: u32 = 26;
pub const AMX_ERR_GENERAL: u32 = 27;

pub const AMX_FLAG_DEBUG: u32 = 0x02;
pub const AMX_FLAG_COMPACT: u32 = 0x04;
pub const AMX_FLAG_BYTEOPC: u32 = 0x08;
pub const AMX_FLAG_NOCHECKS: u32 = 0x10;
pub const AMX_FLAG_NTVREG: u32 = 0x1000;
pub const AMX_FLAG_JITC: u32 = 0x2000;
pub const AMX_FLAG_BROWSE: u32 = 0x4000;
pub const AMX_FLAG_RELOC: u32 = 0x8000;

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