use samp_runtime::Runtime;
use samp_sdk::amx::{Amx, AmxIdent};

use std::ops::Deref;

use crate::error::AmxLockError;
use crate::{GlobalParker, Guard};

#[derive(Debug, Clone)]
pub struct AsyncAmx {
    ident: AmxIdent,
}

impl AsyncAmx {
    pub fn lock<'a>(&'a self) -> Result<AmxGuard<'a>, AmxLockError> {
        let guard = GlobalParker::lock()?;
        let rt = Runtime::get();
        let amx = rt
            .amx_list()
            .get(&self.ident)
            .ok_or(AmxLockError::AmxGone)?;

        Ok(AmxGuard { _guard: guard, amx })
    }

    pub fn try_lock<'a>(&'a self) -> Result<AmxGuard<'a>, AmxLockError> {
        let guard = GlobalParker::try_lock()?;
        let rt = Runtime::get();
        let amx = rt
            .amx_list()
            .get(&self.ident)
            .ok_or(AmxLockError::AmxGone)?;

        Ok(AmxGuard { _guard: guard, amx })
    }
}

pub struct AmxGuard<'a> {
    _guard: Guard,
    amx: &'a Amx,
}

impl<'a> Deref for AmxGuard<'a> {
    type Target = Amx;

    fn deref(&self) -> &Amx {
        self.amx
    }
}

pub trait AmxExt {
    fn to_async(&self) -> AsyncAmx;
}

impl AmxExt for Amx {
    fn to_async(&self) -> AsyncAmx {
        AsyncAmx {
            ident: self.ident(),
        }
    }
}

unsafe impl Sync for AsyncAmx {}
unsafe impl Send for AsyncAmx {}
