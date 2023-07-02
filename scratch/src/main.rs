#![no_std]
#![no_main]

// Must link the runtime to get its exception vector table.
extern crate mimxrt500_rt;

// Must link this generated PAC to get its default interrupt vector table.
extern crate mimxrt595s as pac;

// Must link this to get the flash configuration block.
extern crate mimxrt595_evk;

use mimxrt500_rt::entry;
use panic_rtt_target as _;
use rtt_target::{rprintln, rtt_init_print};

#[entry]
fn main() -> ! {
    rtt_init_print!();

    let p = unsafe { pac::Peripherals::steal() };

    // Enable GPIO port clock
    let clkctl1 = p.CLKCTL1;
    clkctl1.pscctl1_set.write(|w| w.hsgpio0_clk().set_bit());

    // Take GPIO out of reset
    let rstctl1 = p.RSTCTL1;
    rstctl1.prstctl1_clr.write(|w| w.hsgpio0().set_bit());

    // Disable alternate function
    let iopctl = p.IOPCTL;
    iopctl.pio0_14.write(|w| w.fsel().fsel_0());

    // LED is initially off
    let gpio = p.GPIO;
    gpio.clr[0].write(|w| w.clrp14().clear_bit_by_one());

    // The pin is an output
    gpio.dir[0].write(|w| w.dirp14().set_bit());

    loop {
        // Toggle the LED, using the "NOT" register
        gpio.not[0].write(|w| w.notp14().set_bit());
        rprintln!("hello world!");

        for _ in 0..80000 {
            cortex_m::asm::nop();
        }
    }
}
