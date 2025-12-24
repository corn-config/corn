use alloc::string::{String, ToString};
use core::fmt::{Display, Formatter};

pub type Result<T, E = Error> = core::result::Result<T, E>;

#[derive(Debug)]
pub enum Error {
    InputResolveError(String),
    InvalidSpreadError,
    InvalidInterpolationError,
    DeserializationError(String),
    ParseError(String),
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::InputResolveError(input) => {
                write!(f, "failed to resolve referenced input `{input}`")
            }
            Self::InvalidSpreadError => write!(
                f,
                "attempted to spread a type that differs from its containing type"
            ),
            Self::InvalidInterpolationError => write!(
                f,
                "attempted to interpolate a non-string type into a string"
            ),
            Self::DeserializationError(msg) => write!(f, "failed to deserialize input: {msg}"),
            Self::ParseError(msg) => write!(f, "failed to parse input: {msg}"),
        }
    }
}

impl core::error::Error for Error {}

impl serde_core::de::Error for Error {
    fn custom<T>(msg: T) -> Self
    where
        T: Display,
    {
        Error::DeserializationError(msg.to_string())
    }
}
