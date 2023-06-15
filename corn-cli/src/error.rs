use colored::Colorize;
use libcorn::error::Error as CornError;
use std::fmt::{Display, Formatter};
use std::io;

#[derive(Debug)]
pub enum Error {
    /// Error from the Corn parser
    Corn(CornError),
    /// Error while reading the input file from disk
    ReadingFile(io::Error),
    /// Error when serializing output
    Serializing(String),
}

pub trait ExitCode {
    fn get_exit_code(&self) -> i32;
}

impl ExitCode for CornError {
    fn get_exit_code(&self) -> i32 {
        match self {
            CornError::Io(_) => 3,
            CornError::ParserError(_) => 1,
            CornError::InputResolveError(_) => 2,
            CornError::InvalidPathError(_) => 6,
            CornError::InvalidSpreadError(_) => 7,
            CornError::DeserializationError(_) => 5,
        }
    }
}

impl ExitCode for Error {
    fn get_exit_code(&self) -> i32 {
        match self {
            Error::Corn(err) => err.get_exit_code(),
            Error::ReadingFile(_) => 3,
            Error::Serializing(_) => 4,
        }
    }
}

impl std::error::Error for Error {}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Self {
        Self::ReadingFile(err)
    }
}

impl From<serde_json::Error> for Error {
    fn from(err: serde_json::Error) -> Self {
        Self::Serializing(err.to_string())
    }
}

impl From<serde_yaml::Error> for Error {
    fn from(err: serde_yaml::Error) -> Self {
        Self::Serializing(err.to_string())
    }
}

impl From<toml::ser::Error> for Error {
    fn from(err: toml::ser::Error) -> Self {
        Self::Serializing(err.to_string())
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::Corn(err) => write!(f, "{err}"),
            Error::ReadingFile(err) => write!(f, "{err}"),
            Error::Serializing(err) => write!(
                f,
                "The input could not be serialized into the requested output format:\n\t{err}"
            ),
        }
    }
}

/// Pretty-prints `message` to `stderr`.
/// If `context` is supplied,
/// it will be appended to the first line.
pub fn print_err(message: &str, context: Option<String>) {
    if let Some(context) = context {
        eprintln!("{} {}:", "An error occurred".red(), context.red());
    } else {
        eprintln!("{}", "An error occurred:".red());
    }

    eprintln!("\t{}", message.red().bold());
}
