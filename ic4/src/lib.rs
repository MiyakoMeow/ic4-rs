#[macro_use]
pub mod macros;

pub mod buffer;
pub mod device;
pub mod display;
pub mod error;
pub mod grabber;
pub mod property;
pub mod propid;
pub mod traits;

#[allow(unused)]
pub use crate::{
    buffer::*, device::*, display::*, error::*, grabber::*, macros::*, property::*, propid::*,
    traits::*,
};

#[allow(unused)]
pub(crate) use std::{
    ffi::{c_char, c_int, c_void, CStr, CString},
    ptr::NonNull,
    ptr::{from_mut as ptr_from_mut, from_ref as ptr_from_ref, null, null_mut},
};
