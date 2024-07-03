# Fusion AHRS

This library provides idiomatic Rust bindings to the Fusion AHRS C library. See <https://github.com/xioTechnologies/Fusion> for algorithm details.

## Usage

Bindings are generated from the C headers at build time using a `build.rs` script. This script also compiles the C library for the correct target. Both require a C compiler to be installed.

When cross-compiling for other targets you may get an error like:

```text
fatal error: 'math.h' file not found
```

This can be solved by setting the `FUSION_IMU_INCLUDE_PATH` environment variable to the folder that contains the `math.h` header for the target you're compiling for.

## Features

- `serde` - Enables serde support for the input and output types of this crate.
- `defmt` - Derives `defmt::Format` on the input and output types of this crate.

## License

This crate is licenced under the MIT license ([LICENSE-MIT](LICENSE-MIT) or <http://opensource.org/licenses/MIT>). The C library is licensed under the same license.
