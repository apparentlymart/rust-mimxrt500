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
use rtt_target::{rprintln, rtt_init_default};

#[pre_init]
unsafe fn before_main() {
    loop {
        cortex_m::asm::wfi();
    }
}

#[entry]
fn main() -> ! {
    rtt_init_default!();
    rprintln!("hello world!");

    loop {
        cortex_m::asm::wfi();
    }
}
