//! Contains some types representing data structures consumed by the on-chip
//! boot ROM.

/// Describes the shape of the flash configuration header that the Boot ROM
/// expects to find at offset 0x0400 in the flash memory connected to
/// FlexSPI0, if booting from that device.
///
/// The board support package for a board that boots from flash memory on
/// FlexSPI0 should arrange for a suitable value of this type to be placed at
/// 0x0400 in the flash image (0x08000400 if the Flash will be memory mapped)
/// and then the on-chip Boot ROM will retrieve it during boot.
///
/// See _i.MX RT500 Reference Manual_ section 18.6.1.2:
/// FlexSPI NOR Configuration Block(FCB).
#[repr(C)]
#[derive(Clone, Copy)]
pub struct FlexSpiNorFlashConfig {
    /// Common memory configuration info via FlexSPI.
    pub mem_config: FlexSpiMemConfig,
    /// Page size of serial NOR.
    pub page_size: u32,
    /// Sector size of serial NOR.
    pub sector_size: u32,
    /// Clock frequency for IP command.
    pub ip_cmd_serial_clk_freq: u8,
    /// Sector/block size is the same.
    pub is_uniform_block_size: u8,
    /// Data order (D0, D1, D2, D3) is swapped (D1, D0, D3, D2).
    pub is_data_order_swapped: u8,
    /// Reserved for future use.
    pub reserved0: [u8; 1],
    /// Serial NOR flash type: 0/1/2/3.
    pub serial_nor_type: u8,
    /// Need to exit NoCmd mode before other IP command.
    pub need_exit_no_cmd_mode: u8,
    /// Half the Serial Clock for non-read command: true/false.
    pub half_clk_for_non_read_cmd: u8,
    /// Need to Restore NoCmd mode after IP commmand execution.
    pub need_restore_no_cmd_mode: u8,
    /// Block size.
    pub block_size: u32,
    /// Flash state context.
    pub flash_state_ctx: u32,
    /// Reserved for future use.
    pub reserved2: [u32; 10],
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct FlexSpiMemConfig {
    /// Tag, fixed value `0x42464346`.
    pub tag: u32,
    /// Version,[31:24] -'V', [23:16] - Major, [15:8] - Minor, [7:0] - bugfix.
    pub version: u32,
    /// Reserved for future use.
    pub reserved0: u32,
    /// Read Sample Clock Source, valid value: 0/1/3.
    pub read_sample_clk_src: u8,
    /// CS hold time, default value: 3.
    pub cs_hold_time: u8,
    /// CS setup time, default value: 3.
    pub cs_setup_time: u8,
    /// Column Address with, for HyperBus protocol, it is fixed to 3, For
    /// Serial NAND, need to refer to datasheet.
    pub column_address_width: u8,
    /// Device Mode Configure enable flag, 1 - Enable, 0 - Disable.
    pub device_mode_cfg_enable: u8,
    /// Specify the configuration command type:Quad Enable, DPI/QPI/OPI switch,
    /// Generic configuration, etc.
    pub device_mode_type: u8,
    /// Wait time for all configuration commands, unit: 100us, Used for
    /// DPI/QPI/OPI switch or reset command.
    pub wait_time_cfg_commands: u16,
    /// Device mode sequence info, [7:0] - LUT sequence id, [15:8] - LUt
    /// sequence number, [31:16] Reserved.
    pub device_mode_seq: FlexSpiLutSeq,
    /// Argument/Parameter for device configuration.
    pub device_mode_arg: u32,
    /// Configure command Enable Flag, 1 - Enable, 0 - Disable.
    pub config_cmd_enable: u8,
    /// Configure Mode Type, similar as `device_mode_type`.
    pub config_mode_type: [u8; 3],
    /// Sequence info for Device Configuration command, similar as `device_mode_seq`.
    pub config_cmd_seqs: [FlexSpiLutSeq; 3],
    /// Reserved for future use.
    pub reserved1: u32,
    /// Arguments/Parameters for device Configuration commands.
    pub config_cmd_args: [u32; 3],
    /// Reserved for future use.
    pub reserved2: u32,
    /// Controller Misc Options, see Misc feature bit definitions for more details.
    pub controller_misc_option: u32,
    /// Device Type:  See Flash Type Definition for more details.
    pub device_type: u8,
    /// Serial Flash Pad Type: 1 - Single, 2 - Dual, 4 - Quad, 8 - Octal.
    pub serial_flash_pad_type: u8,
    /// Serial Flash Frequencey, device specific definitions, See System Boot
    /// Chapter for more details.
    pub serial_clk_freq: u8,
    /// LUT customization Enable, it is required if the program/erase cannot
    /// be done using 1 LUT sequence, currently, only applicable to HyperFLASH.
    pub lut_custom_seq_enable: u8,
    /// Reserved for future use.
    pub reserved3: [u32; 2],
    /// Size of Flash connected to A1.
    pub serial_flash_a1_size: u32,
    /// Size of Flash connected to A2.
    pub serial_flash_a2_size: u32,
    /// Size of Flash connected to B1.
    pub serial_flash_b1_size: u32,
    /// Size of Flash connected to B2.
    pub serial_flash_b2_size: u32,
    /// CS pad setting override value.
    pub cs_pad_setting_override: u32,
    /// SCLK pad setting override value.
    pub sclk_pad_setting_override: u32,
    /// Data pad setting override value.
    pub data_pad_setting_override: u32,
    /// DQS pad setting override value.
    pub dqs_pad_setting_override: u32,
    /// Timeout threshold for read status command.
    pub timeout_in_ms: u32,
    /// CS deselect interval between two commands.
    pub command_interval: u32,
    /// CLK edge to data valid time for PORT A and PORT B.
    pub data_valid_time: [FlexSpiDllTime; 2],
    /// Busy offset, valid value: 0-31.
    pub busy_offset: u16,
    /// Busy flag polarity, 0 - busy flag is 1 when flash device is busy, 1 -
    /// busy flag is 0 when flash device is busy.
    pub busy_bit_polarity: u16,
    /// Lookup table holds Flash command sequences.
    pub lookup_table: [u32; 64],
    /// Customizable LUT Sequences.
    pub lut_custom_seq: [FlexSpiLutSeq; 12],
    /// Reserved for future use.
    pub reserved4: [u32; 4],
}

impl FlexSpiMemConfig {
    pub const TAG: u32 = 0x42464346;
    pub const VERSION: u32 = 0x56010400;
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct FlexSpiLutSeq {
    /// Sequence Number, valid number: 1-16.
    pub seq_num: u8,
    /// Sequence index, valid number: 0-15.
    pub seq_id: u8,
    /// Reserved for future use.
    pub reserved: u16,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct FlexSpiDllTime {
    /// Data valid time, in terms of 100ps.
    pub time_100ps: u8,
    /// Data valid time, in terms of delay cells.
    pub delay_cells: u8,
}

#[repr(u8)]
#[derive(Clone, Copy, PartialEq, Eq)]
#[allow(non_camel_case_types)]
pub enum FlexspiLutCmd {
    CMD_SDR = 0x01,
    CMD_DDR = 0x21,
    RADDR_SDR = 0x02,
    RADDR_DDR = 0x22,
    CADDR_SDR = 0x03,
    CADDR_DDR = 0x23,
    MODE1_SDR = 0x04,
    MODE1_DDR = 0x24,
    MODE2_SDR = 0x05,
    MODE2_DDR = 0x25,
    MODE4_SDR = 0x06,
    MODE4_DDR = 0x26,
    MODE8_SDR = 0x07,
    MODE8_DDR = 0x27,
    WRITE_SDR = 0x08,
    WRITE_DDR = 0x28,
    READ_SDR = 0x09,
    READ_DDR = 0x29,
    LEARN_SDR = 0x0A,
    LEARN_DDR = 0x2A,
    DATSZ_SDR = 0x0B,
    DATSZ_DDR = 0x2B,
    DUMMY_SDR = 0x0C,
    DUMMY_DDR = 0x2C,
    DUMMY_RWDS_SDR = 0x0D,
    DUMMY_RWDS_DDR = 0x2D,
    JMP_ON_CS = 0x1F,
    STOP_EXE = 0,
}

#[repr(u8)]
#[derive(Clone, Copy, PartialEq, Eq)]
#[allow(non_camel_case_types)]
pub enum FlexspiPad {
    FLEXSPI_1PAD = 0,
    FLEXSPI_2PAD = 1,
    FLEXSPI_4PAD = 2,
    FLEXSPI_8PAD = 3,
}

#[inline(always)]
pub const fn flexspi_lut_seq(
    cmd0: FlexspiLutCmd,
    pad0: FlexspiPad,
    op0: u8,
    cmd1: FlexspiLutCmd,
    pad1: FlexspiPad,
    op1: u8,
) -> u32 {
    let cmd0_raw = ((cmd0 as u32) << 10) & 0xfc00;
    let pad0_raw = ((pad0 as u32) << 8) & 0x300;
    let op0_raw = ((op0 as u32) << 0) & 0xff;
    let cmd1_raw = ((cmd1 as u32) << 26) & 0xfc000000;
    let pad1_raw = ((pad1 as u32) << 24) & 0x3000000;
    let op1_raw = ((op1 as u32) << 16) & 0xff0000;

    cmd0_raw | pad0_raw | op0_raw | cmd1_raw | pad1_raw | op1_raw
}
