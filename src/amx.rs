use std::ptr::read;
use std::mem::transmute;
use std::ffi::CString;
use types;
use types::Cell;
use consts::*;
use data::amx_functions;

pub type AmxResult<T> = Result<T, AmxError>;

/// # AMX
pub struct AMX {
    amx: *mut types::AMX,
}

impl AMX {
    pub fn new(amx: *mut types::AMX) -> AMX {
        AMX {
            amx,
        }
    }

    pub fn register(&self, natives: &Vec<types::AMX_NATIVE_INFO>) -> AmxResult<()> {
        let len = natives.len();
        let ptr = natives.as_slice().as_ptr();

        unsafe {
            let register: types::Register_t = read(transmute(amx_functions + PLUGIN_AMX_EXPORT_Register));

            let result = register(self.amx, ptr, len as i32);

            if result == 0 {
                Ok(())
            } else {
                Err(AmxError::from(result))
            }
        }
    }

    pub fn get_address<T: Sized>(&self, address: Cell) -> AmxResult<Box<T>> {
        unsafe {
            let get_addr: types::GetAddr_t = read(transmute(amx_functions + PLUGIN_AMX_EXPORT_GetAddr));

            let ptr = 0;
            let result = get_addr(self.amx, address, transmute(&ptr));

            if result == 0 {
                Ok(Box::from_raw(transmute(ptr)))
            } else {
                Err(AmxError::from(result))
            }
        }
    }

    pub fn push(&self, value: Cell) -> AmxResult<()> {
        unsafe {
            let push: types::Push_t = read(transmute(amx_functions + PLUGIN_AMX_EXPORT_Push));
            
            let result = push(self.amx, value);

            if result == 0 {
                Ok(())
            } else {
                Err(AmxError::from(result))
            }
        }
    }

    pub fn exec(&self, index: i32) -> AmxResult<i64> {
        unsafe {
            let exec: types::Exec_t = read(transmute(amx_functions + PLUGIN_AMX_EXPORT_Exec));

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

/// Custom error type for AMX errors.
/// Can be casted from i32
///
/// # Examples
/// 
/// ```
/// let find_public: samp_sdk::types::FindPublic_t = ...;
/// let result = find_public(...);
/// return AmxError::from(result);
/// ```
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