use core::panic;
use std::fmt::{Display, Formatter};

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

#[derive(Debug)]
pub enum Cmd {
    Get([Option<String>; 2]),
    Add([Option<String>; 2]),
    Edit([Option<String>; 2]),
    Rm(String),
    Del(String),
    Decr(u32),
    Reset,
}

impl Display for Cmd {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
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
            Self::Reset => "Reset".to_string(),
        };
        write!(f, "{val}")
    }
}

impl Default for Cmd {
    fn default() -> Self {
        Self::Get([None, None])
    }
}

impl Cmd {
    pub fn append(&mut self, value: String) {
        match self {
            Self::Reset => panic!("Can't append to reset command."),
            Self::Decr(0) => {
                *self = Self::Decr(value.parse::<u32>().expect("Couldn't parse integer value."));
            }

            Self::Decr(_) => panic!("Too many arguments for decrement command."),

            Self::Get([None, _]) => *self = Self::Get([Some(value), None]),
            Self::Get([shortc, None]) => *self = Self::Get([shortc.to_owned(), Some(value)]),

            Self::Add([None, _]) => *self = Self::Add([Some(value), None]),
            Self::Add([shortc, None]) => *self = Self::Add([shortc.to_owned(), Some(value)]),

            Self::Edit([None, _]) => *self = Self::Edit([Some(value), None]),
            Self::Edit([shortc, None]) => *self = Self::Edit([shortc.to_owned(), Some(value)]),

            Self::Rm(s) | Self::Del(s) if s.is_empty() => *self = Self::Del(value),

            Self::Get(_) | Self::Edit(_) | Self::Add(_) | Self::Rm(_) | Self::Del(_) => {
                panic!("Too many arguments for {} command.", self.to_string())
            }
        }
    }
}

pub trait ToCmd {
    fn to_cmd(&self) -> Result<Cmd, String>;
}

impl ToCmd for str {
    fn to_cmd(&self) -> Result<Cmd, String> {
        match self {
            "-get" => Ok(Cmd::Get([None, None])),
            "-add" => Ok(Cmd::Add([None, None])),
            "-edit" => Ok(Cmd::Edit([None, None])),
            "-remove" => Ok(Cmd::Rm(String::new())),
            "-decr" => Ok(Cmd::Decr(0)),
            "-reset" => Ok(Cmd::Reset),
            "-del" => Ok(Cmd::Del(String::new())),
            _ => Err(format!("Invalid command <{self}>.")),
        }
    }
}

impl ToCmd for String {
    fn to_cmd(&self) -> Result<Cmd, String> {
        self.as_str().to_cmd()
    }
}
