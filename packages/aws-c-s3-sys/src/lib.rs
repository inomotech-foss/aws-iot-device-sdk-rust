//! C99 library implementation for communicating with the S3 service, designed
//! for maximizing throughput on high bandwidth EC2 instances.

#![no_std]

mod bindings {
    #![allow(
        clippy::all,
        non_camel_case_types,
        non_snake_case,
        non_upper_case_globals,
        rustdoc::all
    )]

    use aws_c_auth_sys::*;
    use aws_c_common_sys::*;
    use aws_c_io_sys::*;

    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
}

pub use bindings::*;
