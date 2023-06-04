#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Byte pin registers for all port GPIO pins"]
    pub port0_b0: PORT0_B0,
    #[doc = "0x01 - Byte pin registers for all port GPIO pins"]
    pub port0_b1: PORT0_B1,
    #[doc = "0x02 - Byte pin registers for all port GPIO pins"]
    pub port0_b2: PORT0_B2,
    #[doc = "0x03 - Byte pin registers for all port GPIO pins"]
    pub port0_b3: PORT0_B3,
    #[doc = "0x04 - Byte pin registers for all port GPIO pins"]
    pub port0_b4: PORT0_B4,
    #[doc = "0x05 - Byte pin registers for all port GPIO pins"]
    pub port0_b5: PORT0_B5,
    #[doc = "0x06 - Byte pin registers for all port GPIO pins"]
    pub port0_b6: PORT0_B6,
    #[doc = "0x07 - Byte pin registers for all port GPIO pins"]
    pub port0_b7: PORT0_B7,
    #[doc = "0x08 - Byte pin registers for all port GPIO pins"]
    pub port0_b8: PORT0_B8,
    #[doc = "0x09 - Byte pin registers for all port GPIO pins"]
    pub port0_b9: PORT0_B9,
    #[doc = "0x0a - Byte pin registers for all port GPIO pins"]
    pub port0_b10: PORT0_B10,
    #[doc = "0x0b - Byte pin registers for all port GPIO pins"]
    pub port0_b11: PORT0_B11,
    #[doc = "0x0c - Byte pin registers for all port GPIO pins"]
    pub port0_b12: PORT0_B12,
    #[doc = "0x0d - Byte pin registers for all port GPIO pins"]
    pub port0_b13: PORT0_B13,
    #[doc = "0x0e - Byte pin registers for all port GPIO pins"]
    pub port0_b14: PORT0_B14,
    #[doc = "0x0f - Byte pin registers for all port GPIO pins"]
    pub port0_b15: PORT0_B15,
    #[doc = "0x10 - Byte pin registers for all port GPIO pins"]
    pub port0_b16: PORT0_B16,
    #[doc = "0x11 - Byte pin registers for all port GPIO pins"]
    pub port0_b17: PORT0_B17,
    #[doc = "0x12 - Byte pin registers for all port GPIO pins"]
    pub port0_b18: PORT0_B18,
    #[doc = "0x13 - Byte pin registers for all port GPIO pins"]
    pub port0_b19: PORT0_B19,
    #[doc = "0x14 - Byte pin registers for all port GPIO pins"]
    pub port0_b20: PORT0_B20,
    #[doc = "0x15 - Byte pin registers for all port GPIO pins"]
    pub port0_b21: PORT0_B21,
    #[doc = "0x16 - Byte pin registers for all port GPIO pins"]
    pub port0_b22: PORT0_B22,
    #[doc = "0x17 - Byte pin registers for all port GPIO pins"]
    pub port0_b23: PORT0_B23,
    #[doc = "0x18 - Byte pin registers for all port GPIO pins"]
    pub port0_b24: PORT0_B24,
    #[doc = "0x19 - Byte pin registers for all port GPIO pins"]
    pub port0_b25: PORT0_B25,
    #[doc = "0x1a - Byte pin registers for all port GPIO pins"]
    pub port0_b26: PORT0_B26,
    #[doc = "0x1b - Byte pin registers for all port GPIO pins"]
    pub port0_b27: PORT0_B27,
    #[doc = "0x1c - Byte pin registers for all port GPIO pins"]
    pub port0_b28: PORT0_B28,
    #[doc = "0x1d - Byte pin registers for all port GPIO pins"]
    pub port0_b29: PORT0_B29,
    #[doc = "0x1e - Byte pin registers for all port GPIO pins"]
    pub port0_b30: PORT0_B30,
    #[doc = "0x1f - Byte pin registers for all port GPIO pins"]
    pub port0_b31: PORT0_B31,
    _reserved32: [u8; 0x0fe0],
    #[doc = "0x1000 - Word pin registers for all port GPIO pins"]
    pub port0_w0: PORT0_W0,
    #[doc = "0x1004 - Word pin registers for all port GPIO pins"]
    pub port0_w1: PORT0_W1,
    #[doc = "0x1008 - Word pin registers for all port GPIO pins"]
    pub port0_w2: PORT0_W2,
    #[doc = "0x100c - Word pin registers for all port GPIO pins"]
    pub port0_w3: PORT0_W3,
    #[doc = "0x1010 - Word pin registers for all port GPIO pins"]
    pub port0_w4: PORT0_W4,
    #[doc = "0x1014 - Word pin registers for all port GPIO pins"]
    pub port0_w5: PORT0_W5,
    #[doc = "0x1018 - Word pin registers for all port GPIO pins"]
    pub port0_w6: PORT0_W6,
    #[doc = "0x101c - Word pin registers for all port GPIO pins"]
    pub port0_w7: PORT0_W7,
    #[doc = "0x1020 - Word pin registers for all port GPIO pins"]
    pub port0_w8: PORT0_W8,
    #[doc = "0x1024 - Word pin registers for all port GPIO pins"]
    pub port0_w9: PORT0_W9,
    #[doc = "0x1028 - Word pin registers for all port GPIO pins"]
    pub port0_w10: PORT0_W10,
    #[doc = "0x102c - Word pin registers for all port GPIO pins"]
    pub port0_w11: PORT0_W11,
    #[doc = "0x1030 - Word pin registers for all port GPIO pins"]
    pub port0_w12: PORT0_W12,
    #[doc = "0x1034 - Word pin registers for all port GPIO pins"]
    pub port0_w13: PORT0_W13,
    #[doc = "0x1038 - Word pin registers for all port GPIO pins"]
    pub port0_w14: PORT0_W14,
    #[doc = "0x103c - Word pin registers for all port GPIO pins"]
    pub port0_w15: PORT0_W15,
    #[doc = "0x1040 - Word pin registers for all port GPIO pins"]
    pub port0_w16: PORT0_W16,
    #[doc = "0x1044 - Word pin registers for all port GPIO pins"]
    pub port0_w17: PORT0_W17,
    #[doc = "0x1048 - Word pin registers for all port GPIO pins"]
    pub port0_w18: PORT0_W18,
    #[doc = "0x104c - Word pin registers for all port GPIO pins"]
    pub port0_w19: PORT0_W19,
    #[doc = "0x1050 - Word pin registers for all port GPIO pins"]
    pub port0_w20: PORT0_W20,
    #[doc = "0x1054 - Word pin registers for all port GPIO pins"]
    pub port0_w21: PORT0_W21,
    #[doc = "0x1058 - Word pin registers for all port GPIO pins"]
    pub port0_w22: PORT0_W22,
    #[doc = "0x105c - Word pin registers for all port GPIO pins"]
    pub port0_w23: PORT0_W23,
    #[doc = "0x1060 - Word pin registers for all port GPIO pins"]
    pub port0_w24: PORT0_W24,
    #[doc = "0x1064 - Word pin registers for all port GPIO pins"]
    pub port0_w25: PORT0_W25,
    #[doc = "0x1068 - Word pin registers for all port GPIO pins"]
    pub port0_w26: PORT0_W26,
    #[doc = "0x106c - Word pin registers for all port GPIO pins"]
    pub port0_w27: PORT0_W27,
    #[doc = "0x1070 - Word pin registers for all port GPIO pins"]
    pub port0_w28: PORT0_W28,
    #[doc = "0x1074 - Word pin registers for all port GPIO pins"]
    pub port0_w29: PORT0_W29,
    #[doc = "0x1078 - Word pin registers for all port GPIO pins"]
    pub port0_w30: PORT0_W30,
    #[doc = "0x107c - Word pin registers for all port GPIO pins"]
    pub port0_w31: PORT0_W31,
    _reserved64: [u8; 0x0f80],
    #[doc = "0x2000 - Port direction"]
    pub dir0: DIR0,
    _reserved65: [u8; 0x7c],
    #[doc = "0x2080 - Port mask"]
    pub mask0: MASK0,
    _reserved66: [u8; 0x7c],
    #[doc = "0x2100 - Port pin"]
    pub pin0: PIN0,
    _reserved67: [u8; 0x7c],
    #[doc = "0x2180 - Masked Port Pin"]
    pub mpin0: MPIN0,
    _reserved68: [u8; 0x7c],
    #[doc = "0x2200 - Port set"]
    pub set0: SET0,
    _reserved69: [u8; 0x7c],
    #[doc = "0x2280 - Port clear"]
    pub clr0: CLR0,
    _reserved70: [u8; 0x7c],
    #[doc = "0x2300 - Port toggle"]
    pub not0: NOT0,
    _reserved71: [u8; 0x7c],
    #[doc = "0x2380 - Port direction set"]
    pub dirset0: DIRSET0,
    _reserved72: [u8; 0x7c],
    #[doc = "0x2400 - Port direction clear"]
    pub dirclr0: DIRCLR0,
    _reserved73: [u8; 0x7c],
    #[doc = "0x2480 - Port direction toggle"]
    pub dirnot0: DIRNOT0,
    _reserved74: [u8; 0x7c],
    #[doc = "0x2500 - Interrupt A enable control"]
    pub intena0: INTENA0,
    _reserved75: [u8; 0x7c],
    #[doc = "0x2580 - Interrupt B enable control"]
    pub intenb0: INTENB0,
    _reserved76: [u8; 0x7c],
    #[doc = "0x2600 - Interupt polarity control"]
    pub intpol0: INTPOL0,
    _reserved77: [u8; 0x7c],
    #[doc = "0x2680 - Interrupt edge select"]
    pub intedg0: INTEDG0,
    _reserved78: [u8; 0x7c],
    #[doc = "0x2700 - Interrupt status for interrupt A"]
    pub intstata0: INTSTATA0,
    _reserved79: [u8; 0x7c],
    #[doc = "0x2780 - Interrupt status for interrupt B"]
    pub intstatb0: INTSTATB0,
}
#[doc = "Port0_B0 (rw) register accessor: an alias for `Reg<PORT0_B0_SPEC>`"]
pub type PORT0_B0 = crate::Reg<port0_b0::PORT0_B0_SPEC>;
#[doc = "Byte pin registers for all port GPIO pins"]
pub mod port0_b0;
#[doc = "Port0_B1 (rw) register accessor: an alias for `Reg<PORT0_B1_SPEC>`"]
pub type PORT0_B1 = crate::Reg<port0_b1::PORT0_B1_SPEC>;
#[doc = "Byte pin registers for all port GPIO pins"]
pub mod port0_b1;
#[doc = "Port0_B2 (rw) register accessor: an alias for `Reg<PORT0_B2_SPEC>`"]
pub type PORT0_B2 = crate::Reg<port0_b2::PORT0_B2_SPEC>;
#[doc = "Byte pin registers for all port GPIO pins"]
pub mod port0_b2;
#[doc = "Port0_B3 (rw) register accessor: an alias for `Reg<PORT0_B3_SPEC>`"]
pub type PORT0_B3 = crate::Reg<port0_b3::PORT0_B3_SPEC>;
#[doc = "Byte pin registers for all port GPIO pins"]
pub mod port0_b3;
#[doc = "Port0_B4 (rw) register accessor: an alias for `Reg<PORT0_B4_SPEC>`"]
pub type PORT0_B4 = crate::Reg<port0_b4::PORT0_B4_SPEC>;
#[doc = "Byte pin registers for all port GPIO pins"]
pub mod port0_b4;
#[doc = "Port0_B5 (rw) register accessor: an alias for `Reg<PORT0_B5_SPEC>`"]
pub type PORT0_B5 = crate::Reg<port0_b5::PORT0_B5_SPEC>;
#[doc = "Byte pin registers for all port GPIO pins"]
pub mod port0_b5;
#[doc = "Port0_B6 (rw) register accessor: an alias for `Reg<PORT0_B6_SPEC>`"]
pub type PORT0_B6 = crate::Reg<port0_b6::PORT0_B6_SPEC>;
#[doc = "Byte pin registers for all port GPIO pins"]
pub mod port0_b6;
#[doc = "Port0_B7 (rw) register accessor: an alias for `Reg<PORT0_B7_SPEC>`"]
pub type PORT0_B7 = crate::Reg<port0_b7::PORT0_B7_SPEC>;
#[doc = "Byte pin registers for all port GPIO pins"]
pub mod port0_b7;
#[doc = "Port0_B8 (rw) register accessor: an alias for `Reg<PORT0_B8_SPEC>`"]
pub type PORT0_B8 = crate::Reg<port0_b8::PORT0_B8_SPEC>;
#[doc = "Byte pin registers for all port GPIO pins"]
pub mod port0_b8;
#[doc = "Port0_B9 (rw) register accessor: an alias for `Reg<PORT0_B9_SPEC>`"]
pub type PORT0_B9 = crate::Reg<port0_b9::PORT0_B9_SPEC>;
#[doc = "Byte pin registers for all port GPIO pins"]
pub mod port0_b9;
#[doc = "Port0_B10 (rw) register accessor: an alias for `Reg<PORT0_B10_SPEC>`"]
pub type PORT0_B10 = crate::Reg<port0_b10::PORT0_B10_SPEC>;
#[doc = "Byte pin registers for all port GPIO pins"]
pub mod port0_b10;
#[doc = "Port0_B11 (rw) register accessor: an alias for `Reg<PORT0_B11_SPEC>`"]
pub type PORT0_B11 = crate::Reg<port0_b11::PORT0_B11_SPEC>;
#[doc = "Byte pin registers for all port GPIO pins"]
pub mod port0_b11;
#[doc = "Port0_B12 (rw) register accessor: an alias for `Reg<PORT0_B12_SPEC>`"]
pub type PORT0_B12 = crate::Reg<port0_b12::PORT0_B12_SPEC>;
#[doc = "Byte pin registers for all port GPIO pins"]
pub mod port0_b12;
#[doc = "Port0_B13 (rw) register accessor: an alias for `Reg<PORT0_B13_SPEC>`"]
pub type PORT0_B13 = crate::Reg<port0_b13::PORT0_B13_SPEC>;
#[doc = "Byte pin registers for all port GPIO pins"]
pub mod port0_b13;
#[doc = "Port0_B14 (rw) register accessor: an alias for `Reg<PORT0_B14_SPEC>`"]
pub type PORT0_B14 = crate::Reg<port0_b14::PORT0_B14_SPEC>;
#[doc = "Byte pin registers for all port GPIO pins"]
pub mod port0_b14;
#[doc = "Port0_B15 (rw) register accessor: an alias for `Reg<PORT0_B15_SPEC>`"]
pub type PORT0_B15 = crate::Reg<port0_b15::PORT0_B15_SPEC>;
#[doc = "Byte pin registers for all port GPIO pins"]
pub mod port0_b15;
#[doc = "Port0_B16 (rw) register accessor: an alias for `Reg<PORT0_B16_SPEC>`"]
pub type PORT0_B16 = crate::Reg<port0_b16::PORT0_B16_SPEC>;
#[doc = "Byte pin registers for all port GPIO pins"]
pub mod port0_b16;
#[doc = "Port0_B17 (rw) register accessor: an alias for `Reg<PORT0_B17_SPEC>`"]
pub type PORT0_B17 = crate::Reg<port0_b17::PORT0_B17_SPEC>;
#[doc = "Byte pin registers for all port GPIO pins"]
pub mod port0_b17;
#[doc = "Port0_B18 (rw) register accessor: an alias for `Reg<PORT0_B18_SPEC>`"]
pub type PORT0_B18 = crate::Reg<port0_b18::PORT0_B18_SPEC>;
#[doc = "Byte pin registers for all port GPIO pins"]
pub mod port0_b18;
#[doc = "Port0_B19 (rw) register accessor: an alias for `Reg<PORT0_B19_SPEC>`"]
pub type PORT0_B19 = crate::Reg<port0_b19::PORT0_B19_SPEC>;
#[doc = "Byte pin registers for all port GPIO pins"]
pub mod port0_b19;
#[doc = "Port0_B20 (rw) register accessor: an alias for `Reg<PORT0_B20_SPEC>`"]
pub type PORT0_B20 = crate::Reg<port0_b20::PORT0_B20_SPEC>;
#[doc = "Byte pin registers for all port GPIO pins"]
pub mod port0_b20;
#[doc = "Port0_B21 (rw) register accessor: an alias for `Reg<PORT0_B21_SPEC>`"]
pub type PORT0_B21 = crate::Reg<port0_b21::PORT0_B21_SPEC>;
#[doc = "Byte pin registers for all port GPIO pins"]
pub mod port0_b21;
#[doc = "Port0_B22 (rw) register accessor: an alias for `Reg<PORT0_B22_SPEC>`"]
pub type PORT0_B22 = crate::Reg<port0_b22::PORT0_B22_SPEC>;
#[doc = "Byte pin registers for all port GPIO pins"]
pub mod port0_b22;
#[doc = "Port0_B23 (rw) register accessor: an alias for `Reg<PORT0_B23_SPEC>`"]
pub type PORT0_B23 = crate::Reg<port0_b23::PORT0_B23_SPEC>;
#[doc = "Byte pin registers for all port GPIO pins"]
pub mod port0_b23;
#[doc = "Port0_B24 (rw) register accessor: an alias for `Reg<PORT0_B24_SPEC>`"]
pub type PORT0_B24 = crate::Reg<port0_b24::PORT0_B24_SPEC>;
#[doc = "Byte pin registers for all port GPIO pins"]
pub mod port0_b24;
#[doc = "Port0_B25 (rw) register accessor: an alias for `Reg<PORT0_B25_SPEC>`"]
pub type PORT0_B25 = crate::Reg<port0_b25::PORT0_B25_SPEC>;
#[doc = "Byte pin registers for all port GPIO pins"]
pub mod port0_b25;
#[doc = "Port0_B26 (rw) register accessor: an alias for `Reg<PORT0_B26_SPEC>`"]
pub type PORT0_B26 = crate::Reg<port0_b26::PORT0_B26_SPEC>;
#[doc = "Byte pin registers for all port GPIO pins"]
pub mod port0_b26;
#[doc = "Port0_B27 (rw) register accessor: an alias for `Reg<PORT0_B27_SPEC>`"]
pub type PORT0_B27 = crate::Reg<port0_b27::PORT0_B27_SPEC>;
#[doc = "Byte pin registers for all port GPIO pins"]
pub mod port0_b27;
#[doc = "Port0_B28 (rw) register accessor: an alias for `Reg<PORT0_B28_SPEC>`"]
pub type PORT0_B28 = crate::Reg<port0_b28::PORT0_B28_SPEC>;
#[doc = "Byte pin registers for all port GPIO pins"]
pub mod port0_b28;
#[doc = "Port0_B29 (rw) register accessor: an alias for `Reg<PORT0_B29_SPEC>`"]
pub type PORT0_B29 = crate::Reg<port0_b29::PORT0_B29_SPEC>;
#[doc = "Byte pin registers for all port GPIO pins"]
pub mod port0_b29;
#[doc = "Port0_B30 (rw) register accessor: an alias for `Reg<PORT0_B30_SPEC>`"]
pub type PORT0_B30 = crate::Reg<port0_b30::PORT0_B30_SPEC>;
#[doc = "Byte pin registers for all port GPIO pins"]
pub mod port0_b30;
#[doc = "Port0_B31 (rw) register accessor: an alias for `Reg<PORT0_B31_SPEC>`"]
pub type PORT0_B31 = crate::Reg<port0_b31::PORT0_B31_SPEC>;
#[doc = "Byte pin registers for all port GPIO pins"]
pub mod port0_b31;
#[doc = "Port0_W0 (rw) register accessor: an alias for `Reg<PORT0_W0_SPEC>`"]
pub type PORT0_W0 = crate::Reg<port0_w0::PORT0_W0_SPEC>;
#[doc = "Word pin registers for all port GPIO pins"]
pub mod port0_w0;
#[doc = "Port0_W1 (rw) register accessor: an alias for `Reg<PORT0_W1_SPEC>`"]
pub type PORT0_W1 = crate::Reg<port0_w1::PORT0_W1_SPEC>;
#[doc = "Word pin registers for all port GPIO pins"]
pub mod port0_w1;
#[doc = "Port0_W2 (rw) register accessor: an alias for `Reg<PORT0_W2_SPEC>`"]
pub type PORT0_W2 = crate::Reg<port0_w2::PORT0_W2_SPEC>;
#[doc = "Word pin registers for all port GPIO pins"]
pub mod port0_w2;
#[doc = "Port0_W3 (rw) register accessor: an alias for `Reg<PORT0_W3_SPEC>`"]
pub type PORT0_W3 = crate::Reg<port0_w3::PORT0_W3_SPEC>;
#[doc = "Word pin registers for all port GPIO pins"]
pub mod port0_w3;
#[doc = "Port0_W4 (rw) register accessor: an alias for `Reg<PORT0_W4_SPEC>`"]
pub type PORT0_W4 = crate::Reg<port0_w4::PORT0_W4_SPEC>;
#[doc = "Word pin registers for all port GPIO pins"]
pub mod port0_w4;
#[doc = "Port0_W5 (rw) register accessor: an alias for `Reg<PORT0_W5_SPEC>`"]
pub type PORT0_W5 = crate::Reg<port0_w5::PORT0_W5_SPEC>;
#[doc = "Word pin registers for all port GPIO pins"]
pub mod port0_w5;
#[doc = "Port0_W6 (rw) register accessor: an alias for `Reg<PORT0_W6_SPEC>`"]
pub type PORT0_W6 = crate::Reg<port0_w6::PORT0_W6_SPEC>;
#[doc = "Word pin registers for all port GPIO pins"]
pub mod port0_w6;
#[doc = "Port0_W7 (rw) register accessor: an alias for `Reg<PORT0_W7_SPEC>`"]
pub type PORT0_W7 = crate::Reg<port0_w7::PORT0_W7_SPEC>;
#[doc = "Word pin registers for all port GPIO pins"]
pub mod port0_w7;
#[doc = "Port0_W8 (rw) register accessor: an alias for `Reg<PORT0_W8_SPEC>`"]
pub type PORT0_W8 = crate::Reg<port0_w8::PORT0_W8_SPEC>;
#[doc = "Word pin registers for all port GPIO pins"]
pub mod port0_w8;
#[doc = "Port0_W9 (rw) register accessor: an alias for `Reg<PORT0_W9_SPEC>`"]
pub type PORT0_W9 = crate::Reg<port0_w9::PORT0_W9_SPEC>;
#[doc = "Word pin registers for all port GPIO pins"]
pub mod port0_w9;
#[doc = "Port0_W10 (rw) register accessor: an alias for `Reg<PORT0_W10_SPEC>`"]
pub type PORT0_W10 = crate::Reg<port0_w10::PORT0_W10_SPEC>;
#[doc = "Word pin registers for all port GPIO pins"]
pub mod port0_w10;
#[doc = "Port0_W11 (rw) register accessor: an alias for `Reg<PORT0_W11_SPEC>`"]
pub type PORT0_W11 = crate::Reg<port0_w11::PORT0_W11_SPEC>;
#[doc = "Word pin registers for all port GPIO pins"]
pub mod port0_w11;
#[doc = "Port0_W12 (rw) register accessor: an alias for `Reg<PORT0_W12_SPEC>`"]
pub type PORT0_W12 = crate::Reg<port0_w12::PORT0_W12_SPEC>;
#[doc = "Word pin registers for all port GPIO pins"]
pub mod port0_w12;
#[doc = "Port0_W13 (rw) register accessor: an alias for `Reg<PORT0_W13_SPEC>`"]
pub type PORT0_W13 = crate::Reg<port0_w13::PORT0_W13_SPEC>;
#[doc = "Word pin registers for all port GPIO pins"]
pub mod port0_w13;
#[doc = "Port0_W14 (rw) register accessor: an alias for `Reg<PORT0_W14_SPEC>`"]
pub type PORT0_W14 = crate::Reg<port0_w14::PORT0_W14_SPEC>;
#[doc = "Word pin registers for all port GPIO pins"]
pub mod port0_w14;
#[doc = "Port0_W15 (rw) register accessor: an alias for `Reg<PORT0_W15_SPEC>`"]
pub type PORT0_W15 = crate::Reg<port0_w15::PORT0_W15_SPEC>;
#[doc = "Word pin registers for all port GPIO pins"]
pub mod port0_w15;
#[doc = "Port0_W16 (rw) register accessor: an alias for `Reg<PORT0_W16_SPEC>`"]
pub type PORT0_W16 = crate::Reg<port0_w16::PORT0_W16_SPEC>;
#[doc = "Word pin registers for all port GPIO pins"]
pub mod port0_w16;
#[doc = "Port0_W17 (rw) register accessor: an alias for `Reg<PORT0_W17_SPEC>`"]
pub type PORT0_W17 = crate::Reg<port0_w17::PORT0_W17_SPEC>;
#[doc = "Word pin registers for all port GPIO pins"]
pub mod port0_w17;
#[doc = "Port0_W18 (rw) register accessor: an alias for `Reg<PORT0_W18_SPEC>`"]
pub type PORT0_W18 = crate::Reg<port0_w18::PORT0_W18_SPEC>;
#[doc = "Word pin registers for all port GPIO pins"]
pub mod port0_w18;
#[doc = "Port0_W19 (rw) register accessor: an alias for `Reg<PORT0_W19_SPEC>`"]
pub type PORT0_W19 = crate::Reg<port0_w19::PORT0_W19_SPEC>;
#[doc = "Word pin registers for all port GPIO pins"]
pub mod port0_w19;
#[doc = "Port0_W20 (rw) register accessor: an alias for `Reg<PORT0_W20_SPEC>`"]
pub type PORT0_W20 = crate::Reg<port0_w20::PORT0_W20_SPEC>;
#[doc = "Word pin registers for all port GPIO pins"]
pub mod port0_w20;
#[doc = "Port0_W21 (rw) register accessor: an alias for `Reg<PORT0_W21_SPEC>`"]
pub type PORT0_W21 = crate::Reg<port0_w21::PORT0_W21_SPEC>;
#[doc = "Word pin registers for all port GPIO pins"]
pub mod port0_w21;
#[doc = "Port0_W22 (rw) register accessor: an alias for `Reg<PORT0_W22_SPEC>`"]
pub type PORT0_W22 = crate::Reg<port0_w22::PORT0_W22_SPEC>;
#[doc = "Word pin registers for all port GPIO pins"]
pub mod port0_w22;
#[doc = "Port0_W23 (rw) register accessor: an alias for `Reg<PORT0_W23_SPEC>`"]
pub type PORT0_W23 = crate::Reg<port0_w23::PORT0_W23_SPEC>;
#[doc = "Word pin registers for all port GPIO pins"]
pub mod port0_w23;
#[doc = "Port0_W24 (rw) register accessor: an alias for `Reg<PORT0_W24_SPEC>`"]
pub type PORT0_W24 = crate::Reg<port0_w24::PORT0_W24_SPEC>;
#[doc = "Word pin registers for all port GPIO pins"]
pub mod port0_w24;
#[doc = "Port0_W25 (rw) register accessor: an alias for `Reg<PORT0_W25_SPEC>`"]
pub type PORT0_W25 = crate::Reg<port0_w25::PORT0_W25_SPEC>;
#[doc = "Word pin registers for all port GPIO pins"]
pub mod port0_w25;
#[doc = "Port0_W26 (rw) register accessor: an alias for `Reg<PORT0_W26_SPEC>`"]
pub type PORT0_W26 = crate::Reg<port0_w26::PORT0_W26_SPEC>;
#[doc = "Word pin registers for all port GPIO pins"]
pub mod port0_w26;
#[doc = "Port0_W27 (rw) register accessor: an alias for `Reg<PORT0_W27_SPEC>`"]
pub type PORT0_W27 = crate::Reg<port0_w27::PORT0_W27_SPEC>;
#[doc = "Word pin registers for all port GPIO pins"]
pub mod port0_w27;
#[doc = "Port0_W28 (rw) register accessor: an alias for `Reg<PORT0_W28_SPEC>`"]
pub type PORT0_W28 = crate::Reg<port0_w28::PORT0_W28_SPEC>;
#[doc = "Word pin registers for all port GPIO pins"]
pub mod port0_w28;
#[doc = "Port0_W29 (rw) register accessor: an alias for `Reg<PORT0_W29_SPEC>`"]
pub type PORT0_W29 = crate::Reg<port0_w29::PORT0_W29_SPEC>;
#[doc = "Word pin registers for all port GPIO pins"]
pub mod port0_w29;
#[doc = "Port0_W30 (rw) register accessor: an alias for `Reg<PORT0_W30_SPEC>`"]
pub type PORT0_W30 = crate::Reg<port0_w30::PORT0_W30_SPEC>;
#[doc = "Word pin registers for all port GPIO pins"]
pub mod port0_w30;
#[doc = "Port0_W31 (rw) register accessor: an alias for `Reg<PORT0_W31_SPEC>`"]
pub type PORT0_W31 = crate::Reg<port0_w31::PORT0_W31_SPEC>;
#[doc = "Word pin registers for all port GPIO pins"]
pub mod port0_w31;
#[doc = "DIR0 (w) register accessor: an alias for `Reg<DIR0_SPEC>`"]
pub type DIR0 = crate::Reg<dir0::DIR0_SPEC>;
#[doc = "Port direction"]
pub mod dir0;
#[doc = "MASK0 (rw) register accessor: an alias for `Reg<MASK0_SPEC>`"]
pub type MASK0 = crate::Reg<mask0::MASK0_SPEC>;
#[doc = "Port mask"]
pub mod mask0;
#[doc = "PIN0 (rw) register accessor: an alias for `Reg<PIN0_SPEC>`"]
pub type PIN0 = crate::Reg<pin0::PIN0_SPEC>;
#[doc = "Port pin"]
pub mod pin0;
#[doc = "MPIN0 (rw) register accessor: an alias for `Reg<MPIN0_SPEC>`"]
pub type MPIN0 = crate::Reg<mpin0::MPIN0_SPEC>;
#[doc = "Masked Port Pin"]
pub mod mpin0;
#[doc = "SET0 (rw) register accessor: an alias for `Reg<SET0_SPEC>`"]
pub type SET0 = crate::Reg<set0::SET0_SPEC>;
#[doc = "Port set"]
pub mod set0;
#[doc = "CLR0 (rw) register accessor: an alias for `Reg<CLR0_SPEC>`"]
pub type CLR0 = crate::Reg<clr0::CLR0_SPEC>;
#[doc = "Port clear"]
pub mod clr0;
#[doc = "NOT0 (w) register accessor: an alias for `Reg<NOT0_SPEC>`"]
pub type NOT0 = crate::Reg<not0::NOT0_SPEC>;
#[doc = "Port toggle"]
pub mod not0;
#[doc = "DIRSET0 (w) register accessor: an alias for `Reg<DIRSET0_SPEC>`"]
pub type DIRSET0 = crate::Reg<dirset0::DIRSET0_SPEC>;
#[doc = "Port direction set"]
pub mod dirset0;
#[doc = "DIRCLR0 (rw) register accessor: an alias for `Reg<DIRCLR0_SPEC>`"]
pub type DIRCLR0 = crate::Reg<dirclr0::DIRCLR0_SPEC>;
#[doc = "Port direction clear"]
pub mod dirclr0;
#[doc = "DIRNOT0 (w) register accessor: an alias for `Reg<DIRNOT0_SPEC>`"]
pub type DIRNOT0 = crate::Reg<dirnot0::DIRNOT0_SPEC>;
#[doc = "Port direction toggle"]
pub mod dirnot0;
#[doc = "INTENA0 (rw) register accessor: an alias for `Reg<INTENA0_SPEC>`"]
pub type INTENA0 = crate::Reg<intena0::INTENA0_SPEC>;
#[doc = "Interrupt A enable control"]
pub mod intena0;
#[doc = "INTENB0 (rw) register accessor: an alias for `Reg<INTENB0_SPEC>`"]
pub type INTENB0 = crate::Reg<intenb0::INTENB0_SPEC>;
#[doc = "Interrupt B enable control"]
pub mod intenb0;
#[doc = "INTPOL0 (rw) register accessor: an alias for `Reg<INTPOL0_SPEC>`"]
pub type INTPOL0 = crate::Reg<intpol0::INTPOL0_SPEC>;
#[doc = "Interupt polarity control"]
pub mod intpol0;
#[doc = "INTEDG0 (rw) register accessor: an alias for `Reg<INTEDG0_SPEC>`"]
pub type INTEDG0 = crate::Reg<intedg0::INTEDG0_SPEC>;
#[doc = "Interrupt edge select"]
pub mod intedg0;
#[doc = "INTSTATA0 (rw) register accessor: an alias for `Reg<INTSTATA0_SPEC>`"]
pub type INTSTATA0 = crate::Reg<intstata0::INTSTATA0_SPEC>;
#[doc = "Interrupt status for interrupt A"]
pub mod intstata0;
#[doc = "INTSTATB0 (rw) register accessor: an alias for `Reg<INTSTATB0_SPEC>`"]
pub type INTSTATB0 = crate::Reg<intstatb0::INTSTATB0_SPEC>;
#[doc = "Interrupt status for interrupt B"]
pub mod intstatb0;
