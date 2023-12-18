//! s2n-tls is a C99 implementation of the TLS/SSL protocols that is designed to
//! be simple, small, fast, and with security as a priority.

#![no_std]

// used internally
extern crate aws_lc_sys;

mod bindings {
    #![allow(
        clippy::all,
        non_camel_case_types,
        non_snake_case,
        non_upper_case_globals,
        rustdoc::all
    )]

    use libc::*;

    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
}

pub use bindings::*;
