//! Parser for the structprop format.
//!
//! This module contains the [`Value`] type that represents a parsed structprop
//! document and the [`parse`] function that converts a raw `&str` into a
//! [`Value::Object`] tree.
//!
//! # Grammar (informal)
//!
//! ```text
//! document   = assignment*
//! assignment = TERM '=' value
//!            | TERM '{' assignment* '}'
//! value      = TERM
//!            | '{' (TERM | '{' assignment* '}')* '}'
//! ```

use crate::error::{Error, Result};
use crate::lexer::{tokenize, Token};
use indexmap::IndexMap;

// ---------------------------------------------------------------------------
// Public types
// ---------------------------------------------------------------------------

/// A node in the structprop value tree produced by [`parse`].
///
/// The tree maps directly onto structprop's three syntactic forms:
///
/// | Structprop syntax | Variant |
/// |---|---|
/// | `key = value` | [`Value::Scalar`] |
/// | `key = { a b c }` | [`Value::Array`] |
/// | `key { … }` | [`Value::Object`] |
///
/// Scalar strings are stored verbatim; numeric or boolean coercion is
/// performed lazily via the [`Value::as_bool`], [`Value::as_i64`], and
/// [`Value::as_f64`] helpers.
#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    /// A bare or quoted string token, stored as-is (no coercion applied).
    ///
    /// Use [`Value::as_bool`], [`Value::as_i64`], or [`Value::as_f64`] to
    /// attempt type coercion, or [`Value::is_null`] to test for `null`.
    Scalar(String),

    /// An ordered list of values, corresponding to `key = { … }` syntax.
    ///
    /// Array items may themselves be [`Value::Scalar`]s or
    /// [`Value::Object`]s (the latter written as `{ key = val … }` inside the
    /// outer braces).
    Array(Vec<Value>),

    /// An ordered map from string keys to values, corresponding to either a
    /// `key { … }` block or the implicit top-level document object.
    ///
    /// Key insertion order is preserved via [`IndexMap`].
    Object(IndexMap<String, Value>),
}

// ---------------------------------------------------------------------------
// Public entry point
// ---------------------------------------------------------------------------

/// Parse a structprop document from `input` and return the top-level
/// [`Value::Object`].
///
/// # Errors
///
/// Returns [`Error::Parse`] if the input contains unexpected tokens or
/// violates the structprop grammar.
///
/// # Examples
///
/// ```
/// use serde_structprop::parse::{parse, Value};
///
/// let v = parse("port = 8080\n").unwrap();
/// if let Value::Object(map) = v {
///     assert_eq!(map["port"].as_i64(), Some(8080));
/// }
/// ```
pub fn parse(input: &str) -> Result<Value> {
    let tokens = tokenize(input);
    let mut pos = 0usize;
    let map = parse_object(&tokens, &mut pos, /*top_level=*/ true)?;
    Ok(Value::Object(map))
}

// ---------------------------------------------------------------------------
// Internal parser helpers
// ---------------------------------------------------------------------------

/// Return a reference to the token at `pos` without advancing, defaulting to
/// [`Token::Eof`] when `pos` is out of bounds.
fn peek(tokens: &[Token], pos: usize) -> &Token {
    tokens.get(pos).unwrap_or(&Token::Eof)
}

/// Advance the position cursor by one.
fn advance(pos: &mut usize) {
    *pos += 1;
}

/// Consume the next token, asserting it is a [`Token::Term`], and return its
/// string value.
///
/// # Errors
///
/// Returns [`Error::Parse`] if the next token is not a term.
fn expect_term(tokens: &[Token], pos: &mut usize) -> Result<String> {
    match tokens.get(*pos) {
        Some(Token::Term(s)) => {
            let s = s.clone();
            advance(pos);
            Ok(s)
        }
        other => Err(Error::Parse(format!("expected term, got {other:?}"))),
    }
}

/// Parse a sequence of assignments into an [`IndexMap`].
///
/// * If `top_level` is `true`, parsing stops at [`Token::Eof`].
/// * If `top_level` is `false`, parsing stops at `}` (which is consumed).
///
/// # Errors
///
/// Returns [`Error::Parse`] on malformed input.
fn parse_object(
    tokens: &[Token],
    pos: &mut usize,
    top_level: bool,
) -> Result<IndexMap<String, Value>> {
    let mut map = IndexMap::new();

    loop {
        match peek(tokens, *pos) {
            Token::Eof => {
                if top_level {
                    break;
                }
                return Err(Error::Parse("unexpected EOF inside object".to_owned()));
            }
            Token::Close => {
                if top_level {
                    return Err(Error::Parse("unexpected '}'".to_owned()));
                }
                advance(pos); // consume '}'
                break;
            }
            Token::Term(_) => {
                let key = expect_term(tokens, pos)?;
                match peek(tokens, *pos) {
                    Token::Eq => {
                        advance(pos); // consume '='
                        let val = parse_value(tokens, pos)?;
                        if map.contains_key(&key) {
                            return Err(Error::Parse(format!("duplicate key '{key}'")));
                        }
                        map.insert(key, val);
                    }
                    Token::Open => {
                        advance(pos); // consume '{'
                        let sub = parse_object(tokens, pos, /*top_level=*/ false)?;
                        if map.contains_key(&key) {
                            return Err(Error::Parse(format!("duplicate key '{key}'")));
                        }
                        map.insert(key, Value::Object(sub));
                    }
                    other => {
                        return Err(Error::Parse(format!(
                            "expected '=' or '{{' after key '{key}', got {other:?}"
                        )));
                    }
                }
            }
            other => {
                return Err(Error::Parse(format!("unexpected token {other:?}")));
            }
        }
    }

    Ok(map)
}

/// Parse a single value: either a scalar term or a `{ … }` block.
///
/// # Errors
///
/// Returns [`Error::Parse`] on unexpected tokens.
fn parse_value(tokens: &[Token], pos: &mut usize) -> Result<Value> {
    match peek(tokens, *pos) {
        Token::Open => {
            advance(pos); // consume '{'
            parse_array_or_object_list(tokens, pos)
        }
        Token::Term(_) => {
            let s = expect_term(tokens, pos)?;
            Ok(Value::Scalar(s))
        }
        other => Err(Error::Parse(format!("expected value, got {other:?}"))),
    }
}

/// Parse the body of a `{ … }` block that follows `=`.
///
/// The block may contain:
/// - A list of scalar terms → [`Value::Array`] of [`Value::Scalar`]s.
/// - A list of `{ … }` sub-objects → [`Value::Array`] of [`Value::Object`]s.
/// - A mix of both.
///
/// # Errors
///
/// Returns [`Error::Parse`] on unexpected tokens or premature EOF.
fn parse_array_or_object_list(tokens: &[Token], pos: &mut usize) -> Result<Value> {
    let mut items: Vec<Value> = Vec::new();

    loop {
        match peek(tokens, *pos) {
            Token::Close => {
                advance(pos); // consume '}'
                break;
            }
            Token::Eof => {
                return Err(Error::Parse("unexpected EOF inside array".to_owned()));
            }
            Token::Open => {
                // A nested object literal inside an array: { key = val … }
                advance(pos); // consume '{'
                let sub = parse_object(tokens, pos, /*top_level=*/ false)?;
                items.push(Value::Object(sub));
            }
            Token::Term(_) => {
                let s = expect_term(tokens, pos)?;
                items.push(Value::Scalar(s));
            }
            other @ Token::Eq => {
                return Err(Error::Parse(format!(
                    "unexpected token in array: {other:?}"
                )));
            }
        }
    }

    Ok(Value::Array(items))
}

// ---------------------------------------------------------------------------
// Scalar coercion helpers
// ---------------------------------------------------------------------------

impl Value {
    /// Try to interpret this [`Value::Scalar`] as a `bool`.
    ///
    /// Returns `Some(true)` for the literal string `"true"`, `Some(false)` for
    /// `"false"`, and `None` for any other value or non-scalar variant.
    ///
    /// This mirrors the Python implementation's `json.loads` coercion.
    #[must_use]
    pub fn as_bool(&self) -> Option<bool> {
        if let Value::Scalar(s) = self {
            match s.as_str() {
                "true" => Some(true),
                "false" => Some(false),
                _ => None,
            }
        } else {
            None
        }
    }

    /// Try to interpret this [`Value::Scalar`] as an `i64`.
    ///
    /// Returns `Some(n)` if the string parses as a signed 64-bit integer, or
    /// `None` otherwise.
    #[must_use]
    pub fn as_i64(&self) -> Option<i64> {
        if let Value::Scalar(s) = self {
            s.parse().ok()
        } else {
            None
        }
    }

    /// Try to interpret this [`Value::Scalar`] as an `f64`.
    ///
    /// Returns `Some(n)` if the string parses as a 64-bit float, or `None`
    /// otherwise.
    #[must_use]
    pub fn as_f64(&self) -> Option<f64> {
        if let Value::Scalar(s) = self {
            s.parse().ok()
        } else {
            None
        }
    }

    /// Returns `true` if this value is the scalar string `"null"`.
    ///
    /// Used by the deserializer to map structprop's `null` token to
    /// [`Option::None`].
    #[must_use]
    pub fn is_null(&self) -> bool {
        matches!(self, Value::Scalar(s) if s == "null")
    }

    /// Returns a short human-readable name for the variant, used in error
    /// messages.
    #[must_use]
    pub fn type_name(&self) -> &'static str {
        match self {
            Value::Scalar(_) => "scalar",
            Value::Array(_) => "array",
            Value::Object(_) => "object",
        }
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_kv() {
        let v = parse("key = value\n").unwrap();
        if let Value::Object(map) = v {
            assert_eq!(map["key"], Value::Scalar("value".into()));
        } else {
            panic!("expected object");
        }
    }

    #[test]
    fn nested_object() {
        let input = "db {\n  host = localhost\n  port = 5432\n}\n";
        let v = parse(input).unwrap();
        if let Value::Object(map) = v {
            if let Value::Object(db) = &map["db"] {
                assert_eq!(db["host"], Value::Scalar("localhost".into()));
                assert_eq!(db["port"], Value::Scalar("5432".into()));
            } else {
                panic!("expected nested object");
            }
        } else {
            panic!("expected object");
        }
    }

    #[test]
    fn array_of_scalars() {
        let input = "tables = { Table1 Table2 }\n";
        let v = parse(input).unwrap();
        if let Value::Object(map) = v {
            assert_eq!(
                map["tables"],
                Value::Array(vec![
                    Value::Scalar("Table1".into()),
                    Value::Scalar("Table2".into()),
                ])
            );
        } else {
            panic!("expected object");
        }
    }

    #[test]
    fn number_scalar() {
        let v = parse("port = 8080\n").unwrap();
        if let Value::Object(map) = v {
            assert_eq!(map["port"].as_i64(), Some(8080));
        }
    }

    #[test]
    fn bool_scalar() {
        let v = parse("enabled = true\n").unwrap();
        if let Value::Object(map) = v {
            assert_eq!(map["enabled"].as_bool(), Some(true));
        }
    }
}
