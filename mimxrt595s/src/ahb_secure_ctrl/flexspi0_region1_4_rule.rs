#[doc = r"Register block"]
#[repr(C)]
pub struct FLEXSPI0_REGION1_4_RULE {
    #[doc = "0x00 - FLEXSPI0 Region index Rule 0 Register"]
    pub flexspi0_region_rule0: FLEXSPI0_REGION_RULE0,
}
#[doc = "FLEXSPI0_REGION_RULE0 (rw) register accessor: an alias for `Reg<FLEXSPI0_REGION_RULE0_SPEC>`"]
pub type FLEXSPI0_REGION_RULE0 = crate::Reg<flexspi0_region_rule0::FLEXSPI0_REGION_RULE0_SPEC>;
#[doc = "FLEXSPI0 Region index Rule 0 Register"]
pub mod flexspi0_region_rule0;
