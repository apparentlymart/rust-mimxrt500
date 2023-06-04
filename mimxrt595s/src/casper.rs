#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control 0"]
    pub ctrl0: CTRL0,
    #[doc = "0x04 - Control 1"]
    pub ctrl1: CTRL1,
    _reserved2: [u8; 0x04],
    #[doc = "0x0c - Status"]
    pub status: STATUS,
    #[doc = "0x10 - Interrupt Enable Set"]
    pub intenset: INTENSET,
    #[doc = "0x14 - Interrupt Enable Clear"]
    pub intenclr: INTENCLR,
    #[doc = "0x18 - Interrupt status"]
    pub intstat: INTSTAT,
    _reserved6: [u8; 0x04],
    #[doc = "0x20 - A Register"]
    pub areg: AREG,
    #[doc = "0x24 - B Register"]
    pub breg: BREG,
    #[doc = "0x28 - C Register"]
    pub creg: CREG,
    #[doc = "0x2c - D Register"]
    pub dreg: DREG,
    #[doc = "0x30 - Result Register 0"]
    pub res0: RES0,
    #[doc = "0x34 - Result Register 1"]
    pub res1: RES1,
    #[doc = "0x38 - Result Register 2"]
    pub res2: RES2,
    #[doc = "0x3c - Result Register 3"]
    pub res3: RES3,
    _reserved14: [u8; 0x20],
    #[doc = "0x60 - Mask"]
    pub mask: MASK,
    #[doc = "0x64 - Remask"]
    pub remask: REMASK,
    _reserved16: [u8; 0x18],
    #[doc = "0x80 - Lock"]
    pub lock: LOCK,
}
#[doc = "CTRL0 (rw) register accessor: an alias for `Reg<CTRL0_SPEC>`"]
pub type CTRL0 = crate::Reg<ctrl0::CTRL0_SPEC>;
#[doc = "Control 0"]
pub mod ctrl0;
#[doc = "CTRL1 (rw) register accessor: an alias for `Reg<CTRL1_SPEC>`"]
pub type CTRL1 = crate::Reg<ctrl1::CTRL1_SPEC>;
#[doc = "Control 1"]
pub mod ctrl1;
#[doc = "STATUS (rw) register accessor: an alias for `Reg<STATUS_SPEC>`"]
pub type STATUS = crate::Reg<status::STATUS_SPEC>;
#[doc = "Status"]
pub mod status;
#[doc = "INTENSET (rw) register accessor: an alias for `Reg<INTENSET_SPEC>`"]
pub type INTENSET = crate::Reg<intenset::INTENSET_SPEC>;
#[doc = "Interrupt Enable Set"]
pub mod intenset;
#[doc = "INTENCLR (rw) register accessor: an alias for `Reg<INTENCLR_SPEC>`"]
pub type INTENCLR = crate::Reg<intenclr::INTENCLR_SPEC>;
#[doc = "Interrupt Enable Clear"]
pub mod intenclr;
#[doc = "INTSTAT (r) register accessor: an alias for `Reg<INTSTAT_SPEC>`"]
pub type INTSTAT = crate::Reg<intstat::INTSTAT_SPEC>;
#[doc = "Interrupt status"]
pub mod intstat;
#[doc = "AREG (rw) register accessor: an alias for `Reg<AREG_SPEC>`"]
pub type AREG = crate::Reg<areg::AREG_SPEC>;
#[doc = "A Register"]
pub mod areg;
#[doc = "BREG (rw) register accessor: an alias for `Reg<BREG_SPEC>`"]
pub type BREG = crate::Reg<breg::BREG_SPEC>;
#[doc = "B Register"]
pub mod breg;
#[doc = "CREG (rw) register accessor: an alias for `Reg<CREG_SPEC>`"]
pub type CREG = crate::Reg<creg::CREG_SPEC>;
#[doc = "C Register"]
pub mod creg;
#[doc = "DREG (rw) register accessor: an alias for `Reg<DREG_SPEC>`"]
pub type DREG = crate::Reg<dreg::DREG_SPEC>;
#[doc = "D Register"]
pub mod dreg;
#[doc = "RES0 (rw) register accessor: an alias for `Reg<RES0_SPEC>`"]
pub type RES0 = crate::Reg<res0::RES0_SPEC>;
#[doc = "Result Register 0"]
pub mod res0;
#[doc = "RES1 (rw) register accessor: an alias for `Reg<RES1_SPEC>`"]
pub type RES1 = crate::Reg<res1::RES1_SPEC>;
#[doc = "Result Register 1"]
pub mod res1;
#[doc = "RES2 (rw) register accessor: an alias for `Reg<RES2_SPEC>`"]
pub type RES2 = crate::Reg<res2::RES2_SPEC>;
#[doc = "Result Register 2"]
pub mod res2;
#[doc = "RES3 (rw) register accessor: an alias for `Reg<RES3_SPEC>`"]
pub type RES3 = crate::Reg<res3::RES3_SPEC>;
#[doc = "Result Register 3"]
pub mod res3;
#[doc = "MASK (rw) register accessor: an alias for `Reg<MASK_SPEC>`"]
pub type MASK = crate::Reg<mask::MASK_SPEC>;
#[doc = "Mask"]
pub mod mask;
#[doc = "REMASK (rw) register accessor: an alias for `Reg<REMASK_SPEC>`"]
pub type REMASK = crate::Reg<remask::REMASK_SPEC>;
#[doc = "Remask"]
pub mod remask;
#[doc = "LOCK (rw) register accessor: an alias for `Reg<LOCK_SPEC>`"]
pub type LOCK = crate::Reg<lock::LOCK_SPEC>;
#[doc = "Lock"]
pub mod lock;
