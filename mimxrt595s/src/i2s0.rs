#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 0x0c00],
    #[doc = "0xc00 - Configuration Register 1 for the Primary Channel Pair"]
    pub cfg1: CFG1,
    #[doc = "0xc04 - Configuration Register 2 for the Primary Channel Pair"]
    pub cfg2: CFG2,
    #[doc = "0xc08 - Status Register for the Primary Channel Pair"]
    pub stat: STAT,
    _reserved3: [u8; 0x10],
    #[doc = "0xc1c - Clock Divider"]
    pub div: DIV,
    #[doc = "0xc20 - Configuration Register 1 for Channel Pair 1"]
    pub pcfg11: PCFG11,
    #[doc = "0xc24 - Configuration Register 2 for Channel Pair 1"]
    pub pcfg21: PCFG21,
    #[doc = "0xc28 - Status Register for Channel Pair 1"]
    pub pstat1: PSTAT1,
    _reserved7: [u8; 0x14],
    #[doc = "0xc40 - Configuration Register 1 for Channel Pair 2"]
    pub pcfg12: PCFG12,
    #[doc = "0xc44 - Configuration Register 2 for Channel Pair 2"]
    pub pcfg22: PCFG22,
    #[doc = "0xc48 - Status Register for Channel Pair 2"]
    pub pstat2: PSTAT2,
    _reserved10: [u8; 0x14],
    #[doc = "0xc60 - Configuration Register 1 for Channel Pair 3"]
    pub pcfg13: PCFG13,
    #[doc = "0xc64 - Configuration Register 2 for Channel Pair 3"]
    pub pcfg23: PCFG23,
    #[doc = "0xc68 - Status Register for Channel Pair 3"]
    pub pstat3: PSTAT3,
    _reserved13: [u8; 0x0194],
    #[doc = "0xe00 - FIFO Configuration and Enable"]
    pub fifocfg: FIFOCFG,
    #[doc = "0xe04 - FIFO Status"]
    pub fifostat: FIFOSTAT,
    #[doc = "0xe08 - FIFO Trigger Settings"]
    pub fifotrig: FIFOTRIG,
    _reserved16: [u8; 0x04],
    #[doc = "0xe10 - FIFO Interrupt Enable Set and Read"]
    pub fifointenset: FIFOINTENSET,
    #[doc = "0xe14 - FIFO Interrupt Enable Clear and Read"]
    pub fifointenclr: FIFOINTENCLR,
    #[doc = "0xe18 - FIFO Interrupt Status"]
    pub fifointstat: FIFOINTSTAT,
    _reserved19: [u8; 0x04],
    #[doc = "0xe20 - FIFO Write Data"]
    pub fifowr: FIFOWR,
    #[doc = "0xe24 - FIFO Write Data for Upper Data Bits"]
    pub fifowr48h: FIFOWR48H,
    _reserved21: [u8; 0x08],
    #[doc = "0xe30 - FIFO Read Data"]
    pub fiford: FIFORD,
    #[doc = "0xe34 - FIFO Read Data for Upper Data Bits"]
    pub fiford48h: FIFORD48H,
    _reserved23: [u8; 0x08],
    #[doc = "0xe40 - FIFO Data Read with No FIFO Pop"]
    pub fifordnopop: FIFORDNOPOP,
    #[doc = "0xe44 - FIFO Data Read for Upper Data Bits with No FIFO Pop"]
    pub fiford48hnopop: FIFORD48HNOPOP,
    #[doc = "0xe48 - FIFO Size Register"]
    pub fifosize: FIFOSIZE,
    _reserved26: [u8; 0x01b0],
    #[doc = "0xffc - I2S Module Identification"]
    pub id: ID,
}
#[doc = "CFG1 (rw) register accessor: an alias for `Reg<CFG1_SPEC>`"]
pub type CFG1 = crate::Reg<cfg1::CFG1_SPEC>;
#[doc = "Configuration Register 1 for the Primary Channel Pair"]
pub mod cfg1;
#[doc = "CFG2 (rw) register accessor: an alias for `Reg<CFG2_SPEC>`"]
pub type CFG2 = crate::Reg<cfg2::CFG2_SPEC>;
#[doc = "Configuration Register 2 for the Primary Channel Pair"]
pub mod cfg2;
#[doc = "STAT (rw) register accessor: an alias for `Reg<STAT_SPEC>`"]
pub type STAT = crate::Reg<stat::STAT_SPEC>;
#[doc = "Status Register for the Primary Channel Pair"]
pub mod stat;
#[doc = "DIV (rw) register accessor: an alias for `Reg<DIV_SPEC>`"]
pub type DIV = crate::Reg<div::DIV_SPEC>;
#[doc = "Clock Divider"]
pub mod div;
#[doc = "PCFG11 (rw) register accessor: an alias for `Reg<PCFG11_SPEC>`"]
pub type PCFG11 = crate::Reg<pcfg11::PCFG11_SPEC>;
#[doc = "Configuration Register 1 for Channel Pair 1"]
pub mod pcfg11;
#[doc = "PCFG21 (rw) register accessor: an alias for `Reg<PCFG21_SPEC>`"]
pub type PCFG21 = crate::Reg<pcfg21::PCFG21_SPEC>;
#[doc = "Configuration Register 2 for Channel Pair 1"]
pub mod pcfg21;
#[doc = "PSTAT1 (r) register accessor: an alias for `Reg<PSTAT1_SPEC>`"]
pub type PSTAT1 = crate::Reg<pstat1::PSTAT1_SPEC>;
#[doc = "Status Register for Channel Pair 1"]
pub mod pstat1;
#[doc = "PCFG12 (rw) register accessor: an alias for `Reg<PCFG12_SPEC>`"]
pub type PCFG12 = crate::Reg<pcfg12::PCFG12_SPEC>;
#[doc = "Configuration Register 1 for Channel Pair 2"]
pub mod pcfg12;
#[doc = "PCFG22 (rw) register accessor: an alias for `Reg<PCFG22_SPEC>`"]
pub type PCFG22 = crate::Reg<pcfg22::PCFG22_SPEC>;
#[doc = "Configuration Register 2 for Channel Pair 2"]
pub mod pcfg22;
#[doc = "PSTAT2 (r) register accessor: an alias for `Reg<PSTAT2_SPEC>`"]
pub type PSTAT2 = crate::Reg<pstat2::PSTAT2_SPEC>;
#[doc = "Status Register for Channel Pair 2"]
pub mod pstat2;
#[doc = "PCFG13 (rw) register accessor: an alias for `Reg<PCFG13_SPEC>`"]
pub type PCFG13 = crate::Reg<pcfg13::PCFG13_SPEC>;
#[doc = "Configuration Register 1 for Channel Pair 3"]
pub mod pcfg13;
#[doc = "PCFG23 (rw) register accessor: an alias for `Reg<PCFG23_SPEC>`"]
pub type PCFG23 = crate::Reg<pcfg23::PCFG23_SPEC>;
#[doc = "Configuration Register 2 for Channel Pair 3"]
pub mod pcfg23;
#[doc = "PSTAT3 (r) register accessor: an alias for `Reg<PSTAT3_SPEC>`"]
pub type PSTAT3 = crate::Reg<pstat3::PSTAT3_SPEC>;
#[doc = "Status Register for Channel Pair 3"]
pub mod pstat3;
#[doc = "FIFOCFG (rw) register accessor: an alias for `Reg<FIFOCFG_SPEC>`"]
pub type FIFOCFG = crate::Reg<fifocfg::FIFOCFG_SPEC>;
#[doc = "FIFO Configuration and Enable"]
pub mod fifocfg;
#[doc = "FIFOSTAT (rw) register accessor: an alias for `Reg<FIFOSTAT_SPEC>`"]
pub type FIFOSTAT = crate::Reg<fifostat::FIFOSTAT_SPEC>;
#[doc = "FIFO Status"]
pub mod fifostat;
#[doc = "FIFOTRIG (rw) register accessor: an alias for `Reg<FIFOTRIG_SPEC>`"]
pub type FIFOTRIG = crate::Reg<fifotrig::FIFOTRIG_SPEC>;
#[doc = "FIFO Trigger Settings"]
pub mod fifotrig;
#[doc = "FIFOINTENSET (rw) register accessor: an alias for `Reg<FIFOINTENSET_SPEC>`"]
pub type FIFOINTENSET = crate::Reg<fifointenset::FIFOINTENSET_SPEC>;
#[doc = "FIFO Interrupt Enable Set and Read"]
pub mod fifointenset;
#[doc = "FIFOINTENCLR (rw) register accessor: an alias for `Reg<FIFOINTENCLR_SPEC>`"]
pub type FIFOINTENCLR = crate::Reg<fifointenclr::FIFOINTENCLR_SPEC>;
#[doc = "FIFO Interrupt Enable Clear and Read"]
pub mod fifointenclr;
#[doc = "FIFOINTSTAT (r) register accessor: an alias for `Reg<FIFOINTSTAT_SPEC>`"]
pub type FIFOINTSTAT = crate::Reg<fifointstat::FIFOINTSTAT_SPEC>;
#[doc = "FIFO Interrupt Status"]
pub mod fifointstat;
#[doc = "FIFOWR (w) register accessor: an alias for `Reg<FIFOWR_SPEC>`"]
pub type FIFOWR = crate::Reg<fifowr::FIFOWR_SPEC>;
#[doc = "FIFO Write Data"]
pub mod fifowr;
#[doc = "FIFOWR48H (w) register accessor: an alias for `Reg<FIFOWR48H_SPEC>`"]
pub type FIFOWR48H = crate::Reg<fifowr48h::FIFOWR48H_SPEC>;
#[doc = "FIFO Write Data for Upper Data Bits"]
pub mod fifowr48h;
#[doc = "FIFORD (r) register accessor: an alias for `Reg<FIFORD_SPEC>`"]
pub type FIFORD = crate::Reg<fiford::FIFORD_SPEC>;
#[doc = "FIFO Read Data"]
pub mod fiford;
#[doc = "FIFORD48H (r) register accessor: an alias for `Reg<FIFORD48H_SPEC>`"]
pub type FIFORD48H = crate::Reg<fiford48h::FIFORD48H_SPEC>;
#[doc = "FIFO Read Data for Upper Data Bits"]
pub mod fiford48h;
#[doc = "FIFORDNOPOP (r) register accessor: an alias for `Reg<FIFORDNOPOP_SPEC>`"]
pub type FIFORDNOPOP = crate::Reg<fifordnopop::FIFORDNOPOP_SPEC>;
#[doc = "FIFO Data Read with No FIFO Pop"]
pub mod fifordnopop;
#[doc = "FIFORD48HNOPOP (r) register accessor: an alias for `Reg<FIFORD48HNOPOP_SPEC>`"]
pub type FIFORD48HNOPOP = crate::Reg<fiford48hnopop::FIFORD48HNOPOP_SPEC>;
#[doc = "FIFO Data Read for Upper Data Bits with No FIFO Pop"]
pub mod fiford48hnopop;
#[doc = "FIFOSIZE (r) register accessor: an alias for `Reg<FIFOSIZE_SPEC>`"]
pub type FIFOSIZE = crate::Reg<fifosize::FIFOSIZE_SPEC>;
#[doc = "FIFO Size Register"]
pub mod fifosize;
#[doc = "ID (r) register accessor: an alias for `Reg<ID_SPEC>`"]
pub type ID = crate::Reg<id::ID_SPEC>;
#[doc = "I2S Module Identification"]
pub mod id;
