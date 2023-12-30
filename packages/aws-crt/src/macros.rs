macro_rules! ref_counted_wrapper {
    ($vis:vis struct $wrapper_name:ident($inner_ty:ty) {
        acquire: $acquire_fn:path,
        release: $release_fn:path,
    }) => {
        $vis struct $wrapper_name(::std::ptr::NonNull<$inner_ty>);

        impl $wrapper_name {
            #[inline]
            pub unsafe fn new_or_error(inner: *mut $inner_ty) -> $crate::Result<Self> {
                match ::std::ptr::NonNull::new(inner) {
                    Some(inner) => Ok(Self(inner)),
                    None => Err($crate::Error::last_in_current_thread()),
                }
            }

            #[inline]
            pub const fn as_ptr(&self) -> *mut $inner_ty {
                self.0.as_ptr()
            }
        }

        impl Clone for $wrapper_name {
            fn clone(&self) -> Self {
                let inner = unsafe { ::std::ptr::NonNull::new_unchecked($acquire_fn(self.as_ptr())) };
                Self(inner)
            }
        }

        impl Drop for $wrapper_name {
            fn drop(&mut self) {
                unsafe { $release_fn(self.as_ptr()) };
            }
        }

        impl ::std::fmt::Debug for $wrapper_name {
            fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                write!(f, "{}({:p})", ::std::stringify!($inner_ty), self.as_ptr())
            }
        }
    };
}
