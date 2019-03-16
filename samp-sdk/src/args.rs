//! Workaround to parse input of natives functions.
use crate::amx::Amx;
use crate::cell::AmxCell;

/// A wrapper of a list of arguments of a native function.
pub struct Args<'a> {
    amx: &'a Amx,
    args: *const i32,
    offset: usize,
}

impl<'a> Args<'a> {
    /// Creates a list from [`Amx`] and arguments.
    ///
    /// # Example
    /// ```
    /// use samp_sdk::args::Args;
    /// use samp_sdk::amx::Amx;
    /// use samp_sdk::cell::AmxString;
    /// # use samp_sdk::raw::types::AMX;
    ///
    /// // native: RawNative(const say_that[]);
    /// extern "C" fn raw_native(amx: *mut AMX, args: *mut i32) -> i32 {
    ///     # let amx_exports = 0;
    ///     // let amx_exports = ...;
    ///     let amx = Amx::new(amx, amx_exports);
    ///     let mut args = Args::new(&amx, args);
    ///
    ///     let say_what = match args.next::<AmxString>() {
    ///         Some(string) => string.to_string(),
    ///         None => {
    ///             println!("RawNative error: no argument");
    ///             return 0;
    ///         }
    ///     };
    ///
    ///     println!("RawNative: {}", say_what);
    ///
    ///     return 1;
    /// }
    /// ```
    ///
    /// [`Amx`]: ../amx/struct.Amx.html
    pub fn new(amx: &'a Amx, args: *const i32) -> Args<'a> {
        Args {
            amx,
            args,
            offset: 0,
        }
    }

    /// Return the next argument in the list (like an iterator).
    ///
    /// When there is no arguments left returns `None`.
    pub fn next<T: AmxCell<'a> + 'a>(&mut self) -> Option<T> {
        let result = self.get(self.offset);
        self.offset += 1;

        result
    }

    /// Get an argument by position, if there is no argument in given location, returns `None`.
    ///
    /// # Example
    /// ```
    /// use samp_sdk::args::Args;
    /// use samp_sdk::amx::Amx;
    /// use samp_sdk::cell::Ref;
    /// # use samp_sdk::raw::types::AMX;
    ///
    /// // native: NativeFn(player_id, &Float:health, &Float:armor);
    /// extern "C" fn raw_native(amx: *mut AMX, args: *mut i32) -> i32 {
    ///     # let amx_exports = 0;
    ///     // let amx_exports = ...;
    ///     let amx = Amx::new(amx, amx_exports);
    ///     let args = Args::new(&amx, args);
    ///
    ///     // change only armor
    ///     args.get::<Ref<f32>>(2)
    ///         .map(|mut armor| *armor = 255.0);
    ///
    ///     return 1;
    /// }
    /// ```
    pub fn get<T: AmxCell<'a> + 'a>(&self, offset: usize) -> Option<T> {
        if offset > self.count() {
            return None;
        }

        unsafe { T::from_raw(self.amx, self.args.add(offset + 1).read()).ok() }
    }

    /// Reset a read offset for [`next()`] method.
    ///
    /// [`next()`]: #method.next
    pub fn reset(&mut self) {
        self.offset = 0;
    }

    /// Get count of arguments in the list.
    pub fn count(&self) -> usize {
        unsafe { (self.args.read() / 4) as usize }
    }
}
