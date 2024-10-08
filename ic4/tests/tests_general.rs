use std::{ffi::CStr, ptr::null_mut};

use ic4::*;

#[test]
fn enumerate_device() -> ic4::Result<()> {
    // ic4::exit_library();
    ic4::init_library(&InitConfig::default_ext())?;
    let mut dev_enum = DeviceEnum::create()?;
    dev_enum.update_device_list()?;
    dev_enum.update_interface_list()?;
    let device_count = dev_enum.get_device_count();
    eprintln!("Device Count: {device_count}");
    #[cfg(not(feature = "test-ensure-existing-device"))]
    if device_count == 0 {
        return Ok(());
    }
    for device_index in 0..device_count {
        let device = dev_enum.get_device_info(device_index as usize)?;
        let device_name = device.get_model_name();
        eprintln!("Model Name: {}", device_name.to_string_lossy());
    }
    drop(dev_enum);
    ic4::exit_library();
    Ok(())
}

#[test]
fn get_image() -> ic4::Result<()> {
    // ic4::exit_library();
    ic4::init_library(&InitConfig::default_ext())?;
    let mut grabber = Grabber::create()?;
    {
        let mut dev_enum = DeviceEnum::create()?;
        dev_enum.update_device_list()?;
        dev_enum.update_interface_list()?;
        #[cfg(not(feature = "test-ensure-existing-device"))]
        let device_count = dev_enum.get_device_count();
        #[cfg(not(feature = "test-ensure-existing-device"))]
        if device_count == 0 {
            return Ok(());
        }
        let mut dev_info = dev_enum.get_device_info(0)?;
        grabber.device_open(&mut dev_info)?;
    }

    let mut prop_map = grabber.device_get_property_map()?;
    prop_map.set_value_i64(PropertyName::Width, 640)?;
    prop_map.set_value_i64(PropertyName::Height, 480)?;

    eprintln!("Device Open!");

    let mut snap_sink = SnapSink::create(&SnapSinkConfig {
        strategy: SnapSinkAllocationStrategy::IC4_SNAPSINK_ALLOCATION_STRATEGY_DEFAULT,
        num_buffers_alloc_on_connect: 1,
        num_buffers_allocation_threshold: 1,
        num_buffers_free_threshold: 1,
        num_buffers_max: 300,
        pixel_formats: std::ptr::from_ref(&PixelFormat::IC4_PIXEL_FORMAT_BGR8),
        num_pixel_formats: 0,
        allocator: AllocatorCallbacks {
            release: None,
            allocate_buffer: None,
            free_buffer: None,
        },
        allocator_context: std::ptr::null_mut(),
    })?;
    let mut display = ic4::Display::create(
        DisplayType::IC4_DISPLAY_DEFAULT,
        WindowHandle::from(ic4_sys::IC4_WINDOW_HANDLE(null_mut())),
    )?;
    grabber.stream_setup(snap_sink.as_mut(), &mut display, false)?;

    eprintln!("SnapSink setup!");

    let mut image = snap_sink.snap_single(10000)?;

    eprintln!("SnapSink get buffer!");

    image.save_as_tiff(
        unsafe { CStr::from_bytes_with_nul_unchecked(&b"./test/result.tiff"[..]) },
        &ImageBufferSaveOptionsTIFF::default_ext(),
    )?;
    ic4::exit_library();
    Ok(())
}
