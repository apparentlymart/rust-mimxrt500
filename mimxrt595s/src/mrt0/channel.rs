#[doc = r"Register block"]
#[repr(C)]
pub struct CHANNEL {
    #[doc = "0x00 - Time Interval Value"]
    pub intval: INTVAL,
    #[doc = "0x04 - Timer"]
    pub timer: TIMER,
    #[doc = "0x08 - Control"]
    pub ctrl: CTRL,
    #[doc = "0x0c - Status"]
    pub stat: STAT,
}
#[doc = "INTVAL (rw) register accessor: an alias for `Reg<INTVAL_SPEC>`"]
pub type INTVAL = crate::Reg<intval::INTVAL_SPEC>;
#[doc = "Time Interval Value"]
pub mod intval;
#[doc = "TIMER (r) register accessor: an alias for `Reg<TIMER_SPEC>`"]
pub type TIMER = crate::Reg<timer::TIMER_SPEC>;
#[doc = "Timer"]
pub mod timer;
#[doc = "CTRL (rw) register accessor: an alias for `Reg<CTRL_SPEC>`"]
pub type CTRL = crate::Reg<ctrl::CTRL_SPEC>;
#[doc = "Control"]
pub mod ctrl;
#[doc = "STAT (rw) register accessor: an alias for `Reg<STAT_SPEC>`"]
pub type STAT = crate::Reg<stat::STAT_SPEC>;
#[doc = "Status"]
pub mod stat;
