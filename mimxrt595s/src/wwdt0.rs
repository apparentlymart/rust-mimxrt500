#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Mode"]
    pub mod_: MOD,
    #[doc = "0x04 - Timer Constant"]
    pub tc: TC,
    #[doc = "0x08 - Feed Sequence"]
    pub feed: FEED,
    #[doc = "0x0c - Timer Value"]
    pub tv: TV,
    _reserved4: [u8; 0x04],
    #[doc = "0x14 - Warning Interrupt Compare Value"]
    pub warnint: WARNINT,
    #[doc = "0x18 - Window Compare Value"]
    pub window: WINDOW,
}
#[doc = "MOD (rw) register accessor: an alias for `Reg<MOD_SPEC>`"]
pub type MOD = crate::Reg<mod_::MOD_SPEC>;
#[doc = "Mode"]
pub mod mod_;
#[doc = "TC (rw) register accessor: an alias for `Reg<TC_SPEC>`"]
pub type TC = crate::Reg<tc::TC_SPEC>;
#[doc = "Timer Constant"]
pub mod tc;
#[doc = "FEED (w) register accessor: an alias for `Reg<FEED_SPEC>`"]
pub type FEED = crate::Reg<feed::FEED_SPEC>;
#[doc = "Feed Sequence"]
pub mod feed;
#[doc = "TV (r) register accessor: an alias for `Reg<TV_SPEC>`"]
pub type TV = crate::Reg<tv::TV_SPEC>;
#[doc = "Timer Value"]
pub mod tv;
#[doc = "WARNINT (rw) register accessor: an alias for `Reg<WARNINT_SPEC>`"]
pub type WARNINT = crate::Reg<warnint::WARNINT_SPEC>;
#[doc = "Warning Interrupt Compare Value"]
pub mod warnint;
#[doc = "WINDOW (rw) register accessor: an alias for `Reg<WINDOW_SPEC>`"]
pub type WINDOW = crate::Reg<window::WINDOW_SPEC>;
#[doc = "Window Compare Value"]
pub mod window;
