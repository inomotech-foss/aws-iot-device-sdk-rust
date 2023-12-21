use std::fmt::{Debug, Display};
use std::ops::{Deref, DerefMut};
use std::ptr::NonNull;

use aws_c_common_sys::{aws_string, aws_string_destroy};

use crate::{Allocator, AllocatorRef};

#[repr(transparent)]
pub struct AwsStr(aws_string);

impl AwsStr {
    #[inline]
    fn allocator(&self) -> Option<AllocatorRef> {
        if self.0.allocator.is_null() {
            None
        } else {
            Some(unsafe { Allocator::new(self.0.allocator) })
        }
    }

    #[inline]
    pub fn as_mut_ptr(&mut self) -> *mut aws_string {
        &mut self.0
    }

    #[inline]
    pub const fn len(&self) -> usize {
        self.0.len
    }

    #[inline]
    pub const fn is_empty(&self) -> bool {
        self.len() == 0
    }

    #[inline]
    pub const fn as_bytes_ptr(&self) -> *const u8 {
        self.0.bytes.as_ptr()
    }

    #[inline]
    pub const fn as_bytes(&self) -> &[u8] {
        unsafe { std::slice::from_raw_parts(self.as_bytes_ptr(), self.len()) }
    }
}

impl AsRef<[u8]> for AwsStr {
    fn as_ref(&self) -> &[u8] {
        self.as_bytes()
    }
}

impl Debug for AwsStr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "\"{}\"", self.as_bytes().escape_ascii())
    }
}

impl Display for AwsStr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_bytes().escape_ascii())
    }
}

#[repr(transparent)]
pub struct AwsString(NonNull<AwsStr>);

impl AwsString {
    #[inline]
    fn as_str(&self) -> &AwsStr {
        unsafe { self.0.as_ref() }
    }

    #[inline]
    fn as_str_mut(&mut self) -> &mut AwsStr {
        unsafe { self.0.as_mut() }
    }
}

impl Drop for AwsString {
    fn drop(&mut self) {
        unsafe { aws_string_destroy(self.as_mut_ptr()) };
    }
}

impl Debug for AwsString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(self.as_str(), f)
    }
}

impl Display for AwsString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Display::fmt(self.as_str(), f)
    }
}

impl Deref for AwsString {
    type Target = AwsStr;

    fn deref(&self) -> &Self::Target {
        self.as_str()
    }
}

impl DerefMut for AwsString {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.as_str_mut()
    }
}
