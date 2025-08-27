use std::collections::HashMap;

use serde::{de, forward_to_deserialize_any, Deserialize};

use crate::{Error, Result, Value};

/// A structure that deserializes Corn into Rust values.
#[derive(Clone)]
pub struct Deserializer<'de> {
    bytes: &'de [u8],
    index: usize,
    variables: HashMap<String, Value>,
}

pub struct Position {
    pub line: usize,
    pub column: usize,
}

impl<'de> Deserializer<'de> {
    /// Refer to the `Deserializer::from_str` method for more info.
    pub fn from_str(input: &'de str) -> Result<Self> {
        let mut de = Self {
            bytes: input.as_bytes(),
            index: 0,
            variables: HashMap::new(),
        };

        de.parse_let_block()?;

        Ok(de)
    }

    fn position(&self, i: usize) -> Position {
        let start_of_line = match memchr::memrchr(b'\n', &self.bytes[..i]) {
            Some(position) => position + 1,
            None => 0,
        };

        Position {
            line: 1 + memchr::memchr_iter(b'\n', &self.bytes[..start_of_line]).count(),
            column: i - start_of_line,
        }
    }

    fn advance(&mut self) {
        self.index += 1;
    }

    fn next(&mut self) -> Result<Option<u8>> {
        let byte = self.peek()?;

        self.advance();

        Ok(byte)
    }

    fn peek(&mut self) -> Result<Option<u8>> {
        Ok(self.bytes.get(self.index).copied())
    }

    fn parse_whitespace(&mut self) -> Result<Option<u8>> {
        loop {
            match self.peek()? {
                Some(byte) if byte.is_ascii_whitespace() => {
                    self.advance();
                }
                other => return Ok(other),
            }
        }
    }

    fn whitespace_or_eof(&mut self) -> Result<u8> {
        match self.parse_whitespace() {
            Ok(Some(byte)) => Ok(byte),
            Ok(None) => Err(Error::Eof),
            Err(err) => Err(err),
        }
    }

    fn parse_key(&mut self) -> Result<String> {
        todo!()
    }

    fn parse_ident(&mut self, ident: &[u8]) -> Result<()> {
        for expected in ident {
            match self.next()? {
                None => {
                    return Err(Error::Eof);
                }
                Some(next) => {
                    if next != *expected {
                        return Err(Error::DeserializationError("No ident".to_string()));
                    }
                }
            }
        }

        Ok(())
    }

    fn parse_integer(&mut self, negative: bool) -> Result<i64> {
        let next = self.whitespace_or_eof()?;
        self.advance();

        match next {
            c @ b'1'..=b'9' => {
                let mut significand = (c - b'0') as i64;

                loop {
                    match self.next()? {
                        Some(token) if token.is_ascii_whitespace() => break,
                        None => break,

                        Some(integer @ b'0'..=b'9') => {
                            let digit = (integer - b'0') as i64;

                            significand = significand * 10 + digit;
                        }

                        Some(token) => {
                            return Err(Error::unexpected_token("integer", token, self.index))
                        }
                    }
                }

                return Ok(if negative {
                    significand.wrapping_neg()
                } else {
                    significand
                });
            }
            _ => todo!(),
        }
    }

    fn parse_let_block(&mut self) -> Result<()> {
        match self.whitespace_or_eof()? {
            b'{' => return Ok(()),
            b'l' => {
                self.parse_ident(b"let")?;

                match self.whitespace_or_eof()? {
                    b'{' => {
                        self.advance();
                        loop {
                            match self.whitespace_or_eof()? {
                                b'$' => {
                                    self.advance();

                                    let start = self.index;

                                    loop {
                                        match self.peek()? {
                                            Some(byte) => {
                                                if byte.is_ascii_whitespace() {
                                                    break;
                                                }

                                                // FIXME: check the input is alphanumeric
                                                // FIXME: check the inputs starts with a letter

                                                self.advance();
                                            }
                                            None => break,
                                        }
                                    }

                                    let end = self.index;

                                    if start == end {
                                        // return Err(Error::EmptyKey);
                                    }

                                    // TODO: better error handling
                                    let input = std::str::from_utf8(&self.bytes[start..end])
                                        .map_err(|_| Error::InvalidUtf8)?;

                                    match self.whitespace_or_eof()? {
                                        b'=' => {
                                            self.advance(); // Skip the equals sign

                                            let value = Value::deserialize(&mut *self)?;

                                            self.variables.insert(input.to_owned(), value);
                                        }
                                        token => {
                                            return Err(Error::unexpected_token(
                                                "=", token, self.index,
                                            ))
                                        }
                                    }
                                }
                                b'}' => {
                                    self.advance();
                                    break;
                                }
                                token => {
                                    return Err(Error::unexpected_token(
                                        "input definition or }",
                                        token,
                                        self.index,
                                    ))
                                }
                            }
                        }

                        match self.whitespace_or_eof()? {
                            b'i' => {
                                self.parse_ident(b"in")?;
                            }
                            token => return Err(Error::unexpected_token("in", token, self.index)),
                        }
                    }
                    token => return Err(Error::unexpected_token("{", token, self.index)),
                }
            }
            token => return Err(Error::unexpected_token("one of: let, {", token, self.index)),
        }

        Ok(())
    }
}

/// Deserializes a Corn-formatted string into a Rust type.
///
/// # Example
///
/// ```
/// use corn::from_str;
///
/// #[derive(serde::Deserialize)]
/// struct Config {
///     name: String,
///     version: u32,
/// }
///
/// let corn_str = "{ name = \"My App\" version = 1 }";
/// let config: Config = from_str(corn_str).unwrap();
/// assert_eq!(config.name, "My App");
/// assert_eq!(config.version, 1);
/// ```
pub fn from_str<'a, T>(s: &'a str) -> Result<T, Error>
where
    T: de::Deserialize<'a>,
{
    let mut deserializer = Deserializer::from_str(s)?;

    T::deserialize(&mut deserializer)
}

// TODO: more specialized number parsing
// TODO: extract parsing logic from deserializer
impl<'de> de::Deserializer<'de> for &mut Deserializer<'de> {
    type Error = Error;

    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        match self.whitespace_or_eof()? {
            b'n' => {
                self.parse_ident(b"null")?;
                visitor.visit_unit()
            }
            b't' => {
                self.parse_ident(b"true")?;
                visitor.visit_bool(true)
            }
            b'f' => {
                self.parse_ident(b"false")?;
                visitor.visit_bool(false)
            }
            b'-' => {
                self.advance();
                visitor.visit_i64(self.parse_integer(true)?)
            }
            b'0'..=b'9' => visitor.visit_i64(self.parse_integer(false)?),
            b'"' => {
                self.advance();

                let start = self.index;

                loop {
                    match self.next()? {
                        Some(byte) => {
                            if byte == b'"' {
                                break;
                            }
                        }
                        None => return Err(Error::Eof),
                    }
                }

                let end = self.index - 1;

                let string =
                    std::str::from_utf8(&self.bytes[start..end]).map_err(|_| Error::InvalidUtf8)?;

                visitor.visit_str(string)
            }
            b'[' => {
                self.advance();
                visitor.visit_seq(SeqAccess::new(self))
            }
            b'{' => {
                self.advance();
                visitor.visit_map(MapAccess::new(self))
            }
            token => Err(Error::unexpected_token(
                "one of: any", // FIXME: include more info
                token,
                self.index,
            )),
        }
    }

    forward_to_deserialize_any! {
        bool i8 i16 i32 i64 i128 u8 u16 u32 u64 u128 f32 f64 char str string
        bytes byte_buf option unit unit_struct newtype_struct seq tuple
        tuple_struct map struct enum identifier ignored_any
    }
}

struct SeqAccess<'a, 'de: 'a> {
    de: &'a mut Deserializer<'de>,
}

impl<'a, 'de> SeqAccess<'a, 'de> {
    pub fn new(de: &'a mut Deserializer<'de>) -> Self {
        Self { de }
    }
}

impl<'a, 'de> de::SeqAccess<'de> for SeqAccess<'a, 'de> {
    type Error = Error;

    fn next_element_seed<T>(
        &mut self,
        seed: T,
    ) -> std::result::Result<Option<T::Value>, Self::Error>
    where
        T: de::DeserializeSeed<'de>,
    {
        match self.de.whitespace_or_eof()? {
            b']' => {
                self.de.advance();
                return Ok(None);
            }
            _ => seed.deserialize(&mut *self.de).map(Some),
        }
    }
}

struct MapAccess<'a, 'de: 'a> {
    de: &'a mut Deserializer<'de>,
}

impl<'a, 'de> MapAccess<'a, 'de> {
    pub fn new(de: &'a mut Deserializer<'de>) -> Self {
        Self { de }
    }
}

impl<'a, 'de> de::MapAccess<'de> for MapAccess<'a, 'de> {
    type Error = Error;

    fn next_key_seed<K>(&mut self, seed: K) -> std::result::Result<Option<K::Value>, Self::Error>
    where
        K: de::DeserializeSeed<'de>,
    {
        match self.de.whitespace_or_eof()? {
            b'}' => {
                self.de.advance();
                return Ok(None);
            }
            b'\'' => {
                unimplemented!("escaped keys")
            }
            _ => {
                let start = self.de.index;

                loop {
                    match self.de.peek()? {
                        Some(byte) => {
                            if byte.is_ascii_whitespace() || matches!(byte, b'=' | b'}') {
                                break;
                            }

                            if byte == b'.' {
                                self.de.advance();

                                unimplemented!("chains")
                            }

                            self.de.advance();
                        }
                        None => break,
                    }
                }

                let end = self.de.index;

                if start == end {
                    // return Err(Error::EmptyKey);
                }

                let key = std::str::from_utf8(&self.de.bytes[start..end])
                    .map_err(|_| Error::InvalidUtf8)?;

                seed.deserialize(de::value::StrDeserializer::new(key))
                    .map(Some)
            }
        }
    }

    fn next_value_seed<V>(&mut self, seed: V) -> std::result::Result<V::Value, Self::Error>
    where
        V: de::DeserializeSeed<'de>,
    {
        match self.de.whitespace_or_eof()? {
            b'=' => {
                self.de.advance(); // Skip the equals sign
                seed.deserialize(&mut *self.de)
            }
            token => Err(Error::unexpected_token("=", token, self.de.index)),
        }
    }
}
