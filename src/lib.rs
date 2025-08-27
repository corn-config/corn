#![warn(missing_docs)]
#![doc = include_str!("../README.md")]

#[cfg(any(
    feature = "lua51",
    feature = "lua52",
    feature = "lua53",
    feature = "lua54",
    feature = "luajit",
    feature = "luajit52"
))]
mod lua;

mod de;
mod error;
mod value;

pub use de::{from_str, Deserializer};
pub use error::{Error, Result};
pub use value::{Object, Value};
