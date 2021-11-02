use std::env;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

fn main() {
    // Put the linker script somewhere the linker can find it
    let out = &PathBuf::from(env::var_os("OUT_DIR").unwrap());
    let buf: &[u8];

    if let Some(_) = env::var_os("CARGO_FEATURE_NO_BOOTLOADER") {
        buf = include_bytes!("memory.nobootloader.x");
    } else if let Some(_) = env::var_os("CARGO_FEATURE_BOOTLOADER") {
        buf = include_bytes!("memory.bootloader.x");
    } else if let Some(_) = env::var_os("CARGO_FEATURE_APPLICATION") {
        buf = include_bytes!("memory.application.x");
    } else {
        println!("cargo:warning=One feature of bootloader, no-bootloader, or application must be set.");
        std::process::exit(1);
    }

    File::create(out.join("memory.x"))
        .unwrap()
        .write_all(buf)
        .unwrap();

    println!("cargo:rustc-link-search={}", out.display());

    // Only re-run the build script when memory.x is changed,
    // instead of when any part of the source code changes.
    println!("cargo:rerun-if-changed=memory.x");
}
