#![no_std]
#![no_main]

// Must link this generated PAC to get its default interrupt vector table.
extern crate mimxrt595s;

// Must link this to get the flash configuration block.
extern crate mimxrt595_evk;

use cortex_m_rt::entry;
use panic_rtt_target as _;
use rtt_target::{rprintln, rtt_init_default};

#[entry]
fn main() -> ! {
    rtt_init_default!();
    rprintln!("hello world!");

    loop {
        cortex_m::asm::wfi();
    }
}

#[doc(hidden)]
pub union Vector {
    handler: unsafe extern "C" fn(),
    reserved: usize,
}

extern "C" {
    fn NonMaskableInt();
    fn HardFaultTrampoline();
    fn MemoryManagement();
    fn BusFault();
    fn UsageFault();
    fn SecureFault();
    fn SVCall();
    fn DebugMonitor();
    fn PendSV();
    fn SysTick();
}

// These are not really functions but instead data fields that the RT500 boot
// ROM expects to find in reserved entries in the vector table.
//
// Our linker script assigns appropriate values to these so that they'll end
// up having suitable values in the vector table despite not actually being
// functions.
extern "C" {
    fn __image_hdr_size();
    fn __image_hdr_type();
    fn __image_hdr_load_addr();
}

#[doc(hidden)]
#[link_section = ".vector_table.rt500_exceptions"]
#[no_mangle]
pub static __RT500_EXCEPTIONS: [Vector; 14] = [
    // Exception 2: Non Maskable Interrupt.
    Vector {
        handler: NonMaskableInt,
    },
    // Exception 3: Hard Fault Interrupt.
    Vector {
        handler: HardFaultTrampoline,
    },
    // Exception 4: Memory Management Interrupt [not on Cortex-M0 variants].
    Vector {
        handler: MemoryManagement,
    },
    // Exception 5: Bus Fault Interrupt [not on Cortex-M0 variants].
    Vector { handler: BusFault },
    // Exception 6: Usage Fault Interrupt [not on Cortex-M0 variants].
    Vector {
        handler: UsageFault,
    },
    // Exception 7: Secure Fault Interrupt [only on Armv8-M].
    Vector {
        handler: SecureFault,
    },
    // Entry 8 is used by the RT500 boot ROM as the image size.
    Vector { handler: __image_hdr_size },
    // Entry 9 is used by the RT500 boot ROM as the image type.
    Vector { handler: __image_hdr_type },
    // 10: Reserved
    Vector { reserved: 0 },
    // Exception 11: SV Call Interrupt.
    Vector { handler: SVCall },
    // Exception 12: Debug Monitor Interrupt [not on Cortex-M0 variants].
    Vector {
        handler: DebugMonitor,
    },
    // Entry 13 is used by the RT500 boot ROM as the image load address.
    Vector { handler: __image_hdr_load_addr },
    // Exception 14: Pend SV Interrupt [not on Cortex-M0 variants].
    Vector { handler: PendSV },
    // Exception 15: System Tick Interrupt.
    Vector { handler: SysTick },
];
