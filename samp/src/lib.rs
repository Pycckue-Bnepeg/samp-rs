pub mod amx;
#[doc(hidden)]
pub mod interlayer;
#[doc(hidden)]
pub mod macros;
pub mod plugin;
pub(crate) mod runtime;

pub use samp_codegen::{initialize_plugin, native};
pub use samp_sdk::{args, cell, consts, error, exports, raw};

pub mod prelude {
    pub use crate::amx::{Amx, AmxExt};
    pub use crate::cell::{AmxCell, AmxString, Buffer, Ref, UnsizedBuffer};
    pub use crate::error::AmxResult;
    pub use crate::plugin::SampPlugin;
}
