/*! 

# SA:MP SDK
This crate is a Rust language wrapper for SA:MP SDK.

*/
#![feature(rust_2018_preview, use_extern_macros)]

#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(dead_code)]
#![allow(non_snake_case)]

#[macro_use] pub mod macros;
pub mod args;
pub mod consts;
pub mod data;
pub mod types;
pub mod amx;

pub use lazy_static::{lazy_static, __lazy_static_internal, __lazy_static_create};