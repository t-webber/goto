# Go To Location

## Overview

This is a simple tool to help you navigate to a location in your file system. It is a simple wrapper around the `cd` command. It is designed to be used in a shell environment.

A tool is also provided to open `VSCode` in a specific location: `c`. You can run it in the `run` window (`windows`+`R`) or in any terminal. Thanks to `c`, you can directly open your project with `VSCode` without having to navigate to the project folder. If you are on `WSL`, you can use `wc` to open the project in `VSCode` in a remote `WSL` window.

You will also found a `expl` script: it opens a specific location in the file explorer. In the same way than `c`, you can use `expl` in the `run` window or in any terminal.

It is written in `Rust` and is cross-platform, available on Windows and Unix-based systems. It also works on [WSL](https://learn.microsoft.com/en-us/windows/wsl/install).

## Usage

You can find examples and documentation [here](../rust_doc/doc/goto/index.html).

## Requirements

You need to have `rustc` and `cargo` installed on your system. You can install them from [here](https://www.rust-lang.org/tools/install).

If you are on Windows, the usage of `goto` requires script authorization. See [here](https://learn.microsoft.com/en-us/powershell/module/microsoft.powershell.core/about/about_execution_policies?view=powershell-7.4) for more information. If you don't want to change the execution, you can still use the `c` and `wc`.

## Installation

1. Clone the projet

```bash
git clone https://www.github.com/t-webber/goto.git
cd goto
```

2. Initialize the libraries:

```bash
# sal touch ni # <-- add this line if you are on powershell
mkdir lib
touch lib/dirs.csv
touch lib/hist.csv
```

3. Compile the project

```bash
cargo build --release
```

4. Move the executable to `release`:

```bash
cp ./target/debug/goto* ./release
```

5. Add the `release` folder to your `PATH` environment variable.

   - On a Unix based OS: `echo "export PATH=$(pwd)/release/:PATH" >> ~/.bashrc`
   - On Windows:
     - Copy the path to the `release` folder (e.g. by using `pwd` in `powershell`)
     - Open the start menu
     - Tap `env` and select `Environment Variables`
     - Click on `Environment Variables` in the bottom right corner
     - Click on `Path` in the `User environment variables` (first half of the windows)
     - Click on `Edit`, then `New` on the right, and add the copied path
     - Then click on `OK`, `OK`, `OK`.
   - and restart the terminal

6. Make the file executable:

   - On a Unix based OS: `chmod +x release/goto`
   - On Windows: the usage of `goto` requires script authorization. See [here](https://learn.microsoft.com/en-us/powershell/module/microsoft.powershell.core/about/about_execution_policies?view=powershell-7.4) for more information. If you don't want to change the execution, you can still use the `c` and `wc`.

7. In order to use `wc`, you will need to change `Ubuntu` to the linux distro you have installed.
