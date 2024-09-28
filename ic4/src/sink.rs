#![allow(clippy::redundant_closure)]

use crate::*;

pub mod queue_sink;
pub mod snap_sink;

/*
 * Sink
 */

bind_ptr_type!(
    Sink,
    ic4_sys::IC4_SINK,
    ic4_sys::ic4_sink_ref,
    ic4_sys::ic4_sink_unref
);

pub type SinkType = ic4_sys::IC4_SINK_TYPE;

pub type SinkMode = ic4_sys::IC4_SINK_MODE;

impl Sink {
    pub fn get_mode(&mut self) -> SinkMode {
        SinkMode::from(unsafe { ic4_sys::ic4_sink_get_mode(self.as_mut_ptr()) })
    }
    pub fn set_mode(&mut self, mode: SinkMode) -> self::Result<()> {
        unsafe {
            ic4_sys::ic4_sink_set_mode(self.as_mut_ptr(), mode)
                .then_some(())
                .ok_or_else(|| self::get_last_error())?;
        }
        Ok(())
    }
    pub fn is_attached(&mut self) -> bool {
        unsafe { ic4_sys::ic4_sink_is_attached(self.as_mut_ptr()) }
    }
    pub fn get_type(&mut self) -> SinkType {
        SinkType::from(unsafe { ic4_sys::ic4_sink_get_type(self.as_mut_ptr()) })
    }
}
