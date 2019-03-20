use crate::runtime::Runtime;
use samp_sdk::raw::types::{AMX, AMX_NATIVE_INFO};

pub fn supports() -> u32 {
    let rt = Runtime::get();
    let supports = rt.supports();

    supports.bits()
}

pub fn load(server_exports: *const usize) {
    let rt = Runtime::get();
    let plugin = Runtime::plugin();

    rt.set_server_exports(server_exports);
    plugin.on_load();
}

pub fn unload() {
    let plugin = Runtime::plugin();
    plugin.on_unload();
}

pub fn amx_load(amx: *mut AMX, natives: &[AMX_NATIVE_INFO]) {
    let rt = Runtime::get();
    let plugin = Runtime::plugin();

    let amx = rt.insert_amx(amx).unwrap();
    let _ = amx.register(natives); // don't care about errors, that function always raises errors.

    plugin.on_amx_load(amx);
}

pub fn amx_unload(amx: *mut AMX) {
    let rt = Runtime::get();
    let plugin = Runtime::plugin();

    if let Some(amx) = rt.remove_amx(amx) {
        plugin.on_amx_unload(&amx);
    }
}

#[inline]
pub fn process_tick() {
    let plugin = Runtime::plugin();
    plugin.process_tick();
}
