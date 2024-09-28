#![allow(clippy::redundant_closure)]

use crate::*;

/*
 * VideoWriter
 */

bind_ptr_type!(
    VideoWriter,
    ic4_sys::IC4_VIDEO_WRITER,
    ic4_sys::ic4_videowriter_ref,
    ic4_sys::ic4_videowriter_unref
);

pub type VideoWriterType = ic4_sys::IC4_VIDEO_WRITER_TYPE;

impl VideoWriter {
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
                ptr_from_ref(&image_type.inner),
                frame_rate,
            )
            .then_some(())
            .ok_or_else(|| self::get_last_error())?;
        }
        Ok(())
    }
    pub fn finish_file(&mut self) -> self::Result<()> {
        unsafe {
            ic4_sys::ic4_videowriter_finish_file(self.as_mut_ptr())
                .then_some(())
                .ok_or_else(|| self::get_last_error())?;
        }
        Ok(())
    }
    pub fn add_frame(&mut self, mut image_buffer: ImageBuffer) -> self::Result<()> {
        unsafe {
            ic4_sys::ic4_videowriter_add_frame(self.as_mut_ptr(), image_buffer.as_mut_ptr())
                .then_some(())
                .ok_or_else(|| self::get_last_error())?;
        }
        Ok(())
    }
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
