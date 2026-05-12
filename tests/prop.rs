use proptest::prelude::*;
use serde::{Deserialize, Serialize};
use serde_structprop::{from_str, to_string};

// ---------------------------------------------------------------------------
// Strategies
// ---------------------------------------------------------------------------

/// Generate structprop scalar strings that round-trip through serialization.
///
/// Uses `prop_oneof!` to guarantee coverage of both syntactic classes:
///
/// - **Bare** scalars (`[a-zA-Z0-9_.-]+`): serialized without quoting.
/// - **Quoted** scalars: contain at least one character that forces quoting
///   (`space`, `\t`, `\n`, `\r`, `#`, `{`, `}`, `=`) and no `"` character,
///   which is the only character that cannot be represented inside a quoted
///   value (the format has no escape mechanism for it).
fn safe_string() -> impl Strategy<Value = String> {
    // Bare: alphanumeric plus the safe punctuation that needs no quoting.
    let bare = "[a-zA-Z0-9_.\\-]{1,32}".prop_map(|s| s);

    // Quoted: must contain at least one character that triggers quoting.
    // We use a regex that starts with a mandatory special char, followed by
    // arbitrary safe content, so the filter always passes.
    // Characters excluded: `"` (unrepresentable inside quotes).
    let quoted = "[a-zA-Z0-9_.\\-]{0,15}[ \t\n\r#{}=][a-zA-Z0-9_.\\-]{0,15}"
        .prop_map(|s| s)
        .prop_filter("non-empty", |s| !s.is_empty());

    prop_oneof![bare, quoted]
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
