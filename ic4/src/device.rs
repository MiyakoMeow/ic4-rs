#![allow(clippy::redundant_closure)]

use crate::*;

/*
 * DeviceEnum
 */

#[doc = " \\struct IC4_DEVICE_ENUM\n Device enumerator type.\n\n Call ic4_devenum_create() to create a device enumerator object.\n\n This type is reference-counted. Call ic4_devenum_ref() to increase the internal reference count,\n or ic4_devenum_unref() to decrease it. If the reference count reaches zero, the object is destroyed.\n\n After ic4_devenum_update_device_list() has been called, ic4_devenum_get_device_count() returns the number\n of detected devices.\n\n Use ic4_devenum_get_devinfo() to get a #IC4_DEVICE_INFO object providing information about one of the\n devices.\n\n @see ic4_devenum_create"]
pub type DeviceEnumOri = ic4_sys::IC4_DEVICE_ENUM;
bind_ptr_type!(
    DeviceEnum,
    ic4_sys::IC4_DEVICE_ENUM,
    ic4_sys::ic4_devenum_ref,
    ic4_sys::ic4_devenum_unref
);

impl DeviceEnum {
    #[doc = " Creates a new device enumerator\n\n @param[out] ppEnumerator A pointer to receive a pointer to the new device enumerator.\\n\n                          When the enumerator is no longer required, release the object reference\n                          using ic4_devenum_unref().\n\n @return \\c true on success, otherwise \\c false. Use ic4_get_last_error() to query error information.\n\n @see ic4_devenum_unref"]
    pub fn create() -> self::Result<Self> {
        let mut self_ptr = null_mut();
        unsafe {
            ic4_sys::ic4_devenum_create(ptr_from_mut(&mut self_ptr))
                .then_some(())
                .ok_or_else(|| self::get_last_error())?;
        }
        Ok(self_ptr.into())
    }
    #[doc = " Searches for video capture devices and populates the enumerator's internal device list.\n\n @param[in] pEnumerator A pointer to a device enumerator\n\n @return \\c true on success, otherwise \\c false. Use ic4_get_last_error() to query error information.\n\n @see ic4_devenum_get_device_count\n @see ic4_devenum_get_devinfo"]
    pub fn update_device_list(&mut self) -> self::Result<()> {
        unsafe {
            ic4_sys::ic4_devenum_update_device_list(self.as_mut_ptr())
                .then_some(())
                .ok_or_else(|| self::get_last_error())?;
        }
        Ok(())
    }
    #[doc = " Searches for interfaces and populates the enumerator's internal interface list.\n\n @param[in] pEnumerator A pointer to a device enumerator\n\n @return \\c true on success, otherwise \\c false. Use ic4_get_last_error() to query error information.\n\n @remarks Using the interface enumeration is entirely optional; for many use cases searching for devices\n\tvia ic4_devenum_update_device_list() is sufficient.\n\n @see ic4_devenum_get_interface_count\n @see ic4_devenum_get_devitf"]
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
    #[doc = " Returns the number of devices discovered by the previous call to ic4_devenum_update_device_list().\n\n @param[in] pEnumerator A pointer to a device enumerator\n\n @return The number of devices in the enumerator's internal device list\\n\n         If an error occurs, the function returns 0. ic4_get_last_error() can query error information.\n\n @see ic4_devenum_get_devinfo"]
    pub fn get_device_count(&mut self) -> c_int {
        unsafe { ic4_sys::ic4_devenum_get_device_count(self.as_mut_ptr()) }
    }
    #[doc = " Returns the number of interfaces discovered by the previous call to ic4_devenum_update_interface_list().\n\n @param[in] pEnumerator A pointer to a device enumerator\n\n @return The number of interfaces in the enumerator's internal interface list\\n\n         If an error occurs, the function returns 0. ic4_get_last_error() can query error information.\n\n @see ic4_devenum_get_devitf"]
    pub fn get_interface_count(&mut self) -> c_int {
        unsafe { ic4_sys::ic4_devenum_get_interface_count(self.as_mut_ptr()) }
    }
}

/*
 * DeviceInfo
 * Interface
 */

#[doc = " \\struct IC4_DEVICE_INFO\n Device information type.\n\n Instances of this type are created by calling ic4_devenum_get_devinfo().\n\n This type is reference-counted. Call ic4_devinfo_ref() to increase the internal reference count,\n or ic4_devinfo_unref() to decrease it. If the reference count reaches zero, the object is destroyed."]
pub type DeviceInfoOri = ic4_sys::IC4_DEVICE_INFO;
bind_ptr_type!(
    DeviceInfo,
    ic4_sys::IC4_DEVICE_INFO,
    ic4_sys::ic4_devinfo_ref,
    ic4_sys::ic4_devinfo_unref
);
#[doc = " \\struct IC4_INTERFACE\n Device interface type.\n\n Interfaces represent physical connections for cameras to the computer, e.g. network adapters or USB controllers.\n\n Instances of this type are created by calling ic4_devenum_get_devitf().\n\n This type is reference-counted. Call ic4_devitf_ref() to increase the internal reference count,\n or ic4_devitf_unref() to decrease it. If the reference count reaches zero, the object is destroyed."]
pub type InterfaceOri = ic4_sys::IC4_INTERFACE;
bind_ptr_type!(
    Interface,
    ic4_sys::IC4_INTERFACE,
    ic4_sys::ic4_devitf_ref,
    ic4_sys::ic4_devitf_unref
);

impl DeviceEnum {
    #[doc = " Returns a #IC4_DEVICE_INFO object describing one of the discovered video capture devices.\n\n @param[in] pEnumerator\tA pointer to a device enumerator\n @param[in] index\t\t\tList position of the device whose information is to be retrieved\n @param[out] ppInfo\t\tA pointer to receive a pointer to the a #IC4_DEVICE_INFO object.\\n\n\t\t\t\t\t\t\tWhen the device information object is no longer required, release the reference\n\t\t\t\t\t\t\tusing ic4_devenum_unref().\n\n @return \\c true on success, otherwise \\c false.\n\n @remarks ic4_devenum_update_device_list() has to be called before this function can return anything useful.\n          Use ic4_devenum_get_device_count() to determine the maximum valid value for \\a index.\n\n @see ic4_devinfo_unref"]
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
    #[doc = " Returns a #IC4_INTERFACE object describing one of the discovered interfaces.\n\n @param[in] pEnumerator\tA pointer to a device enumerator\n @param[in] index\t\t\tList position of the interface to be opened\n @param[out] ppInterface\tA pointer to receive a pointer to the a #IC4_INTERFACE object.\\n\n                          When the interface object is no longer required, release the reference\n                          using ic4_devitf_unref().\n\n @return \\c true on success, otherwise \\c false.\n\n @remarks ic4_devenum_update_interface_list() has to be called before this function can return anything useful.\n          Use ic4_devenum_get_interface_count() to determine the maximum valid value for \\a index.\n\n @see ic4_devinfo_unref"]
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

#[doc = " Function pointer for the device-list-changed handler\n\n @param[in] pDevEnum\tPointer to the device enumerator on which the callback was registered\n @param[in] user_ptr\tUser data that was specified when calling #ic4_devenum_event_add_device_list_changed()"]
pub type DeviceEnumDeviceListChangeHandler = ic4_sys::ic4_devenum_device_list_change_handler;

#[doc = " Function pointer for cleanup of the device-list-changed user data\n\n @param[in] user_ptr\tUser data that was specified when calling #ic4_devenum_event_add_device_list_changed()"]
pub type DeviceEnumDeviceListChangeDeleter = ic4_sys::ic4_devenum_device_list_change_deleter;

impl DeviceEnum {
    /// # Safety
    /// Unknown.
    /// 
    #[doc = " Registers a function to be called when the list of available video capture devices has (potentially) changed\n\n @param[in] pEnumerator\tThe device enumerator for which the callback is registered\n @param[in] handler\t\tThe function to be called when the list of available video capture devices has changed\n @param[in] user_ptr\t\tUser data to be passed in calls to \\a handler.\n @param[in] deleter\t\tA function to be called when the handler was unregistered and the user_ptr will no longer be required.\\n\n\t\t\t\t\t\t\tThe deleter function can be used to release data associated with \\a user_ptr.\\n\n\t\t\t\t\t\t\tThe \\a deleter function will be called when the device-list-changed handler is unregistered,\n\t\t\t\t\t\t\tor the device enumerator object itself is destroyed.\n\n @note\n To unregister a device-list-changed handler, call #ic4_devenum_event_remove_device_list_changed().\\n\n It is not guaranteed that every call to @a handler correlates to an actual change in the device list.\n\n @return \\c true on success, otherwise \\c false.\\n\n\t\t\tUse ic4_get_last_error() to query error information."]
    pub unsafe fn event_add_device_list_changed(
        &mut self,
        handler: DeviceEnumDeviceListChangeHandler,
        user_ptr: *mut c_void,
        deleter: DeviceEnumDeviceListChangeDeleter,
    ) -> self::Result<()> {
        ic4_sys::ic4_devenum_event_add_device_list_changed(
            self.as_mut_ptr(),
            handler,
            user_ptr,
            deleter,
        )
        .then_some(())
        .ok_or_else(|| self::get_last_error())
    }
    /// # Safety
    /// Unknown.
    ///
    #[doc = " Unregisters a device-list-changed handler that was previously registered using #ic4_devenum_event_add_device_list_changed().\n\n @param[in] pEnumerator\tThe device enumerator for which the callback is currently registered\n @param[in] handler\t\tPointer to the function to be unregistered\n @param[in] user_ptr\t\tUser data that the function was previously registered with\n\n @note\n The pair of \\a handler and \\a user_ptr has to be an exact match to the parameters used in the call to #ic4_devenum_event_add_device_list_changed().\n\n @return \\c true on success, otherwise \\c false.\\n\n\t\t\tUse ic4_get_last_error() to query error information."]
    pub unsafe fn event_remove_device_list_changed(
        &mut self,
        handler: DeviceEnumDeviceListChangeHandler,
        user_ptr: *mut c_void,
    ) -> self::Result<()> {
        ic4_sys::ic4_devenum_event_remove_device_list_changed(self.as_mut_ptr(), handler, user_ptr)
            .then_some(())
            .ok_or_else(|| self::get_last_error())
    }
}

#[doc = " @brief Contains the possible transport layer types."]
pub type TLType = ic4_sys::IC4_TL_TYPE;

impl Interface {
    #[doc = " Returns the name of the device interface.\n\n @param[in] pInterface A pointer to a device interface\n\n @return\tA null-terminated string containing the device interface's name.\\n\n\t\t\tThe memory pointed to by the returned pointer is valid as long as the interface object exists.\\n\n\t\t\tIf an error occurs, the function returns @c NULL. Use ic4_get_last_error() to query error information."]
    pub fn get_display_name(&self) -> &CStr {
        unsafe { CStr::from_ptr(ic4_sys::ic4_devitf_get_display_name(self.as_ptr())) }
    }
    #[doc = " @brief Returns the name of the transport layer that provides this interface object.\n\n This string can be interpreted as a name for the driver providing access to devices on the interface.\n\n @param[in] pInterface\tA pointer to a device interface\n\n @return\tA null-terminated string containing the transport layer name.\\n\n\t\t\tThe memory pointed to by the returned pointer is valid as long as the interface object exists.\\n\n\t\t\tIf an error occurs, the function returns @c NULL. Use ic4_get_last_error() to query error information."]
    pub fn get_tl_name(&self) -> &CStr {
        unsafe { CStr::from_ptr(ic4_sys::ic4_devitf_get_tl_name(self.as_ptr())) }
    }
    #[doc = " @brief Returns the version of the transport layer that provides this interface object.\n\n This string can be interpreted as driver version for the driver providing access devices on the interface.\n\n @param[in] pInterface\tA pointer to a device interface\n\n @return\tA null-terminated string containing the transport layer verision.\\n\n\t\t\tThe memory pointed to by the returned pointer is valid as long as the interface object exists.\\n\n\t\t\tIf an error occurs, the function returns @c NULL. Use ic4_get_last_error() to query error information."]
    pub fn get_tl_version(&self) -> &CStr {
        unsafe { CStr::from_ptr(ic4_sys::ic4_devitf_get_tl_version(self.as_ptr())) }
    }
    #[doc = " @brief Returns the type of the transport layer used by this interface.\n\n @param[in] pInterface\tA pointer to a device interface\n\n @return\tA #IC4_TL_TYPE value describing the type of the transport layer."]
    pub fn get_tl_type(&self) -> TLType {
        unsafe { ic4_sys::ic4_devitf_get_tl_type(self.as_ptr()) }
    }
}

impl Interface {
    #[doc = " Searches for video capture devices and populates the device interfaces's internal device list.\n\n @param[in] pInterface A pointer to a device interface\n\n @return \\c true on success, otherwise \\c false. Use ic4_get_last_error() to query error information.\n\n @see ic4_devitf_get_device_count\n @see ic4_devitf_get_devinfo"]
    pub fn update_device_list(&mut self) -> self::Result<()> {
        unsafe {
            ic4_sys::ic4_devitf_update_device_list(self.as_mut_ptr())
                .then_some(())
                .ok_or_else(|| self::get_last_error())?;
        }
        Ok(())
    }
    #[doc = " Returns the number of devices discovered by the previous call to ic4_devitf_update_device_list().\n\n @param[in] pInterface A pointer to a device interface\n\n @return The number of devices in the inferface's internal device list\\n\n         If an error occurs, the function returns 0. ic4_get_last_error() can query error information.\n\n @see ic4_devitf_get_devinfo"]
    pub fn get_device_count(&self) -> c_int {
        unsafe { ic4_sys::ic4_devitf_get_device_count(self.as_ptr()) }
    }
    #[doc = " Returns a #IC4_DEVICE_INFO object describing one of the discovered video capture devices.\n\n @param[in] pInterface\tA pointer to a device interface\n @param[in] index\t\t\tList position of the device whose information is to be retrieved\n @param[out] ppInfo\t\tA pointer to receive a pointer to the a #IC4_DEVICE_INFO object.\\n\n\t\t\t\t\t\t\tWhen the device information object is no longer required, release the reference\n\t\t\t\t\t\t\tusing ic4_devenum_unref().\n\n @return \\c true on success, otherwise \\c false.\n\n @remarks ic4_devitf_update_device_list() has to be called before this function can return anything useful.\n          Use ic4_devitf_get_device_count() to determine the maximum valid value for \\a index.\n\n @see ic4_devinfo_unref"]
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
    #[doc = " Opens the property map for the specified device interface.\n\n The property map can be used to query advanced interface information or configure the interface and its attached devices.\n\n @param[in] pInterface A pointer to a device interface\n @param[out] ppMap     A pointer to a pointer to a #IC4_PROPERTY_MAP object.\\n\n                       When the property map is no longer required, release the reference using ic4_propmap_unref().\n\n @return \\c true on success, otherwise \\c false. Use ic4_get_last_error() to query error information."]
    pub fn get_property_map(&self) -> self::Result<PropertyMap> {
        let mut ptr = null_mut();
        unsafe {
            ic4_sys::ic4_devitf_get_property_map(self.as_ptr(), ptr_from_mut(&mut ptr))
                .then_some(())
                .ok_or_else(|| self::get_last_error())?;
        }
        Ok(ptr.into())
    }
    #[doc = " Checks whether two device interface objects refer to the same interface.\n\n @param[in] pInterface1 First interface object\n @param[in] pInterface2 Second interface object\n\n @return\n    \\c true if the device interface objects refer to the same interface, otherwise \\c false.\\n\n    If both pointers are NULL, the function returns \\c true."]
    pub fn equals(&self, other: &Self) -> bool {
        self == other
    }
}

impl PartialEq for Interface {
    #[doc = " Checks whether two device interface objects refer to the same interface.\n\n @param[in] pInterface1 First interface object\n @param[in] pInterface2 Second interface object\n\n @return\n    \\c true if the device interface objects refer to the same interface, otherwise \\c false.\\n\n    If both pointers are NULL, the function returns \\c true."]
    fn eq(&self, other: &Self) -> bool {
        unsafe { ic4_sys::ic4_devitf_equals(self.as_ptr(), other.as_ptr()) }
    }
}

impl Eq for Interface {}

impl DeviceInfo {
    #[doc = " Get the model name from a device information object.\n\n @param[in] pInfo A pointer to a device information object\n\n @return\n    A pointer to a null-terminated string containing the device's model name, or \\c NULL if an error occured.\\n\n    Use ic4_get_last_error() to query error information.\\n\n    The memory pointed to by the return value is valid as long as the device information object exists."]
    pub fn get_model_name(&self) -> &CStr {
        unsafe { CStr::from_ptr(ic4_sys::ic4_devinfo_get_model_name(self.as_ptr())) }
    }
    #[doc = " Get the textual representation of the serial number from a device information object.\n\n @param[in] pInfo A pointer to a device information object\n\n @return\n    A pointer to a null-terminated string containing the device's serial number, or \\c NULL if an error occured.\\n\n    Use ic4_get_last_error() to query error information.\\n\n    The memory pointed to by the return value is valid as long as the device information object exists.\\n\n\t  The format of the serial number string is device-specific.\n"]
    pub fn get_serial(&self) -> &CStr {
        unsafe { CStr::from_ptr(ic4_sys::ic4_devinfo_get_serial(self.as_ptr())) }
    }
    #[doc = " Get the device version from a device information object.\n\n @param[in] pInfo A pointer to a device information object\n\n @return\n    A pointer to a null-terminated string containing the device's version information, or \\c NULL if an error occured.\\n\n    Use ic4_get_last_error() to query error information.\\n\n    The memory pointed to by the return value is valid as long as the device information object exists.\\n\n\t  The format of the device version is device-specific."]
    pub fn get_version(&self) -> &CStr {
        unsafe { CStr::from_ptr(ic4_sys::ic4_devinfo_get_version(self.as_ptr())) }
    }
    #[doc = " Get the device's user-defined identifier from a device information object.\n\n @param[in] pInfo A pointer to a device information object\n\n @return\n    A pointer to a null-terminated string containing the device's user-defined identifier, or \\c NULL if an error occured.\\n\n    Use ic4_get_last_error() to query error information.\\n\n    The memory pointed to by the return value is valid as long as the device information object exists.\n\n @remarks\n    If supported by the device, the device's user-defined identifier can be configured through the @c DeviceUserID feature\n    in the device's property map."]
    pub fn get_user_id(&self) -> &CStr {
        unsafe { CStr::from_ptr(ic4_sys::ic4_devinfo_get_user_id(self.as_ptr())) }
    }
    #[doc = " Get the device's unique name from a device information object.\n\n @param[in] pInfo A pointer to a device information object\n\n @return\n    A pointer to a null-terminated string containing the device's unique name, or \\c NULL if an error occured.\\n\n    Use ic4_get_last_error() to query error information.\\n\n    The memory pointed to by the return value is valid as long as the device information object exists.\\n\n\t  The unique name consists of an identifier for the device driver and the device's serial number,\n    allowing devices to be uniquely identified by a single string."]
    pub fn get_unique_name(&self) -> &CStr {
        unsafe { CStr::from_ptr(ic4_sys::ic4_devinfo_get_unique_name(self.as_ptr())) }
    }
    #[doc = " Checks whether two device information objects refer to the same video capture device.\n\n @param[in] pInfo1 First device info\n @param[in] pInfo2 Second device info\n\n @return\n    \\c true if the device information objects refer to the same video capture device, otherwise \\c false.\\n\n    If both pointers are NULL, the function returns \\c true."]
    pub fn equals(&self, other: &Self) -> bool {
        self == other
    }
}

impl PartialEq for DeviceInfo {
    #[doc = " Checks whether two device information objects refer to the same video capture device.\n\n @param[in] pInfo1 First device info\n @param[in] pInfo2 Second device info\n\n @return\n    \\c true if the device information objects refer to the same video capture device, otherwise \\c false.\\n\n    If both pointers are NULL, the function returns \\c true."]
    fn eq(&self, other: &Self) -> bool {
        unsafe { ic4_sys::ic4_devinfo_equals(self.as_ptr(), other.as_ptr()) }
    }
}

impl Eq for DeviceInfo {}

impl DeviceInfo {
    #[doc = " @brief Returns the interface the device represented by the device information object is attached to.\n\n @param[in] pInfo\t\t\tA device information object\n @param[out] ppInterface\tA pointer to receive a pointer to the a #IC4_INTERFACE object.\\n\n\t\t\t\t\t\t\tWhen the interface object is no longer required, release the reference\n\t\t\t\t\t\t\tusing ic4_devitf_unref().\n\n @return \\c true on success, otherwise \\c false. Use ic4_get_last_error() to query error information."]
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
