
use super::*;
use crate::gpio::pin::*;
use crate::gpio::pin::alternate::*;

/// A set of pins that can be used for a particular Flexcomm instance.
pub trait PinSet<FC: Flexcomm>: private::Sealed {
    type SckPin;
    type RxdSdaMosiDataPin;
    type TxdSclMisoFramePin;
    type CtsSdaSsel0Pin;
    type RtsSclSsel1Pin;
    type Ssel2Pin;
    type Ssel3Pin;
}

/// A [`PinSet`] for Flexcomm0 whose SCK is PIO0_0.
pub struct Flexcomm0Pio0_0;

impl PinSet<Flexcomm0> for Flexcomm0Pio0_0 {
    type SckPin = Pin<PIO0_0, AltFunc1Digital>;
    type RxdSdaMosiDataPin = Pin<PIO0_2, AltFunc1Digital>;
    type TxdSclMisoFramePin = Pin<PIO0_1, AltFunc1Digital>;
    type CtsSdaSsel0Pin = Pin<PIO0_3, AltFunc1Digital>;
    type RtsSclSsel1Pin = Pin<PIO0_4, AltFunc1Digital>;
    type Ssel2Pin = Pin<PIO0_5, AltFunc1Digital>;
    type Ssel3Pin = Pin<PIO0_6, AltFunc1Digital>;
}
impl private::Sealed for Flexcomm0Pio0_0 {}

/// A [`PinSet`] for Flexcomm1 whose SCK is PIO0_7.
pub struct Flexcomm1Pio0_7;

impl PinSet<Flexcomm1> for Flexcomm1Pio0_7 {
    type SckPin = Pin<PIO0_7, AltFunc1Digital>;
    type RxdSdaMosiDataPin = Pin<PIO0_9, AltFunc1Digital>;
    type TxdSclMisoFramePin = Pin<PIO0_8, AltFunc1Digital>;
    type CtsSdaSsel0Pin = Pin<PIO0_10, AltFunc1Digital>;
    type RtsSclSsel1Pin = Pin<PIO0_11, AltFunc1Digital>;
    type Ssel2Pin = Pin<PIO0_12, AltFunc1Digital>;
    type Ssel3Pin = Pin<PIO0_13, AltFunc1Digital>;
}
impl private::Sealed for Flexcomm1Pio0_7 {}

/// A [`PinSet`] for Flexcomm1 whose SCK is PIO4_4.
pub struct Flexcomm1Pio4_4;

impl PinSet<Flexcomm1> for Flexcomm1Pio4_4 {
    type SckPin = Pin<PIO4_4, AltFunc5Digital>;
    type RxdSdaMosiDataPin = Pin<PIO4_6, AltFunc5Digital>;
    type TxdSclMisoFramePin = Pin<PIO4_5, AltFunc5Digital>;
    type CtsSdaSsel0Pin = Pin<PIO0_10, AltFunc1Digital>;
    type RtsSclSsel1Pin = Pin<PIO0_11, AltFunc1Digital>;
    type Ssel2Pin = Pin<PIO0_12, AltFunc1Digital>;
    type Ssel3Pin = Pin<PIO0_13, AltFunc1Digital>;
}
impl private::Sealed for Flexcomm1Pio4_4 {}
