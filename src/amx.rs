use std::ptr::read;
use std::mem::transmute;
use std::ffi::CString;
use types;
use consts::*;
use data::amx_functions;

pub type AmxResult<T> = Result<T, AmxError>;

pub struct AMX {
    amx: *mut types::AMX,
}

impl AMX {
    pub fn new(amx: *mut types::AMX) -> AMX {
        AMX {
            amx,
        }
    }

    pub fn exec(&self, index: i32) -> AmxResult<i64> {
        unsafe {
            let exec: types::Exec_t = read(transmute(amx_functions + PLUGIN_AMX_EXPORT_FindPublic));

            let retval = -1;
            let result = exec(self.amx, transmute(&retval), index);

            if result == 0 {
                Ok(retval)
            } else {
                Err(AmxError::from(result))
            }
        }
    }

    pub fn find_public(&self, name: &str) -> AmxResult<i32> {
        unsafe {
            let find_public: types::FindPublic_t = read(transmute(amx_functions + PLUGIN_AMX_EXPORT_FindPublic));

            let index = -1;
            let c_name = CString::new(name).unwrap();
            let result = find_public(self.amx, c_name.as_ptr(), transmute(&index));

            if result == 0 {
                Ok(index)
            } else {
                Err(AmxError::from(result))
            }
        }
    }
}

#[derive(Debug)]
pub enum AmxError {
    Exit = 1,
    Assert = 2,
    StackError = 3,
    Bounds = 4,
    MemoryAccess = 5,
    InvalidInstruction = 6,
    StackLow = 7,
    Native = 8,
    Divide = 9,
    Sleep = 10,
    Memory = 16,
    Format = 17,
    Version = 18,
    NotFound = 19,
    Index = 20,
    Debug = 21,
    Init = 22,
    UserData = 23,
    InitJit = 24,
    Params = 25,
    Domain = 26,
    General = 27,
    Unknown,
}

impl From<i32> for AmxError {
    fn from(val: i32) -> Self {
        match val {
            1 => AmxError::Exit,
            2 => AmxError::Assert,
            3 => AmxError::StackError,
            4 => AmxError::Bounds,
            5 => AmxError::MemoryAccess,
            6 => AmxError::InvalidInstruction,
            7 => AmxError::StackLow,
            8 => AmxError::Native,
            9 => AmxError::Divide,
            10 => AmxError::Sleep,
            16 => AmxError::Memory,
            17 => AmxError::Format,
            18 => AmxError::Version,
            19 => AmxError::NotFound,
            20 => AmxError::Index,
            21 => AmxError::Debug,
            22 => AmxError::Init,
            23 => AmxError::UserData,
            24 => AmxError::InitJit,
            25 => AmxError::Params,
            26 => AmxError::Domain,
            27 => AmxError::General,
            _ => AmxError::Unknown,
        }
    }
}