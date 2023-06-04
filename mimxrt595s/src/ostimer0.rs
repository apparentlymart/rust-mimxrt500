#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - EVTIMER Low"]
    pub evtimerl: EVTIMERL,
    #[doc = "0x04 - EVTIMER High"]
    pub evtimerh: EVTIMERH,
    #[doc = "0x08 - Local Capture Low for CPU"]
    pub capture_l: CAPTURE_L,
    #[doc = "0x0c - Local Capture High for CPU"]
    pub capture_h: CAPTURE_H,
    #[doc = "0x10 - Local Match Low for CPU"]
    pub match_l: MATCH_L,
    #[doc = "0x14 - Local Match High for CPU"]
    pub match_h: MATCH_H,
    _reserved6: [u8; 0x04],
    #[doc = "0x1c - OSTIMER Control for CPU"]
    pub osevent_ctrl: OSEVENT_CTRL,
}
#[doc = "EVTIMERL (r) register accessor: an alias for `Reg<EVTIMERL_SPEC>`"]
pub type EVTIMERL = crate::Reg<evtimerl::EVTIMERL_SPEC>;
#[doc = "EVTIMER Low"]
pub mod evtimerl;
#[doc = "EVTIMERH (r) register accessor: an alias for `Reg<EVTIMERH_SPEC>`"]
pub type EVTIMERH = crate::Reg<evtimerh::EVTIMERH_SPEC>;
#[doc = "EVTIMER High"]
pub mod evtimerh;
#[doc = "CAPTURE_L (r) register accessor: an alias for `Reg<CAPTURE_L_SPEC>`"]
pub type CAPTURE_L = crate::Reg<capture_l::CAPTURE_L_SPEC>;
#[doc = "Local Capture Low for CPU"]
pub mod capture_l;
#[doc = "CAPTURE_H (r) register accessor: an alias for `Reg<CAPTURE_H_SPEC>`"]
pub type CAPTURE_H = crate::Reg<capture_h::CAPTURE_H_SPEC>;
#[doc = "Local Capture High for CPU"]
pub mod capture_h;
#[doc = "MATCH_L (rw) register accessor: an alias for `Reg<MATCH_L_SPEC>`"]
pub type MATCH_L = crate::Reg<match_l::MATCH_L_SPEC>;
#[doc = "Local Match Low for CPU"]
pub mod match_l;
#[doc = "MATCH_H (rw) register accessor: an alias for `Reg<MATCH_H_SPEC>`"]
pub type MATCH_H = crate::Reg<match_h::MATCH_H_SPEC>;
#[doc = "Local Match High for CPU"]
pub mod match_h;
#[doc = "OSEVENT_CTRL (rw) register accessor: an alias for `Reg<OSEVENT_CTRL_SPEC>`"]
pub type OSEVENT_CTRL = crate::Reg<osevent_ctrl::OSEVENT_CTRL_SPEC>;
#[doc = "OSTIMER Control for CPU"]
pub mod osevent_ctrl;
