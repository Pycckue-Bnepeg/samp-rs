//! # AMX Constants.
//! 
//! There is a bunch of raw AMX constants. 
//! They shouldn't be used in yours plugin code (exclude SUPPORTS consts).

pub const SUPPORTS_VERSION: u32 = 512;
pub const SUPPORTS_VERSION_MASK: u32 = 65535;
pub const SUPPORTS_AMX_NATIVES: u32 = 65536;
pub const SUPPORTS_PROCESS_TICK: u32 = 131072;

pub const PLUGIN_DATA_LOGPRINTF: u32 = 0 * 4;
pub const PLUGIN_DATA_AMX_EXPORTS: u32 = 16 * 4;
pub const PLUGIN_DATA_CALLPUBLIC_FS: u32 = 17 * 4;
pub const PLUGIN_DATA_CALLPUBLIC_GM: u32 = 18 * 4;

pub const PLUGIN_AMX_EXPORT_Align16: u32 = 0 * 4;
pub const PLUGIN_AMX_EXPORT_Align32: u32 = 1 * 4;
pub const PLUGIN_AMX_EXPORT_Align64: u32 = 2 * 4;
pub const PLUGIN_AMX_EXPORT_Allot: u32 = 3 * 4;
pub const PLUGIN_AMX_EXPORT_Callback: u32 = 4 * 4;
pub const PLUGIN_AMX_EXPORT_Cleanup: u32 = 5 * 4;
pub const PLUGIN_AMX_EXPORT_Clone: u32 = 6 * 4;
pub const PLUGIN_AMX_EXPORT_Exec: u32 = 7 * 4;
pub const PLUGIN_AMX_EXPORT_FindNative: u32 = 8 * 4;
pub const PLUGIN_AMX_EXPORT_FindPublic: u32 = 9 * 4;
pub const PLUGIN_AMX_EXPORT_FindPubVar: u32 = 10 * 4;
pub const PLUGIN_AMX_EXPORT_FindTagId: u32 = 11 * 4;
pub const PLUGIN_AMX_EXPORT_Flags: u32 = 12 * 4;
pub const PLUGIN_AMX_EXPORT_GetAddr: u32 = 13 * 4;
pub const PLUGIN_AMX_EXPORT_GetNative: u32 = 14 * 4;
pub const PLUGIN_AMX_EXPORT_GetPublic: u32 = 15 * 4;
pub const PLUGIN_AMX_EXPORT_GetPubVar: u32 = 16 * 4;
pub const PLUGIN_AMX_EXPORT_GetString: u32 = 17 * 4;
pub const PLUGIN_AMX_EXPORT_GetTag: u32 = 18 * 4;
pub const PLUGIN_AMX_EXPORT_GetUserData: u32 = 19 * 4;
pub const PLUGIN_AMX_EXPORT_Init: u32 = 20 * 4;
pub const PLUGIN_AMX_EXPORT_InitJIT: u32 = 21 * 4;
pub const PLUGIN_AMX_EXPORT_MemInfo: u32 = 22 * 4;
pub const PLUGIN_AMX_EXPORT_NameLength: u32 = 23 * 4;
pub const PLUGIN_AMX_EXPORT_NativeInfo: u32 = 24 * 4;
pub const PLUGIN_AMX_EXPORT_NumNatives: u32 = 25 * 4;
pub const PLUGIN_AMX_EXPORT_NumPublics: u32 = 26 * 4;
pub const PLUGIN_AMX_EXPORT_NumPubVars: u32 = 27 * 4;
pub const PLUGIN_AMX_EXPORT_NumTags: u32 = 28 * 4;
pub const PLUGIN_AMX_EXPORT_Push: u32 = 29 * 4;
pub const PLUGIN_AMX_EXPORT_PushArray: u32 = 30 * 4;
pub const PLUGIN_AMX_EXPORT_PushString: u32 = 31 * 4;
pub const PLUGIN_AMX_EXPORT_RaiseError: u32 = 32 * 4;
pub const PLUGIN_AMX_EXPORT_Register: u32 = 33 * 4;
pub const PLUGIN_AMX_EXPORT_Release: u32 = 34 * 4;
pub const PLUGIN_AMX_EXPORT_SetCallback: u32 = 35 * 4;
pub const PLUGIN_AMX_EXPORT_SetDebugHook: u32 = 36 * 4;
pub const PLUGIN_AMX_EXPORT_SetString: u32 = 37 * 4;
pub const PLUGIN_AMX_EXPORT_SetUserData: u32 = 38 * 4;
pub const PLUGIN_AMX_EXPORT_StrLen: u32 = 39 * 4;
pub const PLUGIN_AMX_EXPORT_UTF8Check: u32 = 40 * 4;
pub const PLUGIN_AMX_EXPORT_UTF8Get: u32 = 41 * 4;
pub const PLUGIN_AMX_EXPORT_UTF8Len: u32 = 42 * 4;
pub const PLUGIN_AMX_EXPORT_UTF8Put: u32 = 43 * 4;

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