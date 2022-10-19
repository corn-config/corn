use std::fmt::{self, Debug, Display, Formatter};

use crate::Rule;

#[derive(Debug)]
pub struct InputResolveError(pub String);
#[derive(Debug)]
pub struct FileReadError(pub String);
#[derive(Debug)]
pub struct SerializationError(pub String);

#[derive(Debug)]
pub enum Error {
    /// Error while parsing the file
    ParserError(pest::error::Error<Rule>),
    /// Error while looking up a referenced an input
    InputResolveError(InputResolveError),
}

impl std::error::Error for Error {}

impl From<InputResolveError> for Error {
    fn from(err: InputResolveError) -> Self {
        Self::InputResolveError(err)
    }
}

impl From<pest::error::Error<Rule>> for Error {
    fn from(err: pest::error::Error<Rule>) -> Self {
        Self::ParserError(err)
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Error::ParserError(_) => writeln!(f, "An error while parsing the input file."),
            Error::InputResolveError(err) => {
                write!(f, "Input `{}` was used but not declared", err.0)
            }
        }
    }
}

pub type Result<T> = std::result::Result<T, Error>;
