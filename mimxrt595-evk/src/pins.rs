use crate::hal::gpio::pin;
use crate::hal::gpio::pin::{
    alternate::*, input::InputFloating, output::OutputPushPull, Pin, Unknown,
};

/// Container for the three pins connected to the on-board RGB LED.
pub struct RgbLedPins {
    /// The pin connected to the red LED. Convert to [`RgbLedRedPin`] to
    /// activate output mode.
    pub red: Pin<pin::PIO0_14, Unknown>,
    /// The pin connected to the green LED. Convert to [`RgbLedGreenPin`] to
    /// activate output mode.
    pub green: Pin<pin::PIO1_0, Unknown>,
    /// The pin connected to the blue LED. Convert to [`RgbLedBluePin`] to
    /// activate output mode.
    pub blue: Pin<pin::PIO3_17, Unknown>,
}

/// The pin type for the red component of the on-board RGB LED, configured as
/// a push-pull output.
pub type RgbLedRedPin = Pin<pin::PIO0_14, OutputPushPull>;

/// The pin type for the green component of the on-board RGB LED, configured as
/// a push-pull output.
pub type RgbLedGreenPin = Pin<pin::PIO1_0, OutputPushPull>;

/// The pin type for the blue component of the on-board RGB LED, configured as
/// a push-pull output.
pub type RgbLedBluePin = Pin<pin::PIO3_17, OutputPushPull>;

/// Container for the two pins that are connected to push switches near the
/// top of the board.
pub struct UserButtonPins {
    /// The pin connected to the button labelled "NMI", also known as SW1.
    ///
    /// The switch is normally open with a pull-up resistor, and pulls the
    /// pin to ground when pressed.
    ///
    /// Convert to [`NmiButtonPin`] to activate input mode.
    pub nmi: Pin<pin::PIO0_25, Unknown>,
    /// The pin connected to the button labelled "IRQ", also known as SW2.
    ///
    /// The switch is normally open with a pull-up resistor, and pulls the
    /// pin to ground when pressed.
    ///
    /// Jumper JP33 must be closed to connect this pin with the push switch.
    ///
    /// Convert to [`IrqButtonPin`] to activate input mode.
    pub irq: Pin<pin::PIO0_10, Unknown>,
}

/// The pin type for the button labelled "NMI" (SW1) near the top of the board.
pub type NmiButtonPin = Pin<pin::PIO0_25, InputFloating>;

/// The pin type for the button labelled "IRQ" (SW2) near the top of the board.
pub type IrqButtonPin = Pin<pin::PIO0_10, InputFloating>;

/// Container for the pins that are connected to the NXP FXOS8700CQ
/// accelerometer.
pub struct AccelerometerPins {
    /// The pin connected to the chip's I2C SCL input.
    ///
    /// Convert to [`AccelerometerI2cSclPin`] to select alternate function 1,
    /// which connects this to Flexcomm4.
    pub i2c_scl: Pin<pin::PIO0_29, Unknown>,

    /// The pin connected to the chip's I2C SDA pin.
    ///
    /// Convert to [`AccelerometerI2cSdaPin`] to select alternate function 1,
    /// which connects this to Flexcomm4.
    pub i2c_sda: Pin<pin::PIO0_30, Unknown>,

    /// The pin connected to the chip's interrupt output.
    ///
    /// Convert to [`AccelerometerInterruptPin`] to activate input mode.
    pub interrupt: Pin<pin::PIO0_22, Unknown>,
}

/// The pin type for the on-board accelerometer's SCL signal.
pub type AccelerometerI2cSclPin = Pin<pin::PIO0_29, AltFunc1Digital>;

/// The pin type for the on-board accelerometer's SDA signal.
pub type AccelerometerI2cSdaPin = Pin<pin::PIO0_30, AltFunc1Digital>;

/// The pin type for the on-board accelerometer's interrupt pin.
pub type AccelerometerInterruptPin = Pin<pin::PIO0_22, InputFloating>;

/// Container for the pins that are connected either to the Cirrus Logic WM8904
/// audio codec or to the pair of audio amplifiers, depending on the settings
/// of jumpers JP6, JP7, JP8, JP27, JP28, and JP29.
///
/// The I3C signals are also connected to the board's I3C connector.
pub struct AudioPins {
    /// The pin connected to the chip's I3C SDA pin.
    pub i3c_sda: Pin<pin::PIO2_30, Unknown>,

    /// The pin connected to the chip's I3C SCL input.
    pub i3c_scl: Pin<pin::PIO2_29, Unknown>,

    /// The pin connected to the chip's I2S BCLK pin.
    pub i2s_bclk: Pin<pin::PIO0_7, Unknown>,

    /// The pin connected to the chip's I2S DAI pin.
    pub i2s_dai: Pin<pin::PIO0_9, Unknown>,

    /// The pin connected to the chip's I2S DAO pin.
    pub i2s_dao: Pin<pin::PIO0_23, Unknown>,

    /// The pin connected to the chip's I2S WS pin.
    pub i2s_ws: Pin<pin::PIO0_8, Unknown>,

    /// The pin connected to the chip's MCLK pin.
    pub mclk: Pin<pin::PIO1_10, Unknown>,

    /// The pin connected to the chip's "GPIO 1 / IRQ" pin.
    pub alt_int: Pin<pin::PIO0_0, Unknown>,
}

/// Container for the pins that are connected to the "FlexIO" connector on
/// the bottom of the board (J43).
///
/// Some of these also connect to pins on the Arduino Uno-style headers at
/// the bottom left of the board.
pub struct FlexIoPins {
    pub d0: Pin<pin::PIO4_20, Unknown>,
    pub d1: Pin<pin::PIO4_21, Unknown>,
    pub d2: Pin<pin::PIO4_22, Unknown>,
    pub d3: Pin<pin::PIO4_23, Unknown>,
    pub d4: Pin<pin::PIO4_24, Unknown>,
    pub d5: Pin<pin::PIO4_25, Unknown>,
    pub d6: Pin<pin::PIO4_26, Unknown>,
    pub d7: Pin<pin::PIO4_27, Unknown>,
    pub d8: Pin<pin::PIO4_28, Unknown>,
    pub d9: Pin<pin::PIO4_29, Unknown>,
    pub d10: Pin<pin::PIO4_30, Unknown>,
    pub d11: Pin<pin::PIO4_31, Unknown>,
    pub d12: Pin<pin::PIO5_0, Unknown>,
    pub d13: Pin<pin::PIO5_1, Unknown>,
    pub d14: Pin<pin::PIO5_2, Unknown>,
    pub d15: Pin<pin::PIO5_3, Unknown>,
}

/// Container for the pins that are connected to the UART bridge through the
/// Link2 debug probe.
///
/// These should be used with Flexcomm0 by configuring the pins for alternate
/// function 1. The Link2 virtual COM port is documented to use 115200 baud,
/// no parity, 8-bit data, 1 stop bit, no flow control.
pub struct IspUartPins {
    /// The TXD pin. Convert to [`IspUartTxdPin`] to activate function 1
    /// to connect to Flexcomm0.
    pub txd: Pin<pin::PIO0_1, Unknown>,
    /// The RXD pin. Convert to [`IspUartRxdPin`] to activate function 1
    /// to connect to Flexcomm0.
    pub rxd: Pin<pin::PIO0_2, Unknown>,
    /// The CTS pin. Convert to [`IspUartCtsPin`] to activate function 1
    /// to connect to Flexcomm0.
    pub cts: Pin<pin::PIO0_3, Unknown>,
    /// The RTS pin. Convert to [`IspUartRtsPin`] to activate function 1
    /// to connect to Flexcomm0.
    pub rts: Pin<pin::PIO0_4, Unknown>,
}

/// The pin type for the ISP/Link2 TXD pin, configured to connect to Flexcomm0.
pub type IspUartTxdPin = Pin<pin::PIO0_1, AltFunc1Digital>;

/// The pin type for the ISP/Link2 RXD pin, configured to connect to Flexcomm0.
pub type IspUartRxdPin = Pin<pin::PIO0_2, AltFunc1Digital>;

/// The pin type for the ISP/Link2 CTS pin, configured to connect to Flexcomm0.
pub type IspUartCtsPin = Pin<pin::PIO0_3, AltFunc1Digital>;

/// The pin type for the ISP/Link2 RTS pin, configured to connect to Flexcomm0.
pub type IspUartRtsPin = Pin<pin::PIO0_4, AltFunc1Digital>;

/// Container for the pins that are connected to both the MMC chip and the
/// SD card socket.
///
/// Due to the shared signals, callers must typically decide between using
/// either the MMC chip or the SD card socket but not both. Use the MMC chip's
/// reset pin to disable it if you intend to use the SD card socket.
pub struct MmcPins {
    /// Clock pin shared between MMC and SD card.
    pub clk: Pin<pin::PIO1_30, Unknown>,
    /// Command flag pin shared between MMC and SD card.
    pub cmd: Pin<pin::PIO1_31, Unknown>,

    /// Data pin shared between MMC and SD card.
    pub d0: Pin<pin::PIO2_0, Unknown>,
    /// Data pin shared between MMC and SD card.
    pub d1: Pin<pin::PIO2_1, Unknown>,
    /// Data pin shared between MMC and SD card.
    pub d2: Pin<pin::PIO2_2, Unknown>,
    /// Data pin shared between MMC and SD card.
    pub d3: Pin<pin::PIO2_3, Unknown>,
    /// Data pin used only for MMC.
    pub d4: Pin<pin::PIO2_5, Unknown>,
    /// Data pin used only for MMC.
    pub d5: Pin<pin::PIO2_6, Unknown>,
    /// Data pin used only for MMC.
    pub d6: Pin<pin::PIO2_7, Unknown>,
    /// Data pin used only for MMC.
    pub d7: Pin<pin::PIO2_8, Unknown>,

    /// SD card detect signal. Active low.
    pub sd_cd: Pin<pin::PIO2_9, Unknown>,

    /// MMC chip reset signal. Active low.
    pub mmc_reset: Pin<pin::PIO2_10, Unknown>,

    /// MMC chip DS signal or SD card power enable.
    pub mmc_ds_sd_power_enable: Pin<pin::PIO2_4, Unknown>,
}

/// The pin type for the MMC/SD Clock pin, configured for SD0 function.
pub type MmcClkPin = Pin<pin::PIO1_30, AltFunc1Digital>;
/// The pin type for the MMC/SD Command pin, configured for SD0 function.
pub type MmcCmdPin = Pin<pin::PIO1_31, AltFunc1Digital>;
/// The pin type for the MMC/SD D0 pin, configured for SD0 function.
pub type MmcD0Pin = Pin<pin::PIO2_0, AltFunc1Digital>;
/// The pin type for the MMC/SD D1 pin, configured for SD0 function.
pub type MmcD1Pin = Pin<pin::PIO2_1, AltFunc1Digital>;
/// The pin type for the MMC/SD D2 pin, configured for SD0 function.
pub type MmcD2Pin = Pin<pin::PIO2_2, AltFunc1Digital>;
/// The pin type for the MMC/SD D3 pin, configured for SD0 function.
pub type MmcD3Pin = Pin<pin::PIO2_3, AltFunc1Digital>;
/// The pin type for the MMC/SD D4 pin, configured for SD0 function.
pub type MmcD4Pin = Pin<pin::PIO2_5, AltFunc1Digital>;
/// The pin type for the MMC/SD D5 pin, configured for SD0 function.
pub type MmcD5Pin = Pin<pin::PIO2_6, AltFunc1Digital>;
/// The pin type for the MMC/SD D6 pin, configured for SD0 function.
pub type MmcD6Pin = Pin<pin::PIO2_7, AltFunc1Digital>;
/// The pin type for the MMC/SD D7 pin, configured for SD0 function.
pub type MmcD7Pin = Pin<pin::PIO2_8, AltFunc1Digital>;
/// The pin type for the MMC/SD Clock pin, configured for SD0 function.
pub type MmcSdCdPin = Pin<pin::PIO2_9, AltFunc1Digital>;
/// The pin type for the MMC/SD Clock pin, configured for SD0 function.
pub type MmcMmcResetPin = Pin<pin::PIO2_10, AltFunc1Digital>;
/// The pin type for the MMC/SD Clock pin, configured for SD0 function.
pub type MmcMmcDsSdPowerEnablePin = Pin<pin::PIO2_4, AltFunc1Digital>;

/// Container for the pins that are connected to the PSRAM chip on FlexSPI1.
pub struct PsramPins {
    /// The pin connected to the PSRAM chip select. Active low.
    pub cs: Pin<pin::PIO4_18, Unknown>,
    /// The pin connected to the PSRAM's reset pin. Active low.
    pub reset: Pin<pin::PIO0_28, Unknown>,
    /// The pin connected to the PSRAM's clock pin.
    pub sclk: Pin<pin::PIO4_11, Unknown>,
    /// The pin that would connect to the inverted SCLK, but this pin is not
    /// connected by default. (Must populate a 0 ohm resistor to
    /// activate it.)
    pub sclk_n: Pin<pin::PIO4_17, Unknown>,
    /// The pin connected to the PSRAM's DQS pin.
    pub dqs: Pin<pin::PIO4_16, Unknown>,

    /// The pin connected to the PSRAM's ADQ0 pin.
    pub d0: Pin<pin::PIO4_12, Unknown>,
    /// The pin connected to the PSRAM's ADQ1 pin.
    pub d1: Pin<pin::PIO4_13, Unknown>,
    /// The pin connected to the PSRAM's ADQ2 pin.
    pub d2: Pin<pin::PIO4_14, Unknown>,
    /// The pin connected to the PSRAM's ADQ3 pin.
    pub d3: Pin<pin::PIO4_15, Unknown>,
    /// The pin connected to the PSRAM's ADQ4 pin.
    pub d4: Pin<pin::PIO5_15, Unknown>,
    /// The pin connected to the PSRAM's ADQ5 pin.
    pub d5: Pin<pin::PIO5_16, Unknown>,
    /// The pin connected to the PSRAM's ADQ6 pin.
    pub d6: Pin<pin::PIO5_17, Unknown>,
    /// The pin connected to the PSRAM's ADQ7 pin.
    pub d7: Pin<pin::PIO5_18, Unknown>,
}

/// The pin type for the PSRAM chip select pin, connected to FlexSPI1.
pub type PsramCsPin = Pin<pin::PIO4_18, AltFunc2Digital>;
/// The pin type for the PSRAM reset pin, configured as a push-pull output.
pub type PsramResetPin = Pin<pin::PIO0_28, OutputPushPull>;
/// The pin type for the PSRAM chip select pin, connected to FlexSPI1.
pub type PsramSclkPin = Pin<pin::PIO4_11, AltFunc2Digital>;
/// The pin type for the PSRAM inverted chip select pin, connected to FlexSPI1.
pub type PsramSclkNPin = Pin<pin::PIO4_17, AltFunc3Digital>;
/// The pin type for the PSRAM DQS pin, connected to FlexSPI1.
pub type PsramDqsPin = Pin<pin::PIO4_16, AltFunc2Digital>;
/// The pin type for the PSRAM D0 pin, connected to FlexSPI1.
pub type PsramD0Pin = Pin<pin::PIO4_12, AltFunc2Digital>;
/// The pin type for the PSRAM D1 pin, connected to FlexSPI1.
pub type PsramD1Pin = Pin<pin::PIO4_13, AltFunc2Digital>;
/// The pin type for the PSRAM D2 pin, connected to FlexSPI1.
pub type PsramD2Pin = Pin<pin::PIO4_14, AltFunc2Digital>;
/// The pin type for the PSRAM D3 pin, connected to FlexSPI1.
pub type PsramD3Pin = Pin<pin::PIO4_15, AltFunc2Digital>;
/// The pin type for the PSRAM D4 pin, connected to FlexSPI1.
pub type PsramD4Pin = Pin<pin::PIO5_15, AltFunc2Digital>;
/// The pin type for the PSRAM D5 pin, connected to FlexSPI1.
pub type PsramD5Pin = Pin<pin::PIO5_16, AltFunc2Digital>;
/// The pin type for the PSRAM D6 pin, connected to FlexSPI1.
pub type PsramD6Pin = Pin<pin::PIO5_17, AltFunc2Digital>;
/// The pin type for the PSRAM D7 pin, connected to FlexSPI1.
pub type PsramD7Pin = Pin<pin::PIO5_18, AltFunc2Digital>;
