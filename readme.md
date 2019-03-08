[![Docs](https://docs.rs/samp/badge.svg)](https://docs.rs/samp)
[![Crates](https://img.shields.io/crates/v/samp.svg)](https://crates.io/crates/samp)
# samp-rs
samp-rs is a tool to develop plugins for [samp](http://sa-mp.com) servers written in rust.

# project structure
* `samp` is a glue between crates described below (that's what you need).
* `samp-codegen` generates raw `extern "C"` functions and does whole nasty job.
* `samp-sdk` contains all types to work with amx.

# usage
* [install](https://rustup.rs) rust compiler (supports only `i686` os versions because of samp server arch).
* add in your `Cargo.toml` this:
```toml
[lib]
crate-type = ["cdylib"] # or dylib

[dependencies]
samp = "0.2.5"
```
* write your first plugin

# examples
* simple memcache plugin in `plugin-example` folder.
* your `lib.rs` file
```rust
use samp::prelude::*; // export most useful types
use samp::{native, initialize_plugin}; // codegen macros

struct Plugin;

impl SampPlugin for Plugin {
    // this function executed when samp server loads your plugin
    fn on_load(&mut self) {
        println!("Plugin is loaded.");
    }
}

impl Plugin {
    #[native(name = "TestNative")]
    fn my_native(&mut self, _amx: &Amx, text: AmxString) -> AmxResult<bool> {
        let text = text.to_string(); // convert amx string into rust string
        println!("rust plugin: {}", text);

        Ok(true)
    }
}

initialize_plugin!(
    natives: [Plugin::my_native],
    {
        let plugin = Plugin; // create a plugin object
        return plugin; // return the plugin into runtime
    }
)
```