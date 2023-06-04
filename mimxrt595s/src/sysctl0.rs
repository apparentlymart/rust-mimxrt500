#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 0x04],
    #[doc = "0x04 - DSP Vector Remap"]
    pub dsp_vect_remap: DSP_VECT_REMAP,
    _reserved1: [u8; 0x04],
    #[doc = "0x0c - DSP Stall Control"]
    pub dspstall: DSPSTALL,
    #[doc = "0x10 - AHB MAX Priority"]
    pub ahbmatrixprior: AHBMATRIXPRIOR,
    _reserved3: [u8; 0x0c],
    #[doc = "0x20..0x28 - AHB Buffer"]
    pub ahbbridgebuffer: [AHBBRIDGEBUFFER; 2],
    _reserved4: [u8; 0x04],
    #[doc = "0x2c - BOOT ROM lockout"]
    pub bootrom_lckout: BOOTROM_LCKOUT,
    #[doc = "0x30 - M33 NMI source selection"]
    pub m33nmisrcsel: M33NMISRCSEL,
    #[doc = "0x34 - System secure tick calibration"]
    pub system_stick_calib: SYSTEM_STICK_CALIB,
    #[doc = "0x38 - System non-secure tick calibration"]
    pub system_nstick_calib: SYSTEM_NSTICK_CALIB,
    _reserved8: [u8; 0x24],
    #[doc = "0x60 - Product ID"]
    pub product_id: PRODUCT_ID,
    #[doc = "0x64 - Silicon Revision ID"]
    pub siliconrev_id: SILICONREV_ID,
    #[doc = "0x68 - JTAG ID"]
    pub jtag_id: JTAG_ID,
    _reserved11: [u8; 0x04],
    #[doc = "0x70 - Non-secure GPIO PSYNC"]
    pub nsgpio_psync: NSGPIO_PSYNC,
    #[doc = "0x74 - Secure GPIO PSYNC"]
    pub sgpio_psync: SGPIO_PSYNC,
    _reserved13: [u8; 0x08],
    #[doc = "0x80 - Auto clock gate override 0"]
    pub autoclkgateoverride0: AUTOCLKGATEOVERRIDE0,
    #[doc = "0x84 - Auto clock gate override 1"]
    pub autoclkgateoverride1: AUTOCLKGATEOVERRIDE1,
    _reserved15: [u8; 0x18],
    #[doc = "0xa0 - Clock gate override 0"]
    pub clkgateoverride0: CLKGATEOVERRIDE0,
    _reserved16: [u8; 0x58],
    #[doc = "0xfc - AHB SRAM access disable"]
    pub ahb_sram_access_disable: AHB_SRAM_ACCESS_DISABLE,
    #[doc = "0x100 - AXI SRAM access disable"]
    pub axi_sram_access_disable: AXI_SRAM_ACCESS_DISABLE,
    #[doc = "0x104 - DSP SRAM access disable"]
    pub dsp_sram_access_disable: DSP_SRAM_ACCESS_DISABLE,
    _reserved19: [u8; 0x08],
    #[doc = "0x110 - Power-Quad Memory Control"]
    pub pq_mem_ctrl: PQ_MEM_CTRL,
    #[doc = "0x114 - FlexSPI0 Memory Control"]
    pub flexspi0_mem_ctrl: FLEXSPI0_MEM_CTRL,
    #[doc = "0x118 - USBHS Memory Control"]
    pub usbhs_mem_ctrl: USBHS_MEM_CTRL,
    #[doc = "0x11c - USDHC0 Memory Control"]
    pub usdhc0_mem_ctrl: USDHC0_MEM_CTRL,
    #[doc = "0x120 - USDHC1 Memory Control"]
    pub usdhc1_mem_ctrl: USDHC1_MEM_CTRL,
    #[doc = "0x124 - CASPER Memory Control"]
    pub casper_mem_ctrl: CASPER_MEM_CTRL,
    #[doc = "0x128 - ROM Memory Control"]
    pub rom_mem_ctrl: ROM_MEM_CTRL,
    #[doc = "0x12c - FlexSPI1 Memory Control"]
    pub flex_spi1_mem_ctrl: FLEX_SPI1_MEM_CTRL,
    #[doc = "0x130 - GPU Memory Control"]
    pub gpu_mem_ctrl: GPU_MEM_CTRL,
    #[doc = "0x134 - MIPI Memory Control"]
    pub mipi_mem_ctrl: MIPI_MEM_CTRL,
    #[doc = "0x138 - LCDIF Memory Control"]
    pub dcn_mem_ctrl: DCN_MEM_CTRL,
    #[doc = "0x13c - SMARTDMA Memory Control"]
    pub smartdma_mem_ctrl: SMARTDMA_MEM_CTRL,
    _reserved31: [u8; 0x02c0],
    #[doc = "0x400 - MIPI DSI Control"]
    pub mipi_dsi_ctrl: MIPI_DSI_CTRL,
    _reserved32: [u8; 0x08],
    #[doc = "0x40c - USB Clock Control"]
    pub usb0clkctrl: USB0CLKCTRL,
    #[doc = "0x410 - USB Clock Status"]
    pub usb0clkstat: USB0CLKSTAT,
    #[doc = "0x414 - USB PHY PLL0 lock time division"]
    pub usbphypll0locktimediv2: USBPHYPLL0LOCKTIMEDIV2,
    _reserved35: [u8; 0x01e8],
    #[doc = "0x600 - Sleep configuration 0"]
    pub pdsleepcfg0: PDSLEEPCFG0,
    #[doc = "0x604 - Sleep configuration 1"]
    pub pdsleepcfg1: PDSLEEPCFG1,
    #[doc = "0x608 - Sleep configuration 2"]
    pub pdsleepcfg2: PDSLEEPCFG2,
    #[doc = "0x60c - Sleep configuration 3"]
    pub pdsleepcfg3: PDSLEEPCFG3,
    #[doc = "0x610 - Run configuration 0"]
    pub pdruncfg0: PDRUNCFG0,
    #[doc = "0x614 - Run configuration 1"]
    pub pdruncfg1: PDRUNCFG1,
    #[doc = "0x618 - Run configuration 2"]
    pub pdruncfg2: PDRUNCFG2,
    #[doc = "0x61c - Run configuration 3"]
    pub pdruncfg3: PDRUNCFG3,
    #[doc = "0x620 - Run configuration 0 set"]
    pub pdruncfg0_set: PDRUNCFG0_SET,
    #[doc = "0x624 - Run configuration 1 set"]
    pub pdruncfg1_set: PDRUNCFG1_SET,
    #[doc = "0x628 - Run configuration 2 set"]
    pub pdruncfg2_set: PDRUNCFG2_SET,
    #[doc = "0x62c - Run configuration 3 set"]
    pub pdruncfg3_set: PDRUNCFG3_SET,
    #[doc = "0x630 - Run configuration 0 clear"]
    pub pdruncfg0_clr: PDRUNCFG0_CLR,
    #[doc = "0x634 - Run configuration 1 clear"]
    pub pdruncfg1_clr: PDRUNCFG1_CLR,
    #[doc = "0x638 - Run configuration 2 clear"]
    pub pdruncfg2_clr: PDRUNCFG2_CLR,
    #[doc = "0x63c - Run configuration 3 clear"]
    pub pdruncfg3_clr: PDRUNCFG3_CLR,
    _reserved51: [u8; 0x20],
    #[doc = "0x660 - PD Wake Configuration"]
    pub pdwakecfg: PDWAKECFG,
    _reserved52: [u8; 0x1c],
    #[doc = "0x680 - Start Enable 0"]
    pub starten0: STARTEN0,
    #[doc = "0x684 - Start Enable 1"]
    pub starten1: STARTEN1,
    #[doc = "0x688 - Start Enable 2"]
    pub starten2: STARTEN2,
    _reserved55: [u8; 0x14],
    #[doc = "0x6a0 - Start Enable 0 Set"]
    pub starten0_set: STARTEN0_SET,
    #[doc = "0x6a4 - Start Enable 1 Set"]
    pub starten1_set: STARTEN1_SET,
    #[doc = "0x6a8 - Start Enable 2"]
    pub starten2_set: STARTEN2_SET,
    _reserved58: [u8; 0x14],
    #[doc = "0x6c0 - Start Enable 0 clear"]
    pub starten0_clr: STARTEN0_CLR,
    #[doc = "0x6c4 - Start Enable 1 clear"]
    pub starten1_clr: STARTEN1_CLR,
    #[doc = "0x6c8 - Start Enable 2"]
    pub starten2_clr: STARTEN2_CLR,
    _reserved61: [u8; 0x38],
    #[doc = "0x704 - FRO Safety"]
    pub frosafety: FROSAFETY,
    _reserved62: [u8; 0x08],
    #[doc = "0x710 - Main Clock Safety"]
    pub mainclksafety: MAINCLKSAFETY,
    _reserved63: [u8; 0x6c],
    #[doc = "0x780 - Hardware Wake"]
    pub hwwake: HWWAKE,
    _reserved64: [u8; 0x0688],
    #[doc = "0xe0c - Temperature Sensor Control"]
    pub tempsensorctl: TEMPSENSORCTL,
    _reserved65: [u8; 0x30],
    #[doc = "0xe40 - Boot State Lock"]
    pub bootstatelock: BOOTSTATELOCK,
    _reserved66: [u8; 0x0c],
    #[doc = "0xe50..0xe70 - Boot State Seed"]
    pub bootstateseed: [BOOTSTATESEED; 8],
    #[doc = "0xe70..0xe90 - HMAC of boot state used for attestation."]
    pub bootstatehmac: [BOOTSTATEHMAC; 8],
    _reserved68: [u8; 0x60],
    #[doc = "0xef0 - FLEXSPI0 Pad Control"]
    pub flexspi0padctl: FLEXSPI0PADCTL,
    #[doc = "0xef4 - FLEXSPI1 Pad Control"]
    pub flexspi1padctl: FLEXSPI1PADCTL,
    #[doc = "0xef8 - SDIO0 Pad Control"]
    pub sdio0padctl: SDIO0PADCTL,
    #[doc = "0xefc - SDIO1 Pad Control"]
    pub sdio1padctl: SDIO1PADCTL,
    #[doc = "0xf00 - Compound Device Identifier (CDI)"]
    pub dicehwregn: DICEHWREGN,
    _reserved73: [u8; 0x4c],
    #[doc = "0xf50..0xf60 - UUID"]
    pub uuid: [UUID; 4],
    _reserved74: [u8; 0x20],
    #[doc = "0xf80 - AES Key Source Select"]
    pub aeskey_srcsel: AESKEY_SRCSEL,
    #[doc = "0xf84 - OTFAD Key Source Select"]
    pub otfadkey_srcsel: OTFADKEY_SRCSEL,
    #[doc = "0xf88 - HASH Hardware Key Disable"]
    pub hashhwkeydisable: HASHHWKEYDISABLE,
    _reserved77: [u8; 0x14],
    #[doc = "0xfa0 - Debug Lock Enable"]
    pub dbg_locken: DBG_LOCKEN,
    #[doc = "0xfa4 - Debug Features"]
    pub dbg_features: DBG_FEATURES,
    #[doc = "0xfa8 - Debug Features Duplicate"]
    pub dbg_features_dp: DBG_FEATURES_DP,
    _reserved80: [u8; 0x08],
    #[doc = "0xfb4 - Code Security for CPU0"]
    pub cs_protcpu0: CS_PROTCPU0,
    #[doc = "0xfb8 - Code Security for CPU1"]
    pub cs_protcpu1: CS_PROTCPU1,
    _reserved82: [u8; 0x04],
    #[doc = "0xfc0 - Debug authorization scratch"]
    pub dbg_auth_scratch: DBG_AUTH_SCRATCH,
    _reserved83: [u8; 0x0c],
    #[doc = "0xfd0 - Key block"]
    pub key_block: KEY_BLOCK,
}
#[doc = "DSP_VECT_REMAP (rw) register accessor: an alias for `Reg<DSP_VECT_REMAP_SPEC>`"]
pub type DSP_VECT_REMAP = crate::Reg<dsp_vect_remap::DSP_VECT_REMAP_SPEC>;
#[doc = "DSP Vector Remap"]
pub mod dsp_vect_remap;
#[doc = "DSPSTALL (rw) register accessor: an alias for `Reg<DSPSTALL_SPEC>`"]
pub type DSPSTALL = crate::Reg<dspstall::DSPSTALL_SPEC>;
#[doc = "DSP Stall Control"]
pub mod dspstall;
#[doc = "AHBMATRIXPRIOR (rw) register accessor: an alias for `Reg<AHBMATRIXPRIOR_SPEC>`"]
pub type AHBMATRIXPRIOR = crate::Reg<ahbmatrixprior::AHBMATRIXPRIOR_SPEC>;
#[doc = "AHB MAX Priority"]
pub mod ahbmatrixprior;
#[doc = "AHBBRIDGEBUFFER (rw) register accessor: an alias for `Reg<AHBBRIDGEBUFFER_SPEC>`"]
pub type AHBBRIDGEBUFFER = crate::Reg<ahbbridgebuffer::AHBBRIDGEBUFFER_SPEC>;
#[doc = "AHB Buffer"]
pub mod ahbbridgebuffer;
#[doc = "BOOTROM_LCKOUT (rw) register accessor: an alias for `Reg<BOOTROM_LCKOUT_SPEC>`"]
pub type BOOTROM_LCKOUT = crate::Reg<bootrom_lckout::BOOTROM_LCKOUT_SPEC>;
#[doc = "BOOT ROM lockout"]
pub mod bootrom_lckout;
#[doc = "M33NMISRCSEL (rw) register accessor: an alias for `Reg<M33NMISRCSEL_SPEC>`"]
pub type M33NMISRCSEL = crate::Reg<m33nmisrcsel::M33NMISRCSEL_SPEC>;
#[doc = "M33 NMI source selection"]
pub mod m33nmisrcsel;
#[doc = "SYSTEM_STICK_CALIB (rw) register accessor: an alias for `Reg<SYSTEM_STICK_CALIB_SPEC>`"]
pub type SYSTEM_STICK_CALIB = crate::Reg<system_stick_calib::SYSTEM_STICK_CALIB_SPEC>;
#[doc = "System secure tick calibration"]
pub mod system_stick_calib;
#[doc = "SYSTEM_NSTICK_CALIB (rw) register accessor: an alias for `Reg<SYSTEM_NSTICK_CALIB_SPEC>`"]
pub type SYSTEM_NSTICK_CALIB = crate::Reg<system_nstick_calib::SYSTEM_NSTICK_CALIB_SPEC>;
#[doc = "System non-secure tick calibration"]
pub mod system_nstick_calib;
#[doc = "PRODUCT_ID (rw) register accessor: an alias for `Reg<PRODUCT_ID_SPEC>`"]
pub type PRODUCT_ID = crate::Reg<product_id::PRODUCT_ID_SPEC>;
#[doc = "Product ID"]
pub mod product_id;
#[doc = "SILICONREV_ID (r) register accessor: an alias for `Reg<SILICONREV_ID_SPEC>`"]
pub type SILICONREV_ID = crate::Reg<siliconrev_id::SILICONREV_ID_SPEC>;
#[doc = "Silicon Revision ID"]
pub mod siliconrev_id;
#[doc = "JTAG_ID (r) register accessor: an alias for `Reg<JTAG_ID_SPEC>`"]
pub type JTAG_ID = crate::Reg<jtag_id::JTAG_ID_SPEC>;
#[doc = "JTAG ID"]
pub mod jtag_id;
#[doc = "NSGPIO_PSYNC (rw) register accessor: an alias for `Reg<NSGPIO_PSYNC_SPEC>`"]
pub type NSGPIO_PSYNC = crate::Reg<nsgpio_psync::NSGPIO_PSYNC_SPEC>;
#[doc = "Non-secure GPIO PSYNC"]
pub mod nsgpio_psync;
#[doc = "SGPIO_PSYNC (rw) register accessor: an alias for `Reg<SGPIO_PSYNC_SPEC>`"]
pub type SGPIO_PSYNC = crate::Reg<sgpio_psync::SGPIO_PSYNC_SPEC>;
#[doc = "Secure GPIO PSYNC"]
pub mod sgpio_psync;
#[doc = "AUTOCLKGATEOVERRIDE0 (rw) register accessor: an alias for `Reg<AUTOCLKGATEOVERRIDE0_SPEC>`"]
pub type AUTOCLKGATEOVERRIDE0 = crate::Reg<autoclkgateoverride0::AUTOCLKGATEOVERRIDE0_SPEC>;
#[doc = "Auto clock gate override 0"]
pub mod autoclkgateoverride0;
#[doc = "AUTOCLKGATEOVERRIDE1 (rw) register accessor: an alias for `Reg<AUTOCLKGATEOVERRIDE1_SPEC>`"]
pub type AUTOCLKGATEOVERRIDE1 = crate::Reg<autoclkgateoverride1::AUTOCLKGATEOVERRIDE1_SPEC>;
#[doc = "Auto clock gate override 1"]
pub mod autoclkgateoverride1;
#[doc = "CLKGATEOVERRIDE0 (rw) register accessor: an alias for `Reg<CLKGATEOVERRIDE0_SPEC>`"]
pub type CLKGATEOVERRIDE0 = crate::Reg<clkgateoverride0::CLKGATEOVERRIDE0_SPEC>;
#[doc = "Clock gate override 0"]
pub mod clkgateoverride0;
#[doc = "AHB_SRAM_ACCESS_DISABLE (rw) register accessor: an alias for `Reg<AHB_SRAM_ACCESS_DISABLE_SPEC>`"]
pub type AHB_SRAM_ACCESS_DISABLE =
    crate::Reg<ahb_sram_access_disable::AHB_SRAM_ACCESS_DISABLE_SPEC>;
#[doc = "AHB SRAM access disable"]
pub mod ahb_sram_access_disable;
#[doc = "AXI_SRAM_ACCESS_DISABLE (rw) register accessor: an alias for `Reg<AXI_SRAM_ACCESS_DISABLE_SPEC>`"]
pub type AXI_SRAM_ACCESS_DISABLE =
    crate::Reg<axi_sram_access_disable::AXI_SRAM_ACCESS_DISABLE_SPEC>;
#[doc = "AXI SRAM access disable"]
pub mod axi_sram_access_disable;
#[doc = "DSP_SRAM_ACCESS_DISABLE (rw) register accessor: an alias for `Reg<DSP_SRAM_ACCESS_DISABLE_SPEC>`"]
pub type DSP_SRAM_ACCESS_DISABLE =
    crate::Reg<dsp_sram_access_disable::DSP_SRAM_ACCESS_DISABLE_SPEC>;
#[doc = "DSP SRAM access disable"]
pub mod dsp_sram_access_disable;
#[doc = "PQ_MEM_CTRL (rw) register accessor: an alias for `Reg<PQ_MEM_CTRL_SPEC>`"]
pub type PQ_MEM_CTRL = crate::Reg<pq_mem_ctrl::PQ_MEM_CTRL_SPEC>;
#[doc = "Power-Quad Memory Control"]
pub mod pq_mem_ctrl;
#[doc = "FLEXSPI0_MEM_CTRL (rw) register accessor: an alias for `Reg<FLEXSPI0_MEM_CTRL_SPEC>`"]
pub type FLEXSPI0_MEM_CTRL = crate::Reg<flexspi0_mem_ctrl::FLEXSPI0_MEM_CTRL_SPEC>;
#[doc = "FlexSPI0 Memory Control"]
pub mod flexspi0_mem_ctrl;
#[doc = "USBHS_MEM_CTRL (rw) register accessor: an alias for `Reg<USBHS_MEM_CTRL_SPEC>`"]
pub type USBHS_MEM_CTRL = crate::Reg<usbhs_mem_ctrl::USBHS_MEM_CTRL_SPEC>;
#[doc = "USBHS Memory Control"]
pub mod usbhs_mem_ctrl;
#[doc = "USDHC0_MEM_CTRL (rw) register accessor: an alias for `Reg<USDHC0_MEM_CTRL_SPEC>`"]
pub type USDHC0_MEM_CTRL = crate::Reg<usdhc0_mem_ctrl::USDHC0_MEM_CTRL_SPEC>;
#[doc = "USDHC0 Memory Control"]
pub mod usdhc0_mem_ctrl;
#[doc = "USDHC1_MEM_CTRL (rw) register accessor: an alias for `Reg<USDHC1_MEM_CTRL_SPEC>`"]
pub type USDHC1_MEM_CTRL = crate::Reg<usdhc1_mem_ctrl::USDHC1_MEM_CTRL_SPEC>;
#[doc = "USDHC1 Memory Control"]
pub mod usdhc1_mem_ctrl;
#[doc = "CASPER_MEM_CTRL (rw) register accessor: an alias for `Reg<CASPER_MEM_CTRL_SPEC>`"]
pub type CASPER_MEM_CTRL = crate::Reg<casper_mem_ctrl::CASPER_MEM_CTRL_SPEC>;
#[doc = "CASPER Memory Control"]
pub mod casper_mem_ctrl;
#[doc = "ROM_MEM_CTRL (rw) register accessor: an alias for `Reg<ROM_MEM_CTRL_SPEC>`"]
pub type ROM_MEM_CTRL = crate::Reg<rom_mem_ctrl::ROM_MEM_CTRL_SPEC>;
#[doc = "ROM Memory Control"]
pub mod rom_mem_ctrl;
#[doc = "FlexSPI1_MEM_CTRL (rw) register accessor: an alias for `Reg<FLEX_SPI1_MEM_CTRL_SPEC>`"]
pub type FLEX_SPI1_MEM_CTRL = crate::Reg<flex_spi1_mem_ctrl::FLEX_SPI1_MEM_CTRL_SPEC>;
#[doc = "FlexSPI1 Memory Control"]
pub mod flex_spi1_mem_ctrl;
#[doc = "GPU_MEM_CTRL (rw) register accessor: an alias for `Reg<GPU_MEM_CTRL_SPEC>`"]
pub type GPU_MEM_CTRL = crate::Reg<gpu_mem_ctrl::GPU_MEM_CTRL_SPEC>;
#[doc = "GPU Memory Control"]
pub mod gpu_mem_ctrl;
#[doc = "MIPI_MEM_CTRL (rw) register accessor: an alias for `Reg<MIPI_MEM_CTRL_SPEC>`"]
pub type MIPI_MEM_CTRL = crate::Reg<mipi_mem_ctrl::MIPI_MEM_CTRL_SPEC>;
#[doc = "MIPI Memory Control"]
pub mod mipi_mem_ctrl;
#[doc = "DCN_MEM_CTRL (rw) register accessor: an alias for `Reg<DCN_MEM_CTRL_SPEC>`"]
pub type DCN_MEM_CTRL = crate::Reg<dcn_mem_ctrl::DCN_MEM_CTRL_SPEC>;
#[doc = "LCDIF Memory Control"]
pub mod dcn_mem_ctrl;
#[doc = "SMARTDMA_MEM_CTRL (rw) register accessor: an alias for `Reg<SMARTDMA_MEM_CTRL_SPEC>`"]
pub type SMARTDMA_MEM_CTRL = crate::Reg<smartdma_mem_ctrl::SMARTDMA_MEM_CTRL_SPEC>;
#[doc = "SMARTDMA Memory Control"]
pub mod smartdma_mem_ctrl;
#[doc = "MIPI_DSI_CTRL (rw) register accessor: an alias for `Reg<MIPI_DSI_CTRL_SPEC>`"]
pub type MIPI_DSI_CTRL = crate::Reg<mipi_dsi_ctrl::MIPI_DSI_CTRL_SPEC>;
#[doc = "MIPI DSI Control"]
pub mod mipi_dsi_ctrl;
#[doc = "USB0CLKCTRL (rw) register accessor: an alias for `Reg<USB0CLKCTRL_SPEC>`"]
pub type USB0CLKCTRL = crate::Reg<usb0clkctrl::USB0CLKCTRL_SPEC>;
#[doc = "USB Clock Control"]
pub mod usb0clkctrl;
#[doc = "USB0CLKSTAT (r) register accessor: an alias for `Reg<USB0CLKSTAT_SPEC>`"]
pub type USB0CLKSTAT = crate::Reg<usb0clkstat::USB0CLKSTAT_SPEC>;
#[doc = "USB Clock Status"]
pub mod usb0clkstat;
#[doc = "USBPHYPLL0LOCKTIMEDIV2 (rw) register accessor: an alias for `Reg<USBPHYPLL0LOCKTIMEDIV2_SPEC>`"]
pub type USBPHYPLL0LOCKTIMEDIV2 = crate::Reg<usbphypll0locktimediv2::USBPHYPLL0LOCKTIMEDIV2_SPEC>;
#[doc = "USB PHY PLL0 lock time division"]
pub mod usbphypll0locktimediv2;
#[doc = "PDSLEEPCFG0 (rw) register accessor: an alias for `Reg<PDSLEEPCFG0_SPEC>`"]
pub type PDSLEEPCFG0 = crate::Reg<pdsleepcfg0::PDSLEEPCFG0_SPEC>;
#[doc = "Sleep configuration 0"]
pub mod pdsleepcfg0;
#[doc = "PDSLEEPCFG1 (rw) register accessor: an alias for `Reg<PDSLEEPCFG1_SPEC>`"]
pub type PDSLEEPCFG1 = crate::Reg<pdsleepcfg1::PDSLEEPCFG1_SPEC>;
#[doc = "Sleep configuration 1"]
pub mod pdsleepcfg1;
#[doc = "PDSLEEPCFG2 (rw) register accessor: an alias for `Reg<PDSLEEPCFG2_SPEC>`"]
pub type PDSLEEPCFG2 = crate::Reg<pdsleepcfg2::PDSLEEPCFG2_SPEC>;
#[doc = "Sleep configuration 2"]
pub mod pdsleepcfg2;
#[doc = "PDSLEEPCFG3 (rw) register accessor: an alias for `Reg<PDSLEEPCFG3_SPEC>`"]
pub type PDSLEEPCFG3 = crate::Reg<pdsleepcfg3::PDSLEEPCFG3_SPEC>;
#[doc = "Sleep configuration 3"]
pub mod pdsleepcfg3;
#[doc = "PDRUNCFG0 (rw) register accessor: an alias for `Reg<PDRUNCFG0_SPEC>`"]
pub type PDRUNCFG0 = crate::Reg<pdruncfg0::PDRUNCFG0_SPEC>;
#[doc = "Run configuration 0"]
pub mod pdruncfg0;
#[doc = "PDRUNCFG1 (rw) register accessor: an alias for `Reg<PDRUNCFG1_SPEC>`"]
pub type PDRUNCFG1 = crate::Reg<pdruncfg1::PDRUNCFG1_SPEC>;
#[doc = "Run configuration 1"]
pub mod pdruncfg1;
#[doc = "PDRUNCFG2 (rw) register accessor: an alias for `Reg<PDRUNCFG2_SPEC>`"]
pub type PDRUNCFG2 = crate::Reg<pdruncfg2::PDRUNCFG2_SPEC>;
#[doc = "Run configuration 2"]
pub mod pdruncfg2;
#[doc = "PDRUNCFG3 (rw) register accessor: an alias for `Reg<PDRUNCFG3_SPEC>`"]
pub type PDRUNCFG3 = crate::Reg<pdruncfg3::PDRUNCFG3_SPEC>;
#[doc = "Run configuration 3"]
pub mod pdruncfg3;
#[doc = "PDRUNCFG0_SET (w) register accessor: an alias for `Reg<PDRUNCFG0_SET_SPEC>`"]
pub type PDRUNCFG0_SET = crate::Reg<pdruncfg0_set::PDRUNCFG0_SET_SPEC>;
#[doc = "Run configuration 0 set"]
pub mod pdruncfg0_set;
#[doc = "PDRUNCFG1_SET (w) register accessor: an alias for `Reg<PDRUNCFG1_SET_SPEC>`"]
pub type PDRUNCFG1_SET = crate::Reg<pdruncfg1_set::PDRUNCFG1_SET_SPEC>;
#[doc = "Run configuration 1 set"]
pub mod pdruncfg1_set;
#[doc = "PDRUNCFG2_SET (w) register accessor: an alias for `Reg<PDRUNCFG2_SET_SPEC>`"]
pub type PDRUNCFG2_SET = crate::Reg<pdruncfg2_set::PDRUNCFG2_SET_SPEC>;
#[doc = "Run configuration 2 set"]
pub mod pdruncfg2_set;
#[doc = "PDRUNCFG3_SET (w) register accessor: an alias for `Reg<PDRUNCFG3_SET_SPEC>`"]
pub type PDRUNCFG3_SET = crate::Reg<pdruncfg3_set::PDRUNCFG3_SET_SPEC>;
#[doc = "Run configuration 3 set"]
pub mod pdruncfg3_set;
#[doc = "PDRUNCFG0_CLR (w) register accessor: an alias for `Reg<PDRUNCFG0_CLR_SPEC>`"]
pub type PDRUNCFG0_CLR = crate::Reg<pdruncfg0_clr::PDRUNCFG0_CLR_SPEC>;
#[doc = "Run configuration 0 clear"]
pub mod pdruncfg0_clr;
#[doc = "PDRUNCFG1_CLR (w) register accessor: an alias for `Reg<PDRUNCFG1_CLR_SPEC>`"]
pub type PDRUNCFG1_CLR = crate::Reg<pdruncfg1_clr::PDRUNCFG1_CLR_SPEC>;
#[doc = "Run configuration 1 clear"]
pub mod pdruncfg1_clr;
#[doc = "PDRUNCFG2_CLR (rw) register accessor: an alias for `Reg<PDRUNCFG2_CLR_SPEC>`"]
pub type PDRUNCFG2_CLR = crate::Reg<pdruncfg2_clr::PDRUNCFG2_CLR_SPEC>;
#[doc = "Run configuration 2 clear"]
pub mod pdruncfg2_clr;
#[doc = "PDRUNCFG3_CLR (rw) register accessor: an alias for `Reg<PDRUNCFG3_CLR_SPEC>`"]
pub type PDRUNCFG3_CLR = crate::Reg<pdruncfg3_clr::PDRUNCFG3_CLR_SPEC>;
#[doc = "Run configuration 3 clear"]
pub mod pdruncfg3_clr;
#[doc = "PDWAKECFG (rw) register accessor: an alias for `Reg<PDWAKECFG_SPEC>`"]
pub type PDWAKECFG = crate::Reg<pdwakecfg::PDWAKECFG_SPEC>;
#[doc = "PD Wake Configuration"]
pub mod pdwakecfg;
#[doc = "STARTEN0 (rw) register accessor: an alias for `Reg<STARTEN0_SPEC>`"]
pub type STARTEN0 = crate::Reg<starten0::STARTEN0_SPEC>;
#[doc = "Start Enable 0"]
pub mod starten0;
#[doc = "STARTEN1 (rw) register accessor: an alias for `Reg<STARTEN1_SPEC>`"]
pub type STARTEN1 = crate::Reg<starten1::STARTEN1_SPEC>;
#[doc = "Start Enable 1"]
pub mod starten1;
#[doc = "STARTEN2 (rw) register accessor: an alias for `Reg<STARTEN2_SPEC>`"]
pub type STARTEN2 = crate::Reg<starten2::STARTEN2_SPEC>;
#[doc = "Start Enable 2"]
pub mod starten2;
#[doc = "STARTEN0_SET (rw) register accessor: an alias for `Reg<STARTEN0_SET_SPEC>`"]
pub type STARTEN0_SET = crate::Reg<starten0_set::STARTEN0_SET_SPEC>;
#[doc = "Start Enable 0 Set"]
pub mod starten0_set;
#[doc = "STARTEN1_SET (rw) register accessor: an alias for `Reg<STARTEN1_SET_SPEC>`"]
pub type STARTEN1_SET = crate::Reg<starten1_set::STARTEN1_SET_SPEC>;
#[doc = "Start Enable 1 Set"]
pub mod starten1_set;
#[doc = "STARTEN2_SET (rw) register accessor: an alias for `Reg<STARTEN2_SET_SPEC>`"]
pub type STARTEN2_SET = crate::Reg<starten2_set::STARTEN2_SET_SPEC>;
#[doc = "Start Enable 2"]
pub mod starten2_set;
#[doc = "STARTEN0_CLR (rw) register accessor: an alias for `Reg<STARTEN0_CLR_SPEC>`"]
pub type STARTEN0_CLR = crate::Reg<starten0_clr::STARTEN0_CLR_SPEC>;
#[doc = "Start Enable 0 clear"]
pub mod starten0_clr;
#[doc = "STARTEN1_CLR (rw) register accessor: an alias for `Reg<STARTEN1_CLR_SPEC>`"]
pub type STARTEN1_CLR = crate::Reg<starten1_clr::STARTEN1_CLR_SPEC>;
#[doc = "Start Enable 1 clear"]
pub mod starten1_clr;
#[doc = "STARTEN2_CLR (rw) register accessor: an alias for `Reg<STARTEN2_CLR_SPEC>`"]
pub type STARTEN2_CLR = crate::Reg<starten2_clr::STARTEN2_CLR_SPEC>;
#[doc = "Start Enable 2"]
pub mod starten2_clr;
#[doc = "FROSAFETY (rw) register accessor: an alias for `Reg<FROSAFETY_SPEC>`"]
pub type FROSAFETY = crate::Reg<frosafety::FROSAFETY_SPEC>;
#[doc = "FRO Safety"]
pub mod frosafety;
#[doc = "MAINCLKSAFETY (rw) register accessor: an alias for `Reg<MAINCLKSAFETY_SPEC>`"]
pub type MAINCLKSAFETY = crate::Reg<mainclksafety::MAINCLKSAFETY_SPEC>;
#[doc = "Main Clock Safety"]
pub mod mainclksafety;
#[doc = "HWWAKE (rw) register accessor: an alias for `Reg<HWWAKE_SPEC>`"]
pub type HWWAKE = crate::Reg<hwwake::HWWAKE_SPEC>;
#[doc = "Hardware Wake"]
pub mod hwwake;
#[doc = "TEMPSENSORCTL (rw) register accessor: an alias for `Reg<TEMPSENSORCTL_SPEC>`"]
pub type TEMPSENSORCTL = crate::Reg<tempsensorctl::TEMPSENSORCTL_SPEC>;
#[doc = "Temperature Sensor Control"]
pub mod tempsensorctl;
#[doc = "BOOTSTATELOCK (rw) register accessor: an alias for `Reg<BOOTSTATELOCK_SPEC>`"]
pub type BOOTSTATELOCK = crate::Reg<bootstatelock::BOOTSTATELOCK_SPEC>;
#[doc = "Boot State Lock"]
pub mod bootstatelock;
#[doc = "BOOTSTATESEED (rw) register accessor: an alias for `Reg<BOOTSTATESEED_SPEC>`"]
pub type BOOTSTATESEED = crate::Reg<bootstateseed::BOOTSTATESEED_SPEC>;
#[doc = "Boot State Seed"]
pub mod bootstateseed;
#[doc = "BOOTSTATEHMAC (rw) register accessor: an alias for `Reg<BOOTSTATEHMAC_SPEC>`"]
pub type BOOTSTATEHMAC = crate::Reg<bootstatehmac::BOOTSTATEHMAC_SPEC>;
#[doc = "HMAC of boot state used for attestation."]
pub mod bootstatehmac;
#[doc = "FLEXSPI0PADCTL (rw) register accessor: an alias for `Reg<FLEXSPI0PADCTL_SPEC>`"]
pub type FLEXSPI0PADCTL = crate::Reg<flexspi0padctl::FLEXSPI0PADCTL_SPEC>;
#[doc = "FLEXSPI0 Pad Control"]
pub mod flexspi0padctl;
#[doc = "FLEXSPI1PADCTL (rw) register accessor: an alias for `Reg<FLEXSPI1PADCTL_SPEC>`"]
pub type FLEXSPI1PADCTL = crate::Reg<flexspi1padctl::FLEXSPI1PADCTL_SPEC>;
#[doc = "FLEXSPI1 Pad Control"]
pub mod flexspi1padctl;
#[doc = "SDIO0PADCTL (rw) register accessor: an alias for `Reg<SDIO0PADCTL_SPEC>`"]
pub type SDIO0PADCTL = crate::Reg<sdio0padctl::SDIO0PADCTL_SPEC>;
#[doc = "SDIO0 Pad Control"]
pub mod sdio0padctl;
#[doc = "SDIO1PADCTL (rw) register accessor: an alias for `Reg<SDIO1PADCTL_SPEC>`"]
pub type SDIO1PADCTL = crate::Reg<sdio1padctl::SDIO1PADCTL_SPEC>;
#[doc = "SDIO1 Pad Control"]
pub mod sdio1padctl;
#[doc = "DICEHWREGn (rw) register accessor: an alias for `Reg<DICEHWREGN_SPEC>`"]
pub type DICEHWREGN = crate::Reg<dicehwregn::DICEHWREGN_SPEC>;
#[doc = "Compound Device Identifier (CDI)"]
pub mod dicehwregn;
#[doc = "UUID (rw) register accessor: an alias for `Reg<UUID_SPEC>`"]
pub type UUID = crate::Reg<uuid::UUID_SPEC>;
#[doc = "UUID"]
pub mod uuid;
#[doc = "AESKEY_SRCSEL (rw) register accessor: an alias for `Reg<AESKEY_SRCSEL_SPEC>`"]
pub type AESKEY_SRCSEL = crate::Reg<aeskey_srcsel::AESKEY_SRCSEL_SPEC>;
#[doc = "AES Key Source Select"]
pub mod aeskey_srcsel;
#[doc = "OTFADKEY_SRCSEL (rw) register accessor: an alias for `Reg<OTFADKEY_SRCSEL_SPEC>`"]
pub type OTFADKEY_SRCSEL = crate::Reg<otfadkey_srcsel::OTFADKEY_SRCSEL_SPEC>;
#[doc = "OTFAD Key Source Select"]
pub mod otfadkey_srcsel;
#[doc = "HASHHWKEYDISABLE (rw) register accessor: an alias for `Reg<HASHHWKEYDISABLE_SPEC>`"]
pub type HASHHWKEYDISABLE = crate::Reg<hashhwkeydisable::HASHHWKEYDISABLE_SPEC>;
#[doc = "HASH Hardware Key Disable"]
pub mod hashhwkeydisable;
#[doc = "DBG_LOCKEN (rw) register accessor: an alias for `Reg<DBG_LOCKEN_SPEC>`"]
pub type DBG_LOCKEN = crate::Reg<dbg_locken::DBG_LOCKEN_SPEC>;
#[doc = "Debug Lock Enable"]
pub mod dbg_locken;
#[doc = "DBG_FEATURES (rw) register accessor: an alias for `Reg<DBG_FEATURES_SPEC>`"]
pub type DBG_FEATURES = crate::Reg<dbg_features::DBG_FEATURES_SPEC>;
#[doc = "Debug Features"]
pub mod dbg_features;
#[doc = "DBG_FEATURES_DP (rw) register accessor: an alias for `Reg<DBG_FEATURES_DP_SPEC>`"]
pub type DBG_FEATURES_DP = crate::Reg<dbg_features_dp::DBG_FEATURES_DP_SPEC>;
#[doc = "Debug Features Duplicate"]
pub mod dbg_features_dp;
#[doc = "CS_PROTCPU0 (rw) register accessor: an alias for `Reg<CS_PROTCPU0_SPEC>`"]
pub type CS_PROTCPU0 = crate::Reg<cs_protcpu0::CS_PROTCPU0_SPEC>;
#[doc = "Code Security for CPU0"]
pub mod cs_protcpu0;
#[doc = "CS_PROTCPU1 (rw) register accessor: an alias for `Reg<CS_PROTCPU1_SPEC>`"]
pub type CS_PROTCPU1 = crate::Reg<cs_protcpu1::CS_PROTCPU1_SPEC>;
#[doc = "Code Security for CPU1"]
pub mod cs_protcpu1;
#[doc = "DBG_AUTH_SCRATCH (rw) register accessor: an alias for `Reg<DBG_AUTH_SCRATCH_SPEC>`"]
pub type DBG_AUTH_SCRATCH = crate::Reg<dbg_auth_scratch::DBG_AUTH_SCRATCH_SPEC>;
#[doc = "Debug authorization scratch"]
pub mod dbg_auth_scratch;
#[doc = "KEY_BLOCK (rw) register accessor: an alias for `Reg<KEY_BLOCK_SPEC>`"]
pub type KEY_BLOCK = crate::Reg<key_block::KEY_BLOCK_SPEC>;
#[doc = "Key block"]
pub mod key_block;
