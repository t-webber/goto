#![allow(dead_code)]

use core::fmt;

/// Macro to print general errors.
/// # Examples
/// ```
/// general_error!("Shortcut {shortc} already exists");
/// ```
#[macro_export]
macro_rules! general_error {
    ($type:expr, $($arg:tt)*) => {
        eprintln!("\x1b[31m[{}] {}:{}:{}.\n{}.\x1b[0m", $type, file!(), line!(), column!(), format!($($arg)*))
    };
}

/// Macro to print user error.
/// # Examples
/// ```
/// user_error!("Shortcut {shortc} already exists");
/// ```
/// Output:
/// ```shell
/// [User Error] Shortcut {shortc} already exists.
/// ```
#[macro_export]
macro_rules! user_error {
  ($($arg:tt)*) => {
    general_error!("User Error", $($arg)*)
  };
}

/// Macro to print internal error.
#[macro_export]
macro_rules! internal_error {
  ($($arg:tt)*) => {
    general_error!("Internal Error", $($arg)*)
  };
}
/// Macro to print internal error.
#[macro_export]
macro_rules! system_error {
  ($($arg:tt)*) => {
    general_error!("System Error", $($arg)*)
  };
}

/// Macro to print data error.
#[macro_export]
macro_rules! data_error {
  ($($arg:tt)*) => {
    general_error!("Data Error", $($arg)*)
  };
}

/// Macro to print file read/write error.
#[macro_export]
macro_rules! file_error {
  ($($arg:tt)*) => {
    general_error!("File Error", $($arg)*)
  };
}
/// Macro to print command error.
#[macro_export]
macro_rules! command_error {
  ($($arg:tt)*) => {
    general_error!("Command Error", $($arg)*)
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
                file_error!("Unable to read file {fpath}: {er}");
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
        self.unwrap_or_else(|er| file_error!("Unable to write in file {fpath}: {er}"));
    }
}

/// Trait to print internal errors (uses `panic!`).
/// These errors are logic erros. When they occur, please contact the developer.
#[allow(dead_code)]
pub trait SingleError<T, U> {
    /// Print an internal error.
    fn internal_error(self, msg: U, default: Option<T>) -> T;
    /// Print a data error.
    fn data_error(self, msg: U, default: Option<T>) -> T;
    /// Print a user error.
    fn user_error(self, msg: U, default: Option<T>) -> T;
}

#[allow(clippy::print_stderr)]
impl<T: Default, U: fmt::Display> SingleError<T, U> for Option<T> {
    fn internal_error(self, msg: U, default: Self) -> T {
        let def = default.unwrap_or_else(|| T::default());
        self.map_or_else(
            || {
                internal_error!("{msg}");
                def
            },
            |val| val,
        )
    }

    fn data_error(self, msg: U, default: Self) -> T {
        let def = default.unwrap_or_else(|| T::default());
        self.map_or_else(
            || {
                data_error!("{msg}");
                def
            },
            |val| val,
        )
    }

    fn user_error(self, msg: U, default: Self) -> T {
        let def = default.unwrap_or_else(|| T::default());
        self.map_or_else(
            || {
                user_error!("{msg}");
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
                internal_error!("{er}.\n{msg}");
                def
            }
        }
    }

    fn data_error(self, msg: U, default: Option<T>) -> T {
        let def = default.unwrap_or_else(|| T::default());
        match self {
            Ok(val) => val,
            Err(er) => {
                data_error!("{er}.\n{msg}");
                def
            }
        }
    }

    fn user_error(self, msg: U, default: Option<T>) -> T {
        let def = default.unwrap_or_else(|| T::default());
        match self {
            Ok(val) => val,
            Err(er) => {
                user_error!("{er}.\n{msg}");
                def
            }
        }
    }
}

/// Trait to print command errors (uses `eprintln!`).
pub trait InteractionError<T> {
    /// Print a command error.
    fn command_error(self, msg: &str) -> T;
    /// Print a user error.
    fn user_error(self, msg: &str) -> T;
    /// Print a system error.
    fn system_error(self, msg: &str) -> T;
}

impl<T: Default, E: fmt::Display> InteractionError<T> for Result<T, E> {
    #[allow(clippy::print_stderr)]
    fn command_error(self, msg: &str) -> T {
        self.unwrap_or_else(|er| {
            command_error!("{msg}.\n{er}");
            T::default()
        })
    }
    #[allow(clippy::print_stderr)]
    fn user_error(self, msg: &str) -> T {
        self.unwrap_or_else(|er| {
            user_error!("{msg}.\n{er}");
            T::default()
        })
    }
    #[allow(clippy::print_stderr)]
    fn system_error(self, msg: &str) -> T {
        self.unwrap_or_else(|er| {
            system_error!("{msg}.\n{er}");
            T::default()
        })
    }
}

impl<T: Default> InteractionError<T> for Option<T> {
    #[allow(clippy::print_stderr)]
    fn command_error(self, msg: &str) -> T {
        self.unwrap_or_else(|| {
            command_error!("{msg}");
            T::default()
        })
    }
    #[allow(clippy::print_stderr)]
    fn user_error(self, msg: &str) -> T {
        self.unwrap_or_else(|| {
            user_error!("{msg}");
            T::default()
        })
    }
    #[allow(clippy::print_stderr)]
    fn system_error(self, msg: &str) -> T {
        self.unwrap_or_else(|| {
            system_error!("{msg}");
            T::default()
        })
    }
}
