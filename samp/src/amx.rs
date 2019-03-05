pub use samp_sdk::amx::*;
use samp_sdk::raw::types::AMX;

use crate::runtime::Runtime;

#[inline]
pub fn get<'a>(ident: AmxIdent) -> Option<&'a Amx> {
    let rt = Runtime::get();
    rt.amx_list().get(&ident)
}

#[derive(Debug, Clone, Copy, PartialEq, Hash, Eq)]
pub struct AmxIdent {
    ident: usize,
}

impl From<*mut AMX> for AmxIdent {
    fn from(ptr: *mut AMX) -> AmxIdent {
        AmxIdent {
            ident: ptr as usize,
        }
    }
}

pub trait AmxExt {
    fn ident(&self) -> AmxIdent;
}

impl AmxExt for Amx {
    #[inline]
    fn ident(&self) -> AmxIdent {
        self.amx().as_ptr().into()
    }
}
