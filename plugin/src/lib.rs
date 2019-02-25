use samp::{initialize_plugin, SampPlugin};
use samp::cell::{Cell};

mod natives;

#[derive(Debug, Clone, Copy)]
enum NativeResult {
    AllGood,
    NoMana,
    NoHealth,
}

impl From<NativeResult> for i32 {
    fn from(value: NativeResult) -> i32 {
        match value {
            NativeResult::AllGood => 0,
            NativeResult::NoMana => 1,
            NativeResult::NoHealth => 2,
        }
    }
}

impl Cell<'_> for NativeResult {
    fn as_cell(&self) -> i32 {
        (*self).into()
    }
}

struct Plugin {
    admin_list: Vec<&'static str>,
}

impl Plugin {
    fn new(admin_list: Vec<&'static str>) -> Plugin {
        Plugin {
            admin_list,
        }
    }
}

impl SampPlugin for Plugin {}

initialize_plugin!(
    natives: [
        Plugin::get_player_ip, 
        Plugin::is_player_admin,
    ],
    {
        samp::enable_process_tick();

        let admin_list = vec!["A", "B"];
        let plugin = Plugin::new(admin_list);
        return plugin;
    }
);