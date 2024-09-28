#![allow(clippy::redundant_closure)]

use crate::*;

pub type InitConfigOri = ic4_sys::IC4_INIT_CONFIG;
bind_type!(InitConfig, InitConfigOri);

impl Default for InitConfig {
    fn default() -> Self {
        Self::from(InitConfigOri {
            api_log_level: LogLevelOri::IC4_LOG_WARN,
            internal_log_level: LogLevelOri::IC4_LOG_WARN,
            log_targets: LogTargetFlagsOri::IC4_LOGTARGET_STDERR,
            log_file: null(),
            reserved0: 0,
        })
    }
}

pub fn init_library(config: &InitConfig) -> self::Result<()> {
    unsafe {
        ic4_sys::ic4_init_library(ptr_from_ref(&config.inner))
            .then_some(())
            .ok_or_else(|| self::get_last_error())?;
    }
    Ok(())
}

pub fn exit_library() {
    unsafe { ic4_sys::ic4_exit_library() }
}
