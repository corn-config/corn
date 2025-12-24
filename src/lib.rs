#![cfg_attr(not(feature = "std"), no_std)]

extern crate alloc;

#[cfg(feature = "std")]
pub(crate) use indexmap::IndexMap;
#[cfg(not(feature = "std"))]
pub(crate) type IndexMap<K, V, S = hashbrown::DefaultHashBuilder> = indexmap::IndexMap<K, V, S>;

mod de;
#[cfg(any(
    feature = "lua51",
    feature = "lua52",
    feature = "lua53",
    feature = "lua54",
    feature = "luajit",
    feature = "luajit52"
))]
mod lua;
#[cfg(feature = "wasm")]
mod wasm;

mod error;
pub(crate) mod value;

pub mod ast;
pub mod lexer;
lalrpop_util::lalrpop_mod!(pub parser, "/corn.rs");

pub use de::{Deserializer, from_str, parse};
pub use error::{Error, Result};
pub use value::{BorrowedObject, BorrowedValue, Integer, Object, Value};
