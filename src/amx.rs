/*!
    Core of SDK to interact with AMX.
*/

use std::ptr::{read};
use std::mem::{transmute, transmute_copy, size_of};
use std::ffi::CString;
use types;
use types::Cell;
use consts::*;
use data::amx_functions;

pub type AmxResult<T> = Result<T, AmxError>;

/// Converts a raw AMX error to `AmxError`.
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

/// Makes an call to any AMX functions and uses `ret!`.
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

/// Gets a function from a raw pointer in `data::amx_functions`.
macro_rules! import {
    ($type:ident) => {
        unsafe {
            read(amx_functions.offset(Exports::$type as isize) as *const $crate::types::$type)
        }
    };
}

/// AMX struct that holds raw `types::AMX` pointer.
pub struct AMX {
    amx: *mut types::AMX,
}

impl AMX {
    /// Converts a raw `types::AMX` pointer.
    ///
    /// Shouldn't used directly in your code. `AMX::new()` is calling from raw natives functions.
    pub fn new(amx: *mut types::AMX) -> AMX {
        AMX {
            amx,
        }
    }

    /// Registers natives functions
    ///
    /// # Examples
    ///
    /// ```
    /// #[macro_use] extern crate samp_sdk;
    /// use samp_sdk::types;
    /// use samp_sdk::amx::{AMX, AmxResult};
    /// use samp_sdk::consts::AMX_ERR_NONE;
    ///
    /// extern "system" fn some_function(_: *mut types::AMX, _: *mut i32) -> i32 { 0 }
    /// extern "system" fn another_function(_: *mut types::AMX, _: *mut i32) -> i32 { 0 }
    ///
    /// fn amx_load(amx: &AMX) -> types::Cell {
    ///
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

    /// Allocates memory cells inside AMX.
    ///
    /// # Return
    /// Return tuple of two addresses:
    /// * The address of the variable relatived to AMX data section.
    /// * The physical address.
    ///
    /// # Examples
    /// Allot one cell to a reference to a value and push it.
    /// ```
    /// use samp_sdk::amx::{AMX, AmxResult};
    /// use samp_sdk::types::Cell;
    ///
    /// fn allocate(amx: &AMX) -> AmxResult<()> {
    ///     let (amx_addr, phys_addr) = amx.allot(1)?;
    ///     let value = phys_addr as *mut Cell;
    ///
    ///     unsafe { *value = 655; }
    ///     amx.push(amx_addr)?;
    ///
    ///     Ok(())
    /// }
    /// ```
    pub fn allot(&self, cells: usize) -> AmxResult<(Cell, usize)> {
        let amx_addr = 0;
        let phys_addr = 0;

        let allot = import!(Allot);

        unsafe {
            call!(allot(self.amx, cells as i32, transmute(&amx_addr), transmute(&phys_addr)) => (amx_addr, phys_addr))
        }
    }

    /// Frees all memory **above** input address.
    pub fn release(&self, address: Cell) -> AmxResult<()> {
        let release = import!(Release);
        call!(release(self.amx, address) => ())
    }

    /// Returns flags of compiled AMX.
    ///
    /// # Examples
    /// ```
    /// #[macro_use] extern crate samp_sdk;
    /// use samp_sdk::amx::AMX;
    /// use samp_sdk::consts::AMX_FLAG_DEBUG;
    ///
    /// fn check_flags(amx: &AMX) {
    ///     let flags = amx.flags().unwrap();
    ///     let has_debug_flag = flags & AMX_FLAG_DEBUG == AMX_FLAG_DEBUG;
    ///
    ///     if (has_debug_flag) {
    ///         log!("AMX has debug information");
    ///     }
    /// }
    /// ```
    pub fn flags(&self) -> AmxResult<u16> {
        let flags = import!(Flags);
        let value: u16 = 0;

        unsafe {
            call!(flags(self.amx, transmute(&value)) => value)
        }
    }

    /// Returns memory information.
    ///
    /// All sizes in bytes.
    /// # Examples
    /// ```
    /// #[macro_use] extern crate samp_sdk;
    /// use samp_sdk::amx::AMX;
    ///
    /// fn log_mem_info(amx: &AMX) {
    ///    let (codesize, datasize, stackheap) = amx.mem_info().unwrap();
    ///    log!("Size of code section {}, data section {}, stack and heap {}", codesize, datasize, stackheap);
    /// }
    /// ```
    pub fn mem_info(&self) -> AmxResult<(i64, i64, i64)> {
        let mem_info = import!(MemInfo);
        let codesize: i64 = 0;
        let datasize: i64 = 0;
        let stackheap: i64 = 0;

        unsafe {
            call!(mem_info(self.amx, transmute(&codesize), transmute(&datasize), transmute(&stackheap)) => (codesize, datasize, stackheap))
        }
    }

    /// Get an address of a reference value given to native.
    ///
    /// # Examples
    ///
    /// ```
    /// use samp_sdk::amx::AMX;
    /// use samp_sdk::types::Cell;
    ///
    /// // native: SomeNative(&int_value);
    /// fn some_native(amx: &AMX, args: *mut Cell) {
    ///     let ptr = unsafe {
    ///         std::ptr::read(args.offset(1))
    ///     };
    ///     let int_value: &mut i32 = amx.get_address(ptr).unwrap();
    ///     *int_value = 10;
    /// }
    /// ```
    pub fn get_address<'a, T: Sized>(&self, address: Cell) -> AmxResult<&'a mut T> {
        unsafe {
            let header = (*self.amx).base as *const types::AMX_HEADER;

            let data = if (*self.amx).data.is_null() {
                (*self.amx).base as usize + (*header).dat as usize
            } else {
                (*self.amx).data as usize
            };

            if address >= (*self.amx).hea && address < (*self.amx).stk || address < 0 || address >= (*self.amx).stp {
                Err(AmxError::MemoryAccess)
            } else {
                Ok(transmute(data + address as usize))
            }
        }
    }

    /// Pushes a primitive value or an address to AMX stack.
    ///
    /// # Examples
    ///
    /// ```
    /// use samp_sdk::amx::AMX;
    ///
    /// fn change_hp(amx: &AMX, player_id: u32, health: f32) {
    ///     let index = amx.find_public("OnPlayerHPChanged").unwrap();
    ///
    ///     amx.push(health);
    ///     amx.push(player_id);
    ///     amx.exec(index);
    /// }
    /// ```
    pub fn push<T: Sized>(&self, value: T) -> AmxResult<()> {
        let push = import!(Push);

        unsafe {
            call!(push(self.amx, transmute_copy(&value)) => ())
        }
    }

    /// Pushes a slice to the AMX stack.
    ///
    /// # Examples
    ///
    /// ```
    /// use samp_sdk::amx::{AMX, AmxResult};
    ///
    /// fn call_with_array(amx: &AMX) -> AmxResult<()> {
    ///     let func = amx.find_public("GiveMeArray")?;
    ///     let player_data = vec![1, 2, 3, 4, 5];
    ///     let player_ids = vec![1, 2, 3, 4, 5];
    ///
    ///     let amx_addr = amx.push_array(&player_data)?; // push an array and save address relatived to first item on the heap.
    ///     amx.push_array(&player_ids)?; // push the next array
    ///     amx.exec(func)?; // exec the public
    ///     amx.release(amx_addr)?; // release all allocated memory inside AMX
    ///
    ///     Ok(())
    /// }
    /// ```
    pub fn push_array<T: Sized>(&self, array: &[T]) -> AmxResult<Cell> {
        let (amx_addr, phys_addr) = self.allot(array.len())?;
        let dest = phys_addr as *mut Cell;

        for i in 0..array.len() {
            unsafe {
                *(dest.offset(i as isize)) = transmute_copy(&array[i]);
            }
        }

        self.push(amx_addr)?;
        Ok(amx_addr)
    }

    /// Allots memory for a string and pushes it to the AMX stack.
    ///
    /// Please, don't use it directly! Better use macros `exec!`, `exec_public!` and `exec_native!`.
    pub fn push_string(&self, string: &str, packed: bool) -> AmxResult<Cell> {
        if packed {
            unimplemented!()
        } else {
            let bytes = string.as_bytes();
            let (amx_addr, phys_addr) = self.allot(bytes.len() + 1)?;
            let dest = phys_addr as *mut Cell;

            for i in 0..string.len() {
                unsafe {
                    *(dest.offset(i as isize)) = transmute_copy(&bytes[i]);
                }
            }

            unsafe {
                *(dest.offset(string.len() as isize)) = 0;
            }

            self.push(amx_addr)?;
            Ok(amx_addr)
        }
    }

    /// Execs an AMX function.
    ///
    /// # Examples
    ///
    /// ```
    /// #[macro_use] extern crate samp_sdk;
    /// use samp_sdk::amx::AMX;
    ///
    /// fn log_player_money(amx: &AMX) {
    ///     let index = amx.find_native("GetPlayerMoney").unwrap();
    ///     amx.push(1); // a player with ID 1
    ///
    ///     match amx.exec(index) {
    ///         Ok(money) => log!("Player has {} money.", money),
    ///         Err(err) => log!("Error: {:?}", err),
    ///     }
    /// }
    /// ```
    pub fn exec(&self, index: i32) -> AmxResult<i64> {
        let exec = import!(Exec);

        let retval = -1;
        unsafe {
            call!(exec(self.amx, transmute(&retval), index) => retval)
        }
    }

    /// Returns an index of a public by its name.
    ///
    /// # Examples
    ///
    /// ```
    /// use samp_sdk::amx::AMX;
    ///
    /// fn hasOnPlayerConnect(amx: &AMX) -> bool {
    ///     let public_index = amx.find_public("OnPlayerConnect").unwrap();
    ///     public_index >= 0
    /// }
    /// ```
    pub fn find_public(&self, name: &str) -> AmxResult<i32> {
        let find_public = import!(FindPublic);

        let index = -1;
        let c_name = CString::new(name).unwrap();

        unsafe {
            call!(find_public(self.amx, c_name.as_ptr(), transmute(&index)) => index)
        }
    }

    /// Returns an index of a native by its name.
    ///
    /// # Examples
    /// See `find_public` and `exec` examples.
    pub fn find_native(&self, name: &str) -> AmxResult<i32> {
        let find_native = import!(FindNative);

        let index = -1;
        let c_name = CString::new(name).unwrap();

        unsafe {
            call!(find_native(self.amx, c_name.as_ptr(), transmute(&index)) => index)
        }
    }

    /// Returns a pointer to a public variable.
    pub fn find_pubvar<T: Sized>(&self, name: &str) -> AmxResult<&mut T> {
        let find_pubvar = import!(FindPubVar);

        let value: Cell = 0;
        let c_name = CString::new(name).unwrap();

        unsafe {
            let retval = find_pubvar(self.amx, c_name.as_ptr(), transmute(&value));

            if retval == 0 {
                self.get_address(value)
            } else {
                Err(AmxError::from(retval))
            }
        }
    }

    /// Gets length of a string.
    pub fn string_len(&self, address: *const Cell) -> AmxResult<usize> {
        let string_len = import!(StrLen);
        let mut length = 0;

        call!(string_len(address, &mut length) => length as usize)
    }

    /// Gets a string from AMX.
    ///
    /// # Examples
    ///
    /// ```
    /// #[macro_use] extern crate samp_sdk;
    /// use samp_sdk::amx::{AMX, AmxResult};
    /// use samp_sdk::types::Cell;
    ///
    /// pub struct MyPlugin;
    ///
    /// impl MyPlugin {
    ///
    ///     fn raw_function(&self, amx: &AMX, params: *mut Cell) -> AmxResult<Cell> {
    ///         unsafe {
    ///             let ptr = std::ptr::read(params.offset(1));
    ///             let mut addr = amx.get_address::<i32>(ptr)?; // get a pointer from amx
    ///             let len = amx.string_len(addr)?; // get string length in amx
    ///             let string = amx.get_string(addr, len + 1)?; // convert amx string to rust String
    ///
    ///             log!("got string: {}", string);
    ///
    ///             std::mem::forget(addr);
    ///         }
    ///
    ///         Ok(0)
    ///     }
    ///
    /// }
    /// ```
    pub fn get_string(&self, address: *const Cell, size: usize) -> AmxResult<String> {
        const UNPACKEDMAX: u32 = ((1u32 << (size_of::<u32>() - 1) * 8) - 1u32);
        const CHARBITS: usize = 8 * size_of::<u8>();

        let mut string = Vec::with_capacity(size);

        unsafe {
            if read(address) as u32 > UNPACKEDMAX {
                // packed string
                let mut i = size_of::<Cell>() - 1;
                let mut cell = 0;
                let mut ch;
                let mut length = 0;
                let mut offset = 0;

                while length < size {
                    if i == size_of::<Cell>() - 1 {
                        cell = read(address.offset(offset));
                        offset += 1;
                    }

                    ch = (cell >> i * CHARBITS) as u8;

                    if ch == 0 {
                        break;
                    }

                    string.push(ch);
                    length += 1;
                    i = (i + size_of::<Cell>() - 1) % size_of::<Cell>();
                }
            } else {
                let mut length = 0;
                let mut byte = read(address.offset(length));

                while byte != 0 && length < size as isize {
                    string.push(byte as u8);
                    length += 1;
                    byte = read(address.offset(length));
                }
            }
            Ok(String::from_utf8_unchecked(string))
        }
    }

    /// Raises an AMX error.
    pub fn raise_error(&self, error: AmxError) -> AmxResult<()> {
        let raise_error = import!(RaiseError);
        call!(raise_error(self.amx, error as i32) => ())
    }
}

/// Custom error type for AMX errors.
/// Can be casted from i32
///
/// # Examples
///
/// ```
/// use samp_sdk::amx::{AMX, AmxResult, AmxError};
/// use samp_sdk::types::Cell;
///
/// fn throw_exit_error(amx: &AMX, params: *mut Cell) -> AmxResult<Cell> {
///     let error_id = 1;
///     let error = AmxError::from(error_id);
///
///     Err(error)
/// }
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
    HeapLow = 8,
    Callback = 9,
    Native = 10,
    Divide = 11,
    Sleep = 12,
    InvalidState = 13,
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
            8 => AmxError::HeapLow,
            9 => AmxError::Callback,
            10 => AmxError::Native,
            11 => AmxError::Divide,
            12 => AmxError::Sleep,
            13 => AmxError::InvalidState,
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