#![allow(clippy::redundant_closure)]

use crate::*;

/*
 * PixelFormat
 */

pub type PixelFormat = ic4_sys::IC4_PIXEL_FORMAT;

impl DefaultExt for PixelFormat {
    fn default_ext() -> Self {
        PixelFormat::IC4_PIXEL_FORMAT_Unspecified
    }
}

impl ToCStr for PixelFormat {
    fn to_cstr(&self) -> &CStr {
        unsafe { CStr::from_ptr(ic4_sys::ic4_pixelformat_tostring(*self)) }
    }
}

pub trait PixelFormatExt {
    fn bpp(&self) -> usize;
}

impl PixelFormatExt for PixelFormat {
    fn bpp(&self) -> usize {
        unsafe { ic4_sys::ic4_pixelformat_bpp(*self) }
    }
}

/*
 * ImageType
 */

pub type ImageType = ic4_sys::IC4_IMAGE_TYPE;

impl DefaultExt for ImageType {
    fn default_ext() -> Self {
        ImageType {
            pixel_format: DefaultExt::default_ext(),
            width: 0,
            height: 0,
        }
    }
}

impl ToCString for ImageType {
    fn to_cstring(&self) -> CString {
        unsafe {
            let mut message_length = 10 * 1024;
            let mut message_vec = vec![0u8; 10 * 1024];
            let result = ic4_sys::ic4_imagetype_tostring(
                ptr_from_ref(self),
                message_vec.as_mut_ptr() as *mut c_char,
                ptr_from_mut(&mut message_length),
            );
            if !result {
                return CString::from_vec_unchecked(
                    b"imagetype: Failed to create buffer!".to_vec(),
                );
            }
            message_vec.resize(message_length - 1, 0);
            CString::from_vec_unchecked(message_vec)
        }
    }
}

/*
 * ImageBuffer
 */

bind_ptr_type!(
    ImageBuffer,
    ic4_sys::IC4_IMAGE_BUFFER,
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
            ic4_sys::ic4_imagebuffer_get_image_type(self.as_ptr(), ptr_from_mut(&mut image_type))
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
            let mut meta_data = Default::default();
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

pub type ImageBufferMemoryRelease = ic4_sys::ic4_imagebuffer_memory_release;

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
    on_release: ImageBufferMemoryRelease,
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

impl DefaultExt for AllocatorCallbacks {
    fn default_ext() -> Self {
        AllocatorCallbacks {
            release: None,
            allocate_buffer: None,
            free_buffer: None,
        }
    }
}

/*
 * BufferPool
 */

pub type BufferPoolConfig = ic4_sys::IC4_BUFFER_POOL_CONFIG;

impl DefaultExt for BufferPoolConfig {
    fn default_ext() -> Self {
        BufferPoolConfig {
            cache_frames_max: 128,
            cache_bytes_max: 0,
            allocator: AllocatorCallbacks {
                release: None,
                allocate_buffer: None,
                free_buffer: None,
            },
            allocator_context: null_mut(),
        }
    }
}

pub type BufferPoolAllocationOptions = ic4_sys::IC4_BUFFERPOOL_ALLOCATION_OPTIONS;

impl DefaultExt for BufferPoolAllocationOptions {
    fn default_ext() -> Self {
        Self {
            alignment: 0,
            pitch: 0,
            buffer_size: 0,
        }
    }
}

bind_ptr_type!(
    BufferPool,
    ic4_sys::IC4_BUFFER_POOL,
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
                ptr_from_ref(image_type),
                ptr_from_ref(options),
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

pub type ImageBufferSaveOptionsBMP = ic4_sys::IC4_IMAGEBUFFER_SAVE_OPTIONS_BMP;

impl DefaultExt for ImageBufferSaveOptionsBMP {
    fn default_ext() -> Self {
        Self {
            store_bayer_raw_data_as_monochrome: 0,
        }
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
                ptr_from_ref(options),
            )
            .then_some(())
            .ok_or_else(|| self::get_last_error())?;
        }
        Ok(())
    }
}

pub type PNGCompressionLevel = ic4_sys::IC4_PNG_COMPRESSION_LEVEL;
pub type ImageBufferSaveOptionsPNG = ic4_sys::IC4_IMAGEBUFFER_SAVE_OPTIONS_PNG;

impl DefaultExt for PNGCompressionLevel {
    fn default_ext() -> Self {
        Self::IC4_PNG_COMPRESSION_AUTO
    }
}

impl DefaultExt for ImageBufferSaveOptionsPNG {
    fn default_ext() -> Self {
        Self {
            compression_level: DefaultExt::default_ext(),
            store_bayer_raw_data_as_monochrome: 0,
        }
    }
}

impl ImageBuffer {
    pub fn save_as_jpeg(
        &mut self,
        file_path: &CStr,
        options: &ImageBufferSaveOptionsJPEG,
    ) -> self::Result<()> {
        unsafe {
            ic4_sys::ic4_imagebuffer_save_as_jpeg(
                self.as_mut_ptr(),
                file_path.as_ptr(),
                ptr_from_ref(options),
            )
            .then_some(())
            .ok_or_else(|| self::get_last_error())?;
        }
        Ok(())
    }
}

pub type ImageBufferSaveOptionsJPEG = ic4_sys::IC4_IMAGEBUFFER_SAVE_OPTIONS_JPEG;

impl DefaultExt for ImageBufferSaveOptionsJPEG {
    fn default_ext() -> Self {
        Self { quality_pct: 90 }
    }
}

impl ImageBuffer {
    pub fn save_as_png(
        &mut self,
        file_path: &CStr,
        options: &ImageBufferSaveOptionsPNG,
    ) -> self::Result<()> {
        unsafe {
            ic4_sys::ic4_imagebuffer_save_as_png(
                self.as_mut_ptr(),
                file_path.as_ptr(),
                ptr_from_ref(options),
            )
            .then_some(())
            .ok_or_else(|| self::get_last_error())?;
        }
        Ok(())
    }
}

pub type ImageBufferSaveOptionsTIFF = ic4_sys::IC4_IMAGEBUFFER_SAVE_OPTIONS_TIFF;

impl DefaultExt for ImageBufferSaveOptionsTIFF {
    fn default_ext() -> Self {
        Self {
            store_bayer_raw_data_as_monochrome: 0,
        }
    }
}

impl ImageBuffer {
    pub fn save_as_tiff(
        &mut self,
        file_path: &CStr,
        options: &ImageBufferSaveOptionsTIFF,
    ) -> self::Result<()> {
        unsafe {
            ic4_sys::ic4_imagebuffer_save_as_tiff(
                self.as_mut_ptr(),
                file_path.as_ptr(),
                ptr_from_ref(options),
            )
            .then_some(())
            .ok_or_else(|| self::get_last_error())?;
        }
        Ok(())
    }
}
