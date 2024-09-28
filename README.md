# A Rust bindings for [IC Imaging Control 4 SDK](https://www.theimagingsource.com/zh-hans-cn/support/download/icimagingcontrol4win-1.1.0.2833/) from [The Imaging Source](https://www.theimagingsource.com/)
For Windows platform, bindings from C headers.

## Build & Run Requirement
[IC Imaging Control 4 SDK](https://www.theimagingsource.com/zh-hans-cn/support/download/icimagingcontrol4win-1.1.0.2833/) is required before using this crate.

Download, install, and check the `bin` directory of the SDK is in the `PATH` variable.

## Notice
1. Call `ic4::init_library` or `ic4_sys::ic4_init_library` before calling any other function of this crate.

2. If possible, **DO NOT** use value in this crate generated from `Default::default`, as the value is from non-zerolized memory spaces. <br>
Use `ic4::DefaultExt::default_ext` instead.

## Progress
[x] Safe Rust bindings are all done.

Tests:
- [x] CString buffer to C functions. 
- [ ] ...

Tests with actual device:
- [ ] Open GigE device.
- [ ] Open stream.
- [ ] Save image to file.
- [ ] Get Image from stream.
- [ ] ...

## Development: Test
Use command below:
```commandline
cargo test -- --test-threads=1
```

With `test-ensure-existing-device` feature:
```commandline
cargo test --features test-ensure-existing-device -- --test-threads=1
```
