use std::ffi::CStr;

use ic4_sys::*;

/*
 * Tools func
 */

fn print_device_info(device_info: *const IC4_DEVICE_INFO) {
    unsafe {
        println!(
            "Model: {} ",
            CStr::from_ptr(ic4_devinfo_get_model_name(device_info)).to_string_lossy()
        );
        println!(
            "Serial: {}",
            CStr::from_ptr(ic4_devinfo_get_serial(device_info)).to_string_lossy()
        );
        println!(
            "Version: {}",
            CStr::from_ptr(ic4_devinfo_get_version(device_info)).to_string_lossy()
        );
    }
}

/*
 * Tests
 */

#[test]
fn enumerate_devinfo() {
    unsafe {
        ic4_init_library(std::ptr::null());
        println!("Enumerating all attached video capture devices in a single list...");

        let mut enumerator = std::ptr::null_mut();
        if !ic4_devenum_create(std::ptr::from_mut(&mut enumerator)) {
            panic!("Failed to create device enumerator");
        }

        if !ic4_devenum_update_device_list(enumerator) {
            panic!("Failed to update device list");
        }

        let count = ic4_devenum_get_device_count(enumerator);
        if count == 0 {
            println!("No devices found\n");
            return;
        }

        println!("Found {} devices: ", count);
        for i in 0..count {
            let mut info = std::ptr::null_mut();
            if !ic4_devenum_get_devinfo(enumerator, i, std::ptr::from_mut(&mut info)) {
                println!("Failed to query device info for index {}", i);
                continue;
            }
            print_device_info(info);
            ic4_devinfo_unref(info);
        }
    }
}

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
        let count = ic4_devenum_get_device_count(enumerator);

        #[cfg(not(feature = "test-ensure-existing-device"))]
        if count == 0 {
            return;
        }

        if !ic4_devenum_get_devinfo(enumerator, 0, std::ptr::from_mut(&mut info)) {
            panic!("Failed to get devinfo!")
        }
        ic4_devenum_unref(enumerator);

        // Open it
        if !ic4_grabber_device_open(grabber, info) {
            panic!("Failed to open device")
        }
        ic4_devinfo_unref(info);
        ic4_exit_library();
    }
}
