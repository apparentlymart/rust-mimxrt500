#[doc = r"Register block"]
#[repr(C)]
pub struct EV {
    #[doc = "0x00 - Event n State"]
    pub ev_state: EV_STATE,
    #[doc = "0x04 - Event n Control"]
    pub ev_ctrl: EV_CTRL,
}
#[doc = "EV_STATE (rw) register accessor: an alias for `Reg<EV_STATE_SPEC>`"]
pub type EV_STATE = crate::Reg<ev_state::EV_STATE_SPEC>;
#[doc = "Event n State"]
pub mod ev_state;
#[doc = "EV_CTRL (rw) register accessor: an alias for `Reg<EV_CTRL_SPEC>`"]
pub type EV_CTRL = crate::Reg<ev_ctrl::EV_CTRL_SPEC>;
#[doc = "Event n Control"]
pub mod ev_ctrl;
