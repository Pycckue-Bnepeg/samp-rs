//! Default AMX constants.
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

impl From<ServerData> for isize {
    fn from(data: ServerData) -> isize {
        data as isize
    }
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
    UserDef(i32),
}

impl From<AmxExecIdx> for i32 {
    fn from(value: AmxExecIdx) -> i32 {
        match value {
            AmxExecIdx::Main => -1,
            AmxExecIdx::Continue => -2,
            AmxExecIdx::UserDef(idx) => idx,
        }
    }
}

impl From<i32> for AmxExecIdx {
    fn from(idx: i32) -> AmxExecIdx {
        match idx {
            -1 => AmxExecIdx::Main,
            -2 => AmxExecIdx::Continue,
            idx => AmxExecIdx::UserDef(idx),
        }
    }
}
