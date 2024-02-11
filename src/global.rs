use core::fmt;
use core::mem;
use core::panic;

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
    let msg = format!($($arg)*);
    eprint!("User error: {}", msg);
  };
}

// trait ExpctErr {
//     fn expcterr(&self, msg: &str, args: std::fmt::Arguments);
// }

// impl<T, E> ExpctErr for Result<T, E>
// where
//     E: std::fmt::Debug,
// {
//     /// Print the error message if the result is an error.
//     fn expcterr(&self, msg: &str, args: std::fmt::Arguments) {
//         self.unwrap_or_else(|err| panic!("{}: {:?}.", format!(msg, args), err));
//     }
// }

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
    Get([Option<String>; 2]),
    /// Add a directory to the file of supported shortcuts.
    Add([Option<String>; 2]),
    /// Edit the path of a directory.
    Edit([Option<String>; 2]),
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
            Self::Get([short, path]) => format!(
                "*goto {} {}*",
                &short.clone().unwrap_or_default(),
                &path.clone().unwrap_or_default()
            ),
            Self::Add([short, path]) => format!(
                "*add {} {}*",
                &short.clone().unwrap_or_default(),
                &path.clone().unwrap_or_default()
            ),
            Self::Edit([short, path]) => format!(
                "*edit {} {}*",
                &short.clone().unwrap_or_default(),
                &path.clone().unwrap_or_default()
            ),
            Self::Rm(short) => format!("*rm {short}*"),
            Self::Del(path) => format!("*del {path}*"),
            Self::Decr(val) => format!("*decr {val}*"),
            Self::Reset => "Reset".to_owned(),
        };
        write!(fmt, "{val}")
    }
}

impl Default for Cmd {
    fn default() -> Self {
        Self::Get([None, None])
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
        match mem::take(self) {
            Self::Reset => panic!("Can't append to reset command."),
            Self::Decr(0) => {
                *self = Self::Decr(value.parse::<u32>().expect("Couldn't parse integer value."));
            }

            Self::Get([None, _]) => *self = Self::Get([Some(value), None]),
            Self::Get([shortc, None]) => *self = Self::Get([shortc, Some(value)]),

            Self::Add([None, _]) => *self = Self::Add([Some(value), None]),
            Self::Add([shortc, None]) => *self = Self::Add([shortc, Some(value)]),

            Self::Edit([None, _]) => *self = Self::Edit([Some(value), None]),
            Self::Edit([shortc, None]) => *self = Self::Edit([shortc, Some(value)]),

            Self::Rm(st) if st.is_empty() => *self = Self::Rm(value),
            Self::Del(st) if st.is_empty() => *self = Self::Del(value),

            Self::Get(_)
            | Self::Edit(_)
            | Self::Add(_)
            | Self::Rm(_)
            | Self::Del(_)
            | Self::Decr(_) => {
                panic!("Too many arguments for {} command.", self)
            }
        }
    }
}

/// Trait to convert to a command.
pub trait ToCmd {
    /// Lone methode of the trait.
    fn to_cmd(&self) -> Result<Cmd, String>;
}

impl ToCmd for str {
    fn to_cmd(&self) -> Result<Cmd, String> {
        match self {
            "-get" => Ok(Cmd::Get([None, None])),
            "-add" => Ok(Cmd::Add([None, None])),
            "-edit" => Ok(Cmd::Edit([None, None])),
            "-remove" => Ok(Cmd::Rm(String::new())),
            "-reset" => Ok(Cmd::Reset),
            "-delete" => Ok(Cmd::Del(String::new())),
            "-decrement" => Ok(Cmd::Decr(0)),
            _ => Err(format!("Invalid command <{self}>.")),
        }
    }
}

impl ToCmd for String {
    fn to_cmd(&self) -> Result<Cmd, String> {
        self.as_str().to_cmd()
    }
}
