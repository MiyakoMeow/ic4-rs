# A Rust bindings for [IC Imaging Control 4 SDK](https://www.theimagingsource.com/zh-hans-cn/support/download/icimagingcontrol4win-1.1.0.2833/) from [The Imaging Source](https://www.theimagingsource.com/)
For Windows platform, bindings from C headers.

## Build & Run Requirement
[IC Imaging Control 4 SDK](https://www.theimagingsource.com/zh-hans-cn/support/download/icimagingcontrol4win-1.1.0.2833/) is required before using this crate.

Download, install, and check the `bin` directory of the SDK is in the `PATH` variable.

## Notice
Call `ic4::init_library` or `ic4_sys::ic4_init_library` before calling any other function of this crate.

## Progress
Safe Rust bindings are done, but there is still no tests.
