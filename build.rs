use std::env;
use std::fs;
use std::path::Path;
use std::process::Command;

fn main() {
    println!("cargo:rerun-if-changed=src/");
    println!("cargo:rerun-if-changed=Cargo.toml");

    // 获取 crate 根路径
    let crate_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let include_dir = Path::new(&crate_dir).join("include");

    // 如果 include/ 不存在就创建它
    if !include_dir.exists() {
        fs::create_dir_all(&include_dir).expect("Failed to create include directory");
    }

    // 生成头文件路径
    let header_path = include_dir.join("sdaa_ctrl.h");

    // 执行 cbindgen 命令
    if let Ok(status) = Command::new("cbindgen")
        .arg("--config")
        .arg("cbindgen.toml") // 可选：可省略
        .arg("--crate")
        .arg("sdaa_ctrl") // ⚠️ 替换为你的 crate 名
        .arg("--output")
        .arg(header_path)
        .current_dir(&crate_dir)
        .status()
    {
        if !status.success() {
            eprintln!("cbindgen failed");
        }
    } else {
        eprintln!("cbindgen failed");
    }
}
