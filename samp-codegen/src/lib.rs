#![recursion_limit = "128"]

extern crate proc_macro;
use proc_macro::TokenStream;

mod native;
mod plugin;

pub(crate) const NATIVE_PREFIX: &str = "__samp_native_";
pub(crate) const REG_PREFIX: &str = "__samp_reg_";

/// Generate C function that parses passed argument and calls current function.
#[proc_macro_attribute]
pub fn native(args: TokenStream, input: TokenStream) -> TokenStream {
    native::create_native(args, input)
}

/// Generates common plugin C interface.
#[proc_macro]
pub fn initialize_plugin(input: TokenStream) -> TokenStream {
    plugin::create_plugin(input)
}
