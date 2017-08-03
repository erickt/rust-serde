// Copyright 2017 Serde Developer
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

// These just test that serde_codegen is able to produce code that compiles
// successfully when there are a variety of generics and non-(de)serializable
// types involved.

#![cfg_attr(feature = "unstable", feature(non_ascii_idents))]

// Clippy false positive
// https://github.com/Manishearth/rust-clippy/issues/292
#![cfg_attr(feature = "cargo-clippy", allow(needless_lifetimes))]

#[macro_use]
extern crate serde_derive;

extern crate serde;
use self::serde::ser::{Serialize, Serializer};
use self::serde::de::{DeserializeOwned, Deserializer};

use std::borrow::Cow;
use std::marker::PhantomData;
use std::result::Result as StdResult;

// Try to trip up the generated code if it fails to use fully qualified paths.
#[allow(dead_code)]
struct Result;
#[allow(dead_code)]
struct Ok;
#[allow(dead_code)]
struct Err;

//////////////////////////////////////////////////////////////////////////

#[test]
fn test_gen() {
    #[derive(Serialize, Deserialize)]
    struct With<T> {
        t: T,
        #[serde(serialize_with="ser_x", deserialize_with="de_x")]
        x: X,
    }
    assert::<With<i32>>();

    #[derive(Serialize, Deserialize)]
    struct WithTogether<T> {
        t: T,
        #[serde(with="both_x")]
        x: X,
    }
    assert::<WithTogether<i32>>();

    #[derive(Serialize, Deserialize)]
    struct WithRef<'a, T: 'a> {
        #[serde(skip_deserializing)]
        t: Option<&'a T>,
        #[serde(serialize_with="ser_x", deserialize_with="de_x")]
        x: X,
    }
    assert::<WithRef<i32>>();

    #[derive(Serialize, Deserialize)]
    struct PhantomX {
        x: PhantomData<X>,
    }
    assert::<PhantomX>();

    #[derive(Serialize, Deserialize)]
    struct PhantomT<T> {
        t: PhantomData<T>,
    }
    assert::<PhantomT<X>>();

    #[derive(Serialize, Deserialize)]
    struct NoBounds<T> {
        t: T,
        option: Option<T>,
        boxed: Box<T>,
        option_boxed: Option<Box<T>>,
    }
    assert::<NoBounds<i32>>();

    #[derive(Serialize, Deserialize)]
    enum EnumWith<T> {
        Unit,
        Newtype(
            #[serde(serialize_with="ser_x", deserialize_with="de_x")]
            X
        ),
        Tuple(
            T,
            #[serde(serialize_with="ser_x", deserialize_with="de_x")]
            X
        ),
        Struct {
            t: T,
            #[serde(serialize_with="ser_x", deserialize_with="de_x")]
            x: X,
        },
    }
    assert::<EnumWith<i32>>();

    #[derive(Serialize)]
    struct MultipleRef<'a, 'b, 'c, T>
    where
        T: 'c,
        'c: 'b,
        'b: 'a,
    {
        t: T,
        rrrt: &'a &'b &'c T,
    }
    assert_ser::<MultipleRef<i32>>();

    #[derive(Serialize, Deserialize)]
    struct Newtype(
        #[serde(serialize_with="ser_x", deserialize_with="de_x")]
        X
    );
    assert::<Newtype>();

    #[derive(Serialize, Deserialize)]
    struct Tuple<T>(
        T,
        #[serde(serialize_with="ser_x", deserialize_with="de_x")]
        X
    );
    assert::<Tuple<i32>>();

    #[derive(Serialize, Deserialize)]
    enum TreeNode<D> {
        Split {
            left: Box<TreeNode<D>>,
            right: Box<TreeNode<D>>,
        },
        Leaf { data: D },
    }
    assert::<TreeNode<i32>>();

    #[derive(Serialize, Deserialize)]
    struct ListNode<D> {
        data: D,
        next: Box<ListNode<D>>,
    }
    assert::<ListNode<i32>>();

    #[derive(Serialize, Deserialize)]
    struct RecursiveA {
        b: Box<RecursiveB>,
    }
    assert::<RecursiveA>();

    #[derive(Serialize, Deserialize)]
    enum RecursiveB {
        A(RecursiveA),
    }
    assert::<RecursiveB>();

    #[derive(Serialize, Deserialize)]
    struct RecursiveGenericA<T> {
        t: T,
        b: Box<RecursiveGenericB<T>>,
    }
    assert::<RecursiveGenericA<i32>>();

    #[derive(Serialize, Deserialize)]
    enum RecursiveGenericB<T> {
        T(T),
        A(RecursiveGenericA<T>),
    }
    assert::<RecursiveGenericB<i32>>();

    #[derive(Serialize)]
    struct OptionStatic<'a> {
        a: Option<&'a str>,
        b: Option<&'static str>,
    }
    assert_ser::<OptionStatic>();

    #[derive(Serialize, Deserialize)]
    #[serde(bound="D: SerializeWith + DeserializeWith")]
    struct WithTraits1<D, E> {
        #[serde(serialize_with="SerializeWith::serialize_with",
                deserialize_with="DeserializeWith::deserialize_with")]
        d: D,
        #[serde(serialize_with="SerializeWith::serialize_with",
                deserialize_with="DeserializeWith::deserialize_with",
                bound="E: SerializeWith + DeserializeWith")]
        e: E,
    }
    assert::<WithTraits1<X, X>>();

    #[derive(Serialize, Deserialize)]
    #[serde(bound(serialize="D: SerializeWith",
                deserialize="D: DeserializeWith"))]
    struct WithTraits2<D, E> {
        #[serde(serialize_with="SerializeWith::serialize_with",
                deserialize_with="DeserializeWith::deserialize_with")]
        d: D,
        #[serde(serialize_with="SerializeWith::serialize_with",
                bound(serialize="E: SerializeWith"))]
        #[serde(deserialize_with="DeserializeWith::deserialize_with",
                bound(deserialize="E: DeserializeWith"))]
        e: E,
    }
    assert::<WithTraits2<X, X>>();

    #[derive(Serialize, Deserialize)]
    struct CowStr<'a>(Cow<'a, str>);
    assert::<CowStr>();

    #[derive(Serialize, Deserialize)]
    #[serde(bound(deserialize = "T::Owned: DeserializeOwned"))]
    struct CowT<'a, T: ?Sized + 'a + ToOwned>(Cow<'a, T>);
    assert::<CowT<str>>();

    #[derive(Serialize, Deserialize)]
    struct EmptyStruct {}
    assert::<EmptyStruct>();

    #[derive(Serialize, Deserialize)]
    enum EmptyEnumVariant {
        EmptyStruct {},
    }
    assert::<EmptyEnumVariant>();

    #[cfg(feature = "unstable")]
    #[derive(Serialize, Deserialize)]
    struct NonAsciiIdents {
        σ: f64,
    }

    #[derive(Serialize, Deserialize)]
    struct EmptyBraced {}

    #[derive(Serialize, Deserialize)]
    #[serde(deny_unknown_fields)]
    struct EmptyBracedDenyUnknown {}

    #[derive(Serialize, Deserialize)]
    struct BracedSkipAll {
        #[serde(skip_deserializing)]
        f: u8,
    }

    #[derive(Serialize, Deserialize)]
    #[serde(deny_unknown_fields)]
    struct BracedSkipAllDenyUnknown {
        #[serde(skip_deserializing)]
        f: u8,
    }

    #[cfg(feature = "unstable")]
    #[derive(Serialize, Deserialize)]
    struct EmptyTuple();

    #[cfg(feature = "unstable")]
    #[derive(Serialize, Deserialize)]
    #[serde(deny_unknown_fields)]
    struct EmptyTupleDenyUnknown();

    #[derive(Serialize, Deserialize)]
    struct TupleSkipAll(
        #[serde(skip_deserializing)]
        u8
    );

    #[derive(Serialize, Deserialize)]
    #[serde(deny_unknown_fields)]
    struct TupleSkipAllDenyUnknown(
        #[serde(skip_deserializing)]
        u8
    );

    #[derive(Serialize, Deserialize)]
    enum EmptyEnum {}

    #[derive(Serialize, Deserialize)]
    #[serde(deny_unknown_fields)]
    enum EmptyEnumDenyUnknown {}

    #[derive(Serialize, Deserialize)]
    enum EnumSkipAll {
        #[serde(skip_deserializing)]
        #[allow(dead_code)]
        Variant,
    }

    #[cfg(feature = "unstable")]
    #[derive(Serialize, Deserialize)]
    enum EmptyVariants {
        Braced {},
        Tuple(),
        BracedSkip {
            #[serde(skip_deserializing)]
            f: u8,
        },
        TupleSkip(
            #[serde(skip_deserializing)]
            u8
        ),
    }

    #[cfg(feature = "unstable")]
    #[derive(Serialize, Deserialize)]
    #[serde(deny_unknown_fields)]
    enum EmptyVariantsDenyUnknown {
        Braced {},
        Tuple(),
        BracedSkip {
            #[serde(skip_deserializing)]
            f: u8,
        },
        TupleSkip(
            #[serde(skip_deserializing)]
            u8
        ),
    }

    #[derive(Serialize, Deserialize)]
    #[serde(deny_unknown_fields)]
    struct UnitDenyUnknown;

    #[derive(Serialize, Deserialize)]
    struct EmptyArray {
        empty: [X; 0],
    }

    enum Or<A, B> {
        A(A),
        B(B),
    }

    #[derive(Serialize, Deserialize)]
    #[serde(untagged, remote = "Or")]
    enum OrDef<A, B> {
        #[allow(dead_code)]
        A(A),
        #[allow(dead_code)]
        B(B),
    }

    struct Str<'a>(&'a str);

    #[derive(Serialize, Deserialize)]
    #[serde(remote = "Str")]
    struct StrDef<'a>(&'a str);

    #[derive(Serialize, Deserialize)]
    struct Remote<'a> {
        #[serde(with = "OrDef")]
        or: Or<u8, bool>,
        #[serde(borrow, with = "StrDef")]
        s: Str<'a>,
    }

    mod vis {
        pub struct S;

        #[derive(Serialize, Deserialize)]
        #[serde(remote = "S")]
        pub struct SDef;
    }

    // This would not work if SDef::serialize / deserialize are private.
    #[derive(Serialize, Deserialize)]
    struct RemoteVisibility {
        #[serde(with = "vis::SDef")]
        s: vis::S,
    }

    #[derive(Serialize, Deserialize)]
    #[serde(empty_struct)]
    struct SerEmptyStruct;

    #[derive(Serialize, Deserialize)]
    enum SerEmptyStructExternal {
        Hello {
            foo: String,
        },
        #[serde(empty_struct)]
        World,
    }

    #[derive(Serialize, Deserialize)]
    #[serde(tag = "tag")]
    enum SerEmptyStructInternal {
        Hello {
            foo: String,
        },
        #[serde(empty_struct)]
        World,
    }

    #[derive(Serialize, Deserialize)]
    #[serde(tag = "tag", content = "content")]
    enum SerEmptyStructAdjacent {
        Hello {
            foo: String,
        },
        #[serde(empty_struct)]
        World,
    }
}

//////////////////////////////////////////////////////////////////////////

fn assert<T: Serialize + DeserializeOwned>() {}
fn assert_ser<T: Serialize>() {}

trait SerializeWith {
    fn serialize_with<S: Serializer>(_: &Self, _: S) -> StdResult<S::Ok, S::Error>;
}

trait DeserializeWith: Sized {
    fn deserialize_with<'de, D: Deserializer<'de>>(_: D) -> StdResult<Self, D::Error>;
}

// Implements neither Serialize nor Deserialize
pub struct X;

pub fn ser_x<S: Serializer>(_: &X, _: S) -> StdResult<S::Ok, S::Error> {
    unimplemented!()
}

pub fn de_x<'de, D: Deserializer<'de>>(_: D) -> StdResult<X, D::Error> {
    unimplemented!()
}

mod both_x {
    pub use super::{ser_x as serialize, de_x as deserialize};
}

impl SerializeWith for X {
    fn serialize_with<S: Serializer>(_: &Self, _: S) -> StdResult<S::Ok, S::Error> {
        unimplemented!()
    }
}

impl DeserializeWith for X {
    fn deserialize_with<'de, D: Deserializer<'de>>(_: D) -> StdResult<Self, D::Error> {
        unimplemented!()
    }
}
