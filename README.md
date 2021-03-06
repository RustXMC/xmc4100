# XMC4100

> THIS IS A WORK IN PROGRESS AND MUCH IS UNTESED

[![crates.io](https://img.shields.io/crates/v/xmc4100.svg)](https://crates.io/crates/xmc4100)
[![rust](https://github.com/xmc-rs/xmc4100/workflows/Rust/badge.svg)](https://github.com/xmc-rs/xmc4100/workflows/Rust/badge.svg)

This is a 'peripheral access crate' for interfacing to the XMC4100 series of microcontrollers for embedded support in Rust that is generated using [svd2rust](https://docs.rs/svd2rust) and an SVD file provided by Infineon.

There is an optional `rt` that can be utilized. An explanation of the feature is given by [svd2rust](https://docs.rs/svd2rust/0.16.1/svd2rust/#the-rt-feature)

All API's and usage (besides what registers exist) are defined by [svd2rust](https://docs.rs/svd2rust)

## Generate Crate from SVD

```bash
# Necessary 3rd-party tools
cargo install svd2rust
cargo install form
rustup component add rustfmt

svd.sh # Generates code from crate and formats to rustfmt
```

## Using Crate

```toml
[dependencies.xmc4100]
version = "0.3.1"
features = ["rt"]
```
