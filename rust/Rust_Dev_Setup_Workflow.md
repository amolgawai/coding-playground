# Rust Development

## IDE tools

### LSP options

With [LSP](https://microsoft.github.io/language-server-protocol/)
Option 1 (Old) - rls

    rustup component add rls rust-analysis rust-src

Option 2 (preferred) - [rust-analyzer](https://github.com/rust-analyzer/rust-analyzer)

### Emacs

[setup guide](https://robert.kra.hn/posts/2021-02-07_rust-with-emacs/#quickstart)

## Project Structure

This is how Cargo documentation describes about the recommended project layout,

    ├── Cargo.toml
    ├── Cargo.lock
    ├── src
    │   ├── main.rs
    │   ├── lib.rs
    │   └── bin
    │       └── another_executable.rs
    ├── tests
    │   └── some_integration_tests.rs
    ├── benches
    │   └── simple_bench.rs
    └── examples
        └── simple_example.rs

The source code goes in the src directory.
The default executable file is src/main.rs.
The default library file is src/lib.rs.
Other executables can be placed in src/bin/*.rs.
Integration tests go in the tests directory (unit tests go in each file they’re testing).
Benchmarks go in the benches directory.
Examples go in the examples directory.

Project structure can be generated using [cargo-generate](https://github.com/cargo-generate/cargo-generate) which has many [templates](https://github.com/cargo-generate/cargo-generate/blob/master/TEMPLATES.md).

## Interesting [crates](https://crates.io)

### CLI

- [shrust  - interactive CLI shells](https://crates.io/crates/shrust)
- [structopts - parse CLI arguments](https://crates.io/crates/structopt)
- [clap - Command Line Argument Parser for Rust](https://github.com/clap-rs/clap)
- [dialoguer - build useful small interactive user inputs for the command line](https://docs.rs/dialoguer/0.8.0/dialoguer/)
- [Cross-platform Terminal Manipulation Library](https://crossterm-rs.github.io/crossterm/)
- [tui-rs - library to build rich terminal user interfaces and dashboards](https://github.com/fdehau/tui-rs)
- [indicatif - build command line interfaces that report progress to users](https://docs.rs/indicatif/0.16.0/indicatif/)

### Misc

- [serde - serialise/deserialise framework](https://crates.io/crates/serde)
- [Chrono: Date and Time for Rust](https://docs.rs/chrono/0.4.19/chrono/)
- [tracing - framework for instrumenting Rust programs to collect structured, event-based diagnostic information](https://docs.rs/tracing/0.1.26/tracing/)
- [anyhow - This library provides anyhow::Error, a trait object based error type](https://github.com/dtolnay/anyhow)
- [eyre - is a fork of anyhow with a support for customized error reports](https://docs.rs/eyre/0.6.5/eyre/)
- [color_eyre - colorful, consistent, and well formatted error reports for all kinds of errors.](https://docs.rs/color-eyre/0.5.11/color_eyre/)
- [indoc - provides a procedural macro for indented string literals](https://docs.rs/indoc/1.0.3/indoc/)

## Dev Tool-chain

Add following at command-line  for cargo mode to work properly

    ;; In order to run cargo-process-fmt you need to have the rustfmt package installed.
    rustup component add rustfmt-preview

    ;; In order to run cargo-process-check you need to have the cargo-check package installed.
    cargo install cargo-check

    ;; In order to run cargo-process-clippy you need to have the clippy package installed.
    rustup component add clippy-preview

    ;; In order to run cargo-process-{add,rm,upgrade} you need to have the cargo-edit package installed.
    cargo install cargo-edit

    ;; project structure generation
    cargo install cargo-generate

### Misc

- [pretty_assertions - beautify assertion output](https://docs.rs/pretty_assertions/0.7.2/pretty_assertions/)
- [Criterion.rs helps you write fast code by detecting and measuring performance improvements or regressions, even small ones, quickly and accurately](https://github.com/bheisler/criterion.rs)
