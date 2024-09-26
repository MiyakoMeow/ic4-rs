macro_rules! bind_type {
    ($name:ident, $inner_type:path) => {
        #[derive(Debug, Clone)]
        pub struct $name {
            pub(crate) inner: $inner_type,
        }

        impl AsRef<$inner_type> for $name {
            fn as_ref(&self) -> &$inner_type {
                &self.inner
            }
        }

        impl AsMut<$inner_type> for $name {
            fn as_mut(&mut self) -> &mut $inner_type {
                &mut self.inner
            }
        }

        impl From<$inner_type> for $name {
            fn from(value: $inner_type) -> $name {
                $name { inner: value }
            }
        }

        impl From<$name> for $inner_type {
            fn from(value: $name) -> $inner_type {
                value.inner
            }
        }
    };
}

macro_rules! bind_ptr_type {
    ($name:ident, $inner_ptr_type:path, $fn_ref:path, $fn_unref:path) => {
        #[derive(Debug)]
        pub struct $name {
            inner: std::ptr::NonNull<$inner_ptr_type>,
        }

        impl $name {
            pub fn as_ptr(&self) -> *const $inner_ptr_type {
                self.inner.as_ptr()
            }
            pub fn as_mut_ptr(&mut self) -> *mut $inner_ptr_type {
                self.inner.as_ptr()
            }
        }

        impl AsRef<std::ptr::NonNull<$inner_ptr_type>> for $name {
            fn as_ref(&self) -> &std::ptr::NonNull<$inner_ptr_type> {
                &self.inner
            }
        }

        impl AsMut<std::ptr::NonNull<$inner_ptr_type>> for $name {
            fn as_mut(&mut self) -> &mut std::ptr::NonNull<$inner_ptr_type> {
                &mut self.inner
            }
        }

        impl From<*mut $inner_ptr_type> for $name {
            fn from(value: *mut $inner_ptr_type) -> $name {
                $name {
                    inner: std::ptr::NonNull::new(value)
                        .expect("ref macros: Failed to create vaild ref!"),
                }
            }
        }

        impl From<$name> for *mut $inner_ptr_type {
            fn from(mut value: $name) -> *mut $inner_ptr_type {
                value.as_mut_ptr()
            }
        }

        impl Clone for $name {
            fn clone(&self) -> $name {
                unsafe { $fn_ref(self.inner.as_ptr()).into() }
            }
        }

        impl Drop for $name {
            fn drop(&mut self) {
                unsafe {
                    $fn_unref(self.as_mut_ptr());
                }
            }
        }
    };
}
