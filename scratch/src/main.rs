#![no_std]
#![no_main]

// Must link the runtime to get its exception vector table.
extern crate mimxrt500_rt;

// Must link this generated PAC to get its default interrupt vector table.
extern crate mimxrt595s as pac;

// Must link this to get the flash configuration block.
extern crate mimxrt595_evk;

use mimxrt500_hal as hal;

use mimxrt500_rt::entry;
use panic_rtt_target as _;
use rtt_target::{rprintln, rtt_init_print};

#[entry]
fn main() -> ! {
    rtt_init_print!();

    rprintln!("startup");

    let p = unsafe { pac::Peripherals::steal() };

    rprintln!("stole peripheral");

    // Enable GPIO port clock
    let clkctl1 = p.CLKCTL1;
    clkctl1.pscctl1_set.write(|w| w.hsgpio0_clk().set_bit());

    rprintln!("enabled clock");

    // Take GPIO out of reset
    let rstctl1 = p.RSTCTL1;
    rstctl1.prstctl1_clr.write(|w| w.hsgpio0().set_bit());

    rprintln!("GPIO out of reset");

    let pins = hal::gpio::Pins::new(p.IOPCTL, p.GPIO);
    rprintln!("we have the pins object");

    // LED is initially off
    let led_pin = pins.pio0_14;
    led_pin.set_output(false);

    rprintln!("LED pin is initially off");

    let led_pin: hal::gpio::pin::Pin<hal::gpio::pin::PIO0_14, hal::gpio::pin::output::OutputPushPull> = led_pin.into();

    rprintln!("LED pin is now a push-pull output");

    loop {
        // Toggle the LED
        led_pin.toggle_output();
        rprintln!("hello world!");

        for _ in 0..80000 {
            cortex_m::asm::nop();
        }
    }
}
