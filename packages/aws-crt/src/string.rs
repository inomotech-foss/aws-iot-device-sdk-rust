use core::fmt::Display;
use core::ptr::NonNull;

use aws_c_common_sys::{aws_allocator, aws_string, aws_string_destroy, aws_string_new_from_string};

#[repr(transparent)]
pub struct String(NonNull<aws_string>);

impl String {
    #[inline]
    fn allocator(&self) -> Option<NonNull<aws_allocator>> {
        NonNull::new(unsafe { self.0.as_ref().allocator })
    }

    #[inline]
    pub const fn len(&self) -> usize {
        unsafe { self.0.as_ref().len }
    }

    #[inline]
    pub const fn as_ptr(&self) -> *const u8 {
        unsafe { self.0.as_ref().bytes.as_ptr() }
    }

    #[inline]
    pub const fn as_bytes(&self) -> &[u8] {
        unsafe { core::slice::from_raw_parts(self.as_ptr(), self.len()) }
    }

    fn try_clone(&self) -> Option<Self> {
        let Some(allocator) = self.allocator() else {
            // no allocator means this is a static string
            return Some(Self(self.0.clone()));
        };
        NonNull::new(unsafe { aws_string_new_from_string(allocator.as_ptr(), self.0.as_ptr()) })
            .map(Self)
    }
}

impl Drop for String {
    fn drop(&mut self) {
        unsafe { aws_string_destroy(self.0.as_ptr()) };
    }
}

impl AsRef<[u8]> for String {
    fn as_ref(&self) -> &[u8] {
        self.as_bytes()
    }
}

impl Display for String {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}", self.as_bytes().escape_ascii())
    }
}
