#![allow(clippy::redundant_closure)]

#[macro_use]
pub mod macros;

pub mod buffer;
pub mod debug;
pub mod device;
pub mod display;
pub mod grabber;
pub mod gui;
pub mod init;
pub mod property;
pub mod propid;
pub mod queue_sink;
pub mod sink;
pub mod snap_sink;
pub mod traits;
pub mod video_writer;

#[allow(unused)]
pub use crate::{
    buffer::*, debug::*, device::*, display::*, grabber::*, init::*, macros::bind_type::*,
    property::*, propid::*, queue_sink::*, sink::*, snap_sink::*, traits::*, video_writer::*,
};

#[allow(unused)]
pub(crate) use std::{
    ffi::{c_char, c_int, c_void, CStr, CString},
    ptr::NonNull,
    ptr::{from_mut as ptr_from_mut, from_ref as ptr_from_ref, null, null_mut},
};

pub type ErrorEnumOri = ic4_sys::IC4_ERROR;
bind_type!(ErrorEnum, ErrorEnumOri);

#[derive(Debug, Clone)]
pub struct Error {
    pub err: ErrorEnum,
    pub message: CString,
}

impl From<CString> for Error {
    fn from(value: CString) -> Self {
        Self {
            err: ErrorEnumOri::IC4_ERROR_NOERROR.into(),
            message: value,
        }
    }
}

pub type Result<T> = std::result::Result<T, self::Error>;

pub fn get_last_error() -> Error {
    unsafe {
        let mut error = ic4_sys::IC4_ERROR::IC4_ERROR_UNKNOWN;
        let mut message_buffer = vec![0u8; 1024 * 1024];
        let mut message_length = 0;
        if !ic4_sys::ic4_get_last_error(
            std::ptr::from_mut(&mut error),
            message_buffer.as_mut_ptr() as *mut std::ffi::c_char,
            std::ptr::from_mut(&mut message_length),
        ) {
            return Error {
                err: ErrorEnumOri::IC4_ERROR_INTERNAL.into(),
                message: CString::from_vec_unchecked(b"Cannot generate ic4::Error!".to_vec()),
            };
        }
        let message = CString::from_vec_unchecked(message_buffer);
        Error {
            err: error.into(),
            message,
        }
    }
}

pub type VersionInfoFlags = ic4_sys::IC4_VERSION_INFO_FLAGS;

pub fn get_version_info(flags: VersionInfoFlags) -> self::Result<CString> {
    let mut message = [0; 1024];
    let mut message_length = 0;
    unsafe {
        ic4_sys::ic4_get_version_info(
            message.as_mut_ptr(),
            ptr_from_mut(&mut message_length),
            flags,
        )
        .then_some(())
        .ok_or_else(|| self::get_last_error())?;
        Ok(CString::from_vec_unchecked({
            message[..message_length].iter().map(|a| *a as u8).collect()
        }))
    }
}
