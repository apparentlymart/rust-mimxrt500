#[doc = r"Register block"]
#[repr(C)]
pub struct CT32BIT_CAP_SEL {
    #[doc = "0x00..0x10 - CT32BIT Timer Capture Multiplexers"]
    pub ct32bit_cap_sel_sub: [CT32BIT_CAP_SEL_SUB; 4],
}
#[doc = "CT32BIT_CAP_SEL_SUB (rw) register accessor: an alias for `Reg<CT32BIT_CAP_SEL_SUB_SPEC>`"]
pub type CT32BIT_CAP_SEL_SUB = crate::Reg<ct32bit_cap_sel_sub::CT32BIT_CAP_SEL_SUB_SPEC>;
#[doc = "CT32BIT Timer Capture Multiplexers"]
pub mod ct32bit_cap_sel_sub;
