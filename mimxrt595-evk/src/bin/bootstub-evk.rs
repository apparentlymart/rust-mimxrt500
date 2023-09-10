//! A standalone boot stub intended for use with the mimxrt595-evk evaluation
//! board.
//!
//! This generates some code suitable for writing into the first page of
//! the flash memory connected to FlexSPI0 on the evaluation board, which is
//! where the main chip's boot ROM expects to find a NOR flash header and
//! an initial vector table. Along with those two prerequisites, the boot
//! stub image also includes a small amount of code which arranges to run
//! a `cortex-m-rt`-style program placed at offset 64KiB in the flash
//! memory.
//!
//! Once this stub is present, you can build your real application using
//! `cortex-m-rt` as normal and arrange for your linker to place its
//! vector table at offset `0x10000` (64KiB) in flash, so that the stub
//! code can find it and jump into it.
#![no_std]
#![no_main]
#![feature(start)]

use core::{arch::asm, panic::PanicInfo};

use mimxrt500_bootstub::{bootstub_standalone, fcb};

bootstub_standalone!(0x10000);
fcb!(mimxrt595_evk::bootstub::FCB);

#[panic_handler]
fn panic_halt(_arg: &PanicInfo) -> ! {
    loop {
        unsafe { asm!("wfi") };
    }
}
