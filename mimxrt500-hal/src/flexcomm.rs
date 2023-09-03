pub mod pins;

use crate::pac;

/// The trait implemented by all of the Flexcomm instances.
pub trait Flexcomm: private::Sealed {
    type MainPeripheral;
}

/// Extension trait for Flexcomm instances that support USART.
pub trait WithUsart: Flexcomm {
    type UsartPeripheral;
}
pub mod usart;

/// Extension trait for Flexcomm instances that support SPI.
pub trait WithSpi: Flexcomm {}
pub mod spi;

/// Extension trait for Flexcomm instances that support I2C.
pub trait WithI2c: Flexcomm {}
pub mod i2c;

/// Extension trait for Flexcomm instances that support I2S.
pub trait WithI2s: Flexcomm {}
pub mod i2s;

pub trait FlexcommPeripheral: private::Sealed {
    type PacPeripheral;
    type Clock;
    type Reset;

    type SckPin;
    type RxdSdaMosiDataPin;
    type TxdSclMisoFramePin;
    type CtsSdaxSsel0Pin;
    type RtsSclxSsel1Pin;
    type Ssel2Pin;
    type Ssel3Pin;
}

macro_rules! flexcomm {
    ($name:ident, $periph:ident) => {
        pub struct $name;
        impl Flexcomm for $name {
            type MainPeripheral = pac::$periph;
        }
        impl private::Sealed for $name {}
    };
}

macro_rules! flexcomm_usart {
    ($name:ident, $periph:ident) => {
        impl WithUsart for $name {
            type UsartPeripheral = pac::$periph;
        }
    };
}

flexcomm!(Flexcomm0, FLEXCOMM0);
flexcomm_usart!(Flexcomm0, USART0);
flexcomm!(Flexcomm1, FLEXCOMM1);
flexcomm_usart!(Flexcomm1, USART1);
flexcomm!(Flexcomm2, FLEXCOMM2);
flexcomm_usart!(Flexcomm2, USART2);
flexcomm!(Flexcomm3, FLEXCOMM3);
flexcomm_usart!(Flexcomm3, USART3);
flexcomm!(Flexcomm4, FLEXCOMM4);
flexcomm_usart!(Flexcomm4, USART4);
flexcomm!(Flexcomm5, FLEXCOMM5);
flexcomm_usart!(Flexcomm5, USART5);
flexcomm!(Flexcomm6, FLEXCOMM6);
flexcomm_usart!(Flexcomm6, USART6);
flexcomm!(Flexcomm7, FLEXCOMM7);
flexcomm_usart!(Flexcomm7, USART7);
flexcomm!(Flexcomm8, FLEXCOMM8);
flexcomm_usart!(Flexcomm8, USART8);
flexcomm!(Flexcomm9, FLEXCOMM9);
flexcomm_usart!(Flexcomm9, USART9);
flexcomm!(Flexcomm10, FLEXCOMM10);
flexcomm_usart!(Flexcomm10, USART10);
flexcomm!(Flexcomm11, FLEXCOMM11);
flexcomm_usart!(Flexcomm11, USART11);
flexcomm!(Flexcomm12, FLEXCOMM12);
flexcomm_usart!(Flexcomm12, USART12);
flexcomm!(Flexcomm13, FLEXCOMM13);
flexcomm!(Flexcomm14, FLEXCOMM14);
flexcomm!(Flexcomm15, FLEXCOMM15);
flexcomm!(Flexcomm16, FLEXCOMM16);

mod private {
    pub trait Sealed {}
}
