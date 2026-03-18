# Rust

## Introduction

This is just a repo for me to learn Rust with.

## Building

You need the rust-analyzer VS Code extension, and you must install the rust toolchain. Note that you must restart currently active shell for any PATH changes.

## Projects

### `src/helloworld`

Basic Rust project for learning fundamentals — variables, control flow, loops, references, and modules.

### `src/multi-bin` — Multiple Binaries in One Crate

Demonstrates how to have multiple executables in a single Cargo project using the `src/bin/` directory. All binaries share code via `lib.rs`.

```bash
cargo run --manifest-path src/multi-bin/Cargo.toml              # default binary
cargo run --manifest-path src/multi-bin/Cargo.toml --bin greeter # greeter binary
cargo run --manifest-path src/multi-bin/Cargo.toml --bin counter # counter binary
cargo test --manifest-path src/multi-bin/Cargo.toml              # run shared lib tests
```

### `src/workspace-demo` — Cargo Workspace

Demonstrates a Cargo workspace with three crates: a shared library (`shared`), a server binary (`app-server`), and a CLI binary (`app-cli`). All crates share one `Cargo.lock` and `target/` directory.

```bash
cargo build --manifest-path src/workspace-demo/Cargo.toml              # build all crates
cargo run --manifest-path src/workspace-demo/Cargo.toml -p app-server  # run server
cargo run --manifest-path src/workspace-demo/Cargo.toml -p app-cli     # run cli (try: -- admin)
cargo test --manifest-path src/workspace-demo/Cargo.toml               # test all crates
```

## Editor Setup

**VSCode**: Build/run/debug configurations are in `.vscode/tasks.json` and `.vscode/launch.json`. Use the Run and Debug panel (Ctrl+Shift+D) to select a target. Requires the C/C++ extension for debugging.

**Zed**: Task definitions are in `.zed/tasks.json`. Open the task runner to build/run/test any project.

## Useful Cargo commands

Cargo is the central tool when working with Rust. It starts, builds, runs and manages dependencies.

* `cargo init`: [docs](https://doc.rust-lang.org/cargo/commands/cargo-init.html) initialises a new rust project, including executables and libraries.
* `cargo build`: builds the current project.
* `cargo run`: builds and runs the current build.
* `cargo add`: [docs](https://doc.rust-lang.org/cargo/commands/cargo-add.html) adds a dependency by updating the toml config.

When adding dependencies, you may need to modify the toml file directly to add or remove features of dependencies.
