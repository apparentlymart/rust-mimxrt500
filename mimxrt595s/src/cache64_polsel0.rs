#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 0x14],
    #[doc = "0x14 - Region 0 Top Boundary"]
    pub reg0_top: REG0_TOP,
    #[doc = "0x18 - Region 1 Top Boundary"]
    pub reg1_top: REG1_TOP,
    #[doc = "0x1c - Policy Select"]
    pub polsel: POLSEL,
}
#[doc = "REG0_TOP (rw) register accessor: an alias for `Reg<REG0_TOP_SPEC>`"]
pub type REG0_TOP = crate::Reg<reg0_top::REG0_TOP_SPEC>;
#[doc = "Region 0 Top Boundary"]
pub mod reg0_top;
#[doc = "REG1_TOP (rw) register accessor: an alias for `Reg<REG1_TOP_SPEC>`"]
pub type REG1_TOP = crate::Reg<reg1_top::REG1_TOP_SPEC>;
#[doc = "Region 1 Top Boundary"]
pub mod reg1_top;
#[doc = "POLSEL (rw) register accessor: an alias for `Reg<POLSEL_SPEC>`"]
pub type POLSEL = crate::Reg<polsel::POLSEL_SPEC>;
#[doc = "Policy Select"]
pub mod polsel;
