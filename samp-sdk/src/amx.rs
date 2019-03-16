//! Core Amx types.
use crate::cell::{AmxCell, AmxPrimitive, AmxString, Buffer, Ref};
use crate::consts::{AmxExecIdx, AmxFlags};
use crate::error::{AmxError, AmxResult};
use crate::exports::*;
use crate::raw::types::{AMX, AMX_HEADER, AMX_NATIVE_INFO};

#[cfg(feature = "encoding")]
use crate::encoding;

use std::ffi::CString;
use std::ptr::NonNull;
use std::borrow::Cow;

macro_rules! amx_try {
    ($call:expr) => {
        let result = $call;

        if result > 0 {
            return Err(result.into());
        }
    };
}

/// A wrapper around a raw pointer to an AMX and exported functions.
#[derive(Debug)]
pub struct Amx {
    ptr: *mut AMX,
    fn_table: usize,
}

impl Amx {
    /// Create an AMX wrapper.
    ///
    /// # Arguments
    /// *ptr* is a raw pointer passed by callbacks (`AmxLoad` for example).
    ///
    /// *fn_table* is a address to the exported function from AMX.
    ///
    /// # Example
    /// ```
    /// use samp_sdk::amx::Amx;
    /// use samp_sdk::consts::ServerData;
    /// use samp_sdk::raw::types::AMX;
    ///
    /// static mut AMX_EXPORTS: usize = 0;
    ///
    /// #[no_mangle]
    /// pub extern "system" fn Load(server_data: *const usize) -> i32 {
    ///     unsafe {
    ///         AMX_EXPORTS = server_data.offset(ServerData::AmxExports.into()).read();
    ///     }
    ///
    ///     return 1;
    /// }
    ///
    /// #[no_mangle]
    /// pub extern "system" fn AmxLoad(amx_ptr: *mut AMX) {
    ///     let _amx = Amx::new(amx_ptr, unsafe { AMX_EXPORTS });
    /// }
    /// ```
    pub fn new(ptr: *mut AMX, fn_table: usize) -> Amx {
        Amx { ptr, fn_table }
    }

    /// Register a list of plugin natives functions.
    ///
    /// # Example
    /// ```
    /// use samp_sdk::amx::Amx;
    /// use samp_sdk::raw::types::{AMX, AMX_NATIVE_INFO};
    ///
    /// use std::ffi::CString;
    ///
    /// #[no_mangle]
    /// pub extern "system" fn AmxLoad(amx_ptr: *mut AMX) {
    /// #   let amx_exports = 0;
    ///     // let amx_exports = ...; // see `Amx::new()` example.
    ///     let amx = Amx::new(amx_ptr, amx_exports);
    ///     let native_name = CString::new("MyNativeName").unwrap(); // that's safe, because there is no zero chars.
    ///     
    ///     let natives = vec![AMX_NATIVE_INFO {
    ///         name: native_name.as_ptr(),
    ///         func: my_native,
    ///     }];
    ///
    ///     let _ = amx.register(&natives);
    /// }
    ///
    /// extern "C" fn my_native(amx: *mut AMX, args: *mut i32) -> i32 {
    ///     println!("YOYW!");
    ///     return 0;
    /// }
    /// ```
    pub fn register(&self, natives: &[AMX_NATIVE_INFO]) -> AmxResult<()> {
        let register = Register::from_table(self.fn_table);
        let len = natives.len();
        let ptr = natives.as_ptr();

        amx_try!(register(self.ptr, ptr, len as i32));

        Ok(())
    }

    pub(crate) fn allot<T: Sized + AmxPrimitive>(&self, cells: usize) -> AmxResult<Ref<T>> {
        let allot = Allot::from_table(self.fn_table);

        let mut amx_addr = 0;
        let mut phys_addr = 0;

        amx_try!(allot(self.ptr, cells as i32, &mut amx_addr, &mut phys_addr));

        unsafe { Ok(Ref::new(amx_addr, phys_addr as *mut T)) }
    }

    // TODO: return any type that can be converted to an amx cell

    /// Execs an AMX function.
    ///
    /// # Examples
    ///
    /// ```
    /// use samp_sdk::amx::Amx;
    ///
    /// fn log_player_money(amx: &Amx) {
    ///     let index = amx.find_public("AntiCheat_GetPlayerMoney").unwrap();
    ///     amx.push(1); // a player with ID 1
    ///
    ///     match amx.exec(index) {
    ///         Ok(money) => println!("Player {} has {} money.", 1, money),
    ///         Err(err) => println!("Error: {:?}", err),
    ///     }
    /// }
    /// ```
    pub fn exec(&self, index: AmxExecIdx) -> AmxResult<i32> {
        let exec = Exec::from_table(self.fn_table);
        let mut retval = 0;

        amx_try!(exec(self.ptr, &mut retval, index.into()));

        Ok(retval)
    }

    /// Returns an index of a native by its name.
    ///
    /// # Examples
    /// See `find_public` and `exec` examples.
    pub fn find_native(&self, name: &str) -> AmxResult<i32> {
        let find_native = FindNative::from_table(self.fn_table);
        let c_str = CString::new(name).map_err(|_| AmxError::NotFound)?;
        let mut index = -1;

        amx_try!(find_native(self.ptr, c_str.as_ptr(), &mut index));

        Ok(index)
    }

    /// Returns an index of a public by its name.
    ///
    /// # Examples
    ///
    /// ```
    /// use samp_sdk::amx::Amx;
    /// use samp_sdk::error::AmxResult;
    ///
    /// fn has_on_player_connect(amx: &Amx) -> AmxResult<bool> {
    ///     let public_index = amx.find_public("OnPlayerConnect")?;
    ///     Ok(i32::from(public_index) >= 0)
    /// }
    /// ```
    pub fn find_public(&self, name: &str) -> AmxResult<AmxExecIdx> {
        let find_public = FindPublic::from_table(self.fn_table);
        let c_str = CString::new(name).map_err(|_| AmxError::NotFound)?;
        let mut index = -1;

        amx_try!(find_public(self.ptr, c_str.as_ptr(), &mut index));

        Ok(AmxExecIdx::from(index))
    }

    /// Returns a pointer to a public variable.
    ///
    /// # Example
    /// ```rust,no_run
    /// use samp_sdk::amx::Amx;
    /// # use samp_sdk::error::AmxResult;
    ///
    /// # fn main() -> AmxResult<()> {
    /// # let amx = Amx::new(std::ptr::null_mut(), 0);
    /// // let amx = Amx::new(...);
    /// let version = amx.find_pubvar::<f32>("my_plugin_version")?;
    ///
    /// if *version < 1.0 {
    ///     println!("You're badass");
    /// } else {
    ///     println!("Alright!");
    /// }
    /// #   Ok(())
    /// # }
    /// ```
    pub fn find_pubvar<T: Sized + AmxPrimitive>(&self, name: &str) -> AmxResult<Ref<T>> {
        let find_pubvar = FindPubVar::from_table(self.fn_table);
        let c_str = CString::new(name).map_err(|_| AmxError::NotFound)?;
        let mut cell_ptr = 0;

        amx_try!(find_pubvar(self.ptr, c_str.as_ptr(), &mut cell_ptr));

        self.get_ref(cell_ptr)
    }

    /// Return flags of a compiled AMX.
    pub fn flags(&self) -> AmxResult<AmxFlags> {
        let flags = Flags::from_table(self.fn_table);
        let mut value: u16 = 0;

        amx_try!(flags(self.ptr, &mut value));

        Ok(AmxFlags::from_bits_truncate(value))
    }

    /// Get a reference ([`Ref<T>`]) to a value stored inside an AMX.
    ///
    /// # Example
    /// ```rust,no_run
    /// use samp_sdk::amx::Amx;
    /// # use samp_sdk::cell::Ref;
    /// # use samp_sdk::error::AmxResult;
    ///
    /// // fn test_native(&self, amx: &Amx, reference: Ref<f32>) -> AmxResult<f32>
    /// #
    /// # struct Plugin;
    /// #
    /// # impl Plugin {
    /// fn test_native(&self, amx: &Amx, cell_idx: i32) -> AmxResult<f32> {
    ///     let reference = amx.get_ref::<f32>(cell_idx)?;
    ///     return Ok(*reference)
    /// }
    /// # }
    /// ```
    ///
    /// [`Ref<T>`]: ../cell/struct.Ref.html
    pub fn get_ref<T: Sized + AmxPrimitive>(&self, address: i32) -> AmxResult<Ref<T>> {
        let amx_ptr = self.amx();
        let header_ptr = self.header();

        let amx = unsafe { amx_ptr.as_ref() };
        let header = unsafe { header_ptr.as_ref() };

        let data = if amx.data.is_null() {
            unsafe { amx.base.offset(header.dat as isize) }
        } else {
            amx.data
        };

        if address >= amx.hea && address < amx.stk || address < 0 || address >= amx.stp {
            return Err(AmxError::MemoryAccess);
        }

        let ptr = unsafe { data.offset(address as isize) };

        unsafe { Ok(Ref::new(address, ptr as *mut T)) }
    }

    #[inline(always)]
    pub(crate) fn release(&self, address: i32) {
        let mut amx = self.amx();
        let amx = unsafe { amx.as_mut() };

        if amx.hea > address {
            amx.hea = address;
        }
    }

    /// Push a value that implements [`AmxCell`] to an AMX stack.
    ///
    /// [`AmxCell`]: ../cell/repr/trait.AmxCell.html
    pub fn push<'a, T: AmxCell<'a>>(&'a self, value: T) -> AmxResult<()> {
        let push = Push::from_table(self.fn_table);

        amx_try!(push(self.ptr, value.as_cell()));

        Ok(())
    }

    /// Get a heap [`Allocator`] for current [`Amx`].
    ///
    /// # Example
    /// ```rust,no_run
    /// use samp_sdk::amx::Amx;
    ///
    /// # use samp_sdk::error::AmxResult;
    /// # use samp_sdk::consts::AmxExecIdx;
    /// #
    /// # fn main() -> AmxResult<()> {
    /// # let amx = Amx::new(std::ptr::null_mut(), 0);
    /// let allocator = amx.allocator();
    /// let string = allocator.allot_string("Hello!")?;
    /// let player_id = 10;
    ///
    /// amx.push(string)?;
    /// amx.push(player_id)?;
    /// amx.exec(AmxExecIdx::UserDef(21))?;
    /// #
    /// #       Ok(())
    /// # }
    /// ```
    ///
    /// [`Allocator`]: struct.Allocator.html
    /// [`Amx`]: struct.Amx.html
    pub fn allocator(&self) -> Allocator {
        Allocator::new(self)
    }

    /// Returns a pointer to a raw [`AMX`] structure.
    ///
    /// [`AMX`]: ../raw/types/struct.AMX.html
    pub fn amx(&self) -> NonNull<AMX> {
        unsafe { NonNull::new_unchecked(self.ptr) }
    }

    /// Returns a pointer to an [`AMX_HEADER`].
    ///
    /// [`AMX_HEADER`]: ../raw/types/struct.AMX_HEADER.html
    pub fn header(&self) -> NonNull<AMX_HEADER> {
        unsafe { NonNull::new_unchecked((*self.ptr).base as *mut AMX_HEADER) }
    }
}

/// AMX memory allocator (on the heap) that frees captured memory after drop.
pub struct Allocator<'amx> {
    amx: &'amx Amx,
    release_addr: i32,
}

impl<'amx> Allocator<'amx> {
    pub(crate) fn new(amx: &'amx Amx) -> Allocator<'amx> {
        let amx_ptr = amx.amx();
        let amx_ptr = unsafe { amx_ptr.as_ref() };

        Allocator {
            amx,
            release_addr: amx_ptr.hea,
        }
    }

    /// Allocate memory for a primitive value.
    ///
    /// # Example
    /// ```rust,no_run
    /// use samp_sdk::amx::Amx;
    ///
    /// # use samp_sdk::error::AmxResult;
    /// # use samp_sdk::consts::AmxExecIdx;
    /// #
    /// # fn main() -> AmxResult<()> {
    /// # let amx = Amx::new(std::ptr::null_mut(), 0);
    /// // forward SomePublicFunc(player_id, &Float:health);
    /// let public_fn = amx.find_public("SomePublicFunc")?;
    /// let allocator = amx.allocator();
    ///
    /// let float_ref = allocator.allot(1.2f32)?;
    /// let player_id = 10;
    ///
    /// amx.push(float_ref)?;
    /// amx.push(player_id)?;
    /// amx.exec(public_fn)?;
    /// #
    /// #       Ok(())
    /// # }
    /// ```
    pub fn allot<T: Sized + AmxPrimitive>(&self, init_value: T) -> AmxResult<Ref<T>> {
        let mut cell = self.amx.allot(1)?;
        *cell = init_value;

        Ok(cell)
    }

    /// Allocate custom sized buffer on the heap.
    ///
    /// # Example
    /// ```rust,no_run
    /// use samp_sdk::amx::Amx;
    ///
    /// # use samp_sdk::error::AmxResult;
    /// # use samp_sdk::consts::AmxExecIdx;
    /// #
    /// # fn main() -> AmxResult<()> {
    /// # let amx = Amx::new(std::ptr::null_mut(), 0);
    /// // forward SomePublicFunc(player_id, ids[], size);
    /// let public_fn = amx.find_public("SomePublicFunc")?;
    /// let allocator = amx.allocator();
    ///
    /// let size = 3;
    /// let mut buffer = allocator.allot_buffer(size)?;
    /// let player_id = 10;
    ///
    /// buffer[0] = 5;
    /// buffer[1] = 2;
    /// buffer[2] = 15;
    ///
    /// amx.push(size)?;
    /// amx.push(buffer)?;
    /// amx.push(player_id)?;
    /// amx.exec(public_fn)?;
    /// #
    /// #       Ok(())
    /// # }
    pub fn allot_buffer(&self, size: usize) -> AmxResult<Buffer> {
        let buffer = self.amx.allot(size)?;

        Ok(Buffer::new(buffer, size))
    }

    /// Allocate an array on the heap, copy values from the passed array and return `Buffer` containing reference to the allocated cell.
    ///
    /// # Example
    /// ```rust,no_run
    /// use samp_sdk::amx::Amx;
    ///
    /// # use samp_sdk::error::AmxResult;
    /// # use samp_sdk::consts::AmxExecIdx;
    /// #
    /// # fn main() -> AmxResult<()> {
    /// # let amx = Amx::new(std::ptr::null_mut(), 0);
    /// // forward SomePublicFunc(player_id, ids[], size);
    /// let public_fn = amx.find_public("SomePublicFunc")?;
    /// let allocator = amx.allocator();
    ///
    /// let buffer = allocator.allot_array(&[5, 2, 15])?;
    /// let player_id = 10;
    ///
    /// amx.push(buffer.len())?;
    /// amx.push(buffer)?;
    /// amx.push(player_id)?;
    /// amx.exec(public_fn)?;
    /// #
    /// #       Ok(())
    /// # }
    pub fn allot_array<T>(&self, array: &[T]) -> AmxResult<Buffer>
    where
        T: AmxCell<'amx> + AmxPrimitive,
    {
        let mut buffer = self.allot_buffer(array.len())?;

        let slice = buffer.as_mut_slice();

        for (idx, item) in array.iter().enumerate() {
            slice[idx] = item.as_cell();
        }

        Ok(buffer)
    }

    /// Alocate a string, copy passed `&str` and return `AmxString` pointing to an `Amx` cell.
    ///
    /// # Example
    /// ```rust,no_run
    /// use samp_sdk::amx::Amx;
    ///
    /// # use samp_sdk::error::AmxResult;
    /// # use samp_sdk::consts::AmxExecIdx;
    /// #
    /// # fn main() -> AmxResult<()> {
    /// # let amx = Amx::new(std::ptr::null_mut(), 0);
    /// // forward SomePublicFunc(player_id, new_name[]);
    /// let public_fn = amx.find_public("SomePublicFunc")?;
    /// let allocator = amx.allocator();
    ///
    /// let string = allocator.allot_string("hello")?;
    /// let player_id = 10;
    ///
    /// amx.push(string)?;
    /// amx.push(player_id)?;
    /// amx.exec(public_fn)?;
    /// #
    /// #       Ok(())
    /// # }
    pub fn allot_string(&self, string: &str) -> AmxResult<AmxString> {
        let bytes = Allocator::string_bytes(string);
        let buffer = self.allot_buffer(bytes.len() + 1)?;

        Ok(unsafe { AmxString::new(buffer, bytes.as_ref()) })
    }

    fn string_bytes<'a>(string: &'a str) -> Cow<'a, [u8]> {
        #[cfg(feature = "encoding")]
        return encoding::get().encode(string).0;

        #[cfg(not(feature = "encoding"))]
        return Cow::from(string.as_bytes());
    }
}

impl Drop for Allocator<'_> {
    fn drop(&mut self) {
        // AMX::release never fails
        self.amx.release(self.release_addr);
    }
}
