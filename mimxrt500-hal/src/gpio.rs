use crate::clocks::GpioClock;
use crate::pac;
use crate::resets::GpioReset;

/// Compile-time (type-based) GPIO pin management.
pub mod pin;

/// Runtime (value-based) GPIO pin management.
pub mod dynpin;

mod internal;

pub struct Dependencies {
    pub iopctl: pac::IOPCTL,
    pub gpio: pac::GPIO,
    pub gpio_clock: GpioClock<true>,
    pub gpio_reset: GpioReset<false>,
}

/// Encapsulates both the GPIO and IOPCTL peripherals, dealing with both
/// general port configuration and GPIO functionality.
///
/// Create a singleton `Pins` by passing the prerequisites to [`Pins::new`],
/// and then move the pins you need from the public fields of that object.
/// Pins are initially in an unknown mode, and so a typical next step is to
/// switch them into the required mode using the `.into` method for type
/// conversion.
pub struct Pins {
    deps: Dependencies,

    pub pio0_0: pin::Pin<pin::PIO0_0, pin::Unknown>,
    pub pio0_1: pin::Pin<pin::PIO0_1, pin::Unknown>,
    pub pio0_2: pin::Pin<pin::PIO0_2, pin::Unknown>,
    pub pio0_3: pin::Pin<pin::PIO0_3, pin::Unknown>,
    pub pio0_4: pin::Pin<pin::PIO0_4, pin::Unknown>,
    pub pio0_5: pin::Pin<pin::PIO0_5, pin::Unknown>,
    pub pio0_6: pin::Pin<pin::PIO0_6, pin::Unknown>,
    pub pio0_7: pin::Pin<pin::PIO0_7, pin::Unknown>,
    pub pio0_8: pin::Pin<pin::PIO0_8, pin::Unknown>,
    pub pio0_9: pin::Pin<pin::PIO0_9, pin::Unknown>,
    pub pio0_10: pin::Pin<pin::PIO0_10, pin::Unknown>,
    pub pio0_11: pin::Pin<pin::PIO0_11, pin::Unknown>,
    pub pio0_12: pin::Pin<pin::PIO0_12, pin::Unknown>,
    pub pio0_13: pin::Pin<pin::PIO0_13, pin::Unknown>,
    pub pio0_14: pin::Pin<pin::PIO0_14, pin::Unknown>,
    pub pio0_15: pin::Pin<pin::PIO0_15, pin::Unknown>,
}

impl Pins {
    #[inline(always)]
    pub const fn new(iopctl: pac::IOPCTL, gpio: pac::GPIO, gpio_clock: GpioClock<true>, gpio_reset: GpioReset<false>) -> Self {
        let deps = Dependencies {
            iopctl,
            gpio,
            gpio_clock,
            gpio_reset,
        };
        Self {
            deps,
            pio0_0: unsafe { pin::Pin::new() },
            pio0_1: unsafe { pin::Pin::new() },
            pio0_2: unsafe { pin::Pin::new() },
            pio0_3: unsafe { pin::Pin::new() },
            pio0_4: unsafe { pin::Pin::new() },
            pio0_5: unsafe { pin::Pin::new() },
            pio0_6: unsafe { pin::Pin::new() },
            pio0_7: unsafe { pin::Pin::new() },
            pio0_8: unsafe { pin::Pin::new() },
            pio0_9: unsafe { pin::Pin::new() },
            pio0_10: unsafe { pin::Pin::new() },
            pio0_11: unsafe { pin::Pin::new() },
            pio0_12: unsafe { pin::Pin::new() },
            pio0_13: unsafe { pin::Pin::new() },
            pio0_14: unsafe { pin::Pin::new() },
            pio0_15: unsafe { pin::Pin::new() },
        }
    }

    /// Consume the [`Pins`] object and recover its dependencies.
    ///
    /// Safety: The three peripherals are returned in an undefined state, and
    /// so should all be reset before using them for anything that expects
    /// them to be in a known state, which includes creating a new instance
    /// of [`Pins`].
    #[inline(always)]
    pub const unsafe fn free(self) -> Dependencies {
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
