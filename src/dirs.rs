use core::fmt::Write;
use std::fs;
use std::process;

use crate::global::Cmd;
use crate::user_error;
// Imports

/// Structure to contain the state of the search
#[derive(Default, Debug)]
struct SearchState {
    /// If a path was found for the given shortcut, `correct` contains the path.
    correct: Option<String>,
    /// If a path was not found for the given shortcut, `prioritised` contains the path with the highest priority.
    prioritised: Option<String>,
    /// The highest priority found.
    max_priory: u32,
}

/// Structure to contain the data of a line in the directory file
struct DirsLine<'dirline> {
    /// The path of the directory
    path: &'dirline str,
    /// The shortcuts of the directory
    shortcs: &'dirline [&'dirline str],
    /// The priority of the directory
    priory: u32,
    /// The priority of the directory incremented by `incr` (see `GlobalData`)
    priory2: u32,
}

impl<'dirline> DirsLine<'dirline> {
    /// Function to convert a `DirsLine` to a string
    fn join(&self, sep: &str) -> String {
        format!(
            "{}{}{}{}{}",
            self.path,
            sep,
            self.shortcs.join(sep),
            sep,
            self.priory
        )
    }
}

/// Function to get the path of a directory from the directory file
/// # Arguments
/// * `dirline` - The line of the directory file
/// * `success` - A mutable reference to a boolean to indicate if the path was found
/// * `sstate` - A mutable reference to the state of the search
/// * `shortc` - The shortcut to search for
/// # Returns
/// The line of the directory file
/// # Note
/// If the path was found, `success` is set to `true` and `sstate.correct` contains the path.
/// If the path was not found, `sstate.prioritised` contains the path with the highest priority.
/// # Example
/// ```
/// let mut success = false;
/// let mut sstate = SearchState::default();
/// let dirline = DirsLine {
///    path: "/home/user/folder",
///    shortcs: &["f", "folder"],
///    priory: 1,
///    priory2: 2,
/// };
/// let path = get(&dirline, &mut success, &mut sstate, "f");
/// ```
/// # Panics
/// If the shortcut is not found in the line
/// # Note
/// The directory file is a simple text file with the following format:
/// ```text
/// /home/user/folder;f;folder;1
/// /home/user/folder2;f2;folder2;2
/// ```
/// Where the first field is the path of the directory, the second field is the shortcuts of the directory, the third field is the priority of the directory.
/// The last line is the most recent directory pushed.
/// 
#[rustfmt::skip]
fn get(dirline: &DirsLine, success: &mut bool, sstate: &mut SearchState, shortc: &str) -> String {
    if dirline.shortcs.contains(&shortc) {
        sstate.correct = Some(String::from(dirline.path));
        *success = true;
        format!("{};{};{}", dirline.path, dirline.shortcs.join(";"), dirline.priory2)
    } else {
        if dirline.priory > sstate.max_priory {
            sstate.max_priory = dirline.priory;
            sstate.prioritised = Some(String::from(dirline.path));
        }
        dirline.join(";")
    }
}

/// Function to remove a shortcut from a line of the directory file
/// # Arguments
/// * `dirline` - The line of the directory file
/// * `success` - A mutable reference to a boolean to indicate if the shortcut was found
/// * `shortc` - The shortcut to remove
/// # Returns
/// The line of the directory file
/// # Example
/// ```
/// let mut success = false;
/// let dirline = DirsLine {
///   path: "/home/user/folder",
///  shortcs: &["f", "folder"],
/// priory: 1,
/// priory2: 2,
/// };
/// let line = remove(&dirline, &mut success, "f");
/// ```
/// # Panics
/// If the shortcut is not found in the line
/// # Note
///
fn remove(dirline: &DirsLine, success: &mut bool, shortc: &str) -> String {
    if dirline.shortcs.contains(&shortc) {
        *success = true;
        if dirline.shortcs.len() == 1 {
            return String::new();
        }
        format!(
            "{};{};{}",
            dirline.path,
            dirline
                .shortcs
                .iter()
                .filter(|&&sh| sh != shortc)
                .copied()
                .collect::<Vec<_>>()
                .join(";"),
            dirline.priory
        )
    } else {
        dirline.join(";")
    }
}

/// Function to add a shortcut to a line of the directory file
/// # Arguments
/// * `dirline` - The line of the directory file
/// * `success` - A mutable reference to a boolean to indicate if the shortcut was added
/// * `new_shortc` - The new shortcut to add
/// * `path` - The path of the directory
/// # Returns
/// The line of the directory file
/// # Example
/// ```
/// let mut success = false;
/// let dirline = DirsLine {
///   path: "/home/user/folder",
///   shortcs: &["f", "folder"],
///   priory: 1,
///   priory2: 2,
/// };
/// let line = add(&dirline, &mut success, "f2", "/home/user/folder");
/// ```
/// # Panics
/// If the shortcut already exists
fn add(dirline: &DirsLine, success: &mut bool, new_shortc: &str, path: &str) -> String {
    if dirline.shortcs.contains(&new_shortc) {
        user_error!("Shortcut {new_shortc} already exists");
        *success = true;
        dirline.join(";")
    } else if path == dirline.path {
        *success = true;
        if dirline.shortcs.contains(&new_shortc) {
            user_error!("shortc already exists");
            dirline.join(";")
        } else {
            format!(
                "{path};{};{new_shortc};{}",
                dirline.shortcs.join(";"),
                dirline.priory2
            )
        }
    } else {
        dirline.join(";")
    }
}

/// # Arguments
/// * `dirline` - The line of the directory file
/// * `success` - A mutable reference to a boolean to indicate if the shortcut was edited
/// * `shortc` - The new shortcut
/// * `path` - The path of the directory
/// # Returns
/// The line of the directory file
/// # Example
/// ```
/// let mut success = false;
/// let dirline = DirsLine {
///   path: "/home/user/folder",
///   shortcs: &["f", "folder"],
///   priory: 1,  
///   priory2: 2,
/// };
/// ```
/// # Panics
/// If the path already exists
///
fn edit(dirline: &DirsLine, success: &mut bool, shortc: &str, path: &str) -> String {
    if dirline.path == path {
        panic!("Path already exists");
    } else if dirline.shortcs.contains(&shortc) {
        *success = true;
        format!("{path};{};{}", dirline.shortcs.join(";"), dirline.priory)
    } else {
        dirline.join(";")
    }
}

/// Function to format a path
/// # Arguments
/// * `path` - The path to format
/// # Returns
/// The formatted path
/// # Example   
/// ```
/// assert!(std_path(&Some(String::from("/home/user/folder/"))) == "/home/user/folder");
/// ```
///
fn std_path(path: &Option<String>) -> String {
    let somepath = path.as_ref().expect("Path not found in command line");
    somepath.strip_suffix('/').unwrap_or(somepath).to_owned()
}

///////////////////////////////: command keywords functions  :///////////////////////////////

/// Function to read a line of the directory file
/// # Arguments
/// * `rdline` - The line of the directory file
/// * `args` - The arguments of the command
/// * `success` - A mutable reference to a boolean to indicate if the command was successful
/// * `incr` - The increment value
/// * `sstate` - A mutable reference to the state of the search
/// # Returns
/// The line of the directory file
/// # Example
/// ```
/// let mut success = false;
/// let mut sstate = SearchState::default();
/// let rdline = "/home/user/folder;f;folder;1";
/// let args = vec!["get".to_string(), "f".to_string()];
/// let line = read_dline(&rdline, &args, &mut success, 1, &mut sstate);
/// ```
/// # Panics
/// If the command is invalid
/// 
#[rustfmt::skip]
fn read_dline( rdline: &str, args: &[Cmd], success: &mut bool, incr: u32, sstate: &mut SearchState ) -> String {

    if *success {
        if rdline.trim().is_empty() {
            return String::new();
        } 
        return format!("{rdline}\n");
    }
    let vecline: Vec<&str> = rdline.split(';').collect();
    if vecline.len() < 2 {
        assert!(vecline.first().unwrap_or(&"").is_empty(), "Invalid rdline {rdline} found in directory library");
        String::new()
    } else {
        let priory = vecline.last().expect("No priority found in line")
            .parse::<u32>()
            .unwrap_or_else(|err| panic!("priory not in integer in line {rdline} : {err}"));

        let dirline = DirsLine {
            path: vecline.first().expect("No path found in line"),
            shortcs: vecline.get(1..vecline.len().checked_sub(1).expect("Enable to subtract")).expect("Slicing error on line : missing values."),
            priory,
        priory2: priory.checked_add(incr).expect("Overflow on priority"),
        };

        #[rustfmt::skip]
        let line2: String = match args.first().expect("Found empty line in dirs") {
            Cmd::Get([shortc, _]) if shortc.is_none() => get(&dirline, success, sstate, ""),
            Cmd::Get([shortc, _]) => get(&dirline, success, sstate, shortc.as_ref().expect("No shortcut found in get command")),
            Cmd::Reset => format!(
                "{};{}",
                vecline.get(0..vecline.len().checked_sub(1).expect("VLine was empty")).expect("vecline was empty").join(";"),
                0
            ),
            Cmd::Decr(decr) => format!(
                "{};{}",
                vecline.get(0..vecline.len().checked_sub(1).expect("Invalid format of line")).expect("Format of line not valid").join(";"),
                priory.saturating_sub(*decr)),

            Cmd::Rm(shortc) => remove(&dirline, success, shortc),
            Cmd::Del(path) => if *dirline.path == *path { *success = true; return String::new() } 
                                       else { dirline.join(";") },

            Cmd::Add([shortc, path]) => add(&dirline, success, shortc.as_ref().expect("Second argument expected to add").as_str(), &std_path(path)),
            Cmd::Edit([shortc, path]) => edit(&dirline, success, shortc.as_ref().expect("Second argument expected to edit").as_str(), &std_path(path)),

            // _ => panic!("Invalid command : {:?}", args),
        };

        if line2.is_empty() { String::new() }
        else { format!("{line2}\n") }
    }
}

/// Function to read the directory file
/// # Arguments
/// * `dirspath` - The path of the directory file
/// * `args` - The arguments of the command
/// * `incr` - The increment value
/// # Returns
/// The path of the directory
/// # Example
/// ```
/// let dirspath = "/home/user/.dirs";
/// let args = vec!["get".to_string(), "f".to_string()];
/// let path = read(&dirspath, &args, 1);
/// ```
/// # Panics
/// If the file is not found
///
pub fn read(dirspath: &str, args: &[Cmd], incr: u32) -> Option<String> {
    let mut sstate = SearchState::default();
    let mut success = false;

    let mut data: String = fs::read_to_string(dirspath)
        .unwrap_or_else(|er| panic!("Unable to read file: {er}"))
        .split('\n')
        .map(|dline| read_dline(dline.trim(), args, &mut success, incr, &mut sstate))
        .collect();

    let temp = sstate
        .correct
        .clone()
        .unwrap_or_else(|| sstate.prioritised.clone().unwrap_or_default());

    let mut res: String = temp;

    #[rustfmt::skip]
    for arg in args { match arg {
        Cmd::Get([shortc, path]) => {
            if success || shortc.is_none() {
                res = format!("{}/{}", res, path.clone().unwrap_or_default());
            } else {
                panic!("Shortcut {} not found.", shortc.as_ref().expect("No shortcut found in get command"));
            };
            return Some(res);
        }

        _ if success => (),
        Cmd::Reset | Cmd::Decr(_) => (),
        Cmd::Add([shortc, path]) => write!(
            data,
            "{};{};0",
            std_path(path),
            shortc.as_ref().expect("Missing shortcut to add")
        )
        .unwrap_or_else(|er| panic!("Unable to add line to data: {er}")),

        Cmd::Edit(_) | Cmd::Rm(_) | Cmd::Del(_) => panic!("Invalid command line: {args:?}"),
    };
}
    fs::write(dirspath, data).unwrap_or_else(|er| panic!("Unable to write file: {er}"));

    None
}

/// Function to print state of the directories
/// # Arguments
/// * `dirspath` - The path of the directory file
/// # Returns
/// `None`
/// # Example
/// ```
/// let dirspath = "/home/user/.dirs";
/// let state = state(&dirspath);
/// ```
/// # Panics
/// If the file is not found
/// # Note
/// The text will be printed in the following format:
/// ```text
/// afirstpath  shortcut1      shortcut2 14
/// asecondpath short1         short2    14
/// third       afirstshortcut           14
/// ```
///
pub fn state(dirspath: &str) -> ! {
    let mut spaces: Vec<usize> = vec![];
    let binding = fs::read_to_string(dirspath)
        .unwrap_or_else(|er| panic!("!State! Enable to read file! {er}"));
    let data = binding.lines().collect::<Vec<&str>>();
    for dline in &data {
        dline.split(';').enumerate().for_each(|(idx, elt)| {
            let new = elt.len().checked_add(1).expect("Overflow on space");
            match spaces.get(idx) {
                Some(space) if new > *space => {
                    *spaces.get_mut(idx).expect("Expected tab size") = new;
                }
                Some(_) => (),
                None => spaces.push(new),
            };
        });
    }
    spaces.pop();
    let total_space = spaces.iter().sum::<usize>();
    let mut state = String::new();
    for dline in &data {
        let mut sline = dline.split(';').collect::<Vec<&str>>();
        let priory = sline.pop().expect("Empty line found in dirs");
        let mut str1 = String::new();
        sline.iter().enumerate().for_each(|(idx, elt)| {
            let space = spaces.get(idx).unwrap_or(&0);
            write!(&mut str1, "{elt:<space$}")
                .unwrap_or_else(|er| panic!("Unable to write to string: {er}"));
        });
        writeln!(state, "{str1:<total_space$}{priory}")
            .unwrap_or_else(|er| panic!("Unable to write to string: {er}"));
    }

    #[allow(clippy::print_stdout)]
    {
        print!("{state}");
    };
    #[allow(clippy::exit)]
    process::exit(0);
}
