use core::fmt::Write;
use std::fs;
use std::process;

use crate::commands::{Cmd, ShortPath};
use crate::errors::{ReadError, SingleError, WriteError};
use crate::{general_error, user_error};

/// Trait to read a vector of a line of the directory file
/// Enables to get path, shortuts, priority from a dline
trait ReadVec {
    /// Get the path of the directory
    fn join_elts(&self, deb: usize, offset: usize, msgf: &str) -> String;
}

impl ReadVec for Vec<&str> {
    fn join_elts(&self, deb: usize, offset: usize, msg: &str) -> String {
        let x = self
            .get(deb..self.len().checked_sub(offset).data_error(msg, None))
            .data_error(msg, None);
        x.join(";")
    }
}

/// Structure to contain the state of the search
#[derive(Default, Debug)]
struct SearchState {
    /// If a path was found for the given shortut, `correct` contains the path.
    correct: Option<String>,
    /// If a path was not found for the given shortut, `prioritised` contains the path with the highest priority.
    prioritised: Option<String>,
    /// The highest priority found.
    max_priory: u32,
}

/// Structure to contain the data of a line in the directory file
#[derive(Debug)]
struct DirsLine<'dirline> {
    /// The path of the directory
    path: &'dirline str,
    /// The shortuts of the directory
    shorts: &'dirline [&'dirline str],
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
            self.shorts.join(sep),
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
/// * `short` - The shortut to search for
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
///    shorts: &["f", "folder"],
///    priory: 1,
///    priory2: 2,
/// };
/// let path = get(&dirline, &mut success, &mut sstate, "f");
/// ```
/// # Panics
/// If the shortut is not found in the line
/// # Note
/// The directory file is a simple text file with the following format:
/// ```text
/// /home/user/folder;f;folder;1
/// /home/user/folder2;f2;folder2;2
/// ```
/// Where the first field is the path of the directory, the second field is the shortuts of the directory, the third field is the priority of the directory.
/// The last line is the most recent directory pushed.
/// 
#[rustfmt::skip]
fn get(dirline: &DirsLine, success: &mut bool, sstate: &mut SearchState, short: &str) -> String {
    if dirline.shorts.contains(&short) {
        sstate.correct = Some(String::from(dirline.path));
        *success = true;
        format!("{};{};{}", dirline.path, dirline.shorts.join(";"), dirline.priory2)
    } else {
        if dirline.priory > sstate.max_priory {
            sstate.max_priory = dirline.priory;
            sstate.prioritised = Some(String::from(dirline.path));
        }
        dirline.join(";")
    }
}

/// Function to remove a shortut from a line of the directory file
/// # Arguments
/// * `dirline` - The line of the directory file
/// * `success` - A mutable reference to a boolean to indicate if the shortut was found
/// * `short` - The shortut to remove
/// # Returns
/// The line of the directory file
/// # Example
/// ```
/// let mut success = false;
/// let dirline = DirsLine {
///   path: "/home/user/folder",
///  shorts: &["f", "folder"],
/// priory: 1,
/// priory2: 2,
/// };
/// let line = remove(&dirline, &mut success, "f");
/// ```
/// # Panics
/// If the shortut is not found in the line
/// # Note
///
fn remove(dirline: &DirsLine, success: &mut bool, short: &str) -> String {
    if dirline.shorts.contains(&short) {
        *success = true;
        if dirline.shorts.len() == 1 {
            return String::new();
        }
        format!(
            "{};{};{}",
            dirline.path,
            dirline
                .shorts
                .iter()
                .filter(|&&sh| sh != short)
                .copied()
                .collect::<Vec<_>>()
                .join(";"),
            dirline.priory
        )
    } else {
        dirline.join(";")
    }
}

/// Function to add a shortut to a line of the directory file
/// # Arguments
/// * `dirline` - The line of the directory file
/// * `success` - A mutable reference to a boolean to indicate if the shortut was added
/// * `new_short` - The new shortut to add
/// * `path` - The path of the directory
/// # Returns
/// The line of the directory file
/// # Example
/// ```
/// let mut success = false;
/// let dirline = DirsLine {
///   path: "/home/user/folder",
///   shorts: &["f", "folder"],
///   priory: 1,
///   priory2: 2,
/// };
/// let line = add(&dirline, &mut success, "f2", "/home/user/folder");
/// ```
/// # Panics
/// If the shortut already exists
fn add(dirline: &DirsLine, success: &mut bool, new_short: &str, path: &str) -> String {
    if dirline.shorts.contains(&new_short) {
        user_error!("Shortcut {new_short} already exists");
        *success = true;
        dirline.join(";")
    } else if path == dirline.path {
        *success = true;
        if dirline.shorts.contains(&new_short) {
            user_error!("Shortcut {new_short} already exists");
            dirline.join(";")
        } else {
            format!(
                "{path};{};{new_short};{}",
                dirline.shorts.join(";"),
                dirline.priory2
            )
        }
    } else {
        dirline.join(";")
    }
}

/// # Arguments
/// * `dirline` - The line of the directory file
/// * `success` - A mutable reference to a boolean to indicate if the shortut was edited
/// * `short` - The new shortut
/// * `path` - The path of the directory
/// # Returns
/// The line of the directory file
/// # Example
/// ```
/// let mut success = false;
/// let dirline = DirsLine {
///   path: "/home/user/folder",
///   shorts: &["f", "folder"],
///   priory: 1,  
///   priory2: 2,
/// };
/// ```
/// # Panics
/// If the path already exists
///
fn edit(dirline: &DirsLine, success: &mut bool, short: &str, path: &str) -> String {
    if dirline.path == path {
        user_error!("Path already exists");
        *success = true;
        dirline.join(";")
    } else if dirline.shorts.contains(&short) {
        *success = true;
        format!("{path};{};{}", dirline.shorts.join(";"), dirline.priory)
    } else {
        dirline.join(";")
    }
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
        #[allow(clippy::expect_used)]
        let priory = vecline.last().expect("[Data Error] Missing priority in dline.")
            .parse::<u32>()
            .data_error(format!("Priority not an integer in {rdline}"), None);

        let dirline = DirsLine {
            path: if let Some(pth) = vecline.first() { pth } else { return String::new() },
            shorts: vecline.get(1..vecline.len().checked_sub(1).data_error("Unable to subtract to priority", None)).data_error("Missing values in line", None),
            priory,
        priory2: priory.checked_add(incr).internal_error("Overflow on priority", None),
        };

        #[rustfmt::skip]
        let line2 = if let Some(first) = args.first() {
            match first {
            Cmd::Get(ShortPath{short: None, ..}) => get(&dirline, success, sstate, ""),
            Cmd::Get(ShortPath{short: Some(short), ..}) => get(&dirline, success, sstate, short),
            Cmd::Reset => format!(
                "{};{}",
                vecline.join_elts(0, 1, "Missing values in line"),
                0
            ),
            Cmd::Decr(decr) => format!(
                "{};{}",
                vecline.join_elts(0, 1, "Missing values in line"),
                priory.saturating_sub(*decr)),

            Cmd::Rm(short) => remove(&dirline, success, short),
            Cmd::Del(path) if *dirline.path == *path => {*success = true; return String::new() },
            Cmd::Del(_) => dirline.join(";"),

            Cmd::Add(ShortPath{short: None, ..} | ShortPath{path: None, ..})
            | Cmd::Edit(ShortPath{short: None, ..} | ShortPath{path: None, ..})
                => { user_error!("Missing shortut or path to <-add> or <-edit>"); String::new() },

            Cmd::Add(ShortPath{short: Some(short), path: Some(path)}) 
                => add(&dirline, success, short.as_str(), path),
            Cmd::Edit(ShortPath{short: Some(short), path: Some(path)}) 
                => edit(&dirline, success, short.as_str(), path),

        }} else {
            #[allow(clippy::print_stderr)]
            {eprintln!("[Internal Error] No option pushed in argument list.");};
            return String::new();
        };

        if line2.is_empty() { String::new() }
        else { format!("{line2}\n") }
    }
}

/// Function to read the directory file
/// # Arguments
/// * `dpath` - The path of the directory file
/// * `args` - The arguments of the command
/// * `incr` - The increment value
/// # Returns
/// The path of the directory
/// # Example
/// ```
/// let dpath = "/home/user/.dirs";
/// let args = vec!["get".to_string(), "f".to_string()];
/// let path = read(&dpath, &args, 1);
/// ```
/// # Panics
/// If the file is not found
///
pub fn read(dpath: &str, args: &[Cmd], incr: u32) -> Option<String> {
    let mut sstate = SearchState::default();
    let mut success = false;

    let mut data: String = fs::read_to_string(dpath)
        .read_error(dpath, None)
        .split('\n')
        .map(|dline| read_dline(dline.trim(), args, &mut success, incr, &mut sstate))
        .collect();

    let mut res: String = sstate
        .correct
        .unwrap_or_else(|| sstate.prioritised.unwrap_or_default());
    res.push('/');
    let mut some = false;

    #[rustfmt::skip]
    for arg in args { match arg {
        Cmd::Get(ShortPath{short: Some(short), ..}) if !success => { some = true; res = short.to_string()},
        Cmd::Get(ShortPath{path, ..}) => { some = true; res.push_str(path.clone().unwrap_or_default().as_str())},

        _ if success => (),
        Cmd::Reset | Cmd::Decr(_) => (),
        Cmd::Add(ShortPath{path: None, ..}) => user_error!("Missing path to <-add>"),
        Cmd::Edit(ShortPath{path: None, ..}) => user_error!("Shortcut not found, and missing path to <-edit>"),
        Cmd::Add(ShortPath{short: opt_short, path: Some(path)}) | Cmd::Edit(ShortPath{short: opt_short, path: Some(path)}) =>
            match opt_short.as_ref() {
                Some(short) => write!(data, "{};{};0", &path, short).write_error("Lines"),
                None => user_error!("Missing shortut to add"),
            },

        Cmd::Rm(_) => user_error!("Failed to remove shortcut: not found"),
        Cmd::Del(_) => user_error!("Failed to delete path: not found"),
        };
    }

    fs::write(dpath, data).write_error(dpath);
    some.then_some(res)
}

/// Function to print state of the directories
/// # Arguments
/// * `dpath` - The path of the directory file
/// # Returns
/// `None`
/// # Panics
/// If the file is not found
/// # Note
/// The text will be printed in the following format:
/// ```text
/// afirstpath  shortut1      shortut2 14
/// asecondpath short1         short2    14
/// third       afirstshortut           14
/// ```
///
pub fn state(dpath: &str) -> ! {
    let mut spaces: Vec<usize> = vec![];
    let binding = fs::read_to_string(dpath).read_error(dpath, None);
    let data = binding.lines().collect::<Vec<&str>>();
    for dline in &data {
        dline.split(';').enumerate().for_each(|(idx, elt)| {
            let new = elt.len().checked_add(1).unwrap_or(elt.len());
            match spaces.get_mut(idx) {
                Some(space) if new > *space => {
                    *space = new;
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
        match sline.pop() {
            None => continue,
            Some(priory) => {
                let mut str1 = String::new();
                sline.iter().enumerate().for_each(|(idx, elt)| {
                    let space = spaces.get(idx).unwrap_or(&0);
                    write!(&mut str1, "{elt:<space$}").write_error("line");
                });
                writeln!(state, "{str1:<total_space$}{priory}").write_error("lines");
            }
        }
    }

    #[allow(clippy::print_stdout)]
    {
        print!("{state}");
    };
    #[allow(clippy::exit)]
    process::exit(0);
}
