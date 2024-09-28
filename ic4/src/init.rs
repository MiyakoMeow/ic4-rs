#![allow(clippy::redundant_closure)]

use crate::*;

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

pub fn init_library(config: &InitConfig) -> self::Result<()> {
    unsafe {
        ic4_sys::ic4_init_library(ptr_from_ref(config))
            .then_some(())
            .ok_or_else(|| self::get_last_error())?;
    }
    Ok(())
}

pub fn exit_library() {
    unsafe { ic4_sys::ic4_exit_library() }
}
