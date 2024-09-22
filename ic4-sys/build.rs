const IC4_NOT_FOUND_TIPS: &str = "IC4 is not found in system path! Please download and install ic4 sdk in your computer! Link: https://www.theimagingsource.com/zh-hans-cn/support/download/icimagingcontrol4win-1.1.0.2833/";

use std::env;
#[allow(unused)]
use std::io::Write;
use std::path::{Path, PathBuf};

use once_cell::sync::Lazy;

#[allow(unused)]
static MANIFEST_DIR: &str = env!("CARGO_MANIFEST_DIR");
#[allow(unused)]
static OUT_DIR: Lazy<PathBuf> = Lazy::new(|| {
    let path = env::var_os("OUT_DIR").expect("The OUT_DIR environment variable is not set");
    PathBuf::from(path)
});

/// 以下内容是给src/bindings目录下、由bindgen生成的rs文件的，不是给此文件（build.rs）的。
#[cfg(feature = "buildtime-bindgen")]
const BINDGEN_RS_FILE_HEADER: &str = "/*
 * 注意：不要修改此文件。
 * 此文件由bindgen生成，转换自特定的一个或多个C++头文件。详见build.rs文件。
 */
#![allow(unused)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
";

fn main() {
    let path_ori = std::env::var_os("Path").unwrap();
    let paths = std::env::split_paths(&path_ori);
    let ic4_root = paths
        .into_iter()
        .find(|path| path.join("ic4core.dll").is_file())
        .expect(IC4_NOT_FOUND_TIPS);
    #[cfg(feature = "buildtime-bindgen")]
    ic4_bindgen(&ic4_root);
    ic4_link(&ic4_root);
}

#[cfg(feature = "buildtime-bindgen")]
fn ic4_bindgen(ic4_root: &Path) {
    // 生成头文件
    let ic4_include_dir = ic4_root.join("..").join("include");
    let current_dir = std::env::current_dir().unwrap();
    let ic4_header_paths = [current_dir.join("wrapper.hpp")].into_iter();

    // The bindgen::Builder is the main entry point
    // to bindgen, and lets you build up options for
    // the resulting bindings.
    let bindings = ic4_header_paths.fold(bindgen::Builder::default(), |builder, path| {
        let path_str = path.to_string_lossy();
        // println!("cargo:rerun-if-changed={}", path_str.as_ref()); // Unneeded
        // The input header we would like to generate
        // bindings for.
        builder.header(path_str.as_ref())
    });
    #[rustfmt::skip]
    let bindings = bindings
        .size_t_is_usize(true)
        .default_macro_constant_type(bindgen::MacroTypeVariation::Signed)
        .fit_macro_constants(true)
        // .disable_name_namespacing()
        // .enable_cxx_namespaces()
        .respect_cxx_access_specs(true)
        .default_enum_style(bindgen::EnumVariation::Rust {
            non_exhaustive: true,
        })
        ;
    let bindings = bindings
        .clang_args(["-I", ic4_include_dir.to_string_lossy().as_ref()])
        .clang_arg("-std=c++14");
    let bindings = bindings
        // Tell cargo to invalidate the built crate whenever any of the
        // included header files changed.
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()));
    let bindings = bindings
        // Finish the builder and generate the bindings.
        .generate()
        // Unwrap the Result and panic on failure.
        .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let target_rs_file_path = Path::new(MANIFEST_DIR)
                .join("src")
                .join("bindings")
                .join("ic4.rs");
    let mut writer = std::io::BufWriter::new(std::fs::File::create(target_rs_file_path).unwrap());
    writer.write_all(BINDGEN_RS_FILE_HEADER.bytes().collect::<Vec<u8>>().as_slice()).unwrap();
    bindings
        .write(Box::new(&mut writer))
        .expect("Couldn't write bindings!");

}

fn ic4_link(ic4_root: &Path) {
    // 实际链接
    let ic4_lib_dir = ic4_root.join("..").join("lib");

    // Tell cargo to look for shared libraries in the specified directory
    println!("cargo:rustc-link-search={}", ic4_lib_dir.to_string_lossy());

    for lib_name in ["ic4core", "ic4gui"] {
        println!(
            "cargo:rerun-if-changed={}",
            ic4_lib_dir
                .join(format!("{}.lib", lib_name))
                .to_string_lossy()
        );
        // Tell cargo to tell rustc to link the system bzip2
        // shared library.
        println!("cargo:rustc-link-lib={}", lib_name);
    }
}

#[allow(unused)]
fn find_all_files(root: &Path) -> Result<Vec<PathBuf>, std::io::Error> {
    let mut iter = vec![];
    for child in root.read_dir()? {
        let child_path = child?.path();
        if child_path.is_file() {
            iter.extend([child_path]);
        } else if child_path.is_dir() {
            iter.extend(find_all_files(child_path.as_path())?.into_iter());
        }
    }
    Ok(iter)
}
