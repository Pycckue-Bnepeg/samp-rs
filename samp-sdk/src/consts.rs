use bitflags::bitflags;

bitflags! {
    pub struct Supports: u32 {
        const VERSION = 512;
        const AMX_NATIVES = 0x10000;
        const PROCESS_TICK = 0x20000;
    }
}

/// Offsets
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ServerData {
    Logprintf = 0,
    AmxExports = 16,
    CallPublicFs = 17,
    CallPublicGm = 18,
}

bitflags! {
    pub struct AmxFlags: u16 {
        const DEBUG = 0x02;
        const COMPACT = 0x04;
        const BYTEOPC = 0x08;
        const NOCHECKS = 0x10;
        const NTVREG = 0x1000;
        const JITC = 0x2000;
        const BROWSE = 0x4000;
        const RELOC = 0x8000;
    }
}

/// Index of an AMX function in memory.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum AmxExecIdx {
    Main,
    Continue,
    Custom(i32),
}

impl From<AmxExecIdx> for i32 {
    fn from(value: AmxExecIdx) -> i32 {
        match value {
            AmxExecIdx::Main => -1,
            AmxExecIdx::Continue => -2,
            AmxExecIdx::Custom(idx) => idx,
        }
    }
}

impl From<i32> for AmxExecIdx {
    fn from(idx: i32) -> AmxExecIdx {
        match idx {
            -1 => AmxExecIdx::Main,
            -2 => AmxExecIdx::Continue,
            idx => AmxExecIdx::Custom(idx),
        }
    }
}

/// List of `amx_*` functions exported via the extern `Load` function.
#[derive(Debug, Clone, Copy, PartialEq)]
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

impl From<Exports> for isize {
    fn from(exports: Exports) -> isize {
        exports as isize
    }
}
