# `get-shell`

![Downloads](https://img.shields.io/crates/d/get-shell)
![License](https://img.shields.io/crates/l/get-shell)
[![crates.io](https://img.shields.io/crates/v/get-shell?logo=rust)](https://crates.io/crates/get-shell)
[![docs.rs](https://docs.rs/get-shell/badge.svg)](https://docs.rs/get-shell)
A simple library to get the shell your binary is runned from

## Installation

Add `get-shell = "0.1.0"` to `[dependencies]` section in your `Cargo.toml`

## Usage

```rs
use get_shell::get_shell_name;

println!("{}", get_shell_name().unwrap());
```
