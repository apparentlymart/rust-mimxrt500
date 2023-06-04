#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Version ID Register"]
    pub ver: VER,
    #[doc = "0x04 - Parameter Register"]
    pub par: PAR,
    _reserved2: [u8; 0x18],
    #[doc = "0x20..0x30 - Transmit Register"]
    pub tr: [TR; 4],
    _reserved3: [u8; 0x10],
    #[doc = "0x40..0x50 - Receive Register"]
    pub rr: [RR; 4],
    _reserved4: [u8; 0x10],
    #[doc = "0x60 - Status Register"]
    pub sr: SR,
    #[doc = "0x64 - Control Register"]
    pub cr: CR,
}
#[doc = "VER (r) register accessor: an alias for `Reg<VER_SPEC>`"]
pub type VER = crate::Reg<ver::VER_SPEC>;
#[doc = "Version ID Register"]
pub mod ver;
#[doc = "PAR (r) register accessor: an alias for `Reg<PAR_SPEC>`"]
pub type PAR = crate::Reg<par::PAR_SPEC>;
#[doc = "Parameter Register"]
pub mod par;
#[doc = "TR (rw) register accessor: an alias for `Reg<TR_SPEC>`"]
pub type TR = crate::Reg<tr::TR_SPEC>;
#[doc = "Transmit Register"]
pub mod tr;
#[doc = "RR (r) register accessor: an alias for `Reg<RR_SPEC>`"]
pub type RR = crate::Reg<rr::RR_SPEC>;
#[doc = "Receive Register"]
pub mod rr;
#[doc = "SR (rw) register accessor: an alias for `Reg<SR_SPEC>`"]
pub type SR = crate::Reg<sr::SR_SPEC>;
#[doc = "Status Register"]
pub mod sr;
#[doc = "CR (rw) register accessor: an alias for `Reg<CR_SPEC>`"]
pub type CR = crate::Reg<cr::CR_SPEC>;
#[doc = "Control Register"]
pub mod cr;
