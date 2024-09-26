#![allow(clippy::redundant_closure)]

#[macro_use]
pub mod macros;

pub mod buffer;
pub mod debug;
pub mod device;
pub mod display;
pub mod error;
pub mod grabber;
pub mod gui;
pub mod init;
pub mod property;
pub mod propid;
pub mod sink;
pub mod traits;
pub mod version;
pub mod video_writer;

#[allow(unused)]
pub use crate::{
    buffer::*, debug::*, device::*, display::*, error::*, grabber::*, init::*,
    macros::bind_type::*, property::*, propid::*, sink::queue_sink::*, sink::snap_sink::*, sink::*,
    traits::*, version::*, video_writer::*,
};

#[allow(unused)]
pub(crate) use std::{
    ffi::{c_char, c_int, c_void, CStr, CString},
    ptr::NonNull,
    ptr::{from_mut as ptr_from_mut, from_ref as ptr_from_ref, null, null_mut},
};
