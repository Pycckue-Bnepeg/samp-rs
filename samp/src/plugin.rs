//! Contains a plugin interface.
use std::ptr::NonNull;

use samp_runtime::Runtime;
pub use samp_runtime::SampPlugin;
use samp_sdk::cell::AmxCell;

#[doc(hidden)]
pub fn initialize<F, T>(constructor: F)
where
    F: FnOnce() -> T + 'static,
    T: SampPlugin + 'static,
{
    let rt = Runtime::initialize();
    let plugin = constructor();

    rt.set_plugin(plugin);

    if rt.is_default_logger_enabled() {
        let logger = logger();
        let _ = logger.apply();
    }

    samp_async::initialize();
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

/// Get a fern [`Dispatch`] and disable auto installing logger.
///
/// # Example
/// ```rust,compile_fail
/// use samp::initialize_plugin;
/// use samp::prelude::*;
///
/// use std::fs::OpenOptions;
///
/// struct MyPlugin;
///
/// impl SampPlugin for MyPlugin {}
///
/// initialize_plugin!({
///     samp::plugin::enable_process_tick();
///     
///     // get a default samp logger (uses samp logprintf).
///     let samp_logger = samp::plugin::logger()
///         .level(log::LevelFilter::Warn); // logging only warn and error messages
///
///     let log_file = fern::log_file("myplugin.log").expect("Something wrong!");
///
///     // log trace and debug messages in an another file
///     let trace_level = fern::Dispatch::new()
///         .level(log::LevelFilter::Trace) // write ALL types of logs
///         .chain(log_file);
///
///     let _ = fern::Dispatch::new()
///         .format(|callback, message, record| {
///             // all messages will be formated like
///             // [MyPlugin][ERROR]: something (error!("something"))
///             // [MyPlugin][INFO]: some info (info!("some info"))
///             callback.finish(format_args!("[MyPlugin][{}]: {}", record.level(), message))
///         })
///         .chain(samp_logger)
///         .chain(trace_level)
///         .apply();
///
///     return MyPlugin;
/// });
/// ```
///
/// [`Dispatch`]: https://docs.rs/fern/0.5.7/fern/struct.Dispatch.html
pub fn logger() -> fern::Dispatch {
    let rt = Runtime::get();
    rt.disable_default_logger();

    fern::Dispatch::new().chain(fern::Output::call(|record| {
        let rt = Runtime::get();
        rt.log(record.args());
    }))
}

#[doc(hidden)]
pub fn get<T: SampPlugin + 'static>() -> NonNull<T> {
    Runtime::plugin_cast()
}

#[doc(hidden)]
pub fn convert_return_value<T: AmxCell<'static>>(value: T) -> i32 {
    value.as_cell()
}
