#![allow(clippy::redundant_closure)]

use crate::*;

pub type DebugBufferStatsOri = ic4_sys::IC4_DBG_BUFFER_STATS;
bind_type!(DebugBufferStats, DebugBufferStatsOri);

impl Default for DebugBufferStats {
    fn default() -> Self {
        DebugBufferStats::from(DebugBufferStatsOri {
            num_announced: 0,
            num_queued: 0,
            num_await_delivery: 0,
        })
    }
}

impl Grabber {
    pub fn debug_device_buffer_stats(&mut self) -> self::Result<DebugBufferStats> {
        let mut property_map: DebugBufferStats = Default::default();
        unsafe {
            ic4_sys::ic4_dbg_grabber_device_buffer_stats(
                self.as_mut_ptr(),
                ptr_from_mut(&mut property_map.inner),
            )
            .then_some(())
            .ok_or_else(|| self::get_last_error())?;
        }
        Ok(property_map)
    }
    pub fn debug_transform_buffer_stats(&mut self) -> self::Result<DebugBufferStats> {
        let mut property_map: DebugBufferStats = Default::default();
        unsafe {
            ic4_sys::ic4_dbg_grabber_transform_buffer_stats(
                self.as_mut_ptr(),
                ptr_from_mut(&mut property_map.inner),
            )
            .then_some(())
            .ok_or_else(|| self::get_last_error())?;
        }
        Ok(property_map)
    }
}

pub type LogLevelOri = ic4_sys::IC4_LOG_LEVEL;
bind_type!(LogLevel, LogLevelOri);
pub type LogTargetFlagsOri = ic4_sys::IC4_LOG_TARGET_FLAGS;
bind_type!(LogTargetFlags, LogTargetFlagsOri);

pub fn debug_count_objects(config: &CStr) -> usize {
    unsafe { ic4_sys::ic4_dbg_count_objects(config.as_ptr()) }
}
