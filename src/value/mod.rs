use indexmap::IndexMap;

mod de;
mod integer;
mod ser;

pub use integer::Integer;

/// Object: Key-value collection that preserves insertion order
pub type Object = IndexMap<String, Value>;

/// Represents a Corn configuration value.
///
/// This enum encompasses all possible value types in the Corn language specification:
/// - String: UTF-8 string values
/// - Integer: 64-bit signed integers
/// - Float: 64-bit floating point numbers
/// - Boolean: true/false values
/// - Object: Key-value collection that preserves insertion order
/// - Array: Ordered collections of values
/// - Null: Represents absence of a value
#[derive(Debug, Clone)]
pub enum Value {
    /// A UTF-8 string value
    String(String),
    /// A 64-bit signed integer
    Integer(Integer),
    /// A 64-bit floating point number
    Float(f64),
    /// A boolean value (true or false)
    Boolean(bool),
    /// A key-value collection that preserves insertion order
    Object(Object),
    /// An ordered collection of values
    Array(Vec<Value>),
    /// Represents the absence of a value
    Null,
}

impl Value {
    /// Returns true if the value is a String.
    pub const fn is_string(&self) -> bool {
        matches!(self, Self::String(_))
    }

    /// Returns true if the value is an Integer.
    pub const fn is_integer(&self) -> bool {
        matches!(self, Self::Integer(_))
    }

    /// Returns true if the value is a Float.
    pub const fn is_float(&self) -> bool {
        matches!(self, Self::Float(_))
    }

    /// Returns true if the value is a Boolean.
    pub const fn is_boolean(&self) -> bool {
        matches!(self, Self::Boolean(_))
    }

    /// Returns true if the value is an Object.
    pub const fn is_object(&self) -> bool {
        matches!(self, Self::Object(_))
    }

    /// Returns true if the value is an Array.
    pub const fn is_array(&self) -> bool {
        matches!(self, Self::Array(_))
    }

    /// Returns true if the value is Null.
    pub const fn is_null(&self) -> bool {
        matches!(self, Self::Null)
    }

    /// Returns the inner String if this value is a String, otherwise None.
    pub fn as_string(&self) -> Option<&String> {
        match self {
            Self::String(s) => Some(s),
            _ => None,
        }
    }

    /// Returns the inner Integer if this value is an Integer, otherwise None.
    pub const fn as_integer(&self) -> Option<&Integer> {
        match self {
            Self::Integer(integer) => Some(integer),
            _ => None,
        }
    }

    /// Returns the inner Float if this value is a Float, otherwise None.
    pub const fn as_float(&self) -> Option<&f64> {
        match self {
            Self::Float(f) => Some(f),
            _ => None,
        }
    }

    /// Returns the inner Boolean if this value is a Boolean, otherwise None.
    pub const fn as_boolean(&self) -> Option<&bool> {
        match self {
            Self::Boolean(b) => Some(b),
            _ => None,
        }
    }

    /// Returns the inner Object if this value is an Object, otherwise None.
    pub fn as_object(&self) -> Option<&Object> {
        match self {
            Self::Object(obj) => Some(obj),
            _ => None,
        }
    }

    /// Returns the inner Array if this value is an Array, otherwise None.
    pub fn as_array(&self) -> Option<&Vec<Value>> {
        match self {
            Self::Array(arr) => Some(arr),
            _ => None,
        }
    }

    /// Returns a mutable reference to the inner String if this value is a String, otherwise None.
    pub fn as_string_mut(&mut self) -> Option<&mut String> {
        match self {
            Self::String(s) => Some(s),
            _ => None,
        }
    }

    /// Returns a mutable reference to the inner Integer if this value is an Integer, otherwise None.
    pub fn as_integer_mut(&mut self) -> Option<&mut Integer> {
        match self {
            Self::Integer(i) => Some(i),
            _ => None,
        }
    }

    /// Returns a mutable reference to the inner Float if this value is a Float, otherwise None.
    pub fn as_float_mut(&mut self) -> Option<&mut f64> {
        match self {
            Self::Float(f) => Some(f),
            _ => None,
        }
    }

    /// Returns a mutable reference to the inner Boolean if this value is a Boolean, otherwise None.
    pub fn as_boolean_mut(&mut self) -> Option<&mut bool> {
        match self {
            Self::Boolean(b) => Some(b),
            _ => None,
        }
    }

    /// Returns a mutable reference to the inner Object if this value is an Object, otherwise None.
    pub fn as_object_mut(&mut self) -> Option<&mut Object> {
        match self {
            Self::Object(obj) => Some(obj),
            _ => None,
        }
    }

    /// Returns a mutable reference to the inner Array if this value is an Array, otherwise None.
    pub fn as_array_mut(&mut self) -> Option<&mut Vec<Value>> {
        match self {
            Self::Array(arr) => Some(arr),
            _ => None,
        }
    }

    /// Takes the inner String if this value is a String, otherwise None.
    pub fn take_string(self) -> Option<String> {
        match self {
            Self::String(s) => Some(s),
            _ => None,
        }
    }

    /// Takes the inner Integer if this value is an Integer, otherwise None.
    pub fn take_integer(self) -> Option<Integer> {
        match self {
            Self::Integer(i) => Some(i),
            _ => None,
        }
    }

    /// Takes the inner Float if this value is a Float, otherwise None.
    pub fn take_float(self) -> Option<f64> {
        match self {
            Self::Float(f) => Some(f),
            _ => None,
        }
    }

    /// Takes the inner Boolean if this value is a Boolean, otherwise None.
    pub fn take_boolean(self) -> Option<bool> {
        match self {
            Self::Boolean(b) => Some(b),
            _ => None,
        }
    }

    /// Takes the inner Object if this value is an Object, otherwise None.
    pub fn take_object(self) -> Option<Object> {
        match self {
            Self::Object(obj) => Some(obj),
            _ => None,
        }
    }

    /// Takes the inner Array if this value is an Array, otherwise None.
    pub fn take_array(self) -> Option<Vec<Value>> {
        match self {
            Self::Array(arr) => Some(arr),
            _ => None,
        }
    }

    /// Returns true if the value is empty.
    /// An empty value is an empty String, empty Object, empty Array, or Null.
    pub fn is_empty(&self) -> bool {
        match self {
            Self::String(s) => s.is_empty(),
            Self::Object(obj) => obj.is_empty(),
            Self::Array(arr) => arr.is_empty(),
            Self::Null => true,
            _ => false,
        }
    }

    /// Returns the number of elements in this Value.
    /// For objects this is the number of key-value pairs, for arrays it's the number of elements,
    /// for strings it's the string length, and for other types it's 0.
    pub fn len(&self) -> usize {
        match self {
            Self::String(s) => s.len(),
            Self::Object(obj) => obj.len(),
            Self::Array(arr) => arr.len(),
            _ => 0,
        }
    }

    /// Get a reference to a value in an object by key.
    /// Returns None if the value is not an object or if the key doesn't exist.
    pub fn get(&self, key: &str) -> Option<&Value> {
        match self {
            Self::Object(obj) => obj.get(key),
            _ => None,
        }
    }

    /// Get a reference to a value in an array by index.
    /// Returns None if the value is not an array or if the index is out of bounds.
    pub fn get_index(&self, index: usize) -> Option<&Value> {
        match self {
            Self::Array(arr) => arr.get(index),
            _ => None,
        }
    }
}
