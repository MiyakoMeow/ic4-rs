use std::{
    ffi::{CStr, CString},
    fmt::Display,
};

pub trait ToCStr {
    fn to_cstr(&self) -> &CStr;
}

pub trait ToCString {
    fn to_cstring(&self) -> CString;
}

impl ToCStr for CStr {
    fn to_cstr(&self) -> &CStr {
        self
    }
}

impl ToCStr for CString {
    fn to_cstr(&self) -> &CStr {
        self.as_c_str()
    }
}

/*
 * Display
 */

impl Display for dyn ToCStr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.to_cstr().to_string_lossy().as_ref())
    }
}

impl Display for dyn ToCString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.to_cstring().to_string_lossy().as_ref())
    }
}
