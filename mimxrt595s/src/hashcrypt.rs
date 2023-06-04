#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control"]
    pub ctrl: CTRL,
    #[doc = "0x04 - Status"]
    pub status: STATUS,
    #[doc = "0x08 - Interrupt Enable"]
    pub intenset: INTENSET,
    #[doc = "0x0c - Interrupt Clear"]
    pub intenclr: INTENCLR,
    #[doc = "0x10 - Memory Control"]
    pub memctrl: MEMCTRL,
    #[doc = "0x14 - Memory Address"]
    pub memaddr: MEMADDR,
    _reserved6: [u8; 0x08],
    #[doc = "0x20 - Input Data"]
    pub indata: INDATA,
    #[doc = "0x24..0x40 - Alias"]
    pub alias: [ALIAS; 7],
    #[doc = "0x40..0x60 - Digest0 n/Output Data0 n"]
    pub digest0: [DIGEST0; 8],
    _reserved9: [u8; 0x20],
    #[doc = "0x80 - Cryptographic Configuration"]
    pub cryptcfg: CRYPTCFG,
    #[doc = "0x84 - Configuration"]
    pub config: CONFIG,
    _reserved11: [u8; 0x04],
    #[doc = "0x8c - Lock"]
    pub lock: LOCK,
    #[doc = "0x90..0xa0 - Mask"]
    pub mask: [MASK; 4],
    #[doc = "0xa0..0xc0 - DIGEST/OUTDATA Reload"]
    pub reload: [RELOAD; 8],
    _reserved14: [u8; 0x10],
    #[doc = "0xd0 - PRNG Seed"]
    pub prng_seed: PRNG_SEED,
    _reserved15: [u8; 0x04],
    #[doc = "0xd8 - PRNG Output"]
    pub prng_out: PRNG_OUT,
}
#[doc = "CTRL (rw) register accessor: an alias for `Reg<CTRL_SPEC>`"]
pub type CTRL = crate::Reg<ctrl::CTRL_SPEC>;
#[doc = "Control"]
pub mod ctrl;
#[doc = "STATUS (rw) register accessor: an alias for `Reg<STATUS_SPEC>`"]
pub type STATUS = crate::Reg<status::STATUS_SPEC>;
#[doc = "Status"]
pub mod status;
#[doc = "INTENSET (rw) register accessor: an alias for `Reg<INTENSET_SPEC>`"]
pub type INTENSET = crate::Reg<intenset::INTENSET_SPEC>;
#[doc = "Interrupt Enable"]
pub mod intenset;
#[doc = "INTENCLR (rw) register accessor: an alias for `Reg<INTENCLR_SPEC>`"]
pub type INTENCLR = crate::Reg<intenclr::INTENCLR_SPEC>;
#[doc = "Interrupt Clear"]
pub mod intenclr;
#[doc = "MEMCTRL (rw) register accessor: an alias for `Reg<MEMCTRL_SPEC>`"]
pub type MEMCTRL = crate::Reg<memctrl::MEMCTRL_SPEC>;
#[doc = "Memory Control"]
pub mod memctrl;
#[doc = "MEMADDR (rw) register accessor: an alias for `Reg<MEMADDR_SPEC>`"]
pub type MEMADDR = crate::Reg<memaddr::MEMADDR_SPEC>;
#[doc = "Memory Address"]
pub mod memaddr;
#[doc = "INDATA (w) register accessor: an alias for `Reg<INDATA_SPEC>`"]
pub type INDATA = crate::Reg<indata::INDATA_SPEC>;
#[doc = "Input Data"]
pub mod indata;
#[doc = "ALIAS (w) register accessor: an alias for `Reg<ALIAS_SPEC>`"]
pub type ALIAS = crate::Reg<alias::ALIAS_SPEC>;
#[doc = "Alias"]
pub mod alias;
#[doc = "DIGEST0 (r) register accessor: an alias for `Reg<DIGEST0_SPEC>`"]
pub type DIGEST0 = crate::Reg<digest0::DIGEST0_SPEC>;
#[doc = "Digest0 n/Output Data0 n"]
pub mod digest0;
#[doc = "CRYPTCFG (rw) register accessor: an alias for `Reg<CRYPTCFG_SPEC>`"]
pub type CRYPTCFG = crate::Reg<cryptcfg::CRYPTCFG_SPEC>;
#[doc = "Cryptographic Configuration"]
pub mod cryptcfg;
#[doc = "CONFIG (r) register accessor: an alias for `Reg<CONFIG_SPEC>`"]
pub type CONFIG = crate::Reg<config::CONFIG_SPEC>;
#[doc = "Configuration"]
pub mod config;
#[doc = "LOCK (rw) register accessor: an alias for `Reg<LOCK_SPEC>`"]
pub type LOCK = crate::Reg<lock::LOCK_SPEC>;
#[doc = "Lock"]
pub mod lock;
#[doc = "MASK (w) register accessor: an alias for `Reg<MASK_SPEC>`"]
pub type MASK = crate::Reg<mask::MASK_SPEC>;
#[doc = "Mask"]
pub mod mask;
#[doc = "RELOAD (rw) register accessor: an alias for `Reg<RELOAD_SPEC>`"]
pub type RELOAD = crate::Reg<reload::RELOAD_SPEC>;
#[doc = "DIGEST/OUTDATA Reload"]
pub mod reload;
#[doc = "PRNG_SEED (w) register accessor: an alias for `Reg<PRNG_SEED_SPEC>`"]
pub type PRNG_SEED = crate::Reg<prng_seed::PRNG_SEED_SPEC>;
#[doc = "PRNG Seed"]
pub mod prng_seed;
#[doc = "PRNG_OUT (w) register accessor: an alias for `Reg<PRNG_OUT_SPEC>`"]
pub type PRNG_OUT = crate::Reg<prng_out::PRNG_OUT_SPEC>;
#[doc = "PRNG Output"]
pub mod prng_out;
