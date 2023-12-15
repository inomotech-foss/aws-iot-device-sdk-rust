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

    // TODO: why is this missing and do we really need it?
    type pthread_once_t = c_int;

    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
}

pub use bindings::*;
