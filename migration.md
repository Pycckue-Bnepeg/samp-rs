# what's new
* replace a `new_plugin!` decl macro with a `initialize_plugin!` proc macro.
* replace a `define_native!` decl macro with a `#[native]` attribute.
* new `AmxCell` trait that allows convert Rust types to AMX types.

# migration guide
* change `samp_sdk = "*"` in `Cargo.toml` to `samp = "0.1.0"`.
* export the prelude module `use samp::prelude::*`.
* remove `new_plugin!` call to `initialize_plugin!`
* delete all `define_native!` calls.

### your old code
```rust
use samp_sdk::new_plugin;
use samp_sdk::...;

// native definitions
define_native!(my_native, string: String);
define_native!(raw_native as raw);

pub struct Plugin {
    <.. code ..>
}

impl Plugin {
    fn load(&self) {
        let natives = natives! {
            "MyNative" => my_native,
            "RawNative" => raw_native
        };

        amx.register(&natives);

        <.. code ..>
    }

    fn unload(&self) { 
        <.. code ..> 
    }

    fn my_native(&self, amx: &AMX, string: String) -> AmxResult<Cell> {
        <.. code ..>
    }

    fn raw_native(&self, amx: &AMX, args: *mut Cell) -> AmxResult<Cell> {
        <.. code ..>
    }
}

impl Default for Plugin {
    fn default() -> Plugin {
        Plugin {
            <.. code ..>
        }
    }
}

new_plugin!(Plugin);
```
### your new code
```rust
use samp::prelude::*;
use samp::args::Args;

struct Plugin {
    <.. code ..>
}

impl Plugin {
    #[native(name = "MyNative")]
    fn my_native(&mut self, amx: &Amx, string: AmxString) -> AmxResult<bool> {
        let string = string.to_string(); // convert AmxString to rust String
        <.. code ..>
    }

    #[native(name = "RawNative", raw)]
    fn raw_native(&mut self, amx: &Amx, args: Args) -> AmxResult<f32> {
        <.. code ..>
    }
}

impl SampPlugin for Plugin {
    fn on_load(&mut self) {
        // no more calls to Amx::register.
        <.. code ..> 
    }

    fn on_unload(&mut self) {
        <.. code ..>
    }
}

initialize_plugin!(
    natives: [
        Plugin::my_native,
        Plugin::raw_native,
    ],
    {
        let plugin = Plugin {
            <.. code ..>
        };

        return plugin;
    }
);
```