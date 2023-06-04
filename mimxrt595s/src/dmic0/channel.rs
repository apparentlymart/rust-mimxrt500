#[doc = r"Register block"]
#[repr(C)]
pub struct CHANNEL {
    #[doc = "0x00 - Oversample Rate"]
    pub osr: OSR,
    #[doc = "0x04 - DMIC Clock"]
    pub divhfclk: DIVHFCLK,
    #[doc = "0x08 - Compensation Filter for 2 FS"]
    pub preac2fscoef: PREAC2FSCOEF,
    #[doc = "0x0c - Compensation Filter for 4 FS"]
    pub preac4fscoef: PREAC4FSCOEF,
    #[doc = "0x10 - Decimator Gain Shift"]
    pub gainshift: GAINSHIFT,
    _reserved5: [u8; 0x6c],
    #[doc = "0x80 - FIFO Control"]
    pub fifo_ctrl: FIFO_CTRL,
    #[doc = "0x84 - FIFO Status"]
    pub fifo_status: FIFO_STATUS,
    #[doc = "0x88 - FIFO Data"]
    pub fifo_data: FIFO_DATA,
    #[doc = "0x8c - Physical Control"]
    pub phy_ctrl: PHY_CTRL,
    #[doc = "0x90 - DC Filter Control"]
    pub dc_ctrl: DC_CTRL,
}
#[doc = "OSR (rw) register accessor: an alias for `Reg<OSR_SPEC>`"]
pub type OSR = crate::Reg<osr::OSR_SPEC>;
#[doc = "Oversample Rate"]
pub mod osr;
#[doc = "DIVHFCLK (rw) register accessor: an alias for `Reg<DIVHFCLK_SPEC>`"]
pub type DIVHFCLK = crate::Reg<divhfclk::DIVHFCLK_SPEC>;
#[doc = "DMIC Clock"]
pub mod divhfclk;
#[doc = "PREAC2FSCOEF (rw) register accessor: an alias for `Reg<PREAC2FSCOEF_SPEC>`"]
pub type PREAC2FSCOEF = crate::Reg<preac2fscoef::PREAC2FSCOEF_SPEC>;
#[doc = "Compensation Filter for 2 FS"]
pub mod preac2fscoef;
#[doc = "PREAC4FSCOEF (rw) register accessor: an alias for `Reg<PREAC4FSCOEF_SPEC>`"]
pub type PREAC4FSCOEF = crate::Reg<preac4fscoef::PREAC4FSCOEF_SPEC>;
#[doc = "Compensation Filter for 4 FS"]
pub mod preac4fscoef;
#[doc = "GAINSHIFT (rw) register accessor: an alias for `Reg<GAINSHIFT_SPEC>`"]
pub type GAINSHIFT = crate::Reg<gainshift::GAINSHIFT_SPEC>;
#[doc = "Decimator Gain Shift"]
pub mod gainshift;
#[doc = "FIFO_CTRL (rw) register accessor: an alias for `Reg<FIFO_CTRL_SPEC>`"]
pub type FIFO_CTRL = crate::Reg<fifo_ctrl::FIFO_CTRL_SPEC>;
#[doc = "FIFO Control"]
pub mod fifo_ctrl;
#[doc = "FIFO_STATUS (rw) register accessor: an alias for `Reg<FIFO_STATUS_SPEC>`"]
pub type FIFO_STATUS = crate::Reg<fifo_status::FIFO_STATUS_SPEC>;
#[doc = "FIFO Status"]
pub mod fifo_status;
#[doc = "FIFO_DATA (r) register accessor: an alias for `Reg<FIFO_DATA_SPEC>`"]
pub type FIFO_DATA = crate::Reg<fifo_data::FIFO_DATA_SPEC>;
#[doc = "FIFO Data"]
pub mod fifo_data;
#[doc = "PHY_CTRL (rw) register accessor: an alias for `Reg<PHY_CTRL_SPEC>`"]
pub type PHY_CTRL = crate::Reg<phy_ctrl::PHY_CTRL_SPEC>;
#[doc = "Physical Control"]
pub mod phy_ctrl;
#[doc = "DC_CTRL (rw) register accessor: an alias for `Reg<DC_CTRL_SPEC>`"]
pub type DC_CTRL = crate::Reg<dc_ctrl::DC_CTRL_SPEC>;
#[doc = "DC Filter Control"]
pub mod dc_ctrl;
