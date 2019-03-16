//! Different smart-pointers to work around raw AMX values.
use std::marker::PhantomData;
use std::ops::{Deref, DerefMut};

use crate::amx::Amx;
use crate::error::AmxResult;

pub mod buffer;
pub mod repr;
pub mod string;

pub use buffer::{Buffer, UnsizedBuffer};
pub use repr::{AmxCell, AmxPrimitive};
pub use string::AmxString;

/// A reference to a cell in the [`Amx`].
///
/// # Notes
/// This type implements [`Deref`] trait that allows you read or write to inner value (like smart pointers [`Box<T>`], [`Rc<T>`]).
///
/// [`Amx`]: ../amx/struct.Amx.html
/// [`Deref`]: https://doc.rust-lang.org/std/ops/trait.Deref.html
/// [`Box<T>`]: https://doc.rust-lang.org/std/boxed/struct.Box.html
/// [`Rc<T>`]: https://doc.rust-lang.org/std/rc/struct.Rc.html
pub struct Ref<'amx, T: Sized + AmxPrimitive> {
    amx_addr: i32,
    phys_addr: *mut T,
    marker: PhantomData<&'amx Amx>,
}

impl<'amx, T: Sized + AmxPrimitive> Ref<'amx, T> {
    /// Create a new wrapper over an AMX cell.
    ///
    /// # Safety
    /// `Ref<T>` **should** be alive as long as `phys_addr` or it will dangling pointer.
    ///
    /// It's not recomended to use directly, instead get a reference from [`Args`] or [`Amx::get_ref`].
    ///
    /// [`Args`]: ../args/struct.Args.html
    /// [`Amx::get_ref`]: ../amx/struct.Amx.html#method.get_ref
    pub unsafe fn new(amx_addr: i32, phys_addr: *mut T) -> Ref<'amx, T> {
        Ref {
            amx_addr,
            phys_addr,
            marker: PhantomData,
        }
    }

    /// Get an inner AMX address to cell (not physical).
    ///
    /// # Example
    /// ```
    /// # use samp_sdk::amx::Amx;
    /// use samp_sdk::cell::Ref;
    /// fn native_fn(amx: &Amx, arg: Ref<usize>) {
    ///     let cell_addr = arg.address();
    ///     println!("The argument stored in the {} cell.", cell_addr);
    /// }
    /// ```
    #[inline]
    pub fn address(&self) -> i32 {
        self.amx_addr
    }

    /// Get a pointer to a memory cell.
    #[inline]
    pub fn as_ptr(&self) -> *const T {
        self.phys_addr
    }

    /// Get a mutable pointer to a memory cell.
    #[inline]
    pub fn as_mut_ptr(&mut self) -> *mut T {
        self.phys_addr
    }
}

impl<T: Sized + AmxPrimitive> Deref for Ref<'_, T> {
    type Target = T;

    fn deref(&self) -> &T {
        unsafe { &*self.phys_addr }
    }
}

impl<T: Sized + AmxPrimitive> DerefMut for Ref<'_, T> {
    fn deref_mut(&mut self) -> &mut T {
        unsafe { &mut *self.phys_addr }
    }
}

impl<'amx, T: Sized + AmxPrimitive> AmxCell<'amx> for Ref<'amx, T> {
    fn from_raw(amx: &'amx Amx, cell: i32) -> AmxResult<Ref<'amx, T>> {
        amx.get_ref(cell)
    }

    fn as_cell(&self) -> i32 {
        self.address()
    }
}
