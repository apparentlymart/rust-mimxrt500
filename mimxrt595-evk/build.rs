use std::env;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

fn main() {
    // Put the linker script somewhere the linker can find it
    let out = &PathBuf::from(env::var_os("OUT_DIR").unwrap());
    File::create(out.join("memory.x"))
        .unwrap()
        .write_all(include_bytes!("memory.x"))
        .unwrap();
    println!("cargo:rustc-link-search={}", out.display());

    let evk_standalone_linkscript = out.join("bootstub-mimxrt595evk-standalone.x");
    File::create(&evk_standalone_linkscript)
        .unwrap()
        .write_all(include_bytes!("bootstub-mimxrt595evk-standalone.x"))
        .unwrap();
    println!("cargo:rustc-link-arg-bin=bootstub-evk=-T{}", evk_standalone_linkscript.display());

    // Re-run the build script if either linker script has changed.
    println!("cargo:rerun-if-changed=memory.x");
    println!("cargo:rerun-if-changed=bootstub-mimxrt595evk-standalone.x");
}
