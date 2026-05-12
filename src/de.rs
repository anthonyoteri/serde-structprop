//! Serde [`Deserializer`](serde::Deserializer) for the structprop format.
//!
//! The public entry point is [`from_str`].  Internally the input is first
//! parsed into a [`Value`] tree by [`parse()`], then the tree is walked
//! by a `ValueDeserializer` which implements [`serde::Deserializer`].
//!
//! # Type mapping
//!
//! | Structprop | Rust / serde |
//! |---|---|
//! | scalar `"true"` / `"false"` | `bool` |
//! | scalar integer string | `i8`–`i64`, `u8`–`u64` |
//! | scalar float string | `f32`, `f64` |
//! | scalar `"null"` | `None` / `()` |
//! | any other scalar | `String` / `&str` |
//! | `key = { … }` | `Vec<T>` / tuple |
//! | `key { … }` | struct / map |

use crate::error::{Error, Result};
use crate::parse::{parse, Value};
use indexmap::IndexMap;
use serde::de::{
    self, DeserializeOwned, DeserializeSeed, EnumAccess, IntoDeserializer, MapAccess, SeqAccess,
    VariantAccess, Visitor,
};

// ---------------------------------------------------------------------------
// Public entry point
// ---------------------------------------------------------------------------

/// Deserialize an instance of `T` from a structprop-formatted string.
///
/// The entire `input` is parsed into a [`Value`] tree first, then the tree is
/// driven through serde's visitor protocol to produce a `T`.
///
/// # Errors
///
/// Returns [`Error::Parse`] if the input is not valid structprop, or a
/// [`Error::Message`] variant if the deserialized data does not match the
/// expected shape of `T`.
///
/// # Examples
///
/// ```
/// use serde::Deserialize;
/// use serde_structprop::from_str;
///
/// #[derive(Deserialize, PartialEq, Debug)]
/// struct Config { host: String, port: u16 }
///
/// let cfg: Config = from_str("host = localhost\nport = 9000\n").unwrap();
/// assert_eq!(cfg.host, "localhost");
/// assert_eq!(cfg.port, 9000);
/// ```
pub fn from_str<T: DeserializeOwned>(input: &str) -> Result<T> {
    let value = parse(input)?;
    T::deserialize(ValueDeserializer(value))
}

// ---------------------------------------------------------------------------
// ValueDeserializer
// ---------------------------------------------------------------------------

/// A [`serde::Deserializer`] that walks a [`Value`] tree.
///
/// Created internally by [`from_str`]; not typically constructed directly.
struct ValueDeserializer(Value);

impl<'de> de::Deserializer<'de> for ValueDeserializer {
    type Error = Error;

    fn deserialize_any<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value> {
        match self.0 {
            Value::Scalar(s) => {
                // Mirror the Python implementation: try JSON-like coercions.
                if s == "null" {
                    return visitor.visit_unit();
                }
                if s == "true" {
                    return visitor.visit_bool(true);
                }
                if s == "false" {
                    return visitor.visit_bool(false);
                }
                if let Ok(n) = s.parse::<i64>() {
                    return visitor.visit_i64(n);
                }
                if let Ok(n) = s.parse::<f64>() {
                    return visitor.visit_f64(n);
                }
                visitor.visit_string(s)
            }
            Value::Array(items) => visitor.visit_seq(SeqDe::new(items)),
            Value::Object(map) => visitor.visit_map(MapDe::new(map)),
        }
    }

    fn deserialize_bool<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value> {
        match &self.0 {
            Value::Scalar(s) => match s.as_str() {
                "true" => visitor.visit_bool(true),
                "false" => visitor.visit_bool(false),
                other => Err(Error::Parse(format!("expected bool, got '{other}'"))),
            },
            _ => Err(Error::Parse("expected scalar for bool".into())),
        }
    }

    fn deserialize_i8<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value> {
        let n = parse_int(&self.0)?;
        visitor.visit_i8(
            i8::try_from(n).map_err(|_| Error::Parse(format!("value {n} out of range for i8")))?,
        )
    }

    fn deserialize_i16<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value> {
        let n = parse_int(&self.0)?;
        visitor.visit_i16(
            i16::try_from(n)
                .map_err(|_| Error::Parse(format!("value {n} out of range for i16")))?,
        )
    }

    fn deserialize_i32<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value> {
        let n = parse_int(&self.0)?;
        visitor.visit_i32(
            i32::try_from(n)
                .map_err(|_| Error::Parse(format!("value {n} out of range for i32")))?,
        )
    }

    fn deserialize_i64<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value> {
        visitor.visit_i64(parse_int(&self.0)?)
    }

    fn deserialize_u8<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value> {
        let n = parse_uint(&self.0)?;
        visitor.visit_u8(
            u8::try_from(n).map_err(|_| Error::Parse(format!("value {n} out of range for u8")))?,
        )
    }

    fn deserialize_u16<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value> {
        let n = parse_uint(&self.0)?;
        visitor.visit_u16(
            u16::try_from(n)
                .map_err(|_| Error::Parse(format!("value {n} out of range for u16")))?,
        )
    }

    fn deserialize_u32<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value> {
        let n = parse_uint(&self.0)?;
        visitor.visit_u32(
            u32::try_from(n)
                .map_err(|_| Error::Parse(format!("value {n} out of range for u32")))?,
        )
    }

    fn deserialize_u64<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value> {
        visitor.visit_u64(parse_uint(&self.0)?)
    }

    fn deserialize_f32<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value> {
        // f64 → f32 truncation is intentional and unavoidable here.
        #[allow(clippy::cast_possible_truncation)]
        visitor.visit_f32(parse_float(&self.0)? as f32)
    }

    fn deserialize_f64<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value> {
        visitor.visit_f64(parse_float(&self.0)?)
    }

    fn deserialize_char<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value> {
        if let Value::Scalar(s) = &self.0 {
            let mut chars = s.chars();
            if let Some(c) = chars.next() {
                if chars.next().is_none() {
                    return visitor.visit_char(c);
                }
            }
        }
        Err(Error::Parse("expected single-character scalar".into()))
    }

    fn deserialize_str<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value> {
        if let Value::Scalar(s) = self.0 {
            visitor.visit_string(s)
        } else {
            Err(Error::Parse("expected scalar string".into()))
        }
    }

    fn deserialize_string<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value> {
        self.deserialize_str(visitor)
    }

    fn deserialize_bytes<V: Visitor<'de>>(self, _visitor: V) -> Result<V::Value> {
        Err(Error::UnsupportedType("bytes"))
    }

    fn deserialize_byte_buf<V: Visitor<'de>>(self, _visitor: V) -> Result<V::Value> {
        Err(Error::UnsupportedType("byte_buf"))
    }

    fn deserialize_option<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value> {
        if self.0.is_null() {
            visitor.visit_none()
        } else {
            visitor.visit_some(self)
        }
    }

    fn deserialize_unit<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value> {
        if self.0.is_null() {
            visitor.visit_unit()
        } else {
            Err(Error::Parse(format!(
                "expected null, found {}",
                self.0.type_name()
            )))
        }
    }

    fn deserialize_unit_struct<V: Visitor<'de>>(
        self,
        _name: &'static str,
        visitor: V,
    ) -> Result<V::Value> {
        self.deserialize_unit(visitor)
    }

    fn deserialize_newtype_struct<V: Visitor<'de>>(
        self,
        _name: &'static str,
        visitor: V,
    ) -> Result<V::Value> {
        visitor.visit_newtype_struct(self)
    }

    fn deserialize_seq<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value> {
        match self.0 {
            Value::Array(items) => visitor.visit_seq(SeqDe::new(items)),
            _ => Err(Error::Parse("expected array value".into())),
        }
    }

    fn deserialize_tuple<V: Visitor<'de>>(self, _len: usize, visitor: V) -> Result<V::Value> {
        self.deserialize_seq(visitor)
    }

    fn deserialize_tuple_struct<V: Visitor<'de>>(
        self,
        _name: &'static str,
        _len: usize,
        visitor: V,
    ) -> Result<V::Value> {
        self.deserialize_seq(visitor)
    }

    fn deserialize_map<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value> {
        match self.0 {
            Value::Object(map) => visitor.visit_map(MapDe::new(map)),
            _ => Err(Error::Parse("expected object value".into())),
        }
    }

    fn deserialize_struct<V: Visitor<'de>>(
        self,
        _name: &'static str,
        _fields: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value> {
        self.deserialize_map(visitor)
    }

    fn deserialize_enum<V: Visitor<'de>>(
        self,
        _name: &'static str,
        _variants: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value> {
        match self.0 {
            Value::Scalar(s) => {
                // Unit variant: just the variant name as a string.
                visitor.visit_enum(s.into_deserializer())
            }
            Value::Object(map) => {
                // Newtype / tuple / struct variant: a single-entry object whose
                // key is the variant name and whose value is the payload.
                let Some((variant, payload)) = map.into_iter().next() else {
                    return Err(Error::Parse("enum object must have exactly one key".into()));
                };
                visitor.visit_enum(EnumDe { variant, payload })
            }
            Value::Array(_) => Err(Error::Parse("expected scalar or object for enum".into())),
        }
    }

    fn deserialize_identifier<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value> {
        self.deserialize_str(visitor)
    }

    fn deserialize_ignored_any<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value> {
        self.deserialize_any(visitor)
    }
}

// ---------------------------------------------------------------------------
// SeqDe – drives sequence deserialization
// ---------------------------------------------------------------------------

/// [`SeqAccess`] impl that iterates over a [`Value::Array`]'s items.
struct SeqDe {
    iter: std::vec::IntoIter<Value>,
}

impl SeqDe {
    fn new(items: Vec<Value>) -> Self {
        SeqDe {
            iter: items.into_iter(),
        }
    }
}

impl<'de> SeqAccess<'de> for SeqDe {
    type Error = Error;

    fn next_element_seed<T: DeserializeSeed<'de>>(&mut self, seed: T) -> Result<Option<T::Value>> {
        match self.iter.next() {
            None => Ok(None),
            Some(v) => seed.deserialize(ValueDeserializer(v)).map(Some),
        }
    }
}

// ---------------------------------------------------------------------------
// MapDe – drives map / struct deserialization
// ---------------------------------------------------------------------------

/// [`MapAccess`] impl that iterates over a [`Value::Object`]'s key-value pairs.
struct MapDe {
    iter: indexmap::map::IntoIter<String, Value>,
    /// The value of the most recently yielded key, waiting for `next_value_seed`.
    current_value: Option<Value>,
}

impl MapDe {
    fn new(map: IndexMap<String, Value>) -> Self {
        MapDe {
            iter: map.into_iter(),
            current_value: None,
        }
    }
}

impl<'de> MapAccess<'de> for MapDe {
    type Error = Error;

    fn next_key_seed<K: DeserializeSeed<'de>>(&mut self, seed: K) -> Result<Option<K::Value>> {
        match self.iter.next() {
            None => Ok(None),
            Some((k, v)) => {
                self.current_value = Some(v);
                seed.deserialize(ValueDeserializer(Value::Scalar(k)))
                    .map(Some)
            }
        }
    }

    fn next_value_seed<V2: DeserializeSeed<'de>>(&mut self, seed: V2) -> Result<V2::Value> {
        match self.current_value.take() {
            None => Err(Error::Parse("value missing".into())),
            Some(v) => seed.deserialize(ValueDeserializer(v)),
        }
    }
}

// ---------------------------------------------------------------------------
// EnumDe – drives enum deserialization for object-encoded variants
// ---------------------------------------------------------------------------

/// [`EnumAccess`] impl for enum variants encoded as single-key objects.
struct EnumDe {
    /// The variant name (the object's sole key).
    variant: String,
    /// The variant payload (the object's sole value).
    payload: Value,
}

impl<'de> EnumAccess<'de> for EnumDe {
    type Error = Error;
    type Variant = VariantDe;

    fn variant_seed<V: DeserializeSeed<'de>>(self, seed: V) -> Result<(V::Value, Self::Variant)> {
        let variant_val = seed.deserialize(ValueDeserializer(Value::Scalar(self.variant)))?;
        Ok((variant_val, VariantDe(self.payload)))
    }
}

/// [`VariantAccess`] impl for the payload of an object-encoded enum variant.
struct VariantDe(Value);

impl<'de> VariantAccess<'de> for VariantDe {
    type Error = Error;

    fn unit_variant(self) -> Result<()> {
        Ok(())
    }

    fn newtype_variant_seed<T: DeserializeSeed<'de>>(self, seed: T) -> Result<T::Value> {
        seed.deserialize(ValueDeserializer(self.0))
    }

    fn tuple_variant<V: Visitor<'de>>(self, _len: usize, visitor: V) -> Result<V::Value> {
        de::Deserializer::deserialize_seq(ValueDeserializer(self.0), visitor)
    }

    fn struct_variant<V: Visitor<'de>>(
        self,
        _fields: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value> {
        de::Deserializer::deserialize_map(ValueDeserializer(self.0), visitor)
    }
}

// ---------------------------------------------------------------------------
// Scalar coercion helpers
// ---------------------------------------------------------------------------

/// Parse a [`Value::Scalar`] as a signed 64-bit integer.
///
/// # Errors
///
/// Returns [`Error::Parse`] if `v` is not a scalar or not a valid `i64`.
fn parse_int(v: &Value) -> Result<i64> {
    if let Value::Scalar(s) = v {
        s.parse::<i64>()
            .map_err(|_| Error::Parse(format!("expected integer, got '{s}'")))
    } else {
        Err(Error::Parse("expected scalar for integer".into()))
    }
}

/// Parse a [`Value::Scalar`] as an unsigned 64-bit integer.
///
/// # Errors
///
/// Returns [`Error::Parse`] if `v` is not a scalar or not a valid `u64`.
fn parse_uint(v: &Value) -> Result<u64> {
    if let Value::Scalar(s) = v {
        s.parse::<u64>()
            .map_err(|_| Error::Parse(format!("expected unsigned integer, got '{s}'")))
    } else {
        Err(Error::Parse("expected scalar for unsigned integer".into()))
    }
}

/// Parse a [`Value::Scalar`] as a 64-bit float.
///
/// # Errors
///
/// Returns [`Error::Parse`] if `v` is not a scalar or not a valid `f64`.
fn parse_float(v: &Value) -> Result<f64> {
    if let Value::Scalar(s) = v {
        s.parse::<f64>()
            .map_err(|_| Error::Parse(format!("expected float, got '{s}'")))
    } else {
        Err(Error::Parse("expected scalar for float".into()))
    }
}
