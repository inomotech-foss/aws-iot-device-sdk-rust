//! Core c99 package for AWS SDK for C.
//!
//! Includes cross-platform primitives, configuration, data structures, and
//! error handling.

#![no_std]

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
