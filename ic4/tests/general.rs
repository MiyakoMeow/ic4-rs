use ic4::*;

use ic4::ToCStr;

#[test]
fn without_setup() {
    // ic4::init_library(&InitConfig::default()).unwrap();
    let Err(ic4::Error { .. }) = ic4::DeviceEnum::create() else {
        panic!("Operation Success without init?!");
    };
}

#[test]
fn enumerate_device() {
    ic4::init_library(&InitConfig::default()).unwrap();
    let mut dev_enum = DeviceEnum::create().unwrap();
    let device_count = dev_enum.get_device_count();
    for device_index in 0..device_count {
        let device = dev_enum.get_device_info(device_index as usize).unwrap();
        let _device_name = device.get_model_name();
        // println!("Model Name: {}", *device_name);
    }
}
