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

pub fn enable_process_tick() {
    let runtime = Runtime::get();
    runtime.enable_process_tick();
}

#[doc(hidden)]
pub fn get<T: SampPlugin + 'static>() -> NonNull<T> {
    Runtime::plugin_cast()
}

pub trait SampPlugin {
    fn on_load(&mut self) {}
    fn on_unload(&mut self) {}
    fn on_amx_load(&mut self, amx: &Amx) {}
    fn on_amx_unload(&mut self, amx: &Amx) {}
    fn process_tick(&mut self) {}
}

pub fn convert_return_value<T: AmxCell<'static>>(value: T) -> i32 {
    value.as_cell()
}
