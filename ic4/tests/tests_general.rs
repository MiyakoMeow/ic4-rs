use std::{ffi::CStr, ptr::null_mut, thread::sleep, time::Duration};

use ic4::*;

#[test]
fn enumerate_device() -> ic4::Result<()> {
    ic4::init_library(&InitConfig::default_ext())?;
    let mut dev_enum = DeviceEnum::create()?;
    let device_count = dev_enum.get_device_count();
    #[cfg(not(feature = "test-ensure-existing-device"))]
    if device_count == 0 {
        return Ok(());
    }
    for device_index in 0..device_count {
        let device = dev_enum.get_device_info(device_index as usize)?;
        let device_name = device.get_model_name();
        println!("Model Name: {}", device_name.to_string_lossy());
    }
    ic4::exit_library();
    Ok(())
}

#[test]
fn get_image() -> ic4::Result<()> {
    ic4::init_library(&InitConfig::default_ext())?;
    let mut dev_enum = DeviceEnum::create()?;
    let device_count = dev_enum.get_device_count();
    #[cfg(not(feature = "test-ensure-existing-device"))]
    if device_count == 0 {
        return Ok(());
    }
    let mut grabber = Grabber::create()?;
    let mut dev_info = dev_enum.get_device_info(0)?;
    grabber.device_open(&mut dev_info)?;

    let mut snap_sink = SnapSink::create(&SnapSinkConfig {
        strategy: SnapSinkAllocationStrategy::IC4_SNAPSINK_ALLOCATION_STRATEGY_DEFAULT,
        num_buffers_alloc_on_connect: 100,
        num_buffers_allocation_threshold: 100,
        num_buffers_free_threshold: 100,
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
    sleep(Duration::from_millis(1000));
    let mut image = snap_sink.snap_single(100)?;
    image.save_as_tiff(
        unsafe { CStr::from_bytes_with_nul_unchecked(&b"./test/result.tiff"[..]) },
        &ImageBufferSaveOptionsTIFF::default_ext(),
    )?;
    ic4::exit_library();
    Ok(())
}
