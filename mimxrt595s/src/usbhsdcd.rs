#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control"]
    pub control: CONTROL,
    #[doc = "0x04 - Clock"]
    pub clock: CLOCK,
    #[doc = "0x08 - Status"]
    pub status: STATUS,
    #[doc = "0x0c - Signal Override"]
    pub signal_override: SIGNAL_OVERRIDE,
    #[doc = "0x10 - TIMER0"]
    pub timer0: TIMER0,
    #[doc = "0x14 - TIMER1"]
    pub timer1: TIMER1,
    _reserved_6_timer2_timer2_bc1: [u8; 0x04],
}
impl RegisterBlock {
    #[doc = "0x18 - TIMER2_BC12"]
    #[inline(always)]
    pub const fn timer2_timer2_bc12(&self) -> &TIMER2_TIMER2_BC12 {
        unsafe { &*(self as *const Self).cast::<u8>().add(24usize).cast() }
    }
    #[doc = "0x18 - TIMER2_BC11"]
    #[inline(always)]
    pub const fn timer2_timer2_bc11(&self) -> &TIMER2_TIMER2_BC11 {
        unsafe { &*(self as *const Self).cast::<u8>().add(24usize).cast() }
    }
}
#[doc = "CONTROL (rw) register accessor: an alias for `Reg<CONTROL_SPEC>`"]
pub type CONTROL = crate::Reg<control::CONTROL_SPEC>;
#[doc = "Control"]
pub mod control;
#[doc = "CLOCK (rw) register accessor: an alias for `Reg<CLOCK_SPEC>`"]
pub type CLOCK = crate::Reg<clock::CLOCK_SPEC>;
#[doc = "Clock"]
pub mod clock;
#[doc = "STATUS (r) register accessor: an alias for `Reg<STATUS_SPEC>`"]
pub type STATUS = crate::Reg<status::STATUS_SPEC>;
#[doc = "Status"]
pub mod status;
#[doc = "SIGNAL_OVERRIDE (rw) register accessor: an alias for `Reg<SIGNAL_OVERRIDE_SPEC>`"]
pub type SIGNAL_OVERRIDE = crate::Reg<signal_override::SIGNAL_OVERRIDE_SPEC>;
#[doc = "Signal Override"]
pub mod signal_override;
#[doc = "TIMER0 (rw) register accessor: an alias for `Reg<TIMER0_SPEC>`"]
pub type TIMER0 = crate::Reg<timer0::TIMER0_SPEC>;
#[doc = "TIMER0"]
pub mod timer0;
#[doc = "TIMER1 (rw) register accessor: an alias for `Reg<TIMER1_SPEC>`"]
pub type TIMER1 = crate::Reg<timer1::TIMER1_SPEC>;
#[doc = "TIMER1"]
pub mod timer1;
#[doc = "TIMER2_TIMER2_BC11 (rw) register accessor: an alias for `Reg<TIMER2_TIMER2_BC11_SPEC>`"]
pub type TIMER2_TIMER2_BC11 = crate::Reg<timer2_timer2_bc11::TIMER2_TIMER2_BC11_SPEC>;
#[doc = "TIMER2_BC11"]
pub mod timer2_timer2_bc11;
#[doc = "TIMER2_TIMER2_BC12 (rw) register accessor: an alias for `Reg<TIMER2_TIMER2_BC12_SPEC>`"]
pub type TIMER2_TIMER2_BC12 = crate::Reg<timer2_timer2_bc12::TIMER2_TIMER2_BC12_SPEC>;
#[doc = "TIMER2_BC12"]
pub mod timer2_timer2_bc12;
