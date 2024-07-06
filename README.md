# Fusion AHRS

[![Crates.io](https://img.shields.io/crates/v/fusion-imu.svg)](https://crates.io/crates/fusion-imu)
[![Docs](https://docs.rs/fusion-imu/badge.svg)](https://docs.rs/fusion-imu/latest/fusion-imu/)
[![CI](https://github.com/avsaase/fusion-imu/workflows/CI/badge.svg)](https://github.com/avsaase/fusion-imu/actions)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/avsaase/fusion-imu)

This library provides idiomatic Rust bindings to the Fusion AHRS C library. See <https://github.com/xioTechnologies/Fusion> for algorithm details.

## Usage

Bindings are generated from the C headers at build time using a `build.rs` script. This script also compiles the C library for the correct target. Both require a C compiler to be installed.

When cross-compiling for other targets you may get an error like:

```text
fatal error: 'math.h' file not found
```

To solve this, set the `FUSION_IMU_INCLUDE_PATH` environment variable to the folder that contains the `math.h` header for the target you're compiling for.

## Features

- `serde` - Enables serde support for the input and output types of this crate.
- `defmt` - Derives `defmt::Format` on the input and output types of this crate.

## License

This crate is licenced under the MIT license ([LICENSE-MIT](LICENSE-MIT) or <http://opensource.org/licenses/MIT>). The C library is licensed under the same license.
