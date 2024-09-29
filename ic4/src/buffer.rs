#![allow(clippy::redundant_closure)]

use crate::*;

/*
 * PixelFormat
 */

#[doc = " The pixel format defines the representation of pixels in an image."]
pub type PixelFormat = ic4_sys::IC4_PIXEL_FORMAT;

impl DefaultExt for PixelFormat {
    fn default_ext() -> Self {
        PixelFormat::IC4_PIXEL_FORMAT_Unspecified
    }
}

impl ToCStr for PixelFormat {
    #[doc = " @brief Returns the name of a pixel format.\n\n @param[in] pixel_format\tA pixel format\n\n @return\tA pointer to a null-terminated string containing the name of the pixel format.\\n\n\t\t\tThe memory pointed to by the return value is valid forever does not need to be released.\\n\n\t\t\tIf the passed pixel format is unknown, the function returns \\c NULL."]
    fn to_cstr(&self) -> &CStr {
        unsafe { CStr::from_ptr(ic4_sys::ic4_pixelformat_tostring(*self)) }
    }
}

pub trait PixelFormatExt {
    #[doc = " @brief Returns the bits per pixel of a pixel format.\n\n @param[in] pixel_format  pixel format\n\n @return\tThe bits required to store one pixel using the given pixel format.\\n\n\t\t\tThe function returns \\c 0 if the bits per pixel of the pixel format is unknown."]
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

#[doc = " @struct IC4_IMAGE_TYPE\n\n Represents an image type, including pixel format and image dimensions.\n\n Using a partially-specified image type is allowed when defining the buffer format of a sink.\n The sink will fill the other fields with data from the device automatically."]
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
    #[doc = " @brief Convert a given image type into a string representation.\n\n @param[out] image_type\t\tPointer to a #IC4_IMAGE_TYPE value to receive the error code.\n @param[out] buffer\t\t\tPointer to a character array to receive the string.\\n\n\t\t\t\t\t\t\t\tThis parameter can be \\c NULL to find out the required space without allocating an initial array.\n @param[in, out] buffer_size\tPointer to a \\c size_t describing the length of the array pointed to by \\a buffer.\\n\n\t\t\t\t\t\t\t\tThe function always writes the actual number of characters required to store the string representation.\n\n @return\t\\c true on success.\n @return\tIf \\a image_type is \\c NULL, the function fails and returns \\c false.\n @return\tIf \\a buffer is not \\c NULL, and \\c *buffer_size is smaller than the required number\n\t\t\tof bytes to store the string, the function fails and returns \\c false.\n @return\tUse #ic4_get_last_error() to query error information."]
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

#[doc = " @struct IC4_IMAGE_BUFFER\n @brief Represents an image buffer\n\n This type is opaque, programs only use pointers of type \\c IC4_IMAGE_BUFFER*.\n\n Image buffer objects are created automatically by the various @ref sink types,\n on request by a @ref IC4_BUFFER_POOL, or by calling @ref ic4_imagebuffer_wrap_memory().\n\n Image buffer objects are reference-counted.\n To share an image buffer object between multiple parts of a program, use ic4_imagebuffer_ref().\n Call ic4_imagebuffer_unref() when a reference is no longer required.\n\n If the reference count reaches zero, the image buffer is returned to its source to be reused. For example,\n an image buffer retrieved from ic4_queuesink_pop_output_buffer() will be re-queued."]
pub type ImageBufferOri = ic4_sys::IC4_IMAGE_BUFFER;
bind_ptr_type!(
    ImageBuffer,
    ic4_sys::IC4_IMAGE_BUFFER,
    ic4_sys::ic4_imagebuffer_ref,
    ic4_sys::ic4_imagebuffer_unref
);

impl ImageBuffer {
    #[doc = " @brief Returns a pointer to the data managed by the image buffer.\n\n @param[in] pImageBuffer An image buffer\n\n @return A pointer to the image data contained in the image buffer,\\n\n\t\t   or \\c NULL if an error occurred. Use ic4_get_last_error() to query error information.\n\n @remarks The memory pointed to by the returned pointer is valid as long as the image buffer object exists."]
    pub fn get_image_ptr(&self) -> *mut std::os::raw::c_void {
        unsafe { ic4_sys::ic4_imagebuffer_get_ptr(self.as_ptr()) }
    }

    #[doc = " @brief Returns the pitch for the image buffer.\n\n The pitch is the distance between the starting memory location of two consecutive lines.\n\n @param[in] pImageBuffer\tAn image buffer\n\n @return\tThe pitch for this image buffer, or \\c 0 if an error occurred.\\n\n\t\t\tUse @ref ic4_get_last_error() to query error information."]
    pub fn get_pitch(&self) -> isize {
        unsafe { ic4_sys::ic4_imagebuffer_get_pitch(self.as_ptr()) }
    }

    #[doc = " @brief Returns the size of the image buffer.\n\n @param[in] pImageBuffer\tAn image buffer\n\n @return\tThe size of this image buffer, or \\c 0 if an error occurred.\\n\n\t\t\tUse @ref ic4_get_last_error() to query error information."]
    pub fn get_buffer_size(&self) -> usize {
        unsafe { ic4_sys::ic4_imagebuffer_get_buffer_size(self.as_ptr()) }
    }

    #[doc = " @brief Queries information about the image buffers's image data type.\n\n @param[in] pImageBuffer\t\tAn image buffer\n @param[out] image_type\tA #IC4_IMAGE_TYPE structure receiving the image data type information.\n\n @return \\c true on success, otherwise \\c false.\\n\n\t\t\tUse ic4_get_last_error() to query error information."]
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

#[doc = " A structure containing frame metadata"]
pub type FrameMetadata = ic4_sys::IC4_FRAME_METADATA;

impl ImageBuffer {
    #[doc = " @brief Retrieves metadata from an image buffer object.\n\n @param[in] pImageBuffer An image buffer\n @param[out] metadata A #IC4_FRAME_METADATA structure receiving the metadata.\n\n @return \\c true on success, otherwise \\c false.\\n\n\t\t\tUse ic4_get_last_error() to query error information."]
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

#[doc = " @brief Contains options to configure the behavior of #ic4_imagebuffer_copy()."]
pub type ImageBufferCopyFlags = ic4_sys::IC4_IMAGEBUFFER_COPY_FLAGS;

impl ImageBuffer {
    /// # TODO
    ///
    /// # Safety
    /// see ic4_sys::ic4_imagebuffer_copy
    ///
    #[doc = " @brief Copies the contents of one image buffer to another image buffer.\n\n @param source\t\tSource buffer to copy from\n @param destination\tDestination buffer to copy into\n @param flags\t\t\tA bitwise combination of values from #IC4_IMAGEBUFFER_COPY_FLAGS.\n\n @remarks\n If the pixel format of the images in @c source and @c destination is not equal, the image is converted. For example,\n if the pixel format of @c source is #IC4_PIXEL_FORMAT_BayerRG8 and the pixel format of @c destination is #IC4_PIXEL_FORMAT_BGR8,\n a demosaicing operation creates a color image.\\n\n If @c flags contains #IC4_IMAGEBUFFER_COPY_SKIP_IMAGE, the function does not copy the image data. The function then only copies the meta data, and a\n program-defined algorithm can handle the image copy operation.\\n\n If @c flags contains #IC4_IMAGEBUFFER_COPY_SKIP_CHUNKDATA, the function does not copy the chunk data contained in @c source. This can be useful if the\n chunk data is large and not required.\n\n @note\n If the width or height of @c source and @c destination are not equal, the function fails and the error value is set to #IC4_ERROR_CONVERSION_NOT_SUPPORTED.\\n\n If there is no algorithm available for the requested conversion, the function fails and the error value is set to #IC4_ERROR_CONVERSION_NOT_SUPPORTED.\\n\n If the @c destination frame is not writable, the function fails and the error value is set to #IC4_ERROR_INVALID_OPERATION.\\n\n\n @return \\c true on success, otherwise \\c false.\\n\n\t\t\tUse ic4_get_last_error() to query error information."]
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
    #[doc = " @brief Checks whether an image buffer object is (safely) writable.\n\n In some situations, image buffer objects are shared between the application holding a handle to the image buffer object\n and the library. For example, the image buffer might be shared with a display or a video writer.\n\n A shared buffer is not safely writable. Writing to a buffer that is shared can lead to unexpected behavior, for example a\n modification may partially appear in the result of an operation that is happening in parallel.\n\n Passing the image buffer into a function such as #ic4_display_display_buffer() or #ic4_videowriter_add_frame() can lead to a buffer becoming shared.\n\n @note\tThis function only checks for whether the buffer is shared with other parts of the library. It is the program's responsibility \\n\n\t\t\tto track whether multiple parts of the program are accessing the buffer's memory at the same time.\n\n @param[in] buffer\tAn image buffer object\n\n @note\tThis function does not track sharing between multiple parts of the application. If the application owns multiple references to\n\t\t\tan image buffer, or shares its buffer memory address between multiple threads, it is the application's responsibility to\n\t\t\tavoid data races.\n\n @return\t@c true if the image buffer is not shared with any part of the library, and is therefore safely writable.\\n\n\t\t\t@c false, if the image buffer is shared and therefore should not be written into."]
    pub fn is_writable(&self) -> bool {
        unsafe { ic4_sys::ic4_imagebuffer_is_writable(self.as_ptr()) }
    }
}

#[doc = " @brief Defines a callback function to be called when an image buffer created by @ref ic4_imagebuffer_wrap_memory\n is destroyed and the image data will no longer be accessed through it.\n\n @param[in] ptr\t\t\tPointer to the image data as passed as @a data into @ref ic4_imagebuffer_wrap_memory\n @param[in] buffer_size\tBuffer size as passed as @a buffer_size into @ref ic4_imagebuffer_wrap_memory\n @param[in] user_ptr\t\tThe @a on_release_user_ptr parameter that was specified when the image buffer was created."]
pub type ImageBufferMemoryRelease = ic4_sys::ic4_imagebuffer_memory_release;

/// # TODO
///
/// # Safety
/// see ic4_sys::ic4_imagebuffer_wrap_memory
///
#[doc = " @brief Creates an image buffer object using external memory as storage area for the image data.\n\n This function can be useful when copying image data into buffers of third-party libraries:\n - Create an image object in the third-party library\n - Wrap the third-party library's image data into an @ref IC4_IMAGE_BUFFER using @ref ic4_imagebuffer_wrap_memory().\n - Copy the data from an existing image buffer object into the third-party image buffer using @ref ic4_imagebuffer_copy().\n\n @param[out] ppBuffer\t\t\t\tPointer to an image buffer handle to receive the new buffer object.\\n\n\t\t\t\t\t\t\t\t\tWhen the buffer is no longer required, release the object reference using @ref ic4_imagebuffer_unref().\n @param[in] data\t\t\t\t\tPointer to a region of memory to be used as image data by the image buffer object\n @param[in] buffer_size\t\t\tSize of the region of memory pointed to by @a data\n @param[in] pitch\t\t\t\t\tDifference between memory addresses of two consecutive lines of image data\n @param[in] image_type\t\t\tType of image to be stored in the image buffer\n @param[in] on_release\t\t\tOptional pointer to a callback function to be called when the image buffer object is destroyed.\\n\n\t\t\t\t\t\t\t\t\tThis can be useful when the program needs to keep track of the memory pointer to by the image buffer\n\t\t\t\t\t\t\t\t\tand has to release it once the image buffer object no longer exists.\\n\n\t\t\t\t\t\t\t\t\tThe lifetime of the image buffer can potentially be extended beyond the existance of its handle\n\t\t\t\t\t\t\t\t\twhen it is shared with API functions, e.g. @ref ic4_display_display_buffer or @ref ic4_videowriter_add_frame.\n @param[in] on_release_user_ptr\tUser pointer to be passed when calling @a on_release.\n\n @return \\c true on success, otherwise \\c false.\\n\n\t\t\tUse ic4_get_last_error() to query error information."]
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

#[doc = " Contains function pointers used to configure a custom allocator for image buffers"]
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

#[doc = " @brief Configures the behavior of a #IC4_BUFFER_POOL."]
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

#[doc = " @brief Contains options to configure the allocation when requesting an image buffer from a buffer pool.\n\n @see ic4_bufferpool_get_buffer"]
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

#[doc = " @struct IC4_BUFFER_POOL\n @brief The buffer pool allows allocating additional image buffers for use by the program.\n\n This type is opaque, programs only use pointers of type \\c IC4_BUFFER_POOL*.\n\n Most programs will only use image buffers provided by one of the sink types.\n However, some programs require additional buffers, for example to use as destination for image processing.\n\n To create additional buffers, first create a buffer pool using #ic4_bufferpool_create().\n Then, use #ic4_bufferpool_get_buffer() to create a new image buffer with a specified image type.\n Allocation options can be specified to customizer image buffer's memory alignment, pitch and total buffer size.\n\n When the image buffer is no longer required, call #ic4_imagebuffer_unref on it. The buffer will then be returned to the buffer pool.\n\n The buffer pool has configurable caching behavior. By default, the buffer pool will cache one image buffer and return it the next\n time a matching image buffer is requested.\n\n Buffer pool objects are reference-counted, and created with an initial reference count of one.\n To share a buffer pool object between multiple parts of a program, create a new reference by calling #ic4_bufferpool_ref().\n When a reference is no longer required, call #ic4_bufferpool_unref().\n\n If the buffer pool object's internal reference count reaches zero, the buffer pool object is destroyed.\n Even after that, image buffers created by the buffer pool are still valid until they are released by calling #ic4_imagebuffer_unref."]
pub type BufferPoolOri = ic4_sys::IC4_BUFFER_POOL;
bind_ptr_type!(
    BufferPool,
    ic4_sys::IC4_BUFFER_POOL,
    ic4_sys::ic4_bufferpool_ref,
    ic4_sys::ic4_bufferpool_unref
);

impl BufferPool {
    #[doc = " @brief Creates a new buffer pool.\n\n @param[in] ppPool\tPointer to a buffer pool handle to receive the new buffer pool.\n\t\t\t\t\t\tWhen the buffer pool is no longer required, release the object reference using #ic4_bufferpool_unref().\n @param[in] config\tPointer to a structure containing the buffer pool configuration\n\n @return \\c true on success, otherwise \\c false.\\n\n\t\t\tUse ic4_get_last_error() to query error information."]
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
    #[doc = " @brief Gets a buffer from the buffer pool.\n\n The buffer is either newly allocated, or retrieved from the buffer pool's buffer cache.\n\n @param[in] pPool\t\t\t\tA buffer pool\n @param[in] image_type\t\tImage type of the requested buffer\n @param[in] options\t\t\tA pointer to a #IC4_BUFFERPOOL_ALLOCATION_OPTIONS structure specifying advance allocation options.\\n\n\t\t\t\t\t\t\t\tMay be @c NULL to use default allocation parameters.\n @param[out] ppImageBuffer\tPointer to an image buffer handle to receive the buffer.\\n\n\t\t\t\t\t\t\t\tWhen the buffer is no longer required, release the object reference using #ic4_imagebuffer_unref().\n\n @return \\c true on success, otherwise \\c false.\\n\n\t\t\tUse ic4_get_last_error() to query error information."]
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

#[doc = " @brief Contains image file storage options for bitmap files."]
pub type ImageBufferSaveOptionsBMP = ic4_sys::IC4_IMAGEBUFFER_SAVE_OPTIONS_BMP;

impl DefaultExt for ImageBufferSaveOptionsBMP {
    fn default_ext() -> Self {
        Self {
            store_bayer_raw_data_as_monochrome: 0,
        }
    }
}

impl ImageBuffer {
    #[doc = " @brief Saves an image buffer as a Bitmap file.\n\n @param[in] pImageBuffer\tAn image buffer\n @param[in] file_path\tPath of the image file\n @param[in] options\tOptions structure configuring the save operation\n\n @note\n Depending on the pixel format of the image buffer, a transformation is applied before saving the image.\n\t- Monochrome pixel formats are converted to Mono8 and stored as a 8-bit monochrome bitmap file\n\t- Bayer, RGB and YUV pixel formats are converted to BGR8 and stored as a 24-bit color bitmap file\n\n @return \\c true on success, otherwise \\c false.\\n\n\t\t\tUse ic4_get_last_error() to query error information."]
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

#[doc = " Defines the possible PNG file compression levels.\n\n Higher compression levels can generate smaller files, but the compression can take more time."]
pub type PNGCompressionLevel = ic4_sys::IC4_PNG_COMPRESSION_LEVEL;

#[doc = " @brief Contains image file storage options for PNG files."]
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
    #[doc = " @brief Saves an image buffer as a Jpeg file.\n\n @param[in] pImageBuffer\tAn image buffer\n @param[in] file_path\tPath of the image file\n @param[in] options\tOptions structure configuring the save operation\n\n @note\n Depending on the pixel format of the image buffer, a transformation is applied before saving the image.\n\t- Monochrome pixel formats are converted to Mono8 and stored as a monochrome jpeg file\n\t- Bayer, RGB and YUV pixel formats are converted to BGR8 stored as a color jpeg file\n\n @return \\c true on success, otherwise \\c false.\\n\n\t\t\tUse ic4_get_last_error() to query error information."]
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

#[doc = " @brief Contains image file storage options for Jpeg files."]
pub type ImageBufferSaveOptionsJPEG = ic4_sys::IC4_IMAGEBUFFER_SAVE_OPTIONS_JPEG;

impl DefaultExt for ImageBufferSaveOptionsJPEG {
    fn default_ext() -> Self {
        Self { quality_pct: 90 }
    }
}

impl ImageBuffer {
    #[doc = " @brief Saves an image buffer as a PNG file.\n\n @param[in] pImageBuffer\tAn image buffer\n @param[in] file_path\tPath of the image file\n @param[in] options\tOptions structure configuring the save operation\n\n @note\n Depending on the pixel format of the image buffer, a transformation is applied before saving the image.\n\t- Monochrome pixel formats with a bit depth higher than 8bpp are converted to Mono16 and stored as a monochrome PNG file with 16 bits per channel\n\t- Mono8 image buffers are stored as a monochrome PNG file with 8 bits per channel\n\t- Bayer format with a bit depth higher than 8bpp are converted to BGRa16 and stored as a 4-channel PNG with 16 bits per channel\n\t- 8-bit Bayer, RGB and YUV pixel formats are converted to BGR8 stored as a 3-channel PNG file with 8 bits per channel\n\n @return \\c true on success, otherwise \\c false.\\n\n\t\t\tUse ic4_get_last_error() to query error information."]
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

#[doc = " @brief Contains image file storage options for TIFF files."]
pub type ImageBufferSaveOptionsTIFF = ic4_sys::IC4_IMAGEBUFFER_SAVE_OPTIONS_TIFF;

impl DefaultExt for ImageBufferSaveOptionsTIFF {
    fn default_ext() -> Self {
        Self {
            store_bayer_raw_data_as_monochrome: 0,
        }
    }
}

impl ImageBuffer {
    #[doc = " @brief Saves an image buffer as a Tiff file.\n\n @param[in] pImageBuffer\tAn image buffer\n @param[in] file_path\tPath of the image file\n @param[in] options\tOptions structure configuring the save operation\n\n @note\n Depending on the pixel format of the image buffer, a transformation is applied before saving the image.\n\t- Monochrome pixel formats with a bit depth higher than 8bpp are converted to Mono16 and stored as a monochrome Tiff file with 16 bits per channel\n\t- Mono8 image buffers are stored as a monochrome Tiff file with 8 bits per channel\n\t- Bayer format with a bit depth higher than 8bpp are converted to BGRa16 and stored as a 4-channel Tiff with 16 bits per channel\n\t- 8-bit Bayer, RGB and YUV pixel formats are converted to BGR8 stored as a 3-channel Tiff file with 8 bits per channel\n\n @return \\c true on success, otherwise \\c false.\\n\n\t\t\tUse ic4_get_last_error() to query error information."]
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
