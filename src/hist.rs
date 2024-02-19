use crate::errors::{ReadError, SingleError, WriteError};
use crate::{data_error, file_error, general_error, user_error};
use std::io::{self, BufRead, BufWriter, Write};
use std::path::Path;
use std::process;
use std::time;
use std::{collections, fs};
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
pub fn pushd(histpath: &str, path: &str) {
    match fs::canonicalize(path) {
        Err(er) => user_error!("{path} doesn't exist: {er}"),
        Ok(absolute_path) if !absolute_path.exists() || !absolute_path.is_dir() => {
            user_error!("{path} is not a directory");
        }
        Ok(absolute_path) => {
            fs::write(
                histpath,
                format!(
                    "{}\n{};{};{}",
                    fs::read_to_string(histpath)
                        .read_error(histpath, None)
                        .trim(),
                    absolute_path.to_string_lossy(),
                    process::id(),
                    time::SystemTime::now()
                        .duration_since(time::UNIX_EPOCH)
                        .internal_error("Time went backwards.", None)
                        .as_secs(),
                ),
            )
            .write_error(histpath);
        }
    }
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
pub fn popd(histpath: &str) -> String {
    let reader = io::BufReader::new(match fs::File::open(histpath) {
        Ok(hist) => hist,
        Err(er) => {
            file_error!("Unable to read {histpath}: {er}");
            return String::new();
        }
    });

    let mut lines = collections::LinkedList::new();

    reader.lines().for_each(|line| match line {
        Ok(line) if !line.trim().is_empty() => {
            match line.split(';').next() {
                Some(current) if Path::new(current).exists() => {
                    lines.push_back(line);
                }
                _ => (),
            };
        }
        Err(er) => data_error!("Unable to read a line in {histpath}: {er}"),
        _ => (),
    });

    let mut writer = BufWriter::new(match fs::File::create(histpath) {
        Ok(hist) => hist,
        Err(er) => {
            file_error!("Unable to write to {histpath}: {er}");
            return String::new();
        }
    });

    lines.pop_back();
    lines.pop_back().map_or_else(
        || {
            file_error!("{histpath} is empty");
            String::new()
        },
        |last_line| {
            for line in lines {
                writeln!(writer, "{line}").write_error(histpath);
            }
            writeln!(writer, "{last_line}").write_error(histpath);
            last_line
                .split(';')
                .next()
                .internal_error("Checked if path was correct, but isn't found", None)
                .to_owned()
        },
    )
}
