use std::default;

use token::{Token, assert_tokens, assert_ser_tokens, assert_de_tokens};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct Default {
    a1: i32,
    #[serde(default)]
    a2: i32,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct Rename {
    a1: i32,
    #[serde(rename="a3")]
    a2: i32,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct FormatRename {
    a1: i32,
    #[serde(rename(xml= "a4", token="a5"))]
    a2: i32,
}

#[derive(Debug, PartialEq, Deserialize, Serialize)]
enum SerEnum<A> {
    Map {
        a: i8,
        #[serde(rename(xml= "c", token="d"))]
        b: A,
    },
}

#[derive(Debug, PartialEq, Deserialize, Serialize)]
struct SkipSerializingFields<A: default::Default> {
    a: i8,
    #[serde(skip_serializing, default)]
    b: A,
}

#[derive(Debug, PartialEq, Deserialize, Serialize)]
struct SkipSerializingIfEmptyFields<A: default::Default> {
    a: i8,
    #[serde(skip_serializing_if_empty, default)]
    b: Vec<A>,
}

#[derive(Debug, PartialEq, Deserialize, Serialize)]
struct SkipSerializingIfNoneFields<A: default::Default> {
    a: i8,
    #[serde(skip_serializing_if_none, default)]
    b: Option<A>,
}

#[derive(Debug, PartialEq, Deserialize, Serialize)]
struct AddSerializer(i8);
impl AddSerializer {
    fn new(v: &i8) -> Self { AddSerializer(*v + 1) }
}

impl Into<i8> for AddSerializer {
    fn into(self) -> i8 {
        self.0 - 1
    }
}

#[derive(Debug, PartialEq, Deserialize, Serialize)]
struct SerializerFields {
    a: i8,
    #[serde(serializer="AddSerializer::new", deserializer="AddSerializer")]
    b: i8,
}

#[test]
fn test_default() {
    assert_de_tokens(
        &Default { a1: 1, a2: 2 },
        vec![
            Token::StructStart("Default", Some(2)),

            Token::MapSep,
            Token::Str("a1"),
            Token::I32(1),

            Token::MapSep,
            Token::Str("a2"),
            Token::I32(2),

            Token::MapEnd,
        ]
    );

    assert_de_tokens(
        &Default { a1: 1, a2: 0 },
        vec![
            Token::StructStart("Default", Some(1)),

            Token::MapSep,
            Token::Str("a1"),
            Token::I32(1),

            Token::MapEnd,
        ]
    );
}

#[test]
fn test_rename() {
    assert_tokens(
        &Rename { a1: 1, a2: 2 },
        vec![
            Token::StructStart("Rename", Some(2)),

            Token::MapSep,
            Token::Str("a1"),
            Token::I32(1),

            Token::MapSep,
            Token::Str("a3"),
            Token::I32(2),

            Token::MapEnd,
        ]
    );
}

#[test]
fn test_serializer() {
    assert_ser_tokens(
        &SerializerFields {
            a: 1,
            b: 2,
        },
        &[
            Token::StructStart("SerializerFields", Some(2)),

            Token::MapSep,
            Token::Str("a"),
            Token::I8(1),

            Token::MapSep,
            Token::Str("b"),
            Token::StructNewtype("AddSerializer"),
            Token::I8(3),

            Token::MapEnd,
        ]
    );

    assert_de_tokens(
        &SerializerFields {
            a: 1,
            b: 2,
        },
        vec![
            Token::StructStart("SerializerFields", Some(2)),

            Token::MapSep,
            Token::Str("a"),
            Token::I8(1),

            Token::MapSep,
            Token::Str("b"),
            Token::StructNewtype("AddSerializer"),
            Token::I8(3),

            Token::MapEnd,
        ]
    );
}

#[test]
fn test_format_rename() {
    assert_tokens(
        &FormatRename { a1: 1, a2: 2 },
        vec![
            Token::StructStart("FormatRename", Some(2)),

            Token::MapSep,
            Token::Str("a1"),
            Token::I32(1),

            Token::MapSep,
            Token::Str("a5"),
            Token::I32(2),

            Token::MapEnd,
        ]
    );
}

#[test]
fn test_enum_format_rename() {
    assert_tokens(
        &SerEnum::Map {
            a: 0,
            b: String::new(),
        },
        vec![
            Token::EnumMapStart("SerEnum", "Map", Some(2)),

            Token::MapSep,
            Token::Str("a"),
            Token::I8(0),

            Token::MapSep,
            Token::Str("d"),
            Token::Str(""),

            Token::MapEnd,
        ]
    );
}

#[test]
fn test_skip_serializing_fields() {
    assert_ser_tokens(
        &SkipSerializingFields {
            a: 1,
            b: 2,
        },
        &[
            Token::StructStart("SkipSerializingFields", Some(1)),

            Token::MapSep,
            Token::Str("a"),
            Token::I8(1),

            Token::MapEnd,
        ]
    );

    assert_de_tokens(
        &SkipSerializingFields {
            a: 1,
            b: 0,
        },
        vec![
            Token::StructStart("SkipSerializingFields", Some(1)),

            Token::MapSep,
            Token::Str("a"),
            Token::I8(1),

            Token::MapEnd,
        ]
    );
}

#[test]
fn test_skip_serializing_fields_if_empty() {
    assert_ser_tokens(
        &SkipSerializingIfEmptyFields::<i32> {
            a: 1,
            b: vec![],
        },
        &[
            Token::StructStart("SkipSerializingIfEmptyFields", Some(1)),

            Token::MapSep,
            Token::Str("a"),
            Token::I8(1),

            Token::MapEnd,
        ]
    );

    assert_de_tokens(
        &SkipSerializingIfEmptyFields::<i32> {
            a: 1,
            b: vec![],
        },
        vec![
            Token::StructStart("SkipSerializingIfEmptyFields", Some(1)),

            Token::MapSep,
            Token::Str("a"),
            Token::I8(1),

            Token::MapEnd,
        ]
    );

    assert_ser_tokens(
        &SkipSerializingIfEmptyFields {
            a: 1,
            b: vec![2],
        },
        &[
            Token::StructStart("SkipSerializingIfEmptyFields", Some(2)),

            Token::MapSep,
            Token::Str("a"),
            Token::I8(1),

            Token::MapSep,
            Token::Str("b"),
            Token::SeqStart(Some(1)),
            Token::SeqSep,
            Token::I32(2),
            Token::SeqEnd,

            Token::MapEnd,
        ]
    );

    assert_de_tokens(
        &SkipSerializingIfEmptyFields {
            a: 1,
            b: vec![2],
        },
        vec![
            Token::StructStart("SkipSerializingIfEmptyFields", Some(2)),

            Token::MapSep,
            Token::Str("a"),
            Token::I8(1),

            Token::MapSep,
            Token::Str("b"),
            Token::SeqStart(Some(1)),
            Token::SeqSep,
            Token::I32(2),
            Token::SeqEnd,

            Token::MapEnd,
        ]
    );
}

#[test]
fn test_skip_serializing_fields_if_none() {
    assert_ser_tokens(
        &SkipSerializingIfNoneFields::<i32> {
            a: 1,
            b: None,
        },
        &[
            Token::StructStart("SkipSerializingIfNoneFields", Some(1)),

            Token::MapSep,
            Token::Str("a"),
            Token::I8(1),

            Token::MapEnd,
        ]
    );

    assert_de_tokens(
        &SkipSerializingIfNoneFields::<i32> {
            a: 1,
            b: None,
        },
        vec![
            Token::StructStart("SkipSerializingIfNoneFields", Some(1)),

            Token::MapSep,
            Token::Str("a"),
            Token::I8(1),

            Token::MapEnd,
        ]
    );

    assert_ser_tokens(
        &SkipSerializingIfNoneFields {
            a: 1,
            b: Some(2),
        },
        &[
            Token::StructStart("SkipSerializingIfNoneFields", Some(2)),

            Token::MapSep,
            Token::Str("a"),
            Token::I8(1),

            Token::MapSep,
            Token::Str("b"),
            Token::Option(true),
            Token::I32(2),

            Token::MapEnd,
        ]
    );

    assert_de_tokens(
        &SkipSerializingIfNoneFields {
            a: 1,
            b: Some(2),
        },
        vec![
            Token::StructStart("SkipSerializingIfNoneFields", Some(2)),

            Token::MapSep,
            Token::Str("a"),
            Token::I8(1),

            Token::MapSep,
            Token::Str("b"),
            Token::Option(true),
            Token::I32(2),

            Token::MapEnd,
        ]
    );
}
