//! Helper module to enable serializing bytes more efficiently

use std::ops;
use std::fmt;
use std::ascii;

use ser;
use de;

///////////////////////////////////////////////////////////////////////////////

/// `Bytes` wraps a `&[u8]` in order to serialize into a byte array.
#[derive(Clone, Copy, Eq, Hash, PartialEq, PartialOrd, Ord)]
pub struct Bytes<'a> {
    bytes: &'a [u8],
}

impl<'a> fmt::Debug for Bytes<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "b\"{}\"", escape_bytestring(self.bytes))
    }
}

impl<'a> From<&'a [u8]> for Bytes<'a> {
    fn from(bytes: &'a [u8]) -> Self {
        Bytes {
            bytes: bytes,
        }
    }
}

impl<'a> From<&'a Vec<u8>> for Bytes<'a> {
    fn from(bytes: &'a Vec<u8>) -> Self {
        Bytes {
            bytes: &bytes,
        }
    }
}

impl<'a> Into<&'a [u8]> for Bytes<'a> {
    fn into(self) -> &'a [u8] {
        self.bytes
    }
}

impl<'a> ops::Deref for Bytes<'a> {
    type Target = [u8];

    fn deref(&self) -> &[u8] { self.bytes }
}

impl<'a> ser::Serialize for Bytes<'a> {
    #[inline]
    fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error>
        where S: ser::Serializer
    {
        serializer.serialize_bytes(self.bytes)
    }
}

///////////////////////////////////////////////////////////////////////////////

/// `ByteBuf` wraps a `Vec<u8>` and serializes as a byte array.
#[derive(Clone, Eq, Hash, PartialEq, PartialOrd, Ord)]
pub struct ByteBuf {
    bytes: Vec<u8>,
}

impl ByteBuf {
    /// Construct a new, empty `ByteBuf`.
    pub fn new() -> Self {
        ByteBuf {
            bytes: Vec::new(),
        }
    }

    /// Construct a new, empty `ByteBuf` with the specified capacity.
    pub fn with_capacity(cap: usize) -> Self {
        ByteBuf {
            bytes: Vec::with_capacity(cap)
        }
    }
}

impl fmt::Debug for ByteBuf {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "b\"{}\"", escape_bytestring(self.bytes.as_ref()))
    }
}

impl Into<Vec<u8>> for ByteBuf {
    fn into(self) -> Vec<u8> {
        self.bytes
    }
}

impl From<Vec<u8>> for ByteBuf {
    fn from(bytes: Vec<u8>) -> Self {
        ByteBuf {
            bytes: bytes,
        }
    }
}

impl AsRef<Vec<u8>> for ByteBuf {
    fn as_ref(&self) -> &Vec<u8> {
        &self.bytes
    }
}

impl AsRef<[u8]> for ByteBuf {
    fn as_ref(&self) -> &[u8] {
        &self.bytes
    }
}

impl AsMut<Vec<u8>> for ByteBuf {
    fn as_mut(&mut self) -> &mut Vec<u8> {
        &mut self.bytes
    }
}

impl AsMut<[u8]> for ByteBuf {
    fn as_mut(&mut self) -> &mut [u8] {
        &mut self.bytes
    }
}

impl ops::Deref for ByteBuf {
    type Target = [u8];

    fn deref(&self) -> &[u8] { &self.bytes[..] }
}

impl ops::DerefMut for ByteBuf {
    fn deref_mut(&mut self) -> &mut [u8] { &mut self.bytes[..] }
}

impl ser::Serialize for ByteBuf {
    fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error>
        where S: ser::Serializer
    {
        serializer.serialize_bytes(&self)
    }
}

/// This type implements the `serde::de::Visitor` trait for a `ByteBuf`.
pub struct ByteBufVisitor;

impl de::Visitor for ByteBufVisitor {
    type Value = ByteBuf;

    #[inline]
    fn visit_unit<E>(&mut self) -> Result<ByteBuf, E>
        where E: de::Error,
    {
        Ok(ByteBuf {
            bytes: Vec::new(),
        })
    }

    #[inline]
    fn visit_seq<V>(&mut self, mut visitor: V) -> Result<ByteBuf, V::Error>
        where V: de::SeqVisitor,
    {
        let (len, _) = visitor.size_hint();
        let mut values = Vec::with_capacity(len);

        while let Some(value) = try!(visitor.visit()) {
            values.push(value);
        }

        try!(visitor.end());

        Ok(ByteBuf {
            bytes: values,
        })
    }

    #[inline]
    fn visit_bytes<E>(&mut self, v: &[u8]) -> Result<ByteBuf, E>
        where E: de::Error,
    {
        self.visit_byte_buf(v.to_vec())
    }

    #[inline]
    fn visit_byte_buf<E>(&mut self, v: Vec<u8>) -> Result<ByteBuf, E>
        where E: de::Error,
    {
        Ok(ByteBuf {
            bytes: v,
        })
    }
}

impl de::Deserialize for ByteBuf {
    #[inline]
    fn deserialize<D>(deserializer: &mut D) -> Result<ByteBuf, D::Error>
        where D: de::Deserializer
    {
        deserializer.deserialize_bytes(ByteBufVisitor)
    }
}

///////////////////////////////////////////////////////////////////////////////

fn escape_bytestring(bytes: &[u8]) -> String {
    let mut result = String::new();
    for &b in bytes {
        for esc in ascii::escape_default(b) {
            result.push(esc as char);
        }
    }
    result
}
