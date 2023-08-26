# tera-rand

[![build status]][actions]
[![latest version]][crates.io]
[![docs]][`tera-rand` documentation]
[![rustc version 1.72+]][rust 1.72]

## tera-rand
`tera-rand` is a library of random data generation functions for the [Tera] template engine. 

To use `tera-rand` in your project, include the following in your `Cargo.toml`:
```toml
[dependencies]
tera-rand = "0.1.2"
```

Please see [`tera-rand` documentation] for examples on using `tera-rand` functions.

## tera-rand-cli

`tera-rand-cli` is a command-line tool for generating a feed of random data from a [Tera] template. 
This random data can be useful for tasks such as simulating traffic or populate data stores. Please
see [`tera-rand-cli` documentation] for examples and use cases.

You can install a `tera-rand-cli` binary from crates.io using `cargo install tera-rand-cli@0.1.1`. 

Alternatively, if you would like to build from source, ensure you have Rust installed at version
1.72 or higher. Then, checkout this repository and run`cargo build --release` from the root 
project directory. The binary should be located under directory `target/release`.

[build status]: https://img.shields.io/github/actions/workflow/status/philosobyte/tera-rand/ci.yml?branch=main
[actions]: https://github.com/philosobyte/tera-rand/actions?query=branch%3Amain
[latest version]: https://img.shields.io/crates/v/tera_rand.svg
[crates.io]: https://crates.io/crates/tera-rand
[docs]: https://docs.rs/tera-rand/badge.svg
[rustc version 1.72+]: https://img.shields.io/badge/rustc-1.72+-lightgray.svg
[rust 1.72]: https://blog.rust-lang.org/2023/08/24/Rust-1.72.0.html
[`tera-rand` documentation]: https://docs.rs/tera-rand
[`tera-rand-cli` documentation]: https://docs.rs/tera-rand-cli

[Tera]: https://github.com/Keats/tera
