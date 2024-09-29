#![allow(clippy::redundant_closure)]

use crate::*;

#[doc = " @brief Shows a dialog that allows the user to select a video capture device. The device is opened in the passed @ref IC4_GRABBER object.\n\n @param[in] hParent\tA parent window for the dialog\n @param[in] pGrabber\tA grabber object in which the new device is to be opened\n\n @return @c true, if a device was selected and opened succesfully, otherwise @c false."]
pub fn grabber_select_device(parent: WindowHandle, grabber: &mut Grabber) -> self::Result<()> {
    unsafe {
        ic4_sys::ic4_gui_grabber_select_device(parent.inner, grabber.as_mut_ptr())
            .then_some(())
            .ok_or_else(|| self::get_last_error())?;
    }
    Ok(())
}

#[doc = " @brief Shows a dialog that allows the user to select a video capture device. A @ref IC4_DEVICE_INFO representing the selected device is returned\n in an output parameter.\n\n @param[in] hParent\tA parent window for the dialog\n @param[out] ppInfo\tA pointer to a handle to receive the device information object.\\n\n\t\t\t\t\t\tWhen the device information object is no longer required, release the object reference using #ic4_devinfo_unref().\n\n @return @c true, if a device was selected, otherwise @c false."]
pub fn select_device(parent: WindowHandle) -> self::Result<DeviceInfo> {
    let mut result_ptr = null_mut();
    unsafe {
        ic4_sys::ic4_gui_select_device(parent.inner, ptr_from_mut(&mut result_ptr))
            .then_some(())
            .ok_or_else(|| self::get_last_error())?;
    }
    Ok(result_ptr.into())
}

#[doc = " @brief Defines a set of flags to customize the behavior of the dialogs displayed by the@ref ic4_gui_grabber_show_device_properties()\n and @ref ic4_gui_show_property_map() functions."]
pub type PropertyDialogFlags = ic4_sys::IC4_PROPERTY_DIALOG_FLAGS;

#[doc = " @brief A structure containing options customizing the appearance and behavior of the property dialog displayed by\n the @ref ic4_gui_grabber_show_device_properties() and @ref ic4_gui_show_property_map() functions."]
pub type PropertyDialogOptions = ic4_sys::IC4_PROPERTY_DIALOG_OPTIONS;

#[doc = " @brief Shows a dialog box allowing the user to configure the properties of the video capture opened in the passed @ref IC4_GRABBER.\n\n @param[in] hParent\tA parent window for the dialog\n @param pGrabber\t\tA grabber object whose video capture device is to be configured\n @param[in] options\tAn options structure to customize the dialog's behavior\n\n @return @c true, if the user closed the dialog using the @a OK button, otherwise @c false."]
pub fn grabber_show_device_properties(
    parent: WindowHandle,
    grabber: &mut Grabber,
    options: PropertyDialogOptions,
) -> self::Result<()> {
    unsafe {
        ic4_sys::ic4_gui_grabber_show_device_properties(
            parent.inner,
            grabber.as_mut_ptr(),
            ptr_from_ref(&options),
        )
        .then_some(())
        .ok_or_else(|| self::get_last_error())?;
    }
    Ok(())
}

#[doc = " @brief Shows a dialog box allowing the user to configure the properties of a @ref IC4_PROPERTY_MAP.\n\n @param[in] hParent\t\tA parent window for the dialog\n @param[in] pPropertyMap\tA property map to configure\n @param[in] options\t\tAn options structure to customize the dialog's behavior\n\n @return @c true, if the user closed the dialog using the @a OK button, otherwise @c false."]
pub fn show_property_map(
    parent: WindowHandle,
    property_map: &mut PropertyMap,
    options: PropertyDialogOptions,
) -> self::Result<()> {
    unsafe {
        ic4_sys::ic4_gui_show_property_map(
            parent.inner,
            property_map.as_mut_ptr(),
            ptr_from_ref(&options),
        )
        .then_some(())
        .ok_or_else(|| self::get_last_error())?;
    }
    Ok(())
}
