use crate::global::{ReadError, SingleError, WriteError};
use std::fs;
use std::process;
use std::time;
// Imports

/// Push a directory to the history file.
/// # Arguments
/// * `histpath` - The path to the history file
/// * `path` - The path to the directory to push
/// # Example
/// ```
/// pushd("lib/hist.csv", "/home/user/folder");
/// ```
/// # Note
/// The history file is a simple text file with the following format:
/// ```text
/// /home/user/folder1;pid1;time1
/// /home/user/folder2;pid2;time2
/// ```
/// Where `pid` is the process id of the process that pushed the directory and `time` is the time in seconds since the Unix Epoch.
/// The last line is the most recent directory pushed.
/// 
#[rustfmt::skip]
pub fn pushd(histpath: &str, path: &str) {
    fs::write(histpath,format!("{}\n{path};{};{}",
        fs::read_to_string(histpath).read_error(histpath, None).trim(),
        process::id(),
        time::SystemTime::now().duration_since(time::UNIX_EPOCH).internal_error("Time went backwards.", None).as_secs(),
    ))
    .write_error(histpath);
}

/// Pop a line from the history file.
/// # Arguments
/// * `lines` - The lines of the history file
/// # Returns
/// The last non empty line.
/// # Example
/// ```rust
/// let lines = vec!["/path1/folder1/;pid1;time1".to_owned(),"/path2/folder2;pid2;time2".to_owned()];
/// assert_eq!(popline(&mut lines),Some("/path2/folder2".to_owned()));
///
/// let lines2 = vec!["/path/folder/pid/time".to_owned(),String::new()];
/// assert_eq!(popline(&mut lines2), Some("/path/folder".to_owned()));
/// ```
/// # Note
/// The function calls itself recursively until it finds a non empty line.
/// If all the lines are empty, the function returns `None`.
fn popline(lines: &mut Vec<String>) -> Option<String> {
    lines.pop().and_then(|lastl| {
        lastl
            .split(';')
            .next()
            .map_or_else(|| popline(lines), |path| Some(path.to_owned()))
    })
}

/// Pop a directory from the history file.
/// # Arguments
/// * `histpath` - The path to the history file
/// # Returns
/// The path of the directory popped.
/// # Errors
/// Raises a warning if the history file is empty.
/// # Example
/// ```
/// let path = popd("lib/hist.csv");
/// ```
/// # Note
/// The history file is a simple text file with the following format:
/// ```text
/// /home/user/folder1;pid1;time1
/// /home/user/folder2;pid2;time2
/// ```
/// Where `pid` is the process id of the process that pushed the directory and `time` is the time in seconds since the Unix Epoch.
/// The last line is the most recent directory pushed.
///
pub fn popd(histpath: &str) -> Option<String> {
    let mut lines = fs::read_to_string(histpath)
        .read_error(histpath, None)
        .split('\n')
        .map(|line| {
            if line.trim().is_empty() {
                String::new()
            } else {
                String::from(line)
            }
        })
        .collect::<Vec<String>>();

    let path = popline(&mut lines);

    #[rustfmt::skip]
    fs::write(histpath, lines.join("\n"))
    .write_error(histpath);
    path
}
