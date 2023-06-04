#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 0x0800],
    #[doc = "0x800 - Configuration Register"]
    pub cfg: CFG,
    #[doc = "0x804 - Status Register"]
    pub stat: STAT,
    #[doc = "0x808 - Interrupt Enable Set Register"]
    pub intenset: INTENSET,
    #[doc = "0x80c - Interrupt Enable Clear Register"]
    pub intenclr: INTENCLR,
    #[doc = "0x810 - Time-out Register"]
    pub timeout: TIMEOUT,
    #[doc = "0x814 - Clock Divider Register"]
    pub clkdiv: CLKDIV,
    #[doc = "0x818 - Interrupt Status Register"]
    pub intstat: INTSTAT,
    _reserved7: [u8; 0x04],
    #[doc = "0x820 - Master Control Register"]
    pub mstctl: MSTCTL,
    #[doc = "0x824 - Master Timing Register"]
    pub msttime: MSTTIME,
    #[doc = "0x828 - Master Data Register"]
    pub mstdat: MSTDAT,
    _reserved10: [u8; 0x14],
    #[doc = "0x840 - Slave Control Register"]
    pub slvctl: SLVCTL,
    #[doc = "0x844 - Slave Data Register"]
    pub slvdat: SLVDAT,
    #[doc = "0x848 - Slave Address Register"]
    pub slvadr0: SLVADR0,
    #[doc = "0x84c - Slave Address Register"]
    pub slvadr1: SLVADR1,
    #[doc = "0x850 - Slave Address Register"]
    pub slvadr2: SLVADR2,
    #[doc = "0x854 - Slave Address Register"]
    pub slvadr3: SLVADR3,
    #[doc = "0x858 - Slave Qualification for Address 0 Register"]
    pub slvqual0: SLVQUAL0,
    _reserved17: [u8; 0x24],
    #[doc = "0x880 - Monitor Receiver Data Register"]
    pub monrxdat: MONRXDAT,
    _reserved18: [u8; 0x0778],
    #[doc = "0xffc - Peripheral Identification Register"]
    pub id: ID,
}
#[doc = "CFG (rw) register accessor: an alias for `Reg<CFG_SPEC>`"]
pub type CFG = crate::Reg<cfg::CFG_SPEC>;
#[doc = "Configuration Register"]
pub mod cfg;
#[doc = "STAT (rw) register accessor: an alias for `Reg<STAT_SPEC>`"]
pub type STAT = crate::Reg<stat::STAT_SPEC>;
#[doc = "Status Register"]
pub mod stat;
#[doc = "INTENSET (rw) register accessor: an alias for `Reg<INTENSET_SPEC>`"]
pub type INTENSET = crate::Reg<intenset::INTENSET_SPEC>;
#[doc = "Interrupt Enable Set Register"]
pub mod intenset;
#[doc = "INTENCLR (w) register accessor: an alias for `Reg<INTENCLR_SPEC>`"]
pub type INTENCLR = crate::Reg<intenclr::INTENCLR_SPEC>;
#[doc = "Interrupt Enable Clear Register"]
pub mod intenclr;
#[doc = "TIMEOUT (rw) register accessor: an alias for `Reg<TIMEOUT_SPEC>`"]
pub type TIMEOUT = crate::Reg<timeout::TIMEOUT_SPEC>;
#[doc = "Time-out Register"]
pub mod timeout;
#[doc = "CLKDIV (rw) register accessor: an alias for `Reg<CLKDIV_SPEC>`"]
pub type CLKDIV = crate::Reg<clkdiv::CLKDIV_SPEC>;
#[doc = "Clock Divider Register"]
pub mod clkdiv;
#[doc = "INTSTAT (r) register accessor: an alias for `Reg<INTSTAT_SPEC>`"]
pub type INTSTAT = crate::Reg<intstat::INTSTAT_SPEC>;
#[doc = "Interrupt Status Register"]
pub mod intstat;
#[doc = "MSTCTL (rw) register accessor: an alias for `Reg<MSTCTL_SPEC>`"]
pub type MSTCTL = crate::Reg<mstctl::MSTCTL_SPEC>;
#[doc = "Master Control Register"]
pub mod mstctl;
#[doc = "MSTTIME (rw) register accessor: an alias for `Reg<MSTTIME_SPEC>`"]
pub type MSTTIME = crate::Reg<msttime::MSTTIME_SPEC>;
#[doc = "Master Timing Register"]
pub mod msttime;
#[doc = "MSTDAT (rw) register accessor: an alias for `Reg<MSTDAT_SPEC>`"]
pub type MSTDAT = crate::Reg<mstdat::MSTDAT_SPEC>;
#[doc = "Master Data Register"]
pub mod mstdat;
#[doc = "SLVCTL (rw) register accessor: an alias for `Reg<SLVCTL_SPEC>`"]
pub type SLVCTL = crate::Reg<slvctl::SLVCTL_SPEC>;
#[doc = "Slave Control Register"]
pub mod slvctl;
#[doc = "SLVDAT (rw) register accessor: an alias for `Reg<SLVDAT_SPEC>`"]
pub type SLVDAT = crate::Reg<slvdat::SLVDAT_SPEC>;
#[doc = "Slave Data Register"]
pub mod slvdat;
#[doc = "SLVADR0 (rw) register accessor: an alias for `Reg<SLVADR0_SPEC>`"]
pub type SLVADR0 = crate::Reg<slvadr0::SLVADR0_SPEC>;
#[doc = "Slave Address Register"]
pub mod slvadr0;
#[doc = "SLVADR1 (rw) register accessor: an alias for `Reg<SLVADR1_SPEC>`"]
pub type SLVADR1 = crate::Reg<slvadr1::SLVADR1_SPEC>;
#[doc = "Slave Address Register"]
pub mod slvadr1;
#[doc = "SLVADR2 (rw) register accessor: an alias for `Reg<SLVADR2_SPEC>`"]
pub type SLVADR2 = crate::Reg<slvadr2::SLVADR2_SPEC>;
#[doc = "Slave Address Register"]
pub mod slvadr2;
#[doc = "SLVADR3 (rw) register accessor: an alias for `Reg<SLVADR3_SPEC>`"]
pub type SLVADR3 = crate::Reg<slvadr3::SLVADR3_SPEC>;
#[doc = "Slave Address Register"]
pub mod slvadr3;
#[doc = "SLVQUAL0 (rw) register accessor: an alias for `Reg<SLVQUAL0_SPEC>`"]
pub type SLVQUAL0 = crate::Reg<slvqual0::SLVQUAL0_SPEC>;
#[doc = "Slave Qualification for Address 0 Register"]
pub mod slvqual0;
#[doc = "MONRXDAT (r) register accessor: an alias for `Reg<MONRXDAT_SPEC>`"]
pub type MONRXDAT = crate::Reg<monrxdat::MONRXDAT_SPEC>;
#[doc = "Monitor Receiver Data Register"]
pub mod monrxdat;
#[doc = "ID (r) register accessor: an alias for `Reg<ID_SPEC>`"]
pub type ID = crate::Reg<id::ID_SPEC>;
#[doc = "Peripheral Identification Register"]
pub mod id;
