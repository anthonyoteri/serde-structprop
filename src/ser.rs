//! Serde [`Serializer`](serde::Serializer) for the structprop format.
//!
//! The public entry point is [`to_string`].  Internally a [`Serializer`] walks
//! the serde data model and writes structprop text directly into an output
//! [`String`].
//!
//! # Type mapping
//!
//! | Rust / serde | Structprop output |
//! |---|---|
//! | `bool` | `true` or `false` scalar |
//! | integer / float | numeric scalar |
//! | `String` / `&str` | bare scalar or `"quoted"` if it contains special chars |
//! | `None` / `()` | `null` scalar |
//! | struct / map | `key { … }` block at the current indentation level |
//! | `Vec<T>` / sequence | `= { … }` inline list |
//! | unit enum variant | bare variant name scalar |
//! | newtype / tuple / struct enum variant | `variant_name { … }` block |

use std::fmt::Write as FmtWrite;

use crate::error::{Error, Result};
use serde::ser::{self, Serialize};

// ---------------------------------------------------------------------------
// Public entry point
// ---------------------------------------------------------------------------

/// Serialize `value` into a structprop-formatted [`String`].
///
/// Top-level structs and maps produce a flat sequence of `key = value` /
/// `key { … }` entries with no enclosing braces.  Sequences produce a bare
/// `{\n … \n}` block.
///
/// # Errors
///
/// Returns [`Error::UnsupportedType`] if `T` contains raw byte slices, or
/// [`Error::Message`] for any other serde-level serialization error.
///
/// # Examples
///
/// ```
/// use serde::Serialize;
/// use serde_structprop::to_string;
///
/// #[derive(Serialize)]
/// struct Cfg { host: String, port: u16 }
///
/// let s = to_string(&Cfg { host: "localhost".into(), port: 8080 }).unwrap();
/// assert!(s.contains("host = localhost"));
/// assert!(s.contains("port = 8080"));
/// ```
pub fn to_string<T: Serialize>(value: &T) -> Result<String> {
    let mut ser = Serializer::new(0);
    value.serialize(&mut ser)?;
    Ok(ser.output)
}

// ---------------------------------------------------------------------------
// Serializer
// ---------------------------------------------------------------------------

/// Core serializer that accumulates structprop text in `output`.
///
/// `indent` is the current nesting depth; each level adds two spaces of
/// leading whitespace to each emitted line.
pub struct Serializer {
    /// The accumulated output string.
    pub(crate) output: String,
    /// Current indentation level in spaces.
    indent: usize,
}

impl Serializer {
    /// Create a new [`Serializer`] starting at `indent` spaces of indentation.
    fn new(indent: usize) -> Self {
        Serializer {
            output: String::new(),
            indent,
        }
    }

    /// Return a string of `self.indent` spaces.
    fn pad(&self) -> String {
        " ".repeat(self.indent)
    }
}

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

/// Wrap `s` in double quotes if it contains any structprop special characters
/// (space, tab, newline, carriage return, `#`, `{`, `}`, or `=`); otherwise
/// return it unchanged.
///
/// Note: the structprop format has no escape sequences, so embedded `"` characters
/// inside a value are not representable. Values containing `"` are returned
/// wrapped in quotes with the inner `"` intact — callers should avoid such values
/// or the round-trip will be lossy.
fn escape(s: &str) -> String {
    if s.chars()
        .any(|c| matches!(c, ' ' | '\t' | '\n' | '\r' | '#' | '{' | '}' | '='))
    {
        format!("\"{s}\"")
    } else {
        s.to_owned()
    }
}

// ---------------------------------------------------------------------------
// serde::Serializer impl
// ---------------------------------------------------------------------------

impl<'a> ser::Serializer for &'a mut Serializer {
    type Ok = ();
    type Error = Error;

    /// Compound serializer types.
    type SerializeSeq = SeqSerializer<'a>;
    /// Compound serializer types.
    type SerializeTuple = SeqSerializer<'a>;
    /// Compound serializer types.
    type SerializeTupleStruct = SeqSerializer<'a>;
    /// Compound serializer types.
    type SerializeTupleVariant = SeqSerializer<'a>;
    /// Compound serializer types.
    type SerializeMap = MapSerializer<'a>;
    /// Compound serializer types.
    type SerializeStruct = MapSerializer<'a>;
    /// Compound serializer types.
    type SerializeStructVariant = MapSerializer<'a>;

    fn serialize_bool(self, v: bool) -> Result<()> {
        self.output.push_str(if v { "true" } else { "false" });
        Ok(())
    }

    fn serialize_i8(self, v: i8) -> Result<()> {
        write!(self.output, "{v}").map_err(|e| Error::Message(e.to_string()))
    }

    fn serialize_i16(self, v: i16) -> Result<()> {
        write!(self.output, "{v}").map_err(|e| Error::Message(e.to_string()))
    }

    fn serialize_i32(self, v: i32) -> Result<()> {
        write!(self.output, "{v}").map_err(|e| Error::Message(e.to_string()))
    }

    fn serialize_i64(self, v: i64) -> Result<()> {
        write!(self.output, "{v}").map_err(|e| Error::Message(e.to_string()))
    }

    fn serialize_u8(self, v: u8) -> Result<()> {
        write!(self.output, "{v}").map_err(|e| Error::Message(e.to_string()))
    }

    fn serialize_u16(self, v: u16) -> Result<()> {
        write!(self.output, "{v}").map_err(|e| Error::Message(e.to_string()))
    }

    fn serialize_u32(self, v: u32) -> Result<()> {
        write!(self.output, "{v}").map_err(|e| Error::Message(e.to_string()))
    }

    fn serialize_u64(self, v: u64) -> Result<()> {
        write!(self.output, "{v}").map_err(|e| Error::Message(e.to_string()))
    }

    fn serialize_f32(self, v: f32) -> Result<()> {
        write!(self.output, "{v}").map_err(|e| Error::Message(e.to_string()))
    }

    fn serialize_f64(self, v: f64) -> Result<()> {
        write!(self.output, "{v}").map_err(|e| Error::Message(e.to_string()))
    }

    fn serialize_char(self, v: char) -> Result<()> {
        // Route through escape() so special characters are quoted.
        self.output.push_str(&escape(&v.to_string()));
        Ok(())
    }

    fn serialize_str(self, v: &str) -> Result<()> {
        self.output.push_str(&escape(v));
        Ok(())
    }

    fn serialize_bytes(self, _v: &[u8]) -> Result<()> {
        Err(Error::UnsupportedType("bytes"))
    }

    fn serialize_none(self) -> Result<()> {
        self.output.push_str("null");
        Ok(())
    }

    fn serialize_some<T: Serialize + ?Sized>(self, value: &T) -> Result<()> {
        value.serialize(self)
    }

    fn serialize_unit(self) -> Result<()> {
        self.output.push_str("null");
        Ok(())
    }

    fn serialize_unit_struct(self, _name: &'static str) -> Result<()> {
        self.serialize_unit()
    }

    fn serialize_unit_variant(
        self,
        _name: &'static str,
        _index: u32,
        variant: &'static str,
    ) -> Result<()> {
        self.serialize_str(variant)
    }

    fn serialize_newtype_struct<T: Serialize + ?Sized>(
        self,
        _name: &'static str,
        value: &T,
    ) -> Result<()> {
        value.serialize(self)
    }

    fn serialize_newtype_variant<T: Serialize + ?Sized>(
        self,
        _name: &'static str,
        _index: u32,
        variant: &'static str,
        value: &T,
    ) -> Result<()> {
        // Encode as `variant = <payload>` (scalar) or `variant { … }` (object block)
        // so the parser produces Object({"variant": payload}), which is exactly what
        // deserialize_enum expects for newtype variants.
        let mut ms = MapSerializer {
            ser: self,
            current_key: None,
            variant_name: None,
        };
        ms.write_kv(variant, value)
    }

    fn serialize_seq(self, _len: Option<usize>) -> Result<Self::SerializeSeq> {
        Ok(SeqSerializer {
            parent: self,
            items: Vec::new(),
            variant: None,
        })
    }

    fn serialize_tuple(self, len: usize) -> Result<Self::SerializeTuple> {
        self.serialize_seq(Some(len))
    }

    fn serialize_tuple_struct(
        self,
        _name: &'static str,
        len: usize,
    ) -> Result<Self::SerializeTupleStruct> {
        self.serialize_seq(Some(len))
    }

    fn serialize_tuple_variant(
        self,
        _name: &'static str,
        _index: u32,
        variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleVariant> {
        Ok(SeqSerializer {
            parent: self,
            items: Vec::new(),
            variant: Some(variant.to_owned()),
        })
    }

    fn serialize_map(self, _len: Option<usize>) -> Result<Self::SerializeMap> {
        Ok(MapSerializer {
            ser: self,
            current_key: None,
            variant_name: None,
        })
    }

    fn serialize_struct(self, _name: &'static str, len: usize) -> Result<Self::SerializeStruct> {
        self.serialize_map(Some(len))
    }

    fn serialize_struct_variant(
        self,
        _name: &'static str,
        _index: u32,
        variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStructVariant> {
        Ok(MapSerializer {
            ser: self,
            current_key: None,
            variant_name: Some(variant.to_owned()),
        })
    }
}

// ---------------------------------------------------------------------------
// SeqSerializer – handles sequences / arrays
// ---------------------------------------------------------------------------

/// [`ser::SerializeSeq`] (and related tuple impls) that collects rendered items
/// then emits them as a `{ … }` block.
pub struct SeqSerializer<'a> {
    parent: &'a mut Serializer,
    /// Each element serialized to a string, accumulated for deferred emission.
    items: Vec<String>,
    /// Set for tuple variants: the variant name to wrap the array under.
    variant: Option<String>,
}

impl ser::SerializeSeq for SeqSerializer<'_> {
    type Ok = ();
    type Error = Error;

    fn serialize_element<T: Serialize + ?Sized>(&mut self, value: &T) -> Result<()> {
        let mut inner = Serializer::new(0);
        value.serialize(&mut inner)?;
        self.items.push(inner.output);
        Ok(())
    }

    fn end(self) -> Result<()> {
        let pad = self.parent.pad();
        let inner_pad = " ".repeat(self.parent.indent + 2);
        writeln!(self.parent.output, "{{").map_err(|e| Error::Message(e.to_string()))?;
        for item in &self.items {
            writeln!(self.parent.output, "{inner_pad}{item}")
                .map_err(|e| Error::Message(e.to_string()))?;
        }
        writeln!(self.parent.output, "{pad}}}").map_err(|e| Error::Message(e.to_string()))
    }
}

impl ser::SerializeTuple for SeqSerializer<'_> {
    type Ok = ();
    type Error = Error;

    fn serialize_element<T: Serialize + ?Sized>(&mut self, value: &T) -> Result<()> {
        ser::SerializeSeq::serialize_element(self, value)
    }

    fn end(self) -> Result<()> {
        ser::SerializeSeq::end(self)
    }
}

impl ser::SerializeTupleStruct for SeqSerializer<'_> {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T: Serialize + ?Sized>(&mut self, value: &T) -> Result<()> {
        ser::SerializeSeq::serialize_element(self, value)
    }

    fn end(self) -> Result<()> {
        ser::SerializeSeq::end(self)
    }
}

impl ser::SerializeTupleVariant for SeqSerializer<'_> {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T: Serialize + ?Sized>(&mut self, value: &T) -> Result<()> {
        ser::SerializeSeq::serialize_element(self, value)
    }

    fn end(self) -> Result<()> {
        // Emit as `variant = { item1\n  item2\n … }` so the parser produces
        // Object({"variant": Array([…])}), matching what deserialize_enum expects.
        let variant = self
            .variant
            .expect("variant name must be set for SerializeTupleVariant");
        let pad = self.parent.pad();
        let inner_pad = " ".repeat(self.parent.indent + 2);
        writeln!(self.parent.output, "{pad}{} = {{", escape(&variant))
            .map_err(|e| Error::Message(e.to_string()))?;
        for item in &self.items {
            writeln!(self.parent.output, "{inner_pad}{item}")
                .map_err(|e| Error::Message(e.to_string()))?;
        }
        writeln!(self.parent.output, "{pad}}}").map_err(|e| Error::Message(e.to_string()))
    }
}

// ---------------------------------------------------------------------------
// MapSerializer – handles maps and structs
// ---------------------------------------------------------------------------

/// [`ser::SerializeMap`] (and struct impls) that writes `key = value` /
/// `key { … }` entries one at a time.
pub struct MapSerializer<'a> {
    ser: &'a mut Serializer,
    /// The most recently serialized key, waiting for its value.
    current_key: Option<String>,
    /// If `Some`, wrap all emitted content in a `variant_name { … }` block.
    variant_name: Option<String>,
}

impl MapSerializer<'_> {
    /// Serialize a single `key = value` or `key { … }` entry into `self.ser`.
    fn write_kv<T: Serialize + ?Sized>(&mut self, key: &str, value: &T) -> Result<()> {
        let indent = self.ser.indent;
        let pad = " ".repeat(indent);

        // Render at indent=0 so SeqSerializer / MapSerializer do not add their
        // own base indentation; we re-apply `pad` below.
        let mut inner = Serializer::new(0);
        value.serialize(&mut inner)?;
        let rendered = inner.output;

        if rendered.contains('\n')
            && !rendered.trim_start().starts_with('{')
            && !rendered.trim_start().starts_with('"')
        {
            // Multi-line object block → `key {\n  …lines re-indented…\n}\n`
            writeln!(self.ser.output, "{pad}{} {{", escape(key))
                .map_err(|e| Error::Message(e.to_string()))?;
            for line in rendered.lines() {
                writeln!(self.ser.output, "{pad}  {line}")
                    .map_err(|e| Error::Message(e.to_string()))?;
            }
            writeln!(self.ser.output, "{pad}}}").map_err(|e| Error::Message(e.to_string()))?;
        } else if rendered.contains('\n') {
            // Multi-line array block starting with `{`.
            // First line (`{`) goes inline with `key = `; remaining lines get
            // `pad` prepended so items sit at `indent+2` and `}` at `indent`.
            let mut lines = rendered.lines();
            let first = lines.next().unwrap_or("{");
            writeln!(self.ser.output, "{pad}{} = {first}", escape(key))
                .map_err(|e| Error::Message(e.to_string()))?;
            for line in lines {
                writeln!(self.ser.output, "{pad}{line}")
                    .map_err(|e| Error::Message(e.to_string()))?;
            }
        } else {
            // Scalar (no newlines at all).
            let rendered = rendered.trim_end_matches('\n');
            writeln!(self.ser.output, "{pad}{} = {rendered}", escape(key))
                .map_err(|e| Error::Message(e.to_string()))?;
        }
        Ok(())
    }
}

impl ser::SerializeMap for MapSerializer<'_> {
    type Ok = ();
    type Error = Error;

    fn serialize_key<T: Serialize + ?Sized>(&mut self, key: &T) -> Result<()> {
        let mut k = Serializer::new(0);
        key.serialize(&mut k)?;
        self.current_key = Some(k.output);
        Ok(())
    }

    fn serialize_value<T: Serialize + ?Sized>(&mut self, value: &T) -> Result<()> {
        let key = self
            .current_key
            .take()
            .ok_or_else(|| Error::Parse("serialize_value called before serialize_key".into()))?;
        self.write_kv(&key, value)
    }

    fn end(self) -> Result<()> {
        if let Some(variant) = self.variant_name {
            // Wrap the content we already wrote in `variant { … }`.
            let pad = " ".repeat(self.ser.indent.saturating_sub(2));
            let header = format!("{pad}{} {{\n", escape(&variant));
            let footer = format!("{pad}}}\n");
            self.ser.output.insert_str(0, &header);
            self.ser.output.push_str(&footer);
        }
        Ok(())
    }
}

impl ser::SerializeStruct for MapSerializer<'_> {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T: Serialize + ?Sized>(
        &mut self,
        key: &'static str,
        value: &T,
    ) -> Result<()> {
        self.write_kv(key, value)
    }

    fn end(self) -> Result<()> {
        ser::SerializeMap::end(self)
    }
}

impl ser::SerializeStructVariant for MapSerializer<'_> {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T: Serialize + ?Sized>(
        &mut self,
        key: &'static str,
        value: &T,
    ) -> Result<()> {
        self.write_kv(key, value)
    }

    fn end(self) -> Result<()> {
        ser::SerializeMap::end(self)
    }
}
