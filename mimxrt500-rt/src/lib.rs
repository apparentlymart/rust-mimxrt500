#![no_std]

pub use cortex_m_rt_macros::{entry, exception, interrupt, pre_init};

#[cfg(cortex_m)]
use core::arch::global_asm;

// HardFault exceptions are bounced through this trampoline which grabs the stack pointer at
// the time of the exception and passes it to the user's HardFault handler in r0.
// Depending on the stack mode in EXC_RETURN, fetches stack from either MSP or PSP.
#[cfg(cortex_m)]
global_asm!(
    ".cfi_sections .debug_frame
     .section .HardFaultTrampoline, \"ax\"
     .global HardFaultTrampline
     .type HardFaultTrampline,%function
     .thumb_func
     .cfi_startproc
     HardFaultTrampoline:",
    "mov r0, lr
     movs r1, #4
     tst r0, r1
     bne 0f
     mrs r0, MSP
     b HardFault
     0:
     mrs r0, PSP
     b HardFault",
    ".cfi_endproc
     .size HardFaultTrampoline, . - HardFaultTrampoline",
);

/// Parse cfg attributes inside a global_asm call.
#[cfg(cortex_m)]
macro_rules! cfg_global_asm {
    {@inner, [$($x:tt)*], } => {
        global_asm!{$($x)*}
    };
    (@inner, [$($x:tt)*], #[cfg($meta:meta)] $asm:literal, $($rest:tt)*) => {
        #[cfg($meta)]
        cfg_global_asm!{@inner, [$($x)* $asm,], $($rest)*}
        #[cfg(not($meta))]
        cfg_global_asm!{@inner, [$($x)*], $($rest)*}
    };
    {@inner, [$($x:tt)*], $asm:literal, $($rest:tt)*} => {
        cfg_global_asm!{@inner, [$($x)* $asm,], $($rest)*}
    };
    {$($asms:tt)*} => {
        cfg_global_asm!{@inner, [], $($asms)*}
    };
}

// This reset vector is the initial entry point after a system reset.
// Calls an optional user-provided __pre_init and then initialises RAM.
// If the target has an FPU, it is enabled.
// Finally jumps to the user main function.
#[cfg(cortex_m)]
cfg_global_asm! {
    ".cfi_sections .debug_frame
     .section .Reset, \"ax\"
     .global Reset
     .type Reset,%function
     .thumb_func",
    ".cfi_startproc
     Reset:",

    // Initialize the stack pointer
    "ldr r0, =_stack_start
     msr msp, r0",

    // Initialize VTOR to point to the start of the vector table
    "ldr r0, =0xe000ed08
     ldr r1, =__vector_table
     str r1, [r0]",

    // Run user pre-init code which must be executed immediately after startup, before the
    // potentially time-consuming memory initialisation takes place.
    // Example use cases include disabling default watchdogs or enabling RAM.
    "bl __pre_init",

    // Initialise .bss memory. `__sbss` and `__ebss` come from the linker script.
    "ldr r0, =__sbss
     ldr r1, =__ebss
     movs r2, #0
     2:
     cmp r1, r0
     beq 3f
     stm r0!, {{r2}}
     b 2b
     3:",

    // Initialise .data memory. `__sdata`, `__sidata`, and `__edata` come from the linker script.
    "ldr r0, =__sdata
     ldr r1, =__edata
     ldr r2, =__sidata
     4:
     cmp r1, r0
     beq 5f
     ldm r2!, {{r3}}
     stm r0!, {{r3}}
     b 4b
     5:",

    // Enable the FPU if we've compiled for the hardfloat ABI.
    // SCB.CPACR is 0xE000_ED88.
    // We enable access to CP10 and CP11 from priviliged and unprivileged mode.
    #[cfg(has_fpu)]
    "ldr r0, =0xE000ED88
     ldr r1, =(0b1111 << 20)
     ldr r2, [r0]
     orr r2, r2, r1
     str r2, [r0]
     dsb
     isb",

    // Jump to user main function.
    // `bl` is used for the extended range, but the user main function should not return,
    // so trap on any unexpected return.
    "bl main
     udf #0",

    ".cfi_endproc
     .size Reset, . - Reset",
}

#[doc(hidden)]
pub union Vector {
    handler: unsafe extern "C" fn(),
    reserved: usize,
}

extern "C" {
    fn Reset() -> !;
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
    // NOTE: Initial stack pointer and reset vector are prepended inside
    // the linker script, so not included directly here.

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
    Vector {
        handler: __image_hdr_size,
    },
    // Entry 9 is used by the RT500 boot ROM as the image type.
    Vector {
        handler: __image_hdr_type,
    },
    // 10: Reserved
    Vector { reserved: 0 },
    // Exception 11: SV Call Interrupt.
    Vector { handler: SVCall },
    // Exception 12: Debug Monitor Interrupt [not on Cortex-M0 variants].
    Vector {
        handler: DebugMonitor,
    },
    // Entry 13 is used by the RT500 boot ROM as the image load address.
    Vector {
        handler: __image_hdr_load_addr,
    },
    // Exception 14: Pend SV Interrupt [not on Cortex-M0 variants].
    Vector { handler: PendSV },
    // Exception 15: System Tick Interrupt.
    Vector { handler: SysTick },
];

/// Returns a pointer to the start of the heap
///
/// The returned pointer is guaranteed to be 4-byte aligned.
#[inline]
pub fn heap_start() -> *mut u32 {
    extern "C" {
        static mut __sheap: u32;
    }

    unsafe { &mut __sheap }
}

// Entry point is Reset.
#[doc(hidden)]
#[cfg_attr(cortex_m, link_section = ".vector_table.reset_vector")]
#[no_mangle]
pub static __RESET_VECTOR: unsafe extern "C" fn() -> ! = Reset;

#[allow(unused_variables)]
#[doc(hidden)]
#[cfg_attr(cortex_m, link_section = ".HardFault.default")]
#[no_mangle]
pub unsafe extern "C" fn HardFault_(ef: &ExceptionFrame) -> ! {
    #[allow(clippy::empty_loop)]
    loop {}
}

#[doc(hidden)]
#[no_mangle]
pub unsafe extern "C" fn DefaultHandler_() -> ! {
    #[allow(clippy::empty_loop)]
    loop {}
}

#[doc(hidden)]
#[no_mangle]
pub unsafe extern "C" fn DefaultPreInit() {}

/// Registers stacked (pushed onto the stack) during an exception.
#[derive(Clone, Copy)]
#[repr(C)]
pub struct ExceptionFrame {
    r0: u32,
    r1: u32,
    r2: u32,
    r3: u32,
    r12: u32,
    lr: u32,
    pc: u32,
    xpsr: u32,
}

impl ExceptionFrame {
    /// Returns the value of (general purpose) register 0.
    #[inline(always)]
    pub fn r0(&self) -> u32 {
        self.r0
    }

    /// Returns the value of (general purpose) register 1.
    #[inline(always)]
    pub fn r1(&self) -> u32 {
        self.r1
    }

    /// Returns the value of (general purpose) register 2.
    #[inline(always)]
    pub fn r2(&self) -> u32 {
        self.r2
    }

    /// Returns the value of (general purpose) register 3.
    #[inline(always)]
    pub fn r3(&self) -> u32 {
        self.r3
    }

    /// Returns the value of (general purpose) register 12.
    #[inline(always)]
    pub fn r12(&self) -> u32 {
        self.r12
    }

    /// Returns the value of the Link Register.
    #[inline(always)]
    pub fn lr(&self) -> u32 {
        self.lr
    }

    /// Returns the value of the Program Counter.
    #[inline(always)]
    pub fn pc(&self) -> u32 {
        self.pc
    }

    /// Returns the value of the Program Status Register.
    #[inline(always)]
    pub fn xpsr(&self) -> u32 {
        self.xpsr
    }

    /// Sets the stacked value of (general purpose) register 0.
    ///
    /// # Safety
    ///
    /// This affects the `r0` register of the preempted code, which must not rely on it getting
    /// restored to its previous value.
    #[inline(always)]
    pub unsafe fn set_r0(&mut self, value: u32) {
        self.r0 = value;
    }

    /// Sets the stacked value of (general purpose) register 1.
    ///
    /// # Safety
    ///
    /// This affects the `r1` register of the preempted code, which must not rely on it getting
    /// restored to its previous value.
    #[inline(always)]
    pub unsafe fn set_r1(&mut self, value: u32) {
        self.r1 = value;
    }

    /// Sets the stacked value of (general purpose) register 2.
    ///
    /// # Safety
    ///
    /// This affects the `r2` register of the preempted code, which must not rely on it getting
    /// restored to its previous value.
    #[inline(always)]
    pub unsafe fn set_r2(&mut self, value: u32) {
        self.r2 = value;
    }

    /// Sets the stacked value of (general purpose) register 3.
    ///
    /// # Safety
    ///
    /// This affects the `r3` register of the preempted code, which must not rely on it getting
    /// restored to its previous value.
    #[inline(always)]
    pub unsafe fn set_r3(&mut self, value: u32) {
        self.r3 = value;
    }

    /// Sets the stacked value of (general purpose) register 12.
    ///
    /// # Safety
    ///
    /// This affects the `r12` register of the preempted code, which must not rely on it getting
    /// restored to its previous value.
    #[inline(always)]
    pub unsafe fn set_r12(&mut self, value: u32) {
        self.r12 = value;
    }

    /// Sets the stacked value of the Link Register.
    ///
    /// # Safety
    ///
    /// This affects the `lr` register of the preempted code, which must not rely on it getting
    /// restored to its previous value.
    #[inline(always)]
    pub unsafe fn set_lr(&mut self, value: u32) {
        self.lr = value;
    }

    /// Sets the stacked value of the Program Counter.
    ///
    /// # Safety
    ///
    /// This affects the `pc` register of the preempted code, which must not rely on it getting
    /// restored to its previous value.
    #[inline(always)]
    pub unsafe fn set_pc(&mut self, value: u32) {
        self.pc = value;
    }

    /// Sets the stacked value of the Program Status Register.
    ///
    /// # Safety
    ///
    /// This affects the `xPSR` registers (`IPSR`, `APSR`, and `EPSR`) of the preempted code, which
    /// must not rely on them getting restored to their previous value.
    #[inline(always)]
    pub unsafe fn set_xpsr(&mut self, value: u32) {
        self.xpsr = value;
    }
}

impl core::fmt::Debug for ExceptionFrame {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        struct Hex(u32);
        impl core::fmt::Debug for Hex {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                write!(f, "0x{:08x}", self.0)
            }
        }
        f.debug_struct("ExceptionFrame")
            .field("r0", &Hex(self.r0))
            .field("r1", &Hex(self.r1))
            .field("r2", &Hex(self.r2))
            .field("r3", &Hex(self.r3))
            .field("r12", &Hex(self.r12))
            .field("lr", &Hex(self.lr))
            .field("pc", &Hex(self.pc))
            .field("xpsr", &Hex(self.xpsr))
            .finish()
    }
}
