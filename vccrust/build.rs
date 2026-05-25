//! Build script for simvcc.
//!
//! This script:
//! - Adds the `sharedlib/` directory to the linker search path for CoolProp.lib
//! - Copies `CoolProp.dll` to the target output directory
//!
//! # Prerequisite
//! You must download `CoolProp.dll` and `CoolProp.lib` from
//! <http://www.coolprop.org/> and place them in the `sharedlib/` directory
//! next to this crate's `Cargo.toml`.

use std::path::PathBuf;
use std::fs;

fn main() {
    let manifest_dir = PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").unwrap());
    let sharedlib_dir = manifest_dir.join("sharedlib");

    // Set linker search path for CoolProp.lib
    println!("cargo:rustc-link-search={}", sharedlib_dir.to_str().unwrap());

    // Copy CoolProp.dll to the executable output directory
    let dll_src = sharedlib_dir.join("CoolProp.dll");
    if dll_src.exists() {
        if let Ok(out_dir) = std::env::var("OUT_DIR") {
            let out_path = PathBuf::from(&out_dir);

            // Walk up to find target/debug or target/release
            let mut current = out_path.as_path();
            while let Some(parent) = current.parent() {
                if let Some(dir_name) = parent.file_name().and_then(|s| s.to_str()) {
                    if dir_name == "debug" || dir_name == "release" {
                        let dll_dst = parent.join("CoolProp.dll");
                        let _ = fs::copy(&dll_src, &dll_dst);
                        break;
                    }
                }
                current = parent;
            }
        }
    }
}
