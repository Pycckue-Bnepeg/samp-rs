use samp::prelude::*;
use samp::AmxAsyncExt;
use samp::{initialize_plugin, native};

use log::info;

use tokio::prelude::*;
use tokio::runtime::Runtime;

use std::time::Duration;

mod timers;

use crate::timers::Timers;

struct TimerPlugin {
    rt: Runtime,
    timers: Timers,
}

impl TimerPlugin {
    #[native(name = "Timeout")]
    pub fn timeout(&mut self, amx: &Amx, callback: AmxString, timeout: u32) -> AmxResult<u32> {
        let amx = amx.to_async();
        let callback = callback.to_string();
        let timeout = Duration::from_millis(timeout as u64);

        let timer_id = self
            .timers
            .new_timeout(&mut self.rt, amx, callback, timeout);

        Ok(timer_id)
    }

    #[native(name = "KillTimeout")]
    pub fn kill_timeout(&mut self, _: &Amx, timer_id: u32) -> AmxResult<bool> {
        self.timers.kill(timer_id);
        Ok(true)
    }

    #[native(name = "Interval")]
    pub fn interval(&mut self, amx: &Amx, callback: AmxString, interval: u32) -> AmxResult<u32> {
        let amx = amx.to_async();
        let callback = callback.to_string();
        let timeout = Duration::from_millis(interval as u64);

        let interval_id = self
            .timers
            .new_interval(&mut self.rt, amx, callback, timeout);

        Ok(interval_id)
    }
}

impl SampPlugin for TimerPlugin {
    fn on_load(&mut self) {
        info!("SAMP timer plugin is successful loaded.");
    }

    fn on_unload(self: Box<TimerPlugin>) {
        let _ = self.rt.shutdown_now().wait();
    }
}

initialize_plugin!(
    natives: [
        TimerPlugin::timeout,
        TimerPlugin::interval,
        TimerPlugin::kill_timeout,
    ],
    {
        let rt = Runtime::new().unwrap();

        let plugin = TimerPlugin {
            rt,
            timers: Timers::new(),
        };

        return plugin;
    }
);
