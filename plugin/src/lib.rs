use std::collections::HashMap;

use samp::{initialize_plugin, plugin::SampPlugin};

mod abilities;
mod error;
mod natives;
mod players;

use abilities::Ability;
use players::Player;

struct MagePlugin {
    abilities: HashMap<usize, Vec<Ability>>,
    mages: HashMap<usize, Player>,
}

impl MagePlugin {
    fn new() -> MagePlugin {
        MagePlugin {
            abilities: HashMap::new(),
            mages: HashMap::new(),
        }
    }
}

impl SampPlugin for MagePlugin {}

initialize_plugin!(
    natives: [
        MagePlugin::create_ability,
        MagePlugin::cast_ability,
    ],
    {
        samp::plugin::enable_process_tick();
        return MagePlugin::new();
    }
);
