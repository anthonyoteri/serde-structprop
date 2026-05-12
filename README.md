# serde-structprop

[![CI](https://github.com/anthonyoteri/serde-structprop/actions/workflows/ci.yml/badge.svg)](https://github.com/anthonyoteri/serde-structprop/actions/workflows/ci.yml)
[![crates.io](https://img.shields.io/crates/v/serde-structprop.svg)](https://crates.io/crates/serde-structprop)
[![docs.rs](https://docs.rs/serde-structprop/badge.svg)](https://docs.rs/serde-structprop)
[![Rust version](https://img.shields.io/badge/rustc-1.85+-orange.svg)](https://blog.rust-lang.org/2025/02/20/Rust-1.85.0.html)
[![License](https://img.shields.io/badge/license-MIT%20OR%20Apache--2.0-blue.svg)](LICENSE-MIT)

A [serde](https://serde.rs/) serializer and deserializer for the
[structprop](https://github.com/edgeware/structprop) configuration file format —
a simple, human-readable format for structured data.

## Format overview

Structprop files are composed of three constructs:

```text
# Lines beginning with # are comments (inline comments are also supported)

# Scalar key-value pair
key = value
key = "value with spaces or special chars"
key = 42
key = -7
key = true

# Nested object block
section {
  nested_key = value
  another    = 123
}

# Array of scalars
list = { a b c }
list = {
  a
  b
  c
}
```

**Special characters** in values (spaces, tabs, newlines, carriage returns,
`#`, `{`, `}`, `=`) must be wrapped in double quotes.  Empty strings are
always quoted as `""`.  Keys follow the same rule.

## Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
serde-structprop = { version = "0.1", features = ["derive"] }
```

The `derive` feature enables serde's own derive macros.  You still need a
direct `serde` dependency in your crate so that `Serialize` and `Deserialize`
are in scope.  If you already depend on `serde` with `features = ["derive"]`,
you can omit the feature flag here:

```toml
[dependencies]
serde = { version = "1", features = ["derive"] }
serde-structprop = "0.1"
```

## Type mapping

| Rust / serde type | Structprop representation |
|---|---|
| `bool` | `true` or `false` |
| `i8` `i16` `i32` `i64` `u8` `u16` `u32` `u64` | bare integer scalar (e.g. `42`, `-7`) |
| `f32`, `f64` | bare float scalar (e.g. `3.14`) |
| `char` | bare single-character scalar |
| `String` / `&str` | bare scalar, or `"quoted"` when it contains special chars or is empty |
| `Option<T>` (Some) | the inner value serialized normally |
| `Option<T>` (None) / `()` | `null` |
| newtype struct (e.g. `struct Meters(f64)`) | transparent — serializes as the inner type |
| unit struct (e.g. `struct Marker;`) | `null` |
| struct / map | `key { … }` block |
| `Vec<T>` / sequence | `key = { … }` list |
| tuple / tuple struct | `key = { … }` list of elements |
| unit enum variant | bare variant name |
| newtype enum variant | `variant_name = <scalar or list>` |
| tuple enum variant | `variant_name = { … }` list |
| struct enum variant | `variant_name { … }` block |
| raw bytes (`serialize_bytes` / `deserialize_bytes`) | **unsupported** — returns `Error::UnsupportedType` |

## Quick start

### Deserializing

```rust
use serde::Deserialize;
use serde_structprop::from_str;

#[derive(Debug, Deserialize)]
struct Config {
    hostname: String,
    port: u16,
    debug: bool,
}

fn main() {
    let input = "
        # server config
        hostname = localhost
        port     = 8080
        debug    = true
    ";

    let cfg: Config = from_str(input).unwrap();
    println!("{cfg:?}");
    // Config { hostname: "localhost", port: 8080, debug: true }
}
```

### Serializing

```rust
use serde::Serialize;
use serde_structprop::to_string;

#[derive(Serialize)]
struct Config {
    hostname: String,
    port: u16,
    debug: bool,
}

fn main() {
    let cfg = Config {
        hostname: "localhost".into(),
        port: 8080,
        debug: true,
    };

    let out = to_string(&cfg).unwrap();
    println!("{out}");
    // hostname = localhost
    // port = 8080
    // debug = true
}
```

### Nested structs

```rust
use serde::{Deserialize, Serialize};
use serde_structprop::{from_str, to_string};

#[derive(Debug, Serialize, Deserialize, PartialEq)]
struct Database {
    hostname: String,
    port: u16,
    name: String,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
struct Config {
    database: Database,
    tables: Vec<String>,
}

fn main() {
    let input = "
        database {
          hostname = db.example.com
          port     = 5432
          name     = myapp
        }

        tables = { users orders products }
    ";

    let cfg: Config = from_str(input).unwrap();
    assert_eq!(cfg.database.port, 5432);
    assert_eq!(cfg.tables, vec!["users", "orders", "products"]);

    // Round-trip back to structprop text
    let out = to_string(&cfg).unwrap();
    println!("{out}");
    // database {
    //   hostname = db.example.com
    //   port = 5432
    //   name = myapp
    // }
    // tables = {
    //   users
    //   orders
    //   products
    // }
}
```

### Quoted values

Values containing spaces, tabs, newlines, or the special characters
(`#`, `{`, `}`, `=`) are quoted automatically on output and must be
quoted in the input.  Empty strings are always quoted as `""`:

```rust
use serde::{Deserialize, Serialize};
use serde_structprop::{from_str, to_string};

#[derive(Debug, Serialize, Deserialize)]
struct S {
    message: String,
    empty: String,
}

fn main() {
    let s: S = from_str(r#"message = "hello world"
empty = """#).unwrap();
    assert_eq!(s.message, "hello world");
    assert_eq!(s.empty, "");

    let out = to_string(&s).unwrap();
    assert_eq!(out, "message = \"hello world\"\nempty = \"\"\n");
}
```

### Optional fields

Fields typed as `Option<T>` are serialized as `null` when `None` and as
the inner value when `Some`.  A missing key in the input also deserializes
as `None`:

```rust
use serde::{Deserialize, Serialize};
use serde_structprop::{from_str, to_string};

#[derive(Debug, Serialize, Deserialize, PartialEq)]
struct S {
    required: String,
    optional: Option<u32>,
}

fn main() {
    // Some value
    let s: S = from_str("required = hello\noptional = 42\n").unwrap();
    assert_eq!(s.optional, Some(42));

    // Explicit null
    let s: S = from_str("required = hello\noptional = null\n").unwrap();
    assert_eq!(s.optional, None);

    // Missing key also deserializes as None
    let s: S = from_str("required = hello\n").unwrap();
    assert_eq!(s.optional, None);

    // None serializes as null
    let out = to_string(&S { required: "hello".into(), optional: None }).unwrap();
    assert!(out.contains("optional = null"));
}
```

## Error handling

All functions return `serde_structprop::Result<T>`, an alias for
`std::result::Result<T, serde_structprop::Error>`.

```rust
use serde_structprop::{from_str, Error};

#[derive(serde::Deserialize)]
struct S { x: u32 }

match from_str::<S>("x = not_a_number\n") {
    Ok(s)                    => println!("x = {}", s.x),
    Err(Error::Parse(msg))   => eprintln!("parse error: {msg}"),
    Err(Error::Message(msg)) => eprintln!("serde error: {msg}"),
    Err(e)                   => eprintln!("other error: {e}"),
}
```

### Error variants

| Variant | When |
|---|---|
| `Error::Parse(String)` | Lexer or parser encountered unexpected input, or a scalar could not be coerced to the requested numeric type |
| `Error::Message(String)` | serde-generated error (e.g. missing required field, unknown variant) |
| `Error::UnsupportedType(&'static str)` | Type has no structprop equivalent (e.g. raw byte slices) |
| `Error::KeyMustBeString` | A map was serialized with a non-string key |

## Module layout

| Module | Contents |
|---|---|
| `serde_structprop::lexer` | Tokenizer: converts raw text to `Token`s |
| `serde_structprop::parse` | Recursive-descent parser: produces a `Value` tree |
| `serde_structprop::de` | `serde::Deserializer` implementation; `from_str` entry point |
| `serde_structprop::ser` | `serde::Serializer` implementation; `to_string` entry point |
| `serde_structprop::error` | `Error` enum and `Result<T>` alias |

## License

Licensed under either of

- [MIT license](LICENSE-MIT)
- [Apache License, Version 2.0](LICENSE-APACHE)

at your option.
