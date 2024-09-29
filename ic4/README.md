# A Rust bindings for **IC Imaging Control 4 SDK** from [The Imaging Source](https://www.theimagingsource.com/)
For Windows platform, bindings from C headers.

## Build & Run Requirement
See [ic4-sys](https://crates.io/crates/ic4-sys).

## Notice
1. Call `ic4::init_library` before calling any other function of this crate.

2. If possible, **DO NOT** use value in this crate generated from `Default::default`, as the value is from non-zerolized memory spaces. <br>
Use `ic4::DefaultExt::default_ext` instead.

## Document Notice
All of api doc is copied from ic4-sys. Just for reference.
