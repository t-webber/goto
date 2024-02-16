# Go To Location

## Overview

This is a simple tool to help you navigate to a location in your file system. It is a simple wrapper around `cd` and `ls` commands. It is designed to be used in a shell environment. It is written in `Rust` and is cross-platform.

It works on [WSL](https://learn.microsoft.com/en-us/windows/wsl/install).

## Usage

You can find examples and documentation [here](../rust_doc/debug/doc/goto/index.html).

## Requirements

You need to have `rustc` and `cargo` installed on your system. You can install it from [here](https://www.rust-lang.org/tools/install).

If you are on Windows, the usage of `goto` requires script authorization. See [here](https://learn.microsoft.com/en-us/powershell/module/microsoft.powershell.core/about/about_execution_policies?view=powershell-7.4) for more information.

## Installation

1. Initialize the libraries and compile the project:

```bash
# sal touch ni # <-- add this line if you are on powershell
mkdir release
touch release/dirs.csv
touch release/hist.csv
```

2. Compile the project

```bash
cargo build --release
```

3. Move the executable to `release`:

```bash
cp ./target/debug/goto* ./release
```

4. Add the `release` folder to your `PATH` environment variable.

   - On a Unix based OS: `echo "export PATH=$(pwd)/release/:PATH" >> ~/.bashrc`
   - On Windows:
     - Copy the path to the `release` folder (e.g. by using `pwd` in `powershell`)
     - Open the start menu
     - Tap `env` and select `Environment Variables`
     - Click on `Environment Variables` in the bottom right corner
     - Click on `Path` in the user environment variables (first half of the windows)
     - Click on `Edit`, then `New` on the right, and add the copied path
     - Then click on `OK`, `OK`, `OK` and restart the terminal

5. If you are on a Unix-type OS, run `chmod +x release/goto` to make the file executable.
