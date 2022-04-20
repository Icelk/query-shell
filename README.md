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

## Changelog

### v0.2.0

-   Updated `sysinfo` to the latest release.
-   Improved runtime performance (from ~170ms to ~140ms).

### v0.1.0

-   Forked from [alicecarroll/get-shell](https://gitlab.com/alicecarroll/get-shell).
-   Removed default features of `sysinfo` (e.g. `rayon`).
-   Added documentation.
-   Not loading everything from the `sysinfo` crate (runtime went from >2s to <200ms).
