#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - System Reset Status Register"]
    pub sysrststat: SYSRSTSTAT,
    _reserved1: [u8; 0x0c],
    #[doc = "0x10 - Peripheral Reset Control Register 0"]
    pub prstctl0: PRSTCTL0,
    #[doc = "0x14 - Peripheral Reset Control Register 1"]
    pub prstctl1: PRSTCTL1,
    #[doc = "0x18 - Peripheral Reset Control Register 2"]
    pub prstctl2: PRSTCTL2,
    _reserved4: [u8; 0x24],
    #[doc = "0x40 - Peripheral Reset Control Register 0 SET"]
    pub prstctl0_set: PRSTCTL0_SET,
    #[doc = "0x44 - Peripheral Reset Control Register 1 SET"]
    pub prstctl1_set: PRSTCTL1_SET,
    #[doc = "0x48 - Peripheral Reset Control Register 2 SET"]
    pub prstctl2_set: PRSTCTL2_SET,
    _reserved7: [u8; 0x24],
    #[doc = "0x70 - Peripheral Reset Control Register 0 CLR"]
    pub prstctl0_clr: PRSTCTL0_CLR,
    #[doc = "0x74 - Peripheral Reset Control Register 1 CLR"]
    pub prstctl1_clr: PRSTCTL1_CLR,
    #[doc = "0x78 - Peripheral Reset Control Register 2 CLR"]
    pub prstctl2_clr: PRSTCTL2_CLR,
}
#[doc = "SYSRSTSTAT (rw) register accessor: an alias for `Reg<SYSRSTSTAT_SPEC>`"]
pub type SYSRSTSTAT = crate::Reg<sysrststat::SYSRSTSTAT_SPEC>;
#[doc = "System Reset Status Register"]
pub mod sysrststat;
#[doc = "PRSTCTL0 (rw) register accessor: an alias for `Reg<PRSTCTL0_SPEC>`"]
pub type PRSTCTL0 = crate::Reg<prstctl0::PRSTCTL0_SPEC>;
#[doc = "Peripheral Reset Control Register 0"]
pub mod prstctl0;
#[doc = "PRSTCTL1 (rw) register accessor: an alias for `Reg<PRSTCTL1_SPEC>`"]
pub type PRSTCTL1 = crate::Reg<prstctl1::PRSTCTL1_SPEC>;
#[doc = "Peripheral Reset Control Register 1"]
pub mod prstctl1;
#[doc = "PRSTCTL2 (rw) register accessor: an alias for `Reg<PRSTCTL2_SPEC>`"]
pub type PRSTCTL2 = crate::Reg<prstctl2::PRSTCTL2_SPEC>;
#[doc = "Peripheral Reset Control Register 2"]
pub mod prstctl2;
#[doc = "PRSTCTL0_SET (w) register accessor: an alias for `Reg<PRSTCTL0_SET_SPEC>`"]
pub type PRSTCTL0_SET = crate::Reg<prstctl0_set::PRSTCTL0_SET_SPEC>;
#[doc = "Peripheral Reset Control Register 0 SET"]
pub mod prstctl0_set;
#[doc = "PRSTCTL1_SET (w) register accessor: an alias for `Reg<PRSTCTL1_SET_SPEC>`"]
pub type PRSTCTL1_SET = crate::Reg<prstctl1_set::PRSTCTL1_SET_SPEC>;
#[doc = "Peripheral Reset Control Register 1 SET"]
pub mod prstctl1_set;
#[doc = "PRSTCTL2_SET (w) register accessor: an alias for `Reg<PRSTCTL2_SET_SPEC>`"]
pub type PRSTCTL2_SET = crate::Reg<prstctl2_set::PRSTCTL2_SET_SPEC>;
#[doc = "Peripheral Reset Control Register 2 SET"]
pub mod prstctl2_set;
#[doc = "PRSTCTL0_CLR (w) register accessor: an alias for `Reg<PRSTCTL0_CLR_SPEC>`"]
pub type PRSTCTL0_CLR = crate::Reg<prstctl0_clr::PRSTCTL0_CLR_SPEC>;
#[doc = "Peripheral Reset Control Register 0 CLR"]
pub mod prstctl0_clr;
#[doc = "PRSTCTL1_CLR (w) register accessor: an alias for `Reg<PRSTCTL1_CLR_SPEC>`"]
pub type PRSTCTL1_CLR = crate::Reg<prstctl1_clr::PRSTCTL1_CLR_SPEC>;
#[doc = "Peripheral Reset Control Register 1 CLR"]
pub mod prstctl1_clr;
#[doc = "PRSTCTL2_CLR (w) register accessor: an alias for `Reg<PRSTCTL2_CLR_SPEC>`"]
pub type PRSTCTL2_CLR = crate::Reg<prstctl2_clr::PRSTCTL2_CLR_SPEC>;
#[doc = "Peripheral Reset Control Register 2 CLR"]
pub mod prstctl2_clr;
