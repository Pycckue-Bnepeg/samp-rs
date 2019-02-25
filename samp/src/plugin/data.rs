use std::ptr::NonNull;

use crate::SampPlugin;
use samp_sdk::consts::Supports;

static mut PLUGIN_DATA: *mut PluginData = 0 as _;

pub(crate) struct PluginData {
    plugin: Option<NonNull<dyn SampPlugin>>,
    proccess_tick: bool,
}

impl PluginData {
    pub fn init() {
        let boxed = Box::new(PluginData {
            plugin: None,
            proccess_tick: false,
        });

        unsafe {
            PLUGIN_DATA = Box::into_raw(boxed);
        }
    }

    pub fn set_plugin<T: SampPlugin + 'static>(&mut self, plugin: T) {
        let boxed = Box::new(plugin);
        self.plugin = NonNull::new(Box::into_raw(boxed));
    }

    pub fn enable_process_tick(&mut self) {
        self.proccess_tick = true;
    }

    #[inline(always)]
    pub fn plugin_cast<T: SampPlugin>(&self) -> NonNull<T> {
        self.plugin.unwrap().cast()
    }

    pub fn supports(&self) -> Supports {
        let mut supports = Supports::VERSION | Supports::AMX_NATIVES;
        
        if self.proccess_tick {
            supports.toggle(Supports::PROCESS_TICK);
        }

        return supports;
    }

    #[inline(always)]
    pub fn get() -> &'static mut PluginData {
        unsafe {
            &mut *PLUGIN_DATA
        }
    }

    #[inline(always)]
    pub fn get_plugin() -> NonNull<dyn SampPlugin> {
        Self::get().plugin.unwrap()
    }
}