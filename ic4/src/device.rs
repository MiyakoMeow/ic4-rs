#![allow(clippy::redundant_closure)]

use crate::*;

/*
 * DeviceEnum
 */

bind_ptr_type!(
    DeviceEnum,
    ic4_sys::IC4_DEVICE_ENUM,
    ic4_sys::ic4_devenum_ref,
    ic4_sys::ic4_devenum_unref
);

impl DeviceEnum {
    pub fn create() -> self::Result<Self> {
        let mut self_ptr = null_mut();
        unsafe {
            ic4_sys::ic4_devenum_create(ptr_from_mut(&mut self_ptr))
                .then_some(())
                .ok_or_else(|| self::get_last_error())?;
        }
        Ok(self_ptr.into())
    }
    pub fn update_device_list(&mut self) -> self::Result<()> {
        unsafe {
            ic4_sys::ic4_devenum_update_device_list(self.as_mut_ptr())
                .then_some(())
                .ok_or_else(|| self::get_last_error())?;
        }
        Ok(())
    }
    pub fn update_interface_list(&mut self) -> self::Result<()> {
        unsafe {
            ic4_sys::ic4_devenum_update_interface_list(self.as_mut_ptr())
                .then_some(())
                .ok_or_else(|| self::get_last_error())?;
        }
        Ok(())
    }
}

impl DeviceEnum {
    pub fn get_device_count(&mut self) -> c_int {
        unsafe { ic4_sys::ic4_devenum_get_device_count(self.as_mut_ptr()) }
    }
    pub fn get_interface_count(&mut self) -> c_int {
        unsafe { ic4_sys::ic4_devenum_get_interface_count(self.as_mut_ptr()) }
    }
}

/*
 * DeviceInfo
 * Interface
 */

bind_ptr_type!(
    DeviceInfo,
    ic4_sys::IC4_DEVICE_INFO,
    ic4_sys::ic4_devinfo_ref,
    ic4_sys::ic4_devinfo_unref
);
bind_ptr_type!(
    Interface,
    ic4_sys::IC4_INTERFACE,
    ic4_sys::ic4_devitf_ref,
    ic4_sys::ic4_devitf_unref
);

impl DeviceEnum {
    pub fn get_device_info(&mut self, index: usize) -> self::Result<DeviceInfo> {
        let mut result = null_mut();
        unsafe {
            ic4_sys::ic4_devenum_get_devinfo(
                self.as_mut_ptr(),
                index as c_int,
                ptr_from_mut(&mut result),
            )
            .then_some(())
            .ok_or_else(|| self::get_last_error())?;
        }
        Ok(result.into())
    }
    pub fn get_device_interface(&mut self, index: usize) -> self::Result<Interface> {
        let mut result = null_mut();
        unsafe {
            ic4_sys::ic4_devenum_get_devitf(
                self.as_mut_ptr(),
                index as c_int,
                ptr_from_mut(&mut result),
            )
            .then_some(())
            .ok_or_else(|| self::get_last_error())?;
        }
        Ok(result.into())
    }
}

pub type DeviceEnumDeviceListChangeHandlerOri = ic4_sys::ic4_devenum_device_list_change_handler;
bind_type!(
    DeviceEnumDeviceListChangeHandler,
    DeviceEnumDeviceListChangeHandlerOri
);
pub type DeviceEnumDeviceListChangeDeleterOri = ic4_sys::ic4_devenum_device_list_change_deleter;
bind_type!(
    DeviceEnumDeviceListChangeDeleter,
    DeviceEnumDeviceListChangeDeleterOri
);

impl DeviceEnum {
    /// # Safety
    /// Unknown.
    pub unsafe fn event_add_device_list_changed(
        &mut self,
        handler: DeviceEnumDeviceListChangeHandler,
        user_ptr: *mut c_void,
        deleter: DeviceEnumDeviceListChangeDeleter,
    ) -> self::Result<()> {
        ic4_sys::ic4_devenum_event_add_device_list_changed(
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
    pub unsafe fn event_remove_device_list_changed(
        &mut self,
        handler: DeviceEnumDeviceListChangeHandler,
        user_ptr: *mut c_void,
    ) -> self::Result<()> {
        ic4_sys::ic4_devenum_event_remove_device_list_changed(
            self.as_mut_ptr(),
            handler.inner,
            user_ptr,
        )
        .then_some(())
        .ok_or_else(|| self::get_last_error())
    }
}

pub type TLTypeOri = ic4_sys::IC4_TL_TYPE;
bind_type!(TLType, TLTypeOri);

impl Interface {
    pub fn get_display_name(&self) -> &CStr {
        unsafe { CStr::from_ptr(ic4_sys::ic4_devitf_get_display_name(self.as_ptr())) }
    }
    pub fn get_tl_name(&self) -> &CStr {
        unsafe { CStr::from_ptr(ic4_sys::ic4_devitf_get_tl_name(self.as_ptr())) }
    }
    pub fn get_tl_version(&self) -> &CStr {
        unsafe { CStr::from_ptr(ic4_sys::ic4_devitf_get_tl_version(self.as_ptr())) }
    }
    pub fn get_tl_type(&self) -> TLType {
        unsafe { ic4_sys::ic4_devitf_get_tl_type(self.as_ptr()).into() }
    }
}

impl Interface {
    pub fn update_device_list(&mut self) -> self::Result<()> {
        unsafe {
            ic4_sys::ic4_devitf_update_device_list(self.as_mut_ptr())
                .then_some(())
                .ok_or_else(|| self::get_last_error())?;
        }
        Ok(())
    }
    pub fn get_device_count(&self) -> c_int {
        unsafe { ic4_sys::ic4_devitf_get_device_count(self.as_ptr()) }
    }
    pub fn get_device_info(&self, index: usize) -> self::Result<DeviceInfo> {
        let mut ptr = null_mut();
        unsafe {
            ic4_sys::ic4_devitf_get_devinfo(self.as_ptr(), index as c_int, ptr_from_mut(&mut ptr))
                .then_some(())
                .ok_or_else(|| self::get_last_error())?;
        }
        Ok(ptr.into())
    }
}

impl Interface {
    pub fn get_property_map(&self) -> self::Result<PropertyMap> {
        let mut ptr = null_mut();
        unsafe {
            ic4_sys::ic4_devitf_get_property_map(self.as_ptr(), ptr_from_mut(&mut ptr))
                .then_some(())
                .ok_or_else(|| self::get_last_error())?;
        }
        Ok(ptr.into())
    }
    pub fn equals(&self, other: &Self) -> bool {
        self == other
    }
}

impl PartialEq for Interface {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ic4_sys::ic4_devitf_equals(self.as_ptr(), other.as_ptr()) }
    }
}

impl Eq for Interface {}

impl DeviceInfo {
    pub fn get_model_name(&self) -> &CStr {
        unsafe { CStr::from_ptr(ic4_sys::ic4_devinfo_get_model_name(self.as_ptr())) }
    }
    pub fn get_serial(&self) -> &CStr {
        unsafe { CStr::from_ptr(ic4_sys::ic4_devinfo_get_serial(self.as_ptr())) }
    }
    pub fn get_version(&self) -> &CStr {
        unsafe { CStr::from_ptr(ic4_sys::ic4_devinfo_get_version(self.as_ptr())) }
    }
    pub fn get_user_id(&self) -> &CStr {
        unsafe { CStr::from_ptr(ic4_sys::ic4_devinfo_get_user_id(self.as_ptr())) }
    }
    pub fn get_unique_name(&self) -> &CStr {
        unsafe { CStr::from_ptr(ic4_sys::ic4_devinfo_get_unique_name(self.as_ptr())) }
    }
    pub fn equals(&self, other: &Self) -> bool {
        self == other
    }
}

impl PartialEq for DeviceInfo {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ic4_sys::ic4_devinfo_equals(self.as_ptr(), other.as_ptr()) }
    }
}

impl Eq for DeviceInfo {}

impl DeviceInfo {
    pub fn get_device_interface(&self) -> self::Result<Interface> {
        let mut ptr = null_mut();
        unsafe {
            ic4_sys::ic4_devinfo_get_devitf(self.as_ptr(), ptr_from_mut(&mut ptr))
                .then_some(())
                .ok_or_else(|| self::get_last_error())?;
        }
        Ok(ptr.into())
    }
}
