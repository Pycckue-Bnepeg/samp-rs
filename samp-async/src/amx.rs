use samp_runtime::Runtime;
use samp_sdk::amx::{Amx, AmxIdent};

use std::ops::Deref;

use crate::error::AmxLockError;
use crate::{Guard, SampThread};

/// An asynchronous version of AMX instance that is [`Send`](https://doc.rust-lang.org/std/marker/trait.Send.html) + [`Sync`](https://doc.rust-lang.org/std/marker/trait.Sync.html)
#[derive(Debug, Clone)]
pub struct AsyncAmx {
    ident: AmxIdent,
}

impl AsyncAmx {
    pub fn lock<'a>(&'a self) -> Result<AmxGuard<'a>, AmxLockError> {
        let thread = SampThread::get();
        let guard = thread.lock()?;

        thread.wait_readiness();

        let rt = Runtime::get();
        let amx = rt
            .amx_list()
            .get(&self.ident)
            .ok_or(AmxLockError::AmxGone)?;

        Ok(AmxGuard { _guard: guard, amx })
    }

    pub fn try_lock<'a>(&'a self) -> Result<AmxGuard<'a>, AmxLockError> {
        let thread = SampThread::get();
        let guard = thread.try_lock()?;

        thread.wait_readiness();

        let rt = Runtime::get();
        let amx = rt
            .amx_list()
            .get(&self.ident)
            .ok_or(AmxLockError::AmxGone)?;

        Ok(AmxGuard { _guard: guard, amx })
    }
}

pub struct AmxGuard<'a> {
    _guard: Guard<'a>,
    amx: &'a Amx,
}

impl<'a> Deref for AmxGuard<'a> {
    type Target = Amx;

    fn deref(&self) -> &Amx {
        self.amx
    }
}

pub trait AmxAsyncExt {
    fn to_async(&self) -> AsyncAmx;
}

impl AmxAsyncExt for Amx {
    fn to_async(&self) -> AsyncAmx {
        AsyncAmx {
            ident: self.ident(),
        }
    }
}

unsafe impl Sync for AsyncAmx {}
unsafe impl Send for AsyncAmx {}
