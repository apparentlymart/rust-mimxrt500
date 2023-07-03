use crate::pac;

/// Compile-time (type-based) GPIO pin management.
pub mod pin;

/// Runtime (value-based) GPIO pin management.
pub mod dynpin;

mod internal;

pub struct PinDependencies {
    iopctl: pac::IOPCTL,
    gpio: pac::GPIO,
}

pub struct Pins {
    deps: PinDependencies,

    pub pio0_0: pin::Pin<
        pin::PIO0_0,
        pin::Unknown,
    >,
    pub pio0_14: pin::Pin<
        pin::PIO0_14,
        pin::Unknown,
    >,
}

impl Pins {
    #[inline(always)]
    pub const fn new(iopctl: pac::IOPCTL, gpio: pac::GPIO) -> Self {
        let deps = PinDependencies { iopctl, gpio };
        Self {
            deps,
            pio0_0: unsafe { pin::Pin::new() },
            pio0_14: unsafe { pin::Pin::new() },
        }
    }

    /// Consume the [`Pins`] object and recover its dependencies.
    ///
    /// Safety: The three peripherals are returned in an undefined state, and
    /// so should all be reset before using them for anything that expects
    /// them to be in a known state, which includes creating a new instance
    /// of [`Pins`].
    #[inline(always)]
    pub const unsafe fn free(self) -> PinDependencies {
        self.deps
    }

    /// Borrow the underlying IOPCTL periopheral.
    ///
    /// Safety: Callers must not change the configuration or state of any
    /// pins that are associated with active [`pin::Pin`] or [`dynpin::DynPin`]
    /// objects.
    #[inline(always)]
    pub const unsafe fn borrow_iopctl(&self) -> &pac::IOPCTL {
        &self.deps.iopctl
    }

    /// Borrow the underlying GPIO periopheral.
    ///
    /// Safety: Callers must not change the configuration or state of any
    /// pins that are associated with active [`pin::Pin`] or [`dynpin::DynPin`]
    /// objects.
    #[inline(always)]
    pub const unsafe fn borrow_gpio(&self) -> &pac::GPIO {
        &self.deps.gpio
    }
}
