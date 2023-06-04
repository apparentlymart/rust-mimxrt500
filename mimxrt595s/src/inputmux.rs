#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00..0x1c - SCT Peripheral Input multiplexer index"]
    pub sct0_in_sel: [SCT0_IN_SEL; 7],
    _reserved1: [u8; 0xe4],
    #[doc = "0x100..0x120 - GPIO Pin Input Multiplexer index"]
    pub pint_sel: [PINT_SEL; 8],
    _reserved2: [u8; 0x20],
    #[doc = "0x140..0x1ac - Fusion DSP Interrupt Input Multiplexer"]
    pub dsp_int_sel: [DSP_INT_SEL; 27],
    _reserved3: [u8; 0x54],
    #[doc = "0x200..0x294 - DMAC0 Input Trigger Select"]
    pub dmac0_itrig_sel: [DMAC0_ITRIG_SEL; 37],
    _reserved4: [u8; 0x6c],
    #[doc = "0x300..0x310 - DMAC0 Output Trigger Select"]
    pub dmac0_otrig_sel: [DMAC0_OTRIG_SEL; 4],
    _reserved5: [u8; 0x10],
    #[doc = "0x320 - DMAC0 Channel mux select 0"]
    pub dmac0_chmux_sel0: DMAC0_CHMUX_SEL0,
    #[doc = "0x324 - DMAC0 Channel mux select 1"]
    pub dmac0_chmux_sel1: DMAC0_CHMUX_SEL1,
    #[doc = "0x328 - DMAC0 Channel mux select 2"]
    pub dmac0_chmux_sel2: DMAC0_CHMUX_SEL2,
    #[doc = "0x32c - DMAC0 Channel mux select 3"]
    pub dmac0_chmux_sel3: DMAC0_CHMUX_SEL3,
    #[doc = "0x330 - DMAC0 Channel mux select 4"]
    pub dmac0_chmux_sel4: DMAC0_CHMUX_SEL4,
    #[doc = "0x334 - DMAC0 Channel mux select 5"]
    pub dmac0_chmux_sel5: DMAC0_CHMUX_SEL5,
    #[doc = "0x338 - DMAC0 Channel mux select 6"]
    pub dmac0_chmux_sel6: DMAC0_CHMUX_SEL6,
    #[doc = "0x33c - DMAC0 Channel mux select 7"]
    pub dmac0_chmux_sel7: DMAC0_CHMUX_SEL7,
    #[doc = "0x340 - DMAC0 Channel mux select 8"]
    pub dmac0_chmux_sel8: DMAC0_CHMUX_SEL8,
    #[doc = "0x344 - DMAC0 Channel mux select 9"]
    pub dmac0_chmux_sel9: DMAC0_CHMUX_SEL9,
    #[doc = "0x348 - DMAC0 Channel mux select 10"]
    pub dmac0_chmux_sel10: DMAC0_CHMUX_SEL10,
    #[doc = "0x34c - DMAC0 Channel mux select 11"]
    pub dmac0_chmux_sel11: DMAC0_CHMUX_SEL11,
    #[doc = "0x350 - DMAC0 Channel mux select 12"]
    pub dmac0_chmux_sel12: DMAC0_CHMUX_SEL12,
    #[doc = "0x354 - DMAC0 Channel mux select 13"]
    pub dmac0_chmux_sel13: DMAC0_CHMUX_SEL13,
    #[doc = "0x358 - DMAC0 Channel mux select 14"]
    pub dmac0_chmux_sel14: DMAC0_CHMUX_SEL14,
    #[doc = "0x35c - DMAC0 Channel mux select 15"]
    pub dmac0_chmux_sel15: DMAC0_CHMUX_SEL15,
    _reserved21: [u8; 0xa0],
    #[doc = "0x400..0x494 - DMAC1 Input Trigger Select"]
    pub dmac1_itrig_sel: [DMAC1_ITRIG_SEL; 37],
    _reserved22: [u8; 0x6c],
    #[doc = "0x500..0x510 - DMAC1 Output Trigger Select"]
    pub dmac1_otrig_sel: [DMAC1_OTRIG_SEL; 4],
    _reserved23: [u8; 0x10],
    #[doc = "0x520 - DMAC1 Channel mux select 0"]
    pub dmac1_chmux_sel0: DMAC1_CHMUX_SEL0,
    #[doc = "0x524 - DMAC1 Channel mux select 1"]
    pub dmac1_chmux_sel1: DMAC1_CHMUX_SEL1,
    #[doc = "0x528 - DMAC1 Channel mux select 2"]
    pub dmac1_chmux_sel2: DMAC1_CHMUX_SEL2,
    #[doc = "0x52c - DMAC1 Channel mux select 3"]
    pub dmac1_chmux_sel3: DMAC1_CHMUX_SEL3,
    #[doc = "0x530 - DMAC1 Channel mux select 4"]
    pub dmac1_chmux_sel4: DMAC1_CHMUX_SEL4,
    #[doc = "0x534 - DMAC1 Channel mux select 5"]
    pub dmac1_chmux_sel5: DMAC1_CHMUX_SEL5,
    #[doc = "0x538 - DMAC1 Channel mux select 6"]
    pub dmac1_chmux_sel6: DMAC1_CHMUX_SEL6,
    #[doc = "0x53c - DMAC1 Channel mux select 7"]
    pub dmac1_chmux_sel7: DMAC1_CHMUX_SEL7,
    #[doc = "0x540 - DMAC1 Channel mux select 8"]
    pub dmac1_chmux_sel8: DMAC1_CHMUX_SEL8,
    #[doc = "0x544 - DMAC1 Channel mux select 9"]
    pub dmac1_chmux_sel9: DMAC1_CHMUX_SEL9,
    #[doc = "0x548 - DMAC1 Channel mux select 10"]
    pub dmac1_chmux_sel10: DMAC1_CHMUX_SEL10,
    #[doc = "0x54c - DMAC1 Channel mux select 11"]
    pub dmac1_chmux_sel11: DMAC1_CHMUX_SEL11,
    #[doc = "0x550 - DMAC1 Channel mux select 12"]
    pub dmac1_chmux_sel12: DMAC1_CHMUX_SEL12,
    #[doc = "0x554 - DMAC1 Channel mux select 13"]
    pub dmac1_chmux_sel13: DMAC1_CHMUX_SEL13,
    #[doc = "0x558 - DMAC1 Channel mux select 14"]
    pub dmac1_chmux_sel14: DMAC1_CHMUX_SEL14,
    #[doc = "0x55c - DMAC1 Channel mux select 15"]
    pub dmac1_chmux_sel15: DMAC1_CHMUX_SEL15,
    _reserved39: [u8; 0xa0],
    #[doc = "0x600..0x650 - no description available"]
    pub ct32bit_cap_sel: [CT32BIT_CAP_SEL; 5],
    _reserved40: [u8; 0xb0],
    #[doc = "0x700..0x708 - Frequency Measurement Input Channel Multiplexers"]
    pub fmeasure_ch_sel: [FMEASURE_CH_SEL; 2],
    _reserved41: [u8; 0x18],
    #[doc = "0x720..0x740 - SMART_DMA trigger channel select"]
    pub smart_dma_trig_ch_sel: [SMART_DMA_TRIG_CH_SEL; 8],
    #[doc = "0x740 - DMAC0 request enable 0"]
    pub dmac0_req_ena0: DMAC0_REQ_ENA0,
    #[doc = "0x744 - DMAC0 request enable 1"]
    pub dmac0_req_ena1: DMAC0_REQ_ENA1,
    #[doc = "0x748 - DMAC0 request enable set 0"]
    pub dmac0_req_ena0_set: DMAC0_REQ_ENA0_SET,
    #[doc = "0x74c - DMAC0 request enable set 1"]
    pub dmac0_req_ena1_set: DMAC0_REQ_ENA1_SET,
    #[doc = "0x750 - DMAC0 request enable clear 0"]
    pub dmac0_req_ena0_clr: DMAC0_REQ_ENA0_CLR,
    #[doc = "0x754 - DMAC0 request enable 1 clear"]
    pub dmac0_req_ena1_clr: DMAC0_REQ_ENA1_CLR,
    _reserved48: [u8; 0x08],
    #[doc = "0x760 - DMAC1 request enable 0"]
    pub dmac1_req_ena0: DMAC1_REQ_ENA0,
    #[doc = "0x764 - DMAC1 request enable 1"]
    pub dmac1_req_ena1: DMAC1_REQ_ENA1,
    #[doc = "0x768 - DMAC1 request enable set 0"]
    pub dmac1_req_ena0_set: DMAC1_REQ_ENA0_SET,
    #[doc = "0x76c - DMAC1 request enable set 1"]
    pub dmac1_req_ena1_set: DMAC1_REQ_ENA1_SET,
    #[doc = "0x770 - DMAC1 request enable clear 0"]
    pub dmac1_req_ena0_clr: DMAC1_REQ_ENA0_CLR,
    #[doc = "0x774 - DMAC1 request enable 1 clear"]
    pub dmac1_req_ena1_clr: DMAC1_REQ_ENA1_CLR,
    _reserved54: [u8; 0x08],
    #[doc = "0x780 - DMAC0 Input Trigger Enable 0"]
    pub dmac0_itrig_ena0: DMAC0_ITRIG_ENA0,
    _reserved55: [u8; 0x04],
    #[doc = "0x788 - DMAC0 Input Trigger Enable 0 Set"]
    pub dmac0_itrig_ena0_set: DMAC0_ITRIG_ENA0_SET,
    _reserved56: [u8; 0x04],
    #[doc = "0x790 - DMAC0 Input Trigger Enable 0 Clear"]
    pub dmac0_itrig_ena0_clr: DMAC0_ITRIG_ENA0_CLR,
    _reserved57: [u8; 0x0c],
    #[doc = "0x7a0 - DMAC1 Input Trigger Enable 0"]
    pub dmac1_itrig_ena0: DMAC1_ITRIG_ENA0,
    _reserved58: [u8; 0x04],
    #[doc = "0x7a8 - DMAC1 Input Trigger Enable 0 set"]
    pub dmac1_itrig_ena0_set: DMAC1_ITRIG_ENA0_SET,
    _reserved59: [u8; 0x04],
    #[doc = "0x7b0 - DMAC1 Input Trigger Enable 0 clear"]
    pub dmac1_itrig_ena0_clr: DMAC1_ITRIG_ENA0_CLR,
}
#[doc = "no description available"]
pub use self::ct32bit_cap_sel::CT32BIT_CAP_SEL;
#[doc = r"Cluster"]
#[doc = "no description available"]
pub mod ct32bit_cap_sel;
#[doc = "SCT0_IN_SEL (rw) register accessor: an alias for `Reg<SCT0_IN_SEL_SPEC>`"]
pub type SCT0_IN_SEL = crate::Reg<sct0_in_sel::SCT0_IN_SEL_SPEC>;
#[doc = "SCT Peripheral Input multiplexer index"]
pub mod sct0_in_sel;
#[doc = "PINT_SEL (rw) register accessor: an alias for `Reg<PINT_SEL_SPEC>`"]
pub type PINT_SEL = crate::Reg<pint_sel::PINT_SEL_SPEC>;
#[doc = "GPIO Pin Input Multiplexer index"]
pub mod pint_sel;
#[doc = "DSP_INT_SEL (rw) register accessor: an alias for `Reg<DSP_INT_SEL_SPEC>`"]
pub type DSP_INT_SEL = crate::Reg<dsp_int_sel::DSP_INT_SEL_SPEC>;
#[doc = "Fusion DSP Interrupt Input Multiplexer"]
pub mod dsp_int_sel;
#[doc = "DMAC0_ITRIG_SEL (rw) register accessor: an alias for `Reg<DMAC0_ITRIG_SEL_SPEC>`"]
pub type DMAC0_ITRIG_SEL = crate::Reg<dmac0_itrig_sel::DMAC0_ITRIG_SEL_SPEC>;
#[doc = "DMAC0 Input Trigger Select"]
pub mod dmac0_itrig_sel;
#[doc = "DMAC0_OTRIG_SEL (rw) register accessor: an alias for `Reg<DMAC0_OTRIG_SEL_SPEC>`"]
pub type DMAC0_OTRIG_SEL = crate::Reg<dmac0_otrig_sel::DMAC0_OTRIG_SEL_SPEC>;
#[doc = "DMAC0 Output Trigger Select"]
pub mod dmac0_otrig_sel;
#[doc = "DMAC0_CHMUX_SEL0 (rw) register accessor: an alias for `Reg<DMAC0_CHMUX_SEL0_SPEC>`"]
pub type DMAC0_CHMUX_SEL0 = crate::Reg<dmac0_chmux_sel0::DMAC0_CHMUX_SEL0_SPEC>;
#[doc = "DMAC0 Channel mux select 0"]
pub mod dmac0_chmux_sel0;
#[doc = "DMAC0_CHMUX_SEL1 (rw) register accessor: an alias for `Reg<DMAC0_CHMUX_SEL1_SPEC>`"]
pub type DMAC0_CHMUX_SEL1 = crate::Reg<dmac0_chmux_sel1::DMAC0_CHMUX_SEL1_SPEC>;
#[doc = "DMAC0 Channel mux select 1"]
pub mod dmac0_chmux_sel1;
#[doc = "DMAC0_CHMUX_SEL2 (rw) register accessor: an alias for `Reg<DMAC0_CHMUX_SEL2_SPEC>`"]
pub type DMAC0_CHMUX_SEL2 = crate::Reg<dmac0_chmux_sel2::DMAC0_CHMUX_SEL2_SPEC>;
#[doc = "DMAC0 Channel mux select 2"]
pub mod dmac0_chmux_sel2;
#[doc = "DMAC0_CHMUX_SEL3 (rw) register accessor: an alias for `Reg<DMAC0_CHMUX_SEL3_SPEC>`"]
pub type DMAC0_CHMUX_SEL3 = crate::Reg<dmac0_chmux_sel3::DMAC0_CHMUX_SEL3_SPEC>;
#[doc = "DMAC0 Channel mux select 3"]
pub mod dmac0_chmux_sel3;
#[doc = "DMAC0_CHMUX_SEL4 (rw) register accessor: an alias for `Reg<DMAC0_CHMUX_SEL4_SPEC>`"]
pub type DMAC0_CHMUX_SEL4 = crate::Reg<dmac0_chmux_sel4::DMAC0_CHMUX_SEL4_SPEC>;
#[doc = "DMAC0 Channel mux select 4"]
pub mod dmac0_chmux_sel4;
#[doc = "DMAC0_CHMUX_SEL5 (rw) register accessor: an alias for `Reg<DMAC0_CHMUX_SEL5_SPEC>`"]
pub type DMAC0_CHMUX_SEL5 = crate::Reg<dmac0_chmux_sel5::DMAC0_CHMUX_SEL5_SPEC>;
#[doc = "DMAC0 Channel mux select 5"]
pub mod dmac0_chmux_sel5;
#[doc = "DMAC0_CHMUX_SEL6 (rw) register accessor: an alias for `Reg<DMAC0_CHMUX_SEL6_SPEC>`"]
pub type DMAC0_CHMUX_SEL6 = crate::Reg<dmac0_chmux_sel6::DMAC0_CHMUX_SEL6_SPEC>;
#[doc = "DMAC0 Channel mux select 6"]
pub mod dmac0_chmux_sel6;
#[doc = "DMAC0_CHMUX_SEL7 (rw) register accessor: an alias for `Reg<DMAC0_CHMUX_SEL7_SPEC>`"]
pub type DMAC0_CHMUX_SEL7 = crate::Reg<dmac0_chmux_sel7::DMAC0_CHMUX_SEL7_SPEC>;
#[doc = "DMAC0 Channel mux select 7"]
pub mod dmac0_chmux_sel7;
#[doc = "DMAC0_CHMUX_SEL8 (rw) register accessor: an alias for `Reg<DMAC0_CHMUX_SEL8_SPEC>`"]
pub type DMAC0_CHMUX_SEL8 = crate::Reg<dmac0_chmux_sel8::DMAC0_CHMUX_SEL8_SPEC>;
#[doc = "DMAC0 Channel mux select 8"]
pub mod dmac0_chmux_sel8;
#[doc = "DMAC0_CHMUX_SEL9 (rw) register accessor: an alias for `Reg<DMAC0_CHMUX_SEL9_SPEC>`"]
pub type DMAC0_CHMUX_SEL9 = crate::Reg<dmac0_chmux_sel9::DMAC0_CHMUX_SEL9_SPEC>;
#[doc = "DMAC0 Channel mux select 9"]
pub mod dmac0_chmux_sel9;
#[doc = "DMAC0_CHMUX_SEL10 (rw) register accessor: an alias for `Reg<DMAC0_CHMUX_SEL10_SPEC>`"]
pub type DMAC0_CHMUX_SEL10 = crate::Reg<dmac0_chmux_sel10::DMAC0_CHMUX_SEL10_SPEC>;
#[doc = "DMAC0 Channel mux select 10"]
pub mod dmac0_chmux_sel10;
#[doc = "DMAC0_CHMUX_SEL11 (rw) register accessor: an alias for `Reg<DMAC0_CHMUX_SEL11_SPEC>`"]
pub type DMAC0_CHMUX_SEL11 = crate::Reg<dmac0_chmux_sel11::DMAC0_CHMUX_SEL11_SPEC>;
#[doc = "DMAC0 Channel mux select 11"]
pub mod dmac0_chmux_sel11;
#[doc = "DMAC0_CHMUX_SEL12 (rw) register accessor: an alias for `Reg<DMAC0_CHMUX_SEL12_SPEC>`"]
pub type DMAC0_CHMUX_SEL12 = crate::Reg<dmac0_chmux_sel12::DMAC0_CHMUX_SEL12_SPEC>;
#[doc = "DMAC0 Channel mux select 12"]
pub mod dmac0_chmux_sel12;
#[doc = "DMAC0_CHMUX_SEL13 (rw) register accessor: an alias for `Reg<DMAC0_CHMUX_SEL13_SPEC>`"]
pub type DMAC0_CHMUX_SEL13 = crate::Reg<dmac0_chmux_sel13::DMAC0_CHMUX_SEL13_SPEC>;
#[doc = "DMAC0 Channel mux select 13"]
pub mod dmac0_chmux_sel13;
#[doc = "DMAC0_CHMUX_SEL14 (rw) register accessor: an alias for `Reg<DMAC0_CHMUX_SEL14_SPEC>`"]
pub type DMAC0_CHMUX_SEL14 = crate::Reg<dmac0_chmux_sel14::DMAC0_CHMUX_SEL14_SPEC>;
#[doc = "DMAC0 Channel mux select 14"]
pub mod dmac0_chmux_sel14;
#[doc = "DMAC0_CHMUX_SEL15 (rw) register accessor: an alias for `Reg<DMAC0_CHMUX_SEL15_SPEC>`"]
pub type DMAC0_CHMUX_SEL15 = crate::Reg<dmac0_chmux_sel15::DMAC0_CHMUX_SEL15_SPEC>;
#[doc = "DMAC0 Channel mux select 15"]
pub mod dmac0_chmux_sel15;
#[doc = "DMAC1_ITRIG_SEL (rw) register accessor: an alias for `Reg<DMAC1_ITRIG_SEL_SPEC>`"]
pub type DMAC1_ITRIG_SEL = crate::Reg<dmac1_itrig_sel::DMAC1_ITRIG_SEL_SPEC>;
#[doc = "DMAC1 Input Trigger Select"]
pub mod dmac1_itrig_sel;
#[doc = "DMAC1_OTRIG_SEL (rw) register accessor: an alias for `Reg<DMAC1_OTRIG_SEL_SPEC>`"]
pub type DMAC1_OTRIG_SEL = crate::Reg<dmac1_otrig_sel::DMAC1_OTRIG_SEL_SPEC>;
#[doc = "DMAC1 Output Trigger Select"]
pub mod dmac1_otrig_sel;
#[doc = "DMAC1_CHMUX_SEL0 (rw) register accessor: an alias for `Reg<DMAC1_CHMUX_SEL0_SPEC>`"]
pub type DMAC1_CHMUX_SEL0 = crate::Reg<dmac1_chmux_sel0::DMAC1_CHMUX_SEL0_SPEC>;
#[doc = "DMAC1 Channel mux select 0"]
pub mod dmac1_chmux_sel0;
#[doc = "DMAC1_CHMUX_SEL1 (rw) register accessor: an alias for `Reg<DMAC1_CHMUX_SEL1_SPEC>`"]
pub type DMAC1_CHMUX_SEL1 = crate::Reg<dmac1_chmux_sel1::DMAC1_CHMUX_SEL1_SPEC>;
#[doc = "DMAC1 Channel mux select 1"]
pub mod dmac1_chmux_sel1;
#[doc = "DMAC1_CHMUX_SEL2 (rw) register accessor: an alias for `Reg<DMAC1_CHMUX_SEL2_SPEC>`"]
pub type DMAC1_CHMUX_SEL2 = crate::Reg<dmac1_chmux_sel2::DMAC1_CHMUX_SEL2_SPEC>;
#[doc = "DMAC1 Channel mux select 2"]
pub mod dmac1_chmux_sel2;
#[doc = "DMAC1_CHMUX_SEL3 (rw) register accessor: an alias for `Reg<DMAC1_CHMUX_SEL3_SPEC>`"]
pub type DMAC1_CHMUX_SEL3 = crate::Reg<dmac1_chmux_sel3::DMAC1_CHMUX_SEL3_SPEC>;
#[doc = "DMAC1 Channel mux select 3"]
pub mod dmac1_chmux_sel3;
#[doc = "DMAC1_CHMUX_SEL4 (rw) register accessor: an alias for `Reg<DMAC1_CHMUX_SEL4_SPEC>`"]
pub type DMAC1_CHMUX_SEL4 = crate::Reg<dmac1_chmux_sel4::DMAC1_CHMUX_SEL4_SPEC>;
#[doc = "DMAC1 Channel mux select 4"]
pub mod dmac1_chmux_sel4;
#[doc = "DMAC1_CHMUX_SEL5 (rw) register accessor: an alias for `Reg<DMAC1_CHMUX_SEL5_SPEC>`"]
pub type DMAC1_CHMUX_SEL5 = crate::Reg<dmac1_chmux_sel5::DMAC1_CHMUX_SEL5_SPEC>;
#[doc = "DMAC1 Channel mux select 5"]
pub mod dmac1_chmux_sel5;
#[doc = "DMAC1_CHMUX_SEL6 (rw) register accessor: an alias for `Reg<DMAC1_CHMUX_SEL6_SPEC>`"]
pub type DMAC1_CHMUX_SEL6 = crate::Reg<dmac1_chmux_sel6::DMAC1_CHMUX_SEL6_SPEC>;
#[doc = "DMAC1 Channel mux select 6"]
pub mod dmac1_chmux_sel6;
#[doc = "DMAC1_CHMUX_SEL7 (rw) register accessor: an alias for `Reg<DMAC1_CHMUX_SEL7_SPEC>`"]
pub type DMAC1_CHMUX_SEL7 = crate::Reg<dmac1_chmux_sel7::DMAC1_CHMUX_SEL7_SPEC>;
#[doc = "DMAC1 Channel mux select 7"]
pub mod dmac1_chmux_sel7;
#[doc = "DMAC1_CHMUX_SEL8 (rw) register accessor: an alias for `Reg<DMAC1_CHMUX_SEL8_SPEC>`"]
pub type DMAC1_CHMUX_SEL8 = crate::Reg<dmac1_chmux_sel8::DMAC1_CHMUX_SEL8_SPEC>;
#[doc = "DMAC1 Channel mux select 8"]
pub mod dmac1_chmux_sel8;
#[doc = "DMAC1_CHMUX_SEL9 (rw) register accessor: an alias for `Reg<DMAC1_CHMUX_SEL9_SPEC>`"]
pub type DMAC1_CHMUX_SEL9 = crate::Reg<dmac1_chmux_sel9::DMAC1_CHMUX_SEL9_SPEC>;
#[doc = "DMAC1 Channel mux select 9"]
pub mod dmac1_chmux_sel9;
#[doc = "DMAC1_CHMUX_SEL10 (rw) register accessor: an alias for `Reg<DMAC1_CHMUX_SEL10_SPEC>`"]
pub type DMAC1_CHMUX_SEL10 = crate::Reg<dmac1_chmux_sel10::DMAC1_CHMUX_SEL10_SPEC>;
#[doc = "DMAC1 Channel mux select 10"]
pub mod dmac1_chmux_sel10;
#[doc = "DMAC1_CHMUX_SEL11 (rw) register accessor: an alias for `Reg<DMAC1_CHMUX_SEL11_SPEC>`"]
pub type DMAC1_CHMUX_SEL11 = crate::Reg<dmac1_chmux_sel11::DMAC1_CHMUX_SEL11_SPEC>;
#[doc = "DMAC1 Channel mux select 11"]
pub mod dmac1_chmux_sel11;
#[doc = "DMAC1_CHMUX_SEL12 (rw) register accessor: an alias for `Reg<DMAC1_CHMUX_SEL12_SPEC>`"]
pub type DMAC1_CHMUX_SEL12 = crate::Reg<dmac1_chmux_sel12::DMAC1_CHMUX_SEL12_SPEC>;
#[doc = "DMAC1 Channel mux select 12"]
pub mod dmac1_chmux_sel12;
#[doc = "DMAC1_CHMUX_SEL13 (rw) register accessor: an alias for `Reg<DMAC1_CHMUX_SEL13_SPEC>`"]
pub type DMAC1_CHMUX_SEL13 = crate::Reg<dmac1_chmux_sel13::DMAC1_CHMUX_SEL13_SPEC>;
#[doc = "DMAC1 Channel mux select 13"]
pub mod dmac1_chmux_sel13;
#[doc = "DMAC1_CHMUX_SEL14 (rw) register accessor: an alias for `Reg<DMAC1_CHMUX_SEL14_SPEC>`"]
pub type DMAC1_CHMUX_SEL14 = crate::Reg<dmac1_chmux_sel14::DMAC1_CHMUX_SEL14_SPEC>;
#[doc = "DMAC1 Channel mux select 14"]
pub mod dmac1_chmux_sel14;
#[doc = "DMAC1_CHMUX_SEL15 (rw) register accessor: an alias for `Reg<DMAC1_CHMUX_SEL15_SPEC>`"]
pub type DMAC1_CHMUX_SEL15 = crate::Reg<dmac1_chmux_sel15::DMAC1_CHMUX_SEL15_SPEC>;
#[doc = "DMAC1 Channel mux select 15"]
pub mod dmac1_chmux_sel15;
#[doc = "FMEASURE_CH_SEL (rw) register accessor: an alias for `Reg<FMEASURE_CH_SEL_SPEC>`"]
pub type FMEASURE_CH_SEL = crate::Reg<fmeasure_ch_sel::FMEASURE_CH_SEL_SPEC>;
#[doc = "Frequency Measurement Input Channel Multiplexers"]
pub mod fmeasure_ch_sel;
#[doc = "SMART_DMA_TRIG_CH_SEL (rw) register accessor: an alias for `Reg<SMART_DMA_TRIG_CH_SEL_SPEC>`"]
pub type SMART_DMA_TRIG_CH_SEL = crate::Reg<smart_dma_trig_ch_sel::SMART_DMA_TRIG_CH_SEL_SPEC>;
#[doc = "SMART_DMA trigger channel select"]
pub mod smart_dma_trig_ch_sel;
#[doc = "DMAC0_REQ_ENA0 (rw) register accessor: an alias for `Reg<DMAC0_REQ_ENA0_SPEC>`"]
pub type DMAC0_REQ_ENA0 = crate::Reg<dmac0_req_ena0::DMAC0_REQ_ENA0_SPEC>;
#[doc = "DMAC0 request enable 0"]
pub mod dmac0_req_ena0;
#[doc = "DMAC0_REQ_ENA1 (rw) register accessor: an alias for `Reg<DMAC0_REQ_ENA1_SPEC>`"]
pub type DMAC0_REQ_ENA1 = crate::Reg<dmac0_req_ena1::DMAC0_REQ_ENA1_SPEC>;
#[doc = "DMAC0 request enable 1"]
pub mod dmac0_req_ena1;
#[doc = "DMAC0_REQ_ENA0_SET (rw) register accessor: an alias for `Reg<DMAC0_REQ_ENA0_SET_SPEC>`"]
pub type DMAC0_REQ_ENA0_SET = crate::Reg<dmac0_req_ena0_set::DMAC0_REQ_ENA0_SET_SPEC>;
#[doc = "DMAC0 request enable set 0"]
pub mod dmac0_req_ena0_set;
#[doc = "DMAC0_REQ_ENA1_SET (rw) register accessor: an alias for `Reg<DMAC0_REQ_ENA1_SET_SPEC>`"]
pub type DMAC0_REQ_ENA1_SET = crate::Reg<dmac0_req_ena1_set::DMAC0_REQ_ENA1_SET_SPEC>;
#[doc = "DMAC0 request enable set 1"]
pub mod dmac0_req_ena1_set;
#[doc = "DMAC0_REQ_ENA0_CLR (rw) register accessor: an alias for `Reg<DMAC0_REQ_ENA0_CLR_SPEC>`"]
pub type DMAC0_REQ_ENA0_CLR = crate::Reg<dmac0_req_ena0_clr::DMAC0_REQ_ENA0_CLR_SPEC>;
#[doc = "DMAC0 request enable clear 0"]
pub mod dmac0_req_ena0_clr;
#[doc = "DMAC0_REQ_ENA1_CLR (rw) register accessor: an alias for `Reg<DMAC0_REQ_ENA1_CLR_SPEC>`"]
pub type DMAC0_REQ_ENA1_CLR = crate::Reg<dmac0_req_ena1_clr::DMAC0_REQ_ENA1_CLR_SPEC>;
#[doc = "DMAC0 request enable 1 clear"]
pub mod dmac0_req_ena1_clr;
#[doc = "DMAC1_REQ_ENA0 (rw) register accessor: an alias for `Reg<DMAC1_REQ_ENA0_SPEC>`"]
pub type DMAC1_REQ_ENA0 = crate::Reg<dmac1_req_ena0::DMAC1_REQ_ENA0_SPEC>;
#[doc = "DMAC1 request enable 0"]
pub mod dmac1_req_ena0;
#[doc = "DMAC1_REQ_ENA1 (rw) register accessor: an alias for `Reg<DMAC1_REQ_ENA1_SPEC>`"]
pub type DMAC1_REQ_ENA1 = crate::Reg<dmac1_req_ena1::DMAC1_REQ_ENA1_SPEC>;
#[doc = "DMAC1 request enable 1"]
pub mod dmac1_req_ena1;
#[doc = "DMAC1_REQ_ENA0_SET (rw) register accessor: an alias for `Reg<DMAC1_REQ_ENA0_SET_SPEC>`"]
pub type DMAC1_REQ_ENA0_SET = crate::Reg<dmac1_req_ena0_set::DMAC1_REQ_ENA0_SET_SPEC>;
#[doc = "DMAC1 request enable set 0"]
pub mod dmac1_req_ena0_set;
#[doc = "DMAC1_REQ_ENA1_SET (rw) register accessor: an alias for `Reg<DMAC1_REQ_ENA1_SET_SPEC>`"]
pub type DMAC1_REQ_ENA1_SET = crate::Reg<dmac1_req_ena1_set::DMAC1_REQ_ENA1_SET_SPEC>;
#[doc = "DMAC1 request enable set 1"]
pub mod dmac1_req_ena1_set;
#[doc = "DMAC1_REQ_ENA0_CLR (rw) register accessor: an alias for `Reg<DMAC1_REQ_ENA0_CLR_SPEC>`"]
pub type DMAC1_REQ_ENA0_CLR = crate::Reg<dmac1_req_ena0_clr::DMAC1_REQ_ENA0_CLR_SPEC>;
#[doc = "DMAC1 request enable clear 0"]
pub mod dmac1_req_ena0_clr;
#[doc = "DMAC1_REQ_ENA1_CLR (rw) register accessor: an alias for `Reg<DMAC1_REQ_ENA1_CLR_SPEC>`"]
pub type DMAC1_REQ_ENA1_CLR = crate::Reg<dmac1_req_ena1_clr::DMAC1_REQ_ENA1_CLR_SPEC>;
#[doc = "DMAC1 request enable 1 clear"]
pub mod dmac1_req_ena1_clr;
#[doc = "DMAC0_ITRIG_ENA0 (rw) register accessor: an alias for `Reg<DMAC0_ITRIG_ENA0_SPEC>`"]
pub type DMAC0_ITRIG_ENA0 = crate::Reg<dmac0_itrig_ena0::DMAC0_ITRIG_ENA0_SPEC>;
#[doc = "DMAC0 Input Trigger Enable 0"]
pub mod dmac0_itrig_ena0;
#[doc = "DMAC0_ITRIG_ENA0_SET (w) register accessor: an alias for `Reg<DMAC0_ITRIG_ENA0_SET_SPEC>`"]
pub type DMAC0_ITRIG_ENA0_SET = crate::Reg<dmac0_itrig_ena0_set::DMAC0_ITRIG_ENA0_SET_SPEC>;
#[doc = "DMAC0 Input Trigger Enable 0 Set"]
pub mod dmac0_itrig_ena0_set;
#[doc = "DMAC0_ITRIG_ENA0_CLR (w) register accessor: an alias for `Reg<DMAC0_ITRIG_ENA0_CLR_SPEC>`"]
pub type DMAC0_ITRIG_ENA0_CLR = crate::Reg<dmac0_itrig_ena0_clr::DMAC0_ITRIG_ENA0_CLR_SPEC>;
#[doc = "DMAC0 Input Trigger Enable 0 Clear"]
pub mod dmac0_itrig_ena0_clr;
#[doc = "DMAC1_ITRIG_ENA0 (rw) register accessor: an alias for `Reg<DMAC1_ITRIG_ENA0_SPEC>`"]
pub type DMAC1_ITRIG_ENA0 = crate::Reg<dmac1_itrig_ena0::DMAC1_ITRIG_ENA0_SPEC>;
#[doc = "DMAC1 Input Trigger Enable 0"]
pub mod dmac1_itrig_ena0;
#[doc = "DMAC1_ITRIG_ENA0_SET (w) register accessor: an alias for `Reg<DMAC1_ITRIG_ENA0_SET_SPEC>`"]
pub type DMAC1_ITRIG_ENA0_SET = crate::Reg<dmac1_itrig_ena0_set::DMAC1_ITRIG_ENA0_SET_SPEC>;
#[doc = "DMAC1 Input Trigger Enable 0 set"]
pub mod dmac1_itrig_ena0_set;
#[doc = "DMAC1_ITRIG_ENA0_CLR (w) register accessor: an alias for `Reg<DMAC1_ITRIG_ENA0_CLR_SPEC>`"]
pub type DMAC1_ITRIG_ENA0_CLR = crate::Reg<dmac1_itrig_ena0_clr::DMAC1_ITRIG_ENA0_CLR_SPEC>;
#[doc = "DMAC1 Input Trigger Enable 0 clear"]
pub mod dmac1_itrig_ena0_clr;
