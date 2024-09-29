#![allow(clippy::redundant_closure)]

use crate::*;

/*
 * SnapSink
 */

bind_type!(SnapSink, Sink);

#[doc = " @brief The buffer allocation strategy defines how many buffers are pre-allocated, when additional buffers are created,\n and when excess buffers are reclaimed."]
pub type SnapSinkAllocationStrategy = ic4_sys::IC4_SNAPSINK_ALLOCATION_STRATEGY;
impl DefaultExt for SnapSinkAllocationStrategy {
    fn default_ext() -> Self {
        SnapSinkAllocationStrategy::IC4_SNAPSINK_ALLOCATION_STRATEGY_DEFAULT
    }
}

#[doc = " @struct IC4_SNAPSINK_CONFIG\n\n @brief Configures the behavior of a snap sink.\n\n A pointer to a \\c IC4_SNAPSINK_CONFIG is passed to @ref ic4_snapsink_create()."]
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
    #[doc = " @brief Creates a new @ref snapsink.\n\n @param[in] ppSink            Pointer to a sink handle to receive the new snap sink.\n @param[in] config            Pointer to a structure containing the sink configuration.\\n\n                              This parameter is optional; passing @c NULL will use the default configuration.\n\n @return \\c true on success, otherwise \\c false.\\n\n\t\t\tUse ic4_get_last_error() to query error information.\n\n The image type of the images the sink receives is determined when the data stream to the sink is created\n in a call to #ic4_grabber_stream_setup() using the following steps:\n  - If @ref IC4_SNAPSINK_CONFIG::num_pixel_formats is \\c 0, the device format is selected.\n  - If the device's output format matches one of the pixel formats passed in @ref IC4_SNAPSINK_CONFIG::pixel_formats, the first match is selected.\n  - If there is no direct match, but a conversion between the device's output format format and one of the passed pixel formats exists,\n      the first format with a conversion is selected.\n  - If no conversion between the device's output format and any of the values in @ref IC4_SNAPSINK_CONFIG::pixel_formats exists, the stream setup fails."]
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
    #[doc = " @brief Queries the image type of the images the sink is configured to receive.\n\n @param[in] pSink         A queue sink\n @param[out] image_type   A structure receiving the image type information\n\n @return \\c true on success, otherwise \\c false.\\n\n\t\t\tUse ic4_get_last_error() to query error information.\n\n @pre This operation is only valid while there is a data stream from a device to the sink."]
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
    #[doc = " @brief Grabs a single image out of the video stream received from the video capture device.\n\n This function waits until either the next buffer from the video capture device is received, or @c timeout_ms milliseconds have passed.\n If the timeout expires, the function fails and the error value is set to @ref IC4_ERROR_TIMEOUT.\n\n @param[in] pSink             A snap sink\n @param[out] ppImageBuffer    A pointer to a frame handle to receive the newly-filled image\n @param[in] timeout_ms        Time to wait for a new image to arrive\n\n @return \\c true on success, otherwise \\c false.\\n\n\t\t\tUse ic4_get_last_error() to query error information.\n\n @pre\n This operation is only valid while the sink is connected to a device in a data stream.\n\n @note\n After a successfull call, the handle pointed to by \\c ppImageBuffer owns the frame object.\\n\n A call to #ic4_imagebuffer_unref() is required to return the image buffer to the sink for reuse.\\n"]
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
    ///
    #[doc = " @brief Grabs a sequence of images out of the video stream received from the video capture device.\n\n This function waits until @a count images have been grabbed, or @a timeout_ms milliseconds have passed.\n If the timeout expires, the function returns the number of images grabber and the error value is set to @ref IC4_ERROR_TIMEOUT.\n\n @param[in] pSink         A snap sink\n @param[out] pImageBufferList   A pointer to an array of frame handles to receive the newly-filled images\n @param[in] count         Number of images to grab\n @param[in] timeout_ms    Time to wait for all images to arrive\n\n @return  The number of images grabbed successfully.\\n\n          If an error occurred, the function returns @c 0. Use ic4_get_last_error() to query error information.\n\n @pre\n This operation is only valid while the sink is connected to a device in a data stream.\n\n @note\n After a successfull call, the handles pointed to by \\c pImageBufferList own the frame objects.\\n\n A call to #ic4_imagebuffer_unref() is required to return each image buffer to the sink for reuse.\\n"]
    pub unsafe fn snap_sequence(
        &mut self,
        count: usize,
        timeout_ms: i64,
        dst: *mut *mut ic4_sys::IC4_IMAGE_BUFFER,
    ) -> usize {
        ic4_sys::ic4_snapsink_snap_sequence(self.inner.as_mut_ptr(), dst, count, timeout_ms)
    }
}
