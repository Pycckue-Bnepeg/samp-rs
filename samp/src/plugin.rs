use std::ptr::NonNull;

use samp_sdk::amx::Amx;
use samp_sdk::cell::AmxCell;

use crate::runtime::Runtime;

#[doc(hidden)]
pub fn initialize<F, T>(constructor: F)
where
    F: FnOnce() -> T + 'static,
    T: SampPlugin + 'static,
{
    let rt = Runtime::initialize();
    let plugin = constructor();

    rt.set_plugin(plugin);
}

/// Enables process_tick function for a plugin.
///
/// # Example
/// ```rust,compile_fail
/// use samp::initialize_plugin;
/// use samp::prelude::*;
///
/// struct MyPlugin;
///
/// impl SampPlugin for MyPlugin {}
///
/// initialize_plugin!({
///     samp::plugin::enable_process_tick();
///     return MyPlugin;
/// });
/// ```
pub fn enable_process_tick() {
    let runtime = Runtime::get();
    runtime.enable_process_tick();
}

#[doc(hidden)]
pub fn get<T: SampPlugin + 'static>() -> NonNull<T> {
    Runtime::plugin_cast()
}

/// An interface that should be implemented by any plugin.
///
/// All methods are optional
pub trait SampPlugin {
    fn on_load(&mut self) {}
    fn on_unload(&mut self) {}

    fn on_amx_load(&mut self, amx: &Amx) {
        let _ = amx;
    }

    fn on_amx_unload(&mut self, amx: &Amx) {
        let _ = amx;
    }

    fn process_tick(&mut self) {}
}

#[doc(hidden)]
pub fn convert_return_value<T: AmxCell<'static>>(value: T) -> i32 {
    value.as_cell()
}
