use std::sync::PoisonError;
use std::sync::{Mutex, MutexGuard};

pub mod amx;
pub mod error;

static mut PARKER: *mut GlobalParker = std::ptr::null_mut();

pub(crate) type Guard = MutexGuard<'static, ()>;

pub(crate) struct GlobalParker {
    mutex: Mutex<()>,
    guard: Option<Guard>,
}

impl GlobalParker {
    /// Park SAMP main thread to allow other threads access to AMX instances
    pub fn park() {
        let _ = unsafe { (*PARKER).guard.take() };
    }

    /// Unpark SAMP main thread
    pub fn unpark() {
        unsafe {
            let guard = (*PARKER).mutex.lock().unwrap();
            (*PARKER).guard = Some(guard);
        }
    }

    #[inline]
    pub fn lock() -> Result<Guard, PoisonError<Guard>> {
        unsafe { (*PARKER).mutex.lock() }
    }

    #[inline]
    pub fn try_lock() -> std::sync::TryLockResult<Guard> {
        unsafe { (*PARKER).mutex.try_lock() }
    }
}

pub fn initialize() {
    let lock = GlobalParker {
        mutex: Mutex::default(),
        guard: None,
    };

    let lock_ptr = Box::into_raw(Box::new(lock));

    unsafe {
        PARKER = lock_ptr;
    }
}

pub fn process() {
    // kind of shit but that will work
    GlobalParker::park();
    std::thread::sleep(std::time::Duration::from_nanos(1));
    GlobalParker::unpark();
}
