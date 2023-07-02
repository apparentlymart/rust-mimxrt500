#![no_std]
#![no_main]

// Must link the runtime to get its exception vector table.
extern crate mimxrt500_rt;

// Must link this generated PAC to get its default interrupt vector table.
extern crate mimxrt595s;

// Must link this to get the flash configuration block.
extern crate mimxrt595_evk;

use mimxrt500_rt::{entry, pre_init};
use panic_rtt_target as _;
use rtt_target::{rprintln, rtt_init_default, rtt_init_print};

const _CLKCTL1_BASE_ADDR: u32 = 0x40021000;
const _RSTCTL1_BASE_ADDR: u32 = 0x40020000;
const _IOCON_BASE_ADDR: u32 = 0x40004000;
const _GPIO_BASE_ADDR: u32 = 0x40100000;

const _PRSTCTL1_CLR: u32 = _RSTCTL1_BASE_ADDR + 0x0074;

const _CLKCTL1_PSCCTL1_SET: u32 = _CLKCTL1_BASE_ADDR + 0x0044;

const _GPIO_DIR0: u32 = _GPIO_BASE_ADDR + 0x2000;
const _GPIO_SET0: u32 = _GPIO_BASE_ADDR + 0x2200;
const _GPIO_CLR0: u32 = _GPIO_BASE_ADDR + 0x2280;
const _GPIO_NOT0: u32 = _GPIO_BASE_ADDR + 0x2300;

const _PIO0_14: u32 = _IOCON_BASE_ADDR + 0x0038;
const _PIO0_26: u32 = _IOCON_BASE_ADDR + 0x0068;
const _PIO0_31: u32 = _IOCON_BASE_ADDR + 0x007c;

#[inline(always)]
const fn raw_ptr<T>(addr: u32) -> *mut T {
    addr as *mut T
}

#[inline(always)]
unsafe fn wr32(addr: u32, v: u32) {
    core::ptr::write_volatile(raw_ptr(addr), v)
}

#[entry]
fn main() -> ! {
    rtt_init_print!();

    const BIT14: u32 = 1 << 14;
    unsafe {
        wr32(_CLKCTL1_PSCCTL1_SET, 1); // Enable GPIO port clock
        wr32(_PRSTCTL1_CLR, 1); // Take GPIO out of reset
        wr32(_PIO0_14, 0); // Disable alternate function
        wr32(_GPIO_CLR0, BIT14); // LED starts off
        wr32(_GPIO_DIR0, BIT14); // Our pin is an output
    }

    loop {
        // Toggle the LED, using the "NOT" register
        unsafe {
            wr32(_GPIO_NOT0, BIT14);
        }
        rprintln!("hello world!");

        for _ in 0..80000 {
            cortex_m::asm::nop();
        }
    }
}
