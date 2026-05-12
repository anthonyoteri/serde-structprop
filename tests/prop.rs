use proptest::prelude::*;
use serde::{Deserialize, Serialize};
use serde_structprop::{from_str, to_string};

// ---------------------------------------------------------------------------
// Strategies
// ---------------------------------------------------------------------------

/// Generate structprop scalar strings that round-trip through serialization.
///
/// The strategy covers two classes:
/// - **Bare** scalars (`[a-zA-Z0-9_.-]+`) that need no quoting.
/// - **Quoted** scalars that contain at least one character requiring quoting
///   (`space`, `\t`, `#`, `{`, `}`, `=`) but no characters that are
///   unescapable inside a quoted string (`"`, `\n`, `\r`).
///
/// Strings containing `"`, `\n`, or `\r` are excluded because the format has
/// no escape mechanism for them inside a quoted value.
fn safe_string() -> impl Strategy<Value = String> {
    // Printable ASCII-ish chars excluding `"`, `\n`, `\r` (unescapable inside
    // a quoted value).  Includes space, tab and structprop metacharacters so
    // that the quoted path is exercised, not just the bare-scalar path.
    "[a-zA-Z0-9_.\\-\t #{}= ]{1,32}"
        .prop_map(|s| s.trim().to_owned())
        .prop_filter("non-empty after trim", |s| !s.is_empty())
}

// ---------------------------------------------------------------------------
// Flat struct
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
struct Flat {
    key: String,
    value: String,
    count: u32,
    flag: bool,
}

proptest! {
    #[test]
    fn roundtrip_flat_struct(
        key   in safe_string(),
        value in safe_string(),
        count in any::<u32>(),
        flag  in any::<bool>(),
    ) {
        let original = Flat { key, value, count, flag };
        let serialized = to_string(&original).unwrap();
        let deserialized: Flat = from_str(&serialized).unwrap();
        prop_assert_eq!(original, deserialized);
    }
}

// ---------------------------------------------------------------------------
// Nested struct
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
struct Inner {
    x: u64,
    label: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
struct Outer {
    name: String,
    inner: Inner,
}

proptest! {
    #[test]
    fn roundtrip_nested_struct(
        name  in safe_string(),
        x     in any::<u64>(),
        label in safe_string(),
    ) {
        let original = Outer { name, inner: Inner { x, label } };
        let serialized = to_string(&original).unwrap();
        let deserialized: Outer = from_str(&serialized).unwrap();
        prop_assert_eq!(original, deserialized);
    }
}

// ---------------------------------------------------------------------------
// Vec of strings
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
struct WithVec {
    tags: Vec<String>,
}

proptest! {
    #[test]
    fn roundtrip_vec_of_strings(
        tags in prop::collection::vec(safe_string(), 0..=16),
    ) {
        let original = WithVec { tags };
        let serialized = to_string(&original).unwrap();
        let deserialized: WithVec = from_str(&serialized).unwrap();
        prop_assert_eq!(original, deserialized);
    }
}

// ---------------------------------------------------------------------------
// Vec of integers
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
struct WithInts {
    ports: Vec<u16>,
}

proptest! {
    #[test]
    fn roundtrip_vec_of_integers(
        ports in prop::collection::vec(any::<u16>(), 0..=16),
    ) {
        let original = WithInts { ports };
        let serialized = to_string(&original).unwrap();
        let deserialized: WithInts = from_str(&serialized).unwrap();
        prop_assert_eq!(original, deserialized);
    }
}

// ---------------------------------------------------------------------------
// Optional fields
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
struct WithOption {
    required: String,
    optional: Option<u32>,
}

proptest! {
    #[test]
    fn roundtrip_option_some(
        required in safe_string(),
        optional in any::<u32>(),
    ) {
        let original = WithOption { required, optional: Some(optional) };
        let serialized = to_string(&original).unwrap();
        let deserialized: WithOption = from_str(&serialized).unwrap();
        prop_assert_eq!(original, deserialized);
    }
}

// ---------------------------------------------------------------------------
// Deeply nested struct
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
struct Leaf {
    value: String,
    count: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
struct Mid {
    label: String,
    leaf: Leaf,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
struct Root {
    name: String,
    mid: Mid,
}

proptest! {
    #[test]
    fn roundtrip_deeply_nested(
        name  in safe_string(),
        label in safe_string(),
        value in safe_string(),
        count in any::<u32>(),
    ) {
        let original = Root {
            name,
            mid: Mid { label, leaf: Leaf { value, count } },
        };
        let serialized = to_string(&original).unwrap();
        let deserialized: Root = from_str(&serialized).unwrap();
        prop_assert_eq!(original, deserialized);
    }
}
