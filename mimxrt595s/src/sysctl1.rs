#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Update Clock Lockout"]
    pub updatelckout: UPDATELCKOUT,
    _reserved1: [u8; 0x0c],
    #[doc = "0x10 - MCLK direction control"]
    pub mclkpindir: MCLKPINDIR,
    _reserved2: [u8; 0x1c],
    #[doc = "0x30 - DSP NMI source selection"]
    pub dspnmisrcsel: DSPNMISRCSEL,
    _reserved3: [u8; 0x0c],
    #[doc = "0x40..0x78 - Flexcomm control selection"]
    pub fcctrlsel: [FCCTRLSEL; 14],
    _reserved4: [u8; 0x08],
    #[doc = "0x80..0x88 - Shared control set"]
    pub sharedctrlset: [SHAREDCTRLSET; 2],
    _reserved5: [u8; 0x0178],
    #[doc = "0x200 - RX Event Pulse Generator"]
    pub rxevpulsegen: RXEVPULSEGEN,
}
#[doc = "UPDATELCKOUT (rw) register accessor: an alias for `Reg<UPDATELCKOUT_SPEC>`"]
pub type UPDATELCKOUT = crate::Reg<updatelckout::UPDATELCKOUT_SPEC>;
#[doc = "Update Clock Lockout"]
pub mod updatelckout;
#[doc = "MCLKPINDIR (rw) register accessor: an alias for `Reg<MCLKPINDIR_SPEC>`"]
pub type MCLKPINDIR = crate::Reg<mclkpindir::MCLKPINDIR_SPEC>;
#[doc = "MCLK direction control"]
pub mod mclkpindir;
#[doc = "DSPNMISRCSEL (rw) register accessor: an alias for `Reg<DSPNMISRCSEL_SPEC>`"]
pub type DSPNMISRCSEL = crate::Reg<dspnmisrcsel::DSPNMISRCSEL_SPEC>;
#[doc = "DSP NMI source selection"]
pub mod dspnmisrcsel;
#[doc = "FCCTRLSEL (rw) register accessor: an alias for `Reg<FCCTRLSEL_SPEC>`"]
pub type FCCTRLSEL = crate::Reg<fcctrlsel::FCCTRLSEL_SPEC>;
#[doc = "Flexcomm control selection"]
pub mod fcctrlsel;
#[doc = "SHAREDCTRLSET (rw) register accessor: an alias for `Reg<SHAREDCTRLSET_SPEC>`"]
pub type SHAREDCTRLSET = crate::Reg<sharedctrlset::SHAREDCTRLSET_SPEC>;
#[doc = "Shared control set"]
pub mod sharedctrlset;
#[doc = "RXEVPULSEGEN (w) register accessor: an alias for `Reg<RXEVPULSEGEN_SPEC>`"]
pub type RXEVPULSEGEN = crate::Reg<rxevpulsegen::RXEVPULSEGEN_SPEC>;
#[doc = "RX Event Pulse Generator"]
pub mod rxevpulsegen;
