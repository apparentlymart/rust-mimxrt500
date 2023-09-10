#![no_std]
#![no_main]

// Must link the runtime to get its exception vector table.
extern crate cortex_m_rt;

// Must link this generated PAC to get its default interrupt vector table.
extern crate mimxrt595s as pac;

// Must link the bootstub crate to get a suitable flash control block and
// boot glue code.
extern crate mimxrt595_evk_bootstub as _;

extern crate mimxrt595_evk as evk;

use mimxrt500_hal as hal;

use cortex_m_rt::entry;
use panic_rtt_target as _;
use rtt_target::{rprintln, rtt_init_print};

#[entry]
fn main() -> ! {
    rtt_init_print!();

    rprintln!("startup");

    let p = unsafe { pac::Peripherals::steal() };

    rprintln!("stole peripheral");

    let clocks = hal::clocks::Clocks::new(p.CLKCTL0, p.CLKCTL1);
    let resets = hal::resets::Resets::new(p.RSTCTL0, p.RSTCTL1);

    let pins = hal::gpio::Pins::new(p.IOPCTL, p.GPIO, clocks.gpio.into(), resets.gpio.into());
    let pins = evk::Pins::wrap(pins);
    rprintln!("we have the pins object");

    let red_pin = pins.rgb_led.red;
    red_pin.set_output(false);
    let blue_pin = pins.rgb_led.blue;
    blue_pin.set_output(true);
    rprintln!("red LED pin is initially off, blue is on");

    let sw1_pin: evk::pins::NmiButtonPin = pins.buttons.nmi.into();

    let red_pin: evk::pins::RgbLedRedPin = red_pin.into();
    let blue_pin: evk::pins::RgbLedBluePin = blue_pin.into();
    rprintln!("LED pin is now a push-pull output");

    blinky(red_pin, blue_pin, sw1_pin)
}

fn blinky(
    mut red_pin: impl embedded_hal::digital::ToggleableOutputPin,
    mut blue_pin: impl embedded_hal::digital::ToggleableOutputPin,
    sw1_pin: impl embedded_hal::digital::InputPin,
) -> ! {
    loop {
        // Toggle the LED
        red_pin.toggle().unwrap();
        blue_pin.toggle().unwrap();
        if sw1_pin.is_low().unwrap() {
            rprintln!("pressed");
        } else {
            rprintln!("not pressed");
        }

        for _ in 0..80000 {
            cortex_m::asm::nop();
        }
    }
}
