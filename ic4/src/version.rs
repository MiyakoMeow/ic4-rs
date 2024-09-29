#![allow(clippy::redundant_closure)]

use crate::*;

#[doc = " @brief Contains retrievable version descriptions"]
pub type VersionInfoFlags = ic4_sys::IC4_VERSION_INFO_FLAGS;

    #[doc = " @brief Retrieve version information description string\n\n @param[out] str         Pointer to a character array to receive an error message.\n @param[in,out] size     Size of str buffer\n @param[in] flags        What version information to retrieve\n\n @return \\c true on success"]
pub fn get_version_info(flags: VersionInfoFlags) -> self::Result<CString> {
    let mut message_length = 10 * 1024;
    let mut message_vec = vec![0; 10 * 1024];
    unsafe {
        ic4_sys::ic4_get_version_info(
            message_vec.as_mut_ptr() as *mut c_char,
            ptr_from_mut(&mut message_length),
            flags,
        )
        .then_some(())
        .ok_or_else(|| self::get_last_error())?;
        message_vec.resize(message_length - 1, 0);
        Ok(CString::from_vec_unchecked(message_vec))
    }
}
