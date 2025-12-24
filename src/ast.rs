use crate::{Integer, lexer::StringPart};
use alloc::vec::Vec;

#[cfg(not(feature = "std"))]
use hashbrown::HashMap;
#[cfg(feature = "std")]
use std::collections::HashMap;

/// Store for input declarations
pub type Inputs<'input> = HashMap<&'input str, Entry<'input>>;

/// Top level ast object
#[derive(Debug, Clone)]
pub struct Root<'input> {
    /// Raw inputs
    pub inputs: Inputs<'input>,
    /// Top level object, values aren't interpolated
    pub object: Object<'input>,
}

/// Represents an object in the AST
#[derive(Debug, Clone)]
pub struct Object<'input> {
    // /// The pairs or spread operations in the object
    pub pairs: Vec<PairOrSpread<'input>>,
}

/// Either a key-value pair or a spread operation
#[derive(Debug, Clone)]
pub enum PairOrSpread<'input> {
    /// A key-value pair in an object
    Pair(ChainedKey<'input>, Entry<'input>),
    /// A spread operation in an object
    Spread(&'input str),
}

/// Represents a chained key like "foo.bar.baz"
#[derive(Debug, Clone)]
pub struct ChainedKey<'input> {
    /// The segments of the key path
    pub segments: Vec<&'input str>,
}

/// An entry can be of various types as defined in the spec
#[derive(Debug, Clone)]
pub enum Entry<'input> {
    /// String literal
    String(Vec<StringPart<'input>>),
    /// Integer value
    Integer(Integer),
    /// Float value
    Float(f64),
    /// Boolean value
    Boolean(bool),
    /// Nested object
    Object(Object<'input>),
    /// Array of entries
    Array(Vec<EntryOrSpread<'input>>),
    /// Null value
    Null,
    /// Reference to an input variable
    Input(&'input str),
}

/// Either a key or an array spread operation
#[derive(Debug, Clone)]
pub enum EntryOrSpread<'input> {
    /// An entry can be of various types as defined in the spec
    Entry(Entry<'input>),
    /// Array spread operation
    Spread(&'input str),
}
