#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - USB Device Command/Status"]
    pub devcmdstat: DEVCMDSTAT,
    #[doc = "0x04 - USB Info"]
    pub info: INFO,
    #[doc = "0x08 - USB EP Command/Status List Start Address"]
    pub epliststart: EPLISTSTART,
    #[doc = "0x0c - USB Data Buffer List Start Address"]
    pub databufstart: DATABUFSTART,
    #[doc = "0x10 - USB Link Power Management"]
    pub lpm: LPM,
    #[doc = "0x14 - USB Endpoint Skip"]
    pub epskip: EPSKIP,
    #[doc = "0x18 - USB Endpoint Buffer in use"]
    pub epinuse: EPINUSE,
    #[doc = "0x1c - USB Endpoint Buffer Configuration"]
    pub epbufcfg: EPBUFCFG,
    #[doc = "0x20 - USB Interrupt Status"]
    pub intstat: INTSTAT,
    #[doc = "0x24 - USB Interrupt Enable"]
    pub inten: INTEN,
    #[doc = "0x28 - USB Set Interrupt Status"]
    pub intsetstat: INTSETSTAT,
    _reserved11: [u8; 0x08],
    #[doc = "0x34 - USB Endpoint Toggle"]
    pub eptoggle: EPTOGGLE,
}
#[doc = "DEVCMDSTAT (rw) register accessor: an alias for `Reg<DEVCMDSTAT_SPEC>`"]
pub type DEVCMDSTAT = crate::Reg<devcmdstat::DEVCMDSTAT_SPEC>;
#[doc = "USB Device Command/Status"]
pub mod devcmdstat;
#[doc = "INFO (rw) register accessor: an alias for `Reg<INFO_SPEC>`"]
pub type INFO = crate::Reg<info::INFO_SPEC>;
#[doc = "USB Info"]
pub mod info;
#[doc = "EPLISTSTART (rw) register accessor: an alias for `Reg<EPLISTSTART_SPEC>`"]
pub type EPLISTSTART = crate::Reg<epliststart::EPLISTSTART_SPEC>;
#[doc = "USB EP Command/Status List Start Address"]
pub mod epliststart;
#[doc = "DATABUFSTART (rw) register accessor: an alias for `Reg<DATABUFSTART_SPEC>`"]
pub type DATABUFSTART = crate::Reg<databufstart::DATABUFSTART_SPEC>;
#[doc = "USB Data Buffer List Start Address"]
pub mod databufstart;
#[doc = "LPM (rw) register accessor: an alias for `Reg<LPM_SPEC>`"]
pub type LPM = crate::Reg<lpm::LPM_SPEC>;
#[doc = "USB Link Power Management"]
pub mod lpm;
#[doc = "EPSKIP (rw) register accessor: an alias for `Reg<EPSKIP_SPEC>`"]
pub type EPSKIP = crate::Reg<epskip::EPSKIP_SPEC>;
#[doc = "USB Endpoint Skip"]
pub mod epskip;
#[doc = "EPINUSE (rw) register accessor: an alias for `Reg<EPINUSE_SPEC>`"]
pub type EPINUSE = crate::Reg<epinuse::EPINUSE_SPEC>;
#[doc = "USB Endpoint Buffer in use"]
pub mod epinuse;
#[doc = "EPBUFCFG (rw) register accessor: an alias for `Reg<EPBUFCFG_SPEC>`"]
pub type EPBUFCFG = crate::Reg<epbufcfg::EPBUFCFG_SPEC>;
#[doc = "USB Endpoint Buffer Configuration"]
pub mod epbufcfg;
#[doc = "INTSTAT (rw) register accessor: an alias for `Reg<INTSTAT_SPEC>`"]
pub type INTSTAT = crate::Reg<intstat::INTSTAT_SPEC>;
#[doc = "USB Interrupt Status"]
pub mod intstat;
#[doc = "INTEN (rw) register accessor: an alias for `Reg<INTEN_SPEC>`"]
pub type INTEN = crate::Reg<inten::INTEN_SPEC>;
#[doc = "USB Interrupt Enable"]
pub mod inten;
#[doc = "INTSETSTAT (rw) register accessor: an alias for `Reg<INTSETSTAT_SPEC>`"]
pub type INTSETSTAT = crate::Reg<intsetstat::INTSETSTAT_SPEC>;
#[doc = "USB Set Interrupt Status"]
pub mod intsetstat;
#[doc = "EPTOGGLE (r) register accessor: an alias for `Reg<EPTOGGLE_SPEC>`"]
pub type EPTOGGLE = crate::Reg<eptoggle::EPTOGGLE_SPEC>;
#[doc = "USB Endpoint Toggle"]
pub mod eptoggle;
