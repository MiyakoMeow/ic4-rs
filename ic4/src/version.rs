#![allow(clippy::redundant_closure)]

use crate::*;

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
