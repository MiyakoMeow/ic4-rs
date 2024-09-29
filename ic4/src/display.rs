#![allow(clippy::redundant_closure)]

use crate::*;

/*
 * Display
 */

#[doc = " @struct IC4_DISPLAY\n @brief Represents a display that can be used to display images.\n\n This type is opaque, programs only use pointers of type \\c IC4_DISPLAY*.\n\n To create a display, use @ref ic4_display_create() or @ref ic4_display_create_external_opengl().\n\n Display objects are generally used in two distinct ways:\n -\tThe display is connected to a data stream when calling @ref ic4_grabber_stream_setup(),\n\t\tautomatically displaying all images from the opened device.\n -\t@ref IC4_IMAGE_BUFFER objects are displayed manually by calling @ref ic4_display_display_buffer().\n\n Display objects are reference-counted. The initial reference count is one.\n\n To share a display object between multiple parts of a program, use ic4_display_ref().\n Call ic4_display_unref() when a reference is no longer required.\n If the reference count reaches zero, the display object is destroyed.\n\n @note\n Some functions, such as @ref ic4_grabber_stream_setup(), share ownership of the display object passed as an argument.\n The display object is kept alive by the #IC4_GRABBER instance even if no external references exist.\n\n @see ic4_display_create\n @see ic4_display_ref\n @see ic4_display_unref"]
pub type DisplayOri = ic4_sys::IC4_DISPLAY;
bind_ptr_type!(
    Display,
    ic4_sys::IC4_DISPLAY,
    ic4_sys::ic4_display_ref,
    ic4_sys::ic4_display_unref
);

#[doc = " @brief Defines the possible display types"]
pub type DisplayType = ic4_sys::IC4_DISPLAY_TYPE;

impl DefaultExt for DisplayType {
    fn default_ext() -> Self {
        Self::IC4_DISPLAY_DEFAULT
    }
}

#[doc = " @brief A structure containing display statistics\n\n This structure contains information about the number of frames that were\n displayed or dropped by a display."]
pub type DisplayStats = ic4_sys::IC4_DISPLAY_STATS;

impl DefaultExt for DisplayStats {
    fn default_ext() -> Self {
        Self {
            num_frames_displayed: 0,
            num_frames_dropped: 0,
        }
    }
}

/// Should be wrapped, because ic4_sys::IC4_WINDOW_HANDLE is a raw c_void pointer.
pub type WindowHandleOri = ic4_sys::IC4_WINDOW_HANDLE;
bind_type!(WindowHandle, WindowHandleOri);

impl WindowHandle {
    pub fn create_null() -> Self {
        WindowHandle::from(ic4_sys::IC4_WINDOW_HANDLE(null_mut()))
    }
    pub fn create_from_ptr(handle_ptr: *mut c_void) -> Self {
        WindowHandle::from(ic4_sys::IC4_WINDOW_HANDLE(handle_ptr))
    }
}

impl Display {
    #[doc = " @brief Creates a new display.\n\n @param[in] type The type of display to create\n @param[in] parent Handle to the parent window to embed the display into.\n @param[out] ppDisplay Pointer to receive the handle to the new display object.\\n\n             When the display is no longer required, release the object reference using ic4_display_unref().\n\n @return \\c true on success, otherwise \\c false.\\n\n\t\t\tUse ic4_get_last_error() to query error information.\n\n @warning\n This function only works in Windows platforms. For other platforms, use @ref ic4_display_create_external_opengl().\n\n @see ic4_display_unref"]
    pub fn create(display_type: DisplayType, parent: WindowHandle) -> self::Result<Self> {
        let mut ptr = null_mut();
        unsafe {
            ic4_sys::ic4_display_create(display_type, parent.into(), ptr_from_mut(&mut ptr))
                .then_some(())
                .ok_or(self::get_last_error())?;
        }
        Ok(ptr.into())
    }
    #[doc = " @brief Creates a new external OpenGL display\n\n @param[out]\t\t\tppDisplay Pointer to receive the handle to the new display object.\\n\n\t\t\t\t\t\tWhen the display is no longer required, release the object reference using ic4_display_unref().\n\n @return \\c true on success, otherwise \\c false.\\n\n\t\t\tUse ic4_get_last_error() to query error information.\n\n @remarks\n To use the external renderer, the application has to follow these steps:\n - Create an OpenGL window, typically using the UI toolkit of the application\n - Call @ref ic4_display_external_opengl_initialize with the OpenGL context activated for the active thread\n - Repeatedly call @ref ic4_display_external_opengl_render with the OpenGL context activated for the active thread\n\n @see ic4_display_unref"]
    pub fn create_external_opengl() -> self::Result<Self> {
        let mut ptr = null_mut();
        unsafe {
            ic4_sys::ic4_display_create_external_opengl(ptr_from_mut(&mut ptr))
                .then_some(())
                .ok_or(self::get_last_error())?;
        }
        Ok(ptr.into())
    }
}

impl Display {
    #[doc = " @brief Displays a specific image buffer.\n\n @param[in] pDisplay A display\n @param[in] buffer The buffer to display\n\n @remarks It is not always necessary to call this function.\\n\n\t\t\tWhen a display is registered with a #IC4_GRABBER using ic4_grabber_set_display(), images are displayed automatically.\n\n @return \\c true on success, otherwise \\c false.\\n\n\t\t\tUse ic4_get_last_error() to query error information.\n\n @remarks\n When @a buffer is @c NULL, the display is cleared and will no longer display the previous buffer."]
    pub fn display_buffer(&mut self, image_buffer: &ImageBuffer) -> self::Result<()> {
        unsafe {
            ic4_sys::ic4_display_display_buffer(self.as_mut_ptr(), image_buffer.as_ptr())
                .then_some(())
                .ok_or(self::get_last_error())?;
        }
        Ok(())
    }
    #[doc = " @brief Queries statistics about a display\n\n @param[in] pDisplay\tA display\n @param[out] stats\tPointer to a #IC4_DISPLAY_STATS structure receiving statistics about the display\n\n @return \\c true on success, otherwise \\c false.\\n\n\t\t\tUse ic4_get_last_error() to query error information."]
    pub fn get_stats(&mut self) -> self::Result<DisplayStats> {
        let mut result: DisplayStats = DisplayStats {
            num_frames_displayed: 0,
            num_frames_dropped: 0,
        };
        unsafe {
            ic4_sys::ic4_display_get_stats(self.as_mut_ptr(), ptr_from_mut(&mut result))
                .then_some(())
                .ok_or(self::get_last_error())?;
        }
        Ok(result)
    }
}

#[doc = " @brief Contains the possible display alignment and stretch modes"]
pub type DisplayRenderPosition = ic4_sys::IC4_DISPLAY_RENDER_POSITION;

impl Display {
    #[doc = " @brief Configure the image scaling and alignment options for a display.\n\n @param[in] pDisplay\tA display\n @param[in] pos\t\tThe scaling and alignment mode to use\n @param[in] left\t\tThe left coordinate of the target rectangle inside the display window\n @param[in] top\t\tThe top coordinate of the target rectangle inside the display window\n @param[in] width\t\tThe width of the target rectangle inside the display window\n @param[in] height\tThe height of the target rectangle inside the display window\n\n @remarks\n The \\a left, \\a top, \\a width and \\a height parameters are ignored unless \\a pos is \\c IC4_DISPLAY_RENDER_POSITION_CUSTOM.\n\n @return \\c true on success, otherwise \\c false.\\n\n\t\t\tUse ic4_get_last_error() to query error information."]
    pub fn set_render_position(
        &mut self,
        pos: DisplayRenderPosition,
        left: c_int,
        top: c_int,
        width: c_int,
        height: c_int,
    ) -> self::Result<()> {
        unsafe {
            ic4_sys::ic4_display_set_render_position(
                self.as_mut_ptr(),
                pos,
                left,
                top,
                width,
                height,
            )
            .then_some(())
            .ok_or(self::get_last_error())?;
        }

        Ok(())
    }
}

#[doc = " Function pointer for the window-closed handler\n\n @param[in] pDisplay\tPointer to the display whose window was closed\n @param[in] user_ptr\tUser data that was specified when calling #ic4_display_event_add_window_closed()"]
pub type DisplayWindowClosedHandler = ic4_sys::ic4_display_window_closed_handler;

#[doc = " Function pointer for cleanup of the device-lost user data\n\n @param[in] user_ptr\tUser data that was specified when calling #ic4_grabber_event_add_device_lost()"]
pub type DisplayWindowClosedDeleter = ic4_sys::ic4_display_window_closed_deleter;

impl Display {
    /// # Safety
    /// Unknown.
    ///
    #[doc = " @brief Registers a callback to be called when the display is closed.\n\n @param[in] pDisplay\tA display\n @param[in] handler\tThe function to be called when the display is closed\n @param[in] user_ptr\tUser data to be passed in calls to \\a handler.\n @param[in] deleter\tA function to be called when the handler was unregistered and the user_ptr will no longer be required.\\n\n\t\t\t\t\t\tThe deleter function can be used to release data associated with \\a user_ptr.\\n\n\t\t\t\t\t\tThe \\a deleter function will be called when the display-closed handler is unregistered,\n\t\t\t\t\t\tor the display object itself is destroyed.\n\n @note\n To unregister a display-closed handler, call #ic4_display_event_remove_window_closed().\n\n @return \\c true on success, otherwise \\c false.\\n\n\t\t\tUse ic4_get_last_error() to query error information."]
    pub unsafe fn event_add_window_closed(
        &mut self,
        handler: DisplayWindowClosedHandler,
        user_ptr: *mut c_void,
        deleter: DisplayWindowClosedDeleter,
    ) -> self::Result<()> {
        ic4_sys::ic4_display_event_add_window_closed(self.as_mut_ptr(), handler, user_ptr, deleter)
            .then_some(())
            .ok_or_else(|| self::get_last_error())
    }
    /// # Safety
    /// Unknown.
    ///
    #[doc = " Unregisters a display-closed handler that was previously registered using #ic4_display_event_add_window_closed().\n\n @param[in] pDisplay\tThe display on which the callback is currently registered\n @param[in] handler\tPointer to the function to be unregistered\n @param[in] user_ptr\tUser data that the function was previously registered with\n\n @note\n The pair of \\a handler and \\a user_ptr has to be an exact match to the parameters used in the call to #ic4_display_event_add_window_closed().\n\n @return \\c true on success, otherwise \\c false.\\n\n\t\t\tUse ic4_get_last_error() to query error information."]
    pub unsafe fn event_remove_window_closed(
        &mut self,
        handler: DisplayWindowClosedHandler,
        user_ptr: *mut c_void,
    ) -> self::Result<()> {
        ic4_sys::ic4_display_event_remove_window_closed(self.as_mut_ptr(), handler, user_ptr)
            .then_some(())
            .ok_or_else(|| self::get_last_error())
    }
}

impl Display {
    #[doc = " @brief Initialize the external OpenGL display\n\n @param[in] pDisplay\tThe external OpenGL display\n\n @return \\c true on success, otherwise \\c false.\\n\n\t\t\tUse @ref ic4_get_last_error() to query error information.\n\n @remarks\n This function must be called with the OpenGL context activated for the executing thread (e.g. @c makeCurrent)."]
    pub fn external_opengl_initialize(&mut self) -> self::Result<()> {
        unsafe {
            ic4_sys::ic4_display_external_opengl_initialize(self.as_mut_ptr())
                .then_some(())
                .ok_or(self::get_last_error())?;
        }
        Ok(())
    }
    #[doc = " @brief Updates the external OpenGL display with the latest image available.\n\n @param[in] pDisplay\tThe external OpenGL display\n @param[in] width\t\tWidth of the display window in physical pixels\n @param[in] height\tHeight of the display window in physical pixels\n\n @return \\c true on success, otherwise \\c false.\\n\n\t\t\tUse @ref ic4_get_last_error() to query error information.\n\n @remarks\n This function must be called with the OpenGL context activated for the executing thread (e.g. @c makeCurrent)."]
    pub fn external_opengl_render(&mut self, width: c_int, height: c_int) -> self::Result<()> {
        unsafe {
            ic4_sys::ic4_display_external_opengl_render(self.as_mut_ptr(), width, height)
                .then_some(())
                .ok_or(self::get_last_error())?;
        }
        Ok(())
    }
    #[doc = " @brief Notifies the external OpenGL display component that the window has been closed.\n\n @param[in] pDisplay\tThe external OpenGL display\n\n @return \\c true on success, otherwise \\c false.\\n\n\t\t\tUse @ref ic4_get_last_error() to query error information."]
    pub fn external_opengl_notify_window_closed(&mut self) -> self::Result<()> {
        unsafe {
            ic4_sys::ic4_display_external_opengl_notify_window_closed(self.as_mut_ptr())
                .then_some(())
                .ok_or(self::get_last_error())?;
        }
        Ok(())
    }
}
