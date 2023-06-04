#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Version ID Register"]
    pub caplength_chipid: CAPLENGTH_CHIPID,
    #[doc = "0x04 - Host Controller Structural Parameters"]
    pub hcsparams: HCSPARAMS,
    #[doc = "0x08 - INT PTD Base Address"]
    pub hccparams: HCCPARAMS,
    #[doc = "0x0c - Frame Length Adjustment"]
    pub fladj_frindex: FLADJ_FRINDEX,
    #[doc = "0x10 - ATL PTD Base Address"]
    pub atl_ptd_base_address: ATL_PTD_BASE_ADDRESS,
    #[doc = "0x14 - ISO PTD Base Address"]
    pub iso_ptd_base_address: ISO_PTD_BASE_ADDRESS,
    #[doc = "0x18 - INT PTD Base Address"]
    pub int_ptd_base_address: INT_PTD_BASE_ADDRESS,
    #[doc = "0x1c - DATA PAYLOAD Base Address"]
    pub data_payload_base_address: DATA_PAYLOAD_BASE_ADDRESS,
    #[doc = "0x20 - USB Command"]
    pub usbcmd: USBCMD,
    #[doc = "0x24 - USB Interrupt Status"]
    pub usbsts: USBSTS,
    #[doc = "0x28 - USB Interrupt Status"]
    pub usbintr: USBINTR,
    #[doc = "0x2c - Port Status and Control"]
    pub portsc1: PORTSC1,
    #[doc = "0x30 - ATL PTD Done Map"]
    pub atl_done: ATL_DONE,
    #[doc = "0x34 - ATL PTD Skip Map"]
    pub atl_skip: ATL_SKIP,
    #[doc = "0x38 - ISO PTD Done Map"]
    pub iso_done: ISO_DONE,
    #[doc = "0x3c - ISO PTD Skip Map"]
    pub iso_skip: ISO_SKIP,
    #[doc = "0x40 - INT PTD Done Map"]
    pub int_done: INT_DONE,
    #[doc = "0x44 - INT PTD Skip Map"]
    pub int_skip: INT_SKIP,
    #[doc = "0x48 - Last PTD in use"]
    pub last_ptd: LAST_PTD,
    _reserved19: [u8; 0x04],
    #[doc = "0x50 - Port Mode"]
    pub port_mode: PORT_MODE,
}
#[doc = "CAPLENGTH_CHIPID (r) register accessor: an alias for `Reg<CAPLENGTH_CHIPID_SPEC>`"]
pub type CAPLENGTH_CHIPID = crate::Reg<caplength_chipid::CAPLENGTH_CHIPID_SPEC>;
#[doc = "Version ID Register"]
pub mod caplength_chipid;
#[doc = "HCSPARAMS (r) register accessor: an alias for `Reg<HCSPARAMS_SPEC>`"]
pub type HCSPARAMS = crate::Reg<hcsparams::HCSPARAMS_SPEC>;
#[doc = "Host Controller Structural Parameters"]
pub mod hcsparams;
#[doc = "HCCPARAMS (rw) register accessor: an alias for `Reg<HCCPARAMS_SPEC>`"]
pub type HCCPARAMS = crate::Reg<hccparams::HCCPARAMS_SPEC>;
#[doc = "INT PTD Base Address"]
pub mod hccparams;
#[doc = "FLADJ_FRINDEX (rw) register accessor: an alias for `Reg<FLADJ_FRINDEX_SPEC>`"]
pub type FLADJ_FRINDEX = crate::Reg<fladj_frindex::FLADJ_FRINDEX_SPEC>;
#[doc = "Frame Length Adjustment"]
pub mod fladj_frindex;
#[doc = "ATL_PTD_BASE_ADDRESS (rw) register accessor: an alias for `Reg<ATL_PTD_BASE_ADDRESS_SPEC>`"]
pub type ATL_PTD_BASE_ADDRESS = crate::Reg<atl_ptd_base_address::ATL_PTD_BASE_ADDRESS_SPEC>;
#[doc = "ATL PTD Base Address"]
pub mod atl_ptd_base_address;
#[doc = "ISO_PTD_BASE_ADDRESS (rw) register accessor: an alias for `Reg<ISO_PTD_BASE_ADDRESS_SPEC>`"]
pub type ISO_PTD_BASE_ADDRESS = crate::Reg<iso_ptd_base_address::ISO_PTD_BASE_ADDRESS_SPEC>;
#[doc = "ISO PTD Base Address"]
pub mod iso_ptd_base_address;
#[doc = "INT_PTD_BASE_ADDRESS (rw) register accessor: an alias for `Reg<INT_PTD_BASE_ADDRESS_SPEC>`"]
pub type INT_PTD_BASE_ADDRESS = crate::Reg<int_ptd_base_address::INT_PTD_BASE_ADDRESS_SPEC>;
#[doc = "INT PTD Base Address"]
pub mod int_ptd_base_address;
#[doc = "DATA_PAYLOAD_BASE_ADDRESS (rw) register accessor: an alias for `Reg<DATA_PAYLOAD_BASE_ADDRESS_SPEC>`"]
pub type DATA_PAYLOAD_BASE_ADDRESS =
    crate::Reg<data_payload_base_address::DATA_PAYLOAD_BASE_ADDRESS_SPEC>;
#[doc = "DATA PAYLOAD Base Address"]
pub mod data_payload_base_address;
#[doc = "USBCMD (rw) register accessor: an alias for `Reg<USBCMD_SPEC>`"]
pub type USBCMD = crate::Reg<usbcmd::USBCMD_SPEC>;
#[doc = "USB Command"]
pub mod usbcmd;
#[doc = "USBSTS (rw) register accessor: an alias for `Reg<USBSTS_SPEC>`"]
pub type USBSTS = crate::Reg<usbsts::USBSTS_SPEC>;
#[doc = "USB Interrupt Status"]
pub mod usbsts;
#[doc = "USBINTR (rw) register accessor: an alias for `Reg<USBINTR_SPEC>`"]
pub type USBINTR = crate::Reg<usbintr::USBINTR_SPEC>;
#[doc = "USB Interrupt Status"]
pub mod usbintr;
#[doc = "PORTSC1 (rw) register accessor: an alias for `Reg<PORTSC1_SPEC>`"]
pub type PORTSC1 = crate::Reg<portsc1::PORTSC1_SPEC>;
#[doc = "Port Status and Control"]
pub mod portsc1;
#[doc = "ATL_DONE (rw) register accessor: an alias for `Reg<ATL_DONE_SPEC>`"]
pub type ATL_DONE = crate::Reg<atl_done::ATL_DONE_SPEC>;
#[doc = "ATL PTD Done Map"]
pub mod atl_done;
#[doc = "ATL_SKIP (rw) register accessor: an alias for `Reg<ATL_SKIP_SPEC>`"]
pub type ATL_SKIP = crate::Reg<atl_skip::ATL_SKIP_SPEC>;
#[doc = "ATL PTD Skip Map"]
pub mod atl_skip;
#[doc = "ISO_DONE (rw) register accessor: an alias for `Reg<ISO_DONE_SPEC>`"]
pub type ISO_DONE = crate::Reg<iso_done::ISO_DONE_SPEC>;
#[doc = "ISO PTD Done Map"]
pub mod iso_done;
#[doc = "ISO_SKIP (rw) register accessor: an alias for `Reg<ISO_SKIP_SPEC>`"]
pub type ISO_SKIP = crate::Reg<iso_skip::ISO_SKIP_SPEC>;
#[doc = "ISO PTD Skip Map"]
pub mod iso_skip;
#[doc = "INT_DONE (rw) register accessor: an alias for `Reg<INT_DONE_SPEC>`"]
pub type INT_DONE = crate::Reg<int_done::INT_DONE_SPEC>;
#[doc = "INT PTD Done Map"]
pub mod int_done;
#[doc = "INT_SKIP (rw) register accessor: an alias for `Reg<INT_SKIP_SPEC>`"]
pub type INT_SKIP = crate::Reg<int_skip::INT_SKIP_SPEC>;
#[doc = "INT PTD Skip Map"]
pub mod int_skip;
#[doc = "LAST_PTD (rw) register accessor: an alias for `Reg<LAST_PTD_SPEC>`"]
pub type LAST_PTD = crate::Reg<last_ptd::LAST_PTD_SPEC>;
#[doc = "Last PTD in use"]
pub mod last_ptd;
#[doc = "PORT_MODE (rw) register accessor: an alias for `Reg<PORT_MODE_SPEC>`"]
pub type PORT_MODE = crate::Reg<port_mode::PORT_MODE_SPEC>;
#[doc = "Port Mode"]
pub mod port_mode;
