fn main() {
    // Specify linker arguments.

    // `--nmagic` is required because some of the bootstub's memory section
    // addresses are not aligned to 0x10000.
    println!("cargo:rustc-link-arg=-Wl,--nmagic");

    // Set the linker script to the one provided by cortex-m-rt.
    println!("cargo:rustc-link-arg=-Tlink.x");
}
