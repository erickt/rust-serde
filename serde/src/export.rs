// Copyright 2017 Serde Developers
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

pub use lib::clone::Clone;
pub use lib::convert::{From, Into};
pub use lib::default::Default;
pub use lib::fmt::{self, Formatter};
pub use lib::marker::PhantomData;
pub use lib::option::Option::{self, None, Some};
pub use lib::result::Result::{self, Ok, Err};

pub use self::string::{from_utf8_lossy, from_int, from_bool};

mod string {
    use lib::*;

    #[cfg(any(feature = "std", feature = "alloc"))]
    pub fn from_utf8_lossy(bytes: &[u8]) -> Cow<str> {
        String::from_utf8_lossy(bytes)
    }

    // The generated code calls this like:
    //
    //     let value = &_serde::export::from_utf8_lossy(bytes);
    //     Err(_serde::de::Error::unknown_variant(value, VARIANTS))
    //
    // so it is okay for the return type to be different from the std case as long
    // as the above works.
    #[cfg(not(any(feature = "std", feature = "alloc")))]
    pub fn from_utf8_lossy(bytes: &[u8]) -> &str {
        // Three unicode replacement characters if it fails. They look like a
        // white-on-black question mark. The user will recognize it as invalid
        // UTF-8.
        str::from_utf8(bytes).unwrap_or("\u{fffd}\u{fffd}\u{fffd}")
    }

    pub fn from_bool(b : bool) -> &'static str {
        if b {
            "true"
        } else {
            "false"
        }
    }

    #[cfg(any(feature = "std", feature = "alloc"))]
    pub fn from_int(i: u64) -> Vec<u8> {
        use lib::fmt::Write;
        let mut buf = String::with_capacity(20);
        write!(&mut buf, "{}", i).ok();
        buf.into_bytes()
    }

    #[cfg(not(any(feature = "std", feature = "alloc")))]
    pub fn from_int(i: u64) -> [u8; 20] {
        use lib::fmt::Write;
        // len(str(1<<64)) = 20
        let mut buf = [0; 20];
        {
            let mut wrap = Wrapper { buf: &mut buf };
            write!(wrap, "{}", i).ok();
        }
        buf
    }

    #[cfg(not(any(feature = "std", feature = "alloc")))]
    struct Wrapper<'a> {
        buf: &'a mut [u8],
    }

    #[cfg(not(any(feature = "std", feature = "alloc")))]
    impl<'a> fmt::Write for Wrapper<'a> {
        // Could panic if buf is too small.
        fn write_str(&mut self, s: &str) -> fmt::Result {
            let bytes = s.as_bytes();
            self.buf[..bytes.len()].copy_from_slice(bytes);
            let this : &mut[u8] = mem::replace(&mut self.buf, &mut []);
            self.buf = &mut this[bytes.len()..];
            Ok(())
        }
    }
}
