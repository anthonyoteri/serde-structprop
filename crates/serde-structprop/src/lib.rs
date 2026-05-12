//! # serde-structprop
//!
//! A [serde](https://serde.rs/) serializer and deserializer for the
//! [structprop](https://github.com/edgeware/structprop) configuration file
//! format — a simple, human-readable format for structured data.
//!
//! ## Format overview
//!
//! ```text
//! # comment
//! key = value
//! key = "value with spaces"
//! key = 42
//! key = true
//!
//! # nested object
//! section {
//!     nested_key = value
//! }
//!
//! # array of scalars
//! list = { a b c }
//! ```
//!
//! ## Quick start
//!
//! ```rust
//! use serde::{Deserialize, Serialize};
//! use serde_structprop::{from_str, to_string};
//!
//! #[derive(Debug, Serialize, Deserialize, PartialEq)]
//! struct Config {
//!     hostname: String,
//!     port: u16,
//! }
//!
//! // Deserialize
//! let input = "hostname = localhost\nport = 8080\n";
//! let cfg: Config = from_str(input).unwrap();
//! assert_eq!(cfg.hostname, "localhost");
//! assert_eq!(cfg.port, 8080);
//!
//! // Serialize
//! let out = to_string(&cfg).unwrap();
//! assert!(out.contains("hostname = localhost"));
//! assert!(out.contains("port = 8080"));
//! ```
//!
//! ## Module layout
//!
//! | Module | Contents |
//! |---|---|
//! | [`lexer`] | Tokenizer: converts raw text to `Token`s |
//! | [`mod@parse`] | Recursive-descent parser: produces a [`parse::Value`] tree |
//! | [`de`] | `serde::Deserializer` implementation |
//! | [`ser`] | `serde::Serializer` implementation |
//! | [`error`] | [`Error`] type shared by all modules |

#![deny(clippy::all, clippy::pedantic)]
#![deny(missing_docs)]

/// Serde deserializer for structprop documents.
pub mod de;
/// Error type shared by the serializer and deserializer.
pub mod error;
/// Lexer (tokenizer) that converts raw structprop text into tokens.
pub mod lexer;
/// Parser that converts a token stream into a [`parse::Value`] tree.
pub mod parse;
/// Serde serializer for structprop documents.
pub mod ser;

pub use de::from_str;
pub use error::{Error, Result};
pub use parse::{parse, Value};
pub use ser::to_string;
