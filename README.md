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

**Special characters** in values (spaces, tabs, `#`, `{`, `}`, `=`) must be
wrapped in double quotes.  Keys follow the same rule.

## Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
serde = { version = "1", features = ["derive"] }
serde-structprop = "0.1"
```

## Type mapping

| Rust / serde type | Structprop representation |
|---|---|
| `bool` | `true` or `false` |
| integer / float | bare numeric scalar |
| `String` / `&str` | bare scalar, or `"quoted"` when it contains special chars |
| `Option<T>` (Some) | the inner value |
| `Option<T>` (None) / `()` | `null` |
| struct / map | `key { … }` block |
| `Vec<T>` / sequence | `key = { … }` list |
| unit enum variant | bare variant name |
| newtype / tuple / struct enum variant | `variant_name { … }` block |
| raw `&[u8]` | **unsupported** — returns `Error::UnsupportedType` |

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

Values containing spaces or structprop special characters (`#`, `{`, `}`, `=`)
must be quoted in the input and are quoted automatically on output:

```rust
use serde::{Deserialize, Serialize};
use serde_structprop::{from_str, to_string};

#[derive(Debug, Serialize, Deserialize)]
struct S {
    message: String,
}

fn main() {
    let s: S = from_str(r#"message = "hello world""#).unwrap();
    assert_eq!(s.message, "hello world");

    let out = to_string(&s).unwrap();
    assert_eq!(out, "message = \"hello world\"\n");
}
```

### Optional fields

```rust
use serde::{Deserialize, Serialize};
use serde_structprop::from_str;

#[derive(Debug, Deserialize)]
struct S {
    required: String,
    optional: Option<u32>,
}

fn main() {
    let s: S = from_str("required = hello\noptional = 42\n").unwrap();
    assert_eq!(s.optional, Some(42));
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
    Err(Error::Parse(msg))   => eprintln!("syntax error: {msg}"),
    Err(Error::Message(msg)) => eprintln!("type error: {msg}"),
    Err(e)                   => eprintln!("other error: {e}"),
}
```

### Error variants

| Variant | When |
|---|---|
| `Error::Parse(String)` | Lexer or parser encountered unexpected input |
| `Error::Message(String)` | serde type mismatch (e.g. string where integer expected) |
| `Error::UnsupportedType(&str)` | Type has no structprop equivalent (e.g. `&[u8]`) |
| `Error::KeyMustBeString` | A map was serialized with a non-string key |

## Module layout

| Module | Contents |
|---|---|
| `serde_structprop::de` | `Deserializer` implementation; `from_str` entry point |
| `serde_structprop::ser` | `Serializer` implementation; `to_string` entry point |
| `serde_structprop::parse` | Recursive-descent parser; `Value` AST enum |
| `serde_structprop::lexer` | Tokenizer; `Token` enum |
| `serde_structprop::error` | `Error` enum and `Result<T>` alias |

## License

Licensed under either of

- [MIT license](LICENSE-MIT)
- [Apache License, Version 2.0](LICENSE-APACHE)

at your option.
