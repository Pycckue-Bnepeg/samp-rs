use tokio::prelude::*;
use tokio::runtime::Runtime;
use tokio::sync::oneshot::{channel, Sender};
use tokio::timer::{Delay, Interval};

use samp::exec_public;
use samp::AsyncAmx;

use std::collections::HashMap;
use std::time::{Duration, Instant};

use log::{error, trace};

pub struct Timers {
    ident: u32,
    list: HashMap<u32, Sender<()>>,
}

impl Timers {
    pub fn new() -> Timers {
        Timers {
            ident: 0,
            list: HashMap::new(),
        }
    }

    pub fn new_timeout(
        &mut self, rt: &mut Runtime, amx: AsyncAmx, callback: String, timeout: Duration,
    ) -> u32 {
        let timer_id = self.next_id();
        let when = Instant::now() + timeout;
        let (sender, receiver) = channel::<()>();

        self.list.insert(timer_id, sender);

        let delay = Delay::new(when)
            .map(move |_| {
                call_timer_public(&amx, &callback);
            })
            .map(move |_| trace!("timeout {} has been executed", timer_id))
            .map_err(move |err| error!("timeout {} tokio error: {:?}", timer_id, err));

        let receiver = receiver
            .map(move |_| trace!("request to destroy {} timeout", timer_id))
            .map_err(move |_| trace!("timeout {} channel closed", timer_id));

        let task = delay.select(receiver).map(|_| ()).map_err(|_| ());

        rt.spawn(task);

        timer_id
    }

    pub fn new_interval(
        &mut self, rt: &mut Runtime, amx: AsyncAmx, callback: String, interval: Duration,
    ) -> u32 {
        let timer_id = self.next_id();
        let (sender, receiver) = channel::<()>();

        self.list.insert(timer_id, sender);

        let interval = Interval::new_interval(interval)
            .map_err(move |err| error!("interval {} tokio error: {:?}", timer_id, err))
            .for_each(move |_| {
                call_timer_public(&amx, &callback);
                trace!("interval {} has been executed", timer_id);
                future::ok(())
            });

        let receiver = receiver
            .map(move |_| trace!("request to destroy {} interval", timer_id))
            .map_err(move |_| trace!("interval {} channel closed", timer_id));

        let task = interval.select(receiver).map(|_| ()).map_err(|_| ());

        rt.spawn(task);

        timer_id
    }

    pub fn kill(&mut self, timer_id: u32) {
        self.list.remove(&timer_id).map(|sender| sender.send(()));
    }

    fn next_id(&mut self) -> u32 {
        let id = self.ident;
        self.ident += 1;

        id
    }
}

fn call_timer_public(amx: &AsyncAmx, callback: &str) {
    let amx = match amx.lock() {
        Ok(amx) => amx,
        Err(_) => return,
    };

    let _ = exec_public!(amx, callback);
}
