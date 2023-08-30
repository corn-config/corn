use std::fmt::{Debug, Display};
use thiserror::Error;

use crate::Rule;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Error, Debug)]
pub enum Error {
    #[error(transparent)]
    Io(#[from] std::io::Error),

    #[error(transparent)]
    ParserError(#[from] Box<pest::error::Error<Rule>>),

    #[error("failed to resolve referenced input `{0}`")]
    InputResolveError(String),

    #[error("attempted to use dot-notation on non-object value at `{0}`")]
    InvalidPathError(String),

    #[error("attempted to spread a type that differs from its containing type at `{0}`")]
    InvalidSpreadError(String),

    #[error("attempted to interpolate a non-string type into a string at `{0}`")]
    InvalidInterpolationError(String),

    #[error("failed to deserialize input: {0}")]
    DeserializationError(String),
}

impl serde::de::Error for Error {
    fn custom<T>(msg: T) -> Self
    where
        T: Display,
    {
        Error::DeserializationError(msg.to_string())
    }
}
