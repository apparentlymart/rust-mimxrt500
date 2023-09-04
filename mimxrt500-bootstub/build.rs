use std::env;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

fn main() {
    // Put the linker script somewhere the linker can find it
    let out = &PathBuf::from(env::var_os("OUT_DIR").unwrap());
    let linkscript = out.join("bootstub-evk.x");
    File::create(&linkscript)
        .unwrap()
        .write_all(include_bytes!("bootstub-evk.x"))
        .unwrap();
    println!("cargo:rustc-link-arg-bin=bootstub-evk=-T{}", linkscript.display());

    // Only re-run the build script when bootstub-evk.x is changed,
    // instead of when any part of the source code changes.
    println!("cargo:rerun-if-changed=bootstub-evk.x");
}
