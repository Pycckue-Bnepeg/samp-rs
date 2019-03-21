use std::sync::atomic::{AtomicBool, Ordering::SeqCst};
use std::sync::Mutex;

use crossbeam_utils::Backoff;

pub mod amx;
pub mod error;

static mut THREAD: *mut SampThread = std::ptr::null_mut();

pub(crate) type Guard<'a> = std::sync::MutexGuard<'a, ()>;

pub(crate) struct SampThread {
    ready: AtomicBool,
    mutex: Mutex<()>, // why mutex? to deal with panics of other threads
}

impl SampThread {
    #[inline]
    pub fn get() -> &'static mut SampThread {
        unsafe { &mut *THREAD }
    }

    #[inline]
    pub fn make_ready(&self) {
        self.ready.store(true, SeqCst);
    }

    pub fn wait_readiness(&self) {
        let backoff = Backoff::new();

        while !self.ready.load(SeqCst) {
            backoff.snooze();
        }
    }

    #[inline]
    pub fn lock(&self) -> Result<Guard, std::sync::PoisonError<Guard>> {
        self.mutex.lock()
    }

    #[inline]
    pub fn try_lock(&self) -> std::sync::TryLockResult<Guard> {
        self.mutex.try_lock()
    }

    pub fn wait_other_threads(&mut self) {
        let guard = self
            .mutex
            .lock()
            .unwrap_or_else(|poison| poison.into_inner());
        self.ready.store(false, SeqCst);
        drop(guard);

        if self.mutex.is_poisoned() {
            // recreate a mutex if some of other threads panic
            self.mutex = Mutex::default();
        }
    }
}

pub fn initialize() {
    let lock = SampThread {
        ready: AtomicBool::new(false),
        mutex: Mutex::default(),
    };

    let lock_ptr = Box::into_raw(Box::new(lock));

    unsafe {
        THREAD = lock_ptr;
    }
}

pub fn process() {
    let thread = SampThread::get();

    thread.make_ready();
    thread.wait_other_threads();
}
