use alloc::{
    borrow::Cow,
    format,
    string::{String, ToString},
    vec::Vec,
};
use serde_core::de::{self, IntoDeserializer};

use crate::{
    BorrowedObject, BorrowedValue, Error, IndexMap, Result,
    ast::{Entry, EntryOrSpread, Inputs, PairOrSpread, Root},
    lexer::{Lexer, StringPart},
    parser::RootParser,
};

/// A structure that deserializes Corn configuration values.
#[derive(Clone)]
pub struct Deserializer<'de> {
    value: BorrowedValue<'de>,
}

/// Parse a Corn configuration string into a borrowed value.
pub fn parse(input: &str) -> Result<BorrowedValue<'_>> {
    Deserializer::parse(input)
}

impl<'de> Deserializer<'de> {
    /// Parse a Corn configuration string into a borrowed value.
    pub fn parse(input: &str) -> Result<BorrowedValue<'_>> {
        let mut lexer = Lexer::new(input);
        let parser = RootParser::new();
        let Root { inputs, object } = parser
            .parse(input, &mut lexer)
            .map_err(|err| Error::ParseError(err.to_string()))?;

        Self::resolve_entry(&Entry::Object(object), &inputs)
    }

    /// Create a deserializer from a Corn configuration string.
    #[allow(clippy::should_implement_trait)]
    pub fn from_str(input: &'de str) -> Result<Self> {
        Self::parse(input).map(|value| Self { value })
    }

    fn with_value(value: BorrowedValue<'de>) -> Self {
        Self { value }
    }

    fn resolve_entry<'input>(
        entry: &Entry<'input>,
        inputs: &Inputs<'input>,
    ) -> Result<BorrowedValue<'input>> {
        match entry {
            Entry::String(parts) => {
                if parts.is_empty() {
                    return Ok(BorrowedValue::String(Cow::Borrowed("")));
                }

                let mut base = String::new();

                for part in parts {
                    match part {
                        StringPart::Literal(lit) => base.push_str(lit),
                        StringPart::Input(input) => {
                            let input = Self::resolve_input(input, inputs)?;

                            match input {
                                BorrowedValue::String(string) => base.push_str(&string),
                                _ => return Err(Error::InvalidInterpolationError),
                            }
                        }
                    }
                }

                Ok(BorrowedValue::String(Cow::Owned(
                    Self::process_multiline_string(&base),
                )))
            }
            Entry::Integer(integer) => Ok(BorrowedValue::Integer(*integer)),
            Entry::Float(float) => Ok(BorrowedValue::Float(*float)),
            Entry::Boolean(boolean) => Ok(BorrowedValue::Boolean(*boolean)),
            Entry::Object(obj) => {
                let mut resolved_object = IndexMap::default();

                for pair_or_spread in &obj.pairs {
                    match pair_or_spread {
                        PairOrSpread::Pair(key, value) => {
                            fn unescape_key(key: &str) -> Cow<'_, str> {
                                if key.contains("\\'") {
                                    Cow::Owned(key.replace("\\'", "'"))
                                } else {
                                    Cow::Borrowed(key)
                                }
                            }

                            let processed_segments: Vec<Cow<str>> = key
                                .segments
                                .iter()
                                .map(|segment| unescape_key(segment))
                                .collect();

                            resolved_object.reserve_exact(processed_segments.len());

                            Self::insert_at_path(
                                &mut resolved_object,
                                &processed_segments,
                                Self::resolve_entry(value, inputs)?,
                            )?;
                        }
                        PairOrSpread::Spread(name) => {
                            if let Some(spread_entry) = inputs.get(name) {
                                match Self::resolve_entry(spread_entry, inputs)? {
                                    BorrowedValue::Object(spread_obj) => {
                                        resolved_object.extend(spread_obj);
                                    }
                                    _ => return Err(Error::InvalidSpreadError),
                                }
                            } else {
                                return Err(Error::InputResolveError(name.to_string()));
                            }
                        }
                    }
                }

                resolved_object.shrink_to_fit();

                Ok(BorrowedValue::Object(resolved_object))
            }
            Entry::Array(items) => {
                let mut resolved_array = Vec::with_capacity(items.len()); // We need at least the same amount of items

                for entry in items {
                    match entry {
                        EntryOrSpread::Entry(entry) => {
                            resolved_array.push(Self::resolve_entry(entry, inputs)?)
                        }
                        EntryOrSpread::Spread(spread) => match Self::resolve_input(spread, inputs)?
                        {
                            BorrowedValue::Array(array) => {
                                resolved_array.extend(array);
                            }
                            _ => return Err(Error::InvalidSpreadError),
                        },
                    }
                }

                Ok(BorrowedValue::Array(resolved_array))
            }
            Entry::Null => Ok(BorrowedValue::Null),
            Entry::Input(input) => Self::resolve_input(input, inputs),
        }
    }

    fn insert_at_path<'input>(
        obj: &mut BorrowedObject<'input>,
        path: &[Cow<'input, str>],
        value: BorrowedValue<'input>,
    ) -> Result<(), Error> {
        if path.is_empty() {
            return Err(Error::DeserializationError("Empty path".to_string()));
        }

        if path.len() == 1 {
            obj.insert(path[0].clone(), value);
            return Ok(());
        }

        let (first, rest) = path.split_first().expect("Internal splitting error");
        let entry = obj
            .entry(first.clone())
            .or_insert_with(|| BorrowedValue::Object(IndexMap::default()));

        match entry {
            BorrowedValue::Object(nested_obj) => {
                Self::insert_at_path(nested_obj, rest, value)?;
            }
            _ => {
                return Err(Error::DeserializationError(format!(
                    "Cannot index into non-object at key: {first}"
                )));
            }
        }

        Ok(())
    }

    fn process_multiline_string(input: &str) -> String {
        if !input.starts_with('\n') {
            return input.to_string();
        }

        let lines: Vec<&str> = input.lines().collect();
        if lines.len() < 3 {
            // Need at least: empty, content, empty/content
            return input.to_string();
        }

        // Skip first empty line and handle last line (may be empty or just whitespace)
        let mut content_lines: Vec<&str> = lines.iter().skip(1).copied().collect();

        // Remove trailing lines that are empty or only whitespace
        while let Some(&last) = content_lines.last() {
            if last.trim().is_empty() {
                content_lines.pop();
            } else {
                break;
            }
        }

        if content_lines.is_empty() {
            return String::new();
        }

        // Find minimum indentation of non-empty lines
        let min_indent = content_lines
            .iter()
            .filter(|line| !line.trim().is_empty())
            .map(|line| line.len() - line.trim_start().len())
            .min()
            .unwrap_or(0);

        // Remove minimum indentation and join with newlines
        let result_lines: Vec<&str> = content_lines
            .iter()
            .map(|line| {
                if line.trim().is_empty() {
                    ""
                } else if line.len() >= min_indent {
                    &line[min_indent..]
                } else {
                    line
                }
            })
            .collect();

        let mut result = result_lines.join("\n");

        if !result.is_empty() {
            result.push('\n');
        }

        result
    }

    fn resolve_input<'input>(
        input: &str,
        inputs: &Inputs<'input>,
    ) -> Result<BorrowedValue<'input>> {
        #[cfg(feature = "std")]
        if let Some(env) = input.strip_prefix("env_")
            && let Ok(env) = std::env::var(env)
        {
            return Ok(BorrowedValue::String(Cow::Owned(env)));
        }

        if let Some(entry) = inputs.get(input) {
            return Self::resolve_entry(entry, inputs);
        }

        Err(Error::InputResolveError(input.to_string()))
    }
}

/// Deserialize a Corn configuration string into a Rust data structure.
pub fn from_str<'a, T>(s: &'a str) -> Result<T, Error>
where
    T: de::Deserialize<'a>,
{
    let mut deserializer = Deserializer::from_str(s)?;

    T::deserialize(&mut deserializer)
}

macro_rules! deserialize_number {
    ($method:ident) => {
        fn $method<V>(self, visitor: V) -> Result<V::Value>
        where
            V: de::Visitor<'de>,
        {
            match self.value {
                BorrowedValue::Integer(integer) => integer.deserialize_any(visitor),
                ref value => Err(value.invalid_type("Integer")),
            }
        }
    };
}

impl<'de> de::Deserializer<'de> for &mut Deserializer<'de> {
    type Error = Error;

    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        match self.value {
            BorrowedValue::String(ref string) => match string {
                Cow::Borrowed(s) => visitor.visit_borrowed_str(s),
                Cow::Owned(s) => visitor.visit_str(s),
            },
            BorrowedValue::Integer(integer) => integer.deserialize_any(visitor),
            BorrowedValue::Float(float) => visitor.visit_f64(float),
            BorrowedValue::Boolean(boolean) => visitor.visit_bool(boolean),
            BorrowedValue::Null => visitor.visit_unit(),
            BorrowedValue::Array(ref mut items) => {
                let mut seq = Vec::new();
                core::mem::swap(items, &mut seq);

                visitor.visit_seq(SeqAccess::new(seq))
            }
            BorrowedValue::Object(ref mut object) => {
                let mut map = IndexMap::default();
                core::mem::swap(object, &mut map);

                visitor.visit_map(MapAccess::new(map))
            }
        }
    }

    fn deserialize_bool<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        match self.value {
            BorrowedValue::Boolean(boolean) => visitor.visit_bool(boolean),
            ref value => Err(value.invalid_type("Boolean")),
        }
    }

    deserialize_number!(deserialize_i8);
    deserialize_number!(deserialize_i16);
    deserialize_number!(deserialize_i32);
    deserialize_number!(deserialize_i64);
    deserialize_number!(deserialize_u8);
    deserialize_number!(deserialize_u16);
    deserialize_number!(deserialize_u32);
    deserialize_number!(deserialize_u64);

    fn deserialize_f32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        self.deserialize_f64(visitor)
    }

    fn deserialize_f64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        match self.value {
            BorrowedValue::Float(float) => visitor.visit_f64(float),
            ref value => Err(value.invalid_type("Float")),
        }
    }

    fn deserialize_char<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        self.deserialize_str(visitor)
    }

    fn deserialize_str<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        match self.value {
            BorrowedValue::String(ref string) => match string {
                Cow::Borrowed(s) => visitor.visit_borrowed_str(s),
                Cow::Owned(s) => visitor.visit_str(s),
            },
            ref value => Err(value.invalid_type("String")),
        }
    }

    fn deserialize_string<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        self.deserialize_str(visitor)
    }

    fn deserialize_bytes<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        match self.value {
            BorrowedValue::String(ref string) => visitor.visit_bytes(string.as_bytes()),
            ref value => Err(value.invalid_type("Byte String")),
        }
    }

    fn deserialize_byte_buf<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        self.deserialize_bytes(visitor)
    }

    fn deserialize_option<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        match self.value {
            BorrowedValue::Null => visitor.visit_none(),
            _ => visitor.visit_some(self),
        }
    }

    fn deserialize_unit<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        match self.value {
            BorrowedValue::Null => visitor.visit_unit(),
            ref value => Err(value.invalid_type("Null")),
        }
    }

    fn deserialize_unit_struct<V>(
        self,
        _name: &'static str,
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        self.deserialize_unit(visitor)
    }

    fn deserialize_newtype_struct<V>(
        self,
        _name: &'static str,
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        visitor.visit_newtype_struct(self)
    }

    fn deserialize_seq<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        match self.value {
            BorrowedValue::Array(ref mut items) => {
                let mut seq = Vec::new();
                core::mem::swap(items, &mut seq);

                visitor.visit_seq(SeqAccess::new(seq))
            }
            ref value => Err(value.invalid_type("Array")),
        }
    }

    fn deserialize_tuple<V>(self, _len: usize, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        self.deserialize_seq(visitor)
    }

    fn deserialize_tuple_struct<V>(
        self,
        _name: &'static str,
        _len: usize,
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        self.deserialize_seq(visitor)
    }

    fn deserialize_map<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        match self.value {
            BorrowedValue::Object(ref mut object) => {
                let mut map = IndexMap::default();
                core::mem::swap(object, &mut map);

                visitor.visit_map(MapAccess::new(map))
            }
            ref value => Err(value.invalid_type("Object")),
        }
    }

    fn deserialize_struct<V>(
        self,
        _name: &'static str,
        _fields: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        self.deserialize_map(visitor)
    }

    fn deserialize_enum<V>(
        self,
        _name: &'static str,
        _variants: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        match self.value {
            BorrowedValue::String(ref string) => {
                visitor.visit_enum(string.as_ref().into_deserializer())
            }
            BorrowedValue::Object(ref mut object) => {
                let mut map = IndexMap::default();
                core::mem::swap(object, &mut map);

                visitor.visit_enum(EnumAccess::new(map))
            }
            ref value => Err(value.invalid_type("String or Object")),
        }
    }

    fn deserialize_identifier<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        self.deserialize_str(visitor)
    }

    fn deserialize_ignored_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        self.deserialize_any(visitor)
    }
}

struct SeqAccess<'de> {
    items: alloc::vec::IntoIter<BorrowedValue<'de>>,
}

impl<'de> SeqAccess<'de> {
    pub fn new(items: Vec<BorrowedValue<'de>>) -> Self {
        Self {
            items: items.into_iter(),
        }
    }
}

impl<'de> de::SeqAccess<'de> for SeqAccess<'de> {
    type Error = Error;

    fn next_element_seed<T>(&mut self, seed: T) -> Result<Option<T::Value>, Self::Error>
    where
        T: de::DeserializeSeed<'de>,
    {
        match self.items.next() {
            Some(item) => {
                let mut deserializer = Deserializer::with_value(item);
                seed.deserialize(&mut deserializer).map(Some)
            }
            None => Ok(None),
        }
    }
}

struct MapAccess<'de> {
    items: indexmap::map::IntoIter<Cow<'de, str>, BorrowedValue<'de>>,
    current_value: Option<BorrowedValue<'de>>,
}

impl<'de> MapAccess<'de> {
    fn new(items: IndexMap<Cow<'de, str>, BorrowedValue<'de>>) -> Self {
        Self {
            items: items.into_iter(),
            current_value: None,
        }
    }
}

impl<'de> de::MapAccess<'de> for MapAccess<'de> {
    type Error = Error;

    fn next_key_seed<K>(&mut self, seed: K) -> Result<Option<K::Value>, Self::Error>
    where
        K: de::DeserializeSeed<'de>,
    {
        match self.items.next() {
            Some((key, value)) => {
                self.current_value = Some(value);
                let mut key_deserializer = Deserializer::with_value(BorrowedValue::String(key));
                seed.deserialize(&mut key_deserializer).map(Some)
            }
            None => Ok(None),
        }
    }

    fn next_value_seed<V>(&mut self, seed: V) -> Result<V::Value, Self::Error>
    where
        V: de::DeserializeSeed<'de>,
    {
        match self.current_value.take() {
            Some(value) => {
                let mut deserializer = Deserializer::with_value(value);
                seed.deserialize(&mut deserializer)
            }
            None => Err(Error::DeserializationError(
                "No value available".to_string(),
            )),
        }
    }
}
struct EnumAccess<'de> {
    object: BorrowedObject<'de>,
}

impl<'de> EnumAccess<'de> {
    fn new(object: BorrowedObject<'de>) -> Self {
        Self { object }
    }
}

impl<'de> de::EnumAccess<'de> for EnumAccess<'de> {
    type Error = Error;
    type Variant = VariantAccess<'de>;

    fn variant_seed<V>(
        self,
        seed: V,
    ) -> core::result::Result<(V::Value, Self::Variant), Self::Error>
    where
        V: de::DeserializeSeed<'de>,
    {
        if self.object.len() != 1 {
            return Err(Error::DeserializationError(format!(
                "Expected enum object with exactly one key, found {}",
                self.object.len()
            )));
        }

        let (key, value) = self
            .object
            .into_iter()
            .next()
            .expect("Internal variant error");

        let mut key_deserializer = Deserializer::with_value(BorrowedValue::String(key));
        let variant = seed.deserialize(&mut key_deserializer)?;

        Ok((variant, VariantAccess::new(value)))
    }
}

struct VariantAccess<'de> {
    value: BorrowedValue<'de>,
}

impl<'de> VariantAccess<'de> {
    fn new(value: BorrowedValue<'de>) -> Self {
        Self { value }
    }
}

impl<'de> de::VariantAccess<'de> for VariantAccess<'de> {
    type Error = Error;

    fn unit_variant(self) -> core::result::Result<(), Self::Error> {
        match self.value {
            BorrowedValue::Null => Ok(()),
            ref value => Err(value.invalid_type("unit variant (null)")),
        }
    }

    fn newtype_variant_seed<T>(self, seed: T) -> core::result::Result<T::Value, Self::Error>
    where
        T: de::DeserializeSeed<'de>,
    {
        seed.deserialize(&mut Deserializer::with_value(self.value))
    }

    fn tuple_variant<V>(
        self,
        _len: usize,
        visitor: V,
    ) -> core::result::Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        match self.value {
            BorrowedValue::Array(items) => visitor.visit_seq(SeqAccess::new(items)),
            ref value => Err(value.invalid_type("tuple variant (array)")),
        }
    }

    fn struct_variant<V>(
        self,
        _fields: &'static [&'static str],
        visitor: V,
    ) -> core::result::Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        match self.value {
            BorrowedValue::Object(object) => visitor.visit_map(MapAccess::new(object)),
            ref value => Err(value.invalid_type("struct variant (object)")),
        }
    }
}
