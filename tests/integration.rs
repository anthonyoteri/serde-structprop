use serde::{Deserialize, Serialize};
use serde_structprop::{from_str, to_string};

// ---------------------------------------------------------------------------
// Deserialization tests
// ---------------------------------------------------------------------------

#[derive(Debug, Deserialize, PartialEq)]
struct Simple {
    hostname: String,
    port: u16,
}

#[test]
fn de_simple_struct() {
    let input = "hostname = localhost\nport = 8080\n";
    let cfg: Simple = from_str(input).unwrap();
    assert_eq!(cfg.hostname, "localhost");
    assert_eq!(cfg.port, 8080);
}

#[test]
fn de_quoted_value() {
    #[derive(Debug, Deserialize, PartialEq)]
    struct S {
        message: String,
    }
    let input = r#"message = "hello world""#;
    let s: S = from_str(input).unwrap();
    assert_eq!(s.message, "hello world");
}

#[test]
fn de_nested_struct() {
    #[derive(Debug, Deserialize, PartialEq)]
    struct Database {
        hostname: String,
        port: u16,
    }
    #[derive(Debug, Deserialize, PartialEq)]
    struct Config {
        database: Database,
    }

    let input = "database {\n  hostname = localhost\n  port = 5432\n}\n";
    let cfg: Config = from_str(input).unwrap();
    assert_eq!(cfg.database.hostname, "localhost");
    assert_eq!(cfg.database.port, 5432);
}

#[test]
fn de_vec_of_strings() {
    #[derive(Debug, Deserialize, PartialEq)]
    struct S {
        tables: Vec<String>,
    }
    let input = "tables = { Table1 Table2 Table3 }\n";
    let s: S = from_str(input).unwrap();
    assert_eq!(s.tables, vec!["Table1", "Table2", "Table3"]);
}

#[test]
fn de_vec_of_integers() {
    #[derive(Debug, Deserialize, PartialEq)]
    struct S {
        ports: Vec<u16>,
    }
    let input = "ports = { 80 443 8080 }\n";
    let s: S = from_str(input).unwrap();
    assert_eq!(s.ports, vec![80u16, 443, 8080]);
}

#[test]
fn de_bool_field() {
    #[derive(Debug, Deserialize, PartialEq)]
    struct S {
        enabled: bool,
    }
    let input = "enabled = true\n";
    let s: S = from_str(input).unwrap();
    assert!(s.enabled);
}

#[test]
fn de_option_some() {
    #[derive(Debug, Deserialize, PartialEq)]
    struct S {
        value: Option<u32>,
    }
    let input = "value = 42\n";
    let s: S = from_str(input).unwrap();
    assert_eq!(s.value, Some(42));
}

#[test]
fn de_comments_ignored() {
    let input = "# This is a comment\nhostname = server\nport = 9000\n";
    let cfg: Simple = from_str(input).unwrap();
    assert_eq!(cfg.hostname, "server");
    assert_eq!(cfg.port, 9000);
}

#[test]
fn de_full_example() {
    // Mirrors the README example from structprop.
    #[derive(Debug, Deserialize, PartialEq)]
    struct Database {
        hostname: String,
        username: String,
        password: String,
        port: u16,
        database: String,
    }
    #[derive(Debug, Deserialize, PartialEq)]
    struct Config {
        database: Database,
        tables: Vec<String>,
    }

    let input = r#"
# This is a simple example config file
database {
  hostname = localhost
  username = dbuser
  password = secret
  port = 12361
  database = TheDatabase
}

tables = { Table1 Table2 }
"#;

    let cfg: Config = from_str(input).unwrap();
    assert_eq!(cfg.database.hostname, "localhost");
    assert_eq!(cfg.database.username, "dbuser");
    assert_eq!(cfg.database.password, "secret");
    assert_eq!(cfg.database.port, 12361);
    assert_eq!(cfg.database.database, "TheDatabase");
    assert_eq!(cfg.tables, vec!["Table1", "Table2"]);
}

// ---------------------------------------------------------------------------
// Serialization tests
// ---------------------------------------------------------------------------

#[derive(Debug, Serialize, Deserialize, PartialEq)]
struct Config {
    hostname: String,
    port: u16,
}

#[test]
fn ser_simple_struct() {
    let cfg = Config {
        hostname: "localhost".into(),
        port: 8080,
    };
    let out = to_string(&cfg).unwrap();
    assert!(out.contains("hostname = localhost"), "got: {}", out);
    assert!(out.contains("port = 8080"), "got: {}", out);
}

#[test]
fn ser_quoted_strings() {
    #[derive(Serialize)]
    struct S {
        message: String,
    }
    let s = S { message: "hello world".into() };
    let out = to_string(&s).unwrap();
    assert!(out.contains(r#"message = "hello world""#), "got: {}", out);
}

#[test]
fn ser_nested_struct() {
    #[derive(Serialize)]
    struct Inner {
        x: u32,
    }
    #[derive(Serialize)]
    struct Outer {
        inner: Inner,
    }
    let o = Outer { inner: Inner { x: 7 } };
    let out = to_string(&o).unwrap();
    assert!(out.contains("inner {"), "got: {}", out);
    assert!(out.contains("x = 7"), "got: {}", out);
}

#[test]
fn ser_vec_of_strings() {
    #[derive(Serialize)]
    struct S {
        items: Vec<String>,
    }
    let s = S { items: vec!["a".into(), "b".into(), "c".into()] };
    let out = to_string(&s).unwrap();
    assert!(out.contains("items"), "got: {}", out);
    assert!(out.contains("a"), "got: {}", out);
    assert!(out.contains("b"), "got: {}", out);
}

// ---------------------------------------------------------------------------
// Round-trip tests
// ---------------------------------------------------------------------------

#[test]
fn roundtrip_simple() {
    let original = Config { hostname: "myserver".into(), port: 3000 };
    let serialized = to_string(&original).unwrap();
    let deserialized: Config = from_str(&serialized).unwrap();
    assert_eq!(original, deserialized);
}

#[test]
fn roundtrip_nested() {
    #[derive(Debug, Serialize, Deserialize, PartialEq)]
    struct Inner {
        value: String,
        count: u32,
    }
    #[derive(Debug, Serialize, Deserialize, PartialEq)]
    struct Outer {
        name: String,
        inner: Inner,
    }

    let original = Outer {
        name: "test".into(),
        inner: Inner { value: "foo".into(), count: 42 },
    };
    let serialized = to_string(&original).unwrap();
    let deserialized: Outer = from_str(&serialized).unwrap();
    assert_eq!(original, deserialized);
}

#[test]
fn roundtrip_vec() {
    #[derive(Debug, Serialize, Deserialize, PartialEq)]
    struct S {
        tags: Vec<String>,
    }
    let original = S { tags: vec!["rust".into(), "serde".into(), "config".into()] };
    let serialized = to_string(&original).unwrap();
    let deserialized: S = from_str(&serialized).unwrap();
    assert_eq!(original, deserialized);
}
