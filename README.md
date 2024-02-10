# Go To Location

## Overview

This is a simple tool to help you navigate to a location in your file system. It is a simple wrapper around `cd` and `ls` commands. It is designed to be used in a shell environment. It is written in `Rust` and is cross-platform.

## Usage

## Prerequisites

You need to have `rustc` and `cargo` installed on your system. You can install it from [here](https://www.rust-lang.org/tools/install).

The usage of `goto` requires authorisation. You can do this by running the following command:

- Linux: `chmod +x release/goto`
- Windows: See [here](https://learn.microsoft.com/en-us/powershell/module/microsoft.powershell.core/about/about_execution_policies?view=powershell-7.4)

## Installation

You need to compile the rust project using `cargo`:

```bash
cargo run --release
```

Then, add the folder `release` to your `PATH` environment variable.
