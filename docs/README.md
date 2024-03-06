# Go To Location

## Overview

This is a simple tool to help you navigate to a location in your file system. It is a simple wrapper around the `cd` command. It is designed to be used in a shell environment.

Create a shortcut to a directory that you use often, and then use the shortcut to navigate to that directory. It is a simple way to save time and reduce the number of keystrokes needed to navigate to a directory.

You can also use it as a plain `cd` command, and it will work the same way.

## Tools

- `g`: go to a location in the file system (usage: shell)
- `c`: open a location in `VSCode` (usage: shell or run window)
- `wc`: open a location in `VSCode` in a remote `WSL` window (usage: shell or run window)
- `e`: open a location in the file explorer (usage: shell or run window)

It is written in `Rust` and is cross-platform, available on Windows and Unix-based systems. It also works on [WSL](https://learn.microsoft.com/en-us/windows/wsl/install) with the same shortcuts

## Usage

You can find examples and documentation [here](./cargo/doc/goto/index.html).

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
