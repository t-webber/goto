# Go To Location

## Overview

This is a simple tool to help you navigate to a location in your file system. It is a simple wrapper around `cd` and `ls` commands. It is designed to be used in a shell environment. It is written in `Rust` and is cross-platform.

It works on [WSL](https://learn.microsoft.com/en-us/windows/wsl/install).

## Usage

You can find examples and documentation [here](../rust_doc/doc/goto/index.html).

## Requirements

You need to have `rustc` and `cargo` installed on your system. You can install it from [here](https://www.rust-lang.org/tools/install).

The usage of `goto` requires script authorisation. You can do this by running the following command:

- Linux: `chmod +x release/gt`
- Windows: See [here](https://learn.microsoft.com/en-us/powershell/module/microsoft.powershell.core/about/about_execution_policies?view=powershell-7.4)

## Installation

1. Compile the project

```bash
cargo build --release
```

2. Move the executable to `release`:

```bash
cp ./target/debug/goto* ./release
```

3. Add the `release` folder to your `PATH` environment variable.

   - On a Unix based OS: `echo "export PATH=$(pwd)/release/:PATH" >> ~/.bashrc`
   - On Windows: `Environment Variables` (start menu) -> `Environment Variables` -> `Path` -> `Edit` -> `New` -> `C:\path\to\release`
