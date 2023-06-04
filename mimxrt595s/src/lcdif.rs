#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 0x1240],
    #[doc = "0x1240 - Frame Buffer Configuration 0"]
    pub frame_buffer_config0: FRAME_BUFFER_CONFIG0,
    _reserved1: [u8; 0x1c],
    #[doc = "0x1260 - Starting Address of the Frame Buffer"]
    pub frame_buffer_address0: FRAME_BUFFER_ADDRESS0,
    _reserved2: [u8; 0x1c],
    #[doc = "0x1280 - Stride of the Frame Buffer in Bytes"]
    pub frame_buffer_stride0: FRAME_BUFFER_STRIDE0,
    _reserved3: [u8; 0xdc],
    #[doc = "0x1360 - Configuration for Dithering"]
    pub display_dither_config0: DISPLAY_DITHER_CONFIG0,
    _reserved4: [u8; 0x1c],
    #[doc = "0x1380 - Dither Table Low"]
    pub display_dither_table_low0: DISPLAY_DITHER_TABLE_LOW0,
    _reserved5: [u8; 0x1c],
    #[doc = "0x13a0 - Dither Table High"]
    pub display_dither_table_high0: DISPLAY_DITHER_TABLE_HIGH0,
    _reserved6: [u8; 0x1c],
    #[doc = "0x13c0 - Panel Configuration"]
    pub panel_config0: PANEL_CONFIG0,
    _reserved7: [u8; 0x1c],
    #[doc = "0x13e0 - Timing for Hardware Panel Sequencing"]
    pub panel_timing0: PANEL_TIMING0,
    _reserved8: [u8; 0x1c],
    #[doc = "0x1400 - Horizontal Total and Display End Counters"]
    pub hdisplay0: HDISPLAY0,
    _reserved9: [u8; 0x1c],
    #[doc = "0x1420 - Horizontal Sync Counters"]
    pub hsync0: HSYNC0,
    _reserved10: [u8; 0x5c],
    #[doc = "0x1480 - Vertical Total and Display End Counters"]
    pub vdisplay0: VDISPLAY0,
    _reserved11: [u8; 0x1c],
    #[doc = "0x14a0 - Vertical Sync Counters"]
    pub vsync0: VSYNC0,
    _reserved12: [u8; 0x1c],
    #[doc = "0x14c0 - Current x,y Location of Display Controller"]
    pub display_current_location0: DISPLAY_CURRENT_LOCATION0,
    _reserved13: [u8; 0x1c],
    #[doc = "0x14e0 - Index into Gamma Table"]
    pub gamma_index0: GAMMA_INDEX0,
    _reserved14: [u8; 0x1c],
    #[doc = "0x1500 - Translation Values for the Gamma Table"]
    pub gamma_data0: GAMMA_DATA0,
    _reserved15: [u8; 0x1c],
    #[doc = "0x1520 - Configuration for the Cursor"]
    pub cursor_config: CURSOR_CONFIG,
    _reserved16: [u8; 0x0c],
    #[doc = "0x1530 - Address of the Cursor Shape"]
    pub cursor_address: CURSOR_ADDRESS,
    _reserved17: [u8; 0x0c],
    #[doc = "0x1540 - Location of the cursor on the owning display"]
    pub cursor_location: CURSOR_LOCATION,
    _reserved18: [u8; 0x0c],
    #[doc = "0x1550 - Background Color for Masked Cursors"]
    pub cursor_background: CURSOR_BACKGROUND,
    _reserved19: [u8; 0x0c],
    #[doc = "0x1560 - Foreground Color for Masked Cursors"]
    pub cursor_foreground: CURSOR_FOREGROUND,
    _reserved20: [u8; 0x9c],
    #[doc = "0x1600 - Display Interrupt"]
    pub display_intr: DISPLAY_INTR,
    _reserved21: [u8; 0x0c],
    #[doc = "0x1610 - Interrupt Enable for Display_0 (and Display_1 if present)"]
    pub display_intr_enable: DISPLAY_INTR_ENABLE,
    _reserved22: [u8; 0x0c],
    #[doc = "0x1620 - DBI Configuration 0"]
    pub dbi_config0: DBI_CONFIG0,
    _reserved23: [u8; 0x1c],
    #[doc = "0x1640 - Reset DBI Interface to Idle State"]
    pub dbi_if_reset0: DBI_IF_RESET0,
    _reserved24: [u8; 0x1c],
    #[doc = "0x1660 - DBI Write Characteristics 1"]
    pub dbi_wr_char10: DBI_WR_CHAR10,
    _reserved25: [u8; 0x1c],
    #[doc = "0x1680 - DBI Write Characteristics 2"]
    pub dbi_wr_char20: DBI_WR_CHAR20,
    _reserved26: [u8; 0x1c],
    #[doc = "0x16a0 - DBI Command In/Out Port"]
    pub dbi_cmd0: DBI_CMD0,
    _reserved27: [u8; 0x1c],
    #[doc = "0x16c0 - DPI Configuration 0"]
    pub dpi_config0: DPI_CONFIG0,
    _reserved28: [u8; 0x2c],
    #[doc = "0x16f0 - Revision for the LCDIF Peripheral in BCD"]
    pub dc_chip_rev: DC_CHIP_REV,
    _reserved29: [u8; 0x0c],
    #[doc = "0x1700 - Shows the release date for the IP in YYYYMMDD (year, month)"]
    pub dc_chip_date: DC_CHIP_DATE,
    _reserved30: [u8; 0x1c],
    #[doc = "0x1720 - Patch Revision"]
    pub dc_chip_patch_rev: DC_CHIP_PATCH_REV,
    _reserved31: [u8; 0x1c],
    #[doc = "0x1740 - Tile Input Configuration"]
    pub dc_tile_in_cfg0: DC_TILE_IN_CFG0,
    _reserved32: [u8; 0x1c],
    #[doc = "0x1760 - UV Frame Buffer Address when Tile Input"]
    pub dc_tile_uv_frame_buffer_adr0: DC_TILE_UV_FRAME_BUFFER_ADR0,
    _reserved33: [u8; 0x1c],
    #[doc = "0x1780 - UV Frame Buffer Stride when Tile Input"]
    pub dc_tile_uv_frame_buffer_str0: DC_TILE_UV_FRAME_BUFFER_STR0,
    _reserved34: [u8; 0x2c],
    #[doc = "0x17b0 - Product ID"]
    pub dc_product_id: DC_PRODUCT_ID,
    _reserved35: [u8; 0x6c],
    #[doc = "0x1820 - Debug Counter Select"]
    pub debug_counter_select0: DEBUG_COUNTER_SELECT0,
    _reserved36: [u8; 0x1c],
    #[doc = "0x1840 - Debug Counter Value"]
    pub debug_counter_value0: DEBUG_COUNTER_VALUE0,
}
#[doc = "FrameBufferConfig0 (rw) register accessor: an alias for `Reg<FRAME_BUFFER_CONFIG0_SPEC>`"]
pub type FRAME_BUFFER_CONFIG0 = crate::Reg<frame_buffer_config0::FRAME_BUFFER_CONFIG0_SPEC>;
#[doc = "Frame Buffer Configuration 0"]
pub mod frame_buffer_config0;
#[doc = "FrameBufferAddress0 (rw) register accessor: an alias for `Reg<FRAME_BUFFER_ADDRESS0_SPEC>`"]
pub type FRAME_BUFFER_ADDRESS0 = crate::Reg<frame_buffer_address0::FRAME_BUFFER_ADDRESS0_SPEC>;
#[doc = "Starting Address of the Frame Buffer"]
pub mod frame_buffer_address0;
#[doc = "FrameBufferStride0 (rw) register accessor: an alias for `Reg<FRAME_BUFFER_STRIDE0_SPEC>`"]
pub type FRAME_BUFFER_STRIDE0 = crate::Reg<frame_buffer_stride0::FRAME_BUFFER_STRIDE0_SPEC>;
#[doc = "Stride of the Frame Buffer in Bytes"]
pub mod frame_buffer_stride0;
#[doc = "DisplayDitherConfig0 (rw) register accessor: an alias for `Reg<DISPLAY_DITHER_CONFIG0_SPEC>`"]
pub type DISPLAY_DITHER_CONFIG0 = crate::Reg<display_dither_config0::DISPLAY_DITHER_CONFIG0_SPEC>;
#[doc = "Configuration for Dithering"]
pub mod display_dither_config0;
#[doc = "DisplayDitherTableLow0 (rw) register accessor: an alias for `Reg<DISPLAY_DITHER_TABLE_LOW0_SPEC>`"]
pub type DISPLAY_DITHER_TABLE_LOW0 =
    crate::Reg<display_dither_table_low0::DISPLAY_DITHER_TABLE_LOW0_SPEC>;
#[doc = "Dither Table Low"]
pub mod display_dither_table_low0;
#[doc = "DisplayDitherTableHigh0 (rw) register accessor: an alias for `Reg<DISPLAY_DITHER_TABLE_HIGH0_SPEC>`"]
pub type DISPLAY_DITHER_TABLE_HIGH0 =
    crate::Reg<display_dither_table_high0::DISPLAY_DITHER_TABLE_HIGH0_SPEC>;
#[doc = "Dither Table High"]
pub mod display_dither_table_high0;
#[doc = "PanelConfig0 (rw) register accessor: an alias for `Reg<PANEL_CONFIG0_SPEC>`"]
pub type PANEL_CONFIG0 = crate::Reg<panel_config0::PANEL_CONFIG0_SPEC>;
#[doc = "Panel Configuration"]
pub mod panel_config0;
#[doc = "PanelTiming0 (rw) register accessor: an alias for `Reg<PANEL_TIMING0_SPEC>`"]
pub type PANEL_TIMING0 = crate::Reg<panel_timing0::PANEL_TIMING0_SPEC>;
#[doc = "Timing for Hardware Panel Sequencing"]
pub mod panel_timing0;
#[doc = "HDisplay0 (rw) register accessor: an alias for `Reg<HDISPLAY0_SPEC>`"]
pub type HDISPLAY0 = crate::Reg<hdisplay0::HDISPLAY0_SPEC>;
#[doc = "Horizontal Total and Display End Counters"]
pub mod hdisplay0;
#[doc = "HSync0 (rw) register accessor: an alias for `Reg<HSYNC0_SPEC>`"]
pub type HSYNC0 = crate::Reg<hsync0::HSYNC0_SPEC>;
#[doc = "Horizontal Sync Counters"]
pub mod hsync0;
#[doc = "VDisplay0 (rw) register accessor: an alias for `Reg<VDISPLAY0_SPEC>`"]
pub type VDISPLAY0 = crate::Reg<vdisplay0::VDISPLAY0_SPEC>;
#[doc = "Vertical Total and Display End Counters"]
pub mod vdisplay0;
#[doc = "VSync0 (rw) register accessor: an alias for `Reg<VSYNC0_SPEC>`"]
pub type VSYNC0 = crate::Reg<vsync0::VSYNC0_SPEC>;
#[doc = "Vertical Sync Counters"]
pub mod vsync0;
#[doc = "DisplayCurrentLocation0 (r) register accessor: an alias for `Reg<DISPLAY_CURRENT_LOCATION0_SPEC>`"]
pub type DISPLAY_CURRENT_LOCATION0 =
    crate::Reg<display_current_location0::DISPLAY_CURRENT_LOCATION0_SPEC>;
#[doc = "Current x,y Location of Display Controller"]
pub mod display_current_location0;
#[doc = "GammaIndex0 (rw) register accessor: an alias for `Reg<GAMMA_INDEX0_SPEC>`"]
pub type GAMMA_INDEX0 = crate::Reg<gamma_index0::GAMMA_INDEX0_SPEC>;
#[doc = "Index into Gamma Table"]
pub mod gamma_index0;
#[doc = "GammaData0 (rw) register accessor: an alias for `Reg<GAMMA_DATA0_SPEC>`"]
pub type GAMMA_DATA0 = crate::Reg<gamma_data0::GAMMA_DATA0_SPEC>;
#[doc = "Translation Values for the Gamma Table"]
pub mod gamma_data0;
#[doc = "CursorConfig (rw) register accessor: an alias for `Reg<CURSOR_CONFIG_SPEC>`"]
pub type CURSOR_CONFIG = crate::Reg<cursor_config::CURSOR_CONFIG_SPEC>;
#[doc = "Configuration for the Cursor"]
pub mod cursor_config;
#[doc = "CursorAddress (rw) register accessor: an alias for `Reg<CURSOR_ADDRESS_SPEC>`"]
pub type CURSOR_ADDRESS = crate::Reg<cursor_address::CURSOR_ADDRESS_SPEC>;
#[doc = "Address of the Cursor Shape"]
pub mod cursor_address;
#[doc = "CursorLocation (rw) register accessor: an alias for `Reg<CURSOR_LOCATION_SPEC>`"]
pub type CURSOR_LOCATION = crate::Reg<cursor_location::CURSOR_LOCATION_SPEC>;
#[doc = "Location of the cursor on the owning display"]
pub mod cursor_location;
#[doc = "CursorBackground (rw) register accessor: an alias for `Reg<CURSOR_BACKGROUND_SPEC>`"]
pub type CURSOR_BACKGROUND = crate::Reg<cursor_background::CURSOR_BACKGROUND_SPEC>;
#[doc = "Background Color for Masked Cursors"]
pub mod cursor_background;
#[doc = "CursorForeground (rw) register accessor: an alias for `Reg<CURSOR_FOREGROUND_SPEC>`"]
pub type CURSOR_FOREGROUND = crate::Reg<cursor_foreground::CURSOR_FOREGROUND_SPEC>;
#[doc = "Foreground Color for Masked Cursors"]
pub mod cursor_foreground;
#[doc = "DisplayIntr (rw) register accessor: an alias for `Reg<DISPLAY_INTR_SPEC>`"]
pub type DISPLAY_INTR = crate::Reg<display_intr::DISPLAY_INTR_SPEC>;
#[doc = "Display Interrupt"]
pub mod display_intr;
#[doc = "DisplayIntrEnable (rw) register accessor: an alias for `Reg<DISPLAY_INTR_ENABLE_SPEC>`"]
pub type DISPLAY_INTR_ENABLE = crate::Reg<display_intr_enable::DISPLAY_INTR_ENABLE_SPEC>;
#[doc = "Interrupt Enable for Display_0 (and Display_1 if present)"]
pub mod display_intr_enable;
#[doc = "DbiConfig0 (rw) register accessor: an alias for `Reg<DBI_CONFIG0_SPEC>`"]
pub type DBI_CONFIG0 = crate::Reg<dbi_config0::DBI_CONFIG0_SPEC>;
#[doc = "DBI Configuration 0"]
pub mod dbi_config0;
#[doc = "DbiIfReset0 (w) register accessor: an alias for `Reg<DBI_IF_RESET0_SPEC>`"]
pub type DBI_IF_RESET0 = crate::Reg<dbi_if_reset0::DBI_IF_RESET0_SPEC>;
#[doc = "Reset DBI Interface to Idle State"]
pub mod dbi_if_reset0;
#[doc = "DbiWrChar10 (rw) register accessor: an alias for `Reg<DBI_WR_CHAR10_SPEC>`"]
pub type DBI_WR_CHAR10 = crate::Reg<dbi_wr_char10::DBI_WR_CHAR10_SPEC>;
#[doc = "DBI Write Characteristics 1"]
pub mod dbi_wr_char10;
#[doc = "DbiWrChar20 (rw) register accessor: an alias for `Reg<DBI_WR_CHAR20_SPEC>`"]
pub type DBI_WR_CHAR20 = crate::Reg<dbi_wr_char20::DBI_WR_CHAR20_SPEC>;
#[doc = "DBI Write Characteristics 2"]
pub mod dbi_wr_char20;
#[doc = "DbiCmd0 (w) register accessor: an alias for `Reg<DBI_CMD0_SPEC>`"]
pub type DBI_CMD0 = crate::Reg<dbi_cmd0::DBI_CMD0_SPEC>;
#[doc = "DBI Command In/Out Port"]
pub mod dbi_cmd0;
#[doc = "DpiConfig0 (rw) register accessor: an alias for `Reg<DPI_CONFIG0_SPEC>`"]
pub type DPI_CONFIG0 = crate::Reg<dpi_config0::DPI_CONFIG0_SPEC>;
#[doc = "DPI Configuration 0"]
pub mod dpi_config0;
#[doc = "DcChipRev (r) register accessor: an alias for `Reg<DC_CHIP_REV_SPEC>`"]
pub type DC_CHIP_REV = crate::Reg<dc_chip_rev::DC_CHIP_REV_SPEC>;
#[doc = "Revision for the LCDIF Peripheral in BCD"]
pub mod dc_chip_rev;
#[doc = "DcChipDate (r) register accessor: an alias for `Reg<DC_CHIP_DATE_SPEC>`"]
pub type DC_CHIP_DATE = crate::Reg<dc_chip_date::DC_CHIP_DATE_SPEC>;
#[doc = "Shows the release date for the IP in YYYYMMDD (year, month)"]
pub mod dc_chip_date;
#[doc = "DcChipPatchRev (r) register accessor: an alias for `Reg<DC_CHIP_PATCH_REV_SPEC>`"]
pub type DC_CHIP_PATCH_REV = crate::Reg<dc_chip_patch_rev::DC_CHIP_PATCH_REV_SPEC>;
#[doc = "Patch Revision"]
pub mod dc_chip_patch_rev;
#[doc = "DcTileInCfg0 (rw) register accessor: an alias for `Reg<DC_TILE_IN_CFG0_SPEC>`"]
pub type DC_TILE_IN_CFG0 = crate::Reg<dc_tile_in_cfg0::DC_TILE_IN_CFG0_SPEC>;
#[doc = "Tile Input Configuration"]
pub mod dc_tile_in_cfg0;
#[doc = "DcTileUvFrameBufferAdr0 (rw) register accessor: an alias for `Reg<DC_TILE_UV_FRAME_BUFFER_ADR0_SPEC>`"]
pub type DC_TILE_UV_FRAME_BUFFER_ADR0 =
    crate::Reg<dc_tile_uv_frame_buffer_adr0::DC_TILE_UV_FRAME_BUFFER_ADR0_SPEC>;
#[doc = "UV Frame Buffer Address when Tile Input"]
pub mod dc_tile_uv_frame_buffer_adr0;
#[doc = "DcTileUvFrameBufferStr0 (rw) register accessor: an alias for `Reg<DC_TILE_UV_FRAME_BUFFER_STR0_SPEC>`"]
pub type DC_TILE_UV_FRAME_BUFFER_STR0 =
    crate::Reg<dc_tile_uv_frame_buffer_str0::DC_TILE_UV_FRAME_BUFFER_STR0_SPEC>;
#[doc = "UV Frame Buffer Stride when Tile Input"]
pub mod dc_tile_uv_frame_buffer_str0;
#[doc = "DcProductId (r) register accessor: an alias for `Reg<DC_PRODUCT_ID_SPEC>`"]
pub type DC_PRODUCT_ID = crate::Reg<dc_product_id::DC_PRODUCT_ID_SPEC>;
#[doc = "Product ID"]
pub mod dc_product_id;
#[doc = "DebugCounterSelect0 (rw) register accessor: an alias for `Reg<DEBUG_COUNTER_SELECT0_SPEC>`"]
pub type DEBUG_COUNTER_SELECT0 = crate::Reg<debug_counter_select0::DEBUG_COUNTER_SELECT0_SPEC>;
#[doc = "Debug Counter Select"]
pub mod debug_counter_select0;
#[doc = "DebugCounterValue0 (rw) register accessor: an alias for `Reg<DEBUG_COUNTER_VALUE0_SPEC>`"]
pub type DEBUG_COUNTER_VALUE0 = crate::Reg<debug_counter_value0::DEBUG_COUNTER_VALUE0_SPEC>;
#[doc = "Debug Counter Value"]
pub mod debug_counter_value0;
