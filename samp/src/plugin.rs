pub mod data;

use crate::SampPlugin;
pub(crate) use data::PluginData;
use samp_sdk::raw::types::AMX_NATIVE_INFO;
use samp_sdk::cell::Cell;
use std::ptr::NonNull;

#[inline(always)]
pub fn load() {
    unsafe { PluginData::get_plugin().as_mut().on_load() }
}

#[inline(always)]
pub fn unload() {
    unsafe { PluginData::get_plugin().as_mut().on_unload() }
}

#[inline(always)]
pub fn supports() -> u32 {
    let supports = PluginData::get().supports();
    return supports.bits();
}

#[inline(always)]
pub fn amx_load(_natives: &[AMX_NATIVE_INFO]) {
    unsafe { PluginData::get_plugin().as_mut().on_amx_load() }
}

#[inline(always)]
pub fn amx_unload() {
    unsafe { PluginData::get_plugin().as_mut().on_amx_unload() }
}

#[inline(always)]
pub fn get<T: SampPlugin>() -> NonNull<T> {
    PluginData::get().plugin_cast()
}

pub fn enable_process_tick() {
    PluginData::get().enable_process_tick();
}

pub fn init<T, F>(initializer: F)
where
    F: FnOnce() -> T + 'static,
    T: SampPlugin + 'static,
{
    PluginData::init();

    let plugin_data = PluginData::get();
    let plugin = initializer();

    plugin_data.set_plugin(plugin);
}

pub fn convert_return_value<T: Cell<'static>>(retval: T) -> i32 {
    retval.as_cell()
}