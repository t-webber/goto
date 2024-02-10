// clippy::restriction,
#![warn(clippy::all, clippy::pedantic, clippy::nursery, clippy::cargo)]
#![feature(stmt_expr_attributes)]
#![allow(clippy::implicit_return, clippy::single_call_fn)]
#![allow(clippy::expect_used, clippy::panic)]

//! Main module: `goto` is a command line tool to navigate through directories.

// use core::iter;
use regex::Regex;
use crate::global::{Cmd, ToCmd};
use std::collections;
use std::env;
use std::process;

/// This module contains the functions to push and pop directories from the history file.
mod dirs;
/// This module contains all the static functions avalaible in all the program.
mod global;
/// This module contains the functions to read and write the supported directories, their shortcuts and their usage.
mod hist;

/// Structure to contain all the static data of the program
struct GlobalData<'global> {
    /// Path to the file containing the directories
    dirs: &'global str,
    /// Path to the file containing the history for popd/pushd
    hist: &'global str,
    /// When a folder is used, `incr` is used to increment the usage of the folder
    incr: u32,
    /// Gives the number of arguments for each supported command (except the basic `goto` command that can take 0, 1 or 2 arguments).
    argcs: collections::HashMap<&'global str, usize>,
    unix: bool,
    aliass: collections::HashMap<&'global str, &'global str>,
    no_read: [&'global str; 5],
}

/// Default implementation for `GlobalData`
impl<'global> Default for GlobalData<'global> {
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
        aliass.insert("-g", "-get");
        aliass.insert("-d", "-delete");
        aliass.insert("-e", "-edit");
        aliass.insert("-res", "-reset");
        aliass.insert("-c", "-clear");
        aliass.insert("?", "-state");
        aliass.insert("-dec", "-decrement");
        aliass.insert("-p", "-pop");

        let unix = cfg!(target_os = "linux");
        assert!(unix || cfg!(target_os = "windows"), "Unsupported OS");

        Self {
            dirs: if unix {"/mnt/w/files/dev/rust/goto/lib/dirs.csv" } else  {"w:/files/dev/rust/goto/lib/dirs.csv" },
            hist: if unix {"/mnt/w/files/dev/rust/goto/lib/hist.csv" } else  {"w:/files/dev/rust/goto/lib/hist.csv" },
            incr: 10,
            unix,
            argcs,
            aliass,
            no_read: ["-noclear", "-code", "-still", "-pop", "-state"],
        }
    }
}

///////////////////////////////: dir global functions  :///////////////////////////////

/// Find the path of the directory to go to.
/// # Arguments
/// * `dirs` - The path to the file containing the directories
/// * `hist` - The path to the file containing the history for popd/pushd
/// * `args` - The arguments of the command
/// * `incr` - The increment to add to the usage of the directory
/// # Returns
/// The path of the directory to go to, if the command is valid.
/// # Panics
/// If the command is invalid.
/// # Note
/// This function is used to find the path of the directory to go to, and to update the usage of the directory if the command is valid.
/// The function also prints the path of the directory to go to, and calls the `code` function to open the directory in Visual Studio Code.
/// The function also calls the `clear` function to clear the terminal, unless the `noclear` argument is present.
fn no_read(dirs: &str, hist: &str, args2: &[String]) -> Option<String> {
    let mut res = None;
    #[rustfmt::skip]
    args2.iter().for_each(|arg| match arg.as_str() {
        "-pop" => res = Some(hist::popd(hist)),
        "-state" => dirs::state(dirs),
        "-code" | "-noclear" => (),
        "-still" => todo!(),
        _ => panic!("Invalid command <{}> in <{}>", arg, env::args().collect::<Vec<String>>().join(" ")),
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
/// The function panics if the `code` command is not found.
/// The function is called after finding the path of the directory to go to, and after updating the usage of the directory.
///
fn code(args2: &[String], path: &str) {
    if args2.contains(&String::from("code")) {
        process::Command::new("code")
            .arg(path)
            .spawn()
            .unwrap_or_else(|er| panic!("Unable to open code {er}"));
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
/// The function is used to avoid the use of `env::args().nth(n).expect("message")` that would panic if the iterator does not have enough elements.
/// The function is also used to check that the number of arguments of the command is valid.
/// The function is also used to separate the arguments of the command that are not part of the command.
///
// #[rustfmt::skip]
fn get_args(gdata: &GlobalData) -> (Vec<Cmd>, Vec<String>, bool) {
    let mut cmdline = env::args().skip(1);
    let mut args1: Vec<Cmd> = vec![];
    let mut args2: Vec<String> = vec![];
    let mut get = false;
    let mut last: Option<String> = None;

    loop {
        match cmdline.next() {
            None => break,
            Some(arg) => {
                let curr = (*gdata.aliass.get(arg.as_str()).unwrap_or(&arg.as_str())).to_string();

                #[rustfmt::skip]
                match gdata.argcs.get(curr.as_str()) {
                    Some(value) => {
                        assert!(cmdline.len() >= *value, 
                                "Missing argument of {} in <{}>", 
                                &curr, env::args().collect::<Vec<String>>().join(" ")
                        );
                        last = Some(curr.clone());
                        args1.push(curr.to_cmd().unwrap());
                    }

                    None => match curr.as_str() {
                        "features" => break,
                        "-get" => { last = Some(String::from("-get")); get = true; let cmd = curr.to_cmd().unwrap(); args1.push(cmd); }
                        // Is a no-read command (code, clear, still, pop, state, noclear, etc.)
                        _ if gdata.no_read.contains(&curr.as_str()) => args2.push(curr.clone()),
                        _ if last.is_none() => { args1.push(Cmd::Get([Some(curr.clone()), None])); last = Some(String::from("-get")); }
                        _ if last.as_ref().unwrap() == "-get" => {
                            let last =  args1.last_mut().unwrap_or_else(|| panic!("Invalid command <{}> in <{}>",
                                                                 curr, env::args().collect::<Vec<String>>().join(" ")));
                            last.append(curr);
                        },
                        _ => { panic!("Invalid command <{}> in <{}>", curr, env::args().collect::<Vec<String>>().join(" ")); },
                    },
                }
            },
        };
    }

    if args1.is_empty() {
        args1.push(Cmd::default());
    }

    (args1, args2, get)
}

///////////////////////////////: main functions  :///////////////////////////////

fn main() {
    let gdata = GlobalData::default();
    let (args1, args2, get) = get_args(&gdata);
    dbg_print!(">>> <{:?}> with <{:?}>\n", args1, args2);
    let path  = dirs::read(gdata.dirs, &args1, gdata.incr);
    let os_path: Option<String> = if gdata.unix {
        let re = Regex::new(r"(?i)[a-z]:").unwrap();
        path.map(|p| re.replace(&p, |caps: &regex::Captures| format!("/mnt/{}/", &caps[0].to_lowercase())).to_string())
    } else {path};

    if let Some(found) = &os_path {
        hist::pushd(gdata.hist, found);
        code(&args2, found);
    };

    #[allow(clippy::print_stdout)]
    no_read(gdata.dirs, gdata.hist, &args2);
    print!(
        "{}#{}#{}",
        u8::from(!args2.contains(&String::from("-noclear"))),
        u8::from(get),
        os_path.unwrap_or_default()
    );
}
