#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 0x08],
    #[doc = "0x08 - Bus Matrix Issuing Functionality Modification."]
    pub fn_mod_bm_iss: FN_MOD_BM_ISS,
    _reserved1: [u8; 0x18],
    #[doc = "0x24 - Bypass Merge"]
    pub fn_mod2: FN_MOD2,
    _reserved2: [u8; 0xe0],
    #[doc = "0x108 - Issuing Functionality Modification"]
    pub fn_mod: FN_MOD,
}
#[doc = "FN_MOD_BM_ISS (rw) register accessor: an alias for `Reg<FN_MOD_BM_ISS_SPEC>`"]
pub type FN_MOD_BM_ISS = crate::Reg<fn_mod_bm_iss::FN_MOD_BM_ISS_SPEC>;
#[doc = "Bus Matrix Issuing Functionality Modification."]
pub mod fn_mod_bm_iss;
#[doc = "FN_MOD2 (rw) register accessor: an alias for `Reg<FN_MOD2_SPEC>`"]
pub type FN_MOD2 = crate::Reg<fn_mod2::FN_MOD2_SPEC>;
#[doc = "Bypass Merge"]
pub mod fn_mod2;
#[doc = "FN_MOD (rw) register accessor: an alias for `Reg<FN_MOD_SPEC>`"]
pub type FN_MOD = crate::Reg<fn_mod::FN_MOD_SPEC>;
#[doc = "Issuing Functionality Modification"]
pub mod fn_mod;
