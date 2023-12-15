//! C99 implementation of the vnd.amazon.event-stream content-type.

#![no_std]

mod bindings {
    #![allow(
        clippy::all,
        non_camel_case_types,
        non_snake_case,
        non_upper_case_globals,
        rustdoc::all
    )]

    use aws_c_common_sys::*;
    use libc::*;

    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
}

pub use bindings::*;
