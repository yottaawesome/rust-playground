# Rust

## Introduction

This is just a repo for me to learn Rust with.

## Building

You need the rust-analyzer VS Code extension, and you must install the rust toolchain. Note that you must restart currently active shell for any PATH changes.

## Useful Cargo commands.

Cargo is the central tool when working with Rust. It starts, builds, runs and manages dependencies.

* `cargo init`: [docs](https://doc.rust-lang.org/cargo/commands/cargo-init.html) initialises a new rust project, including executables and libraries.
* `cargo build`: builds the current project.
* `cargo run`: builds and runs the current build.
* `cargo add`: [docs](https://doc.rust-lang.org/cargo/commands/cargo-add.html) adds a dependency by updating the toml config.

When adding dependencies, you may need to modify the toml file directly to add or remove features of dependencies.
