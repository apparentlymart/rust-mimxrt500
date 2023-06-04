#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Version ID Register"]
    pub verid: VERID,
    #[doc = "0x04 - Parameter Register"]
    pub param: PARAM,
    #[doc = "0x08 - CMP Control Register 0"]
    pub c0: C0,
    #[doc = "0x0c - CMP Control Register 1"]
    pub c1: C1,
    #[doc = "0x10 - CMP Control Register 2"]
    pub c2: C2,
    #[doc = "0x14 - CMP Control Register 3"]
    pub c3: C3,
    #[doc = "0x18 - Round-Robin Timer Control Register"]
    pub rr_timer_cr: RR_TIMER_CR,
}
#[doc = "VERID (r) register accessor: an alias for `Reg<VERID_SPEC>`"]
pub type VERID = crate::Reg<verid::VERID_SPEC>;
#[doc = "Version ID Register"]
pub mod verid;
#[doc = "PARAM (r) register accessor: an alias for `Reg<PARAM_SPEC>`"]
pub type PARAM = crate::Reg<param::PARAM_SPEC>;
#[doc = "Parameter Register"]
pub mod param;
#[doc = "C0 (rw) register accessor: an alias for `Reg<C0_SPEC>`"]
pub type C0 = crate::Reg<c0::C0_SPEC>;
#[doc = "CMP Control Register 0"]
pub mod c0;
#[doc = "C1 (rw) register accessor: an alias for `Reg<C1_SPEC>`"]
pub type C1 = crate::Reg<c1::C1_SPEC>;
#[doc = "CMP Control Register 1"]
pub mod c1;
#[doc = "C2 (rw) register accessor: an alias for `Reg<C2_SPEC>`"]
pub type C2 = crate::Reg<c2::C2_SPEC>;
#[doc = "CMP Control Register 2"]
pub mod c2;
#[doc = "C3 (rw) register accessor: an alias for `Reg<C3_SPEC>`"]
pub type C3 = crate::Reg<c3::C3_SPEC>;
#[doc = "CMP Control Register 3"]
pub mod c3;
#[doc = "RR_TIMER_CR (rw) register accessor: an alias for `Reg<RR_TIMER_CR_SPEC>`"]
pub type RR_TIMER_CR = crate::Reg<rr_timer_cr::RR_TIMER_CR_SPEC>;
#[doc = "Round-Robin Timer Control Register"]
pub mod rr_timer_cr;
