#![warn(
    clippy::all,
    clippy::pedantic,
    // clippy::restriction,
    clippy::nursery,
    clippy::cargo
)]
#![feature(stmt_expr_attributes)]
#![allow(clippy::implicit_return, clippy::single_call_fn)]
#![allow(clippy::pattern_type_mismatch)]
///////////////////////////////: Documentation  :///////////////////////////////

//! `goto` is a command line tool to navigate through directories.
//! # Arguments
//! * None: Go to the directory associated with the shortcut.
//!     - Usage: `. gt [shortcut]`.
//!     - Note: If no shortcut is given, you will travel to the most used directory.
//! * `-add` (or `-a`) - Add a directory to the list of supported directories.
//!     - Usage: `. gt -add [shortcut] [path]`.
//!     - Note: If path is not given, the current directory is used. If shortcut is not given, the name of the current folder is used.
//! * `-remove` (or `-rm`) - Remove a directory from the list of supported directories.
//!     - Usage: `. gt -remove [shortcut]`.
//!     - Panics: If no shortcut is given.
//! * `-delete` (or `-del`) - Remove a shortcut from the list of supported shortcuts.
//!     - Usage: `. gt -delete [path]`.
//!     - Panics: If no path is given.
//! * `-edit` - Edit the path of a directory associated with a shortcut.
//!     - Usage: `. gt -edit [shortcut] [new path]`.
//!     - Note: If no new path is given, the current directory is used. If no shortcut is given, the name of the current folder is used.
//! * `-reset` - Reset the usage of all the directories in the list of supported directories (all set to 0, ⚠️no confirmation and no backup⚠️).
//! * `-decrement` (or `-decr`) - Decrement the usage of a directory in the list of supported directories.
//!     - Usage: `. gt -decrement [int]`.
//!     - Panics: If no decrementation level is given.
//! * `-pop` (or `-p`) - Pop the last directory from the history of directories (only works if you use only `goto` to navigate through directories).
//! * `-state` (or `?`) - Print the state of the list of supported directories.
//!     - Note: The state is printed in the following format: `path shortcut1 shortcut2 ... priority_level\n...`.
//! * `-noclear` (or `-nc` or `!`) - The terminal is cleared by default after the command. Use this argument to avoid clearing the terminal.
//! * `-code` (or `-c`) - Open the directory in Visual Studio Code.
//! * `-still` (or `%`) - Don't change directory after the command (useful with `-code`).
//! * `-get` (or `-g`) - Get the path of the directory to go to (implies `-still` and `-noclear`).
//!    - Usage: `. gt -get [shortcut]`.
//!    - Note: If no shortcut is given, you will get the path of the most used directory.
//! * `-clear` (or `-cls`) - Totally errases the list of supported directories (⚠️no confirmation and no backup⚠️).
//! # Examples
//! In reality, the command will clear the terminal once the command has finished, but the examples are written as if the terminal was not cleared.
//! ### Add
//! ```bash
//! ~ $ . gt -add mydir /path/to/dir
//! ~ $ . gt mydir
//! /path/to/dir $ . gt -add mydir2 /path/to/dir
//! ~ $ cd /
//! ~ $ . gt mydir2
//! /path/to/dir $
//! ```
//! ### Edit
//! ```bash
//! ~ $ . gt -edit mydir /new/path/to/dir
//! ~ $ . gt mydir
//! /new/path/to/dir $ . gt -remove mydir
//! /new/path/to/dir $ . gt mydir // Error: mydir is not a valid shortcut
//! ```
//! ### Pop
//! ```bash
//! ~ $ . gt -add shortcut1 /path/to/a/dir
//! ~ $ . gt -add shortcut2 /snd/path/to/dir
//! ~ $ . gt shortcut1
//! /path/to/a/dir $ . gt shortcut2
//! /snd/path/to/dir $ . gt -pop
//! /path/to/a/dir $
//! ```
//! ### Code
//! ```bash
//! ~ $ . gt -code shortcut1     // Opens vscode in /new/path/to/dir
//! /path/to/a/dir $           // Add -still to avoid changing directory
//! ````
//! # Note
//! In `powershell`, you don't need to use the `.` before the command.
//! Not yet supported by `cmd`.

///////////////////////////////: Imports  :///////////////////////////////

/// This module contains the structure of the options of the goto command that access the file containing the shorcuts. (-get, -add, -edit, ...)
mod commands;
/// This module contains the functions to push and pop directories from the history file.
mod dirs;
/// This module contains all the error functions avalaible in all the program.
mod errors;
/// This module contains the functions to read and write the supported directories, their shortcuts and their usage.
mod hist;

use errors::WriteError;

use crate::commands::{AppendDefault, Cmd, ShortPath, ToCmd};
use crate::errors::{CommandError, ReadError};

use std::env;
use std::process;
use std::{collections, fs};

///////////////////////////////: Global static data  :///////////////////////////////

/// Structure to contain all the static data of the program
struct GlobalData<'global> {
    /// Path to the file containing the list of shortcuts defined by the user with the `-add` and `-edit` commands.
    dirs: String,
    /// Path to the file containing the history for the `-pop` command.
    hist: String,
    /// When a folder is used, `incr` is used to increment the usage of the folder
    incr: u32,
    /// Gives the number of arguments for each supported command (except the basic `goto` command that can take 0, 1 or 2 arguments).
    argcs: collections::HashMap<&'global str, usize>,
    /// `true` if the OS is unix, `false` if the OS is windows
    unix: bool,
    /// Gives the alias of every supported command
    aliass: collections::HashMap<&'global str, &'global str>,
    /// Gives the arguments that don't require reading `lib/dirs.csv`.
    no_dirs: [&'global str; 6], //TODO: Specifying here the length is awful.
}

/// Defining the data for `GlobalData`
impl<'global> Default for GlobalData<'global> {
    #[allow(clippy::expect_used)]
    fn default() -> Self {
        let mut argcs = collections::HashMap::new();
        argcs.insert("-add", 2);
        argcs.insert("-remove", 1);
        argcs.insert("-delete", 1);
        argcs.insert("-edit", 2);
        argcs.insert("-reset", 0);
        argcs.insert("-del", 1);
        argcs.insert("-decrement", 1);

        let mut aliass = collections::HashMap::new();
        aliass.insert("-a", "-add");
        aliass.insert("-rm", "-remove");
        aliass.insert("-del", "-delete");
        aliass.insert("-decr", "-decrement");
        aliass.insert("-p", "-pop");
        aliass.insert("?", "-state");
        aliass.insert("-nc", "-noclear");
        aliass.insert("!", "-noclear");
        aliass.insert("-c", "-code");
        aliass.insert("%", "-still");
        aliass.insert("-g", "-get");
        aliass.insert("-cls", "-clear");

        let unix = cfg!(target_os = "linux");
        assert!(unix || cfg!(target_os = "windows"), "Unsupported OS");

        let curr = env::current_exe()
            .read_error("main.rs", None)
            .parent()
            .expect("No parent of main.rs.")
            .parent()
            .expect("No grandparent of main.rs.")
            .join("lib");

        Self {
            dirs: match curr.join("dirs.csv").into_os_string().into_string() {
                Ok(path) => path,
                Err(er) => {
                    user_error!("Unable to read file dirs.csv: {er:?}");
                    String::default()
                }
            },
            hist: match curr.join("hist.csv").into_os_string().into_string() {
                Ok(path) => path,
                Err(er) => {
                    user_error!("Unable to read file hist.csv: {er:?}");
                    String::default()
                }
            },
            incr: 10,
            unix,
            argcs,
            aliass,
            no_dirs: ["-noclear", "-code", "-still", "-pop", "-state", "-clear"],
        }
    }
}

///////////////////////////////: No dirs functions  :///////////////////////////////

/// Find the path of the directory to go to.
/// # Arguments
/// * `dirs` - The path to the file containing the directories
/// * `hist` - The path to the file containing the history for popd/pushd
/// * `args` - The arguments of the command
/// * `incr` - The increment to add to the usage of the directory
/// # Returns
/// The path of the directory to go to, if the command is valid.
/// # Warning
/// If the command is invalid.
/// # Note
/// This function is used to find the path of the directory to go to, and to update the usage of the directory if the command is valid.
/// The function also prints the path of the directory to go to, and calls the `code` function to open the directory in Visual Studio Code.
/// The function also calls the `clear` function to clear the terminal, unless the `noclear` argument is present.
fn no_dirs(dirs: &str, hist: &str, args2: &[String]) -> Option<String> {
    let mut res = None;
    #[rustfmt::skip]
    args2.iter().for_each(|arg| match arg.as_str() {
        "-pop" => res = Some(hist::popd(hist)),
        "-state" => dirs::state(dirs),
        "-clear" => fs::write(dirs, "").write_error(dirs),
        "-code" | "-noclear" | "-still" => (),
        _ => user_error!("Invalid command <{}> in <{}>", arg, env::args().collect::<Vec<String>>().join(" ")),
    });
    res
}

/// Open the directory in Visual Studio Code.
/// # Arguments
/// * `args2` - The arguments of the command
/// * `path` - The path of the directory to open
/// # Note
/// This function is used to open the directory in Visual Studio Code, if the `code` argument is present.
/// The function uses the `code` command to open the directory in Visual Studio Code.
/// The function raises a warning if the `code` command is not found.
/// The function is called after finding the path of the directory to go to, and after updating the usage of the directory.
///
fn vscode(args2: &[String], path: &str) {
    if args2.contains(&String::from("-code")) {
        match process::Command::new("code").arg(path).spawn() {
            Ok(mut subprocesses) => match subprocesses.wait() {
                Ok(_) => (),
                Err(er) => command_error!("Unable to open VSCode: {er}"),
            },
            Err(er) => command_error!("Unable to open VSCode: {er}"),
        }
    }
}

/// Clear the terminal, unless the `noclear` argument is present.
/// # Arguments
/// * `args2` - The `no_dirs` arguments of the command
/// * `get` - `true` if the `get` argument is present, `false` otherwise
/// # Note
/// This function is used to clear the terminal, unless the `-noclear` argument is present.
/// The function is called after opening the directory in Visual Studio Code, and after updating the usage of the directory.
/// The function is also called at the beginning of the program, to clear the terminal before the command is executed.
#[rustfmt::skip]
#[allow(clippy::print_stderr)]
fn clear_terminal(args2: &[String], get: bool) {
    if !args2.contains(&String::from("-noclear")) && !get && !args2.contains(&String::from("-still")) {
        //TODO: This is horrible, but I don't know how to clear the terminal in a better way.
        eprint!("\x1B[2J\x1B[1;1H");
    }
}

///////////////////////////////: goto functions  :///////////////////////////////

/// Get the arguments of the command.
/// # Arguments
/// * `argcs` - The number of arguments for each supported command
/// # Returns
/// A tuple containing the arguments of the command, and the arguments of the command that are not part of the command.
/// # Note
/// This function is used to get the arguments of the command, and to separate the arguments of the command that are not part of the command.
/// The function also checks that the number of arguments of the command is valid.
/// The function is called at the beginning of the program, to get the arguments of the command, and to separate the arguments of the command that are not part of the command.
/// The function is also used to check that the number of arguments of the command is valid.
/// The function is also used to separate the arguments of the command that are not part of the command.
///
// #[rustfmt::skip]
fn get_args(gdata: &GlobalData) -> (Vec<Cmd>, Vec<String>, bool) {
    let mut cmdline = env::args().skip(1);
    let mut args1: Vec<Cmd> = vec![];
    let mut args2: Vec<String> = vec![];
    let mut get = false;
    let here11 = env::current_dir();
    let here22 = here11.command_error("Unable to get current directory. Access denied.");
    let here = here22
        .to_str()
        .command_error("Unable to convert current directory to string. Access denied.");

    loop {
        let temp = cmdline.next();
        match temp {
            None => break,
            Some(arg) => {
                let curr = (*gdata.aliass.get(arg.as_str()).unwrap_or(&arg.as_str())).to_owned();

                #[rustfmt::skip]
                match gdata.argcs.get(curr.as_str()) {
                    Some(value) => {
                        if cmdline.len() < *value {
                            args1.last_mut().append_default(here);
                        };
                        args1.push(curr.to_cmd());
                    }

                    None => match curr.as_str() {
                        "features" => break,
                        "-get" => { get = true; 
                                    args1.push(curr.to_cmd()); },
                        // Is a no_dirs command (code, clear, still, pop, state, noclear, etc.)
                        _ if gdata.no_dirs.contains(&curr.as_str()) => args2.push(curr.clone()),
                        // Is an argument to a previous option
                        _ => match args1.last_mut() {
                            None => args1.push(Cmd::Get(ShortPath{ short: Some(curr.clone()), path: None})),
                            Some(last) => last.append(curr),
                            }
                    },
                }
            }
        };
    }

    args1.last_mut().append_default(here);
    if args1.is_empty() {
        args1.push(Cmd::default());
    }

    (args1, args2, get)
}

/// Convers path to unix or dos, depending on the OS.
/// # Arguments
/// * `ipath` - A path in DOS or UNIX format
/// * `unix` - `true` if the path should be converted to UNIX format, `false` if the path should be converted to DOS format
/// # Returns
/// The path in DOS or UNIX format, depending on the OS.
fn dos2unix(path: String, unix: bool) -> String {
    let chars: Vec<char> = path.chars().collect();
    if chars.get(1) == Some(&':') && unix {
        let rest: String = chars.get(3..).unwrap_or_default().iter().collect();
        format!("/mnt/{}/{}", chars.first().unwrap_or(&' '), rest)
    } else if path.contains("wsl.localhost") && unix {
        let index = path.find("wsl.localhost").unwrap_or(0);
        path.get(index..).unwrap_or_default().to_owned()
    } else if path.starts_with("/mnt/") && !unix {
        let rest: String = chars.get(6..).unwrap_or_default().iter().collect();
        format!("{}:{}", chars.get(5).unwrap_or(&'c'), rest)
    } else {
        path
    }
}

///////////////////////////////: Main  :///////////////////////////////

#[rustfmt::skip]
fn main() {
    let gdata = GlobalData::default();
    let (args1, args2, get) = get_args(&gdata);
    clear_terminal(&args2, get);
    let short_path = dirs::read(&gdata.dirs, &args1, gdata.incr);
    let pop_path = no_dirs(&gdata.dirs, &gdata.hist, &args2); // result of pop

    let read = pop_path.as_ref().is_none() && short_path.as_ref().is_some();

    let os_path = dos2unix( pop_path.unwrap_or_else(|| short_path.unwrap_or_default()), gdata.unix);

    if read { hist::pushd(&gdata.hist, &os_path); };

    vscode(&args2, &os_path);

    #[rustfmt::skip]
    #[allow(clippy::print_stdout)]
    {print!("{}#{}#{}",
            u8::from(args2.contains(&String::from("-still"))),
            u8::from(get),
            &os_path);};
}
