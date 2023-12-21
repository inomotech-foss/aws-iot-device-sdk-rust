use aws_c_common_sys::{aws_allocator, aws_default_allocator};

pub type AllocatorRef = &'static Allocator;

#[repr(transparent)]
pub struct Allocator(aws_allocator);

impl Allocator {
    /// # Safety
    ///
    /// `allocator` must be a valid (non-null) pointer with static lifetime.
    #[inline]
    pub(crate) unsafe fn new(allocator: *mut aws_allocator) -> AllocatorRef {
        debug_assert!(!allocator.is_null());
        &mut *allocator.cast()
    }

    #[inline]
    pub fn default() -> AllocatorRef {
        unsafe { Self::new(aws_default_allocator()) }
    }

    #[inline]
    pub const fn as_ptr(&self) -> *mut aws_allocator {
        (&self.0 as *const aws_allocator).cast_mut()
    }
}
