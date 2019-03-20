use std::sync::{PoisonError, TryLockError};

use crate::Guard;

pub enum AmxLockError {
    AmxGone,
    PoisonError(PoisonError<Guard>),
    TryLockError(TryLockError<Guard>),
}

impl From<PoisonError<Guard>> for AmxLockError {
    fn from(err: PoisonError<Guard>) -> AmxLockError {
        AmxLockError::PoisonError(err)
    }
}

impl From<TryLockError<Guard>> for AmxLockError {
    fn from(err: TryLockError<Guard>) -> AmxLockError {
        AmxLockError::TryLockError(err)
    }
}
