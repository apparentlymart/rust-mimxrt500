use std::env;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;
fn main() {
    let target = env::var("TARGET").unwrap();
    if target.starts_with("thumbv8m.main-") {
        println!("cargo:rustc-cfg=cortex_m");
        println!("cargo:rustc-cfg=armv8m");
        println!("cargo:rustc-cfg=armv8m_main");

        let out = &PathBuf::from(env::var_os("OUT_DIR").unwrap());
        File::create(out.join("mimxrt500-link.x"))
            .unwrap()
            .write_all(include_bytes!("mimxrt500-link.x"))
            .unwrap();
        println!("cargo:rustc-link-search={}", out.display());
        println!("cargo:rerun-if-changed=mimxrt500-link.x");
        println!("cargo:rerun-if-changed=build.rs");
    }
    if target.ends_with("-eabihf") {
        // The chip itself does have an FPU, but some programs might
        // prefer not to use the hardfloat ABI.
        println!("cargo:rustc-cfg=has_fpu");
    }
}
