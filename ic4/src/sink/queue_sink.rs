#![allow(clippy::redundant_closure)]

use crate::*;

/*
 * QueueSink
 */

bind_type!(QueueSink, Sink);

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
