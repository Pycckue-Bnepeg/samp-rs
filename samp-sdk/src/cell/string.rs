//! String interperation inside an AMX.
use std::fmt;

use super::{AmxCell, Buffer, UnsizedBuffer};
use crate::amx::Amx;
use crate::error::AmxResult;
#[cfg(feature = "encoding")]
use crate::encoding;

const MAX_UNPACKED: i32 = 0x00FF_FFFF;

// A wrapper around an AMX string.
pub struct AmxString<'amx> {
    inner: Buffer<'amx>,
    // real length of the string
    len: usize,
}

impl<'amx> AmxString<'amx> {
    /// Create a new AmxString from an allocated buffer and fill it with a string
    pub unsafe fn new(mut buffer: Buffer<'amx>, bytes: &[u8]) -> AmxString<'amx> {
        // let _ = put_in_buffer(&mut buffer, string); // here can't be an error.
        for (idx, byte) in bytes.iter().enumerate() {
            buffer[idx] = i32::from(*byte);
        }

        buffer[bytes.len()] = 0;

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
                std::ptr::copy(
                    self.inner.as_ptr() as *const u8,
                    vec.as_mut_ptr(),
                    vec.len(),
                );
            }
        } else {
            for item in self.inner.iter().take(self.len) {
                vec.push(*item as u8);
            }
        }

        vec
    }

    /// Convert an AMX string to a `String`.
    /// Only ASCII chars by default. Pass `cp1251` to crate features to enable Windows 1251 encoding (TODO).
    ///
    /// # Example
    /// ```
    /// use samp_sdk::cell::AmxString;
    /// # use samp_sdk::amx::Amx;
    /// # use samp_sdk::error::AmxResult;
    /// #
    /// # fn current_date() -> String {
    /// #       String::from("Today")
    /// # }
    ///
    /// # struct Plugin {
    /// #     logger_enabled: bool,
    /// # }
    /// #
    /// # impl Plugin {
    ///
    /// fn log_error(&self, amx: &Amx, text: AmxString) -> AmxResult<bool> {
    ///     if !self.logger_enabled {
    ///         return Ok(false);
    ///     }
    ///
    ///     let string = text.to_string();
    ///     println!("[{}] PluginName error: {}", current_date(), string);
    ///
    ///     Ok(true)
    /// }
    /// # }
    /// ```
    pub fn to_string(&self) -> String {
        #[cfg(feature = "encoding")]
        return encoding::get().decode(&self.to_bytes()).0.into_owned();

        #[cfg(not(feature = "encoding"))]
        return unsafe { String::from_utf8_unchecked(self.to_bytes()) };
    }

    /// Return a length of a string.
    pub fn len(&self) -> usize {
        self.len
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
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

    length
}

impl fmt::Display for AmxString<'_> {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(fmt, "{}", self.to_string())
    }
}

/// Fill a buffer with given string.
///
/// # Example
/// ```rust,no_run
/// use samp_sdk::cell::Buffer;
/// use samp_sdk::cell::string;
/// # use samp_sdk::error::AmxResult;
/// # use samp_sdk::amx::Amx;
///
/// # fn main() -> AmxResult<()> {
/// # let amx = Amx::new(std::ptr::null_mut(), 0);
/// // let mut buffer = ...;
/// // let amx = ...;
/// let allocator = amx.allocator();
/// let mut buffer = allocator.allot_buffer(25)?; // let's think that we got a mutable buffer from a native function input.
/// let string = "Hello, world!".to_string();
/// string::put_in_buffer(&mut buffer, &string)?; // store string in the AMX heap.
///
///
/// #   Ok(())
/// # }
/// ```
/// # Errors
/// Return `AmxError::General` when length of string bytes is more than size of the buffer.
pub fn put_in_buffer(buffer: &mut Buffer, string: &str) -> AmxResult<()> {
    #[cfg(feature = "encoding")]
    let bytes = encoding::get().encode(string).0;
    
    #[cfg(not(feature = "encoding"))]
    let bytes = std::borrow::Cow::from(string.as_bytes());

    let bytes = bytes.as_ref();

    if bytes.len() >= buffer.len() {
        return Err(crate::error::AmxError::General);
    }

    for (idx, byte) in bytes.iter().enumerate() {
        buffer[idx] = i32::from(*byte);
    }

    buffer[bytes.len()] = 0;

    Ok(())
}
