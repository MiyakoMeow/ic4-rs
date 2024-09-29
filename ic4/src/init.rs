#![allow(clippy::redundant_closure)]

use crate::*;

#[doc = " @brief The library initialization config structure\n\n Passed to @ref ic4_init_library when initializing the IC Imaging Control 4 C Library."]
pub type InitConfig = ic4_sys::IC4_INIT_CONFIG;

impl DefaultExt for InitConfig {
    fn default_ext() -> Self {
        InitConfig {
            api_log_level: LogLevel::IC4_LOG_WARN,
            internal_log_level: LogLevel::IC4_LOG_WARN,
            log_targets: LogTargetFlags::IC4_LOGTARGET_STDERR,
            log_file: null(),
            reserved0: 0,
        }
    }
}

#[doc = " @brief Initializes the IC Imaging Control 4 C Library\n\n ic4_init_library must be called before any other library function.\n\n @param init_config A structure configuring library settings, e.g. the log level.\n\n @return @c true on success, otherwise @c false."]
pub fn init_library(config: &InitConfig) -> self::Result<()> {
    unsafe {
        ic4_sys::ic4_init_library(ptr_from_ref(config))
            .then_some(())
            .ok_or_else(|| self::get_last_error())?;
    }
    Ok(())
}

#[doc = " @brief Un-initializes the library.\n\n Every successful call to @c ic4_init_library should be balanced by a matching call to @a ic4_exit_library before unloading the library DLL."]
pub fn exit_library() {
    unsafe { ic4_sys::ic4_exit_library() }
}
