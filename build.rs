use std::env;
use std::fs;
use std::path::Path;


fn main() {
    println!("cargo:rerun-if-changed=src/");
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=Cargo.toml");
    println!("cargo:rerun-if-changed=cbindgen.toml");

    // 获取 crate 根路径
    let crate_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let include_dir = Path::new(&crate_dir).join("include");

    // 如果 include/ 不存在就创建它
    if !include_dir.exists() {
        fs::create_dir_all(&include_dir).expect("Failed to create include directory");
    }

    // 生成头文件路径
    let header_path = include_dir.join("sdaa_ctrl.h");

    cbindgen::Builder::new()
        .with_crate(crate_dir)
        .with_config(cbindgen::Config::from_file("cbindgen.toml").unwrap())
        .generate()
        .unwrap()
        .write_to_file(header_path);
}
