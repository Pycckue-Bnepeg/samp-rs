use std::fmt;

use super::{Buffer, AmxCell, UnsizedBuffer};
use crate::amx::Amx;
use crate::error::AmxResult;

const MAX_UNPACKED: i32 = 0x00FFFFFF;

// A wrapper around an AMX string.
pub struct AmxString<'amx> {
    inner: Buffer<'amx>,
    // real length of the string
    len: usize,
}

impl<'amx> AmxString<'amx> {
    /// Create a new AmxString from an allocated buffer and fill it with a string
    pub fn new(mut buffer: Buffer<'amx>, string: &str) -> AmxString<'amx> {
        let bytes = string.as_bytes();

        for (idx, byte) in bytes.iter().enumerate() {
            buffer[idx] = *byte as i32;
        }

        AmxString {
            len: buffer.len(),
            inner: buffer,
        }
    }

    /// Convert an AMX string to a `Vec<u8>`.
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut vec = Vec::with_capacity(self.len);

        // packed string
        if self.inner[0] > MAX_UNPACKED {
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

    /// Convert an AMX string to a `String`.
    /// Only ASCII chars by default. Pass `cp1251` to crate features to enable Windows 1251 encoding (TODO).
    /// 
    /// # Example
    /// ```
    /// #[native(name = "LogError")]
    /// fn log_error(&self, amx: &Amx, text: AmxString) -> AmxResult<bool> {
    ///     if !self.logger_enabled {
    ///         return false;
    ///     }
    /// 
    ///     let string = text.to_string();
    ///     println!("[{}] PluginName error: {}", current_date(), string);
    /// 
    ///     return true;
    /// }
    /// ```
    pub fn to_string(&self) -> String {
        unsafe {
            String::from_utf8_unchecked(self.to_bytes())
        }
    }

    /// Return a length of a string.
    pub fn len(&self) -> usize {
        self.len
    }

    /// Return a length of a buffer of a string
    pub fn bytes_len(&self) -> usize {
        self.inner.len()
    }
}

impl<'amx> AmxCell<'amx> for AmxString<'amx> {
    fn from_raw(amx: &'amx Amx, cell: i32) -> AmxResult<AmxString<'amx>> {
        let buffer = UnsizedBuffer::from_raw(amx, cell)?;
        let ptr = buffer.as_ptr();
        let packed = unsafe { ptr.read() > MAX_UNPACKED };

        let (str_len, buf_len) = {
            if packed {
                let len = strlen(ptr as *const i8, 0);
                let buf_len = len / 4 + 1; // count of 4 length cells
                (len, buf_len)
            } else {
                let len = strlen(ptr, 0);
                (len, len + 1)
            }
        };

        Ok(AmxString {
            inner: buffer.into_sized_buffer(buf_len),
            len: str_len,
        })
    }

    fn as_cell(&self) -> i32 {
        self.inner.as_cell()
    }
}

fn strlen<T: PartialEq>(mut string: *const T, zerochar: T) -> usize {
    let mut length = 0;

    unsafe {
        while string.read() != zerochar {
            string = string.offset(1);
            length += 1;
        }
    }

    return length;
}

impl fmt::Display for AmxString<'_> {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(fmt, "{}", self.to_string())
    }
}