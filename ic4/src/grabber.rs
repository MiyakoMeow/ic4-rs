#![allow(clippy::redundant_closure)]

use crate::*;

/*
 * Grabber
 */

#[doc = " @struct IC4_GRABBER\n\n @brief Represents an opened video capture device, allowing device configuration and stream setup.\n\n The grabber object is the core component used when working with video capture devices.\n\n This type is opaque, programs only use pointers of type \\c IC4_GRABBER*.\n\n Grabber objects are reference-counted, and created with an initial reference count of one.\n To share a grabber object between multiple parts of a program, create a new reference by calling #ic4_grabber_ref().\n When a reference is no longer required, call #ic4_grabber_unref().\n\n If the grabber object's internal reference count reaches zero, the grabber object is destroyed.\n\n @note\n Some object references, e.g. #IC4_IMAGE_BUFFER, can keep the device and/or driver opened as long as they exist,\n since they point into device driver memory. To free all device-related resources, all objects references have to be released by calling\n their unref-function."]
pub type GrabberOri = ic4_sys::IC4_GRABBER;
bind_ptr_type!(
    Grabber,
    ic4_sys::IC4_GRABBER,
    ic4_sys::ic4_grabber_ref,
    ic4_sys::ic4_grabber_unref
);

impl Grabber {
    #[doc = " @brief Creates a new grabber.\n\n @param[out] ppGrabber Pointer to receive the handle to the new grabber object.\\n\n             When the grabber is no longer required, release the object reference using #ic4_grabber_unref().\n\n @return \\c true on success, otherwise \\c false.\\n\n\t\t\tUse ic4_get_last_error() to query error information.\n\n @see ic4_grabber_unref"]
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
    #[doc = " @brief Opens the video capture device specified by the passed #IC4_DEVICE_INFO.\n\n @param[in] pGrabber\tA grabber instance that does not have an opened video capture device\n @param[in] dev\t\tA #IC4_DEVICE_INFO representing the video capture device to be opened\n\n @return \\c true on success, otherwise \\c false.\\n\n\t\t\tUse ic4_get_last_error() to query error information.\n\n @remarks\n If the grabber already has a device open, the function will fail and the error value is set to @ref IC4_ERROR_INVALID_OPERATION.\n\n @see ic4_grabber_open_dev_by_name\n @see ic4_grabber_device_close\n @see ic4_grabber_is_device_open\n @see ic4_grabber_is_device_valid"]
    pub fn device_open(&mut self, device_info: &mut DeviceInfo) -> self::Result<()> {
        unsafe {
            ic4_sys::ic4_grabber_device_open(self.as_mut_ptr(), device_info.as_mut_ptr())
                .then_some(())
                .ok_or_else(|| self::get_last_error())?;
        }
        Ok(())
    }
    #[doc = " @brief Opens the video capture matching the specified identifier.\n\n @param[in] pGrabber\t\tA grabber instance that does not have an opened video capture device\n @param[in] identifier\tThe model name, unique name, serial, user id, IPV4 address or MAC address of a connected video capture device\n\n @return \\c true on success, otherwise \\c false.\\n\n\t\t\tUse ic4_get_last_error() to query error information.\n\n @remarks\n If the grabber already has a device open, the function will fail and the error value is set to #IC4_ERROR_INVALID_OPERATION. \\n\n If there are multiple devices matching the specified identifier, the function will fail and the error value is set to #IC4_ERROR_AMBIGUOUS. \\n\n If there is no device with the specified identifier, the function will fail and the error value is set to #IC4_ERROR_DEVICE_NOT_FOUND. \\n\n\n @see ic4_grabber_device_open\n @see ic4_grabber_device_close\n @see ic4_grabber_is_device_open\n @see ic4_grabber_is_device_valid"]
    pub fn device_open_by_identifier(&mut self, identifier: &CStr) -> self::Result<()> {
        unsafe {
            ic4_sys::ic4_grabber_device_open_by_identifier(self.as_mut_ptr(), identifier.as_ptr())
                .then_some(())
                .ok_or_else(|| self::get_last_error())?;
        }
        Ok(())
    }
    #[doc = " @brief Returns information about the currently opened video capture device.\n\n @param[in] pGrabber\tA grabber instance with an opened video capture device\n @param[out] ppDev\tA pointer to a handle to receive the device information.\\n\n\t\t\t\t\t\tWhen the device information is no longer required, release the object reference using #ic4_devinfo_unref().\n\n @return \\c true on success, otherwise \\c false.\\n\n\t\t\tUse ic4_get_last_error() to query error information.\n\n @remarks\n If the grabber does not have an opened video capture device, the function will fail and the error value is set to #IC4_ERROR_INVALID_OPERATION.\\n\n\n @see ic4_grabber_is_device_open"]
    pub fn get_device(&mut self) -> self::Result<DeviceInfo> {
        let mut device_info = null_mut();
        unsafe {
            ic4_sys::ic4_grabber_get_device(self.as_mut_ptr(), ptr_from_mut(&mut device_info))
                .then_some(())
                .ok_or_else(|| self::get_last_error())?;
        }
        Ok(device_info.into())
    }
    #[doc = " @brief Checks whether the grabber currently has an opened video capture device.\n\n @param[in] pGrabber\tA grabber object\n\n @return \\c true, if the grabber has an opened video capture device, otherwise \\c false.\n\n @remarks\n This function neither clears nor sets the error status returned by ic4_get_last_error().\n\n @see ic4_grabber_device_open"]
    pub fn is_device_open(&mut self) -> bool {
        unsafe { ic4_sys::ic4_grabber_is_device_open(self.as_mut_ptr()) }
    }
    #[doc = " @brief Checks whether the grabber's currently opened video capture device is ready to use.\n\n @param[in] pGrabber\tA grabber object\n\n @return \\c true, if the grabber has an opened video capture device that is ready to use, otherwise \\c false.\n\n @remarks\n This function neither clears nor sets the error status returned by ic4_get_last_error().\\n\n There are multiple reasons for why this function may return \\c false:\n\t- No device has been opened\n\t- The device was disconnected\n\t- There is a loose hardware connection\n\t- There was an internal error in the video capture device\n\t- There was a driver error"]
    pub fn is_device_valid(&mut self) -> bool {
        unsafe { ic4_sys::ic4_grabber_is_device_valid(self.as_mut_ptr()) }
    }
    #[doc = " @brief Closes the video capture device currently opened by this grabber instance\n\n @param[in] pGrabber\tA grabber with an opened video capture device\n\n @return \\c true on success, otherwise \\c false.\\n\n\t\t\tUse ic4_get_last_error() to query error information.\n\n @remarks\n If the device is closed, all its resources are released:\n\t- If image acquisition is active, it is stopped.\n\t- If a data stream was set up, it is stopped.\n  - References to data stream-related objects are released, possibly destroying the sink and/or display.\n  - @ref properties objects become invalid.\n"]
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
    #[doc = " @brief Returns the property map for the currently opened video capture device.\n\n The property map returned from this function is the origin for all device feature manipulation operations.\n\n @param[in] pGrabber\t\t\tA grabber with an opened video capture device\n @param[out] ppPropertyMap\tPointer to a handle that receives the property map.\\n\n\t\t\t\t\t\t\t\tWhen the property map is longer required, call #ic4_propmap_unref().\n\n @return \\c true on success, otherwise \\c false.\\n\n\t\t\tUse ic4_get_last_error() to query error information.\n\n @remarks\n The property map handle and dependent objects retrieved from it will not keep the device in an opened state.\n If the device is closed, all future operations on the property map will result in an error.\n\n @see ic4_propmap_unref"]
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
    #[doc = " @brief Returns the property map for the driver of the currently opened video capture device.\n\n The property map returned from this function is the origin for driver-related feature operations.\n\n @param[in] pGrabber\t\t\tA grabber with an opened video capture device\n @param[out] ppPropertyMap\tPointer to a handle that receives the property map.\\n\n\t\t\t\t\t\t\t\tWhen the property map is longer required, call #ic4_propmap_unref().\n\n @return \\c true on success, otherwise \\c false.\\n\n\t\t\tUse ic4_get_last_error() to query error information.\n\n @remarks\n The property map handle and dependent objects retrieved from it will not keep the device in an opened state.\n If the device is closed, all future operations on the property map will result in an error.\n\n @see ic4_propmap_unref"]
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
    #[doc = " Establishes the data stream from the device.\n\n A data stream is required for image acquisition from the video capture device, and must include a sink (@ref sink), a @ref display, or both.\n\n @param[in] pGrabber\t\t\tA grabber object that has opened a video capture device\n @param[in] sink\t\t\t\tA sink (@ref sink) to receive the images\n @param[in] display\t\t\tA @ref display to display images\n @param[in] start_acquisition\tIf \\c true, immediately start image acquisition after the data stream was set up.\\n\n\t\t\t\t\t\t\t\tOtherwise, a call to #ic4_grabber_acquisition_start() is required to start image acquisition later.\n\n @return \\c true on success, otherwise \\c false.\\n\n\t\t\tUse ic4_get_last_error() to query error information.\n\n @pre\n A device was previously opened using #ic4_grabber_device_open() or one of its sibling functions.\n\n @note\n The grabber takes references to the passed sink and display, tying their lifetime to the grabber until the data stream is stopped.\n\n @see ic4_grabber_stream_stop\n @see ic4_grabber_acquisition_start"]
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
    #[doc = " Stops a data stream that was previously set up by a call to #ic4_grabber_stream_setup().\n\n @param[in] pGrabber\tA grabber with an established data stream\n\n @return \\c true on success, otherwise \\c false.\\n\n\t\t\tUse ic4_get_last_error() to query error information.\n\n @note\n This function releases the sink and display references that were passed to #ic4_grabber_stream_setup().\\n\n If there are no external references to the sink or display, the sind or display is destroyed.\n\n @see ic4_grabber_stream_setup"]
    pub fn stream_stop(&mut self) -> self::Result<()> {
        unsafe {
            ic4_sys::ic4_grabber_stream_stop(self.as_mut_ptr())
                .then_some(())
                .ok_or_else(|| self::get_last_error())?;
        }
        Ok(())
    }
    #[doc = " Checks whethere there is a data stream established from this grabber's video capture device.\n\n @param[in] pGrabber\tA grabber object\n\n @return\t\\c true, if a data stream was previously established by calling #ic4_grabber_stream_setup().\\n\n\t\t\tOtherwise, or if the data stream was stopped again, \\c false.\n\n @remarks\n This function neither clears nor sets the error status returned by ic4_get_last_error().\n\n @see ic4_grabber_stream_setup\n @see ic4_grabber_stream_stop"]
    pub fn is_streaming(&mut self) -> bool {
        unsafe { ic4_sys::ic4_grabber_is_streaming(self.as_mut_ptr()) }
    }
    #[doc = " Starts the acquisition of images from the video capture device.\n\n @param[in] pGrabber\tA grabber with an established data stream\n\n @return\t\\c true on success, otherwise \\c false.\\n\n\t\t\tUse ic4_get_last_error() to query error information.\n\n @pre A data stream has was previously established using #ic4_grabber_stream_setup().\n\n @note\n This operation is equivalent to executing the \\c AcquisitionStart command on the video capture device's property map.\n\n @see ic4_grabber_acquisition_stop\n @see ic4_grabber_stream_setup"]
    pub fn acquisition_start(&mut self) -> self::Result<()> {
        unsafe {
            ic4_sys::ic4_grabber_acquisition_start(self.as_mut_ptr())
                .then_some(())
                .ok_or_else(|| self::get_last_error())?;
        }
        Ok(())
    }
    #[doc = " Stops the acquisition of images from the video capture device.\n\n @param[in] pGrabber\tA grabber with acquisition active\n\n @return\t\\c true on success, otherwise \\c false.\\n\n\t\t\tUse ic4_get_last_error() to query error information.\n\n @note\n This operation is equivalent to executing the \\c AcquisitionStop command on the video capture device's property map.\n\n @see ic4_grabber_acquisition_start"]
    pub fn acquisition_stop(&mut self) -> self::Result<()> {
        unsafe {
            ic4_sys::ic4_grabber_acquisition_stop(self.as_mut_ptr())
                .then_some(())
                .ok_or_else(|| self::get_last_error())?;
        }
        Ok(())
    }
    #[doc = " Checks whether image acquisition is currently enabled for this grabber's video capture device.\n\n @param[in] pGrabber\tA grabber object\n\n @return\t\\c true, if image acquisition is currently active, otherwise \\c false.\n\n @remarks\n This function neither clears nor sets the error status returned by @ref ic4_get_last_error.\n\n @see ic4_grabber_acquisition_start\n @see ic4_grabber_acquisition_stop"]
    pub fn is_acquisition_active(&mut self) -> bool {
        unsafe { ic4_sys::ic4_grabber_is_acquisition_active(self.as_mut_ptr()) }
    }
    #[doc = " Returns a reference to the \\ref sink object that was passed to #ic4_grabber_stream_setup()\n when setting up the currently established data stream.\n\n @param[in] pGrabber\tA grabber with an established data stream\n @param[out] ppSink\tA pointer to a sink handle to receive the currently connected sink.\\n\n\t\t\t\t\t\tThis is a new reference. If it is no longer in use, it must be released using #ic4_sink_unref().\n\n @return\t\\c true on success, otherwise \\c false.\\n\n\t\t\tUse ic4_get_last_error() to query error information.\n\n @see ic4_grabber_stream_setup"]
    pub fn get_sink(&mut self) -> self::Result<Sink> {
        let mut sink = null_mut();
        unsafe {
            ic4_sys::ic4_grabber_get_sink(self.as_mut_ptr(), ptr_from_mut(&mut sink))
                .then_some(())
                .ok_or_else(|| self::get_last_error())?;
        }
        Ok(sink.into())
    }
    #[doc = " Returns a reference to the \\ref display object that was passed to #ic4_grabber_stream_setup()\n when setting up the currently established data stream.\n\n @param[in] pGrabber\t\tA grabber with an established data stream\n @param[out] ppDisplay\tA pointer to a sink handle to receive the currently connected display.\\n\n\t\t\t\t\t\t\tThis is a new reference. If it is no longer in use, it must be released using #ic4_display_unref().\n\n @return\t\\c true on success, otherwise \\c false.\\n\n\t\t\tUse ic4_get_last_error() to query error information.\n\n @see ic4_grabber_stream_setup"]
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

#[doc = " Function pointer for the device-lost handler\n\n @param[in] pGrabber\tPointer to the grabber whose device was lost\n @param[in] user_ptr\tUser data that was specified when calling #ic4_grabber_event_add_device_lost()"]
pub type GrabberDeviceLostHandler = ic4_sys::ic4_grabber_device_lost_handler;

#[doc = " Function pointer for cleanup of the device-lost user data\n\n @param[in] user_ptr\tUser data that was specified when calling #ic4_grabber_event_add_device_lost()"]
pub type GrabberDeviceLostDeleter = ic4_sys::ic4_grabber_device_lost_deleter;

impl Grabber {
    /// # Safety
    /// Unknown.
    ///
    #[doc = " Registers a function to be called when the currently opened video capture device was disconnected.\n\n @param[in] pGrabber\tThe grabber for which the callback is registered\n @param[in] handler\tThe function to be called when the device is lost\n @param[in] user_ptr\tUser data to be passed in calls to \\a handler.\n @param[in] deleter\tA function to be called when the handler was unregistered and the user_ptr will no longer be required.\\n\n\t\t\t\t\t\tThe deleter function can be used to release data associated with \\a user_ptr.\\n\n\t\t\t\t\t\tThe \\a deleter function will be called when the device-lost handler is unregistered,\n\t\t\t\t\t\tor the grabber object itself is destroyed.\n\n @note\n To unregister a device-lost handler, call #ic4_grabber_event_remove_device_lost().\n\n @return \\c true on success, otherwise \\c false.\\n\n\t\t\tUse ic4_get_last_error() to query error information."]
    pub unsafe fn event_add_device_lost(
        &mut self,
        handler: GrabberDeviceLostHandler,
        user_ptr: *mut c_void,
        deleter: GrabberDeviceLostDeleter,
    ) -> self::Result<()> {
        ic4_sys::ic4_grabber_event_add_device_lost(
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
    #[doc = " Unregisters a device-lost handler that was previously registered using #ic4_grabber_event_add_device_lost().\n\n @param[in] pGrabber\tThe grabber on which the callback is currently registered\n @param[in] handler\tPointer to the function to be unregistered\n @param[in] user_ptr\tUser data that the function was previously registered with\n\n @note\n The pair of \\a handler and \\a user_ptr has to be an exact match to the parameters used in the call to #ic4_grabber_event_add_device_lost().\n\n @return \\c true on success, otherwise \\c false.\\n\n\t\t\tUse ic4_get_last_error() to query error information."]
    pub unsafe fn event_remove_device_lost(
        &mut self,
        handler: GrabberDeviceLostHandler,
        user_ptr: *mut c_void,
    ) -> self::Result<()> {
        ic4_sys::ic4_grabber_event_remove_device_lost(self.as_mut_ptr(), handler, user_ptr)
            .then_some(())
            .ok_or_else(|| self::get_last_error())
    }
}

/*
 * Stream
 */

#[doc = " @brief Contains statistics counters that can be used to analyze the stream behavior and identify possible bottlenecks.\n\n This structure is filled by calling #ic4_grabber_get_stream_stats()."]
pub type StreamStats = ic4_sys::IC4_STREAM_STATS;

impl DefaultExt for StreamStats {
    fn default_ext() -> Self {
        StreamStats {
            device_delivered: 0,
            device_transmission_error: 0,
            device_underrun: 0,
            transform_delivered: 0,
            transform_underrun: 0,
            sink_delivered: 0,
            sink_underrun: 0,
            sink_ignored: 0,
        }
    }
}

impl Grabber {
    #[doc = " @brief Query statistics counters from the currently running or previously stopped data stream.\n\n @param[in] pGrabber\tA grabber object\n @param[out] stats\tA pointer to a structure to receive the stream statistics\n\n @return \\c true on success, otherwise \\c false.\\n\n\t\t\tUse ic4_get_last_error() to query error information.\n\n @pre This operation is only valid after a data stream was established once."]
    pub fn get_stream_stats(&mut self) -> self::Result<StreamStats> {
        let mut stream_stats: StreamStats = Default::default();
        unsafe {
            ic4_sys::ic4_grabber_get_stream_stats(
                self.as_mut_ptr(),
                ptr_from_mut(&mut stream_stats),
            )
            .then_some(())
            .ok_or_else(|| self::get_last_error())?;
        }
        Ok(stream_stats)
    }
}

#[doc = " @brief Callback function called to allocate memory during the call of #ic4_grabber_device_save_state.\n\n @param[in]\tsize\tSize of the memory buffer to be allocated.\n\n @return\t\tThe pointer to the allocated memory buffer, or @c NULL if the allocation was not possible.\n\n @note\n If this function returns @c NULL, the call to #ic4_grabber_device_save_state will fail.\n\n @see ic4_grabber_device_save_state"]
pub type DeviceStateAllocator = ic4_sys::ic4_device_state_allocator;

impl Grabber {
    #[doc = " @brief Saves the currently opened video capture device and all its settings into a memory buffer.\n\n @param[in]\tpGrabber\tA grabber object with an opened device\n @param[in]\talloc\t\tPointer to a function that allocates the memory buffer.\\n\n\t\t\t\t\t\t\tFor example, @c malloc can be passed here.\n @param[out]\tppData\t\tPointer to a pointer to receive the newly-allocated memory buffer containing the device state.\\n\n\t\t\t\t\t\t\tThe caller is responsible for releasing the memory, using a function that can free memory returned by @c alloc.\n @param[out]\tdata_size\tPointer to size_t to receive the size of the memory buffer allocated by the call\n\n @return \\c true on success, otherwise \\c false.\\n\n\t\t\tUse ic4_get_last_error() to query error information.\n\n @note\n To restore the device state at a later time, use #ic4_grabber_device_open_from_state.\\n\n In addition to serializing the device's properties (like #ic4_propmap_serialize_to_memory() would), this function also saves the\n currently opened video capture device so that it can be re-opened at a later time with all settings restored.\n\n @see ic4_grabber_load_device_state\n @see ic4_grabber_save_device_state_to_file"]
    pub fn device_save_state(&mut self, alloc: DeviceStateAllocator) -> self::Result<&mut [u8]> {
        let mut vaild_value_set = null_mut();
        let mut vaild_value_length = 0;
        unsafe {
            ic4_sys::ic4_grabber_device_save_state(
                self.as_mut_ptr(),
                alloc,
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
    #[doc = " @brief Saves the currently opened video capture device and all its settings into a file.\n\n @param[in]\tpGrabber\tA grabber object with an opened device\n @param[in]\tfile_path\tPath to a file that the device state is written to.\n\n @return \\c true on success, otherwise \\c false.\\n\n\t\t\tUse ic4_get_last_error() to query error information.\n\n @note\n To restore the device state at a later time, use #ic4_grabber_device_open_from_state_file().\\n\n In addition to serializing the device's properties (like #ic4_propmap_serialize_to_file() would), this function also saves the\n currently opened video capture device so that it can be re-opened at a later time with all settings restored.\n\n @see ic4_grabber_device_open_from_state_file\n @see ic4_grabber_device_save_state"]
    pub fn device_save_state_to_file(&mut self, path: &CStr) -> self::Result<()> {
        unsafe {
            ic4_sys::ic4_grabber_device_save_state_to_file(self.as_mut_ptr(), path.as_ptr())
                .then_some(())
                .ok_or_else(|| self::get_last_error())?;
        }
        Ok(())
    }
    #[doc = " @brief Restores the opened device and its settings from a memory buffer containing data that was previously written by #ic4_grabber_device_save_state.\n\n @param[in]\tpGrabber\tA grabber object without an opened device\n @param[in]\tdata\t\tPointer to a memory buffer containing data that was written by #ic4_grabber_device_save_state\n @param[in]\tdata_size\tSize of the memory buffer pointed to by @c pData\n\n @return \\c true on success, otherwise \\c false.\\n\n\t\t\tUse ic4_get_last_error() to query error information.\n\n @note\n If the memory buffer contains settings for properties that could not be written, the function fails and the error value is set to #IC4_ERROR_INCOMPLETE.\n\n @see ic4_grabber_device_save_state"]
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
    #[doc = " @brief Restores the opened device and its settings from a file that was previously written by #ic4_grabber_device_save_state_to_file().\n\n @param[in]\tpGrabber\tA grabber object without an opened device\n @param[in]\tfile_path\tPath to a file containing device state information\n\n @return \\c true on success, otherwise \\c false.\\n\n\t\t\tUse ic4_get_last_error() to query error information.\n\n @note\n If the file contains settings for properties that could not be written, the function fails and the error value is set to #IC4_ERROR_INCOMPLETE.\n\n @see ic4_grabber_device_save_state_to_file\n @see ic4_grabber_device_open_from_state"]
    pub fn device_open_from_state_file(&mut self, path: &CStr) -> self::Result<()> {
        unsafe {
            ic4_sys::ic4_grabber_device_open_from_state_file(self.as_mut_ptr(), path.as_ptr())
                .then_some(())
                .ok_or_else(|| self::get_last_error())?;
        }
        Ok(())
    }
}
