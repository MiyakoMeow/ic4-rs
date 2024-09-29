#![allow(clippy::redundant_closure)]

use crate::*;

/*
 * QueueSink
 */

bind_type!(QueueSink, Sink);

#[doc = " @struct IC4_QUEUESINK_CONFIG\n\n @brief Configures the behavior of a queue sink.\n\n A pointer to a \\c IC4_QUEUESINK_CONFIG is passed to @ref ic4_queuesink_create()."]
pub type QueueSinkConfig = ic4_sys::IC4_QUEUESINK_CONFIG;

impl DefaultExt for QueueSinkConfig {
    fn default_ext() -> Self {
        QueueSinkConfig {
            callbacks: DefaultExt::default_ext(),
            callback_context: null_mut(),
            pixel_formats: ptr_from_ref(&PixelFormat::IC4_PIXEL_FORMAT_Unspecified),
            num_pixel_formats: 0,
            allocator: DefaultExt::default_ext(),
            allocator_context: null_mut(),
            max_output_buffers: 0,
        }
    }
}

#[doc = " Contains function pointers used to specify the behavior of a queue sink."]
pub type QueueSinkCallbacks = ic4_sys::IC4_QUEUESINK_CALLBACKS;

impl DefaultExt for QueueSinkCallbacks {
    fn default_ext() -> Self {
        QueueSinkCallbacks {
            release: None,
            sink_connected: None,
            sink_disconnected: None,
            frames_queued: None,
        }
    }
}

#[doc = " @brief Contains information about the current queue lengths inside the queue sink."]
pub type QueueSinkQueueSizes = ic4_sys::IC4_QUEUESINK_QUEUE_SIZES;

impl DefaultExt for QueueSinkQueueSizes {
    fn default_ext() -> Self {
        QueueSinkQueueSizes {
            free_queue_length: 0,
            output_queue_length: 0,
        }
    }
}

impl QueueSink {
    #[doc = " @brief Creates a new @ref queuesink.\n\n @param[in] ppSink            Pointer to a sink handle to receive the new queue sink.\n @param[in] config            Pointer to a structure containing the sink configuration\n\n @return \\c true on success, otherwise \\c false.\\n\n\t\t\tUse ic4_get_last_error() to query error information.\n\n The image type of the images the sink receives is determined when the data stream to the sink is created\n in a call to #ic4_grabber_stream_setup() using the following steps:\n  - If @ref IC4_QUEUESINK_CONFIG::num_pixel_formats is \\c 0, the device format is selected.\n  - If the device's output format matches one of the pixel formats passed in @ref IC4_QUEUESINK_CONFIG::pixel_formats, the first match is selected.\n  - If there is no direct match, but a conversion between the device's output format format and one of the passed pixel formats exists,\n      the first format with a conversion is selected.\n  - If no conversion between the device's output format and any of the values in @ref IC4_QUEUESINK_CONFIG::pixel_formats exists, the stream setup fails."]
    pub fn create(config: &QueueSinkConfig) -> self::Result<QueueSink> {
        let mut ptr = null_mut();
        unsafe {
            ic4_sys::ic4_queuesink_create(ptr_from_mut(&mut ptr), ptr_from_ref(config))
                .then_some(())
                .ok_or_else(|| self::get_last_error())?;
        }
        Ok(QueueSink::from(Sink::from(ptr)))
    }
}

impl QueueSink {
    #[doc = " @brief Queries the image type of the images the sink is configured to receive.\n\n @param[in] pSink         A queue sink\n @param[out] image_type   A structure receiving the image type information\n\n @return \\c true on success, otherwise \\c false.\\n\n\t\t\tUse ic4_get_last_error() to query error information.\n\n @pre This operation is only valid while there is a data stream from a device to the sink."]
    pub fn get_output_image_type(&self) -> self::Result<ImageType> {
        let mut image_type: ImageType = Default::default();
        unsafe {
            ic4_sys::ic4_queuesink_get_output_image_type(
                self.inner.as_ptr(),
                ptr_from_mut(&mut image_type),
            )
            .then_some(())
            .ok_or_else(|| self::get_last_error())?;
        }
        Ok(image_type)
    }
}

impl QueueSink {
    #[doc = " @brief Allocates a number of buffers matching the sink's image type and puts them into the free queue.\n\n @param[in] pSink         A queue sink\n @param[in] num_buffers   Number of buffers to allocate\n\n @return \\c true on success, otherwise \\c false.\\n\n\t\t\tUse ic4_get_last_error() to query error information.\n\n @pre This operation is only valid while the sink's image type is known. This is the case\\n\n          - inside the @ref IC4_QUEUESINK_CALLBACKS.sink_connected callback function\n          - when the sink was successfully connected to a data stream"]
    pub fn alloc_and_queue_buffers(&mut self, num_buffers: usize) -> self::Result<()> {
        unsafe {
            ic4_sys::ic4_queuesink_alloc_and_queue_buffers(self.inner.as_mut_ptr(), num_buffers)
                .then_some(())
                .ok_or_else(|| self::get_last_error())?;
        }
        Ok(())
    }
}

impl QueueSink {
    #[doc = " @brief Retrieves a buffer that was filled with image data from the sink's output queue.\n\n @param[in] pSink         A queue sink\n @param[out] ppImageBuffer      A pointer to a frame handle to receive the newly-filled image\\n\n\n @return \\c true if a buffer was successfully dequeued, otherwise \\c false.\\n\n\t\t\tUse ic4_get_last_error() to query error information.\n\n @pre\n This operation is only valid while the sink is connected to a device in a data stream.\n\n @note\n The buffers are retrieved in order they were received from the video capture device; the oldest image is\n returned first.\n\n @note\n After a successfull call, the handle pointed to by \\c ppImageBuffer owns the frame object.\\n\n A call to #ic4_imagebuffer_unref() is required to put the image buffer into the sink's free queue for later reuse.\\n"]
    pub fn pop_output_buffer(&mut self) -> self::Result<ImageBuffer> {
        let mut buffer_ptr = null_mut();
        unsafe {
            ic4_sys::ic4_queuesink_pop_output_buffer(
                self.inner.as_mut_ptr(),
                ptr_from_mut(&mut buffer_ptr),
            )
            .then_some(())
            .ok_or_else(|| self::get_last_error())?;
        }
        Ok(buffer_ptr.into())
    }
}

impl QueueSink {
    #[doc = " @brief Checks whether the data stream this sink is connected to is in the process of being stopped.\n\n This function can be used to cancel a long-running operation in the @ref IC4_QUEUESINK_CALLBACKS.frames_queued callback.\n\n @param[in] pSink             A queue sink\n @param[out] cancel_requested Pointer to a flag that receives the cancel request\n\n @return \\c true on success, otherwise \\c false.\\n\n\t\t\tUse ic4_get_last_error() to query error information.\n"]
    pub fn is_cancel_requested(&self) -> self::Result<bool> {
        let mut result = Default::default();
        unsafe {
            ic4_sys::ic4_queuesink_is_cancel_requested(
                self.inner.as_ptr(),
                ptr_from_mut(&mut result),
            )
            .then_some(())
            .ok_or_else(|| self::get_last_error())?;
        }
        Ok(result)
    }
}

impl QueueSink {
    #[doc = " @brief Query information about the number of image buffers in the queue sink's queues.\n\n @param[in] pSink     A queue sink\n @param[out] sizes    A pointer to a structure to receive the queue lengths\n\n @pre\n This operation is only valid while there is a data stream from a device to the sink.\n\n @return \\c true on success, otherwise \\c false.\\n\n\t\t\tUse ic4_get_last_error() to query error information."]
    pub fn get_queue_sizes(&self) -> self::Result<QueueSinkQueueSizes> {
        let mut result: QueueSinkQueueSizes = Default::default();
        unsafe {
            ic4_sys::ic4_queuesink_get_queue_sizes(self.inner.as_ptr(), ptr_from_mut(&mut result))
                .then_some(())
                .ok_or_else(|| self::get_last_error())?;
        }
        Ok(result)
    }
}
