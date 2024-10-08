use core::panic;
use std::ffi::{c_char, CStr, CString};

use ic4_sys::*;

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
            eprintln!("No devices found.\n");
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
fn grabbing_image() {
    use ic4_sys::*;
    unsafe {
        ic4_init_library(std::ptr::null_mut());

        // Create a grabber object
        let mut grabber = std::ptr::null_mut();
        if !ic4_grabber_create(std::ptr::from_mut(&mut grabber)) {
            print_last_error();
            panic!("Failed when executing ic4_grabber_create");
        }

        // Find the first available video capture device
        let mut enumerator = std::ptr::null_mut();
        if !ic4_devenum_create(std::ptr::from_mut(&mut enumerator)) {
            print_last_error();
            panic!("Failed when executing ic4_devenum_create");
        }
        ic4_devenum_update_device_list(enumerator);

        let count = ic4_devenum_get_device_count(enumerator);
        if count == 0 {
            eprintln!("No devices found.\n");
            return;
        }

        let mut info = std::ptr::null_mut();
        if !ic4_devenum_get_devinfo(enumerator, 0, std::ptr::from_mut(&mut info)) {
            print_last_error();
            panic!("Failed when executing ic4_devenum_get_devinfo");
        }
        ic4_devenum_unref(enumerator);

        // Open it
        if !ic4_grabber_device_open(grabber, info) {
            ic4_devinfo_unref(info);
            print_last_error();
            panic!("Failed to open device!");
        }
        ic4_devinfo_unref(info);

        let mut map = std::ptr::null_mut();
        if !ic4_grabber_device_get_property_map(grabber, std::ptr::from_mut(&mut map)) {
            print_last_error();
            panic!("Failed when executing ic4_grabber_device_get_property_map");
        }

        // Set the resolution to 640x480
        if !ic4_propmap_set_value_int64(map, b"Width\0".as_ptr() as *const c_char, 640) {
            print_last_error();
            panic!("Failed when executing ic4_propmap_set_value_int64");
        }
        if !ic4_propmap_set_value_int64(map, b"Height\0".as_ptr() as *const c_char, 480) {
            print_last_error();
            panic!("Failed when executing ic4_propmap_set_value_int64");
        }

        ic4_propmap_unref(map);

        let mut sink = std::ptr::null_mut();
        if !ic4_snapsink_create(std::ptr::from_mut(&mut sink), std::ptr::null_mut()) {
            print_last_error();
            panic!("Failed when executing ic4_snapsink_create");
        }

        if !ic4_grabber_stream_setup(grabber, sink, std::ptr::null_mut(), true) {
            print_last_error();
            panic!("Failed to open stream!");
        }

        let mut buffer = std::ptr::null_mut();
        if !ic4_snapsink_snap_single(sink, std::ptr::from_mut(&mut buffer), 1000) {
            print_last_error();
            panic!("Failed to get image buffer\n");
        }

        if !ic4_imagebuffer_save_as_bmp(
            buffer,
            CStr::from_bytes_with_nul_unchecked(b"test.bmp").as_ptr(),
            std::ptr::from_ref(&IC4_IMAGEBUFFER_SAVE_OPTIONS_BMP {
                store_bayer_raw_data_as_monochrome: 0,
            }),
        ) {
            print_last_error();
            panic!("Failed to save image buffer\n");
        }

        ic4_grabber_stream_stop(grabber);
    }
}

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

fn print_last_error() {
    unsafe {
        let mut error = ic4_sys::IC4_ERROR::IC4_ERROR_UNKNOWN;
        let mut message_length = 10 * 1024;
        let mut message_vec: Vec<_> = std::iter::repeat(0u8).take(message_length).collect();
        if !ic4_sys::ic4_get_last_error(
            std::ptr::from_mut(&mut error),
            message_vec.as_mut_ptr() as *mut c_char,
            std::ptr::from_mut(&mut message_length),
        ) {
            panic!("Failed to get error!")
        }
        message_vec.resize(message_length - 1, 0); // Remove tailing '\0'
        println!(
            "{:?} {}",
            error,
            CString::from_vec_with_nul_unchecked(message_vec).to_string_lossy()
        );
    }
}
