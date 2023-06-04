#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Master Configuration Register"]
    pub mconfig: MCONFIG,
    #[doc = "0x04 - Slave Configuration Register"]
    pub sconfig: SCONFIG,
    #[doc = "0x08 - Slave Status Register"]
    pub sstatus: SSTATUS,
    #[doc = "0x0c - Slave Control Register"]
    pub sctrl: SCTRL,
    #[doc = "0x10 - Slave Interrupt Set Register"]
    pub sintset: SINTSET,
    #[doc = "0x14 - Slave Interrupt Clear Register"]
    pub sintclr: SINTCLR,
    #[doc = "0x18 - Slave Interrupt Mask Register"]
    pub sintmasked: SINTMASKED,
    #[doc = "0x1c - Slave Errors and Warnings Register"]
    pub serrwarn: SERRWARN,
    #[doc = "0x20 - Slave DMA Control Register"]
    pub sdmactrl: SDMACTRL,
    _reserved9: [u8; 0x08],
    #[doc = "0x2c - Slave Data Control Register"]
    pub sdatactrl: SDATACTRL,
    #[doc = "0x30 - Slave Write Data Byte Register"]
    pub swdatab: SWDATAB,
    #[doc = "0x34 - Slave Write Data Byte End"]
    pub swdatabe: SWDATABE,
    #[doc = "0x38 - Slave Write Data Half-word Register"]
    pub swdatah: SWDATAH,
    #[doc = "0x3c - Slave Write Data Half-word End Register"]
    pub swdatahe: SWDATAHE,
    #[doc = "0x40 - Slave Read Data Byte Register"]
    pub srdatab: SRDATAB,
    _reserved15: [u8; 0x04],
    #[doc = "0x48 - Slave Read Data Half-word Register"]
    pub srdatah: SRDATAH,
    _reserved16: [u8; 0x14],
    #[doc = "0x60 - Slave Capabilities Register"]
    pub scapabilities: SCAPABILITIES,
    #[doc = "0x64 - Slave Dynamic Address Register"]
    pub sdynaddr: SDYNADDR,
    #[doc = "0x68 - Slave Maximum Limits Register"]
    pub smaxlimits: SMAXLIMITS,
    #[doc = "0x6c - Slave ID Part Number Register"]
    pub sidpartno: SIDPARTNO,
    #[doc = "0x70 - Slave ID Extension Register"]
    pub sidext: SIDEXT,
    #[doc = "0x74 - Slave Vendor ID Register"]
    pub svendorid: SVENDORID,
    #[doc = "0x78 - Slave Time Control Clock Register"]
    pub stcclock: STCCLOCK,
    #[doc = "0x7c - Slave Message-Mapped Address Register"]
    pub smsgmapaddr: SMSGMAPADDR,
    _reserved24: [u8; 0x04],
    #[doc = "0x84 - Master Main Control Register"]
    pub mctrl: MCTRL,
    #[doc = "0x88 - Master Status Register"]
    pub mstatus: MSTATUS,
    #[doc = "0x8c - Master In-band Interrupt Registry and Rules Register"]
    pub mibirules: MIBIRULES,
    #[doc = "0x90 - Master Interrupt Set Register"]
    pub mintset: MINTSET,
    #[doc = "0x94 - Master Interrupt Clear Register"]
    pub mintclr: MINTCLR,
    #[doc = "0x98 - Master Interrupt Mask Register"]
    pub mintmasked: MINTMASKED,
    #[doc = "0x9c - Master Errors and Warnings Register"]
    pub merrwarn: MERRWARN,
    #[doc = "0xa0 - Master DMA Control Register"]
    pub mdmactrl: MDMACTRL,
    _reserved32: [u8; 0x08],
    #[doc = "0xac - Master Data Control Register"]
    pub mdatactrl: MDATACTRL,
    #[doc = "0xb0 - Master Write Data Byte Register"]
    pub mwdatab: MWDATAB,
    #[doc = "0xb4 - Master Write Data Byte End Register"]
    pub mwdatabe: MWDATABE,
    #[doc = "0xb8 - Master Write Data Half-word Register"]
    pub mwdatah: MWDATAH,
    #[doc = "0xbc - Master Write Data Byte End Register"]
    pub mwdatahe: MWDATAHE,
    #[doc = "0xc0 - Master Read Data Byte Register"]
    pub mrdatab: MRDATAB,
    _reserved38: [u8; 0x04],
    #[doc = "0xc8 - Master Read Data Half-word Register"]
    pub mrdatah: MRDATAH,
    #[doc = "0xcc - Write Byte Data 1 (to bus)"]
    pub mwdatab1: MWDATAB1,
    _reserved_40_mwmsg_sdr_mwmsg_sdr: [u8; 0x04],
    #[doc = "0xd4 - Master Read Message in SDR mode"]
    pub mrmsg_sdr: MRMSG_SDR,
    _reserved_42_mwmsg_ddr_mwmsg_ddr: [u8; 0x04],
    #[doc = "0xdc - Master Read Message in DDR mode"]
    pub mrmsg_ddr: MRMSG_DDR,
    _reserved44: [u8; 0x04],
    #[doc = "0xe4 - Master Dynamic Address Register"]
    pub mdynaddr: MDYNADDR,
    _reserved45: [u8; 0x0f14],
    #[doc = "0xffc - Slave Module ID"]
    pub sid: SID,
}
impl RegisterBlock {
    #[doc = "0xd0 - Master Write Message Data in SDR mode"]
    #[inline(always)]
    pub const fn mwmsg_sdr_mwmsg_sdr_data(&self) -> &MWMSG_SDR_MWMSG_SDR_DATA {
        unsafe { &*(self as *const Self).cast::<u8>().add(208usize).cast() }
    }
    #[doc = "0xd0 - Master Write Message in SDR mode"]
    #[inline(always)]
    pub const fn mwmsg_sdr_mwmsg_sdr_control(&self) -> &MWMSG_SDR_MWMSG_SDR_CONTROL {
        unsafe { &*(self as *const Self).cast::<u8>().add(208usize).cast() }
    }
    #[doc = "0xd8 - Master Write Message Data in DDR mode"]
    #[inline(always)]
    pub const fn mwmsg_ddr_mwmsg_ddr_data(&self) -> &MWMSG_DDR_MWMSG_DDR_DATA {
        unsafe { &*(self as *const Self).cast::<u8>().add(216usize).cast() }
    }
    #[doc = "0xd8 - Master Write Message in DDR mode"]
    #[inline(always)]
    pub const fn mwmsg_ddr_mwmsg_ddr_control(&self) -> &MWMSG_DDR_MWMSG_DDR_CONTROL {
        unsafe { &*(self as *const Self).cast::<u8>().add(216usize).cast() }
    }
}
#[doc = "MCONFIG (rw) register accessor: an alias for `Reg<MCONFIG_SPEC>`"]
pub type MCONFIG = crate::Reg<mconfig::MCONFIG_SPEC>;
#[doc = "Master Configuration Register"]
pub mod mconfig;
#[doc = "SCONFIG (rw) register accessor: an alias for `Reg<SCONFIG_SPEC>`"]
pub type SCONFIG = crate::Reg<sconfig::SCONFIG_SPEC>;
#[doc = "Slave Configuration Register"]
pub mod sconfig;
#[doc = "SSTATUS (rw) register accessor: an alias for `Reg<SSTATUS_SPEC>`"]
pub type SSTATUS = crate::Reg<sstatus::SSTATUS_SPEC>;
#[doc = "Slave Status Register"]
pub mod sstatus;
#[doc = "SCTRL (rw) register accessor: an alias for `Reg<SCTRL_SPEC>`"]
pub type SCTRL = crate::Reg<sctrl::SCTRL_SPEC>;
#[doc = "Slave Control Register"]
pub mod sctrl;
#[doc = "SINTSET (rw) register accessor: an alias for `Reg<SINTSET_SPEC>`"]
pub type SINTSET = crate::Reg<sintset::SINTSET_SPEC>;
#[doc = "Slave Interrupt Set Register"]
pub mod sintset;
#[doc = "SINTCLR (rw) register accessor: an alias for `Reg<SINTCLR_SPEC>`"]
pub type SINTCLR = crate::Reg<sintclr::SINTCLR_SPEC>;
#[doc = "Slave Interrupt Clear Register"]
pub mod sintclr;
#[doc = "SINTMASKED (r) register accessor: an alias for `Reg<SINTMASKED_SPEC>`"]
pub type SINTMASKED = crate::Reg<sintmasked::SINTMASKED_SPEC>;
#[doc = "Slave Interrupt Mask Register"]
pub mod sintmasked;
#[doc = "SERRWARN (rw) register accessor: an alias for `Reg<SERRWARN_SPEC>`"]
pub type SERRWARN = crate::Reg<serrwarn::SERRWARN_SPEC>;
#[doc = "Slave Errors and Warnings Register"]
pub mod serrwarn;
#[doc = "SDMACTRL (rw) register accessor: an alias for `Reg<SDMACTRL_SPEC>`"]
pub type SDMACTRL = crate::Reg<sdmactrl::SDMACTRL_SPEC>;
#[doc = "Slave DMA Control Register"]
pub mod sdmactrl;
#[doc = "SDATACTRL (rw) register accessor: an alias for `Reg<SDATACTRL_SPEC>`"]
pub type SDATACTRL = crate::Reg<sdatactrl::SDATACTRL_SPEC>;
#[doc = "Slave Data Control Register"]
pub mod sdatactrl;
#[doc = "SWDATAB (w) register accessor: an alias for `Reg<SWDATAB_SPEC>`"]
pub type SWDATAB = crate::Reg<swdatab::SWDATAB_SPEC>;
#[doc = "Slave Write Data Byte Register"]
pub mod swdatab;
#[doc = "SWDATABE (w) register accessor: an alias for `Reg<SWDATABE_SPEC>`"]
pub type SWDATABE = crate::Reg<swdatabe::SWDATABE_SPEC>;
#[doc = "Slave Write Data Byte End"]
pub mod swdatabe;
#[doc = "SWDATAH (w) register accessor: an alias for `Reg<SWDATAH_SPEC>`"]
pub type SWDATAH = crate::Reg<swdatah::SWDATAH_SPEC>;
#[doc = "Slave Write Data Half-word Register"]
pub mod swdatah;
#[doc = "SWDATAHE (w) register accessor: an alias for `Reg<SWDATAHE_SPEC>`"]
pub type SWDATAHE = crate::Reg<swdatahe::SWDATAHE_SPEC>;
#[doc = "Slave Write Data Half-word End Register"]
pub mod swdatahe;
#[doc = "SRDATAB (r) register accessor: an alias for `Reg<SRDATAB_SPEC>`"]
pub type SRDATAB = crate::Reg<srdatab::SRDATAB_SPEC>;
#[doc = "Slave Read Data Byte Register"]
pub mod srdatab;
#[doc = "SRDATAH (r) register accessor: an alias for `Reg<SRDATAH_SPEC>`"]
pub type SRDATAH = crate::Reg<srdatah::SRDATAH_SPEC>;
#[doc = "Slave Read Data Half-word Register"]
pub mod srdatah;
#[doc = "SCAPABILITIES (r) register accessor: an alias for `Reg<SCAPABILITIES_SPEC>`"]
pub type SCAPABILITIES = crate::Reg<scapabilities::SCAPABILITIES_SPEC>;
#[doc = "Slave Capabilities Register"]
pub mod scapabilities;
#[doc = "SDYNADDR (rw) register accessor: an alias for `Reg<SDYNADDR_SPEC>`"]
pub type SDYNADDR = crate::Reg<sdynaddr::SDYNADDR_SPEC>;
#[doc = "Slave Dynamic Address Register"]
pub mod sdynaddr;
#[doc = "SMAXLIMITS (rw) register accessor: an alias for `Reg<SMAXLIMITS_SPEC>`"]
pub type SMAXLIMITS = crate::Reg<smaxlimits::SMAXLIMITS_SPEC>;
#[doc = "Slave Maximum Limits Register"]
pub mod smaxlimits;
#[doc = "SIDPARTNO (rw) register accessor: an alias for `Reg<SIDPARTNO_SPEC>`"]
pub type SIDPARTNO = crate::Reg<sidpartno::SIDPARTNO_SPEC>;
#[doc = "Slave ID Part Number Register"]
pub mod sidpartno;
#[doc = "SIDEXT (rw) register accessor: an alias for `Reg<SIDEXT_SPEC>`"]
pub type SIDEXT = crate::Reg<sidext::SIDEXT_SPEC>;
#[doc = "Slave ID Extension Register"]
pub mod sidext;
#[doc = "SVENDORID (rw) register accessor: an alias for `Reg<SVENDORID_SPEC>`"]
pub type SVENDORID = crate::Reg<svendorid::SVENDORID_SPEC>;
#[doc = "Slave Vendor ID Register"]
pub mod svendorid;
#[doc = "STCCLOCK (rw) register accessor: an alias for `Reg<STCCLOCK_SPEC>`"]
pub type STCCLOCK = crate::Reg<stcclock::STCCLOCK_SPEC>;
#[doc = "Slave Time Control Clock Register"]
pub mod stcclock;
#[doc = "SMSGMAPADDR (r) register accessor: an alias for `Reg<SMSGMAPADDR_SPEC>`"]
pub type SMSGMAPADDR = crate::Reg<smsgmapaddr::SMSGMAPADDR_SPEC>;
#[doc = "Slave Message-Mapped Address Register"]
pub mod smsgmapaddr;
#[doc = "MCTRL (rw) register accessor: an alias for `Reg<MCTRL_SPEC>`"]
pub type MCTRL = crate::Reg<mctrl::MCTRL_SPEC>;
#[doc = "Master Main Control Register"]
pub mod mctrl;
#[doc = "MSTATUS (rw) register accessor: an alias for `Reg<MSTATUS_SPEC>`"]
pub type MSTATUS = crate::Reg<mstatus::MSTATUS_SPEC>;
#[doc = "Master Status Register"]
pub mod mstatus;
#[doc = "MIBIRULES (rw) register accessor: an alias for `Reg<MIBIRULES_SPEC>`"]
pub type MIBIRULES = crate::Reg<mibirules::MIBIRULES_SPEC>;
#[doc = "Master In-band Interrupt Registry and Rules Register"]
pub mod mibirules;
#[doc = "MINTSET (rw) register accessor: an alias for `Reg<MINTSET_SPEC>`"]
pub type MINTSET = crate::Reg<mintset::MINTSET_SPEC>;
#[doc = "Master Interrupt Set Register"]
pub mod mintset;
#[doc = "MINTCLR (w) register accessor: an alias for `Reg<MINTCLR_SPEC>`"]
pub type MINTCLR = crate::Reg<mintclr::MINTCLR_SPEC>;
#[doc = "Master Interrupt Clear Register"]
pub mod mintclr;
#[doc = "MINTMASKED (r) register accessor: an alias for `Reg<MINTMASKED_SPEC>`"]
pub type MINTMASKED = crate::Reg<mintmasked::MINTMASKED_SPEC>;
#[doc = "Master Interrupt Mask Register"]
pub mod mintmasked;
#[doc = "MERRWARN (rw) register accessor: an alias for `Reg<MERRWARN_SPEC>`"]
pub type MERRWARN = crate::Reg<merrwarn::MERRWARN_SPEC>;
#[doc = "Master Errors and Warnings Register"]
pub mod merrwarn;
#[doc = "MDMACTRL (rw) register accessor: an alias for `Reg<MDMACTRL_SPEC>`"]
pub type MDMACTRL = crate::Reg<mdmactrl::MDMACTRL_SPEC>;
#[doc = "Master DMA Control Register"]
pub mod mdmactrl;
#[doc = "MDATACTRL (rw) register accessor: an alias for `Reg<MDATACTRL_SPEC>`"]
pub type MDATACTRL = crate::Reg<mdatactrl::MDATACTRL_SPEC>;
#[doc = "Master Data Control Register"]
pub mod mdatactrl;
#[doc = "MWDATAB (w) register accessor: an alias for `Reg<MWDATAB_SPEC>`"]
pub type MWDATAB = crate::Reg<mwdatab::MWDATAB_SPEC>;
#[doc = "Master Write Data Byte Register"]
pub mod mwdatab;
#[doc = "MWDATABE (w) register accessor: an alias for `Reg<MWDATABE_SPEC>`"]
pub type MWDATABE = crate::Reg<mwdatabe::MWDATABE_SPEC>;
#[doc = "Master Write Data Byte End Register"]
pub mod mwdatabe;
#[doc = "MWDATAH (w) register accessor: an alias for `Reg<MWDATAH_SPEC>`"]
pub type MWDATAH = crate::Reg<mwdatah::MWDATAH_SPEC>;
#[doc = "Master Write Data Half-word Register"]
pub mod mwdatah;
#[doc = "MWDATAHE (w) register accessor: an alias for `Reg<MWDATAHE_SPEC>`"]
pub type MWDATAHE = crate::Reg<mwdatahe::MWDATAHE_SPEC>;
#[doc = "Master Write Data Byte End Register"]
pub mod mwdatahe;
#[doc = "MRDATAB (r) register accessor: an alias for `Reg<MRDATAB_SPEC>`"]
pub type MRDATAB = crate::Reg<mrdatab::MRDATAB_SPEC>;
#[doc = "Master Read Data Byte Register"]
pub mod mrdatab;
#[doc = "MRDATAH (r) register accessor: an alias for `Reg<MRDATAH_SPEC>`"]
pub type MRDATAH = crate::Reg<mrdatah::MRDATAH_SPEC>;
#[doc = "Master Read Data Half-word Register"]
pub mod mrdatah;
#[doc = "MWDATAB1 (w) register accessor: an alias for `Reg<MWDATAB1_SPEC>`"]
pub type MWDATAB1 = crate::Reg<mwdatab1::MWDATAB1_SPEC>;
#[doc = "Write Byte Data 1 (to bus)"]
pub mod mwdatab1;
#[doc = "MWMSG_SDR_MWMSG_SDR_CONTROL (w) register accessor: an alias for `Reg<MWMSG_SDR_MWMSG_SDR_CONTROL_SPEC>`"]
pub type MWMSG_SDR_MWMSG_SDR_CONTROL =
    crate::Reg<mwmsg_sdr_mwmsg_sdr_control::MWMSG_SDR_MWMSG_SDR_CONTROL_SPEC>;
#[doc = "Master Write Message in SDR mode"]
pub mod mwmsg_sdr_mwmsg_sdr_control;
#[doc = "MWMSG_SDR_MWMSG_SDR_DATA (w) register accessor: an alias for `Reg<MWMSG_SDR_MWMSG_SDR_DATA_SPEC>`"]
pub type MWMSG_SDR_MWMSG_SDR_DATA =
    crate::Reg<mwmsg_sdr_mwmsg_sdr_data::MWMSG_SDR_MWMSG_SDR_DATA_SPEC>;
#[doc = "Master Write Message Data in SDR mode"]
pub mod mwmsg_sdr_mwmsg_sdr_data;
#[doc = "MRMSG_SDR (r) register accessor: an alias for `Reg<MRMSG_SDR_SPEC>`"]
pub type MRMSG_SDR = crate::Reg<mrmsg_sdr::MRMSG_SDR_SPEC>;
#[doc = "Master Read Message in SDR mode"]
pub mod mrmsg_sdr;
#[doc = "MWMSG_DDR_MWMSG_DDR_CONTROL (w) register accessor: an alias for `Reg<MWMSG_DDR_MWMSG_DDR_CONTROL_SPEC>`"]
pub type MWMSG_DDR_MWMSG_DDR_CONTROL =
    crate::Reg<mwmsg_ddr_mwmsg_ddr_control::MWMSG_DDR_MWMSG_DDR_CONTROL_SPEC>;
#[doc = "Master Write Message in DDR mode"]
pub mod mwmsg_ddr_mwmsg_ddr_control;
#[doc = "MWMSG_DDR_MWMSG_DDR_DATA (w) register accessor: an alias for `Reg<MWMSG_DDR_MWMSG_DDR_DATA_SPEC>`"]
pub type MWMSG_DDR_MWMSG_DDR_DATA =
    crate::Reg<mwmsg_ddr_mwmsg_ddr_data::MWMSG_DDR_MWMSG_DDR_DATA_SPEC>;
#[doc = "Master Write Message Data in DDR mode"]
pub mod mwmsg_ddr_mwmsg_ddr_data;
#[doc = "MRMSG_DDR (rw) register accessor: an alias for `Reg<MRMSG_DDR_SPEC>`"]
pub type MRMSG_DDR = crate::Reg<mrmsg_ddr::MRMSG_DDR_SPEC>;
#[doc = "Master Read Message in DDR mode"]
pub mod mrmsg_ddr;
#[doc = "MDYNADDR (rw) register accessor: an alias for `Reg<MDYNADDR_SPEC>`"]
pub type MDYNADDR = crate::Reg<mdynaddr::MDYNADDR_SPEC>;
#[doc = "Master Dynamic Address Register"]
pub mod mdynaddr;
#[doc = "SID (r) register accessor: an alias for `Reg<SID_SPEC>`"]
pub type SID = crate::Reg<sid::SID_SPEC>;
#[doc = "Slave Module ID"]
pub mod sid;
