#![allow(clippy::redundant_closure)]

use crate::*;

pub fn grabber_select_device(parent: WindowHandle, grabber: &mut Grabber) -> self::Result<()> {
    unsafe {
        ic4_sys::ic4_gui_grabber_select_device(parent.inner, grabber.as_mut_ptr())
            .then_some(())
            .ok_or_else(|| self::get_last_error())?;
    }
    Ok(())
}

pub fn select_device(parent: WindowHandle) -> self::Result<DeviceInfo> {
    let mut result_ptr = null_mut();
    unsafe {
        ic4_sys::ic4_gui_select_device(parent.inner, ptr_from_mut(&mut result_ptr))
            .then_some(())
            .ok_or_else(|| self::get_last_error())?;
    }
    Ok(result_ptr.into())
}

pub type PropertyDialogFlags = ic4_sys::IC4_PROPERTY_DIALOG_FLAGS;

pub type PropertyDialogOptions = ic4_sys::IC4_PROPERTY_DIALOG_OPTIONS;

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
