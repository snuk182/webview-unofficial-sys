use cmake;
use std::env;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};

fn main() {
    let target = env::var("TARGET").unwrap();

    let mut build = cmake::Config::new("webview-official");
    build
        .define("CMAKE_BUILD_TYPE", "Release")
        .define("CMAKE_INSTALL_PREFIX", "webview")
        .define("CMAKE_CXX_STANDARD", "17")
        .define("CMAKE_C_STANDARD", "11")
        .define("BUILD_SHARED_LIBS", "OFF")
        .define("BUILD_EXAMPLES", "OFF")
        .define("BUILD_TESTING", "OFF")
        .define("BUILD_GTK3", "ON")
        .define("BUILD_GTK4", "OFF");
    if target.contains("windows") {
        build
            .define("BUILD_WIN32", "ON")
            .define("BUILD_WIN64", "OFF");
    } else if target.contains("linux") {
        build
            .define("BUILD_WIN32", "OFF")
            .define("BUILD_WIN64", "OFF");
    } else if target.contains("apple") {
        build
            .define("BUILD_WIN32", "OFF")
            .define("BUILD_WIN64", "OFF");
    } else {
        panic!("Unsupported target: {}", target);
    }
    let dst = build.build();
    let lib_dir = dst.join("build").join("webview").join("lib");
    let include_dir = dst.join("build").join("webview").join("include");
    let lib_path = lib_dir.join("libwebview.a");
    let include_path = include_dir.join("webview");
    let out_dir = env::var("OUT_DIR").unwrap();
    let out_path = PathBuf::from(&out_dir);
    let out_include_path = out_path.join("include");
    let out_lib_path = out_path.join("lib");
    println!("cargo:rustc-link-search=native={}", out_lib_path.display());
    println!("cargo:rustc-link-search=native={}", lib_dir.display());
    println!("cargo:rustc-link-search=native={}", include_dir.display());
    println!("cargo:rustc-link-search=native={}", out_include_path.display());
    fs::create_dir_all(&out_include_path).unwrap();
    fs::create_dir_all(&out_lib_path).unwrap();
    fs::copy(&lib_path, out_lib_path.join("libwebview.a")).unwrap();
    copy_recursively(&include_path, out_include_path.join("webview")).unwrap();
    println!("cargo:rustc-link-lib=static=webview");
    println!("cargo:rustc-link-search=native={}", out_lib_path.display());
    println!("cargo:rustc-link-search=native={}", lib_dir.display());
    println!("cargo:rustc-link-search=native={}", include_dir.display());
    println!("cargo:rustc-link-search=native={}", out_include_path.display());
}

fn copy_recursively<P: AsRef<Path>, Q: AsRef<Path>>(source: P, destination: Q) -> io::Result<()> {
    fs::create_dir_all(destination.as_ref())?;
    for entry in fs::read_dir(source.as_ref())? {
        let entry = entry?;
        let filetype = entry.file_type()?;
        if filetype.is_dir() {
            copy_recursively(entry.path(), destination.as_ref().join(entry.file_name()))?;
        } else {
            fs::copy(entry.path(), destination.as_ref().join(entry.file_name()))?;
        }
    }
    Ok(())
}