use std::path::PathBuf;

fn main() {
    let manifest_dir = PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").unwrap());
    let sharedlib_dir = manifest_dir.join("sharedlib");
    
    // 只设置链接库搜索路径！
    println!("cargo:rustc-link-search={}", sharedlib_dir.to_str().unwrap());
}
