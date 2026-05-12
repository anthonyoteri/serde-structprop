# structprop-validator

[![CI](https://github.com/anthonyoteri/serde-structprop/actions/workflows/ci.yml/badge.svg)](https://github.com/anthonyoteri/serde-structprop/actions/workflows/ci.yml)
[![crates.io](https://img.shields.io/crates/v/structprop-validator.svg)](https://crates.io/crates/structprop-validator)
[![Rust version](https://img.shields.io/badge/rustc-1.85+-orange.svg)](https://blog.rust-lang.org/2025/02/20/Rust-1.85.0.html)
[![License](https://img.shields.io/badge/license-MIT%20OR%20Apache--2.0-blue.svg)](LICENSE-MIT)

A command-line tool that validates [structprop](https://github.com/edgeware/structprop)
configuration files, reporting parse errors with line numbers.

For the Rust library (serde serializer/deserializer), see
[serde-structprop](https://crates.io/crates/serde-structprop).

## Installation

```sh
cargo install structprop-validator
```

## Usage

```text
structprop-validator [FILE]...
```

Pass one or more file paths to validate them.  With no arguments the tool
reads from standard input.

```sh
# Validate a single file
structprop-validator config.sp

# Validate several files at once
structprop-validator *.sp

# Pipe input from another command
cat config.sp | structprop-validator
```

### Output

Each file that parses successfully prints a confirmation line:

```text
config.sp: ok
```

A file with errors prints the error to stderr and exits with status 1:

```text
'config.sp' is not valid structprop: parse error: line 3: 'host = ...' is not valid inside an array; wrap it in braces for a nested object: '{ host = ... }'
```

### Exit status

| Code | Meaning |
|------|---------|
| `0` | All inputs are valid |
| `1` | One or more inputs contain errors |

## Format overview

Structprop files consist of scalar key-value pairs, nested object blocks,
and arrays:

```text
# Scalar
key = value
quoted = "value with spaces"

# Nested object
section {
  host = localhost
  port = 5432
}

# Array of scalars
tables = { users orders products }

# Array of objects
servers = {
  { host = a port = 1 }
  { host = b port = 2 }
}
```

See the [serde-structprop README](https://github.com/anthonyoteri/serde-structprop)
for the full format specification.

## License

Licensed under either of

- [MIT license](https://github.com/anthonyoteri/serde-structprop/blob/main/LICENSE-MIT)
- [Apache License, Version 2.0](https://github.com/anthonyoteri/serde-structprop/blob/main/LICENSE-APACHE)

at your option.
