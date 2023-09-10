//! Generates a NOR flash header, initial vector table, and small shim code
//! that can be written to the start of the NOR flash connected to FlexSPI0
//! on an i.MX RT500-series chip to deal with its unusual requirements before
//! jumping into a more normal-looking embedded Rust application linked to
//! somewhere else in memory -- typically a later page or block of the same
//! Flash memory, but that's not required.
//!
//! This series of chips has no built-in flash memory and so instead has a
//! boot ROM that probes various peripherals to try to find some external
//! memory containing a boot image. One option is a NOR flash connected to
//! FlexSPI0, but that requires some chip-specific header information in
//! the first page of flash that is intermingled with the initial vector
//! table, and is thus incompatible with the image layout expected by
//! the `cortex-m-rt` crate.
//!
//! To allow using `cortex-m-rt` in the normal way, this crate can help
//! generate a small stub image to write into the first page of memory,
//! separately from the main application, which then expects to find a
//! normal-looking `cortex-m-rt`-based application at some other memory
//! address and begins executing it.
//!
//! This does mean "wasting" at least a page of flash memory and having
//! a redundant extra vector table used only by the boot stub, but that's
//! typically a small price to pay for the convenience of meeting the
//! expectations of the embedded Rust ecosystem for Cortex-M-based platforms.
#![no_std]

use core::arch::asm;

pub mod bootrom;

extern "C" {
    #[doc(hidden)]
    pub fn __mimxrt500_bootstub_main(app_vectors: *const u32);
    #[doc(hidden)]
    pub fn __mimxrt500_bootstub();
    #[doc(hidden)]
    fn __mimxrt500_bootstub_image_start();
    #[doc(hidden)]
    pub fn __vector_table();
}

#[macro_export]
macro_rules! bootstub_standalone {
    ($app_vectors:literal) => {
        ::core::arch::global_asm!(
            ".cfi_sections .debug_frame",
            ".section .mimxrt500_bootstub.text, \"ax\"",
            ".global {entry}",
            ".type {entry},%function",
            ".thumb_func",
            ".cfi_startproc",
            "{entry}:",
            concat!("ldr r0,=", $app_vectors),
            "b {bootstub}",
            ".cfi_endproc",
            ".size {entry}, . - {entry}",
            entry = sym $crate::__mimxrt500_bootstub,
            bootstub = sym $crate::__mimxrt500_bootstub_main,
        );
    }
}

#[macro_export]
macro_rules! bootstub_builtin {
    () => {
        ::core::arch::global_asm!(
            ".cfi_sections .debug_frame",
            ".section .mimxrt500_bootstub.text, \"ax\"",
            ".global {entry}",
            ".type {entry},%function",
            ".thumb_func",
            ".cfi_startproc",
            "{entry}:",
            "ldr r0,={vectors}",
            "b {bootstub}",
            ".cfi_endproc",
            ".size {entry}, . - {entry}",
            entry = sym $crate::__mimxrt500_bootstub,
            bootstub = sym $crate::__mimxrt500_bootstub_main,
            vectors = sym $crate::__vector_table,
        );
    }
}


#[macro_export]
macro_rules! bootstub_builtin_custom {
    ($app_vectors:path) => {
        ::core::arch::global_asm!(
            ".cfi_sections .debug_frame",
            ".section .mimxrt500_bootstub.text, \"ax\"",
            ".global {entry}",
            ".type {entry},%function",
            ".thumb_func",
            ".cfi_startproc",
            "{entry}:",
            "ldr r0,={vectors}",
            "b {bootstub}",
            ".cfi_endproc",
            ".size {entry}, . - {entry}",
            entry = sym $crate::__mimxrt500_bootstub,
            bootstub = sym $crate::__mimxrt500_bootstub_main,
            vectors = sym $app_vectors,
        );
    }
}

#[macro_export]
macro_rules! fcb {
    ($data:expr) => {
        #[doc(hidden)]
        #[link_section = ".mimxrt500_bootstub.fcb"]
        #[no_mangle]
        static __MIMXRT500_FCB: $crate::bootrom::FlexSpiNorFlashConfig = $data;
    };
}

::core::arch::global_asm!(
    ".cfi_sections .debug_frame",
    ".section .mimxrt500_bootstub.text, \"ax\"",
    ".global {bootstub}",
    ".type {bootstub},%function",
    ".thumb_func",
    ".cfi_startproc",
    "{bootstub}:",

    // Use the application's vector table
    "ldr r1, =0xe000ed08", // r1 points at VTOR
    "str r0, [r1]", // store app_vectors argument (r0) to VTOR (*r1)

    // Load application's initial stack pointer
    "ldr sp, [r0, #0]",

    // Jump to application's reset vector
    "ldr pc, [r0, #4]",

    "1:",
    "b 1b",
    ".cfi_endproc",
    ".size {bootstub}, . - {bootstub}",
    bootstub = sym __mimxrt500_bootstub_main,
);

#[link_section = ".mimxrt500_bootstub.text"]
extern "C" fn default_exception_handler() {
    loop {
        unsafe { asm!("wfi") };
    }
}

#[doc(hidden)]
pub union Vector {
    handler: unsafe extern "C" fn(),
    reserved: u32,
}

const DEFAULT_VECTOR: Vector = Vector {
    handler: default_exception_handler,
};
const RESERVED_VECTOR: Vector = Vector { reserved: 0 };

const IMGTYPE_PLAIN_NO_SECURE: u32 = 0x00004000;

#[doc(hidden)]
#[link_section = ".mimxrt500_bootstub.exceptions"]
#[no_mangle]
pub static __mimxrt500_bootstub_exceptions: [Vector; 16] = [
    // Initial stack pointer is irrelevant because we don't use the stack,
    // but the boot ROM seems to verify that this points to a reasonable
    // address in RAM, so we'll just arbitrarily choose one.
    Vector { reserved: 0x5000 },
    // Reset vector is the generated boot stub
    Vector {
        handler: __mimxrt500_bootstub,
    },
    // NMI Exception
    DEFAULT_VECTOR,
    // HardFault Exception
    DEFAULT_VECTOR,
    // MemManage Exception
    DEFAULT_VECTOR,
    // BusFault Exception
    DEFAULT_VECTOR,
    // UsageFault Exception
    DEFAULT_VECTOR,
    // SecureFault Exception
    DEFAULT_VECTOR,
    // Entry 8 is used by the RT500 boot ROM as the image size, which
    // isn't really relevant here because we're not using the boot ROM's
    // checksum and copy-to-RAM features. We'll just claim that the
    // vector table is the entirety of the image.
    Vector { reserved: 256 * 4 },
    // Entry 9 is used by the RT500 boot ROM as the image type.
    Vector {
        reserved: IMGTYPE_PLAIN_NO_SECURE,
    },
    // Reserved entry
    RESERVED_VECTOR,
    // SVCall Exception
    DEFAULT_VECTOR,
    // Debug Monitor Exception
    DEFAULT_VECTOR,
    // Entry 13 is used by the RT500 boot ROM as the image load address,
    // which must point to the start of this vector table for a flash XIP
    // image.
    Vector {
        handler: __mimxrt500_bootstub_image_start,
    },
    // PendSV Exception
    DEFAULT_VECTOR,
    // SysTick Exception.
    DEFAULT_VECTOR,
];
