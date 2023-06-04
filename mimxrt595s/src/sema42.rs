#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Gate"]
    pub gate3: GATE3,
    #[doc = "0x01 - Gate"]
    pub gate2: GATE2,
    #[doc = "0x02 - Gate"]
    pub gate1: GATE1,
    #[doc = "0x03 - Gate"]
    pub gate0: GATE0,
    #[doc = "0x04 - Gate"]
    pub gate7: GATE7,
    #[doc = "0x05 - Gate"]
    pub gate6: GATE6,
    #[doc = "0x06 - Gate"]
    pub gate5: GATE5,
    #[doc = "0x07 - Gate"]
    pub gate4: GATE4,
    #[doc = "0x08 - Gate"]
    pub gate11: GATE11,
    #[doc = "0x09 - Gate"]
    pub gate10: GATE10,
    #[doc = "0x0a - Gate"]
    pub gate9: GATE9,
    #[doc = "0x0b - Gate"]
    pub gate8: GATE8,
    #[doc = "0x0c - Gate"]
    pub gate15: GATE15,
    #[doc = "0x0d - Gate"]
    pub gate14: GATE14,
    #[doc = "0x0e - Gate"]
    pub gate13: GATE13,
    #[doc = "0x0f - Gate"]
    pub gate12: GATE12,
    _reserved16: [u8; 0x32],
    _reserved_16_rstgt_rstgt: [u8; 0x02],
}
impl RegisterBlock {
    #[doc = "0x42 - Reset Gate Write"]
    #[inline(always)]
    pub const fn rstgt_rstgt_w(&self) -> &RSTGT_RSTGT_W {
        unsafe { &*(self as *const Self).cast::<u8>().add(66usize).cast() }
    }
    #[doc = "0x42 - Reset Gate Read"]
    #[inline(always)]
    pub const fn rstgt_rstgt_r(&self) -> &RSTGT_RSTGT_R {
        unsafe { &*(self as *const Self).cast::<u8>().add(66usize).cast() }
    }
}
#[doc = "GATE3 (rw) register accessor: an alias for `Reg<GATE3_SPEC>`"]
pub type GATE3 = crate::Reg<gate3::GATE3_SPEC>;
#[doc = "Gate"]
pub mod gate3;
#[doc = "GATE2 (rw) register accessor: an alias for `Reg<GATE2_SPEC>`"]
pub type GATE2 = crate::Reg<gate2::GATE2_SPEC>;
#[doc = "Gate"]
pub mod gate2;
#[doc = "GATE1 (rw) register accessor: an alias for `Reg<GATE1_SPEC>`"]
pub type GATE1 = crate::Reg<gate1::GATE1_SPEC>;
#[doc = "Gate"]
pub mod gate1;
#[doc = "GATE0 (rw) register accessor: an alias for `Reg<GATE0_SPEC>`"]
pub type GATE0 = crate::Reg<gate0::GATE0_SPEC>;
#[doc = "Gate"]
pub mod gate0;
#[doc = "GATE7 (rw) register accessor: an alias for `Reg<GATE7_SPEC>`"]
pub type GATE7 = crate::Reg<gate7::GATE7_SPEC>;
#[doc = "Gate"]
pub mod gate7;
#[doc = "GATE6 (rw) register accessor: an alias for `Reg<GATE6_SPEC>`"]
pub type GATE6 = crate::Reg<gate6::GATE6_SPEC>;
#[doc = "Gate"]
pub mod gate6;
#[doc = "GATE5 (rw) register accessor: an alias for `Reg<GATE5_SPEC>`"]
pub type GATE5 = crate::Reg<gate5::GATE5_SPEC>;
#[doc = "Gate"]
pub mod gate5;
#[doc = "GATE4 (rw) register accessor: an alias for `Reg<GATE4_SPEC>`"]
pub type GATE4 = crate::Reg<gate4::GATE4_SPEC>;
#[doc = "Gate"]
pub mod gate4;
#[doc = "GATE11 (rw) register accessor: an alias for `Reg<GATE11_SPEC>`"]
pub type GATE11 = crate::Reg<gate11::GATE11_SPEC>;
#[doc = "Gate"]
pub mod gate11;
#[doc = "GATE10 (rw) register accessor: an alias for `Reg<GATE10_SPEC>`"]
pub type GATE10 = crate::Reg<gate10::GATE10_SPEC>;
#[doc = "Gate"]
pub mod gate10;
#[doc = "GATE9 (rw) register accessor: an alias for `Reg<GATE9_SPEC>`"]
pub type GATE9 = crate::Reg<gate9::GATE9_SPEC>;
#[doc = "Gate"]
pub mod gate9;
#[doc = "GATE8 (rw) register accessor: an alias for `Reg<GATE8_SPEC>`"]
pub type GATE8 = crate::Reg<gate8::GATE8_SPEC>;
#[doc = "Gate"]
pub mod gate8;
#[doc = "GATE15 (rw) register accessor: an alias for `Reg<GATE15_SPEC>`"]
pub type GATE15 = crate::Reg<gate15::GATE15_SPEC>;
#[doc = "Gate"]
pub mod gate15;
#[doc = "GATE14 (rw) register accessor: an alias for `Reg<GATE14_SPEC>`"]
pub type GATE14 = crate::Reg<gate14::GATE14_SPEC>;
#[doc = "Gate"]
pub mod gate14;
#[doc = "GATE13 (rw) register accessor: an alias for `Reg<GATE13_SPEC>`"]
pub type GATE13 = crate::Reg<gate13::GATE13_SPEC>;
#[doc = "Gate"]
pub mod gate13;
#[doc = "GATE12 (rw) register accessor: an alias for `Reg<GATE12_SPEC>`"]
pub type GATE12 = crate::Reg<gate12::GATE12_SPEC>;
#[doc = "Gate"]
pub mod gate12;
#[doc = "RSTGT_RSTGT_R (r) register accessor: an alias for `Reg<RSTGT_RSTGT_R_SPEC>`"]
pub type RSTGT_RSTGT_R = crate::Reg<rstgt_rstgt_r::RSTGT_RSTGT_R_SPEC>;
#[doc = "Reset Gate Read"]
pub mod rstgt_rstgt_r;
#[doc = "RSTGT_RSTGT_W (w) register accessor: an alias for `Reg<RSTGT_RSTGT_W_SPEC>`"]
pub type RSTGT_RSTGT_W = crate::Reg<rstgt_rstgt_w::RSTGT_RSTGT_W_SPEC>;
#[doc = "Reset Gate Write"]
pub mod rstgt_rstgt_w;
