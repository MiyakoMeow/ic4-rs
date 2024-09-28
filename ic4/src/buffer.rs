#![allow(clippy::redundant_closure)]

use crate::*;

/*
 * PixelFormat
 */

pub type PixelFormatOri = ic4_sys::IC4_PIXEL_FORMAT;
bind_type!(PixelFormat, PixelFormatOri);

impl ToCStr for PixelFormat {
    fn to_cstr(&self) -> &CStr {
        unsafe { CStr::from_ptr(ic4_sys::ic4_pixelformat_tostring(self.inner)) }
    }
}

impl PixelFormat {
    pub fn bpp(&self) -> usize {
        unsafe { ic4_sys::ic4_pixelformat_bpp(self.inner) }
    }
}

/*
 * ImageType
 */
pub type ImageTypeOri = ic4_sys::IC4_IMAGE_TYPE;
bind_type!(ImageType, ImageTypeOri);

impl Default for ImageType {
    fn default() -> Self {
        Self::from(ImageTypeOri {
            pixel_format: PixelFormatOri::IC4_PIXEL_FORMAT_BGR8,
            width: 0,
            height: 0,
        })
    }
}

impl ToCString for ImageType {
    fn to_cstring(&self) -> CString {
        unsafe {
            let mut message_buffer = vec![0u8; 1024 * 1024];
            let mut message_length = 0;
            let result = ic4_sys::ic4_imagetype_tostring(
                ptr_from_ref(&self.inner),
                message_buffer.as_mut_ptr() as *mut c_char,
                ptr_from_mut(&mut message_length),
            );
            if !result {
                return CString::from_vec_unchecked(
                    b"imagetype: Failed to create buffer!".to_vec(),
                );
            }
            CString::from_vec_unchecked(message_buffer)
        }
    }
}

/*
 * ImageBuffer
 */

pub type ImageBufferOri = ic4_sys::IC4_IMAGE_BUFFER;
bind_ptr_type!(
    ImageBuffer,
    ImageBufferOri,
    ic4_sys::ic4_imagebuffer_ref,
    ic4_sys::ic4_imagebuffer_unref
);

impl ImageBuffer {
    pub fn get_image_ptr(&self) -> *mut std::os::raw::c_void {
        unsafe { ic4_sys::ic4_imagebuffer_get_ptr(self.as_ptr()) }
    }

    pub fn get_pitch(&self) -> isize {
        unsafe { ic4_sys::ic4_imagebuffer_get_pitch(self.as_ptr()) }
    }

    pub fn get_buffer_size(&self) -> usize {
        unsafe { ic4_sys::ic4_imagebuffer_get_buffer_size(self.as_ptr()) }
    }

    pub fn get_image_type(&self) -> self::Result<ImageType> {
        unsafe {
            let mut image_type: ImageType = Default::default();
            ic4_sys::ic4_imagebuffer_get_image_type(
                self.as_ptr(),
                ptr_from_mut(&mut image_type.inner),
            )
            .then_some(())
            .ok_or_else(|| self::get_last_error())?;
            Ok(image_type)
        }
    }
}

/*
 * FrameMetadata
 */

pub type FrameMetadata = ic4_sys::IC4_FRAME_METADATA;

impl ImageBuffer {
    pub fn get_metadata(&self) -> self::Result<FrameMetadata> {
        unsafe {
            let mut meta_data = FrameMetadata {
                device_frame_number: 0,
                device_timestamp_ns: 0,
            };
            ic4_sys::ic4_imagebuffer_get_metadata(self.as_ptr(), ptr_from_mut(&mut meta_data))
                .then_some(())
                .ok_or_else(|| self::get_last_error())?;
            Ok(meta_data)
        }
    }
}

/*
 * ImageBufferCopyFlags
 */

pub type ImageBufferCopyFlags = ic4_sys::IC4_IMAGEBUFFER_COPY_FLAGS;

impl ImageBuffer {
    /// # TODO
    ///
    /// # Safety
    /// see ic4_sys::ic4_imagebuffer_copy
    pub unsafe fn copy_to(
        &self,
        destination: *mut ic4_sys::IC4_IMAGE_BUFFER,
        flags: std::os::raw::c_uint,
    ) -> self::Result<()> {
        unsafe {
            ic4_sys::ic4_imagebuffer_copy(self.as_ptr(), destination, flags)
                .then_some(())
                .ok_or_else(|| self::get_last_error())?;
        }
        Ok(())
    }
}

impl ImageBuffer {
    pub fn is_writable(&self) -> bool {
        unsafe { ic4_sys::ic4_imagebuffer_is_writable(self.as_ptr()) }
    }
}

pub type ImageBufferMemoryReleaseOri = ic4_sys::ic4_imagebuffer_memory_release;

/// # TODO
///
/// # Safety
/// see ic4_sys::ic4_imagebuffer_wrap_memory
pub unsafe fn imagebuffer_wrap_memory(
    pp_buffer: *mut *mut ic4_sys::IC4_IMAGE_BUFFER,
    data: *mut c_void,
    buffer_size: usize,
    pitch: isize,
    image_type: *const ic4_sys::IC4_IMAGE_TYPE,
    on_release: ImageBufferMemoryReleaseOri,
    on_release_user_ptr: *mut c_void,
) -> bool {
    ic4_sys::ic4_imagebuffer_wrap_memory(
        pp_buffer,
        data,
        buffer_size,
        pitch,
        image_type,
        on_release,
        on_release_user_ptr,
    )
}

pub type AllocatorCallbacks = ic4_sys::IC4_ALLOCATOR_CALLBACKS;

/*
 * BufferPool
 */

pub type BufferPoolConfig = ic4_sys::IC4_BUFFER_POOL_CONFIG;

pub fn default_buffer_pool_config() -> BufferPoolConfig {
    BufferPoolConfig {
        cache_frames_max: 0,
        cache_bytes_max: 0,
        allocator: AllocatorCallbacks {
            release: None,
            allocate_buffer: None,
            free_buffer: None,
        },
        allocator_context: null_mut(),
    }
}

pub type BufferPoolAllocationOptionsOri = ic4_sys::IC4_BUFFERPOOL_ALLOCATION_OPTIONS;
bind_type!(BufferPoolAllocationOptions, BufferPoolAllocationOptionsOri);

impl Default for BufferPoolAllocationOptions {
    fn default() -> Self {
        Self::from(BufferPoolAllocationOptionsOri {
            alignment: 0,
            pitch: 0,
            buffer_size: 0,
        })
    }
}

pub type BufferPoolOri = ic4_sys::IC4_BUFFER_POOL;
bind_ptr_type!(
    BufferPool,
    BufferPoolOri,
    ic4_sys::ic4_bufferpool_ref,
    ic4_sys::ic4_bufferpool_unref
);

impl BufferPool {
    pub fn create(config: &BufferPoolConfig) -> self::Result<Self> {
        let mut buffer_pool = null_mut();
        unsafe {
            ic4_sys::ic4_bufferpool_create(ptr_from_mut(&mut buffer_pool), ptr_from_ref(config))
                .then_some(())
                .ok_or_else(|| self::get_last_error())?;
        }
        Ok(Self::from(buffer_pool))
    }
}

impl BufferPool {
    pub fn get_buffer(
        &mut self,
        image_type: &ImageType,
        options: &BufferPoolAllocationOptions,
    ) -> self::Result<ImageBuffer> {
        let mut image_buffer = null_mut();
        unsafe {
            ic4_sys::ic4_bufferpool_get_buffer(
                self.as_mut_ptr(),
                ptr_from_ref(&image_type.inner),
                ptr_from_ref(&options.inner),
                ptr_from_mut(&mut image_buffer),
            )
            .then_some(())
            .ok_or_else(|| self::get_last_error())?;
        }
        Ok(ImageBuffer::from(image_buffer))
    }
}

/*
 * ImageBuffer: Save
 */

pub type ImageBufferSaveOptionsBMPOri = ic4_sys::IC4_IMAGEBUFFER_SAVE_OPTIONS_BMP;
bind_type!(ImageBufferSaveOptionsBMP, ImageBufferSaveOptionsBMPOri);

impl Default for ImageBufferSaveOptionsBMP {
    fn default() -> Self {
        Self::from(ImageBufferSaveOptionsBMPOri {
            store_bayer_raw_data_as_monochrome: 0,
        })
    }
}

pub type PNGCompressionLevelOri = ic4_sys::IC4_PNG_COMPRESSION_LEVEL;
bind_type!(PNGCompressionLevel, PNGCompressionLevelOri);

pub type ImageBufferSaveOptionsPNGOri = ic4_sys::IC4_IMAGEBUFFER_SAVE_OPTIONS_PNG;
bind_type!(ImageBufferSaveOptionsPNG, ImageBufferSaveOptionsPNGOri);

impl Default for ImageBufferSaveOptionsPNG {
    fn default() -> Self {
        Self::from(ImageBufferSaveOptionsPNGOri {
            store_bayer_raw_data_as_monochrome: 0,
            compression_level: PNGCompressionLevelOri::IC4_PNG_COMPRESSION_AUTO,
        })
    }
}

pub type ImageBufferSaveOptionsJPEGOri = ic4_sys::IC4_IMAGEBUFFER_SAVE_OPTIONS_JPEG;
bind_type!(ImageBufferSaveOptionsJPEG, ImageBufferSaveOptionsJPEGOri);

impl Default for ImageBufferSaveOptionsJPEG {
    fn default() -> Self {
        Self::from(ImageBufferSaveOptionsJPEGOri { quality_pct: 0 })
    }
}

pub type ImageBufferSaveOptionsTIFFOri = ic4_sys::IC4_IMAGEBUFFER_SAVE_OPTIONS_TIFF;
bind_type!(ImageBufferSaveOptionsTIFF, ImageBufferSaveOptionsTIFFOri);

impl Default for ImageBufferSaveOptionsTIFF {
    fn default() -> Self {
        Self::from(ImageBufferSaveOptionsTIFFOri {
            store_bayer_raw_data_as_monochrome: 0,
        })
    }
}

impl ImageBuffer {
    pub fn save_as_bmp(
        &mut self,
        file_path: &CStr,
        options: &ImageBufferSaveOptionsBMP,
    ) -> self::Result<()> {
        unsafe {
            ic4_sys::ic4_imagebuffer_save_as_bmp(
                self.as_mut_ptr(),
                file_path.as_ptr(),
                ptr_from_ref(&options.inner),
            )
            .then_some(())
            .ok_or_else(|| self::get_last_error())?;
        }
        Ok(())
    }
    pub fn save_as_jpeg(
        &mut self,
        file_path: &CStr,
        options: &ImageBufferSaveOptionsJPEG,
    ) -> self::Result<()> {
        unsafe {
            ic4_sys::ic4_imagebuffer_save_as_jpeg(
                self.as_mut_ptr(),
                file_path.as_ptr(),
                ptr_from_ref(&options.inner),
            )
            .then_some(())
            .ok_or_else(|| self::get_last_error())?;
        }
        Ok(())
    }
    pub fn save_as_png(
        &mut self,
        file_path: &CStr,
        options: &ImageBufferSaveOptionsPNG,
    ) -> self::Result<()> {
        unsafe {
            ic4_sys::ic4_imagebuffer_save_as_png(
                self.as_mut_ptr(),
                file_path.as_ptr(),
                ptr_from_ref(&options.inner),
            )
            .then_some(())
            .ok_or_else(|| self::get_last_error())?;
        }
        Ok(())
    }
    pub fn save_as_tiff(
        &mut self,
        file_path: &CStr,
        options: &ImageBufferSaveOptionsTIFF,
    ) -> self::Result<()> {
        unsafe {
            ic4_sys::ic4_imagebuffer_save_as_tiff(
                self.as_mut_ptr(),
                file_path.as_ptr(),
                ptr_from_ref(&options.inner),
            )
            .then_some(())
            .ok_or_else(|| self::get_last_error())?;
        }
        Ok(())
    }
}
