use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, Debug)]
enum Outer {
    Variant(Inner),
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
enum Inner {
    A,
    Newtype(i32),
}

#[test]
fn test_nested_enum_serialization() {
    let thing = Outer::Variant(Inner::Newtype(1));
    let serialized = yaml_serde::to_string(&thing).unwrap();
    
    println!("Serialized:\n{}", serialized);
    
    // Check that it contains the expected tags and nesting
    assert!(serialized.contains("!Variant"));
    assert!(serialized.contains("- !Newtype 1"));

    let deserialized: Outer = yaml_serde::from_str(&serialized).unwrap();
    assert_eq!(thing, deserialized);
}

#[test]
fn test_nested_enum_complex() {
    #[derive(Serialize, Deserialize, PartialEq, Debug)]
    enum Outer {
        Newtype(Inner),
        Tuple(Inner, i32),
        Struct { inner: Inner, other: i32 },
    }

    #[derive(Serialize, Deserialize, PartialEq, Debug)]
    enum Inner {
        Newtype(i32),
        Tuple(i32, i32),
        Struct { x: i32, y: i32 },
    }

    // Newtype(Newtype)
    let thing = Outer::Newtype(Inner::Newtype(1));
    let serialized = yaml_serde::to_string(&thing).unwrap();
    let deserialized: Outer = yaml_serde::from_str(&serialized).unwrap();
    assert_eq!(thing, deserialized);

    // Newtype(Tuple)
    let thing = Outer::Newtype(Inner::Tuple(1, 2));
    let serialized = yaml_serde::to_string(&thing).unwrap();
    let deserialized: Outer = yaml_serde::from_str(&serialized).unwrap();
    assert_eq!(thing, deserialized);

    // Tuple(Struct, i32)
    let thing = Outer::Tuple(Inner::Struct { x: 1, y: 2 }, 42);
    let serialized = yaml_serde::to_string(&thing).unwrap();
    let deserialized: Outer = yaml_serde::from_str(&serialized).unwrap();
    assert_eq!(thing, deserialized);

    // Struct { inner: Newtype, other: i32 }
    let thing = Outer::Struct { inner: Inner::Newtype(1), other: 42 };
    let serialized = yaml_serde::to_string(&thing).unwrap();
    let deserialized: Outer = yaml_serde::from_str(&serialized).unwrap();
    assert_eq!(thing, deserialized);
}

#[test]
fn test_nested_unit_enum() {
    #[derive(Serialize, Deserialize, PartialEq, Debug)]
    enum Outer { V(Inner) }
    #[derive(Serialize, Deserialize, PartialEq, Debug)]
    enum Inner { A, B }

    let thing = Outer::V(Inner::A);
    let serialized = yaml_serde::to_string(&thing).unwrap();
    println!("Nested unit enum:\n{}", serialized);
    // Should be !V A
    assert!(serialized.contains("!V A") || serialized.contains("!V\n- A"));

    let deserialized: Outer = yaml_serde::from_str(&serialized).unwrap();
    assert_eq!(thing, deserialized);
}
