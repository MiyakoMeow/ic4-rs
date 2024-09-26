#![allow(clippy::redundant_closure)]

use crate::*;

/*
 * SnapSink
 */

bind_type!(SnapSink, Sink);

pub type SnapSinkAllocationStrategyOri = ic4_sys::IC4_SNAPSINK_ALLOCATION_STRATEGY;
bind_type!(SnapSinkAllocationStrategy, SnapSinkAllocationStrategyOri);

pub type SnapSinkConfigOri = ic4_sys::IC4_SNAPSINK_CONFIG;
bind_type!(SnapSinkConfig, SnapSinkConfigOri);

impl SnapSink {
    pub fn create(config: &SnapSinkConfig) -> self::Result<SnapSink> {
        let mut ptr = null_mut();
        unsafe {
            ic4_sys::ic4_snapsink_create(ptr_from_mut(&mut ptr), ptr_from_ref(&config.inner))
                .then_some(())
                .ok_or_else(|| self::get_last_error())?;
        }
        Ok(SnapSink::from(Sink::from(ptr)))
    }
}

impl SnapSink {
    pub fn get_output_image_type(&self) -> self::Result<ImageType> {
        let mut image_type: ImageType = Default::default();
        unsafe {
            ic4_sys::ic4_snapsink_get_output_image_type(
                self.inner.as_ptr(),
                ptr_from_mut(&mut image_type.inner),
            )
            .then_some(())
            .ok_or_else(|| self::get_last_error())?;
        }
        Ok(image_type)
    }
}

impl SnapSink {
    pub fn snap_single(&mut self, timeout_ms: i64) -> self::Result<ImageBuffer> {
        let mut buffer_ptr = null_mut();
        unsafe {
            ic4_sys::ic4_snapsink_snap_single(
                self.inner.as_mut_ptr(),
                ptr_from_mut(&mut buffer_ptr),
                timeout_ms,
            )
            .then_some(())
            .ok_or_else(|| self::get_last_error())?;
        }
        Ok(buffer_ptr.into())
    }
    // TODO
    // pub fn snap_sequence(&mut self, count: usize, timeout_ms: i64) -> self::Result<ImageBuffer> {
    //     let mut buffer_ptr = null_mut();
    //     unsafe {
    //         ic4_sys::ic4_snapsink_snap_sequence(
    //             self.inner.as_mut_ptr(),
    //             ptr_from_mut(&mut buffer_ptr),
    //             count,
    //             timeout_ms,
    //         )
    //         .then_some(())
    //         .ok_or_else(|| self::get_last_error())?;
    //     }
    //     Ok(buffer_ptr.into())
    // }
}
