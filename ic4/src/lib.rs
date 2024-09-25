use std::ptr::NonNull;

#[derive(Debug, Clone)]
pub enum IC4Error {
    Failed,
}

pub type IC4Result<T> = Result<T, IC4Error>;

#[allow(unused)]
macro_rules! bind_ptr_type {
    ($name:ident, $inner_ptr_type:path) => {
        #[derive(Debug)]
        #[repr(transparent)]
        pub struct $name {
            pub(crate) inner: std::ptr::NonNull<$inner_ptr_type>,
        }

        impl $name {
            pub(crate) fn new(mut inner: $inner_ptr_type) -> Option<Self> {
                NonNull::new(std::ptr::from_mut(&mut inner)).map(|inner| Self { inner })
            }
            pub fn as_ptr(&mut self) -> *mut $inner_ptr_type {
                self.inner.as_ptr()
            }
        }

        impl AsRef<NonNull<$inner_ptr_type>> for $name {
            fn as_ref(&self) -> &NonNull<$inner_ptr_type> {
                &self.inner
            }
        }

        impl AsMut<NonNull<$inner_ptr_type>> for $name {
            fn as_mut(&mut self) -> &mut NonNull<$inner_ptr_type> {
                &mut self.inner
            }
        }
    };
}

#[derive(Debug)]
pub struct Lib {
    #[allow(unused)]
    status: bool,
}

pub type InitConfig = ic4_sys::IC4_INIT_CONFIG;

impl Lib {
    pub fn create_with_default() -> IC4Result<Self> {
        let default = InitConfig {
            api_log_level: ic4_sys::IC4_LOG_LEVEL::IC4_LOG_INFO,
            internal_log_level: ic4_sys::IC4_LOG_LEVEL::IC4_LOG_INFO,
            log_targets: ic4_sys::IC4_LOG_TARGET_FLAGS::IC4_LOGTARGET_STDOUT,
            log_file: std::ptr::null(),
            reserved0: 0,
        };
        Self::create(default)
    }
    pub fn create(mut config: InitConfig) -> IC4Result<Self> {
        unsafe { ic4_sys::ic4_init_library(std::ptr::from_mut(&mut config)) }
            .then_some(Self { status: true })
            .ok_or(IC4Error::Failed)
    }
}

impl Drop for Lib {
    #[allow(unused)]
    fn drop(&mut self) {
        unsafe { ic4_sys::ic4_exit_library() }
    }
}

pub struct Grabber {
    pub(crate) parent: Lib,
    pub(crate) inner: std::ptr::NonNull<ic4_sys::IC4_GRABBER>,
}

impl Grabber {
    pub fn new(parent: Lib, mut inner: ic4_sys::IC4_GRABBER) -> Option<Self> {
        NonNull::new(std::ptr::from_mut(&mut inner)).map(|inner| Self { parent, inner })
    }
    pub fn parent_ref(&self) -> &Lib {
        &self.parent
    }
    pub fn parent_mut(&mut self) -> &mut Lib {
        &mut self.parent
    }
    pub fn as_ptr(&mut self) -> *mut ic4_sys::IC4_GRABBER {
        self.inner.as_ptr()
    }
}

impl AsRef<NonNull<ic4_sys::IC4_GRABBER>> for Grabber {
    fn as_ref(&self) -> &NonNull<ic4_sys::IC4_GRABBER> {
        &self.inner
    }
}

impl AsMut<NonNull<ic4_sys::IC4_GRABBER>> for Grabber {
    fn as_mut(&mut self) -> &mut NonNull<ic4_sys::IC4_GRABBER> {
        &mut self.inner
    }
}

impl Grabber {}
