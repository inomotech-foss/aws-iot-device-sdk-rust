//! Core c99 package for AWS SDK for C.
//!
//! Includes cross-platform primitives, configuration, data structures, and
//! error handling.

#![no_std]
#![allow(
    clippy::all,
    clippy::wildcard_imports,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    rustdoc::all
)]

#[cfg(target_vendor = "apple")]
use core_foundation_sys::base::CFAllocatorRef;
use libc::*;

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
