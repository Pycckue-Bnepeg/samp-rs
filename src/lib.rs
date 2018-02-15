//! # SA:MP SDK
//! `samp_sdk` is a Rust lang bindings for original C SA:MP SDK

#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(dead_code)]
#![allow(non_snake_case)]

#[macro_use] extern crate lazy_static;
extern crate libc;

#[macro_use] pub mod macros;
pub mod consts;
pub mod data;
pub mod types;
pub mod amx;