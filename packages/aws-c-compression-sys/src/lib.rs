//! This is a cross-platform C99 implementation of compression algorithms such
//! as gzip, and huffman encoding/decoding. Currently only huffman is
//! implemented.

extern crate aws_c_common_sys;

mod bindings {
    #![allow(
        clippy::all,
        clippy::wildcard_imports,
        non_camel_case_types,
        non_snake_case,
        non_upper_case_globals,
        rustdoc::all
    )]
    use aws_c_common_sys::*;

    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
}

pub use bindings::*;
