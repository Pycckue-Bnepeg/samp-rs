pub use samp_codegen::initialize_plugin;
pub use samp_codegen::native;

pub mod plugin;

pub trait SampPlugin {
    fn on_load(&mut self) {}
    fn on_unload(&mut self) {}
    fn on_amx_load(&mut self) {}
    fn on_amx_unload(&mut self) {}
    fn process_tick(&mut self) {}
}

pub use plugin::enable_process_tick;

pub use samp_sdk::{amx, args, cell, consts, error, raw};
