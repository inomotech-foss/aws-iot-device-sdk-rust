//! C99 implementation of AWS IoT cloud services integration with devices.

#![no_std]

extern crate aws_c_common_sys;
extern crate aws_c_http_sys;
extern crate aws_c_io_sys;
extern crate aws_c_mqtt_sys;

mod bindings {
    #![allow(
        clippy::all,
        non_camel_case_types,
        non_snake_case,
        non_upper_case_globals,
        rustdoc::all
    )]

    use aws_c_common_sys::*;
    use aws_c_http_sys::*;
    use aws_c_io_sys::*;
    use aws_c_mqtt_sys::*;

    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
}

pub use bindings::*;
