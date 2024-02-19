use crate::{user_error, general_error};
use core::fmt;
use core::mem;
use crate::errors::SingleError;

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
            Self::Get(ShortPath{short,path: None}) => *self = Self::Get(ShortPath{short, path: Some(std_path(&value))}),

            Self::Add(ShortPath{short: None, ..}) => *self = Self::Add(ShortPath{short: Some(value), path: None}),
            Self::Add(ShortPath{short, path: None}) => *self = Self::Add(ShortPath{short, path: Some(std_path(&value))}),

            Self::Edit(ShortPath{short: None, ..}) => *self = Self::Edit(ShortPath{short: Some(value), path: None}),
            Self::Edit(ShortPath{short, path: None}) => *self = Self::Edit(ShortPath{short, path: Some(std_path(&value))}),

            Self::Rm(st) if st.is_empty() => *self = Self::Rm(value),
            Self::Del(st) if st.is_empty() => *self = Self::Del(value),

            Self::Get(_) | Self::Edit(_) | Self::Add(_)
            | Self::Rm(_) | Self::Del(_) | Self::Decr(_) => {
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

/// Function to get the directory from a path.
/// # Arguments
/// * `path` - The path to get the directory from.
/// # Returns
/// The directory.
/// # Examples
/// ```
/// let dir = path2dir("C:/Users/username/Documents");
/// assert_eq!(dir, "Documents");
/// ```
fn path2dir(path: &str) -> String {
    path.split('/')
        .last()
        .internal_error("The path is not valid", None)
        .split('\\')
        .last()
        .internal_error("The path is still not valid", None)
        .to_owned()
}

/// Function to format a path
/// # Arguments
/// * `path` - The path to format
/// # Returns
/// The formatted path
/// # Example   
/// ```
/// assert!(std_path(&Some(String::from("D:/Windows\\PeRso"))) == "d:/Windows/PeRso");
/// ```
///
pub fn std_path(ipath: &str) -> String {
    let mut path = ipath.to_owned().replace('\\', "/");
    
    let here = std::env::current_dir().user_error("Couldn't access current path", None).to_str().user_error("Error while casting current path to str", None).to_owned().replace('\\', "/");
    let last_slash = here.rfind('/').user_error("Expected a slash in the path", None);
    let (father, _) = here.split_at(last_slash);
    
    path = path.replace("..", father);
    match path.chars().next() {
        None => path = here,
        Some('.') => path = father.to_owned() + &path[1..],
        Some('/') => (),
        _ if path.chars().nth(1) == Some(':') => (),
        _ => path = format!("{here}/{path}"),
    }

    let mut to_lower = true; 
    let mut res = path.chars()
        .map(|char| match char {
            '\\' => {
                to_lower = false;
                '/'
            }
            ':' | '/' if to_lower => {
                to_lower = false;
                char
            }
            _ if to_lower => char.to_ascii_lowercase(),
            _ => char,
        })
        .collect::<String>();
    if res.ends_with('/') {
        res.pop();
    }
    res    
}

/// Trait to append a default value to a command.
pub trait AppendDefault {
    /// Lone method of the trait.
    fn append_default(self, value: &str);
}

impl AppendDefault for Option<&mut Cmd> {
    fn append_default(self, value: &str) {
        if let Some(cmd) = self {
            #[rustfmt::skip]
            match cmd {
                Cmd::Add(ShortPath {short: None, path: None})
                | Cmd::Edit(ShortPath {short: None, path: None}) => {
                    cmd.append(path2dir(value)); cmd.append(value.to_owned());
                }
                Cmd::Add(ShortPath { path: None, .. })
                | Cmd::Edit(ShortPath { path: None, .. }) => cmd.append(value.to_owned()),

                Cmd::Get(_) | Cmd::Add(_) | Cmd::Edit(_) 
                | Cmd::Rm(_) | Cmd::Del(_) | Cmd::Decr(_) | Cmd::Reset => (),
            }
        }
    }
}