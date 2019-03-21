use std::fmt;
use std::sync::{PoisonError, TryLockError};

use crate::Guard;

#[derive(Debug)]
pub enum AmxLockError<'a> {
    AmxGone,
    PoisonError(PoisonError<Guard<'a>>),
    TryLockError(TryLockError<Guard<'a>>),
}

impl<'a> From<PoisonError<Guard<'a>>> for AmxLockError<'a> {
    fn from(err: PoisonError<Guard>) -> AmxLockError {
        AmxLockError::PoisonError(err)
    }
}

impl<'a> From<TryLockError<Guard<'a>>> for AmxLockError<'a> {
    fn from(err: TryLockError<Guard>) -> AmxLockError {
        AmxLockError::TryLockError(err)
    }
}

impl<'a> fmt::Display for AmxLockError<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            AmxLockError::AmxGone => write!(f, "AMX instance is unloaded"),
            AmxLockError::PoisonError(err) => write!(f, "{}", err),
            AmxLockError::TryLockError(err) => write!(f, "{}", err),
        }
    }
}
