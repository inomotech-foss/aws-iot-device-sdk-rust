use aws_c_io_sys::{
    aws_event_loop_group, aws_event_loop_group_acquire, aws_event_loop_group_new_default,
    aws_event_loop_group_release,
};

use crate::{AllocatorRef, Result};

ref_counted_wrapper!(struct Inner(aws_event_loop_group) {
    acquire: aws_event_loop_group_acquire,
    release: aws_event_loop_group_release,
});

#[derive(Clone)]
pub struct EventLoopGroup(Inner);

impl EventLoopGroup {
    pub fn new_default(allocator: AllocatorRef, max_threads: u16) -> Result<Self> {
        unsafe {
            Inner::new_or_error(aws_event_loop_group_new_default(
                allocator.as_ptr(),
                max_threads,
                std::ptr::null_mut(),
            ))
        }
        .map(Self)
    }

    pub const fn as_ptr(&self) -> *mut aws_event_loop_group {
        self.0.as_ptr()
    }
}
