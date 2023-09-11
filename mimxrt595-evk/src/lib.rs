//! A board support package for MIMXRT595-EVK, the official evaluation kit
//! for the NXP i.MX RT500 family of microcontrollers.
//!
//! It also includes a `memory.x` file intended for inclusion into the linker
//! script provided by crate `cortex-m-rt`.
//!
//! If you are building an application targeting this board, you will need to
//! arrange for your flash image to contain a suitable flash control block
//! and initial vector table so that the on-chip boot ROM will consider it
//! eligable for use as boot media. You can get a default implementation of
//! that by linking the separate crate `mimxrt595-evk-bootstub` into your
//! program, or customize what's generated using the macros in crate
//! `mimxrt500-bootstub`.
#![no_std]

/// The RT500 HAL crate, re-exported for convenience.
pub use mimxrt500_hal as hal;

/// Additional types used inside [`Pins`].
pub mod pins;

pub mod bootstub;

/// Wraps the HAL-level Pins with alternative pin names that match the
/// documented connections on the evaluation kit board.
///
/// Where possible the pins are grouped into nested structs based on the
/// physical packages they are connected to, for convenience when passing all
/// of the related pins together to a function that initializes a device driver.
/// Some pins are connected to multiple on-board devices, so this categorization
/// is not comprehensive.
///
/// To avoid emitting mode-setting code for pins that might never actually be
/// used, the pins are still provided initially in unknown mode, and can then
/// be converted into the appropriate mode by converting into one of the
/// provided type aliases that specify the typical mode used for the device
/// in question. For example, the following sets the red LED pin to be a GPIO
/// output, with the `.into` method performing the mode change automatically:
///
/// ```rust
/// let red_pin: mimxrt595_evk::pins::RgbLedRedPin = pins.rgb_led.red.into();
/// ```
pub struct Pins {
    /// The pins connected to the on-board accelerometer chip.
    pub accelerometer: pins::AccelerometerPins,

    /// The pins connected to either of the on-board audio chips.
    pub audio: pins::AudioPins,

    /// The two pins connected to the "user buttons" at the top of the board,
    /// as described in user guide section 8.8.
    pub buttons: pins::UserButtonPins,

    /// The pins connected to the "FlexIO" connector on the bottom of the board.
    pub flex_io: pins::FlexIoPins,

    /// The pins connected to the Link2 virtual COM port.
    pub isp_uart: pins::IspUartPins,

    /// The pins connected to both the MMC chip and the SD card socket.
    pub mmc: pins::MmcPins,

    /// The pins connected to the on-board PSRAM chip.
    pub psram: pins::PsramPins,

    /// The three pins connected to the on-board RGB LED, as described in
    /// user guide section 8.7.
    pub rgb_led: pins::RgbLedPins,
}

impl Pins {
    /// Wrap the HAL-level pins object with the board-specific pins API.
    #[inline(always)]
    pub fn wrap(hal_pins: hal::gpio::Pins) -> Self {
        Pins {
            rgb_led: pins::RgbLedPins {
                red: hal_pins.pio0_14,
                green: hal_pins.pio1_0,
                blue: hal_pins.pio3_17,
            },
            buttons: pins::UserButtonPins {
                nmi: hal_pins.pio0_25,
                irq: hal_pins.pio0_10,
            },
            accelerometer: pins::AccelerometerPins {
                i2c_scl: hal_pins.pio0_29,
                i2c_sda: hal_pins.pio0_30,
                interrupt: hal_pins.pio0_22,
            },
            audio: pins::AudioPins {
                i3c_sda: hal_pins.pio2_30,
                i3c_scl: hal_pins.pio2_29,
                i2s_bclk: hal_pins.pio0_7,
                i2s_dai: hal_pins.pio0_9,
                i2s_dao: hal_pins.pio0_23,
                i2s_ws: hal_pins.pio0_8,
                mclk: hal_pins.pio1_10,
                alt_int: hal_pins.pio0_0,
            },
            flex_io: pins::FlexIoPins {
                d0: hal_pins.pio4_20,
                d1: hal_pins.pio4_21,
                d2: hal_pins.pio4_22,
                d3: hal_pins.pio4_23,
                d4: hal_pins.pio4_24,
                d5: hal_pins.pio4_25,
                d6: hal_pins.pio4_26,
                d7: hal_pins.pio4_27,
                d8: hal_pins.pio4_28,
                d9: hal_pins.pio4_29,
                d10: hal_pins.pio4_30,
                d11: hal_pins.pio4_31,
                d12: hal_pins.pio5_0,
                d13: hal_pins.pio5_1,
                d14: hal_pins.pio5_2,
                d15: hal_pins.pio5_3,
            },
            isp_uart: pins::IspUartPins {
                txd: hal_pins.pio0_1,
                rxd: hal_pins.pio0_2,
                cts: hal_pins.pio0_3,
                rts: hal_pins.pio0_4,
            },
            mmc: pins::MmcPins {
                clk: hal_pins.pio1_30,
                cmd: hal_pins.pio1_31,
                d0: hal_pins.pio2_0,
                d1: hal_pins.pio2_1,
                d2: hal_pins.pio2_2,
                d3: hal_pins.pio2_3,
                d4: hal_pins.pio2_5,
                d5: hal_pins.pio2_6,
                d6: hal_pins.pio2_7,
                d7: hal_pins.pio2_8,
                sd_cd: hal_pins.pio2_9,
                mmc_reset: hal_pins.pio2_10,
                mmc_ds_sd_power_enable: hal_pins.pio2_4,
            },
            psram: pins::PsramPins {
                cs: hal_pins.pio4_18,
                reset: hal_pins.pio0_28,
                sclk: hal_pins.pio4_11,
                sclk_n: hal_pins.pio4_17,
                dqs: hal_pins.pio4_16,
                d0: hal_pins.pio4_12,
                d1: hal_pins.pio4_13,
                d2: hal_pins.pio4_14,
                d3: hal_pins.pio4_15,
                d4: hal_pins.pio5_15,
                d5: hal_pins.pio5_16,
                d6: hal_pins.pio5_17,
                d7: hal_pins.pio5_18,
            },
        }
    }
}

use mimxrt500_hal::bootrom::{
    flexspi_lut_seq, FlexSpiDllTime, FlexSpiLutSeq, FlexSpiMemConfig, FlexSpiNorFlashConfig,
    FlexspiLutCmd::*, FlexspiPad::*,
};

#[used]
#[no_mangle]
#[link_section = ".boot_hdr"]
static NOR_FLASH_CONFIG: FlexSpiNorFlashConfig = FlexSpiNorFlashConfig {
    mem_config: FlexSpiMemConfig {
        tag: FlexSpiMemConfig::TAG,
        version: FlexSpiMemConfig::VERSION,
        reserved0: 0,
        read_sample_clk_src: 3,
        cs_hold_time: 3,
        cs_setup_time: 3,
        column_address_width: 0,
        device_mode_cfg_enable: 1,
        device_mode_type: 2,
        wait_time_cfg_commands: 1,
        device_mode_seq: FlexSpiLutSeq {
            seq_num: 1,
            seq_id: 6,
            reserved: 0,
        },
        device_mode_arg: 2, // Enable OPI DDR mode
        config_cmd_enable: 0,
        config_mode_type: [0; 3],
        config_cmd_seqs: [
            FlexSpiLutSeq {
                seq_num: 0,
                seq_id: 0,
                reserved: 0,
            },
            FlexSpiLutSeq {
                seq_num: 0,
                seq_id: 0,
                reserved: 0,
            },
            FlexSpiLutSeq {
                seq_num: 0,
                seq_id: 0,
                reserved: 0,
            },
        ],
        reserved1: 0,
        config_cmd_args: [0; 3],
        reserved2: 0,
        controller_misc_option: 0x50,
        device_type: 0x01,
        serial_flash_pad_type: 8,
        serial_clk_freq: 3,
        lut_custom_seq_enable: 0,
        reserved3: [0; 2],
        serial_flash_a1_size: 64 * 1024 * 1024,
        serial_flash_a2_size: 0,
        serial_flash_b1_size: 0,
        serial_flash_b2_size: 0,
        cs_pad_setting_override: 0,
        sclk_pad_setting_override: 0,
        data_pad_setting_override: 0,
        dqs_pad_setting_override: 0,
        timeout_in_ms: 0,
        command_interval: 0,
        data_valid_time: [FlexSpiDllTime {
            delay_cells: 0,
            time_100ps: 0,
        }; 2],
        busy_offset: 0,
        busy_bit_polarity: 0,
        lookup_table: [
            // Read
            flexspi_lut_seq(CMD_DDR, FLEXSPI_8PAD, 0xee, CMD_DDR, FLEXSPI_8PAD, 0x11),
            flexspi_lut_seq(RADDR_DDR, FLEXSPI_8PAD, 0x20, DUMMY_DDR, FLEXSPI_8PAD, 0x04),
            flexspi_lut_seq(READ_DDR, FLEXSPI_8PAD, 0x04, STOP_EXE, FLEXSPI_1PAD, 0x00),
            0,
            // Read status SPI
            flexspi_lut_seq(CMD_SDR, FLEXSPI_1PAD, 0x05, READ_SDR, FLEXSPI_1PAD, 0x04),
            0,
            0,
            0,
            // Read status OPI
            flexspi_lut_seq(CMD_DDR, FLEXSPI_8PAD, 0x05, CMD_DDR, FLEXSPI_8PAD, 0xFA),
            flexspi_lut_seq(RADDR_DDR, FLEXSPI_8PAD, 0x20, DUMMY_DDR, FLEXSPI_8PAD, 0x04),
            flexspi_lut_seq(READ_DDR, FLEXSPI_8PAD, 0x04, STOP_EXE, FLEXSPI_1PAD, 0x00),
            0,
            // Write enable
            flexspi_lut_seq(CMD_SDR, FLEXSPI_1PAD, 0x06, STOP_EXE, FLEXSPI_1PAD, 0x00),
            0,
            0,
            0,
            // Write enable - OPI
            flexspi_lut_seq(CMD_DDR, FLEXSPI_8PAD, 0x06, CMD_DDR, FLEXSPI_8PAD, 0xF9),
            0,
            0,
            0,
            // Erase Sector
            flexspi_lut_seq(CMD_DDR, FLEXSPI_8PAD, 0x21, CMD_DDR, FLEXSPI_8PAD, 0xDE),
            flexspi_lut_seq(RADDR_DDR, FLEXSPI_8PAD, 0x20, STOP_EXE, FLEXSPI_1PAD, 0x00),
            0,
            0,
            // Enable OPI DDR mode
            flexspi_lut_seq(CMD_SDR, FLEXSPI_1PAD, 0x72, CMD_SDR, FLEXSPI_1PAD, 0x00),
            flexspi_lut_seq(CMD_SDR, FLEXSPI_1PAD, 0x00, CMD_SDR, FLEXSPI_1PAD, 0x00),
            flexspi_lut_seq(CMD_SDR, FLEXSPI_1PAD, 0x00, WRITE_SDR, FLEXSPI_1PAD, 0x01),
            0,
            // Unused
            0,
            0,
            0,
            0,
            // Erase block
            flexspi_lut_seq(CMD_DDR, FLEXSPI_8PAD, 0xDC, CMD_DDR, FLEXSPI_8PAD, 0x23),
            flexspi_lut_seq(RADDR_DDR, FLEXSPI_8PAD, 0x20, STOP_EXE, FLEXSPI_1PAD, 0x00),
            0,
            0,
            // Page program
            flexspi_lut_seq(CMD_DDR, FLEXSPI_8PAD, 0x12, CMD_DDR, FLEXSPI_8PAD, 0xED),
            flexspi_lut_seq(RADDR_DDR, FLEXSPI_8PAD, 0x20, WRITE_DDR, FLEXSPI_8PAD, 0x04),
            0,
            0,
            // Unused
            0,
            0,
            0,
            0,
            // Erase chip
            flexspi_lut_seq(CMD_DDR, FLEXSPI_8PAD, 0x60, CMD_DDR, FLEXSPI_8PAD, 0x9F),
            0,
            0,
            0,
            // Remainder is unused
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
        ],
        lut_custom_seq: [FlexSpiLutSeq {
            seq_num: 0,
            seq_id: 0,
            reserved: 0,
        }; 12],
        reserved4: [0; 4],
    },
    page_size: 256,
    sector_size: 4096,
    ip_cmd_serial_clk_freq: 1,
    is_uniform_block_size: 0,
    is_data_order_swapped: 0,
    reserved0: [0; 1],
    serial_nor_type: 2,
    need_exit_no_cmd_mode: 0,
    half_clk_for_non_read_cmd: 0,
    need_restore_no_cmd_mode: 0,
    block_size: 64 * 1024,
    flash_state_ctx: 0x07008200,
    reserved2: [0; 10],
};
