use std::marker::PhantomData;

use serde::{ser, Serialize};

use error::Error;
use token::Token;

pub struct Serializer<'a, I>
    where I: Iterator<Item=&'a Token<'a>>,
{
    tokens: I,
    phantom: PhantomData<&'a Token<'a>>,
}

impl<'a, I> Serializer<'a, I>
    where I: Iterator<Item=&'a Token<'a>>,
{
    pub fn new(tokens: I) -> Serializer<'a, I> {
        Serializer {
            tokens: tokens,
            phantom: PhantomData,
        }
    }

    pub fn next_token(&mut self) -> Option<&'a Token<'a>> {
        self.tokens.next()
    }
}

impl<'s, 'a, I> ser::Serializer for &'s mut Serializer<'a, I>
    where I: Iterator<Item=&'a Token<'a>>,
{
    type Ok = ();
    type Error = Error;

    type SerializeSeq = Self;
    type SerializeTuple = Self;
    type SerializeTupleStruct = Self;
    type SerializeMap = Self;
    type SerializeStruct = Self;
    type SerializeEnum = Self;

    fn serialize_bool(self, v: bool) -> Result<(), Error> {
        assert_eq!(self.tokens.next(), Some(&Token::Bool(v)));
        Ok(())
    }

    fn serialize_isize(self, v: isize) -> Result<(), Error> {
        assert_eq!(self.tokens.next(), Some(&Token::Isize(v)));
        Ok(())
    }

    fn serialize_i8(self, v: i8) -> Result<(), Error> {
        assert_eq!(self.tokens.next(), Some(&Token::I8(v)));
        Ok(())
    }

    fn serialize_i16(self, v: i16) -> Result<(), Error> {
        assert_eq!(self.tokens.next(), Some(&Token::I16(v)));
        Ok(())
    }

    fn serialize_i32(self, v: i32) -> Result<(), Error> {
        assert_eq!(self.tokens.next(), Some(&Token::I32(v)));
        Ok(())
    }

    fn serialize_i64(self, v: i64) -> Result<(), Error> {
        assert_eq!(self.tokens.next(), Some(&Token::I64(v)));
        Ok(())
    }

    fn serialize_usize(self, v: usize) -> Result<(), Error> {
        assert_eq!(self.tokens.next(), Some(&Token::Usize(v)));
        Ok(())
    }

    fn serialize_u8(self, v: u8) -> Result<(), Error> {
        assert_eq!(self.tokens.next(), Some(&Token::U8(v)));
        Ok(())
    }

    fn serialize_u16(self, v: u16) -> Result<(), Error> {
        assert_eq!(self.tokens.next(), Some(&Token::U16(v)));
        Ok(())
    }

    fn serialize_u32(self, v: u32) -> Result<(), Error> {
        assert_eq!(self.tokens.next(), Some(&Token::U32(v)));
        Ok(())
    }

    fn serialize_u64(self, v: u64) -> Result<(), Error> {
        assert_eq!(self.tokens.next(), Some(&Token::U64(v)));
        Ok(())
    }

    fn serialize_f32(self, v: f32) -> Result<(), Error> {
        assert_eq!(self.tokens.next(), Some(&Token::F32(v)));
        Ok(())
    }

    fn serialize_f64(self, v: f64) -> Result<(), Error> {
        assert_eq!(self.tokens.next(), Some(&Token::F64(v)));
        Ok(())
    }

    fn serialize_char(self, v: char) -> Result<(), Error> {
        assert_eq!(self.tokens.next(), Some(&Token::Char(v)));
        Ok(())
    }

    fn serialize_str(self, v: &str) -> Result<(), Error> {
        assert_eq!(self.tokens.next(), Some(&Token::Str(v)));
        Ok(())
    }

    fn serialize_bytes(self, value: &[u8]) -> Result<(), Self::Error> {
        assert_eq!(self.tokens.next(), Some(&Token::Bytes(value)));
        Ok(())
    }

    fn serialize_unit(self) -> Result<(), Error> {
        assert_eq!(self.tokens.next(), Some(&Token::Unit));
        Ok(())
    }

    fn serialize_unit_struct(self, name: &str) -> Result<(), Error> {
        assert_eq!(self.tokens.next(), Some(&Token::UnitStruct(name)));
        Ok(())
    }

    fn serialize_newtype_struct<T>(self,
                                   name: &'static str,
                                   value: T) -> Result<(), Error>
        where T: Serialize,
    {
        assert_eq!(self.tokens.next(), Some(&Token::StructNewType(name)));
        value.serialize(self)
    }

    fn serialize_none(self) -> Result<(), Error> {
        assert_eq!(self.tokens.next(), Some(&Token::Option(false)));
        Ok(())
    }

    fn serialize_some<V>(self, value: V) -> Result<(), Error>
        where V: Serialize,
    {
        assert_eq!(self.tokens.next(), Some(&Token::Option(true)));
        value.serialize(self)
    }

    fn serialize_seq(self, len: Option<usize>) -> Result<Self, Error> {
        assert_eq!(self.tokens.next(), Some(&Token::SeqStart(len)));
        Ok(self)
    }

    fn serialize_seq_fixed_size(self, len: usize) -> Result<Self, Error> {
        assert_eq!(self.tokens.next(), Some(&Token::SeqArrayStart(len)));
        Ok(self)
    }

    fn serialize_tuple(self, len: usize) -> Result<Self, Error> {
        assert_eq!(self.tokens.next(), Some(&Token::TupleStart(len)));
        Ok(self)
    }

    fn serialize_tuple_struct(self, name: &'static str, len: usize) -> Result<Self, Error> {
        assert_eq!(self.tokens.next(), Some(&Token::TupleStructStart(name, len)));
        Ok(self)
    }

    fn serialize_map(self, len: Option<usize>) -> Result<Self, Error> {
        assert_eq!(self.tokens.next(), Some(&Token::MapStart(len)));
        Ok(self)
    }

    fn serialize_struct(self, name: &str, len: usize) -> Result<Self, Error> {
        assert_eq!(self.tokens.next(), Some(&Token::StructStart(name, len)));
        Ok(self)
    }

    fn serialize_enum(self, name: &str, variants: &'static [&'static str]) -> Result<Self, Error> {
        assert_eq!(self.tokens.next(), Some(&Token::EnumStart(name, variants)));
        Ok(self)
    }
}

impl<'s, 'a, I> ser::SerializeSeq for &'s mut Serializer<'a, I>
    where I: Iterator<Item=&'a Token<'a>>,
{
    type Ok = ();
    type Error = Error;

    fn serialize_element<T>(&mut self, value: T) -> Result<(), Error>
        where T: Serialize
    {
        assert_eq!(self.tokens.next(), Some(&Token::SeqSep));
        value.serialize(&mut **self)
    }

    fn end(self) -> Result<(), Error> {
        assert_eq!(self.tokens.next(), Some(&Token::SeqEnd));
        Ok(())
    }
}

impl<'s, 'a, I> ser::SerializeTuple for &'s mut Serializer<'a, I>
    where I: Iterator<Item=&'a Token<'a>>,
{
    type Ok = ();
    type Error = Error;

    fn serialize_element<T>(&mut self, value: T) -> Result<(), Error>
        where T: Serialize
    {
        assert_eq!(self.tokens.next(), Some(&Token::TupleSep));
        value.serialize(&mut **self)
    }

    fn end(self) -> Result<(), Error> {
        assert_eq!(self.tokens.next(), Some(&Token::TupleEnd));
        Ok(())
    }
}

impl<'s, 'a, I> ser::SerializeTupleStruct for &'s mut Serializer<'a, I>
    where I: Iterator<Item=&'a Token<'a>>,
{
    type Ok = ();
    type Error = Error;

    fn serialize_field<T>(&mut self, value: T) -> Result<(), Error>
        where T: Serialize
    {
        assert_eq!(self.tokens.next(), Some(&Token::TupleStructSep));
        value.serialize(&mut **self)
    }

    fn end(self) -> Result<(), Error> {
        assert_eq!(self.tokens.next(), Some(&Token::TupleStructEnd));
        Ok(())
    }
}

impl<'s, 'a, I> ser::SerializeTupleVariant for &'s mut Serializer<'a, I>
    where I: Iterator<Item=&'a Token<'a>>,
{
    type Ok = ();
    type Error = Error;

    fn serialize_field<T>(&mut self, value: T) -> Result<(), Error>
        where T: Serialize
    {
        assert_eq!(self.tokens.next(), Some(&Token::EnumSeqSep));
        value.serialize(&mut **self)
    }

    fn end(self) -> Result<(), Error> {
        assert_eq!(self.tokens.next(), Some(&Token::EnumSeqEnd));
        Ok(())
    }
}

impl<'s, 'a, I> ser::SerializeMap for &'s mut Serializer<'a, I>
    where I: Iterator<Item=&'a Token<'a>>,
{
    type Ok = ();
    type Error = Error;

    fn serialize_key<T>(&mut self, key: T) -> Result<(), Self::Error> where T: Serialize {
        assert_eq!(self.tokens.next(), Some(&Token::MapSep));
        key.serialize(&mut **self)
    }

    fn serialize_value<T>(&mut self, value: T) -> Result<(), Self::Error> where T: Serialize {
        value.serialize(&mut **self)
    }

    fn end(self) -> Result<(), Self::Error> {
        assert_eq!(self.tokens.next(), Some(&Token::MapEnd));
        Ok(())
    }
}

impl<'s, 'a, I> ser::SerializeStruct for &'s mut Serializer<'a, I>
    where I: Iterator<Item=&'a Token<'a>>,
{
    type Ok = ();
    type Error = Error;

    fn serialize_field<V>(&mut self, key: &'static str, value: V) -> Result<(), Self::Error> where V: Serialize {
        assert_eq!(self.tokens.next(), Some(&Token::StructSep));
        try!(key.serialize(&mut **self));
        value.serialize(&mut **self)
    }

    fn end(self) -> Result<(), Self::Error> {
        assert_eq!(self.tokens.next(), Some(&Token::StructEnd));
        Ok(())
    }
}

impl<'s, 'a, I> ser::SerializeStructVariant for &'s mut Serializer<'a, I>
    where I: Iterator<Item=&'a Token<'a>>,
{
    type Ok = ();
    type Error = Error;

    fn serialize_field<V>(&mut self, key: &'static str, value: V) -> Result<(), Self::Error> where V: Serialize {
        assert_eq!(self.tokens.next(), Some(&Token::EnumMapSep));
        try!(key.serialize(&mut **self));
        value.serialize(&mut **self)
    }

    fn end(self) -> Result<(), Self::Error> {
        assert_eq!(self.tokens.next(), Some(&Token::EnumMapEnd));
        Ok(())
    }
}

impl<'s, 'a, I> ser::SerializeEnum for &'s mut Serializer<'a, I>
    where I: Iterator<Item=&'a Token<'a>>,
{
    type Ok = ();
    type Error = Error;

    type SerializeTupleVariant = Self;
    type SerializeStructVariant = Self;

    fn serialize_struct_variant(self,
                                variant_index: usize,
                                len: usize) -> Result<Self, Error>
    {
        assert_eq!(self.tokens.next(), Some(&Token::EnumMapStart(variant_index, len)));
        Ok(self)
    }

    fn serialize_unit_variant(self,
                              variant_index: usize) -> Result<(), Error> {
        assert_eq!(self.tokens.next(), Some(&Token::EnumUnit(variant_index)));
        Ok(())
    }

    fn serialize_newtype_variant<T>(self,
                                    variant_index: usize,
                                    value: T) -> Result<(), Error>
        where T: Serialize,
    {
        assert_eq!(self.tokens.next(), Some(&Token::EnumNewType(variant_index)));
        value.serialize(self)
    }

    fn serialize_tuple_variant(self,
                               variant_index: usize,
                               len: usize) -> Result<Self, Error>
    {
        assert_eq!(self.tokens.next(), Some(&Token::EnumSeqStart(variant_index, len)));
        Ok(self)
    }
}
