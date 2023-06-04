#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - USART Configuration"]
    pub cfg: CFG,
    #[doc = "0x04 - USART Control"]
    pub ctl: CTL,
    #[doc = "0x08 - USART Status"]
    pub stat: STAT,
    #[doc = "0x0c - Interrupt Enable Read and Set for USART (not FIFO) Status"]
    pub intenset: INTENSET,
    #[doc = "0x10 - Interrupt Enable Clear"]
    pub intenclr: INTENCLR,
    _reserved5: [u8; 0x0c],
    #[doc = "0x20 - Baud Rate Generator"]
    pub brg: BRG,
    #[doc = "0x24 - Interrupt Status"]
    pub intstat: INTSTAT,
    #[doc = "0x28 - Oversample Selection Register for Asynchronous Communication"]
    pub osr: OSR,
    #[doc = "0x2c - Address Register for Automatic Address Matching"]
    pub addr: ADDR,
    _reserved9: [u8; 0x0dd0],
    #[doc = "0xe00 - FIFO Configuration"]
    pub fifocfg: FIFOCFG,
    #[doc = "0xe04 - FIFO Status"]
    pub fifostat: FIFOSTAT,
    #[doc = "0xe08 - FIFO Trigger Settings for Interrupt and DMA Request"]
    pub fifotrig: FIFOTRIG,
    _reserved12: [u8; 0x04],
    #[doc = "0xe10 - FIFO Interrupt Enable"]
    pub fifointenset: FIFOINTENSET,
    #[doc = "0xe14 - FIFO Interrupt Enable Clear"]
    pub fifointenclr: FIFOINTENCLR,
    #[doc = "0xe18 - FIFO Interrupt Status"]
    pub fifointstat: FIFOINTSTAT,
    _reserved15: [u8; 0x04],
    #[doc = "0xe20 - FIFO Write Data"]
    pub fifowr: FIFOWR,
    _reserved16: [u8; 0x0c],
    #[doc = "0xe30 - FIFO Read Data"]
    pub fiford: FIFORD,
    _reserved17: [u8; 0x0c],
    #[doc = "0xe40 - FIFO Data Read with No FIFO Pop"]
    pub fifordnopop: FIFORDNOPOP,
    _reserved18: [u8; 0x04],
    #[doc = "0xe48 - FIFO Size"]
    pub fifosize: FIFOSIZE,
    _reserved19: [u8; 0x01b0],
    #[doc = "0xffc - Peripheral Identification"]
    pub id: ID,
}
#[doc = "CFG (rw) register accessor: an alias for `Reg<CFG_SPEC>`"]
pub type CFG = crate::Reg<cfg::CFG_SPEC>;
#[doc = "USART Configuration"]
pub mod cfg;
#[doc = "CTL (rw) register accessor: an alias for `Reg<CTL_SPEC>`"]
pub type CTL = crate::Reg<ctl::CTL_SPEC>;
#[doc = "USART Control"]
pub mod ctl;
#[doc = "STAT (rw) register accessor: an alias for `Reg<STAT_SPEC>`"]
pub type STAT = crate::Reg<stat::STAT_SPEC>;
#[doc = "USART Status"]
pub mod stat;
#[doc = "INTENSET (rw) register accessor: an alias for `Reg<INTENSET_SPEC>`"]
pub type INTENSET = crate::Reg<intenset::INTENSET_SPEC>;
#[doc = "Interrupt Enable Read and Set for USART (not FIFO) Status"]
pub mod intenset;
#[doc = "INTENCLR (w) register accessor: an alias for `Reg<INTENCLR_SPEC>`"]
pub type INTENCLR = crate::Reg<intenclr::INTENCLR_SPEC>;
#[doc = "Interrupt Enable Clear"]
pub mod intenclr;
#[doc = "BRG (rw) register accessor: an alias for `Reg<BRG_SPEC>`"]
pub type BRG = crate::Reg<brg::BRG_SPEC>;
#[doc = "Baud Rate Generator"]
pub mod brg;
#[doc = "INTSTAT (r) register accessor: an alias for `Reg<INTSTAT_SPEC>`"]
pub type INTSTAT = crate::Reg<intstat::INTSTAT_SPEC>;
#[doc = "Interrupt Status"]
pub mod intstat;
#[doc = "OSR (rw) register accessor: an alias for `Reg<OSR_SPEC>`"]
pub type OSR = crate::Reg<osr::OSR_SPEC>;
#[doc = "Oversample Selection Register for Asynchronous Communication"]
pub mod osr;
#[doc = "ADDR (rw) register accessor: an alias for `Reg<ADDR_SPEC>`"]
pub type ADDR = crate::Reg<addr::ADDR_SPEC>;
#[doc = "Address Register for Automatic Address Matching"]
pub mod addr;
#[doc = "FIFOCFG (rw) register accessor: an alias for `Reg<FIFOCFG_SPEC>`"]
pub type FIFOCFG = crate::Reg<fifocfg::FIFOCFG_SPEC>;
#[doc = "FIFO Configuration"]
pub mod fifocfg;
#[doc = "FIFOSTAT (rw) register accessor: an alias for `Reg<FIFOSTAT_SPEC>`"]
pub type FIFOSTAT = crate::Reg<fifostat::FIFOSTAT_SPEC>;
#[doc = "FIFO Status"]
pub mod fifostat;
#[doc = "FIFOTRIG (rw) register accessor: an alias for `Reg<FIFOTRIG_SPEC>`"]
pub type FIFOTRIG = crate::Reg<fifotrig::FIFOTRIG_SPEC>;
#[doc = "FIFO Trigger Settings for Interrupt and DMA Request"]
pub mod fifotrig;
#[doc = "FIFOINTENSET (rw) register accessor: an alias for `Reg<FIFOINTENSET_SPEC>`"]
pub type FIFOINTENSET = crate::Reg<fifointenset::FIFOINTENSET_SPEC>;
#[doc = "FIFO Interrupt Enable"]
pub mod fifointenset;
#[doc = "FIFOINTENCLR (rw) register accessor: an alias for `Reg<FIFOINTENCLR_SPEC>`"]
pub type FIFOINTENCLR = crate::Reg<fifointenclr::FIFOINTENCLR_SPEC>;
#[doc = "FIFO Interrupt Enable Clear"]
pub mod fifointenclr;
#[doc = "FIFOINTSTAT (r) register accessor: an alias for `Reg<FIFOINTSTAT_SPEC>`"]
pub type FIFOINTSTAT = crate::Reg<fifointstat::FIFOINTSTAT_SPEC>;
#[doc = "FIFO Interrupt Status"]
pub mod fifointstat;
#[doc = "FIFOWR (w) register accessor: an alias for `Reg<FIFOWR_SPEC>`"]
pub type FIFOWR = crate::Reg<fifowr::FIFOWR_SPEC>;
#[doc = "FIFO Write Data"]
pub mod fifowr;
#[doc = "FIFORD (r) register accessor: an alias for `Reg<FIFORD_SPEC>`"]
pub type FIFORD = crate::Reg<fiford::FIFORD_SPEC>;
#[doc = "FIFO Read Data"]
pub mod fiford;
#[doc = "FIFORDNOPOP (r) register accessor: an alias for `Reg<FIFORDNOPOP_SPEC>`"]
pub type FIFORDNOPOP = crate::Reg<fifordnopop::FIFORDNOPOP_SPEC>;
#[doc = "FIFO Data Read with No FIFO Pop"]
pub mod fifordnopop;
#[doc = "FIFOSIZE (r) register accessor: an alias for `Reg<FIFOSIZE_SPEC>`"]
pub type FIFOSIZE = crate::Reg<fifosize::FIFOSIZE_SPEC>;
#[doc = "FIFO Size"]
pub mod fifosize;
#[doc = "ID (r) register accessor: an alias for `Reg<ID_SPEC>`"]
pub type ID = crate::Reg<id::ID_SPEC>;
#[doc = "Peripheral Identification"]
pub mod id;
