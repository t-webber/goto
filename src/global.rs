use core::fmt;
use core::mem;

/// Macro to print debug information, only if the feature `debug-print` is activated.
#[macro_export]
macro_rules! dbg_print {
  ($($arg:tt)*) => {
    #[cfg(feature = "debug-print")]
    eprint!($($arg)*);
  };
}

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

/// Contains the shortcut and the path.
/// Is used to store them and to pass them to a Cmd element.
#[derive(Debug, Default)]
pub struct ShortPath {
    /// Shortcut entered by the user.
    pub short: Option<String>,
    /// Path to the directory, or subpath of the directory corresponding to the shortcut.
    pub path: Option<String>,
}

/// `enum` to store the command to execute, and its arguments
/// # Examples
/// ```
/// let cmd = Cmd::Get([Some(String::from("short")), Some(String::from("path"))]);
/// ```
/// # Note
/// The command is used to get the path of a directory, to add a directory to the file of supported shortcuts, to edit the path of a directory, to remove a directory from the file of supported shortcuts, to delete a directory from the file of supported shortcuts, to decrement the usage of all directories, or to reset the usage of all directories to 0.
#[derive(Debug)]
pub enum Cmd {
    /// Get the path of a directory.
    Get(ShortPath),
    /// Add a directory to the file of supported shortcuts.
    Add(ShortPath),
    /// Edit the path of a directory.
    Edit(ShortPath),
    /// Remove a directory from the file of supported shortcuts.
    Rm(String),
    /// Delete a directory from the file of supported shortcuts.
    Del(String),
    /// Decrement the usage of all directories.
    Decr(u32),
    /// Reset the usage of all directories to 0.
    Reset,
}

impl fmt::Display for Cmd {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        let val = match self {
            Self::Get(ShortPath { short, path }) => {
                format!(
                    "<goto {} {}>",
                    &short.clone().unwrap_or_default(),
                    &path.clone().unwrap_or_default()
                )
            }
            Self::Add(ShortPath { short, path }) => {
                format!(
                    "<add {} {}>",
                    &short.clone().unwrap_or_default(),
                    &path.clone().unwrap_or_default()
                )
            }
            Self::Edit(ShortPath { short, path }) => {
                format!(
                    "<edit {} {}>",
                    &short.clone().unwrap_or_default(),
                    &path.clone().unwrap_or_default()
                )
            }
            Self::Rm(short) => format!("<rm {short}>"),
            Self::Del(path) => format!("<del {path}>"),
            Self::Decr(val) => format!("<decr {val}>"),
            Self::Reset => "<reset>".to_owned(),
        };
        write!(fmt, "{val}")
    }
}

impl Default for Cmd {
    fn default() -> Self {
        Self::Get(ShortPath::default())
    }
}

impl Cmd {
    /// Implement the `append` method for the `Cmd` enum.
    /// # Arguments
    /// * `value` - The value to append to the command.
    /// # Panics
    /// If the command is already full of all its arguments.
    /// # Examples
    /// ```
    /// let mut cmd = Cmd::Get([None, None]);
    /// cmd.append(String::from("short"));
    /// cmd.append(String::from("path"));
    /// ```
    ///
    pub fn append(&mut self, value: String) {
        #[rustfmt::skip]
        match mem::take(self) {
            Self::Reset => user_error!("The <-reset> option takes no arguments."),
            Self::Decr(0) => {
                *self = Self::Decr(value.parse::<u32>().unwrap_or_else(|_| {
                    user_error!("The value of <-decrement> must be an integer.");
                    0
                }));
            }

            Self::Get(ShortPath{short: None, ..}) => *self = Self::Get(ShortPath{short: Some(value), path: None}),
            Self::Get(ShortPath{short,path: None}) => *self = Self::Get(ShortPath{short, path: Some(value)}),

            Self::Add(ShortPath{short: None, ..}) => *self = Self::Add(ShortPath{short: Some(value), path: None}),
            Self::Add(ShortPath{short, path: None}) => *self = Self::Add(ShortPath{short, path: Some(value)}),

            Self::Edit(ShortPath{short: None, ..}) => *self = Self::Edit(ShortPath{short: Some(value), path: None}),
            Self::Edit(ShortPath{short, path: None}) => *self = Self::Edit(ShortPath{short, path: Some(value)}),

            Self::Rm(st) if st.is_empty() => *self = Self::Rm(value),
            Self::Del(st) if st.is_empty() => *self = Self::Del(value),

            Self::Get(_)
            | Self::Edit(_)
            | Self::Add(_)
            | Self::Rm(_)
            | Self::Del(_)
            | Self::Decr(_) => {
                user_error!("Too many arguments for <{self}> command.");
            }
        }
    }
}

/// Trait to convert to a command.
pub trait ToCmd {
    /// Lone methode of the trait.
    fn to_cmd(&self) -> Cmd;
}

impl ToCmd for str {
    fn to_cmd(&self) -> Cmd {
        #[allow(clippy::print_stderr)]
        match self {
            "-get" => Cmd::Get(ShortPath::default()),
            "-add" => Cmd::Add(ShortPath::default()),
            "-edit" => Cmd::Edit(ShortPath::default()),
            "-remove" => Cmd::Rm(String::new()),
            "-reset" => Cmd::Reset,
            "-delete" => Cmd::Del(String::new()),
            "-decrement" => Cmd::Decr(0),
            _ => {
                eprintln!("[Internal error] Trying to convert <{self}> to valid command.");
                Cmd::default()
            }
        }
    }
}

impl ToCmd for String {
    fn to_cmd(&self) -> Cmd {
        self.as_str().to_cmd()
    }
}

// /// Trait to convert to a reference of a command
// /// in order to match with exact pattern.
// pub enum RefCmd<'refcmd> {
//     /// Get the path of a directory.
//     Get([Option<&'refcmd String>; 2]),
//     /// Add a directory to the file of supported shortcuts.
//     Add([Option<&'refcmd String>; 2]),
//     /// Edit the path of a directory.
//     Edit([Option<&'refcmd String>; 2]),
//     /// Remove a directory from the file of supported shortcuts.
//     Rm(&'refcmd String),
//     /// Delete a directory from the file of supported shortcuts.
//     Del(&'refcmd String),
//     /// Decrement the usage of all directories.
//     Decr(&'refcmd u32),
//     /// Reset the usage of all directories to 0.
//     Reset,
// }

// pub enum VarLenInner<'innerslice> {
//     None,
//     One(Option<&'innerslice String>),
//     Two(InnerSlice<'innerslice>),
// }

// pub struct InnerSlice<'innerslice>([Option<&'innerslice String>; 2]);

// // pub struct OuterSlice([Option<String>]);

// impl<'innerslice> AsRef<VarLenInner<'innerslice>> for Vec<Option<String>> {
//     fn as_ref(&self) -> &VarLenInner<'innerslice> {
//         match self.as_slice() {
//             [a] => VarLenInner::<'innerslice>::One(a.as_ref::<'innerslice>()),
//             [a, b] => VarLenInner::Two(InnerSlice([a.as_ref(), b.as_ref()])),
//             _ => {
//                 eprintln!("Overflow in slice vec conversion");
//                 VarLenInner::None
//             }
//         }
//     }
// }

// impl<'innerslice> VarLenInner<'innerslice> {
//     fn to_2slice(self) -> [Option<&'innerslice String>; 2] {
//         match self {
//             VarLenInner::Two(InnerSlice([a, b])) => [a, b],
//             _ => {
//                 eprint!("[Internal Error] Casting VarLenInner with missing arguments to a length 2 slice.");
//                 [None, None]
//             }
//         }
//     }
// }

// impl<'refcmd> AsRef<RefCmd<'refcmd>> for Cmd {
//     fn as_ref(&self) -> &RefCmd<'refcmd> {
//         match self {
//             Cmd::Get(v) => {
//                 let x: &VarLenInner = v.to_vec().as_ref();
//                 &RefCmd::Get(x.to_2slice())
//             }
//             Cmd::Add(v) => {
//                 let x: &VarLenInner = v.to_vec().as_ref();
//                 &RefCmd::Add(x.to_2slice())
//             }
//             Cmd::Edit(v) => {
//                 let x: &VarLenInner = v.to_vec().as_ref();
//                 &RefCmd::Edit(x.to_2slice())
//             }
//             Cmd::Rm(s) => &RefCmd::Rm(s),
//             Cmd::Del(s) => &RefCmd::Del(s),
//             Cmd::Decr(n) => &RefCmd::Decr(n),
//             Cmd::Reset => &RefCmd::Reset,
//         }
//     }
// }
