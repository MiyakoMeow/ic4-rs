#![allow(clippy::redundant_closure)]

use crate::*;

#[doc = " @brief Contains the possible error codes"]
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

#[doc = " @brief Query information about the error of the previous library function call\n\n @param[out] pError Pointer to a #IC4_ERROR value to receive the error code.\n @param[out] message Pointer to a character array to receive an error message.<br>\n This parameter is optional and may be \\c NULL.\n @param[in,out] message_length Pointer to a \\c size_t describing the length of the array pointed to by \\a message.<br>\n If \\a message is not \\c NULL, this parameter is required.<br>\n The function always writes the actual number of characters required to store the error message.\n\n @return \\c true on success.\n @return If \\a pError is \\c NULL, the function fails and returns \\c false.\n @return If \\a message is not \\c NULL and \\a message_length is \\c NULL, the function fails and returns \\c false.\n @return If \\a *message_length is lower than the number of characters required to store the error message, the function fails and returns \\c false.\n\n @note\n The last error information is stored in a thread-local way. A call to \\c ic4_get_last_error\n returns error information about the previous function call that happened on the same thread\n that \\c ic4_get_last_error is called from.\n\n @note\n An error while calling \\c ic4_get_last_error does not update the internally stored last error."]
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
