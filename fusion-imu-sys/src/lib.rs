#![doc = include_str!("../README.md")]
#![no_std]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(improper_ctypes)]
#![allow(rustdoc::bare_urls)]
#![allow(clippy::approx_constant)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bindings_are_generated() {
        let _vector = FusionVector {
            array: [0.0, 0.0, 0.0],
        };
    }
}
