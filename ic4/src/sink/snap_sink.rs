#![allow(clippy::redundant_closure)]

use crate::*;

/*
 * SnapSink
 */

bind_type!(SnapSink, Sink);

pub type SnapSinkAllocationStrategy = ic4_sys::IC4_SNAPSINK_ALLOCATION_STRATEGY;
impl DefaultExt for SnapSinkAllocationStrategy {
    fn default_ext() -> Self {
        SnapSinkAllocationStrategy::IC4_SNAPSINK_ALLOCATION_STRATEGY_DEFAULT
    }
}

pub type SnapSinkConfig = ic4_sys::IC4_SNAPSINK_CONFIG;

impl DefaultExt for SnapSinkConfig {
    fn default_ext() -> Self {
        SnapSinkConfig {
            strategy: DefaultExt::default_ext(),
            num_buffers_alloc_on_connect: 0,
            num_buffers_allocation_threshold: 0,
            num_buffers_free_threshold: 0,
            num_buffers_max: 0,
            pixel_formats: ptr_from_ref(&PixelFormat::IC4_PIXEL_FORMAT_Unspecified),
            num_pixel_formats: 1,
            allocator: DefaultExt::default_ext(),
            allocator_context: null_mut(),
        }
    }
}

impl SnapSink {
    pub fn create(config: &SnapSinkConfig) -> self::Result<SnapSink> {
        let mut ptr = null_mut();
        unsafe {
            ic4_sys::ic4_snapsink_create(ptr_from_mut(&mut ptr), ptr_from_ref(config))
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
                ptr_from_mut(&mut image_type),
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
    /// TODO Size of ImageBuffer Not Known.
    ///
    /// # Safety
    /// Unknown
    pub unsafe fn snap_sequence(
        &mut self,
        count: usize,
        timeout_ms: i64,
        dst: *mut *mut ic4_sys::IC4_IMAGE_BUFFER,
    ) -> usize {
        ic4_sys::ic4_snapsink_snap_sequence(self.inner.as_mut_ptr(), dst, count, timeout_ms)
    }
}
