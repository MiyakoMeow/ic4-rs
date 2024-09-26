#![allow(clippy::redundant_closure)]

use crate::*;

/*
 * Grabber
 */

bind_ptr_type!(
    Grabber,
    ic4_sys::IC4_GRABBER,
    ic4_sys::ic4_grabber_ref,
    ic4_sys::ic4_grabber_unref
);

impl Grabber {
    pub fn create() -> self::Result<Self> {
        let mut self_ptr = null_mut();
        unsafe {
            ic4_sys::ic4_grabber_create(ptr_from_mut(&mut self_ptr))
                .then_some(())
                .ok_or_else(|| self::get_last_error())?;
        }
        Ok(self_ptr.into())
    }
}

impl Grabber {
    pub fn device_open(&mut self, device_info: &mut DeviceInfo) -> self::Result<()> {
        unsafe {
            ic4_sys::ic4_grabber_device_open(self.as_mut_ptr(), device_info.as_mut_ptr())
                .then_some(())
                .ok_or_else(|| self::get_last_error())?;
        }
        Ok(())
    }
    pub fn device_open_by_identifier(&mut self, identifier: &CStr) -> self::Result<()> {
        unsafe {
            ic4_sys::ic4_grabber_device_open_by_identifier(self.as_mut_ptr(), identifier.as_ptr())
                .then_some(())
                .ok_or_else(|| self::get_last_error())?;
        }
        Ok(())
    }
    pub fn get_device(&mut self) -> self::Result<DeviceInfo> {
        let mut device_info = null_mut();
        unsafe {
            ic4_sys::ic4_grabber_get_device(self.as_mut_ptr(), ptr_from_mut(&mut device_info))
                .then_some(())
                .ok_or_else(|| self::get_last_error())?;
        }
        Ok(device_info.into())
    }
    pub fn is_device_open(&mut self) -> bool {
        unsafe { ic4_sys::ic4_grabber_is_device_open(self.as_mut_ptr()) }
    }
    pub fn is_device_valid(&mut self) -> bool {
        unsafe { ic4_sys::ic4_grabber_is_device_valid(self.as_mut_ptr()) }
    }
    pub fn device_close(&mut self) -> self::Result<()> {
        unsafe {
            ic4_sys::ic4_grabber_device_close(self.as_mut_ptr())
                .then_some(())
                .ok_or_else(|| self::get_last_error())?;
        }
        Ok(())
    }
}

impl Grabber {
    pub fn device_get_property_map(&mut self) -> self::Result<PropertyMap> {
        let mut property_map = null_mut();
        unsafe {
            ic4_sys::ic4_grabber_device_get_property_map(
                self.as_mut_ptr(),
                ptr_from_mut(&mut property_map),
            )
            .then_some(())
            .ok_or_else(|| self::get_last_error())?;
        }
        Ok(property_map.into())
    }
    pub fn driver_get_property_map(&mut self) -> self::Result<PropertyMap> {
        let mut property_map = null_mut();
        unsafe {
            ic4_sys::ic4_grabber_driver_get_property_map(
                self.as_mut_ptr(),
                ptr_from_mut(&mut property_map),
            )
            .then_some(())
            .ok_or_else(|| self::get_last_error())?;
        }
        Ok(property_map.into())
    }
}

impl Grabber {
    pub fn stream_setup(
        &mut self,
        sink: &mut Sink,
        display: &mut self::Display,
        start_acquisition: bool,
    ) -> self::Result<()> {
        unsafe {
            ic4_sys::ic4_grabber_stream_setup(
                self.as_mut_ptr(),
                sink.as_mut_ptr(),
                display.as_mut_ptr(),
                start_acquisition,
            )
            .then_some(())
            .ok_or_else(|| self::get_last_error())?;
        }
        Ok(())
    }
    pub fn stream_stop(&mut self) -> self::Result<()> {
        unsafe {
            ic4_sys::ic4_grabber_stream_stop(self.as_mut_ptr())
                .then_some(())
                .ok_or_else(|| self::get_last_error())?;
        }
        Ok(())
    }
    pub fn is_streaming(&mut self) -> bool {
        unsafe { ic4_sys::ic4_grabber_is_streaming(self.as_mut_ptr()) }
    }
    pub fn acquisition_start(&mut self) -> self::Result<()> {
        unsafe {
            ic4_sys::ic4_grabber_acquisition_start(self.as_mut_ptr())
                .then_some(())
                .ok_or_else(|| self::get_last_error())?;
        }
        Ok(())
    }
    pub fn acquisition_stop(&mut self) -> self::Result<()> {
        unsafe {
            ic4_sys::ic4_grabber_acquisition_stop(self.as_mut_ptr())
                .then_some(())
                .ok_or_else(|| self::get_last_error())?;
        }
        Ok(())
    }
    pub fn is_acquisition_active(&mut self) -> bool {
        unsafe { ic4_sys::ic4_grabber_is_acquisition_active(self.as_mut_ptr()) }
    }
    pub fn get_sink(&mut self) -> self::Result<Sink> {
        let mut sink = null_mut();
        unsafe {
            ic4_sys::ic4_grabber_get_sink(self.as_mut_ptr(), ptr_from_mut(&mut sink))
                .then_some(())
                .ok_or_else(|| self::get_last_error())?;
        }
        Ok(sink.into())
    }
    pub fn get_display(&mut self) -> self::Result<self::Display> {
        let mut display = null_mut();
        unsafe {
            ic4_sys::ic4_grabber_get_display(self.as_mut_ptr(), ptr_from_mut(&mut display))
                .then_some(())
                .ok_or_else(|| self::get_last_error())?;
        }
        Ok(display.into())
    }
}

pub type GrabberDeviceLostHandlerOri = ic4_sys::ic4_grabber_device_lost_handler;
bind_type!(GrabberDeviceLostHandler, GrabberDeviceLostHandlerOri);
pub type GrabberDeviceLostDeleterOri = ic4_sys::ic4_grabber_device_lost_deleter;
bind_type!(GrabberDeviceLostDeleter, GrabberDeviceLostDeleterOri);

impl Grabber {
    /// # Safety
    /// Unknown.
    pub unsafe fn event_add_device_lost(
        &mut self,
        handler: GrabberDeviceLostHandler,
        user_ptr: *mut c_void,
        deleter: GrabberDeviceLostDeleter,
    ) -> self::Result<()> {
        ic4_sys::ic4_grabber_event_add_device_lost(
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
    pub unsafe fn event_remove_device_lost(
        &mut self,
        handler: GrabberDeviceLostHandler,
        user_ptr: *mut c_void,
    ) -> self::Result<()> {
        ic4_sys::ic4_grabber_event_remove_device_lost(self.as_mut_ptr(), handler.inner, user_ptr)
            .then_some(())
            .ok_or_else(|| self::get_last_error())
    }
}

/*
 * Stream
 */
pub type StreamStatsOri = ic4_sys::IC4_STREAM_STATS;
bind_type!(StreamStats, StreamStatsOri);

impl Default for StreamStats {
    fn default() -> Self {
        StreamStats::from(StreamStatsOri {
            device_delivered: 0,
            device_transmission_error: 0,
            device_underrun: 0,
            transform_delivered: 0,
            transform_underrun: 0,
            sink_delivered: 0,
            sink_underrun: 0,
            sink_ignored: 0,
        })
    }
}

impl Grabber {
    pub fn get_stream_stats(&mut self) -> self::Result<StreamStats> {
        let mut property_map: StreamStats = Default::default();
        unsafe {
            ic4_sys::ic4_grabber_get_stream_stats(
                self.as_mut_ptr(),
                ptr_from_mut(&mut property_map.inner),
            )
            .then_some(())
            .ok_or_else(|| self::get_last_error())?;
        }
        Ok(property_map)
    }
}

pub type DeviceStateAllocatorOri = ic4_sys::ic4_device_state_allocator;
bind_type!(DeviceStateAllocator, DeviceStateAllocatorOri);

impl Grabber {
    pub fn device_save_state(&mut self, alloc: DeviceStateAllocator) -> self::Result<&mut [u8]> {
        let mut vaild_value_set = null_mut();
        let mut vaild_value_length = 0;
        unsafe {
            ic4_sys::ic4_grabber_device_save_state(
                self.as_mut_ptr(),
                alloc.inner,
                ptr_from_mut(&mut vaild_value_set),
                ptr_from_mut(&mut vaild_value_length),
            )
            .then_some(())
            .ok_or_else(|| self::get_last_error())?;
            Ok(std::slice::from_raw_parts_mut(
                vaild_value_set as *mut u8,
                vaild_value_length,
            ))
        }
    }
    pub fn device_save_state_to_file(&mut self, path: &CStr) -> self::Result<()> {
        unsafe {
            ic4_sys::ic4_grabber_device_save_state_to_file(self.as_mut_ptr(), path.as_ptr())
                .then_some(())
                .ok_or_else(|| self::get_last_error())?;
        }
        Ok(())
    }
    pub fn device_open_from_state(&mut self, data: &[u8]) -> self::Result<()> {
        unsafe {
            ic4_sys::ic4_grabber_device_open_from_state(
                self.as_mut_ptr(),
                data.as_ptr() as *const c_void,
                data.len(),
            )
            .then_some(())
            .ok_or_else(|| self::get_last_error())?;
        }
        Ok(())
    }
    pub fn device_open_from_state_file(&mut self, path: &CStr) -> self::Result<()> {
        unsafe {
            ic4_sys::ic4_grabber_device_open_from_state_file(self.as_mut_ptr(), path.as_ptr())
                .then_some(())
                .ok_or_else(|| self::get_last_error())?;
        }
        Ok(())
    }
}
