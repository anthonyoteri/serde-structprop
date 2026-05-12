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

// ---------------------------------------------------------------------------
// Bug fix tests
// ---------------------------------------------------------------------------

// Cycle 1: serialize_struct_variant must not corrupt output via insert_str(0)
#[test]
fn ser_struct_variant_does_not_corrupt_preceding_fields() {
    #[derive(Serialize)]
    enum Shape {
        Circle { radius: u32 },
    }
    #[derive(Serialize)]
    struct Config {
        name: String,
        shape: Shape,
    }
    let cfg = Config {
        name: "test".into(),
        shape: Shape::Circle { radius: 5 },
    };
    let out = to_string(&cfg).unwrap();
    // "name" must appear before "Circle" — insert_str(0) bug put the variant
    // header at position 0, before any previously-written fields.
    let pos_name = out.find("name").expect("missing 'name'");
    let pos_circle = out.find("Circle").expect("missing 'Circle'");
    assert!(
        pos_name < pos_circle,
        "'name' should appear before 'Circle', got:\n{out}"
    );
}

// Cycle 2: tuple variant with zero elements must not panic
#[test]
fn ser_tuple_variant_zero_elements_does_not_panic() {
    #[derive(Serialize)]
    enum E {
        Empty(),
    }
    // Must not panic — previously items.remove(0) panicked on empty vec.
    let result = to_string(&E::Empty());
    assert!(result.is_ok(), "expected Ok, got {result:?}");
}

// Cycle 2b: tuple variant sentinel collision — variant named "__variant__Foo"
#[test]
fn ser_tuple_variant_sentinel_collision() {
    #[derive(Serialize, Deserialize, PartialEq, Debug)]
    enum E {
        #[serde(rename = "__variant__Tricky")]
        Tricky(u32),
    }
    let out = to_string(&E::Tricky(42)).unwrap();
    // The variant name in the output must be the full "__variant__Tricky",
    // not just "Tricky" (which would happen if sentinel stripping ate the prefix).
    assert!(
        out.contains("__variant__Tricky"),
        "variant name mangled, got:\n{out}"
    );
}

// Cycle 3: escape() must quote strings containing newlines
#[test]
fn ser_string_with_newline_is_quoted() {
    #[derive(Serialize, Deserialize, PartialEq, Debug)]
    struct S {
        msg: String,
    }
    let s = S {
        msg: "hello\nworld".into(),
    };
    let out = to_string(&s).unwrap();
    // A bare newline in the output would break the line-oriented parser.
    // The value must be wrapped in quotes.
    assert!(
        out.contains('"'),
        "expected quoted output for newline-containing string, got:\n{out}"
    );
    // And it must round-trip.
    let back: S = from_str(&out).unwrap();
    assert_eq!(back, s);
}

// Nested struct containing a string with a newline must round-trip correctly.
// Regression for write_kv re-indent bug: blindly prefixing every line with
// spaces would corrupt the continuation lines of a quoted multi-line scalar.
#[test]
fn roundtrip_nested_struct_with_newline_string() {
    #[derive(Serialize, Deserialize, PartialEq, Debug)]
    struct Inner {
        msg: String,
    }
    #[derive(Serialize, Deserialize, PartialEq, Debug)]
    struct Outer {
        name: String,
        inner: Inner,
    }
    let orig = Outer {
        name: "test".into(),
        inner: Inner {
            msg: "hello\nworld".into(),
        },
    };
    let out = to_string(&orig).unwrap();
    let back: Outer = from_str(&out).unwrap();
    assert_eq!(back, orig, "round-trip failed; output was:\n{out}");
}

// Cycle 4: serialize_char must quote structprop special characters
#[test]
fn ser_char_special_chars_are_quoted() {
    #[derive(Serialize, Deserialize, PartialEq, Debug)]
    struct S {
        c: char,
    }
    for special in [' ', '#', '{', '}', '='] {
        let s = S { c: special };
        let out = to_string(&s).unwrap();
        let back: S = from_str(&out).unwrap();
        assert_eq!(
            back, s,
            "char '{special}' did not round-trip; output was:\n{out}"
        );
    }
}

// Cycle 5: duplicate keys must produce an error, not silently overwrite
#[test]
fn de_duplicate_key_is_an_error() {
    use std::collections::HashMap;
    let input = "port = 1234\nport = 5678\n";
    let result: Result<HashMap<String, String>, _> = from_str(input);
    assert!(
        result.is_err(),
        "expected error for duplicate key, got {result:?}"
    );
}

// Cycle 6: unterminated quoted string must produce an error, not silently drop content
#[test]
fn de_unterminated_quoted_string_is_an_error() {
    use std::collections::HashMap;
    let input = "key = \"unterminated";
    let result: Result<HashMap<String, String>, _> = from_str(input);
    assert!(
        result.is_err(),
        "expected error for unterminated string, got {result:?}"
    );
}

// Cycle 7: large integer (> i64::MAX) must not silently become a float
#[test]
fn de_large_integer_is_not_silently_coerced_to_float() {
    // u64::MAX cannot be represented as i64; previously deserialize_any would
    // fall through i64 parse failure → f64 parse, silently losing precision.
    #[derive(Debug, Deserialize)]
    struct S {
        val: u64,
    }
    let input = format!("val = {}\n", u64::MAX);
    let s: S = from_str(&input).unwrap();
    assert_eq!(s.val, u64::MAX);
}

// Cycle 8: enum round-trips — unit, newtype, tuple, struct variants
#[derive(Serialize, Deserialize, PartialEq, Debug)]
enum Color {
    Red,
    Green,
    Blue,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(u8, u8, u8),
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
struct WithEnum {
    color: Color,
}

#[test]
fn roundtrip_unit_enum_variant() {
    let s = WithEnum { color: Color::Red };
    let out = to_string(&s).unwrap();
    let back: WithEnum = from_str(&out).unwrap();
    assert_eq!(back, s);
}

#[test]
fn roundtrip_newtype_enum_variant() {
    let msg = Message::Write("hello".into());
    let out = to_string(&msg).unwrap();
    let back: Message = from_str(&out).unwrap();
    assert_eq!(back, msg);
}

#[test]
fn roundtrip_struct_enum_variant() {
    let msg = Message::Move { x: 10, y: 20 };
    let out = to_string(&msg).unwrap();
    let back: Message = from_str(&out).unwrap();
    assert_eq!(back, msg);
}

#[test]
fn roundtrip_tuple_enum_variant() {
    let msg = Message::ChangeColor(255, 128, 0);
    let out = to_string(&msg).unwrap();
    let back: Message = from_str(&out).unwrap();
    assert_eq!(back, msg);
}

// Cycle 9: Option::None round-trip
#[test]
fn roundtrip_option_none() {
    #[derive(Serialize, Deserialize, PartialEq, Debug)]
    struct S {
        #[serde(default)]
        value: Option<u32>,
    }
    // None serializes to `null`; must deserialize back to None.
    let s = S { value: None };
    let out = to_string(&s).unwrap();
    assert!(
        out.contains("null"),
        "expected 'null' in output, got:\n{out}"
    );
    let back: S = from_str(&out).unwrap();
    assert_eq!(back, s);
}

// Option field absent from document should deserialize as None via serde default
#[test]
fn de_option_field_absent_defaults_to_none() {
    #[derive(Deserialize, PartialEq, Debug)]
    struct S {
        name: String,
        #[serde(default)]
        count: Option<u32>,
    }
    let s: S = from_str("name = foo\n").unwrap();
    assert_eq!(s.count, None);
}

// deserialize_unit must reject non-null values, and accept "null"
#[test]
fn de_unit_field_roundtrip() {
    #[derive(Debug, Deserialize, PartialEq)]
    struct Doc {
        key: (),
    }
    // "null" is the structprop representation of a unit value — must succeed.
    let doc: Doc = from_str("key = null\n").expect("expected Ok deserializing null as unit");
    assert_eq!(doc.key, ());

    // Any non-null scalar must be rejected when the target type is unit.
    let err: Result<Doc, _> = from_str("key = hello\n");
    assert!(
        err.is_err(),
        "expected error deserializing non-null as unit"
    );
}

// serialize_key must return KeyMustBeString for non-string key types.
// Only str/String keys are accepted; char and enum variants are rejected too.
#[test]
fn ser_non_string_map_key_is_an_error() {
    use std::collections::HashMap;

    // Integer key
    let mut int_map: HashMap<u32, &str> = HashMap::new();
    int_map.insert(42, "hello");
    assert!(
        to_string(&int_map).is_err(),
        "expected error serializing integer map key"
    );

    // char key
    let mut char_map: HashMap<char, &str> = HashMap::new();
    char_map.insert('x', "hello");
    assert!(
        to_string(&char_map).is_err(),
        "expected error serializing char map key"
    );
}

// ---------------------------------------------------------------------------
// Regression tests
// ---------------------------------------------------------------------------

#[test]
fn ser_empty_string_is_quoted() {
    // Empty strings must be quoted so that `key = ` is never emitted —
    // an unquoted empty value produces no token and fails to parse back.
    #[derive(Serialize, Deserialize, PartialEq, Debug)]
    struct Cfg {
        val: String,
    }
    let original = Cfg { val: String::new() };
    let s = to_string(&original).unwrap();
    assert!(s.contains("val = \"\""), "expected quoted empty string, got: {s:?}");
    let roundtripped: Cfg = from_str(&s).unwrap();
    assert_eq!(original, roundtripped);
}
