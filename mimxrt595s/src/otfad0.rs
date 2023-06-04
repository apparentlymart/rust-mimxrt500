#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 0x0c00],
    #[doc = "0xc00 - Control Register"]
    pub cr: CR,
    #[doc = "0xc04 - Status Register"]
    pub sr: SR,
    _reserved2: [u8; 0xf8],
    #[doc = "0xd00..0xd20 - no description available"]
    pub ctx0: CTX,
    _reserved3: [u8; 0x20],
    #[doc = "0xd40..0xd60 - no description available"]
    pub ctx1: CTX,
    _reserved4: [u8; 0x20],
    #[doc = "0xd80..0xda0 - no description available"]
    pub ctx2: CTX,
    _reserved5: [u8; 0x20],
    #[doc = "0xdc0..0xde0 - no description available"]
    pub ctx3: CTX,
}
#[doc = "CR (rw) register accessor: an alias for `Reg<CR_SPEC>`"]
pub type CR = crate::Reg<cr::CR_SPEC>;
#[doc = "Control Register"]
pub mod cr;
#[doc = "SR (r) register accessor: an alias for `Reg<SR_SPEC>`"]
pub type SR = crate::Reg<sr::SR_SPEC>;
#[doc = "Status Register"]
pub mod sr;
#[doc = "no description available"]
pub use self::ctx::CTX;
#[doc = r"Cluster"]
#[doc = "no description available"]
pub mod ctx;
