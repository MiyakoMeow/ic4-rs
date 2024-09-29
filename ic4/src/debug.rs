#![allow(clippy::redundant_closure)]

use crate::*;

pub type DebugBufferStats = ic4_sys::IC4_DBG_BUFFER_STATS;

impl DefaultExt for DebugBufferStats {
    fn default_ext() -> Self {
        DebugBufferStats {
            num_announced: 0,
            num_queued: 0,
            num_await_delivery: 0,
        }
    }
}

impl Grabber {
    pub fn debug_device_buffer_stats(&mut self) -> self::Result<DebugBufferStats> {
        let mut property_map: DebugBufferStats = Default::default();
        unsafe {
            ic4_sys::ic4_dbg_grabber_device_buffer_stats(
                self.as_mut_ptr(),
                ptr_from_mut(&mut property_map),
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
                ptr_from_mut(&mut property_map),
            )
            .then_some(())
            .ok_or_else(|| self::get_last_error())?;
        }
        Ok(property_map)
    }
}

#[doc = " @brief Defines the possible library log levels"]
pub type LogLevel = ic4_sys::IC4_LOG_LEVEL;

#[doc = " @brief Defines the possible log targets"]
pub type LogTargetFlags = ic4_sys::IC4_LOG_TARGET_FLAGS;

pub fn debug_count_objects(config: &CStr) -> usize {
    unsafe { ic4_sys::ic4_dbg_count_objects(config.as_ptr()) }
}
