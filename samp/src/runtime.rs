use samp_sdk::consts::{ServerData, Supports};
use samp_sdk::raw::types::AMX;

use std::collections::HashMap;
use std::ptr::NonNull;

use crate::amx::{Amx, AmxIdent};
use crate::plugin::SampPlugin;

static mut RUNTIME: *mut Runtime = std::ptr::null_mut();

pub struct Runtime {
    plugin: Option<NonNull<dyn SampPlugin + 'static>>,
    process_tick: bool,
    server_exports: *const usize,
    amx_list: HashMap<AmxIdent, Amx>,
}

impl Runtime {
    pub fn initialize() -> &'static mut Runtime {
        let rt = Runtime {
            plugin: None,
            process_tick: false,
            server_exports: std::ptr::null(),
            amx_list: HashMap::default(),
        };

        let boxed = Box::new(rt);

        unsafe {
            RUNTIME = Box::into_raw(boxed);
        }

        Runtime::get()
    }

    #[inline]
    pub fn amx_exports(&self) -> usize {
        unsafe {
            self.server_exports
                .offset(ServerData::AmxExports.into())
                .read()
        }
    }

    pub fn insert_amx(&mut self, amx: *mut AMX) -> Option<&Amx> {
        let ident = AmxIdent::from(amx);
        let amx = Amx::new(amx, self.amx_exports());

        self.amx_list.insert(ident, amx);
        self.amx_list.get(&ident)
    }

    pub fn remove_amx(&mut self, amx: *mut AMX) -> Option<Amx> {
        let ident = AmxIdent::from(amx);
        self.amx_list.remove(&ident)
    }

    pub fn supports(&self) -> Supports {
        let mut supports = Supports::VERSION | Supports::AMX_NATIVES;

        if self.process_tick {
            supports.toggle(Supports::PROCESS_TICK);
        }

        supports
    }

    #[inline]
    pub fn amx_list(&self) -> &HashMap<AmxIdent, Amx> {
        &self.amx_list
    }

    pub fn set_plugin<T>(&mut self, plugin: T)
    where
        T: SampPlugin + 'static,
    {
        let boxed = Box::new(plugin);
        self.plugin = NonNull::new(Box::into_raw(boxed));
    }

    pub fn set_server_exports(&mut self, exports: *const usize) {
        self.server_exports = exports;
    }

    pub fn enable_process_tick(&mut self) {
        self.process_tick = true;
    }

    #[inline]
    pub fn get() -> &'static mut Runtime {
        unsafe { &mut *RUNTIME }
    }

    #[inline]
    pub fn plugin() -> &'static mut dyn SampPlugin {
        unsafe { (*RUNTIME).plugin.as_mut().unwrap().as_mut() }
    }

    #[inline]
    pub fn plugin_cast<T: SampPlugin>() -> NonNull<T> {
        let rt = Runtime::get();
        rt.plugin.as_ref().unwrap().cast()
    }
}
