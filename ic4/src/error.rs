#![allow(clippy::redundant_closure)]

use crate::*;

pub type ErrorEnum = ic4_sys::IC4_ERROR;

#[derive(Debug, Clone)]
pub struct Error {
    pub err: ErrorEnum,
    pub message: CString,
}

impl From<CString> for Error {
    fn from(value: CString) -> Self {
        Self {
            err: ErrorEnum::IC4_ERROR_NOERROR,
            message: value,
        }
    }
}

pub type Result<T> = std::result::Result<T, self::Error>;

pub fn get_last_error() -> Error {
    unsafe {
        let mut error = ErrorEnum::IC4_ERROR_UNKNOWN;
        let mut message_length = 10 * 1024;
        let mut message_vec: Vec<_> = std::iter::repeat(0u8).take(message_length).collect();
        if !ic4_sys::ic4_get_last_error(
            std::ptr::from_mut(&mut error),
            message_vec.as_mut_ptr() as *mut c_char,
            std::ptr::from_mut(&mut message_length),
        ) {
            return Error {
                err: ErrorEnum::IC4_ERROR_INTERNAL,
                message: CString::from_vec_unchecked(b"Cannot generate ic4::Error!".to_vec()),
            };
        }
        message_vec.resize(message_length - 1, 0); // Remove tailing '\0'
        Error {
            err: error,
            message: CString::from_vec_unchecked(message_vec),
        }
    }
}
