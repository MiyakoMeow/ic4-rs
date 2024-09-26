#![allow(clippy::redundant_closure)]

use crate::*;

/*
 * Sink
 */

bind_ptr_type!(
    Sink,
    ic4_sys::IC4_SINK,
    ic4_sys::ic4_sink_ref,
    ic4_sys::ic4_sink_unref
);

pub type SinkTypeOri = ic4_sys::IC4_SINK_TYPE;
bind_type!(SinkType, SinkTypeOri);

pub type SinkModeOri = ic4_sys::IC4_SINK_MODE;
bind_type!(SinkMode, SinkModeOri);

impl Default for SinkMode {
    fn default() -> Self {
        Self::from(SinkModeOri::IC4_SINK_MODE_INVALID)
    }
}

impl Sink {
    pub fn get_mode(&mut self) -> SinkMode {
        SinkMode::from(unsafe { ic4_sys::ic4_sink_get_mode(self.as_mut_ptr()) })
    }
    pub fn set_mode(&mut self, mode: SinkMode) -> self::Result<()> {
        unsafe {
            ic4_sys::ic4_sink_set_mode(self.as_mut_ptr(), mode.inner)
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
