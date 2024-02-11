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
        fs::read_to_string(histpath).unwrap_or_else(|er| panic!("Unable to read hist file: {er}")).trim(),
        process::id(),
        time::SystemTime::now().duration_since(time::UNIX_EPOCH).expect("Time went backwards").as_secs(),
    ))
    .unwrap_or_else(|er| panic!("Unable to write hist file {er}"));
}

/// Pop a directory from the history file.
/// # Arguments
/// * `histpath` - The path to the history file
/// # Returns
/// The path of the directory popped.
/// # Panics
/// If the history file is empty.
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
pub fn popd(histpath: &str) -> String {
    let mut lines = fs::read_to_string(histpath)
        .unwrap_or_else(|er| panic!("Unable to read file {er}"))
        .split('\n')
        .map(|line| {
            if line.trim().is_empty() {
                String::new()
            } else {
                String::from(line)
            }
        })
        .collect::<Vec<String>>();

    let lastl = lines.pop().expect("No folders in history");
    let path = lastl.split(';').next().expect("Empty Line").to_owned();

    match fs::write(histpath, lines.join("\n")) {
        Err(er) => panic!("Unable to write file {er}"),
        Ok(()) => path,
    }
}
