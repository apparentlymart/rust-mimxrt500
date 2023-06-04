#[doc = r"Register block"]
#[repr(C)]
pub struct OUT {
    #[doc = "0x00 - Output n Set"]
    pub out_set: OUT_SET,
    #[doc = "0x04 - Output n Clear"]
    pub out_clr: OUT_CLR,
}
#[doc = "OUT_SET (rw) register accessor: an alias for `Reg<OUT_SET_SPEC>`"]
pub type OUT_SET = crate::Reg<out_set::OUT_SET_SPEC>;
#[doc = "Output n Set"]
pub mod out_set;
#[doc = "OUT_CLR (rw) register accessor: an alias for `Reg<OUT_CLR_SPEC>`"]
pub type OUT_CLR = crate::Reg<out_clr::OUT_CLR_SPEC>;
#[doc = "Output n Clear"]
pub mod out_clr;
