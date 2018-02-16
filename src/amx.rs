/*!

*/

use std::ptr::read;
use std::mem::transmute;
use std::ffi::CString;
use types;
use types::Cell;
use consts::*;
use data::amx_functions;

pub type AmxResult<T> = Result<T, AmxError>;

macro_rules! ret {
    ($res:ident, $ret:expr) => {
        {
            if $res == 0 {
                Ok($ret)
            } else {
                Err(AmxError::from($res))
            }
        }
    }
}

macro_rules! call {
    (
        $ex:expr
        =>
        $ret:expr
    ) => {
        {
            let result = $ex;
            ret!(result, $ret)
        }
    }
}

macro_rules! import {
    ($type:ident) => {
        unsafe {
            read(transmute::<u32, *const types::$type>(amx_functions + Exports::$type as u32))
        }
    };
}

/// AMX struct that holds raw `types::AMX` pointer.
pub struct AMX {
    amx: *mut types::AMX,
}

impl AMX {
    /// Convert raw `types::AMX` pointer.
    pub fn new(amx: *mut types::AMX) -> AMX {
        AMX {
            amx,
        }
    }

    /// Register natives functions
    ///
    /// # Examples
    /// 
    /// ```
    /// fn amx_load(amx: AMX) -> Cell {
    ///     let natives = natives!{
    ///         "SomeFunction" => some_function,
    ///         "AnotherFunction" => another_function   
    ///     };
    /// 
    ///     amx.register(&natives).unwrap();
    ///     
    ///     AMX_ERR_NONE
    /// }
    /// ```
    pub fn register(&self, natives: &Vec<types::AMX_NATIVE_INFO>) -> AmxResult<()> {
        let len = natives.len();
        let ptr = natives.as_slice().as_ptr();

        let register = import!(Register);
        call!(register(self.amx, ptr, len as i32) => ())
    }

    /// Get an address of a reference value given to native.
    ///
    /// You **must** use `std::mem::forget` for this value because `get_address` return `Box<T>` which releases memory.
    ///
    /// # Examples
    ///
    /// ```
    /// // native: SomeNative(&int_value);
    /// fn some_native(amx: AMX, args: *mut Cell) -> Cell {
    ///     let arg = std::ptr::read((args as usize + 4) as *const Cell);
    ///     let int_value: Box<i32> = amx.get_address(arg).unwrap();
    ///     *int_value = 10;
    ///     
    ///     std::mem::forget(int_value);
    /// }
    /// ```
    pub fn get_address<T: Sized>(&self, address: Cell) -> AmxResult<Box<T>> {
        let get_addr = import!(GetAddr);

        let ptr = 0;

        unsafe {
            call!(get_addr(self.amx, address, transmute(&ptr)) => Box::from_raw(transmute(ptr)))
        }
    }

    pub fn push(&self, value: Cell) -> AmxResult<()> {
        let push = import!(Push);
        call!(push(self.amx, value) => ())
    }

    pub fn exec(&self, index: i32) -> AmxResult<i64> {
        let exec = import!(Exec);

        let retval = -1;
        unsafe {
            call!(exec(self.amx, transmute(&retval), index) => retval)
        }
    }

    pub fn find_public(&self, name: &str) -> AmxResult<i32> {
        let find_public = import!(FindPublic);

        let index = -1;
        let c_name = CString::new(name).unwrap();
        
        unsafe {
            call!(find_public(self.amx, c_name.as_ptr(), transmute(&index)) => index)
        }
    }

    pub fn find_native(&self, name: &str) -> AmxResult<i32> {
        let find_native = import!(FindNative);

        let index = -1;
        let c_name = CString::new(name).unwrap();
        
        unsafe {
            call!(find_native(self.amx, c_name.as_ptr(), transmute(&index)) => index)
        }
    }

    pub fn find_pubvar(&self, name: &str) -> AmxResult<i32> {
        let find_pubvar = import!(FindPubVar);

        let value = -1;
        let c_name = CString::new(name).unwrap();

        unsafe {
            call!(find_pubvar(self.amx, c_name.as_ptr(), transmute(&value)) => value)
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