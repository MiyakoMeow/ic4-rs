#![allow(clippy::redundant_closure)]

use crate::*;

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
