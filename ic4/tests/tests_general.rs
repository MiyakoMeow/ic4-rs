use std::{ffi::CStr, ptr::null_mut, thread::sleep, time::Duration};

use ic4::*;

#[test]
fn without_setup() {
    // ic4::init_library(&InitConfig::default()).unwrap();
    let Err(ic4::Error { .. }) = ic4::DeviceEnum::create() else {
        panic!("Operation Success without init?!");
    };
}

#[test]
fn enumerate_device() {
    ic4::init_library(&InitConfig::default_ext()).unwrap();
    let mut dev_enum = DeviceEnum::create().unwrap();
    let device_count = dev_enum.get_device_count();
    #[cfg(not(feature = "test-ensure-existing-device"))]
    if device_count == 0 {
        return;
    }
    for device_index in 0..device_count {
        let device = dev_enum.get_device_info(device_index as usize).unwrap();
        let device_name = device.get_model_name();
        println!("Model Name: {}", device_name.to_string_lossy());
    }
}

#[test]
fn get_image() {
    ic4::init_library(&InitConfig::default_ext()).unwrap();
    let mut dev_enum = DeviceEnum::create().unwrap();
    let device_count = dev_enum.get_device_count();
    #[cfg(not(feature = "test-ensure-existing-device"))]
    if device_count == 0 {
        return;
    }
    let mut grabber = Grabber::create().unwrap();
    let mut dev_info = dev_enum.get_device_info(0).unwrap();
    grabber.device_open(&mut dev_info).unwrap();

    let mut snap_sink = SnapSink::create(&SnapSinkConfig {
        ..DefaultExt::default_ext()
    })
    .unwrap();
    let mut display = ic4::Display::create(
        DisplayType::IC4_DISPLAY_DEFAULT,
        WindowHandle::from(ic4_sys::IC4_WINDOW_HANDLE(null_mut())),
    )
    .unwrap();
    grabber
        .stream_setup(snap_sink.as_mut(), &mut display, false)
        .unwrap();
    sleep(Duration::from_millis(1000));
    let mut image = snap_sink.snap_single(100).unwrap();
    image
        .save_as_tiff(
            unsafe { CStr::from_bytes_with_nul_unchecked(&b"./test/result.tiff"[..]) },
            &ImageBufferSaveOptionsTIFF::default_ext(),
        )
        .unwrap();
}
