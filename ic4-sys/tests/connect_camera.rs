use ic4_sys::*;

#[test]
fn connect_camera() {
    unsafe {
        ic4_init_library(std::ptr::null());

        // Create a grabber object
        let mut grabber = std::ptr::null_mut();
        if !ic4_grabber_create(std::ptr::from_mut(&mut grabber)) {
            panic!("Failed to create grabber")
        }

        // Find the first available video capture device
        let mut enumerator = std::ptr::null_mut();
        if !ic4_devenum_create(std::ptr::from_mut(&mut enumerator)) {
            panic!("Failed to create devenum")
        }

        if !ic4_devenum_update_device_list(enumerator) {
            panic!("Failed to update devenum")
        }
        let mut info = std::ptr::null_mut();
        if !ic4_devenum_get_devinfo(enumerator, 0, std::ptr::from_mut(&mut info)) {
            return;
        }
        ic4_devenum_unref(enumerator);

        // Open it
        if !ic4_grabber_device_open(grabber, info) {
            panic!("Failed to open device")
        }
        ic4_devinfo_unref(info);
    }
}
