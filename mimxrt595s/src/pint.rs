#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Pin Interrupt Mode"]
    pub isel: ISEL,
    #[doc = "0x04 - Pin Interrupt Level or Rising Edge Interrupt Enable"]
    pub ienr: IENR,
    #[doc = "0x08 - Pin Interrupt Level or Rising Edge Interrupt Set"]
    pub sienr: SIENR,
    #[doc = "0x0c - Pin Interrupt Level (Rising Edge Interrupt) Clear"]
    pub cienr: CIENR,
    #[doc = "0x10 - Pin Interrupt Active Level or Falling Edge Interrupt Enable"]
    pub ienf: IENF,
    #[doc = "0x14 - Pin Interrupt Active Level or Falling Edge Interrupt Set"]
    pub sienf: SIENF,
    #[doc = "0x18 - Pin Interrupt Active Level or Falling Edge Interrupt Clear"]
    pub cienf: CIENF,
    #[doc = "0x1c - Pin Interrupt Rising Edge"]
    pub rise: RISE,
    #[doc = "0x20 - Pin Interrupt Falling Edge"]
    pub fall: FALL,
    #[doc = "0x24 - Pin Interrupt Status"]
    pub ist: IST,
    #[doc = "0x28 - Pattern Match Interrupt Control"]
    pub pmctrl: PMCTRL,
    #[doc = "0x2c - Pattern Match Interrupt Bit-Slice Source"]
    pub pmsrc: PMSRC,
    #[doc = "0x30 - Pattern Match Interrupt Bit Slice Configuration"]
    pub pmcfg: PMCFG,
}
#[doc = "ISEL (rw) register accessor: an alias for `Reg<ISEL_SPEC>`"]
pub type ISEL = crate::Reg<isel::ISEL_SPEC>;
#[doc = "Pin Interrupt Mode"]
pub mod isel;
#[doc = "IENR (rw) register accessor: an alias for `Reg<IENR_SPEC>`"]
pub type IENR = crate::Reg<ienr::IENR_SPEC>;
#[doc = "Pin Interrupt Level or Rising Edge Interrupt Enable"]
pub mod ienr;
#[doc = "SIENR (w) register accessor: an alias for `Reg<SIENR_SPEC>`"]
pub type SIENR = crate::Reg<sienr::SIENR_SPEC>;
#[doc = "Pin Interrupt Level or Rising Edge Interrupt Set"]
pub mod sienr;
#[doc = "CIENR (rw) register accessor: an alias for `Reg<CIENR_SPEC>`"]
pub type CIENR = crate::Reg<cienr::CIENR_SPEC>;
#[doc = "Pin Interrupt Level (Rising Edge Interrupt) Clear"]
pub mod cienr;
#[doc = "IENF (rw) register accessor: an alias for `Reg<IENF_SPEC>`"]
pub type IENF = crate::Reg<ienf::IENF_SPEC>;
#[doc = "Pin Interrupt Active Level or Falling Edge Interrupt Enable"]
pub mod ienf;
#[doc = "SIENF (w) register accessor: an alias for `Reg<SIENF_SPEC>`"]
pub type SIENF = crate::Reg<sienf::SIENF_SPEC>;
#[doc = "Pin Interrupt Active Level or Falling Edge Interrupt Set"]
pub mod sienf;
#[doc = "CIENF (w) register accessor: an alias for `Reg<CIENF_SPEC>`"]
pub type CIENF = crate::Reg<cienf::CIENF_SPEC>;
#[doc = "Pin Interrupt Active Level or Falling Edge Interrupt Clear"]
pub mod cienf;
#[doc = "RISE (rw) register accessor: an alias for `Reg<RISE_SPEC>`"]
pub type RISE = crate::Reg<rise::RISE_SPEC>;
#[doc = "Pin Interrupt Rising Edge"]
pub mod rise;
#[doc = "FALL (rw) register accessor: an alias for `Reg<FALL_SPEC>`"]
pub type FALL = crate::Reg<fall::FALL_SPEC>;
#[doc = "Pin Interrupt Falling Edge"]
pub mod fall;
#[doc = "IST (rw) register accessor: an alias for `Reg<IST_SPEC>`"]
pub type IST = crate::Reg<ist::IST_SPEC>;
#[doc = "Pin Interrupt Status"]
pub mod ist;
#[doc = "PMCTRL (rw) register accessor: an alias for `Reg<PMCTRL_SPEC>`"]
pub type PMCTRL = crate::Reg<pmctrl::PMCTRL_SPEC>;
#[doc = "Pattern Match Interrupt Control"]
pub mod pmctrl;
#[doc = "PMSRC (rw) register accessor: an alias for `Reg<PMSRC_SPEC>`"]
pub type PMSRC = crate::Reg<pmsrc::PMSRC_SPEC>;
#[doc = "Pattern Match Interrupt Bit-Slice Source"]
pub mod pmsrc;
#[doc = "PMCFG (rw) register accessor: an alias for `Reg<PMCFG_SPEC>`"]
pub type PMCFG = crate::Reg<pmcfg::PMCFG_SPEC>;
#[doc = "Pattern Match Interrupt Bit Slice Configuration"]
pub mod pmcfg;
