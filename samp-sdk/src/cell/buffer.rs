//! Contains types to interact with AMX arrays.
use std::ops::{Deref, DerefMut};

use super::{AmxCell, Ref};
use crate::amx::Amx;
use crate::error::AmxResult;

/// Contains a pointer to sequence of `Amx` cells.
///
/// Can be dereferenced to a [`slice`].
///
/// # Example
/// ```
/// use samp_sdk::cell::{UnsizedBuffer, Buffer};
/// # use samp_sdk::amx::Amx;
///
/// // native: IGiveYouABuffer(buffer[]);
/// fn it_gave_me_a_buffer(amx: &Amx, buffer: UnsizedBuffer, size: usize) {
///     let mut buffer: Buffer = buffer.into_sized_buffer(size);
///     
///     println!("Got {:?}", buffer);
///     
///     buffer.iter_mut().map(|elem| *elem = *elem * 2);
///
///     println!("Change to {:?}", buffer);
/// }
/// ```
///
/// [`slice`]: https://doc.rust-lang.org/std/primitive.slice.html
pub struct Buffer<'amx> {
    inner: Ref<'amx, i32>,
    len: usize,
}

impl<'amx> Buffer<'amx> {
    /// Create a buffer from a reference to its first element.
    pub fn new(reference: Ref<'amx, i32>, len: usize) -> Buffer<'amx> {
        Buffer {
            inner: reference,
            len,
        }
    }

    /// Extracts a slice containing the entire buffer.
    #[inline]
    pub fn as_slice(&self) -> &[i32] {
        unsafe { std::slice::from_raw_parts(self.inner.as_ptr(), self.len) }
    }

    /// Extracts a mutable slice of the entire buffer.
    #[inline]
    pub fn as_mut_slice(&mut self) -> &mut [i32] {
        unsafe { std::slice::from_raw_parts_mut(self.inner.as_mut_ptr(), self.len) }
    }
}

// Buffer cannot be parsed
impl<'amx> AmxCell<'amx> for Buffer<'amx> {
    #[inline]
    fn as_cell(&self) -> i32 {
        self.inner.as_cell()
    }
}

impl Deref for Buffer<'_> {
    type Target = [i32];

    fn deref(&self) -> &[i32] {
        self.as_slice()
    }
}

impl DerefMut for Buffer<'_> {
    fn deref_mut(&mut self) -> &mut [i32] {
        self.as_mut_slice()
    }
}

impl std::fmt::Debug for Buffer<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self.as_slice())
    }
}

/// It's more like a temorary buffer that comes from AMX when a native is calling.
///
/// # Example
/// ```
/// use samp_sdk::cell::UnsizedBuffer;
/// # use samp_sdk::amx::Amx;
/// # use samp_sdk::error::AmxResult;
///
/// fn null_my_array(amx: &Amx, array: UnsizedBuffer, length: usize) -> AmxResult<u32> {
///     let mut array = array.into_sized_buffer(length);
///
///     unsafe {
///         let slice = array.as_mut_slice();
///         std::ptr::write_bytes(slice.as_mut_ptr(), 0, length);
///     }
///
///     return Ok(1)
/// }
/// ```
pub struct UnsizedBuffer<'amx> {
    inner: Ref<'amx, i32>,
}

impl<'amx> UnsizedBuffer<'amx> {
    /// Convert `UnsizedBuffer` into `Buffer` with given length.
    ///
    /// # Example
    /// ```
    /// use samp_sdk::cell::UnsizedBuffer;
    /// # use samp_sdk::amx::Amx;
    ///
    /// fn push_ones(amx: &Amx, array: UnsizedBuffer, length: usize) {
    ///     let mut buffer = array.into_sized_buffer(length);
    ///     let slice = buffer.as_mut_slice();
    ///     
    ///     for item in slice.iter_mut() {
    ///         *item = 1;
    ///     }
    /// }
    /// ```
    pub fn into_sized_buffer(self, len: usize) -> Buffer<'amx> {
        Buffer::new(self.inner, len)
    }

    /// Return a raw pointer to an inner value.
    #[inline]
    pub fn as_ptr(&self) -> *const i32 {
        self.inner.as_ptr()
    }

    /// Return a mutable raw pointer to an inner value.
    #[inline]
    pub fn as_mut_ptr(&mut self) -> *mut i32 {
        self.inner.as_mut_ptr()
    }
}

impl<'amx> AmxCell<'amx> for UnsizedBuffer<'amx> {
    fn from_raw(amx: &'amx Amx, cell: i32) -> AmxResult<UnsizedBuffer<'amx>> {
        Ok(UnsizedBuffer {
            inner: amx.get_ref(cell)?,
        })
    }

    #[inline]
    fn as_cell(&self) -> i32 {
        self.inner.as_cell()
    }
}
