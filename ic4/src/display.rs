#![allow(clippy::redundant_closure)]

use crate::*;

/*
 * Display
 */

bind_ptr_type!(
    DisplayIC,
    ic4_sys::IC4_DISPLAY,
    ic4_sys::ic4_display_ref,
    ic4_sys::ic4_display_unref
);

pub type DisplayTypeOri = ic4_sys::IC4_DISPLAY_TYPE;
bind_type!(DisplayType, DisplayTypeOri);

pub type DisplayStatsOri = ic4_sys::IC4_DISPLAY_STATS;
bind_type!(DisplayStats, DisplayStatsOri);

impl Default for DisplayStats {
    fn default() -> Self {
        Self {
            inner: DisplayStatsOri {
                num_frames_displayed: 0,
                num_frames_dropped: 0,
            },
        }
    }
}

pub type WindowHandleOri = ic4_sys::IC4_WINDOW_HANDLE;
bind_type!(WindowHandle, WindowHandleOri);

impl DisplayIC {
    pub fn create(display_type: DisplayType, parent: WindowHandle) -> self::Result<Self> {
        let mut ptr = null_mut();
        unsafe {
            ic4_sys::ic4_display_create(display_type.into(), parent.into(), ptr_from_mut(&mut ptr))
                .then_some(())
                .ok_or(self::get_last_error())?;
        }
        Ok(ptr.into())
    }

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

impl DisplayIC {
    pub fn display_buffer(&mut self, image_buffer: &ImageBuffer) -> self::Result<()> {
        unsafe {
            ic4_sys::ic4_display_display_buffer(self.as_mut_ptr(), image_buffer.as_ptr())
                .then_some(())
                .ok_or(self::get_last_error())?;
        }
        Ok(())
    }
    pub fn get_stats(&mut self) -> self::Result<DisplayStats> {
        let mut result: DisplayStats = Default::default();
        unsafe {
            ic4_sys::ic4_display_get_stats(self.as_mut_ptr(), ptr_from_mut(&mut result.inner))
                .then_some(())
                .ok_or(self::get_last_error())?;
        }
        Ok(result)
    }
}

pub type DisplayRenderPositionOri = ic4_sys::IC4_DISPLAY_RENDER_POSITION;
bind_type!(DisplayRenderPosition, DisplayRenderPositionOri);

impl DisplayIC {
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
                pos.inner,
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

pub type DisplayWindowClosedHandlerOri = ic4_sys::ic4_display_window_closed_handler;
bind_type!(DisplayWindowClosedHandler, DisplayWindowClosedHandlerOri);
pub type DisplayWindowClosedDeleterOri = ic4_sys::ic4_display_window_closed_deleter;
bind_type!(DisplayWindowClosedDeleter, DisplayWindowClosedDeleterOri);

impl DisplayIC {
    /// # Safety
    /// Unknown.
    pub unsafe fn event_add_window_closed(
        &mut self,
        handler: DisplayWindowClosedHandler,
        user_ptr: *mut c_void,
        deleter: DisplayWindowClosedDeleter,
    ) -> self::Result<()> {
        ic4_sys::ic4_display_event_add_window_closed(
            self.as_mut_ptr(),
            handler.inner,
            user_ptr,
            deleter.inner,
        )
        .then_some(())
        .ok_or_else(|| self::get_last_error())
    }
    /// # Safety
    /// Unknown.
    pub unsafe fn event_remove_window_closed(
        &mut self,
        handler: DisplayWindowClosedHandler,
        user_ptr: *mut c_void,
    ) -> self::Result<()> {
        ic4_sys::ic4_display_event_remove_window_closed(self.as_mut_ptr(), handler.inner, user_ptr)
            .then_some(())
            .ok_or_else(|| self::get_last_error())
    }
}

impl DisplayIC {
    pub fn external_opengl_initialize(&mut self) -> self::Result<()> {
        unsafe {
            ic4_sys::ic4_display_external_opengl_initialize(self.as_mut_ptr())
                .then_some(())
                .ok_or(self::get_last_error())?;
        }
        Ok(())
    }
    pub fn external_opengl_render(&mut self, width: c_int, height: c_int) -> self::Result<()> {
        unsafe {
            ic4_sys::ic4_display_external_opengl_render(self.as_mut_ptr(), width, height)
                .then_some(())
                .ok_or(self::get_last_error())?;
        }
        Ok(())
    }
    pub fn external_opengl_notify_window_closed(&mut self) -> self::Result<()> {
        unsafe {
            ic4_sys::ic4_display_external_opengl_notify_window_closed(self.as_mut_ptr())
                .then_some(())
                .ok_or(self::get_last_error())?;
        }
        Ok(())
    }
}
