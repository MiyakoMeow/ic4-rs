# A Rust bindings for **IC Imaging Control 4 SDK** from [The Imaging Source](https://www.theimagingsource.com/)
For Windows platform, bindings from C headers.

## Build & Run Requirement
See [ic4-sys](https://crates.io/crates/ic4-sys).

## Progress
- [x] Raw bindings.
- [x] Safe Rust bindings.

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
