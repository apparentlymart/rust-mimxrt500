#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 0x0400],
    #[doc = "0x400 - Configuration Register"]
    pub cfg: CFG,
    #[doc = "0x404 - Delay Register"]
    pub dly: DLY,
    #[doc = "0x408 - Status Register"]
    pub stat: STAT,
    #[doc = "0x40c - Interrupt Enable Register"]
    pub intenset: INTENSET,
    #[doc = "0x410 - Interrupt Enable Clear Register"]
    pub intenclr: INTENCLR,
    _reserved5: [u8; 0x10],
    #[doc = "0x424 - Clock Divider Register"]
    pub div: DIV,
    #[doc = "0x428 - Interrupt Status Register"]
    pub intstat: INTSTAT,
    _reserved7: [u8; 0x09d4],
    #[doc = "0xe00 - FIFO Configuration Register"]
    pub fifocfg: FIFOCFG,
    #[doc = "0xe04 - FIFO Status Register"]
    pub fifostat: FIFOSTAT,
    #[doc = "0xe08 - FIFO Trigger Register"]
    pub fifotrig: FIFOTRIG,
    _reserved10: [u8; 0x04],
    #[doc = "0xe10 - FIFO Interrupt Enable Register"]
    pub fifointenset: FIFOINTENSET,
    #[doc = "0xe14 - FIFO Interrupt Enable Clear Register"]
    pub fifointenclr: FIFOINTENCLR,
    #[doc = "0xe18 - FIFO Interrupt Status Register"]
    pub fifointstat: FIFOINTSTAT,
    _reserved13: [u8; 0x04],
    #[doc = "0xe20 - FIFO Write Data Register"]
    pub fifowr: FIFOWR,
    _reserved14: [u8; 0x0c],
    #[doc = "0xe30 - FIFO Read Data Register"]
    pub fiford: FIFORD,
    _reserved15: [u8; 0x0c],
    #[doc = "0xe40 - FIFO Data Read with no FIFO Pop Register"]
    pub fifordnopop: FIFORDNOPOP,
    _reserved16: [u8; 0x04],
    #[doc = "0xe48 - FIFO Size Register"]
    pub fifosize: FIFOSIZE,
    _reserved17: [u8; 0x01b0],
    #[doc = "0xffc - Peripheral Identification Register"]
    pub id: ID,
}
#[doc = "CFG (rw) register accessor: an alias for `Reg<CFG_SPEC>`"]
pub type CFG = crate::Reg<cfg::CFG_SPEC>;
#[doc = "Configuration Register"]
pub mod cfg;
#[doc = "DLY (rw) register accessor: an alias for `Reg<DLY_SPEC>`"]
pub type DLY = crate::Reg<dly::DLY_SPEC>;
#[doc = "Delay Register"]
pub mod dly;
#[doc = "STAT (rw) register accessor: an alias for `Reg<STAT_SPEC>`"]
pub type STAT = crate::Reg<stat::STAT_SPEC>;
#[doc = "Status Register"]
pub mod stat;
#[doc = "INTENSET (rw) register accessor: an alias for `Reg<INTENSET_SPEC>`"]
pub type INTENSET = crate::Reg<intenset::INTENSET_SPEC>;
#[doc = "Interrupt Enable Register"]
pub mod intenset;
#[doc = "INTENCLR (w) register accessor: an alias for `Reg<INTENCLR_SPEC>`"]
pub type INTENCLR = crate::Reg<intenclr::INTENCLR_SPEC>;
#[doc = "Interrupt Enable Clear Register"]
pub mod intenclr;
#[doc = "DIV (rw) register accessor: an alias for `Reg<DIV_SPEC>`"]
pub type DIV = crate::Reg<div::DIV_SPEC>;
#[doc = "Clock Divider Register"]
pub mod div;
#[doc = "INTSTAT (r) register accessor: an alias for `Reg<INTSTAT_SPEC>`"]
pub type INTSTAT = crate::Reg<intstat::INTSTAT_SPEC>;
#[doc = "Interrupt Status Register"]
pub mod intstat;
#[doc = "FIFOCFG (rw) register accessor: an alias for `Reg<FIFOCFG_SPEC>`"]
pub type FIFOCFG = crate::Reg<fifocfg::FIFOCFG_SPEC>;
#[doc = "FIFO Configuration Register"]
pub mod fifocfg;
#[doc = "FIFOSTAT (rw) register accessor: an alias for `Reg<FIFOSTAT_SPEC>`"]
pub type FIFOSTAT = crate::Reg<fifostat::FIFOSTAT_SPEC>;
#[doc = "FIFO Status Register"]
pub mod fifostat;
#[doc = "FIFOTRIG (rw) register accessor: an alias for `Reg<FIFOTRIG_SPEC>`"]
pub type FIFOTRIG = crate::Reg<fifotrig::FIFOTRIG_SPEC>;
#[doc = "FIFO Trigger Register"]
pub mod fifotrig;
#[doc = "FIFOINTENSET (rw) register accessor: an alias for `Reg<FIFOINTENSET_SPEC>`"]
pub type FIFOINTENSET = crate::Reg<fifointenset::FIFOINTENSET_SPEC>;
#[doc = "FIFO Interrupt Enable Register"]
pub mod fifointenset;
#[doc = "FIFOINTENCLR (rw) register accessor: an alias for `Reg<FIFOINTENCLR_SPEC>`"]
pub type FIFOINTENCLR = crate::Reg<fifointenclr::FIFOINTENCLR_SPEC>;
#[doc = "FIFO Interrupt Enable Clear Register"]
pub mod fifointenclr;
#[doc = "FIFOINTSTAT (r) register accessor: an alias for `Reg<FIFOINTSTAT_SPEC>`"]
pub type FIFOINTSTAT = crate::Reg<fifointstat::FIFOINTSTAT_SPEC>;
#[doc = "FIFO Interrupt Status Register"]
pub mod fifointstat;
#[doc = "FIFOWR (w) register accessor: an alias for `Reg<FIFOWR_SPEC>`"]
pub type FIFOWR = crate::Reg<fifowr::FIFOWR_SPEC>;
#[doc = "FIFO Write Data Register"]
pub mod fifowr;
#[doc = "FIFORD (r) register accessor: an alias for `Reg<FIFORD_SPEC>`"]
pub type FIFORD = crate::Reg<fiford::FIFORD_SPEC>;
#[doc = "FIFO Read Data Register"]
pub mod fiford;
#[doc = "FIFORDNOPOP (r) register accessor: an alias for `Reg<FIFORDNOPOP_SPEC>`"]
pub type FIFORDNOPOP = crate::Reg<fifordnopop::FIFORDNOPOP_SPEC>;
#[doc = "FIFO Data Read with no FIFO Pop Register"]
pub mod fifordnopop;
#[doc = "FIFOSIZE (r) register accessor: an alias for `Reg<FIFOSIZE_SPEC>`"]
pub type FIFOSIZE = crate::Reg<fifosize::FIFOSIZE_SPEC>;
#[doc = "FIFO Size Register"]
pub mod fifosize;
#[doc = "ID (r) register accessor: an alias for `Reg<ID_SPEC>`"]
pub type ID = crate::Reg<id::ID_SPEC>;
#[doc = "Peripheral Identification Register"]
pub mod id;
