#![allow(clippy::redundant_closure)]

use crate::*;

/*
 * VideoWriter
 */

#[doc = " @struct IC4_VIDEO_WRITER\n\n @brief Represents a video writer\n\n This type is opaque, programs only use pointers of type \\c IC4_VIDEO_WRITER*.\n\n Video writer objects are reference-counted.\n To increase the reference count, use #ic4_videowriter_ref().\n Call ic4_videowriter_unref() when a reference is no longer required."]
pub type VideoWriterOri = ic4_sys::IC4_VIDEO_WRITER;
bind_ptr_type!(
    VideoWriter,
    ic4_sys::IC4_VIDEO_WRITER,
    ic4_sys::ic4_videowriter_ref,
    ic4_sys::ic4_videowriter_unref
);

#[doc = " Defines the available video writer types"]
pub type VideoWriterType = ic4_sys::IC4_VIDEO_WRITER_TYPE;

impl DefaultExt for VideoWriterType {
    fn default_ext() -> Self {
        Self::IC4_VIDEO_WRITER_MP4_H264
    }
}

impl VideoWriter {
    #[doc = " @brief Creates a new video writer of the specified type.\n\n @param[in] type\t\t\t\tSelects the type of video writer to create\n @param[out] ppVideoWriter\tPointer to receive the handle of the new video writer\\n\n\t\t\t\t\t\t\t\tWhen the video writer is no longer required, release the object reference using #ic4_videowriter_unref().\n\n @return \\c true on success, otherwise \\c false.\\n\n\t\t\tUse ic4_get_last_error() to query error information."]
    pub fn create(video_writer_type: VideoWriterType) -> self::Result<Self> {
        let mut self_ptr = null_mut();
        unsafe {
            ic4_sys::ic4_videowriter_create(video_writer_type, ptr_from_mut(&mut self_ptr))
                .then_some(())
                .ok_or_else(|| self::get_last_error())?;
        }
        Ok(self_ptr.into())
    }
}

impl VideoWriter {
    #[doc = " @brief Opens a new video file ready to write images into.\n\n @param[in] pVideoWriter\tA video writer\n @param[in] file_name\t\tName of the new video file\n @param[in] image_type\tImage type of the images that are going to be written\n @param[in] frame_rate\tPlayback frame rate of the video file\n\n @return \\c true on success, otherwise \\c false.\\n\n\t\t\tUse ic4_get_last_error() to query error information."]
    pub fn begin_file(
        &mut self,
        file_name: &CStr,
        image_type: &ImageType,
        frame_rate: f64,
    ) -> self::Result<()> {
        unsafe {
            ic4_sys::ic4_videowriter_begin_file(
                self.as_mut_ptr(),
                file_name.as_ptr(),
                ptr_from_ref(image_type),
                frame_rate,
            )
            .then_some(())
            .ok_or_else(|| self::get_last_error())?;
        }
        Ok(())
    }
    #[doc = " @brief Finishes writing a video file.\n\n @param[in] pVideoWriter\tA video writer that previously began writing a file\n\n @return \\c true on success, otherwise \\c false.\\n\n\t\t\tUse ic4_get_last_error() to query error information."]
    pub fn finish_file(&mut self) -> self::Result<()> {
        unsafe {
            ic4_sys::ic4_videowriter_finish_file(self.as_mut_ptr())
                .then_some(())
                .ok_or_else(|| self::get_last_error())?;
        }
        Ok(())
    }
    #[doc = " @brief Adds an image to the currently open video file.\n\n @param[in] pVideoWriter\tA video writer that previously began writing a file\n @param[in] buffer\t\tAn image buffer\n\n @note\n The image buffer's image type must be equal to the \\c image_type parameter passed to #ic4_videowriter_begin_file() when starting the file.\\n\n The video writer can retain a reference to the image buffer. This can delay the release and possible reuse of the image buffer.\n In this case, the buffer becomes shared, and is no longer safely writable (see @ref ic4_imagebuffer_is_writable).\\n\n Use @ref ic4_videowriter_add_frame_copy to always let the video writer immediately copy the data out of the image buffer.\n\n @return \\c true on success, otherwise \\c false.\\n\n\t\t\tUse ic4_get_last_error() to query error information."]
    pub fn add_frame(&mut self, image_buffer: ImageBuffer) -> self::Result<()> {
        unsafe {
            ic4_sys::ic4_videowriter_add_frame(
                self.as_mut_ptr(),
                image_buffer.clone().as_mut_ptr(),
            )
            .then_some(())
            .ok_or_else(|| self::get_last_error())?;
        }
        Ok(())
    }
    #[doc = " @brief Adds an image to the currently open video file, copying its contents in the process.\n\n @param[in] pVideoWriter\tA video writer that previously began writing a file\n @param[in] buffer\t\tAn image buffer\n\n @note\n The image buffer's image type must be equal to the \\c image_type parameter passed to #ic4_videowriter_begin_file() when starting the file.\\n\n The image buffer's contents will be copied, so that the buffer's reference count is not increased and it can be reused immedietely if\n the final reference is released.\\n\n Use @ref ic4_videowriter_add_frame to avoid the copy operation if it is not necessary.\n\n @return \\c true on success, otherwise \\c false.\\n\n\t\t\tUse ic4_get_last_error() to query error information."]
    pub fn add_frame_copy(&mut self, image_buffer: &ImageBuffer) -> self::Result<()> {
        unsafe {
            ic4_sys::ic4_videowriter_add_frame_copy(self.as_mut_ptr(), image_buffer.as_ptr())
                .then_some(())
                .ok_or_else(|| self::get_last_error())?;
        }
        Ok(())
    }
}

impl VideoWriter {
    #[doc = " @brief Returns the property map for encoder configuration.\n\n @param[in] pVideoWriter\t\tA video writer\n @param[out] ppPropertyMap\tPointer to a handle that receives the property map.\\n\n\t\t\t\t\t\t\t\tWhen the property map is longer required, call #ic4_propmap_unref().\n\n @return \\c true on success, otherwise \\c false.\\n\n\t\t\tUse ic4_get_last_error() to query error information."]
    pub fn get_property_map(&mut self) -> self::Result<PropertyMap> {
        let mut result_ptr = null_mut();
        unsafe {
            ic4_sys::ic4_videowriter_get_property_map(
                self.as_mut_ptr(),
                ptr_from_mut(&mut result_ptr),
            )
            .then_some(())
            .ok_or_else(|| self::get_last_error())?;
        }
        Ok(result_ptr.into())
    }
}
