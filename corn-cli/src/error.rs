use colored::*;
use libcorn::error::{Error as CornError, FileReadError, InputResolveError, SerializationError};
use libcorn::Rule;
use pest::error::{ErrorVariant, LineColLocation};
use std::fmt::{Display, Formatter};
use std::io;
use std::path::Path;

#[derive(Debug)]
pub enum Error {
    /// Error from the Corn parser
    Corn(CornError),
    /// Error while reading the input file from disk
    ReadingFile(io::Error),
    /// Error when serializing output
    Serializing(String),
}

type PestError = pest::error::Error<Rule>;

pub trait ExitCode {
    const EXIT_CODE: i32;
}

impl ExitCode for pest::error::Error<Rule> {
    const EXIT_CODE: i32 = 1;
}

impl ExitCode for InputResolveError {
    const EXIT_CODE: i32 = 2;
}

impl ExitCode for FileReadError {
    const EXIT_CODE: i32 = 3;
}

impl ExitCode for SerializationError {
    const EXIT_CODE: i32 = 4;
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
            Error::Corn(err) => write!(f, "{}", err),
            Error::ReadingFile(err) => write!(f, "{}", err),
            Error::Serializing(err) => write!(
                f,
                "The input could not be serialized into the requested output format:\n\t{}",
                err
            ),
        }
    }
}

impl Error {
    pub fn get_exit_code(&self) -> i32 {
        match self {
            Error::Corn(CornError::ParserError(_)) => pest::error::Error::EXIT_CODE,
            Error::Corn(CornError::InputResolveError(_)) => pest::error::Error::EXIT_CODE,
            Error::ReadingFile(_) => FileReadError::EXIT_CODE,
            Error::Serializing(_) => SerializationError::EXIT_CODE,
        }
    }
}

/// Pretty-prints `message` to `stderr`.
/// If `context` is supplied,
/// it will be appended to the first line.
pub fn print_err(message: String, context: Option<String>) {
    if let Some(context) = context {
        eprintln!("{} {}:", "An error occurred".red(), context.red());
    } else {
        eprintln!("{}", "An error occurred:".red());
    }

    eprintln!("\t{}", message.red().bold());
}

/// Pretty prints a parser error,
/// indicating where in the corn-cli source code the error occurred
/// and the rules the parser expected in that position.
///
/// The output is designed to mimic the Rust compiler output.
pub fn format_parser_err(error: PestError, file: String, path: &Path) -> String {
    let message = match error.variant {
        ErrorVariant::ParsingError {
            positives,
            negatives: _negatives,
        } => {
            format!(
                "Expected one of:\n\t{}",
                positives
                    .iter()
                    .map(|rule| rule.to_string())
                    .collect::<Vec<String>>()
                    .join(", ")
            )
        }
        ErrorVariant::CustomError { message } => message,
    }
    .red()
    .bold();

    let pos: ((usize, usize), (usize, usize)) = match error.line_col {
        LineColLocation::Pos((row, col)) => ((row, col), (row, col)),
        LineColLocation::Span((row1, col1), (row2, col2)) => ((row1, col1), (row2, col2)),
    };

    let line = file.lines().nth(pos.0 .0 - 1).unwrap();

    let underline: String = (0..line.len())
        .map(|i| {
            if i >= pos.0 .1 - 1 && i < pos.1 .1 {
                '^'
            } else {
                ' '
            }
        })
        .collect();

    let bar = "  | ".blue();

    format!(
        "--> {path}:{start_pos}:{end_pos}\n\
        {bar}\n\
        {bar}{line}\n\
        {bar}{underline}\n\
        {bar}\n\
        {message}",
        bar = bar,
        path = path.display(),
        start_pos = pos.0 .0,
        end_pos = pos.0 .1,
        line = line,
        underline = underline,
        message = message
    )
}
