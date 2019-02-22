use std::fmt;

use super::{Buffer, Cell, UnsizedBuffer};
use crate::amx::Amx;
use crate::error::AmxResult;

pub struct AmxString<'amx> {
    inner: Buffer<'amx>,
}

impl<'amx> AmxString<'amx> {
    pub fn new(mut buffer: Buffer<'amx>, string: &str) -> AmxString<'amx> {
        let bytes = string.as_bytes();

        for (idx, byte) in bytes.iter().enumerate() {
            buffer[idx] = *byte as i32;
        }

        AmxString {
            inner: buffer,
        }
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        let mut vec = Vec::with_capacity(self.inner.len());

        // packed string
        if self.inner[0] > 0x00FFFFFF {
            unsafe {
                std::ptr::copy(self.inner.as_ptr() as *const u8, vec.as_mut_ptr(), vec.len());
            }
        } else {
            for (idx, item) in vec.iter_mut().enumerate() {
                *item = self.inner[idx] as u8;
            }
        }

        return vec;
    }

    pub fn to_string(&self) -> String {
        unsafe {
            String::from_utf8_unchecked(self.to_bytes())
        }
    }
}

impl<'amx> Cell<'amx> for AmxString<'amx> {
    fn from_raw(amx: &'amx Amx, cell: i32) -> AmxResult<AmxString<'amx>> {
        let buffer = UnsizedBuffer::from_raw(amx, cell)?;

        let length = unsafe {
            let mut ptr = buffer.as_ptr();
            let mut length = 0;

            while ptr.read() != 0 {
                length += 1;
                ptr = ptr.offset(1);
            }

            length
        };

        Ok(AmxString {
            inner: buffer.into_sized_buffer(length),
        })
    }

    fn as_cell(&self) -> i32 {
        self.inner.as_cell()
    }
}

impl fmt::Display for AmxString<'_> {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(fmt, "{}", self.to_string())
    }
}