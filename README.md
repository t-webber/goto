# Go To Location

> A command line tool to navigate fast and easily through directories, open apps in the right path, and more.

## Overview

This is a simple tool to help you navigate to a location in your file system. It is a simple wrapper around `cd` and `ls` commands. It is designed to be used in a shell environment. It is written in `Rust` and is cross-platform.

## Documentation & usage

The documentation with examples of use can be found [here](./doc/doc/goto/index.html)

## Prerequisites

You need to have `rustc` and `cargo` installed on your system. You can install it from [here](https://www.rust-lang.org/tools/install).

The usage of `goto` requires script authorisation to run. You can achieve this by running the following command:

- **Linux**: `chmod +x release/gt`
- **Windows**: See [here](https://learn.microsoft.com/en-us/powershell/module/microsoft.powershell.core/about/about_execution_policies?view=powershell-7.4)

## Installation

First, create two files for your settings:

```bash
touch lib/dirs.csv
touch lib/hist.csv
```

You need to compile the rust project using `cargo`:

```bash
cargo run --release
```

Then, add the folder `release` to your `PATH` environment variable.

- **Linux**: `echo "export PATH=$PATH:/path/to/release" >> ~/.bashrc`
- **Windows**:
  1. Copy the path to the `release` folder (`cd release; echo $(pwd)` then copy it).
  2. Open the start menu, type `env` and select `Edit the system environment variables`.
  3. Click on `Environment Variables` in the bottom right corner.
  4. Select `Path` in the `User variables` (top half of the screen) and click on `Edit`.
  5. Click on `New` and paste the path you copied at the beginning.

## Known Issues

They can be found [here](./doc/todo.md)

## Credits

On **Windows**, the executable files were created using `PS2EXE`. You can find it [here](https://github.com/MScholtes/PS2EXE).
