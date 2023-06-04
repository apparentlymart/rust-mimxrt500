#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Power Down"]
    pub pwd: PWD,
    #[doc = "0x04 - Power Down Register Set"]
    pub pwd_set: PWD_SET,
    #[doc = "0x08 - Power Down Register Clear"]
    pub pwd_clr: PWD_CLR,
    #[doc = "0x0c - Power Down Register Toggle"]
    pub pwd_tog: PWD_TOG,
    #[doc = "0x10 - TX Control"]
    pub tx: TX,
    #[doc = "0x14 - TX Control Set"]
    pub tx_set: TX_SET,
    #[doc = "0x18 - TX Control Clear"]
    pub tx_clr: TX_CLR,
    #[doc = "0x1c - TX Control Toggle"]
    pub tx_tog: TX_TOG,
    #[doc = "0x20 - RX Control"]
    pub rx: RX,
    #[doc = "0x24 - RX Control Set"]
    pub rx_set: RX_SET,
    #[doc = "0x28 - RX Control Clear"]
    pub rx_clr: RX_CLR,
    #[doc = "0x2c - RX Control Toggle"]
    pub rx_tog: RX_TOG,
    #[doc = "0x30 - General Purpose Control"]
    pub ctrl: CTRL,
    #[doc = "0x34 - General Purpose Control Set"]
    pub ctrl_set: CTRL_SET,
    #[doc = "0x38 - General Purpose Control Clear"]
    pub ctrl_clr: CTRL_CLR,
    #[doc = "0x3c - General Purpose Control Toggle"]
    pub ctrl_tog: CTRL_TOG,
    #[doc = "0x40 - Status"]
    pub status: STATUS,
    _reserved17: [u8; 0x0c],
    #[doc = "0x50 - Debug 0"]
    pub debug0: DEBUG0,
    #[doc = "0x54 - Debug 0 Set"]
    pub debug0_set: DEBUG0_SET,
    #[doc = "0x58 - Debug Clear"]
    pub debug0_clr: DEBUG0_CLR,
    #[doc = "0x5c - Debug Toggle"]
    pub debug0_tog: DEBUG0_TOG,
    _reserved21: [u8; 0x10],
    #[doc = "0x70 - UTMI Debug 1"]
    pub debug1: DEBUG1,
    #[doc = "0x74 - UTMI Debug 1 Set"]
    pub debug1_set: DEBUG1_SET,
    #[doc = "0x78 - UTMI Debug 1 Clear"]
    pub debug1_clr: DEBUG1_CLR,
    #[doc = "0x7c - UTMI Debug 1 Toggle"]
    pub debug1_tog: DEBUG1_TOG,
    #[doc = "0x80 - Version"]
    pub version: VERSION,
    _reserved26: [u8; 0x1c],
    #[doc = "0xa0 - PLL Control/Status"]
    pub pll_sic: PLL_SIC,
    #[doc = "0xa4 - PLL Control/Status Set"]
    pub pll_sic_set: PLL_SIC_SET,
    #[doc = "0xa8 - PLL Control/Status Clear"]
    pub pll_sic_clr: PLL_SIC_CLR,
    #[doc = "0xac - PLL Control/Status Toggle"]
    pub pll_sic_tog: PLL_SIC_TOG,
    _reserved30: [u8; 0x10],
    #[doc = "0xc0 - VBUS detect"]
    pub usb1_vbus_detect: USB1_VBUS_DETECT,
    #[doc = "0xc4 - VBUS detect Set"]
    pub usb1_vbus_detect_set: USB1_VBUS_DETECT_SET,
    #[doc = "0xc8 - VBUS detect Clear"]
    pub usb1_vbus_detect_clr: USB1_VBUS_DETECT_CLR,
    #[doc = "0xcc - VBUS detect Toggle"]
    pub usb1_vbus_detect_tog: USB1_VBUS_DETECT_TOG,
    #[doc = "0xd0 - VBUS Detect Status"]
    pub usb1_vbus_det_stat: USB1_VBUS_DET_STAT,
    _reserved35: [u8; 0x0c],
    #[doc = "0xe0 - Charger Detect Control"]
    pub usb1_chrg_detect: USB1_CHRG_DETECT,
    #[doc = "0xe4 - Charger Detect Control Set"]
    pub usb1_chrg_detect_set: USB1_CHRG_DETECT_SET,
    #[doc = "0xe8 - Charger Detect Control Clear"]
    pub usb1_chrg_detect_clr: USB1_CHRG_DETECT_CLR,
    #[doc = "0xec - Charger Detect Control Toggle"]
    pub usb1_chrg_detect_tog: USB1_CHRG_DETECT_TOG,
    #[doc = "0xf0 - Charge Detect Status"]
    pub usb1_chrg_det_stat: USB1_CHRG_DET_STAT,
    _reserved40: [u8; 0x0c],
    #[doc = "0x100 - Analog Control"]
    pub anactrl: ANACTRL,
    #[doc = "0x104 - Analog Control Set"]
    pub anactrl_set: ANACTRL_SET,
    #[doc = "0x108 - Analog Control Clear"]
    pub anactrl_clr: ANACTRL_CLR,
    #[doc = "0x10c - Analog Control Toggle"]
    pub anactrl_tog: ANACTRL_TOG,
    #[doc = "0x110 - USB PHY Loopback Control/Status"]
    pub usb1_loopback: USB1_LOOPBACK,
    #[doc = "0x114 - USB PHY Loopback Control/Status Set"]
    pub usb1_loopback_set: USB1_LOOPBACK_SET,
    #[doc = "0x118 - USB PHY Loopback Control/Status Clear"]
    pub usb1_loopback_clr: USB1_LOOPBACK_CLR,
    #[doc = "0x11c - USB PHY Loopback Control/Status Toggle"]
    pub usb1_loopback_tog: USB1_LOOPBACK_TOG,
    #[doc = "0x120 - Loopback Packet Number Select"]
    pub usb1_loopback_hsfscnt: USB1_LOOPBACK_HSFSCNT,
    #[doc = "0x124 - USB PHY Loopback Packet Number Select Set"]
    pub usb1_loopback_hsfscnt_set: USB1_LOOPBACK_HSFSCNT_SET,
    #[doc = "0x128 - USB PHY Loopback Packet Number Select Clear"]
    pub usb1_loopback_hsfscnt_clr: USB1_LOOPBACK_HSFSCNT_CLR,
    #[doc = "0x12c - USB PHY Loopback Packet Number Select Toggle"]
    pub usb1_loopback_hsfscnt_tog: USB1_LOOPBACK_HSFSCNT_TOG,
    #[doc = "0x130 - Trim Override Enable"]
    pub trim_override_en: TRIM_OVERRIDE_EN,
    #[doc = "0x134 - Trim Set"]
    pub trim_override_en_set: TRIM_OVERRIDE_EN_SET,
    #[doc = "0x138 - Trim Clear"]
    pub trim_override_en_clr: TRIM_OVERRIDE_EN_CLR,
    #[doc = "0x13c - Trim Toggle"]
    pub trim_override_en_tog: TRIM_OVERRIDE_EN_TOG,
}
#[doc = "PWD (rw) register accessor: an alias for `Reg<PWD_SPEC>`"]
pub type PWD = crate::Reg<pwd::PWD_SPEC>;
#[doc = "Power Down"]
pub mod pwd;
#[doc = "PWD_SET (rw) register accessor: an alias for `Reg<PWD_SET_SPEC>`"]
pub type PWD_SET = crate::Reg<pwd_set::PWD_SET_SPEC>;
#[doc = "Power Down Register Set"]
pub mod pwd_set;
#[doc = "PWD_CLR (rw) register accessor: an alias for `Reg<PWD_CLR_SPEC>`"]
pub type PWD_CLR = crate::Reg<pwd_clr::PWD_CLR_SPEC>;
#[doc = "Power Down Register Clear"]
pub mod pwd_clr;
#[doc = "PWD_TOG (rw) register accessor: an alias for `Reg<PWD_TOG_SPEC>`"]
pub type PWD_TOG = crate::Reg<pwd_tog::PWD_TOG_SPEC>;
#[doc = "Power Down Register Toggle"]
pub mod pwd_tog;
#[doc = "TX (rw) register accessor: an alias for `Reg<TX_SPEC>`"]
pub type TX = crate::Reg<tx::TX_SPEC>;
#[doc = "TX Control"]
pub mod tx;
#[doc = "TX_SET (rw) register accessor: an alias for `Reg<TX_SET_SPEC>`"]
pub type TX_SET = crate::Reg<tx_set::TX_SET_SPEC>;
#[doc = "TX Control Set"]
pub mod tx_set;
#[doc = "TX_CLR (rw) register accessor: an alias for `Reg<TX_CLR_SPEC>`"]
pub type TX_CLR = crate::Reg<tx_clr::TX_CLR_SPEC>;
#[doc = "TX Control Clear"]
pub mod tx_clr;
#[doc = "TX_TOG (rw) register accessor: an alias for `Reg<TX_TOG_SPEC>`"]
pub type TX_TOG = crate::Reg<tx_tog::TX_TOG_SPEC>;
#[doc = "TX Control Toggle"]
pub mod tx_tog;
#[doc = "RX (rw) register accessor: an alias for `Reg<RX_SPEC>`"]
pub type RX = crate::Reg<rx::RX_SPEC>;
#[doc = "RX Control"]
pub mod rx;
#[doc = "RX_SET (rw) register accessor: an alias for `Reg<RX_SET_SPEC>`"]
pub type RX_SET = crate::Reg<rx_set::RX_SET_SPEC>;
#[doc = "RX Control Set"]
pub mod rx_set;
#[doc = "RX_CLR (rw) register accessor: an alias for `Reg<RX_CLR_SPEC>`"]
pub type RX_CLR = crate::Reg<rx_clr::RX_CLR_SPEC>;
#[doc = "RX Control Clear"]
pub mod rx_clr;
#[doc = "RX_TOG (rw) register accessor: an alias for `Reg<RX_TOG_SPEC>`"]
pub type RX_TOG = crate::Reg<rx_tog::RX_TOG_SPEC>;
#[doc = "RX Control Toggle"]
pub mod rx_tog;
#[doc = "CTRL (rw) register accessor: an alias for `Reg<CTRL_SPEC>`"]
pub type CTRL = crate::Reg<ctrl::CTRL_SPEC>;
#[doc = "General Purpose Control"]
pub mod ctrl;
#[doc = "CTRL_SET (rw) register accessor: an alias for `Reg<CTRL_SET_SPEC>`"]
pub type CTRL_SET = crate::Reg<ctrl_set::CTRL_SET_SPEC>;
#[doc = "General Purpose Control Set"]
pub mod ctrl_set;
#[doc = "CTRL_CLR (rw) register accessor: an alias for `Reg<CTRL_CLR_SPEC>`"]
pub type CTRL_CLR = crate::Reg<ctrl_clr::CTRL_CLR_SPEC>;
#[doc = "General Purpose Control Clear"]
pub mod ctrl_clr;
#[doc = "CTRL_TOG (rw) register accessor: an alias for `Reg<CTRL_TOG_SPEC>`"]
pub type CTRL_TOG = crate::Reg<ctrl_tog::CTRL_TOG_SPEC>;
#[doc = "General Purpose Control Toggle"]
pub mod ctrl_tog;
#[doc = "STATUS (r) register accessor: an alias for `Reg<STATUS_SPEC>`"]
pub type STATUS = crate::Reg<status::STATUS_SPEC>;
#[doc = "Status"]
pub mod status;
#[doc = "DEBUG0 (rw) register accessor: an alias for `Reg<DEBUG0_SPEC>`"]
pub type DEBUG0 = crate::Reg<debug0::DEBUG0_SPEC>;
#[doc = "Debug 0"]
pub mod debug0;
#[doc = "DEBUG0_SET (rw) register accessor: an alias for `Reg<DEBUG0_SET_SPEC>`"]
pub type DEBUG0_SET = crate::Reg<debug0_set::DEBUG0_SET_SPEC>;
#[doc = "Debug 0 Set"]
pub mod debug0_set;
#[doc = "DEBUG0_CLR (rw) register accessor: an alias for `Reg<DEBUG0_CLR_SPEC>`"]
pub type DEBUG0_CLR = crate::Reg<debug0_clr::DEBUG0_CLR_SPEC>;
#[doc = "Debug Clear"]
pub mod debug0_clr;
#[doc = "DEBUG0_TOG (rw) register accessor: an alias for `Reg<DEBUG0_TOG_SPEC>`"]
pub type DEBUG0_TOG = crate::Reg<debug0_tog::DEBUG0_TOG_SPEC>;
#[doc = "Debug Toggle"]
pub mod debug0_tog;
#[doc = "DEBUG1 (rw) register accessor: an alias for `Reg<DEBUG1_SPEC>`"]
pub type DEBUG1 = crate::Reg<debug1::DEBUG1_SPEC>;
#[doc = "UTMI Debug 1"]
pub mod debug1;
#[doc = "DEBUG1_SET (rw) register accessor: an alias for `Reg<DEBUG1_SET_SPEC>`"]
pub type DEBUG1_SET = crate::Reg<debug1_set::DEBUG1_SET_SPEC>;
#[doc = "UTMI Debug 1 Set"]
pub mod debug1_set;
#[doc = "DEBUG1_CLR (rw) register accessor: an alias for `Reg<DEBUG1_CLR_SPEC>`"]
pub type DEBUG1_CLR = crate::Reg<debug1_clr::DEBUG1_CLR_SPEC>;
#[doc = "UTMI Debug 1 Clear"]
pub mod debug1_clr;
#[doc = "DEBUG1_TOG (rw) register accessor: an alias for `Reg<DEBUG1_TOG_SPEC>`"]
pub type DEBUG1_TOG = crate::Reg<debug1_tog::DEBUG1_TOG_SPEC>;
#[doc = "UTMI Debug 1 Toggle"]
pub mod debug1_tog;
#[doc = "VERSION (r) register accessor: an alias for `Reg<VERSION_SPEC>`"]
pub type VERSION = crate::Reg<version::VERSION_SPEC>;
#[doc = "Version"]
pub mod version;
#[doc = "PLL_SIC (rw) register accessor: an alias for `Reg<PLL_SIC_SPEC>`"]
pub type PLL_SIC = crate::Reg<pll_sic::PLL_SIC_SPEC>;
#[doc = "PLL Control/Status"]
pub mod pll_sic;
#[doc = "PLL_SIC_SET (rw) register accessor: an alias for `Reg<PLL_SIC_SET_SPEC>`"]
pub type PLL_SIC_SET = crate::Reg<pll_sic_set::PLL_SIC_SET_SPEC>;
#[doc = "PLL Control/Status Set"]
pub mod pll_sic_set;
#[doc = "PLL_SIC_CLR (rw) register accessor: an alias for `Reg<PLL_SIC_CLR_SPEC>`"]
pub type PLL_SIC_CLR = crate::Reg<pll_sic_clr::PLL_SIC_CLR_SPEC>;
#[doc = "PLL Control/Status Clear"]
pub mod pll_sic_clr;
#[doc = "PLL_SIC_TOG (rw) register accessor: an alias for `Reg<PLL_SIC_TOG_SPEC>`"]
pub type PLL_SIC_TOG = crate::Reg<pll_sic_tog::PLL_SIC_TOG_SPEC>;
#[doc = "PLL Control/Status Toggle"]
pub mod pll_sic_tog;
#[doc = "USB1_VBUS_DETECT (rw) register accessor: an alias for `Reg<USB1_VBUS_DETECT_SPEC>`"]
pub type USB1_VBUS_DETECT = crate::Reg<usb1_vbus_detect::USB1_VBUS_DETECT_SPEC>;
#[doc = "VBUS detect"]
pub mod usb1_vbus_detect;
#[doc = "USB1_VBUS_DETECT_SET (rw) register accessor: an alias for `Reg<USB1_VBUS_DETECT_SET_SPEC>`"]
pub type USB1_VBUS_DETECT_SET = crate::Reg<usb1_vbus_detect_set::USB1_VBUS_DETECT_SET_SPEC>;
#[doc = "VBUS detect Set"]
pub mod usb1_vbus_detect_set;
#[doc = "USB1_VBUS_DETECT_CLR (rw) register accessor: an alias for `Reg<USB1_VBUS_DETECT_CLR_SPEC>`"]
pub type USB1_VBUS_DETECT_CLR = crate::Reg<usb1_vbus_detect_clr::USB1_VBUS_DETECT_CLR_SPEC>;
#[doc = "VBUS detect Clear"]
pub mod usb1_vbus_detect_clr;
#[doc = "USB1_VBUS_DETECT_TOG (rw) register accessor: an alias for `Reg<USB1_VBUS_DETECT_TOG_SPEC>`"]
pub type USB1_VBUS_DETECT_TOG = crate::Reg<usb1_vbus_detect_tog::USB1_VBUS_DETECT_TOG_SPEC>;
#[doc = "VBUS detect Toggle"]
pub mod usb1_vbus_detect_tog;
#[doc = "USB1_VBUS_DET_STAT (r) register accessor: an alias for `Reg<USB1_VBUS_DET_STAT_SPEC>`"]
pub type USB1_VBUS_DET_STAT = crate::Reg<usb1_vbus_det_stat::USB1_VBUS_DET_STAT_SPEC>;
#[doc = "VBUS Detect Status"]
pub mod usb1_vbus_det_stat;
#[doc = "USB1_CHRG_DETECT (rw) register accessor: an alias for `Reg<USB1_CHRG_DETECT_SPEC>`"]
pub type USB1_CHRG_DETECT = crate::Reg<usb1_chrg_detect::USB1_CHRG_DETECT_SPEC>;
#[doc = "Charger Detect Control"]
pub mod usb1_chrg_detect;
#[doc = "USB1_CHRG_DETECT_SET (rw) register accessor: an alias for `Reg<USB1_CHRG_DETECT_SET_SPEC>`"]
pub type USB1_CHRG_DETECT_SET = crate::Reg<usb1_chrg_detect_set::USB1_CHRG_DETECT_SET_SPEC>;
#[doc = "Charger Detect Control Set"]
pub mod usb1_chrg_detect_set;
#[doc = "USB1_CHRG_DETECT_CLR (rw) register accessor: an alias for `Reg<USB1_CHRG_DETECT_CLR_SPEC>`"]
pub type USB1_CHRG_DETECT_CLR = crate::Reg<usb1_chrg_detect_clr::USB1_CHRG_DETECT_CLR_SPEC>;
#[doc = "Charger Detect Control Clear"]
pub mod usb1_chrg_detect_clr;
#[doc = "USB1_CHRG_DETECT_TOG (rw) register accessor: an alias for `Reg<USB1_CHRG_DETECT_TOG_SPEC>`"]
pub type USB1_CHRG_DETECT_TOG = crate::Reg<usb1_chrg_detect_tog::USB1_CHRG_DETECT_TOG_SPEC>;
#[doc = "Charger Detect Control Toggle"]
pub mod usb1_chrg_detect_tog;
#[doc = "USB1_CHRG_DET_STAT (r) register accessor: an alias for `Reg<USB1_CHRG_DET_STAT_SPEC>`"]
pub type USB1_CHRG_DET_STAT = crate::Reg<usb1_chrg_det_stat::USB1_CHRG_DET_STAT_SPEC>;
#[doc = "Charge Detect Status"]
pub mod usb1_chrg_det_stat;
#[doc = "ANACTRL (rw) register accessor: an alias for `Reg<ANACTRL_SPEC>`"]
pub type ANACTRL = crate::Reg<anactrl::ANACTRL_SPEC>;
#[doc = "Analog Control"]
pub mod anactrl;
#[doc = "ANACTRL_SET (rw) register accessor: an alias for `Reg<ANACTRL_SET_SPEC>`"]
pub type ANACTRL_SET = crate::Reg<anactrl_set::ANACTRL_SET_SPEC>;
#[doc = "Analog Control Set"]
pub mod anactrl_set;
#[doc = "ANACTRL_CLR (rw) register accessor: an alias for `Reg<ANACTRL_CLR_SPEC>`"]
pub type ANACTRL_CLR = crate::Reg<anactrl_clr::ANACTRL_CLR_SPEC>;
#[doc = "Analog Control Clear"]
pub mod anactrl_clr;
#[doc = "ANACTRL_TOG (rw) register accessor: an alias for `Reg<ANACTRL_TOG_SPEC>`"]
pub type ANACTRL_TOG = crate::Reg<anactrl_tog::ANACTRL_TOG_SPEC>;
#[doc = "Analog Control Toggle"]
pub mod anactrl_tog;
#[doc = "USB1_LOOPBACK (rw) register accessor: an alias for `Reg<USB1_LOOPBACK_SPEC>`"]
pub type USB1_LOOPBACK = crate::Reg<usb1_loopback::USB1_LOOPBACK_SPEC>;
#[doc = "USB PHY Loopback Control/Status"]
pub mod usb1_loopback;
#[doc = "USB1_LOOPBACK_SET (rw) register accessor: an alias for `Reg<USB1_LOOPBACK_SET_SPEC>`"]
pub type USB1_LOOPBACK_SET = crate::Reg<usb1_loopback_set::USB1_LOOPBACK_SET_SPEC>;
#[doc = "USB PHY Loopback Control/Status Set"]
pub mod usb1_loopback_set;
#[doc = "USB1_LOOPBACK_CLR (rw) register accessor: an alias for `Reg<USB1_LOOPBACK_CLR_SPEC>`"]
pub type USB1_LOOPBACK_CLR = crate::Reg<usb1_loopback_clr::USB1_LOOPBACK_CLR_SPEC>;
#[doc = "USB PHY Loopback Control/Status Clear"]
pub mod usb1_loopback_clr;
#[doc = "USB1_LOOPBACK_TOG (rw) register accessor: an alias for `Reg<USB1_LOOPBACK_TOG_SPEC>`"]
pub type USB1_LOOPBACK_TOG = crate::Reg<usb1_loopback_tog::USB1_LOOPBACK_TOG_SPEC>;
#[doc = "USB PHY Loopback Control/Status Toggle"]
pub mod usb1_loopback_tog;
#[doc = "USB1_LOOPBACK_HSFSCNT (rw) register accessor: an alias for `Reg<USB1_LOOPBACK_HSFSCNT_SPEC>`"]
pub type USB1_LOOPBACK_HSFSCNT = crate::Reg<usb1_loopback_hsfscnt::USB1_LOOPBACK_HSFSCNT_SPEC>;
#[doc = "Loopback Packet Number Select"]
pub mod usb1_loopback_hsfscnt;
#[doc = "USB1_LOOPBACK_HSFSCNT_SET (rw) register accessor: an alias for `Reg<USB1_LOOPBACK_HSFSCNT_SET_SPEC>`"]
pub type USB1_LOOPBACK_HSFSCNT_SET =
    crate::Reg<usb1_loopback_hsfscnt_set::USB1_LOOPBACK_HSFSCNT_SET_SPEC>;
#[doc = "USB PHY Loopback Packet Number Select Set"]
pub mod usb1_loopback_hsfscnt_set;
#[doc = "USB1_LOOPBACK_HSFSCNT_CLR (rw) register accessor: an alias for `Reg<USB1_LOOPBACK_HSFSCNT_CLR_SPEC>`"]
pub type USB1_LOOPBACK_HSFSCNT_CLR =
    crate::Reg<usb1_loopback_hsfscnt_clr::USB1_LOOPBACK_HSFSCNT_CLR_SPEC>;
#[doc = "USB PHY Loopback Packet Number Select Clear"]
pub mod usb1_loopback_hsfscnt_clr;
#[doc = "USB1_LOOPBACK_HSFSCNT_TOG (rw) register accessor: an alias for `Reg<USB1_LOOPBACK_HSFSCNT_TOG_SPEC>`"]
pub type USB1_LOOPBACK_HSFSCNT_TOG =
    crate::Reg<usb1_loopback_hsfscnt_tog::USB1_LOOPBACK_HSFSCNT_TOG_SPEC>;
#[doc = "USB PHY Loopback Packet Number Select Toggle"]
pub mod usb1_loopback_hsfscnt_tog;
#[doc = "TRIM_OVERRIDE_EN (rw) register accessor: an alias for `Reg<TRIM_OVERRIDE_EN_SPEC>`"]
pub type TRIM_OVERRIDE_EN = crate::Reg<trim_override_en::TRIM_OVERRIDE_EN_SPEC>;
#[doc = "Trim Override Enable"]
pub mod trim_override_en;
#[doc = "TRIM_OVERRIDE_EN_SET (rw) register accessor: an alias for `Reg<TRIM_OVERRIDE_EN_SET_SPEC>`"]
pub type TRIM_OVERRIDE_EN_SET = crate::Reg<trim_override_en_set::TRIM_OVERRIDE_EN_SET_SPEC>;
#[doc = "Trim Set"]
pub mod trim_override_en_set;
#[doc = "TRIM_OVERRIDE_EN_CLR (rw) register accessor: an alias for `Reg<TRIM_OVERRIDE_EN_CLR_SPEC>`"]
pub type TRIM_OVERRIDE_EN_CLR = crate::Reg<trim_override_en_clr::TRIM_OVERRIDE_EN_CLR_SPEC>;
#[doc = "Trim Clear"]
pub mod trim_override_en_clr;
#[doc = "TRIM_OVERRIDE_EN_TOG (rw) register accessor: an alias for `Reg<TRIM_OVERRIDE_EN_TOG_SPEC>`"]
pub type TRIM_OVERRIDE_EN_TOG = crate::Reg<trim_override_en_tog::TRIM_OVERRIDE_EN_TOG_SPEC>;
#[doc = "Trim Toggle"]
pub mod trim_override_en_tog;
