use std::fmt::Display;

/// A type alias for the standard Result type, defaulting to the crate's Error type.
pub type Result<T, E = Error> = std::result::Result<T, E>;

/// Represents all possible errors that can occur when parsing or processing Corn data.
#[derive(Debug)]
pub enum Error {
    /// An I/O error that occurred during file operations.
    Io(std::io::Error),

    /// Indicates an unexpected end of input during parsing.
    Eof,

    InvalidUtf8,

    /// Indicates that an unexpected token was encountered during parsing.
    UnexpectedToken {
        /// Description of what was expected at this position
        expected: &'static str,
        /// The actual byte value found
        found: u8,
        /// The position in the input where the unexpected token was found
        index: usize,
    },

    /// This variant is typically used when handling serde-specific deserialization errors.
    DeserializationError(String),
}

impl Error {
    /// Creates a new UnexpectedToken error with the provided details.
    pub const fn unexpected_token(expected: &'static str, found: u8, index: usize) -> Self {
        Self::UnexpectedToken {
            expected,
            found,
            index,
        }
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(e) => e.fmt(f),
            Self::Eof => write!(f, "Unexpected end of input"),
            Self::InvalidUtf8 => todo!(),
            Self::UnexpectedToken {
                expected,
                found,
                index,
            } => write!(
                f,
                "Unexpected Token {found} at index {index}, expected {expected}"
            ),
            Self::DeserializationError(s) => write!(f, "Failed to deserialize input: {s}"),
        }
    }
}

impl std::error::Error for Error {}

impl serde::de::Error for Error {
    fn custom<T>(msg: T) -> Self
    where
        T: Display,
    {
        Error::DeserializationError(msg.to_string())
    }
}
