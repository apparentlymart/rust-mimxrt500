#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control"]
    pub ctrl: CTRL,
    #[doc = "0x04 - Status"]
    pub stat: STAT,
    #[doc = "0x08 - Capture Configuration"]
    pub cfg: CFG,
    #[doc = "0x0c - Capture Clear"]
    pub capclr: CAPCLR,
    #[doc = "0x10..0x20 - Capture"]
    pub cap: [CAP; 4],
}
#[doc = "CTRL (rw) register accessor: an alias for `Reg<CTRL_SPEC>`"]
pub type CTRL = crate::Reg<ctrl::CTRL_SPEC>;
#[doc = "Control"]
pub mod ctrl;
#[doc = "STAT (rw) register accessor: an alias for `Reg<STAT_SPEC>`"]
pub type STAT = crate::Reg<stat::STAT_SPEC>;
#[doc = "Status"]
pub mod stat;
#[doc = "CFG (rw) register accessor: an alias for `Reg<CFG_SPEC>`"]
pub type CFG = crate::Reg<cfg::CFG_SPEC>;
#[doc = "Capture Configuration"]
pub mod cfg;
#[doc = "CAPCLR (w) register accessor: an alias for `Reg<CAPCLR_SPEC>`"]
pub type CAPCLR = crate::Reg<capclr::CAPCLR_SPEC>;
#[doc = "Capture Clear"]
pub mod capclr;
#[doc = "CAP (r) register accessor: an alias for `Reg<CAP_SPEC>`"]
pub type CAP = crate::Reg<cap::CAP_SPEC>;
#[doc = "Capture"]
pub mod cap;
