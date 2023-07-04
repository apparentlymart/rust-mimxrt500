//! This crate contains implementations of some Embedded-HAL traits for the
//! i.MX RT500 series of microcontrollers from NXP.
//!
//! It is currently very early and not nearly complete.
#![no_std]

pub use embedded_hal as ehal;

#[cfg(not(any(feature = "library", feature = "device")))]
compile_error!(
    "The HAL is usually built for a specific target device, selected using a \
    feature.  If mimxrt500-hal is being built as a library, bypass this check by \
    specifying the `library` feature."
);

#[cfg(all(feature = "library", feature = "device"))]
compile_error!("Cannot combine `library` and `device` features");

#[cfg(all(feature = "fowlp", feature = "wlcsp"))]
compile_error!("Cannot combine `fowlp` and `wlcsp` features");

macro_rules! define_pac {
    ( $( ($pac:ident, $feat:literal)),+ ) => {
        $(
            #[cfg(feature = $feat)]
            pub use $pac as pac;
        )+
    };
}

define_pac!((mimxrt595s, "mimxrt595s"));

/// System and peripheral clock control.
#[cfg(feature = "device")]
pub mod clocks;

/// Peripheral reset control.
#[cfg(feature = "device")]
pub mod resets;

/// GPIO functionality and pin alternate function management.
#[cfg(feature = "device")]
pub mod gpio;

/// Types representing the data structures that the Boot ROM expects to find
/// in different kinds of boot media.
#[cfg(feature = "device")]
pub mod bootrom;

#[cfg(feature = "use_rtt")]
pub use jlink_rtt;

#[cfg(feature = "use_rtt")]
#[macro_export]
macro_rules! dbgprint {
    ($($arg:tt)*) => {
        {
            use core::fmt::Write;
            let mut out = $crate::jlink_rtt::NonBlockingOutput::new();
            writeln!(out, $($arg)*).ok();
        }
    };
}

#[cfg(not(feature = "use_rtt"))]
#[macro_export]
macro_rules! dbgprint {
    ($($arg:tt)*) => {{}};
}
