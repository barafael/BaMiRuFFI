/* !allow might be useful if the C lib used doesn't follow those rules */
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

/* Include the automatically generated bindings */
// #[allow(improper_ctypes)] // TODO: where to put this? In build.rs?
include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

use std::ffi::CString;

fn main() {
    println!("Hello World from Rust main function.");
    unsafe {
    hello_world_static();
    }
}
