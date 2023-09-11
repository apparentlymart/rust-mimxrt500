use paste::paste;

use crate::clocks::GpioClock;
use crate::pac;
use crate::resets::GpioReset;

/// Compile-time (type-based) GPIO pin management.
pub mod pin;

/// Runtime (value-based) GPIO pin management.
pub mod dynpin;

mod internal;

/// The objects that get consumed when constructing a [`Pins`] object.
pub struct Dependencies {
    pub iopctl: pac::IOPCTL,
    pub gpio: pac::GPIO,
    pub gpio_clock: GpioClock<true>,
    pub gpio_reset: GpioReset<false>,
}

macro_rules! pins{
    (
        $(
            ($Port:literal, $Pin:literal),
        )+
    ) => {
        paste! {
            /// Encapsulates both the GPIO and IOPCTL peripherals, dealing with both
            /// general port configuration and GPIO functionality.
            ///
            /// Create a singleton `Pins` by passing the prerequisites to [`Pins::new`],
            /// and then move the pins you need from the public fields of that object.
            /// Pins are initially in an unknown mode, and so a typical next step is to
            /// switch them into the required mode using the `.into` method for type
            /// conversion.
            #[allow(non_snake_case)]
            pub struct Pins {
                deps: Dependencies,
                $(
                    pub [<pio $Port _ $Pin>]: pin::Pin<pin::[<PIO $Port _ $Pin>], pin::Unknown>,
                )+
            }
            impl Pins {
                /// Take ownership of the PAC
                /// [`PORT`](crate::pac::PORT) and split it into
                /// discrete [`Pin`]s
                #[inline(always)]
                pub const fn new(iopctl: pac::IOPCTL, gpio: pac::GPIO, gpio_clock: GpioClock<true>, gpio_reset: GpioReset<false>) -> Pins {
                    Pins {
                        deps: Dependencies {
                            iopctl,
                            gpio,
                            gpio_clock,
                            gpio_reset,
                        },
                        // Safety: we only create one `Pin` per `PinId`
                        $(
                            [<pio $Port _ $Pin>]: unsafe { pin::Pin::new() },
                        )+
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
            }
        }
    };
}

pins!(
    (0, 0),
    (0, 1),
    (0, 2),
    (0, 3),
    (0, 4),
    (0, 5),
    (0, 6),
    (0, 7),
    (0, 8),
    (0, 9),
    (0, 10),
    (0, 11),
    (0, 12),
    (0, 13),
    (0, 14),
    (0, 15),
    (0, 16),
    (0, 17),
    (0, 18),
    (0, 19),
    (0, 20),
    (0, 21),
    (0, 22),
    (0, 23),
    (0, 24),
    (0, 25),
    (0, 26),
    (0, 27),
    (0, 28),
    (0, 29),
    (0, 30),
    (0, 31),
    (1, 0),
    (1, 1),
    (1, 2),
    (1, 3),
    (1, 4),
    (1, 5),
    (1, 6),
    (1, 7),
    (1, 8),
    (1, 9),
    (1, 10),
    (1, 11),
    (1, 12),
    (1, 13),
    (1, 14),
    (1, 15),
    (1, 16),
    (1, 17),
    (1, 18),
    (1, 19),
    (1, 20),
    (1, 21),
    (1, 22),
    (1, 23),
    (1, 24),
    (1, 25),
    (1, 26),
    (1, 27),
    (1, 28),
    (1, 29),
    (1, 30),
    (1, 31),
    (2, 0),
    (2, 1),
    (2, 2),
    (2, 3),
    (2, 4),
    (2, 5),
    (2, 6),
    (2, 7),
    (2, 8),
    (2, 9),
    (2, 10),
    (2, 11),
    (2, 12),
    (2, 13),
    (2, 14),
    (2, 15),
    (2, 16),
    (2, 17),
    (2, 18),
    (2, 19),
    (2, 20),
    (2, 21),
    (2, 22),
    (2, 23),
    (2, 24),
    (2, 25),
    (2, 26),
    (2, 27),
    (2, 28),
    (2, 29),
    (2, 30),
    (2, 31),
    (3, 0),
    (3, 1),
    (3, 2),
    (3, 3),
    (3, 4),
    (3, 5),
    (3, 6),
    (3, 7),
    (3, 8),
    (3, 9),
    (3, 10),
    (3, 11),
    (3, 12),
    (3, 13),
    (3, 14),
    (3, 15),
    (3, 16),
    (3, 17),
    (3, 18),
    (3, 19),
    (3, 20),
    (3, 21),
    (3, 22),
    (3, 23),
    (3, 24),
    (3, 25),
    (3, 26),
    (3, 27),
    (3, 28),
    (3, 29),
    (3, 30),
    (3, 31),
    (4, 0),
    (4, 1),
    (4, 2),
    (4, 3),
    (4, 4),
    (4, 5),
    (4, 6),
    (4, 7),
    (4, 8),
    (4, 9),
    (4, 10),
    (4, 11),
    (4, 12),
    (4, 13),
    (4, 14),
    (4, 15),
    (4, 16),
    (4, 17),
    (4, 18),
    (4, 19),
    (4, 20),
    (4, 21),
    (4, 22),
    (4, 23),
    (4, 24),
    (4, 25),
    (4, 26),
    (4, 27),
    (4, 28),
    (4, 29),
    (4, 30),
    (4, 31),
    (5, 0),
    (5, 1),
    (5, 2),
    (5, 3),
    (5, 4),
    (5, 5),
    (5, 6),
    (5, 7),
    (5, 8),
    (5, 9),
    (5, 10),
    (5, 11),
    (5, 12),
    (5, 13),
    (5, 14),
    (5, 15),
    (5, 16),
    (5, 17),
    (5, 18),
    (5, 19),
    (5, 20),
    (5, 21),
    (5, 22),
    (5, 23),
    (5, 24),
    (5, 25),
    (5, 26),
    (5, 27),
    (5, 28),
    (5, 29),
    (5, 30),
    (5, 31),
    (6, 0),
    (6, 1),
    (6, 2),
    (6, 3),
    (6, 4),
    (6, 5),
    (6, 6),
    (6, 7),
    (6, 8),
    (6, 9),
    (6, 10),
    (6, 11),
    (6, 12),
    (6, 13),
    (6, 14),
    (6, 15),
    (6, 16),
    (6, 17),
    (6, 18),
    (6, 19),
    (6, 20),
    (6, 21),
    (6, 22),
    (6, 23),
    (6, 24),
    (6, 25),
    (6, 26),
    (6, 27),
);
