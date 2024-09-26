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

// TODO

/*
 * Sink
 */

bind_ptr_type!(
    Sink,
    ic4_sys::IC4_SINK,
    ic4_sys::ic4_sink_ref,
    ic4_sys::ic4_sink_unref
);
