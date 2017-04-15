/* !allow might be useful if the C lib used doesn't follow those rules */
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

/* Include the automatically generated bindings */
#[allow(improper_ctypes)] // TODO: Right place here? What does this do?
include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

extern crate libc;

use libc::{c_char, uint32_t};
use std::ffi::CStr;
use std::str;

fn main() {
    println!("Hello World from Rust main function.");
    unsafe {
    hello_world_static();
    let greeting: *mut i8 = hello_world_return_pointer();
    println!("{}", how_many_characters(greeting));
    }
}

#[no_mangle]
pub extern fn how_many_characters(s: *const c_char) -> uint32_t {
    let c_str = unsafe {
        assert!(!s.is_null());

        CStr::from_ptr(s)
    };

    let r_str = c_str.to_str().unwrap();
    r_str.chars().count() as uint32_t
}
