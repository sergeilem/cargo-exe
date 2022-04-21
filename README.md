# Cargo-EXE

[
![Crates.io](https://img.shields.io/crates/v/cargo-exe?logo=rust)
](https://crates.io/crates/cargo-exe)
[
![docs.rs](https://docs.rs/cargo-exe/badge.svg)
](https://docs.rs/cargo-exe)

This small utility command is an extension for Cargo to print the path to the output of `cargo build`.
It is intended primarily for use in shell scripts, but may also be useful as a general purpose command.


## Installation

With [Cargo](https://github.com/rust-lang/cargo) installed, the following command will build and install `cargo-exe` from [crates.io](https://crates.io) automatically:

```bash
$ cargo install cargo-exe
```

As long as `$HOME/.cargo/bin/` is included in `$PATH`, the subcommand should be available immediately.


## Usage

The simplest usage is to run the subcommand with no arguments.
This returns the path to the Debug build.

```bash
$ cargo exe
./target/debug/binary
```

With the `--release` flag, the Release build will be returned instead.

```bash
$ cargo exe --release
./target/release/binary
```

If the `--latest` flag is enabled, the program will actually search through everything in `./target`, and print the path to the *most recently modified* binary it can find.
This may be especially helpful in a script that attempts to perform further operations on an executable after building it:

```bash
#!/bin/bash
cargo build "$@" && mv $(cargo exe --latest) ~/.local/bin
```

If a path is provided as an argument, either to a `Cargo.toml` file or to a directory containing one, that project will be used instead of the one in the current directory.

```bash
$ cargo exe /devel/rust/project
/devel/rust/project/target/debug/project
```

Finally, the `-f` option allows manual specification of the file name to search for.
This bypasses the need to determine the executable name by reading a manifest file.
In addition to searching for the latest version of any arbitrary file involved in the build, this enables generating paths to builds of projects that do not exist.

This may or may not have any practical application.

```bash
$ cargo exe /root/aaa -f fake.exe
/root/aaa/target/debug/fake.exe
```
