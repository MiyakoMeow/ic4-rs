#![allow(clippy::redundant_closure)]

use crate::*;

pub mod queue_sink;
pub mod snap_sink;

/*
 * Sink
 */

#[doc = " @struct IC4_SINK\n\n @brief Represents a sink, allowing programmatic access to image data.\n\n This type is opaque, programs only use pointers of type \\c IC4_SINK*.\n\n The \\c IC4_SINK* handle type is used for all sink types, there is no type casting required.\n The actual type of a sink object can be examined by a call to #ic4_sink_get_type().\n\n Sink objects are reference-counted, and created with an initial reference count of one.\n To share a sink object between multiple parts of a program, create a new reference by calling #ic4_sink_ref().\n When a reference is no longer required, call #ic4_sink_unref().\n\n If the sink object's internal reference count reaches zero, the sink object is destroyed."]
pub type SinkOri = ic4_sys::IC4_SINK;
bind_ptr_type!(
    Sink,
    ic4_sys::IC4_SINK,
    ic4_sys::ic4_sink_ref,
    ic4_sys::ic4_sink_unref
);

#[doc = " Identifies the type of a sink"]
pub type SinkType = ic4_sys::IC4_SINK_TYPE;

#[doc = " Defines the possible sink modes"]
pub type SinkMode = ic4_sys::IC4_SINK_MODE;

impl Sink {
    #[doc = " @brief Gets the current sink mode of a sink\n\n @param[in] pSink\tA sink\n\n @return\tA #IC4_SINK_MODE value describing the current sink mode.\\n\n\t\t\tIf an error occurred, the return value is \\c IC4_SINK_MODE_INVALID.\n\t\t\tUse ic4_get_last_error() to query error information.\n"]
    pub fn get_mode(&mut self) -> SinkMode {
        SinkMode::from(unsafe { ic4_sys::ic4_sink_get_mode(self.as_mut_ptr()) })
    }
    #[doc = " @brief Sets the sink mode for a sink.\n\n @param[in] pSink\tA sink\n @param[in] mode\tThe new sink mode\n\n @return \\c true on success, otherwise \\c false.\\n\n\t\t\tUse ic4_get_last_error() to query error information."]
    pub fn set_mode(&mut self, mode: SinkMode) -> self::Result<()> {
        unsafe {
            ic4_sys::ic4_sink_set_mode(self.as_mut_ptr(), mode)
                .then_some(())
                .ok_or_else(|| self::get_last_error())?;
        }
        Ok(())
    }
    #[doc = " Checks whether a sink is currently attached to a @ref grabber.\n\n @param[in] pSink\n\n @return \\c true, if the sink is attached, otherwise \\c false."]
    pub fn is_attached(&mut self) -> bool {
        unsafe { ic4_sys::ic4_sink_is_attached(self.as_mut_ptr()) }
    }
    #[doc = " Returns the actual type of the sink\n\n @param[in] pSink A sink\n\n @return The type of the sink, or \\c IC4_SINK_TYPE_INVALID in case of an error.\\n\n\t\t\tUse ic4_get_last_error() to query error information."]
    pub fn get_type(&mut self) -> SinkType {
        SinkType::from(unsafe { ic4_sys::ic4_sink_get_type(self.as_mut_ptr()) })
    }
}
