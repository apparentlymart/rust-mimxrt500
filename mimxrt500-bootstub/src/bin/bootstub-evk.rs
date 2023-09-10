//! A standalone boot stub intended for use with the mimxrt595-evk evaluation
//! board.
//!
//! This generates some code suitable for writing into the first page of
//! the flash memory connected to FlexSPI0 on the evaluation board, which is
//! where the main chip's boot ROM expects to find a NOR flash header and
//! an initial vector table. Along with those two prerequisites, the boot
//! stub image also includes a small amount of code which arranges to run
//! a `cortex-m-rt`-style program placed at offset 64KiB in the flash
//! memory.
//!
//! Once this stub is present, you can build your real application using
//! `cortex-m-rt` as normal and arrange for your linker to place its
//! vector table at offset `0x10000` (64KiB) in flash, so that the stub
//! code can find it and jump into it.
#![no_std]
#![no_main]
#![feature(start)]

use core::{panic::PanicInfo, arch::asm};

use mimxrt500_bootstub::{
    bootrom::{
        flexspi_lut_seq, FlexSpiDllTime, FlexSpiLutSeq, FlexSpiMemConfig, FlexSpiNorFlashConfig,
        FlexspiLutCmd::*, FlexspiPad::*,
    },
    bootstub_standalone, fcb,
};

bootstub_standalone!(0x10000);
fcb!(FlexSpiNorFlashConfig {
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
});

#[panic_handler]
fn panic_halt(_arg: &PanicInfo) -> ! {
    loop {
        unsafe { asm!("wfi") };
    }
}

