#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 0x04],
    #[doc = "0x04 - Status"]
    pub status: STATUS,
    #[doc = "0x08 - Wakeup, Interrupt, Reset Flags"]
    pub flags: FLAGS,
    #[doc = "0x0c - PMC control register"]
    pub ctrl: CTRL,
    #[doc = "0x10 - PMC controls used during run mode"]
    pub runctrl: RUNCTRL,
    #[doc = "0x14 - PMC controls used during deep sleep mode"]
    pub sleepctrl: SLEEPCTRL,
    #[doc = "0x18 - PMC Active vddcore LVD monitor trip adjust"]
    pub lvdcorectrl: LVDCORECTRL,
    _reserved6: [u8; 0x08],
    #[doc = "0x24 - PMC Automatic wakeup from deepsleep mode"]
    pub autowkup: AUTOWKUP,
    #[doc = "0x28 - PMIC Power Mode Select Control Configuration"]
    pub pmiccfg: PMICCFG,
    #[doc = "0x2c - PMC GPIO VDDIO Range Selection Control"]
    pub padvrange: PADVRANGE,
    #[doc = "0x30 - PMC Memory sequencer Control"]
    pub memseqctrl: MEMSEQCTRL,
    _reserved10: [u8; 0x2c],
    #[doc = "0x60 - PMC Temperature Sensor Control"]
    pub tsensor: TSENSOR,
}
#[doc = "STATUS (r) register accessor: an alias for `Reg<STATUS_SPEC>`"]
pub type STATUS = crate::Reg<status::STATUS_SPEC>;
#[doc = "Status"]
pub mod status;
#[doc = "FLAGS (rw) register accessor: an alias for `Reg<FLAGS_SPEC>`"]
pub type FLAGS = crate::Reg<flags::FLAGS_SPEC>;
#[doc = "Wakeup, Interrupt, Reset Flags"]
pub mod flags;
#[doc = "CTRL (rw) register accessor: an alias for `Reg<CTRL_SPEC>`"]
pub type CTRL = crate::Reg<ctrl::CTRL_SPEC>;
#[doc = "PMC control register"]
pub mod ctrl;
#[doc = "RUNCTRL (rw) register accessor: an alias for `Reg<RUNCTRL_SPEC>`"]
pub type RUNCTRL = crate::Reg<runctrl::RUNCTRL_SPEC>;
#[doc = "PMC controls used during run mode"]
pub mod runctrl;
#[doc = "SLEEPCTRL (rw) register accessor: an alias for `Reg<SLEEPCTRL_SPEC>`"]
pub type SLEEPCTRL = crate::Reg<sleepctrl::SLEEPCTRL_SPEC>;
#[doc = "PMC controls used during deep sleep mode"]
pub mod sleepctrl;
#[doc = "LVDCORECTRL (rw) register accessor: an alias for `Reg<LVDCORECTRL_SPEC>`"]
pub type LVDCORECTRL = crate::Reg<lvdcorectrl::LVDCORECTRL_SPEC>;
#[doc = "PMC Active vddcore LVD monitor trip adjust"]
pub mod lvdcorectrl;
#[doc = "AUTOWKUP (rw) register accessor: an alias for `Reg<AUTOWKUP_SPEC>`"]
pub type AUTOWKUP = crate::Reg<autowkup::AUTOWKUP_SPEC>;
#[doc = "PMC Automatic wakeup from deepsleep mode"]
pub mod autowkup;
#[doc = "PMICCFG (rw) register accessor: an alias for `Reg<PMICCFG_SPEC>`"]
pub type PMICCFG = crate::Reg<pmiccfg::PMICCFG_SPEC>;
#[doc = "PMIC Power Mode Select Control Configuration"]
pub mod pmiccfg;
#[doc = "PADVRANGE (rw) register accessor: an alias for `Reg<PADVRANGE_SPEC>`"]
pub type PADVRANGE = crate::Reg<padvrange::PADVRANGE_SPEC>;
#[doc = "PMC GPIO VDDIO Range Selection Control"]
pub mod padvrange;
#[doc = "MEMSEQCTRL (rw) register accessor: an alias for `Reg<MEMSEQCTRL_SPEC>`"]
pub type MEMSEQCTRL = crate::Reg<memseqctrl::MEMSEQCTRL_SPEC>;
#[doc = "PMC Memory sequencer Control"]
pub mod memseqctrl;
#[doc = "TSENSOR (rw) register accessor: an alias for `Reg<TSENSOR_SPEC>`"]
pub type TSENSOR = crate::Reg<tsensor::TSENSOR_SPEC>;
#[doc = "PMC Temperature Sensor Control"]
pub mod tsensor;
