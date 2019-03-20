//! String encoding.
use encoding_rs::Encoding;
pub use encoding_rs::{WINDOWS_1251, WINDOWS_1252};

static mut DEFAULT_ENCODING: &Encoding = WINDOWS_1252;

pub fn set_default_encoding(encoding: &'static Encoding) {
    unsafe {
        DEFAULT_ENCODING = encoding;
    }
}

pub(crate) fn get() -> &'static Encoding {
    unsafe { DEFAULT_ENCODING }
}
