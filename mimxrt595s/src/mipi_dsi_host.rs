#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - no description available"]
    pub dsi_host_cfg_num_lanes: DSI_HOST_CFG_NUM_LANES,
    #[doc = "0x04 - no description available"]
    pub dsi_host_cfg_noncontinuous_clk: DSI_HOST_CFG_NONCONTINUOUS_CLK,
    #[doc = "0x08 - no description available"]
    pub dsi_host_cfg_t_pre: DSI_HOST_CFG_T_PRE,
    #[doc = "0x0c - no description available"]
    pub dsi_host_cfg_t_post: DSI_HOST_CFG_T_POST,
    #[doc = "0x10 - no description available"]
    pub dsi_host_cfg_tx_gap: DSI_HOST_CFG_TX_GAP,
    #[doc = "0x14 - no description available"]
    pub dsi_host_cfg_autoinsert_eotp: DSI_HOST_CFG_AUTOINSERT_EOTP,
    #[doc = "0x18 - no description available"]
    pub dsi_host_cfg_extra_cmds_after_eotp: DSI_HOST_CFG_EXTRA_CMDS_AFTER_EOTP,
    #[doc = "0x1c - no description available"]
    pub dsi_host_cfg_htx_to_count: DSI_HOST_CFG_HTX_TO_COUNT,
    #[doc = "0x20 - no description available"]
    pub dsi_host_cfg_lrx_h_to_count: DSI_HOST_CFG_LRX_H_TO_COUNT,
    #[doc = "0x24 - no description available"]
    pub dsi_host_cfg_bta_h_to_count: DSI_HOST_CFG_BTA_H_TO_COUNT,
    #[doc = "0x28 - no description available"]
    pub dsi_host_cfg_twakeup: DSI_HOST_CFG_TWAKEUP,
    #[doc = "0x2c - no description available"]
    pub dsi_host_cfg_status_out: DSI_HOST_CFG_STATUS_OUT,
    #[doc = "0x30 - no description available"]
    pub dsi_host_rx_error_status: DSI_HOST_RX_ERROR_STATUS,
    _reserved13: [u8; 0xcc],
    #[doc = "0x100 - no description available"]
    pub dsi_host_cfg_dbi_pixel_payload_size: DSI_HOST_CFG_DBI_PIXEL_PAYLOAD_SIZE,
    #[doc = "0x104 - no description available"]
    pub dsi_host_cfg_dbi_pixel_fifo_send_level: DSI_HOST_CFG_DBI_PIXEL_FIFO_SEND_LEVEL,
    _reserved15: [u8; 0xf8],
    #[doc = "0x200 - no description available"]
    pub dsi_host_cfg_dpi_pixel_payload_size: DSI_HOST_CFG_DPI_PIXEL_PAYLOAD_SIZE,
    #[doc = "0x204 - no description available"]
    pub dsi_host_cfg_dpi_pixel_fifo_send_level: DSI_HOST_CFG_DPI_PIXEL_FIFO_SEND_LEVEL,
    #[doc = "0x208 - no description available"]
    pub dsi_host_cfg_dpi_interface_color_coding: DSI_HOST_CFG_DPI_INTERFACE_COLOR_CODING,
    #[doc = "0x20c - no description available"]
    pub dsi_host_cfg_dpi_pixel_format: DSI_HOST_CFG_DPI_PIXEL_FORMAT,
    #[doc = "0x210 - no description available"]
    pub dsi_host_cfg_dpi_vsync_polarity: DSI_HOST_CFG_DPI_VSYNC_POLARITY,
    #[doc = "0x214 - no description available"]
    pub dsi_host_cfg_dpi_hsync_polarity: DSI_HOST_CFG_DPI_HSYNC_POLARITY,
    #[doc = "0x218 - no description available"]
    pub dsi_host_cfg_dpi_video_mode: DSI_HOST_CFG_DPI_VIDEO_MODE,
    #[doc = "0x21c - no description available"]
    pub dsi_host_cfg_dpi_hfp: DSI_HOST_CFG_DPI_HFP,
    #[doc = "0x220 - no description available"]
    pub dsi_host_cfg_dpi_hbp: DSI_HOST_CFG_DPI_HBP,
    #[doc = "0x224 - no description available"]
    pub dsi_host_cfg_dpi_hsa: DSI_HOST_CFG_DPI_HSA,
    #[doc = "0x228 - no description available"]
    pub dsi_host_cfg_dpi_enable_mult_pkts: DSI_HOST_CFG_DPI_ENABLE_MULT_PKTS,
    #[doc = "0x22c - no description available"]
    pub dsi_host_cfg_dpi_vbp: DSI_HOST_CFG_DPI_VBP,
    #[doc = "0x230 - no description available"]
    pub dsi_host_cfg_dpi_vfp: DSI_HOST_CFG_DPI_VFP,
    #[doc = "0x234 - no description available"]
    pub dsi_host_cfg_dpi_bllp_mode: DSI_HOST_CFG_DPI_BLLP_MODE,
    #[doc = "0x238 - no description available"]
    pub dsi_host_cfg_dpi_use_null_pkt_bllp: DSI_HOST_CFG_DPI_USE_NULL_PKT_BLLP,
    #[doc = "0x23c - no description available"]
    pub dsi_host_cfg_dpi_vactive: DSI_HOST_CFG_DPI_VACTIVE,
    #[doc = "0x240 - no description available"]
    pub dsi_host_cfg_dpi_vc: DSI_HOST_CFG_DPI_VC,
    _reserved32: [u8; 0x3c],
    #[doc = "0x280 - no description available"]
    pub dsi_host_tx_payload: DSI_HOST_TX_PAYLOAD,
    #[doc = "0x284 - no description available"]
    pub dsi_host_pkt_control: DSI_HOST_PKT_CONTROL,
    #[doc = "0x288 - no description available"]
    pub dsi_host_send_packet: DSI_HOST_SEND_PACKET,
    #[doc = "0x28c - no description available"]
    pub dsi_host_pkt_status: DSI_HOST_PKT_STATUS,
    #[doc = "0x290 - no description available"]
    pub dsi_host_pkt_fifo_wr_level: DSI_HOST_PKT_FIFO_WR_LEVEL,
    #[doc = "0x294 - no description available"]
    pub dsi_host_pkt_fifo_rd_level: DSI_HOST_PKT_FIFO_RD_LEVEL,
    #[doc = "0x298 - no description available"]
    pub dsi_host_pkt_rx_payload: DSI_HOST_PKT_RX_PAYLOAD,
    #[doc = "0x29c - no description available"]
    pub dsi_host_pkt_rx_pkt_header: DSI_HOST_PKT_RX_PKT_HEADER,
    #[doc = "0x2a0 - no description available"]
    pub dsi_host_irq_status: DSI_HOST_IRQ_STATUS,
    #[doc = "0x2a4 - no description available"]
    pub dsi_host_irq_status2: DSI_HOST_IRQ_STATUS2,
    #[doc = "0x2a8 - no description available"]
    pub dsi_host_irq_mask: DSI_HOST_IRQ_MASK,
    #[doc = "0x2ac - no description available"]
    pub dsi_host_irq_mask2: DSI_HOST_IRQ_MASK2,
    _reserved44: [u8; 0x50],
    #[doc = "0x300 - no description available"]
    pub dphy_pd_dphy: DPHY_PD_DPHY,
    #[doc = "0x304 - no description available"]
    pub dphy_m_prg_hs_prepare: DPHY_M_PRG_HS_PREPARE,
    #[doc = "0x308 - no description available"]
    pub dphy_mc_prg_hs_prepare: DPHY_MC_PRG_HS_PREPARE,
    #[doc = "0x30c - no description available"]
    pub dphy_m_prg_hs_zero: DPHY_M_PRG_HS_ZERO,
    #[doc = "0x310 - no description available"]
    pub dphy_mc_prg_hs_zero: DPHY_MC_PRG_HS_ZERO,
    #[doc = "0x314 - no description available"]
    pub dphy_m_prg_hs_trail: DPHY_M_PRG_HS_TRAIL,
    #[doc = "0x318 - no description available"]
    pub dphy_mc_prg_hs_trail: DPHY_MC_PRG_HS_TRAIL,
    #[doc = "0x31c - no description available"]
    pub dphy_tst: DPHY_TST,
    #[doc = "0x320 - no description available"]
    pub dphy_rterm_sel: DPHY_RTERM_SEL,
    #[doc = "0x324 - no description available"]
    pub dphy_auto_pd_en: DPHY_AUTO_PD_EN,
    #[doc = "0x328 - no description available"]
    pub dphy_rxlprp: DPHY_RXLPRP,
    #[doc = "0x32c - no description available"]
    pub dphy_rxcdrp: DPHY_RXCDRP,
}
#[doc = "DSI_HOST_CFG_NUM_LANES (rw) register accessor: an alias for `Reg<DSI_HOST_CFG_NUM_LANES_SPEC>`"]
pub type DSI_HOST_CFG_NUM_LANES = crate::Reg<dsi_host_cfg_num_lanes::DSI_HOST_CFG_NUM_LANES_SPEC>;
#[doc = "no description available"]
pub mod dsi_host_cfg_num_lanes;
#[doc = "DSI_HOST_CFG_NONCONTINUOUS_CLK (rw) register accessor: an alias for `Reg<DSI_HOST_CFG_NONCONTINUOUS_CLK_SPEC>`"]
pub type DSI_HOST_CFG_NONCONTINUOUS_CLK =
    crate::Reg<dsi_host_cfg_noncontinuous_clk::DSI_HOST_CFG_NONCONTINUOUS_CLK_SPEC>;
#[doc = "no description available"]
pub mod dsi_host_cfg_noncontinuous_clk;
#[doc = "DSI_HOST_CFG_T_PRE (rw) register accessor: an alias for `Reg<DSI_HOST_CFG_T_PRE_SPEC>`"]
pub type DSI_HOST_CFG_T_PRE = crate::Reg<dsi_host_cfg_t_pre::DSI_HOST_CFG_T_PRE_SPEC>;
#[doc = "no description available"]
pub mod dsi_host_cfg_t_pre;
#[doc = "DSI_HOST_CFG_T_POST (rw) register accessor: an alias for `Reg<DSI_HOST_CFG_T_POST_SPEC>`"]
pub type DSI_HOST_CFG_T_POST = crate::Reg<dsi_host_cfg_t_post::DSI_HOST_CFG_T_POST_SPEC>;
#[doc = "no description available"]
pub mod dsi_host_cfg_t_post;
#[doc = "DSI_HOST_CFG_TX_GAP (rw) register accessor: an alias for `Reg<DSI_HOST_CFG_TX_GAP_SPEC>`"]
pub type DSI_HOST_CFG_TX_GAP = crate::Reg<dsi_host_cfg_tx_gap::DSI_HOST_CFG_TX_GAP_SPEC>;
#[doc = "no description available"]
pub mod dsi_host_cfg_tx_gap;
#[doc = "DSI_HOST_CFG_AUTOINSERT_EOTP (rw) register accessor: an alias for `Reg<DSI_HOST_CFG_AUTOINSERT_EOTP_SPEC>`"]
pub type DSI_HOST_CFG_AUTOINSERT_EOTP =
    crate::Reg<dsi_host_cfg_autoinsert_eotp::DSI_HOST_CFG_AUTOINSERT_EOTP_SPEC>;
#[doc = "no description available"]
pub mod dsi_host_cfg_autoinsert_eotp;
#[doc = "DSI_HOST_CFG_EXTRA_CMDS_AFTER_EOTP (rw) register accessor: an alias for `Reg<DSI_HOST_CFG_EXTRA_CMDS_AFTER_EOTP_SPEC>`"]
pub type DSI_HOST_CFG_EXTRA_CMDS_AFTER_EOTP =
    crate::Reg<dsi_host_cfg_extra_cmds_after_eotp::DSI_HOST_CFG_EXTRA_CMDS_AFTER_EOTP_SPEC>;
#[doc = "no description available"]
pub mod dsi_host_cfg_extra_cmds_after_eotp;
#[doc = "DSI_HOST_CFG_HTX_TO_COUNT (rw) register accessor: an alias for `Reg<DSI_HOST_CFG_HTX_TO_COUNT_SPEC>`"]
pub type DSI_HOST_CFG_HTX_TO_COUNT =
    crate::Reg<dsi_host_cfg_htx_to_count::DSI_HOST_CFG_HTX_TO_COUNT_SPEC>;
#[doc = "no description available"]
pub mod dsi_host_cfg_htx_to_count;
#[doc = "DSI_HOST_CFG_LRX_H_TO_COUNT (rw) register accessor: an alias for `Reg<DSI_HOST_CFG_LRX_H_TO_COUNT_SPEC>`"]
pub type DSI_HOST_CFG_LRX_H_TO_COUNT =
    crate::Reg<dsi_host_cfg_lrx_h_to_count::DSI_HOST_CFG_LRX_H_TO_COUNT_SPEC>;
#[doc = "no description available"]
pub mod dsi_host_cfg_lrx_h_to_count;
#[doc = "DSI_HOST_CFG_BTA_H_TO_COUNT (rw) register accessor: an alias for `Reg<DSI_HOST_CFG_BTA_H_TO_COUNT_SPEC>`"]
pub type DSI_HOST_CFG_BTA_H_TO_COUNT =
    crate::Reg<dsi_host_cfg_bta_h_to_count::DSI_HOST_CFG_BTA_H_TO_COUNT_SPEC>;
#[doc = "no description available"]
pub mod dsi_host_cfg_bta_h_to_count;
#[doc = "DSI_HOST_CFG_TWAKEUP (rw) register accessor: an alias for `Reg<DSI_HOST_CFG_TWAKEUP_SPEC>`"]
pub type DSI_HOST_CFG_TWAKEUP = crate::Reg<dsi_host_cfg_twakeup::DSI_HOST_CFG_TWAKEUP_SPEC>;
#[doc = "no description available"]
pub mod dsi_host_cfg_twakeup;
#[doc = "DSI_HOST_CFG_STATUS_OUT (r) register accessor: an alias for `Reg<DSI_HOST_CFG_STATUS_OUT_SPEC>`"]
pub type DSI_HOST_CFG_STATUS_OUT =
    crate::Reg<dsi_host_cfg_status_out::DSI_HOST_CFG_STATUS_OUT_SPEC>;
#[doc = "no description available"]
pub mod dsi_host_cfg_status_out;
#[doc = "DSI_HOST_RX_ERROR_STATUS (r) register accessor: an alias for `Reg<DSI_HOST_RX_ERROR_STATUS_SPEC>`"]
pub type DSI_HOST_RX_ERROR_STATUS =
    crate::Reg<dsi_host_rx_error_status::DSI_HOST_RX_ERROR_STATUS_SPEC>;
#[doc = "no description available"]
pub mod dsi_host_rx_error_status;
#[doc = "DSI_HOST_CFG_DBI_PIXEL_PAYLOAD_SIZE (rw) register accessor: an alias for `Reg<DSI_HOST_CFG_DBI_PIXEL_PAYLOAD_SIZE_SPEC>`"]
pub type DSI_HOST_CFG_DBI_PIXEL_PAYLOAD_SIZE =
    crate::Reg<dsi_host_cfg_dbi_pixel_payload_size::DSI_HOST_CFG_DBI_PIXEL_PAYLOAD_SIZE_SPEC>;
#[doc = "no description available"]
pub mod dsi_host_cfg_dbi_pixel_payload_size;
#[doc = "DSI_HOST_CFG_DBI_PIXEL_FIFO_SEND_LEVEL (rw) register accessor: an alias for `Reg<DSI_HOST_CFG_DBI_PIXEL_FIFO_SEND_LEVEL_SPEC>`"]
pub type DSI_HOST_CFG_DBI_PIXEL_FIFO_SEND_LEVEL =
    crate::Reg<dsi_host_cfg_dbi_pixel_fifo_send_level::DSI_HOST_CFG_DBI_PIXEL_FIFO_SEND_LEVEL_SPEC>;
#[doc = "no description available"]
pub mod dsi_host_cfg_dbi_pixel_fifo_send_level;
#[doc = "DSI_HOST_CFG_DPI_PIXEL_PAYLOAD_SIZE (rw) register accessor: an alias for `Reg<DSI_HOST_CFG_DPI_PIXEL_PAYLOAD_SIZE_SPEC>`"]
pub type DSI_HOST_CFG_DPI_PIXEL_PAYLOAD_SIZE =
    crate::Reg<dsi_host_cfg_dpi_pixel_payload_size::DSI_HOST_CFG_DPI_PIXEL_PAYLOAD_SIZE_SPEC>;
#[doc = "no description available"]
pub mod dsi_host_cfg_dpi_pixel_payload_size;
#[doc = "DSI_HOST_CFG_DPI_PIXEL_FIFO_SEND_LEVEL (rw) register accessor: an alias for `Reg<DSI_HOST_CFG_DPI_PIXEL_FIFO_SEND_LEVEL_SPEC>`"]
pub type DSI_HOST_CFG_DPI_PIXEL_FIFO_SEND_LEVEL =
    crate::Reg<dsi_host_cfg_dpi_pixel_fifo_send_level::DSI_HOST_CFG_DPI_PIXEL_FIFO_SEND_LEVEL_SPEC>;
#[doc = "no description available"]
pub mod dsi_host_cfg_dpi_pixel_fifo_send_level;
#[doc = "DSI_HOST_CFG_DPI_INTERFACE_COLOR_CODING (rw) register accessor: an alias for `Reg<DSI_HOST_CFG_DPI_INTERFACE_COLOR_CODING_SPEC>`"]
pub type DSI_HOST_CFG_DPI_INTERFACE_COLOR_CODING = crate::Reg<
    dsi_host_cfg_dpi_interface_color_coding::DSI_HOST_CFG_DPI_INTERFACE_COLOR_CODING_SPEC,
>;
#[doc = "no description available"]
pub mod dsi_host_cfg_dpi_interface_color_coding;
#[doc = "DSI_HOST_CFG_DPI_PIXEL_FORMAT (rw) register accessor: an alias for `Reg<DSI_HOST_CFG_DPI_PIXEL_FORMAT_SPEC>`"]
pub type DSI_HOST_CFG_DPI_PIXEL_FORMAT =
    crate::Reg<dsi_host_cfg_dpi_pixel_format::DSI_HOST_CFG_DPI_PIXEL_FORMAT_SPEC>;
#[doc = "no description available"]
pub mod dsi_host_cfg_dpi_pixel_format;
#[doc = "DSI_HOST_CFG_DPI_VSYNC_POLARITY (rw) register accessor: an alias for `Reg<DSI_HOST_CFG_DPI_VSYNC_POLARITY_SPEC>`"]
pub type DSI_HOST_CFG_DPI_VSYNC_POLARITY =
    crate::Reg<dsi_host_cfg_dpi_vsync_polarity::DSI_HOST_CFG_DPI_VSYNC_POLARITY_SPEC>;
#[doc = "no description available"]
pub mod dsi_host_cfg_dpi_vsync_polarity;
#[doc = "DSI_HOST_CFG_DPI_HSYNC_POLARITY (rw) register accessor: an alias for `Reg<DSI_HOST_CFG_DPI_HSYNC_POLARITY_SPEC>`"]
pub type DSI_HOST_CFG_DPI_HSYNC_POLARITY =
    crate::Reg<dsi_host_cfg_dpi_hsync_polarity::DSI_HOST_CFG_DPI_HSYNC_POLARITY_SPEC>;
#[doc = "no description available"]
pub mod dsi_host_cfg_dpi_hsync_polarity;
#[doc = "DSI_HOST_CFG_DPI_VIDEO_MODE (rw) register accessor: an alias for `Reg<DSI_HOST_CFG_DPI_VIDEO_MODE_SPEC>`"]
pub type DSI_HOST_CFG_DPI_VIDEO_MODE =
    crate::Reg<dsi_host_cfg_dpi_video_mode::DSI_HOST_CFG_DPI_VIDEO_MODE_SPEC>;
#[doc = "no description available"]
pub mod dsi_host_cfg_dpi_video_mode;
#[doc = "DSI_HOST_CFG_DPI_HFP (rw) register accessor: an alias for `Reg<DSI_HOST_CFG_DPI_HFP_SPEC>`"]
pub type DSI_HOST_CFG_DPI_HFP = crate::Reg<dsi_host_cfg_dpi_hfp::DSI_HOST_CFG_DPI_HFP_SPEC>;
#[doc = "no description available"]
pub mod dsi_host_cfg_dpi_hfp;
#[doc = "DSI_HOST_CFG_DPI_HBP (rw) register accessor: an alias for `Reg<DSI_HOST_CFG_DPI_HBP_SPEC>`"]
pub type DSI_HOST_CFG_DPI_HBP = crate::Reg<dsi_host_cfg_dpi_hbp::DSI_HOST_CFG_DPI_HBP_SPEC>;
#[doc = "no description available"]
pub mod dsi_host_cfg_dpi_hbp;
#[doc = "DSI_HOST_CFG_DPI_HSA (rw) register accessor: an alias for `Reg<DSI_HOST_CFG_DPI_HSA_SPEC>`"]
pub type DSI_HOST_CFG_DPI_HSA = crate::Reg<dsi_host_cfg_dpi_hsa::DSI_HOST_CFG_DPI_HSA_SPEC>;
#[doc = "no description available"]
pub mod dsi_host_cfg_dpi_hsa;
#[doc = "DSI_HOST_CFG_DPI_ENABLE_MULT_PKTS (rw) register accessor: an alias for `Reg<DSI_HOST_CFG_DPI_ENABLE_MULT_PKTS_SPEC>`"]
pub type DSI_HOST_CFG_DPI_ENABLE_MULT_PKTS =
    crate::Reg<dsi_host_cfg_dpi_enable_mult_pkts::DSI_HOST_CFG_DPI_ENABLE_MULT_PKTS_SPEC>;
#[doc = "no description available"]
pub mod dsi_host_cfg_dpi_enable_mult_pkts;
#[doc = "DSI_HOST_CFG_DPI_VBP (rw) register accessor: an alias for `Reg<DSI_HOST_CFG_DPI_VBP_SPEC>`"]
pub type DSI_HOST_CFG_DPI_VBP = crate::Reg<dsi_host_cfg_dpi_vbp::DSI_HOST_CFG_DPI_VBP_SPEC>;
#[doc = "no description available"]
pub mod dsi_host_cfg_dpi_vbp;
#[doc = "DSI_HOST_CFG_DPI_VFP (rw) register accessor: an alias for `Reg<DSI_HOST_CFG_DPI_VFP_SPEC>`"]
pub type DSI_HOST_CFG_DPI_VFP = crate::Reg<dsi_host_cfg_dpi_vfp::DSI_HOST_CFG_DPI_VFP_SPEC>;
#[doc = "no description available"]
pub mod dsi_host_cfg_dpi_vfp;
#[doc = "DSI_HOST_CFG_DPI_BLLP_MODE (rw) register accessor: an alias for `Reg<DSI_HOST_CFG_DPI_BLLP_MODE_SPEC>`"]
pub type DSI_HOST_CFG_DPI_BLLP_MODE =
    crate::Reg<dsi_host_cfg_dpi_bllp_mode::DSI_HOST_CFG_DPI_BLLP_MODE_SPEC>;
#[doc = "no description available"]
pub mod dsi_host_cfg_dpi_bllp_mode;
#[doc = "DSI_HOST_CFG_DPI_USE_NULL_PKT_BLLP (rw) register accessor: an alias for `Reg<DSI_HOST_CFG_DPI_USE_NULL_PKT_BLLP_SPEC>`"]
pub type DSI_HOST_CFG_DPI_USE_NULL_PKT_BLLP =
    crate::Reg<dsi_host_cfg_dpi_use_null_pkt_bllp::DSI_HOST_CFG_DPI_USE_NULL_PKT_BLLP_SPEC>;
#[doc = "no description available"]
pub mod dsi_host_cfg_dpi_use_null_pkt_bllp;
#[doc = "DSI_HOST_CFG_DPI_VACTIVE (rw) register accessor: an alias for `Reg<DSI_HOST_CFG_DPI_VACTIVE_SPEC>`"]
pub type DSI_HOST_CFG_DPI_VACTIVE =
    crate::Reg<dsi_host_cfg_dpi_vactive::DSI_HOST_CFG_DPI_VACTIVE_SPEC>;
#[doc = "no description available"]
pub mod dsi_host_cfg_dpi_vactive;
#[doc = "DSI_HOST_CFG_DPI_VC (rw) register accessor: an alias for `Reg<DSI_HOST_CFG_DPI_VC_SPEC>`"]
pub type DSI_HOST_CFG_DPI_VC = crate::Reg<dsi_host_cfg_dpi_vc::DSI_HOST_CFG_DPI_VC_SPEC>;
#[doc = "no description available"]
pub mod dsi_host_cfg_dpi_vc;
#[doc = "DSI_HOST_TX_PAYLOAD (rw) register accessor: an alias for `Reg<DSI_HOST_TX_PAYLOAD_SPEC>`"]
pub type DSI_HOST_TX_PAYLOAD = crate::Reg<dsi_host_tx_payload::DSI_HOST_TX_PAYLOAD_SPEC>;
#[doc = "no description available"]
pub mod dsi_host_tx_payload;
#[doc = "DSI_HOST_PKT_CONTROL (rw) register accessor: an alias for `Reg<DSI_HOST_PKT_CONTROL_SPEC>`"]
pub type DSI_HOST_PKT_CONTROL = crate::Reg<dsi_host_pkt_control::DSI_HOST_PKT_CONTROL_SPEC>;
#[doc = "no description available"]
pub mod dsi_host_pkt_control;
#[doc = "DSI_HOST_SEND_PACKET (rw) register accessor: an alias for `Reg<DSI_HOST_SEND_PACKET_SPEC>`"]
pub type DSI_HOST_SEND_PACKET = crate::Reg<dsi_host_send_packet::DSI_HOST_SEND_PACKET_SPEC>;
#[doc = "no description available"]
pub mod dsi_host_send_packet;
#[doc = "DSI_HOST_PKT_STATUS (r) register accessor: an alias for `Reg<DSI_HOST_PKT_STATUS_SPEC>`"]
pub type DSI_HOST_PKT_STATUS = crate::Reg<dsi_host_pkt_status::DSI_HOST_PKT_STATUS_SPEC>;
#[doc = "no description available"]
pub mod dsi_host_pkt_status;
#[doc = "DSI_HOST_PKT_FIFO_WR_LEVEL (r) register accessor: an alias for `Reg<DSI_HOST_PKT_FIFO_WR_LEVEL_SPEC>`"]
pub type DSI_HOST_PKT_FIFO_WR_LEVEL =
    crate::Reg<dsi_host_pkt_fifo_wr_level::DSI_HOST_PKT_FIFO_WR_LEVEL_SPEC>;
#[doc = "no description available"]
pub mod dsi_host_pkt_fifo_wr_level;
#[doc = "DSI_HOST_PKT_FIFO_RD_LEVEL (r) register accessor: an alias for `Reg<DSI_HOST_PKT_FIFO_RD_LEVEL_SPEC>`"]
pub type DSI_HOST_PKT_FIFO_RD_LEVEL =
    crate::Reg<dsi_host_pkt_fifo_rd_level::DSI_HOST_PKT_FIFO_RD_LEVEL_SPEC>;
#[doc = "no description available"]
pub mod dsi_host_pkt_fifo_rd_level;
#[doc = "DSI_HOST_PKT_RX_PAYLOAD (r) register accessor: an alias for `Reg<DSI_HOST_PKT_RX_PAYLOAD_SPEC>`"]
pub type DSI_HOST_PKT_RX_PAYLOAD =
    crate::Reg<dsi_host_pkt_rx_payload::DSI_HOST_PKT_RX_PAYLOAD_SPEC>;
#[doc = "no description available"]
pub mod dsi_host_pkt_rx_payload;
#[doc = "DSI_HOST_PKT_RX_PKT_HEADER (r) register accessor: an alias for `Reg<DSI_HOST_PKT_RX_PKT_HEADER_SPEC>`"]
pub type DSI_HOST_PKT_RX_PKT_HEADER =
    crate::Reg<dsi_host_pkt_rx_pkt_header::DSI_HOST_PKT_RX_PKT_HEADER_SPEC>;
#[doc = "no description available"]
pub mod dsi_host_pkt_rx_pkt_header;
#[doc = "DSI_HOST_IRQ_STATUS (r) register accessor: an alias for `Reg<DSI_HOST_IRQ_STATUS_SPEC>`"]
pub type DSI_HOST_IRQ_STATUS = crate::Reg<dsi_host_irq_status::DSI_HOST_IRQ_STATUS_SPEC>;
#[doc = "no description available"]
pub mod dsi_host_irq_status;
#[doc = "DSI_HOST_IRQ_STATUS2 (r) register accessor: an alias for `Reg<DSI_HOST_IRQ_STATUS2_SPEC>`"]
pub type DSI_HOST_IRQ_STATUS2 = crate::Reg<dsi_host_irq_status2::DSI_HOST_IRQ_STATUS2_SPEC>;
#[doc = "no description available"]
pub mod dsi_host_irq_status2;
#[doc = "DSI_HOST_IRQ_MASK (rw) register accessor: an alias for `Reg<DSI_HOST_IRQ_MASK_SPEC>`"]
pub type DSI_HOST_IRQ_MASK = crate::Reg<dsi_host_irq_mask::DSI_HOST_IRQ_MASK_SPEC>;
#[doc = "no description available"]
pub mod dsi_host_irq_mask;
#[doc = "DSI_HOST_IRQ_MASK2 (rw) register accessor: an alias for `Reg<DSI_HOST_IRQ_MASK2_SPEC>`"]
pub type DSI_HOST_IRQ_MASK2 = crate::Reg<dsi_host_irq_mask2::DSI_HOST_IRQ_MASK2_SPEC>;
#[doc = "no description available"]
pub mod dsi_host_irq_mask2;
#[doc = "DPHY_PD_DPHY (rw) register accessor: an alias for `Reg<DPHY_PD_DPHY_SPEC>`"]
pub type DPHY_PD_DPHY = crate::Reg<dphy_pd_dphy::DPHY_PD_DPHY_SPEC>;
#[doc = "no description available"]
pub mod dphy_pd_dphy;
#[doc = "DPHY_M_PRG_HS_PREPARE (rw) register accessor: an alias for `Reg<DPHY_M_PRG_HS_PREPARE_SPEC>`"]
pub type DPHY_M_PRG_HS_PREPARE = crate::Reg<dphy_m_prg_hs_prepare::DPHY_M_PRG_HS_PREPARE_SPEC>;
#[doc = "no description available"]
pub mod dphy_m_prg_hs_prepare;
#[doc = "DPHY_MC_PRG_HS_PREPARE (rw) register accessor: an alias for `Reg<DPHY_MC_PRG_HS_PREPARE_SPEC>`"]
pub type DPHY_MC_PRG_HS_PREPARE = crate::Reg<dphy_mc_prg_hs_prepare::DPHY_MC_PRG_HS_PREPARE_SPEC>;
#[doc = "no description available"]
pub mod dphy_mc_prg_hs_prepare;
#[doc = "DPHY_M_PRG_HS_ZERO (rw) register accessor: an alias for `Reg<DPHY_M_PRG_HS_ZERO_SPEC>`"]
pub type DPHY_M_PRG_HS_ZERO = crate::Reg<dphy_m_prg_hs_zero::DPHY_M_PRG_HS_ZERO_SPEC>;
#[doc = "no description available"]
pub mod dphy_m_prg_hs_zero;
#[doc = "DPHY_MC_PRG_HS_ZERO (rw) register accessor: an alias for `Reg<DPHY_MC_PRG_HS_ZERO_SPEC>`"]
pub type DPHY_MC_PRG_HS_ZERO = crate::Reg<dphy_mc_prg_hs_zero::DPHY_MC_PRG_HS_ZERO_SPEC>;
#[doc = "no description available"]
pub mod dphy_mc_prg_hs_zero;
#[doc = "DPHY_M_PRG_HS_TRAIL (rw) register accessor: an alias for `Reg<DPHY_M_PRG_HS_TRAIL_SPEC>`"]
pub type DPHY_M_PRG_HS_TRAIL = crate::Reg<dphy_m_prg_hs_trail::DPHY_M_PRG_HS_TRAIL_SPEC>;
#[doc = "no description available"]
pub mod dphy_m_prg_hs_trail;
#[doc = "DPHY_MC_PRG_HS_TRAIL (rw) register accessor: an alias for `Reg<DPHY_MC_PRG_HS_TRAIL_SPEC>`"]
pub type DPHY_MC_PRG_HS_TRAIL = crate::Reg<dphy_mc_prg_hs_trail::DPHY_MC_PRG_HS_TRAIL_SPEC>;
#[doc = "no description available"]
pub mod dphy_mc_prg_hs_trail;
#[doc = "DPHY_TST (rw) register accessor: an alias for `Reg<DPHY_TST_SPEC>`"]
pub type DPHY_TST = crate::Reg<dphy_tst::DPHY_TST_SPEC>;
#[doc = "no description available"]
pub mod dphy_tst;
#[doc = "DPHY_RTERM_SEL (rw) register accessor: an alias for `Reg<DPHY_RTERM_SEL_SPEC>`"]
pub type DPHY_RTERM_SEL = crate::Reg<dphy_rterm_sel::DPHY_RTERM_SEL_SPEC>;
#[doc = "no description available"]
pub mod dphy_rterm_sel;
#[doc = "DPHY_AUTO_PD_EN (rw) register accessor: an alias for `Reg<DPHY_AUTO_PD_EN_SPEC>`"]
pub type DPHY_AUTO_PD_EN = crate::Reg<dphy_auto_pd_en::DPHY_AUTO_PD_EN_SPEC>;
#[doc = "no description available"]
pub mod dphy_auto_pd_en;
#[doc = "DPHY_RXLPRP (rw) register accessor: an alias for `Reg<DPHY_RXLPRP_SPEC>`"]
pub type DPHY_RXLPRP = crate::Reg<dphy_rxlprp::DPHY_RXLPRP_SPEC>;
#[doc = "no description available"]
pub mod dphy_rxlprp;
#[doc = "DPHY_RXCDRP (rw) register accessor: an alias for `Reg<DPHY_RXCDRP_SPEC>`"]
pub type DPHY_RXCDRP = crate::Reg<dphy_rxcdrp::DPHY_RXCDRP_SPEC>;
#[doc = "no description available"]
pub mod dphy_rxcdrp;
