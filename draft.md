```Rust
use samp_sdk::{samp_plugin, native, log};
use samp_sdk::amx::Amx;
use samp_sdk::types::{AmxError, AmxResult, Cell};

#[samp_plugin]
struct PluginExample {
    amx_list: Vec<Amx>,
}

impl PluginExample {
    pub fn load(&self) -> bool {
        log!("Example plugin is ready!");
        return true;
    }
    
    pub fn amx_load(&mut self, amx: Amx) -> AmxResult<()> {
        if amx.get_pubvar("ENABLE_MY_PLUGIN_PLEASE").is_none() {
            Err(AmxError::NotFound)
        } else {
            Ok(())
        }
    }
}
```