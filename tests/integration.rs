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
    let input = "message = \"hello world\"";
    let s: S = from_str(input).unwrap();
    assert_eq!(s.message, "hello world");
}

#[test]
fn de_nested_struct() {
    #[derive(Debug, Deserialize, PartialEq)]
    struct DbConn {
        hostname: String,
        port: u16,
    }
    #[derive(Debug, Deserialize, PartialEq)]
    struct Config {
        database: DbConn,
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
    struct DbConn {
        hostname: String,
        username: String,
        password: String,
        port: u16,
        name: String,
    }
    #[derive(Debug, Deserialize, PartialEq)]
    struct Config {
        database: DbConn,
        tables: Vec<String>,
    }

    let input = "
# This is a simple example config file
database {
  hostname = localhost
  username = dbuser
  password = secret
  port = 12361
  name = TheDatabase
}

tables = { Table1 Table2 }
";

    let cfg: Config = from_str(input).unwrap();
    assert_eq!(cfg.database.hostname, "localhost");
    assert_eq!(cfg.database.username, "dbuser");
    assert_eq!(cfg.database.password, "secret");
    assert_eq!(cfg.database.port, 12361);
    assert_eq!(cfg.database.name, "TheDatabase");
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
    assert!(out.contains("hostname = localhost"), "got: {out}");
    assert!(out.contains("port = 8080"), "got: {out}");
}

#[test]
fn ser_quoted_strings() {
    #[derive(Serialize)]
    struct S {
        message: String,
    }
    let s = S {
        message: "hello world".into(),
    };
    let out = to_string(&s).unwrap();
    assert!(out.contains("message = \"hello world\""), "got: {out}");
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
    let o = Outer {
        inner: Inner { x: 7 },
    };
    let out = to_string(&o).unwrap();
    assert!(out.contains("inner {"), "got: {out}");
    assert!(out.contains("x = 7"), "got: {out}");
}

#[test]
fn ser_vec_of_strings() {
    #[derive(Serialize)]
    struct S {
        items: Vec<String>,
    }
    let s = S {
        items: vec!["a".into(), "b".into(), "c".into()],
    };
    let out = to_string(&s).unwrap();
    assert!(out.contains("items"), "got: {out}");
    assert!(out.contains('a'), "got: {out}");
    assert!(out.contains('b'), "got: {out}");
}

// ---------------------------------------------------------------------------
// Round-trip tests
// ---------------------------------------------------------------------------

#[test]
fn roundtrip_simple() {
    let original = Config {
        hostname: "myserver".into(),
        port: 3000,
    };
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
        inner: Inner {
            value: "foo".into(),
            count: 42,
        },
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
    let original = S {
        tags: vec!["rust".into(), "serde".into(), "config".into()],
    };
    let serialized = to_string(&original).unwrap();
    let deserialized: S = from_str(&serialized).unwrap();
    assert_eq!(original, deserialized);
}

// ---------------------------------------------------------------------------
// Python test-suite counterparts
// ---------------------------------------------------------------------------

#[test]
fn de_parser_error_on_missing_term() {
    // A key with no value / block is a parse error.
    let result = from_str::<Simple>("hostname\n");
    assert!(
        result.is_err(),
        "expected parse error for bare key with no value"
    );
}

#[test]
fn de_inline_comment() {
    // Inline comments (after a value) should be ignored.
    #[derive(Debug, Deserialize, PartialEq)]
    struct S {
        key: String,
    }
    let input = "key = value # this is an inline comment\n";
    let s: S = from_str(input).unwrap();
    assert_eq!(s.key, "value");
}

#[test]
fn de_quoted_key() {
    // A quoted key should be parsed and the quotes stripped.
    #[derive(Debug, Deserialize, PartialEq)]
    struct S {
        my_key: String,
    }
    let input = "\"my_key\" = hello\n";
    let s: S = from_str(input).unwrap();
    assert_eq!(s.my_key, "hello");
}

#[test]
fn de_empty_object() {
    // An empty block should deserialise without error.
    #[derive(Debug, Deserialize, PartialEq)]
    struct Inner {}
    #[derive(Debug, Deserialize, PartialEq)]
    struct Outer {
        section: Inner,
    }
    let input = "section {\n}\n";
    let outer: Outer = from_str(input).unwrap();
    let _ = outer; // just assert no panic / error
}

#[test]
fn de_object_key_value_single_line() {
    // Block with a single key = value entry.
    #[derive(Debug, Deserialize, PartialEq)]
    struct Inner {
        x: u32,
    }
    #[derive(Debug, Deserialize, PartialEq)]
    struct Outer {
        section: Inner,
    }
    let input = "section {\n  x = 42\n}\n";
    let outer: Outer = from_str(input).unwrap();
    assert_eq!(outer.section.x, 42);
}

#[test]
fn de_nested_objects_mixed_array() {
    // Nested block that also contains an array.
    #[derive(Debug, Deserialize, PartialEq)]
    struct Inner {
        tags: Vec<String>,
    }
    #[derive(Debug, Deserialize, PartialEq)]
    struct Outer {
        section: Inner,
    }
    let input = "section {\n  tags = { alpha beta }\n}\n";
    let outer: Outer = from_str(input).unwrap();
    assert_eq!(outer.section.tags, vec!["alpha", "beta"]);
}

#[test]
fn de_false_bool() {
    #[derive(Debug, Deserialize, PartialEq)]
    struct S {
        flag: bool,
    }
    let input = "flag = false\n";
    let s: S = from_str(input).unwrap();
    assert!(!s.flag);
}

#[test]
fn ser_dump_list_exact() {
    // Mirrors the Python `test_dump_list` expectation.
    #[derive(Serialize)]
    struct S {
        a: Vec<String>,
    }
    let s = S {
        a: vec!["a".into(), "b".into(), "c".into()],
    };
    let out = to_string(&s).unwrap();
    assert_eq!(out, "a = {\n  a\n  b\n  c\n}\n", "got: {out:?}");
}

#[test]
fn ser_dump_dict_exact() {
    // Mirrors the Python `test_dump_dict` expectation.
    #[derive(Serialize)]
    struct Inner {
        b: String,
    }
    #[derive(Serialize)]
    struct Outer {
        a: Inner,
    }
    let o = Outer {
        a: Inner { b: "c".into() },
    };
    let out = to_string(&o).unwrap();
    assert_eq!(out, "a {\n  b = c\n}\n", "got: {out:?}");
}

#[test]
fn ser_dump_true_bool() {
    #[derive(Serialize)]
    struct S {
        flag: bool,
    }
    let out = to_string(&S { flag: true }).unwrap();
    assert_eq!(out, "flag = true\n");
}

#[test]
fn ser_dump_false_bool() {
    #[derive(Serialize)]
    struct S {
        flag: bool,
    }
    let out = to_string(&S { flag: false }).unwrap();
    assert_eq!(out, "flag = false\n");
}

#[test]
fn ser_escape_space_in_value() {
    // Values containing spaces must be quoted on output.
    #[derive(Serialize)]
    struct S {
        msg: String,
    }
    let out = to_string(&S {
        msg: "hello world".into(),
    })
    .unwrap();
    assert!(out.contains("msg = \"hello world\""), "got: {out:?}");
}

#[test]
fn ser_object_order_is_kept() {
    // Field declaration order must be preserved in serialized output.
    #[derive(Serialize)]
    struct S {
        first: u32,
        second: u32,
        third: u32,
    }
    let out = to_string(&S {
        first: 1,
        second: 2,
        third: 3,
    })
    .unwrap();
    let pos_first = out.find("first").unwrap();
    let pos_second = out.find("second").unwrap();
    let pos_third = out.find("third").unwrap();
    assert!(
        pos_first < pos_second && pos_second < pos_third,
        "got: {out:?}"
    );
}
