# BaMiRuFFI

### A bare minimum Rust FFI demo

##### Overview
A compiled Rust program is just a binary for a certain architecture. So is a compiled C program.

This means that a function call in Rust looks similar to one in C in terms of the assembler output.

A rust program can call a function in a C program if the two programs are 'compiled together', and this is exactly what the Rust Foreign Function Interface is useful for.

However, to be able to 'see' your C functions from Rust, there needs to be a link which connects your C function calls in Rust to their counterparts in the C program 
(this is needed by the Rust compiler - it wants to know all function signatures at compile time in order to check types). This is usually handled by a header file in C.

Rust-bindgen can automagically translate any C header file you throw at it to an equivalent Rust binding. It still uses raw pointers though.

In the end of the compilation, the gcc crate handles linking both the original C library binary and your Rust binary together so that you can call C functions from Rust.

All those build steps are done in build.rs, and all the tools needed (bindgen and the gcc crate) are only build dependencies [as specified in Cargo.toml]).
