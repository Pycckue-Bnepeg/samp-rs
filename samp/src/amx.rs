//! Core Amx types with additional functions.
pub use samp_sdk::amx::*;
use samp_sdk::raw::types::AMX;

use crate::runtime::Runtime;

/// Get a reference to an `Amx` by given `AmxIdent`.
///
/// # Example
/// ```
/// use samp::prelude::*;
/// use samp::exec_public;
/// # use samp::native;
/// # use samp::amx::AmxIdent;
///
/// # struct Plugin {
/// #       subscribtions: std::collections::HashMap<String, Vec<AmxIdent>>,
/// # }
/// #
/// # impl SampPlugin for Plugin {}
/// #
/// # impl Plugin {
///
/// #[native(name = "SubscribeToEvent")]
/// fn subscribe(&mut self, amx: &Amx, event_name: AmxString) -> AmxResult<bool> {
///     let event_name = event_name.to_string();
///     let subs = self.subscribtions.entry(event_name).or_insert(vec![]);
///     subs.push(amx.ident());
///
///     Ok(true)
/// }
///
/// fn publish(&self, event_name: &str) {
///     if let Some(subs) = self.subscribtions.get(event_name) {
///         for ident in subs {
///             if let Some(amx) = samp::amx::get(*ident) {
///                 let _ = exec_public!(amx, event_name);
///             }
///         }
///     }
/// }
///
/// # }
/// ```
#[inline]
pub fn get<'a>(ident: AmxIdent) -> Option<&'a Amx> {
    let rt = Runtime::get();
    rt.amx_list().get(&ident)
}

/// An unique identifier of an `Amx` instance.
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

/// Extended functional of an `Amx`.
pub trait AmxExt {
    /// Get an identifier of an `Amx`.
    ///
    /// # Example
    /// ```
    /// use samp::prelude::*;
    /// # use samp::native;
    /// # struct Plugin;
    /// #
    /// # impl SampPlugin for Plugin {}
    /// #
    /// # impl Plugin {
    ///
    /// #[native(name = "A")]
    /// fn native_a(&mut self, amx: &Amx) -> AmxResult<bool> {
    ///     let ident = amx.ident();
    ///     // now you can use ident to get this Amx later by samp::amx::get
    ///     Ok(true)
    /// }
    ///
    /// # }
    /// ```
    fn ident(&self) -> AmxIdent;
}

impl AmxExt for Amx {
    #[inline]
    fn ident(&self) -> AmxIdent {
        self.amx().as_ptr().into()
    }
}
