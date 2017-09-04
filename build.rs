extern crate bundler;

use std::env;
use std::path::PathBuf;
use bundler::{Bundler, ActionConfig};

pub fn main() {
    build_resources();
    build_sdl();
}

fn build_resources() {
    println!("cargo:warning=running main build script");

    let mut bundler = Bundler::default();

    bundler.add_actions(&[
        bundler::plugin::CopyConfig::new(&["res", "shader"], &["shader"]).boxed(),
        bundler::plugin::CopyConfig::new(&["res", "image"], &["image"]).boxed(),
    ])
        .expect("failed to add actions");

    bundler.set_target_rel_path(&[])
        .expect("failed to bundle");
}

fn build_sdl() {
    let target = env::var("TARGET").unwrap();
    if target.contains("pc-windows") {
        let manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
        let mut lib_dir = manifest_dir.join("ext");
        let mut dll_dir = manifest_dir.join("ext");
        if target.contains("msvc") {
            lib_dir.push("msvc");
            dll_dir.push("msvc");
        } else {
            lib_dir.push("gnu-mingw");
            dll_dir.push("gnu-mingw");
        }
        lib_dir.push("lib");
        dll_dir.push("dll");
        if target.contains("x86_64") {
            lib_dir.push("64");
            dll_dir.push("64");
        } else {
            lib_dir.push("32");
            dll_dir.push("32");
        }
        println!("cargo:rustc-link-search=all={}", lib_dir.display());
        for entry in std::fs::read_dir(dll_dir).expect("Can't read DLL dir") {
            let entry_path = entry.expect("Invalid fs entry").path();
            let file_name_result = entry_path.file_name();
            let mut new_file_path = manifest_dir.clone();
            if let Some(file_name) = file_name_result {
                let file_name = file_name.to_str().unwrap();
                if file_name.ends_with(".dll") {
                    new_file_path.push(file_name);
                    std::fs::copy(&entry_path, new_file_path.as_path()).expect("Can't copy from DLL dir");
                }
            }
        }
    }
}