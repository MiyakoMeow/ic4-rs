#[test]
fn without_setup() {
    // ic4::init_library().unwrap();
    let Err(ic4::Error{..}) = ic4::DeviceEnum::create() else {
        panic!("Operation Success without init?!");
    };
}
