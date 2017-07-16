#![feature(lang_items, start, compiler_builtins_lib)]
#![no_std]

extern crate libc;
extern crate compiler_builtins;

#[start]
fn start(_argc: isize, _argv: *const *const u8) -> isize {
    0
}

#[lang = "eh_unwind_resume"]
#[no_mangle]
pub extern fn rust_eh_unwind_resume() {}


//////////////////////////////////////////////////////////////////////////////

#[macro_use]
extern crate serde_derive;

#[derive(Serialize, Deserialize)]
struct Unit;

#[derive(Serialize, Deserialize)]
struct Newtype(u8);

#[derive(Serialize, Deserialize)]
struct Tuple(u8, u8);

#[derive(Serialize, Deserialize)]
struct Struct { f: u8 }

#[derive(Serialize, Deserialize)]
enum Enum {
    Unit,
    Newtype(u8),
    Tuple(u8, u8),
    Struct { f: u8 },
}
