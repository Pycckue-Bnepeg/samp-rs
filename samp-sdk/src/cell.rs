use std::ops::{Deref, DerefMut};
use std::marker::PhantomData;

use crate::amx::Amx;
use crate::error::AmxResult;

pub mod repr;
pub mod buffer;
pub mod string;

pub use repr::{Cell, AmxPrimitive};
pub use buffer::{Buffer, UnsizedBuffer};
pub use string::AmxString;

/// A reference to a cell in the AMX.
pub struct Ref<'amx, T: Sized + AmxPrimitive> {
    amx_addr: i32,
    phys_addr: *mut T,
    marker: PhantomData<&'amx Amx>,
}

impl<'amx, T: Sized + AmxPrimitive> Ref<'amx, T> {
    /// Create a new wrapper over an AMX cell.
    pub unsafe fn new(amx_addr: i32, phys_addr: *mut T) -> Ref<'amx, T> {
        Ref {
            amx_addr,
            phys_addr,
            marker: PhantomData,
        }
    }

    #[inline]
    pub fn address(&self) -> i32 {
        self.amx_addr
    }

    #[inline]
    pub fn as_ptr(&self) -> *const T {
        self.phys_addr
    }

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

impl<'amx, T: Sized + AmxPrimitive> Cell<'amx> for Ref<'amx, T> {
    fn from_raw(amx: &'amx Amx, cell: i32) -> AmxResult<Ref<'amx, T>> {
        amx.get_ref(cell)
    }

    fn as_cell(&self) -> i32 {
        self.address()
    }
}