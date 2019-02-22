use crate::plugin::PluginData;
pub use samp_sdk::amx::Amx;

pub fn current<'a>() -> Option<&'a Amx> {
    PluginData::current_amx()
}