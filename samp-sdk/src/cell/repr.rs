use crate::amx::Amx;
use crate::error::{AmxResult, AmxError};

/// Cell trait is a core trait of whole SDK.
/// It shows that value can be borrowed (or copied if it's a primitive) from AMX and passed to it.
pub trait Cell<'amx> where Self: Sized {
    fn from_raw(_amx: &'amx Amx, _cell: i32) -> AmxResult<Self> where Self: 'amx {
        Err(AmxError::General)
    }

    /// Get a value which can be passed to AMX.
    fn as_cell(&self) -> i32;
}

/// A marker showing that a value can be stored directly on a stack or a heap of an AMX.
/// Types: i8, u8, i16, u16, i32, u32, usize, isize, f32, bool
/// There is no values that's bigger than 4 bytes, because size of an AMX cell is 32 bits.
pub unsafe trait AmxPrimitive where Self: Sized {}

impl<'a, T: Cell<'a>> Cell<'a> for &'a T {
    fn as_cell(&self) -> i32 {
        (**self).as_cell()
    }
}

impl<'a, T: Cell<'a>> Cell<'a> for &'a mut T {
    fn as_cell(&self) -> i32 {
        (**self).as_cell()
    }
}

macro_rules! impl_for_primitive {
    ($type:ty) => {
        impl Cell<'_> for $type {
            fn from_raw(_amx: &Amx, cell: i32) -> AmxResult<Self> {
                Ok(cell as Self)
            }

            fn as_cell(&self) -> i32 {
                *self as i32
            }
        }

        unsafe impl AmxPrimitive for $type {}
    };
}

impl_for_primitive!(i8);
impl_for_primitive!(u8);
impl_for_primitive!(i16);
impl_for_primitive!(u16);
impl_for_primitive!(i32);
impl_for_primitive!(u32);
impl_for_primitive!(usize);
impl_for_primitive!(isize);

impl Cell<'_> for f32 {
    fn from_raw(_amx: &Amx, cell: i32) -> AmxResult<f32> {
        Ok(unsafe { std::mem::transmute(cell) })
    }

    fn as_cell(&self) -> i32 {
        // can't use `as` here because a float value will be an integer
        // for example if you pass 10.0 (0x41200000) it will be 10 (0x0A)
        unsafe { std::mem::transmute(*self) }
    }
}

impl Cell<'_> for bool {
    fn from_raw(_amx: &Amx, cell: i32) -> AmxResult<bool> {
        // just to be sure that boolean value will be correct I don't use there `std::mem::transmute` or `as` keyword.
        Ok(cell != 0)
    }

    fn as_cell(&self) -> i32 {
        if *self { 1 } else { 0 }
    }
}

unsafe impl AmxPrimitive for f32 {}
unsafe impl AmxPrimitive for bool {}