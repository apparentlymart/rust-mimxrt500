#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 0x0100],
    #[doc = "0x100 - Read channel QoS value"]
    pub read_qos: READ_QOS,
    #[doc = "0x104 - WRITE channel QoS value"]
    pub write_qos: WRITE_QOS,
    #[doc = "0x108 - Issuing Functionality Modification"]
    pub fn_mod: FN_MOD,
}
#[doc = "READ_QOS (rw) register accessor: an alias for `Reg<READ_QOS_SPEC>`"]
pub type READ_QOS = crate::Reg<read_qos::READ_QOS_SPEC>;
#[doc = "Read channel QoS value"]
pub mod read_qos;
#[doc = "WRITE_QOS (rw) register accessor: an alias for `Reg<WRITE_QOS_SPEC>`"]
pub type WRITE_QOS = crate::Reg<write_qos::WRITE_QOS_SPEC>;
#[doc = "WRITE channel QoS value"]
pub mod write_qos;
#[doc = "FN_MOD (rw) register accessor: an alias for `Reg<FN_MOD_SPEC>`"]
pub type FN_MOD = crate::Reg<fn_mod::FN_MOD_SPEC>;
#[doc = "Issuing Functionality Modification"]
pub mod fn_mod;
