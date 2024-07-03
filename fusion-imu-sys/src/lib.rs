//! # fusion-sys
//!
//! This library provides generated Rust bindings to the Fusion C library. You
//! probably want to use the higher-level wrappers in `fusion` instead.

#![no_std]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(improper_ctypes)]
#![allow(rustdoc::bare_urls)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
