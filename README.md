# A Rust bindings for [IC Imaging Control 4 SDK](https://www.theimagingsource.com/zh-hans-cn/support/download/icimagingcontrol4win-1.1.0.2833/) from [The Imaging Source](https://www.theimagingsource.com/)

## Build Requirement
[IC Imaging Control 4 SDK](https://www.theimagingsource.com/zh-hans-cn/support/download/icimagingcontrol4win-1.1.0.2833/) is required before using this crate.

Download, install, and check the `bin` directory of the SDK is in the `PATH` variable.

## Progress
Now there is all raw bindings from C. Cpp bindings can not be finished by rust-bindgen.

For contribers, modify `wrapper.hpp` and use `buildtime-bindgen` feature to try fixing this problem.
