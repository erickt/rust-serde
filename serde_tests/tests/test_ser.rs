use std::collections::BTreeMap;
use std::path::{Path, PathBuf};
use std::str;

use num::FromPrimitive;
use num::bigint::{BigInt, BigUint};
use num::complex::Complex;
use num::rational::Ratio;

use token::{self, Token};

//////////////////////////////////////////////////////////////////////////

#[derive(Serialize)]
struct UnitStruct;

#[derive(Serialize)]
struct TupleStruct(i32, i32, i32);

#[derive(Serialize)]
struct Struct {
    a: i32,
    b: i32,
    c: i32,
}

#[derive(Serialize)]
enum Enum {
    Unit,
    One(i32),
    Seq(i32, i32),
    Map { a: i32, b: i32 },
}

//////////////////////////////////////////////////////////////////////////

declare_ser_tests! {
    test_unit {
        () => &[Token::Unit],
    }
    test_bool {
        true => &[Token::Bool(true)],
        false => &[Token::Bool(false)],
    }
    test_isizes {
        0isize => &[Token::Isize(0)],
        0i8 => &[Token::I8(0)],
        0i16 => &[Token::I16(0)],
        0i32 => &[Token::I32(0)],
        0i64 => &[Token::I64(0)],
    }
    test_usizes {
        0usize => &[Token::Usize(0)],
        0u8 => &[Token::U8(0)],
        0u16 => &[Token::U16(0)],
        0u32 => &[Token::U32(0)],
        0u64 => &[Token::U64(0)],
    }
    test_floats {
        0f32 => &[Token::F32(0.)],
        0f64 => &[Token::F64(0.)],
    }
    test_char {
        'a' => &[Token::Char('a')],
    }
    test_str {
        "abc" => &[Token::Str("abc")],
        "abc".to_owned() => &[Token::Str("abc")],
    }
    test_option {
        None::<i32> => &[Token::Option(false)],
        Some(1) => &[
            Token::Option(true),
            Token::I32(1),
        ],
    }
    test_result {
        Ok::<i32, i32>(0) => &[
            Token::EnumNewtype("Result", "Ok"),
            Token::I32(0),
        ],
        Err::<i32, i32>(1) => &[
            Token::EnumNewtype("Result", "Err"),
            Token::I32(1),
        ],
    }
    test_slice {
        &[0][..0] => &[
            Token::SeqStart(Some(0)),
            Token::SeqEnd,
        ],
        &[1, 2, 3][..] => &[
            Token::SeqStart(Some(3)),
                Token::SeqSep,
                Token::I32(1),

                Token::SeqSep,
                Token::I32(2),

                Token::SeqSep,
                Token::I32(3),
            Token::SeqEnd,
        ],
    }
    test_array {
        [0; 0] => &[
            Token::SeqStart(Some(0)),
            Token::SeqEnd,
        ],
        [1, 2, 3] => &[
            Token::SeqStart(Some(3)),
                Token::SeqSep,
                Token::I32(1),

                Token::SeqSep,
                Token::I32(2),

                Token::SeqSep,
                Token::I32(3),
            Token::SeqEnd,
        ],
    }
    test_vec {
        Vec::<isize>::new() => &[
            Token::SeqStart(Some(0)),
            Token::SeqEnd,
        ],
        vec![vec![], vec![1], vec![2, 3]] => &[
            Token::SeqStart(Some(3)),
                Token::SeqSep,
                Token::SeqStart(Some(0)),
                Token::SeqEnd,

                Token::SeqSep,
                Token::SeqStart(Some(1)),
                    Token::SeqSep,
                    Token::I32(1),
                Token::SeqEnd,

                Token::SeqSep,
                Token::SeqStart(Some(2)),
                    Token::SeqSep,
                    Token::I32(2),

                    Token::SeqSep,
                    Token::I32(3),
                Token::SeqEnd,
            Token::SeqEnd,
        ],
    }
    test_tuple {
        (1,) => &[
            Token::SeqStart(Some(1)),
                Token::SeqSep,
                Token::I32(1),
            Token::SeqEnd,
        ],
        (1, 2, 3) => &[
            Token::SeqStart(Some(3)),
                Token::SeqSep,
                Token::I32(1),

                Token::SeqSep,
                Token::I32(2),

                Token::SeqSep,
                Token::I32(3),
            Token::SeqEnd,
        ],
    }
    test_btreemap {
        btreemap![1 => 2] => &[
            Token::MapStart(Some(1)),
                Token::MapSep,
                Token::I32(1),
                Token::I32(2),
            Token::MapEnd,
        ],
        btreemap![1 => 2, 3 => 4] => &[
            Token::MapStart(Some(2)),
                Token::MapSep,
                Token::I32(1),
                Token::I32(2),

                Token::MapSep,
                Token::I32(3),
                Token::I32(4),
            Token::MapEnd,
        ],
        btreemap![1 => btreemap![], 2 => btreemap![3 => 4, 5 => 6]] => &[
            Token::MapStart(Some(2)),
                Token::MapSep,
                Token::I32(1),
                Token::MapStart(Some(0)),
                Token::MapEnd,

                Token::MapSep,
                Token::I32(2),
                Token::MapStart(Some(2)),
                    Token::MapSep,
                    Token::I32(3),
                    Token::I32(4),

                    Token::MapSep,
                    Token::I32(5),
                    Token::I32(6),
                Token::MapEnd,
            Token::MapEnd,
        ],
    }
    test_unit_struct {
        UnitStruct => &[Token::UnitStruct("UnitStruct")],
    }
    test_tuple_struct {
        TupleStruct(1, 2, 3) => &[
            Token::TupleStructStart("TupleStruct", Some(3)),
                Token::SeqSep,
                Token::I32(1),

                Token::SeqSep,
                Token::I32(2),

                Token::SeqSep,
                Token::I32(3),
            Token::SeqEnd,
        ],
    }
    test_struct {
        Struct { a: 1, b: 2, c: 3 } => &[
            Token::StructStart("Struct", Some(3)),
                Token::MapSep,
                Token::Str("a"),
                Token::I32(1),

                Token::MapSep,
                Token::Str("b"),
                Token::I32(2),

                Token::MapSep,
                Token::Str("c"),
                Token::I32(3),
            Token::MapEnd,
        ],
    }
    test_enum {
        Enum::Unit => &[Token::EnumUnit("Enum", "Unit")],
        Enum::One(42) => &[Token::EnumNewtype("Enum", "One"), Token::I32(42)],
        Enum::Seq(1, 2) => &[
            Token::EnumSeqStart("Enum", "Seq", Some(2)),
                Token::SeqSep,
                Token::I32(1),

                Token::SeqSep,
                Token::I32(2),
            Token::SeqEnd,
        ],
        Enum::Map { a: 1, b: 2 } => &[
            Token::EnumMapStart("Enum", "Map", Some(2)),
                Token::MapSep,
                Token::Str("a"),
                Token::I32(1),

                Token::MapSep,
                Token::Str("b"),
                Token::I32(2),
            Token::MapEnd,
        ],
    }
    test_num_bigint {
        BigInt::from_i64(123).unwrap() => &[Token::Str("123")],
        BigInt::from_i64(-123).unwrap() => &[Token::Str("-123")],
    }
    test_num_biguint {
        BigUint::from_i64(123).unwrap() => &[Token::Str("123")],
    }
    test_num_complex {
        Complex::new(1, 2) => &[
            Token::SeqStart(Some(2)),
                Token::SeqSep,
                Token::I32(1),

                Token::SeqSep,
                Token::I32(2),
            Token::SeqEnd,
        ],
    }
    test_num_ratio {
        Ratio::new(1, 2) => &[
            Token::SeqStart(Some(2)),
                Token::SeqSep,
                Token::I32(1),

                Token::SeqSep,
                Token::I32(2),
            Token::SeqEnd,
        ],
    }
    test_path {
        Path::new("/usr/local/lib") => &[
            Token::Str("/usr/local/lib"),
        ],
    }
    test_path_buf {
        PathBuf::from("/usr/local/lib") => &[
            Token::Str("/usr/local/lib"),
        ],
    }
}

#[test]
fn test_cannot_serialize_paths() {
    let path = unsafe {
        str::from_utf8_unchecked(b"Hello \xF0\x90\x80World")
    };
    token::assert_ser_tokens_error(
        &Path::new(path),
        &[Token::Str("Hello �World")],
        token::Error::InvalidValue("Path contains invalid UTF-8 characters".to_owned()));

    let mut path_buf = PathBuf::new();
    path_buf.push(path);

    token::assert_ser_tokens_error(
        &path_buf,
        &[Token::Str("Hello �World")],
        token::Error::InvalidValue("Path contains invalid UTF-8 characters".to_owned()));
}
