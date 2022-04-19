![Downloads](https://img.shields.io/crates/d/query-shell)
![License](https://img.shields.io/crates/l/query-shell)
[![crates.io](https://img.shields.io/crates/v/query-shell?logo=rust)](https://crates.io/crates/query-shell)
[![docs.rs](https://docs.rs/query-shell/badge.svg)](https://docs.rs/query-shell)

# `query-shell`

> A simple library to get the user's shell.

Forked from [alicecarroll/get-shell](https://gitlab.com/alicecarroll/get-shell).

## Installation

Add `query-shell = "0.1"` to `[dependencies]` section in your `Cargo.toml`.

## Usage

```rs
use query_shell::get_shell_name;

println!("{}", get_shell_name().unwrap());
```
