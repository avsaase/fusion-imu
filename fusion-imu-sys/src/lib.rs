//! # fusion-sys
//!
//! This library provides generated Rust bindings to the Fusion AHRS C library. You
//! probably want to use the high-level wrappers in
//! [`fusion-imu`](https://crates.io/crates/fusion-imu) instead.

#![no_std]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(improper_ctypes)]
#![allow(rustdoc::bare_urls)]
#![allow(clippy::approx_constant)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
