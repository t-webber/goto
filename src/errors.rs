use std::fmt;

/// Macro to print user error.
/// # Examples
/// ```
/// user_error!("Shortcut {shortc} already exists");
/// ```
#[macro_export]
macro_rules! user_error {
  ($($arg:tt)*) => {
    eprint!("[User Error] {}.", format!($($arg)*))
  };
}

/// Trait to print error of opening a file (uses `eprintln!`)
/// # Examples
/// ```
/// let fpath = "file.txt";
/// fs::read_to_string(fpath).file_error(fpath);
///
pub trait ReadError<T> {
    /// Unable to open a file.
    fn read_error(self, fpath: &str, default: Option<T>) -> T;
}

#[allow(clippy::print_stderr)]
impl<T: Default, E: fmt::Display> ReadError<T> for Result<T, E> {
    fn read_error(self, fpath: &str, default: Option<T>) -> T {
        let def = default.unwrap_or_else(|| T::default());
        match self {
            Ok(val) => val,
            Err(er) => {
                eprintln!("[File Error] Unable to read file {fpath}: {er}.");
                def
            }
        }
    }
}

/// Trait to print error of writing in a file (uses `eprintln!`)
/// # Examples
/// ```
/// let fpath = "file.txt";
/// fs::read_to_string(fpath).file_error(fpath);
///
pub trait WriteError<E> {
    /// Unable to write a file.
    fn write_error(self, fpath: &str);
}

#[allow(clippy::print_stderr)]
impl<E: fmt::Display> WriteError<E> for Result<(), E> {
    fn write_error(self, fpath: &str) {
        self.unwrap_or_else(|er| eprintln!("[File Error] Unable to write in file {fpath}: {er}."));
    }
}

/// Trait to print internal errors (uses `panic!`).
/// These errors are logic erros. When they occur, please contact the developer.
pub trait SingleError<T, U> {
    /// Print an internal error.
    fn internal_error(self, msg: U, default: Option<T>) -> T;
    /// Print a data error.
    fn data_error(self, msg: U, default: Option<T>) -> T;
}

#[allow(clippy::print_stderr)]
impl<T: Default, U: fmt::Display> SingleError<T, U> for Option<T> {
    fn internal_error(self, msg: U, default: Self) -> T {
        let def = default.unwrap_or_else(|| T::default());
        self.map_or_else(
            || {
                eprint!("[Internal Error] {msg}");
                def
            },
            |val| val,
        )
    }

    fn data_error(self, msg: U, default: Self) -> T {
        let def = default.unwrap_or_else(|| T::default());
        self.map_or_else(
            || {
                eprint!("[Data Error] {msg}");
                def
            },
            |val| val,
        )
    }
}

#[allow(clippy::print_stderr)]
impl<T: Default, U: fmt::Display, E: fmt::Display> SingleError<T, U> for Result<T, E> {
    fn internal_error(self, msg: U, default: Option<T>) -> T {
        let def = default.unwrap_or_else(|| T::default());
        match self {
            Ok(val) => val,
            Err(er) => {
                eprint!("[Internal Error] {er}.\n{msg}.");
                def
            }
        }
    }

    fn data_error(self, msg: U, default: Option<T>) -> T {
        let def = default.unwrap_or_else(|| T::default());
        match self {
            Ok(val) => val,
            Err(er) => {
                eprint!("[Data Error] {er}.\n{msg}.");
                def
            }
        }
    }
}