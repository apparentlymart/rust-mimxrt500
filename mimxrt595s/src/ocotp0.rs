#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00..0x7c0 - One Time Programmable Controller shadow"]
    pub otp_shadow: [OTP_SHADOW; 496],
    _reserved1: [u8; 0x40],
    #[doc = "0x800 - Control/Address"]
    pub otp_ctrl: OTP_CTRL,
    #[doc = "0x804 - Power-down"]
    pub otp_pdn: OTP_PDN,
    #[doc = "0x808 - OTP programming data"]
    pub otp_write_data: OTP_WRITE_DATA,
    #[doc = "0x80c - OTP read start control"]
    pub otp_read_ctrl: OTP_READ_CTRL,
    #[doc = "0x810 - OTP read data"]
    pub otp_read_data: OTP_READ_DATA,
    #[doc = "0x814 - OTP clock divider"]
    pub otp_clk_div: OTP_CLK_DIV,
    _reserved7: [u8; 0x04],
    #[doc = "0x81c - CRC address range"]
    pub otp_crc_addr: OTP_CRC_ADDR,
    #[doc = "0x820 - CRC result"]
    pub otp_crc_value: OTP_CRC_VALUE,
    #[doc = "0x824 - OTP Status"]
    pub otp_status: OTP_STATUS,
    _reserved10: [u8; 0x04],
    #[doc = "0x82c - VERSION ID"]
    pub otp_version: OTP_VERSION,
}
#[doc = "OTP_SHADOW (rw) register accessor: an alias for `Reg<OTP_SHADOW_SPEC>`"]
pub type OTP_SHADOW = crate::Reg<otp_shadow::OTP_SHADOW_SPEC>;
#[doc = "One Time Programmable Controller shadow"]
pub mod otp_shadow;
#[doc = "OTP_CTRL (rw) register accessor: an alias for `Reg<OTP_CTRL_SPEC>`"]
pub type OTP_CTRL = crate::Reg<otp_ctrl::OTP_CTRL_SPEC>;
#[doc = "Control/Address"]
pub mod otp_ctrl;
#[doc = "OTP_PDN (rw) register accessor: an alias for `Reg<OTP_PDN_SPEC>`"]
pub type OTP_PDN = crate::Reg<otp_pdn::OTP_PDN_SPEC>;
#[doc = "Power-down"]
pub mod otp_pdn;
#[doc = "OTP_WRITE_DATA (rw) register accessor: an alias for `Reg<OTP_WRITE_DATA_SPEC>`"]
pub type OTP_WRITE_DATA = crate::Reg<otp_write_data::OTP_WRITE_DATA_SPEC>;
#[doc = "OTP programming data"]
pub mod otp_write_data;
#[doc = "OTP_READ_CTRL (rw) register accessor: an alias for `Reg<OTP_READ_CTRL_SPEC>`"]
pub type OTP_READ_CTRL = crate::Reg<otp_read_ctrl::OTP_READ_CTRL_SPEC>;
#[doc = "OTP read start control"]
pub mod otp_read_ctrl;
#[doc = "OTP_READ_DATA (r) register accessor: an alias for `Reg<OTP_READ_DATA_SPEC>`"]
pub type OTP_READ_DATA = crate::Reg<otp_read_data::OTP_READ_DATA_SPEC>;
#[doc = "OTP read data"]
pub mod otp_read_data;
#[doc = "OTP_CLK_DIV (rw) register accessor: an alias for `Reg<OTP_CLK_DIV_SPEC>`"]
pub type OTP_CLK_DIV = crate::Reg<otp_clk_div::OTP_CLK_DIV_SPEC>;
#[doc = "OTP clock divider"]
pub mod otp_clk_div;
#[doc = "OTP_CRC_ADDR (rw) register accessor: an alias for `Reg<OTP_CRC_ADDR_SPEC>`"]
pub type OTP_CRC_ADDR = crate::Reg<otp_crc_addr::OTP_CRC_ADDR_SPEC>;
#[doc = "CRC address range"]
pub mod otp_crc_addr;
#[doc = "OTP_CRC_VALUE (r) register accessor: an alias for `Reg<OTP_CRC_VALUE_SPEC>`"]
pub type OTP_CRC_VALUE = crate::Reg<otp_crc_value::OTP_CRC_VALUE_SPEC>;
#[doc = "CRC result"]
pub mod otp_crc_value;
#[doc = "OTP_STATUS (rw) register accessor: an alias for `Reg<OTP_STATUS_SPEC>`"]
pub type OTP_STATUS = crate::Reg<otp_status::OTP_STATUS_SPEC>;
#[doc = "OTP Status"]
pub mod otp_status;
#[doc = "OTP_VERSION (r) register accessor: an alias for `Reg<OTP_VERSION_SPEC>`"]
pub type OTP_VERSION = crate::Reg<otp_version::OTP_VERSION_SPEC>;
#[doc = "VERSION ID"]
pub mod otp_version;
