#![no_std]

extern crate aws_c_common_sys;
#[cfg(all(unix, not(target_os = "macos")))]
extern crate aws_lc_sys;
#[cfg(windows)]
extern crate schannel;
#[cfg(target_os = "macos")]
extern crate security_framework;

mod bindings {
    #![allow(
        clippy::all,
        non_camel_case_types,
        non_snake_case,
        non_upper_case_globals,
        rustdoc::all
    )]
    use aws_c_common_sys::*;

    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
}

pub use bindings::*;
