// These just test that serde_codegen is able to produce code that compiles
// successfully when there are a variety of generics and non-(de)serializable
// types involved.

extern crate serde;
use self::serde::ser::{Serialize, Serializer};
use self::serde::de::{Deserialize, Deserializer};

use std::marker::PhantomData;

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
    struct Bounds<T: Serialize + Deserialize> {
        t: T,
        option: Option<T>,
        boxed: Box<T>,
        option_boxed: Option<Box<T>>,
    }
    assert::<Bounds<i32>>();

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
            X),
        Tuple(
            T,
            #[serde(serialize_with="ser_x", deserialize_with="de_x")]
            X),
        Struct {
            t: T,
            #[serde(serialize_with="ser_x", deserialize_with="de_x")]
            x: X },
    }
    assert::<EnumWith<i32>>();

    #[derive(Serialize)]
    struct MultipleRef<'a, 'b, 'c, T> where T: 'c, 'c: 'b, 'b: 'a {
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
        X,
    );
    assert::<Tuple<i32>>();

    #[derive(Serialize, Deserialize)]
    enum TreeNode<D> {
        Split {
            left: Box<TreeNode<D>>,
            right: Box<TreeNode<D>>,
        },
        Leaf {
            data: D,
        },
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
}

//////////////////////////////////////////////////////////////////////////

fn assert<T: Serialize + Deserialize>() {}
fn assert_ser<T: Serialize>() {}

trait SerializeWith {
    fn serialize_with<S: Serializer>(_: &Self, _: &mut S) -> Result<(), S::Error>;
}

trait DeserializeWith: Sized {
    fn deserialize_with<D: Deserializer>(_: &mut D) -> Result<Self, D::Error>;
}

// Implements neither Serialize nor Deserialize
struct X;

fn ser_x<S: Serializer>(_: &X, _: &mut S) -> Result<(), S::Error> {
    unimplemented!()
}

fn de_x<D: Deserializer>(_: &mut D) -> Result<X, D::Error> {
    unimplemented!()
}

impl SerializeWith for X {
    fn serialize_with<S: Serializer>(_: &Self, _: &mut S) -> Result<(), S::Error> {
        unimplemented!()
    }
}

impl DeserializeWith for X {
    fn deserialize_with<D: Deserializer>(_: &mut D) -> Result<Self, D::Error> {
        unimplemented!()
    }
}
