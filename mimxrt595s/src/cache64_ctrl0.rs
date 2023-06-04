#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 0x0800],
    #[doc = "0x800 - Cache Control"]
    pub ccr: CCR,
    #[doc = "0x804 - Cache Line Control"]
    pub clcr: CLCR,
    #[doc = "0x808 - Cache Search Address"]
    pub csar: CSAR,
    #[doc = "0x80c - Cache Read/Write Value"]
    pub ccvr: CCVR,
}
#[doc = "CCR (rw) register accessor: an alias for `Reg<CCR_SPEC>`"]
pub type CCR = crate::Reg<ccr::CCR_SPEC>;
#[doc = "Cache Control"]
pub mod ccr;
#[doc = "CLCR (rw) register accessor: an alias for `Reg<CLCR_SPEC>`"]
pub type CLCR = crate::Reg<clcr::CLCR_SPEC>;
#[doc = "Cache Line Control"]
pub mod clcr;
#[doc = "CSAR (rw) register accessor: an alias for `Reg<CSAR_SPEC>`"]
pub type CSAR = crate::Reg<csar::CSAR_SPEC>;
#[doc = "Cache Search Address"]
pub mod csar;
#[doc = "CCVR (rw) register accessor: an alias for `Reg<CCVR_SPEC>`"]
pub type CCVR = crate::Reg<ccvr::CCVR_SPEC>;
#[doc = "Cache Read/Write Value"]
pub mod ccvr;
