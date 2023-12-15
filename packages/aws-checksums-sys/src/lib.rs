//! Cross-Platform HW accelerated CRC32c and CRC32 with fallback to efficient SW
//! implementations.

#![no_std]

mod bindings {
    #![allow(
        clippy::all,
        non_camel_case_types,
        non_snake_case,
        non_upper_case_globals,
        rustdoc::all
    )]

    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
}

pub use bindings::*;
