#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 0x10],
    #[doc = "0x10 - Clock Control 0"]
    pub pscctl0: PSCCTL0,
    #[doc = "0x14 - Clock Control 1"]
    pub pscctl1: PSCCTL1,
    #[doc = "0x18 - Clock Control 2"]
    pub pscctl2: PSCCTL2,
    _reserved3: [u8; 0x24],
    #[doc = "0x40 - Clock Set 0"]
    pub pscctl0_set: PSCCTL0_SET,
    #[doc = "0x44 - Clock Set 1"]
    pub pscctl1_set: PSCCTL1_SET,
    #[doc = "0x48 - Clock Set 2"]
    pub pscctl2_set: PSCCTL2_SET,
    _reserved6: [u8; 0x24],
    #[doc = "0x70 - Clock Clear 0"]
    pub pscctl0_clr: PSCCTL0_CLR,
    #[doc = "0x74 - Clock Clear 1"]
    pub pscctl1_clr: PSCCTL1_CLR,
    #[doc = "0x78 - Clock Clear 2"]
    pub pscctl2_clr: PSCCTL2_CLR,
    _reserved9: [u8; 0x0184],
    #[doc = "0x200 - Audio PLL0 Clock Select"]
    pub audiopll0clksel: AUDIOPLL0CLKSEL,
    #[doc = "0x204 - Audio PLL0 Control 0"]
    pub audiopll0ctl0: AUDIOPLL0CTL0,
    _reserved11: [u8; 0x04],
    #[doc = "0x20c - Audio PLL0 Lock Time Divide-by-2"]
    pub audiopll0locktimediv2: AUDIOPLL0LOCKTIMEDIV2,
    #[doc = "0x210 - Audio PLL0 Numerator"]
    pub audiopll0num: AUDIOPLL0NUM,
    #[doc = "0x214 - Audio PLL0 Denominator"]
    pub audiopll0denom: AUDIOPLL0DENOM,
    #[doc = "0x218 - Audio PLL0 PFD"]
    pub audiopll0pfd: AUDIOPLL0PFD,
    _reserved15: [u8; 0x24],
    #[doc = "0x240 - Audio PLL Clock Divider"]
    pub audiopllclkdiv: AUDIOPLLCLKDIV,
    _reserved16: [u8; 0x01bc],
    #[doc = "0x400 - DSP CPU Clock Divider"]
    pub dspcpuclkdiv: DSPCPUCLKDIV,
    _reserved17: [u8; 0x2c],
    #[doc = "0x430 - DSP CPU Clock Select A"]
    pub dspcpuclksela: DSPCPUCLKSELA,
    #[doc = "0x434 - DSP CPU Clock Select B"]
    pub dspcpuclkselb: DSPCPUCLKSELB,
    _reserved19: [u8; 0x48],
    #[doc = "0x480 - OS Event Timer Functional Clock Select"]
    pub oseventtfclksel: OSEVENTTFCLKSEL,
    _reserved20: [u8; 0x7c],
    #[doc = "0x500 - Fractional Rate Generator 0 Clock Select"]
    pub frg0clksel: FRG0CLKSEL,
    #[doc = "0x504 - Fractional Rate Generator 0 Control"]
    pub frg0ctl: FRG0CTL,
    #[doc = "0x508 - Flexcomm0 Clock Select"]
    pub fc0fclksel: FC0FCLKSEL,
    _reserved23: [u8; 0x14],
    #[doc = "0x520 - Fractional Rate Generator 1 Clock Select"]
    pub frg1clksel: FRG1CLKSEL,
    #[doc = "0x524 - Fractional Rate Generator 1 Control"]
    pub frg1ctl: FRG1CTL,
    #[doc = "0x528 - Flexcomm1 Clock Select"]
    pub fc1fclksel: FC1FCLKSEL,
    _reserved26: [u8; 0x14],
    #[doc = "0x540 - Fractional Rate Generator 2 Clock Select"]
    pub frg2clksel: FRG2CLKSEL,
    #[doc = "0x544 - Fractional Rate Generator 2 Control"]
    pub frg2ctl: FRG2CTL,
    #[doc = "0x548 - Flexcomm2 Clock Select"]
    pub fc2fclksel: FC2FCLKSEL,
    _reserved29: [u8; 0x14],
    #[doc = "0x560 - Fractional Rate Generator 3 Clock Select"]
    pub frg3clksel: FRG3CLKSEL,
    #[doc = "0x564 - Fractional Rate Generator 3 Control"]
    pub frg3ctl: FRG3CTL,
    #[doc = "0x568 - Flexcomm3 Clock Select"]
    pub fc3fclksel: FC3FCLKSEL,
    _reserved32: [u8; 0x14],
    #[doc = "0x580 - Fractional Rate Generator 4 Clock Select"]
    pub frg4clksel: FRG4CLKSEL,
    #[doc = "0x584 - Fractional Rate Generator 4 Control"]
    pub frg4ctl: FRG4CTL,
    #[doc = "0x588 - Flexcomm4 Clock Select"]
    pub fc4fclksel: FC4FCLKSEL,
    _reserved35: [u8; 0x14],
    #[doc = "0x5a0 - Fractional Rate Generator 5 Clock Select"]
    pub frg5clksel: FRG5CLKSEL,
    #[doc = "0x5a4 - Fractional Rate Generator 5 Control"]
    pub frg5ctl: FRG5CTL,
    #[doc = "0x5a8 - Flexcomm5 Clock Select"]
    pub fc5fclksel: FC5FCLKSEL,
    _reserved38: [u8; 0x14],
    #[doc = "0x5c0 - Fractional Rate Generator 6 Clock Select"]
    pub frg6clksel: FRG6CLKSEL,
    #[doc = "0x5c4 - Fractional Rate Generator 6 Control"]
    pub frg6ctl: FRG6CTL,
    #[doc = "0x5c8 - Flexcomm6 Clock Select"]
    pub fc6fclksel: FC6FCLKSEL,
    _reserved41: [u8; 0x14],
    #[doc = "0x5e0 - Fractional Rate Generator 7 Clock Select"]
    pub frg7clksel: FRG7CLKSEL,
    #[doc = "0x5e4 - Fractional Rate Generator 7 Control"]
    pub frg7ctl: FRG7CTL,
    #[doc = "0x5e8 - Flexcomm7 Clock Select"]
    pub fc7fclksel: FC7FCLKSEL,
    _reserved44: [u8; 0x14],
    #[doc = "0x600 - Fractional Rate Generator 8 Clock Select"]
    pub frg8clksel: FRG8CLKSEL,
    #[doc = "0x604 - Fractional Rate Generator 8 Control"]
    pub frg8ctl: FRG8CTL,
    #[doc = "0x608 - Flexcomm8 Clock Select"]
    pub fc8fclksel: FC8FCLKSEL,
    _reserved47: [u8; 0x14],
    #[doc = "0x620 - Fractional Rate Generator 9 Clock Select"]
    pub frg9clksel: FRG9CLKSEL,
    #[doc = "0x624 - Fractional Rate Generator 9 Control"]
    pub frg9ctl: FRG9CTL,
    #[doc = "0x628 - Flexcomm9 Clock Select"]
    pub fc9fclksel: FC9FCLKSEL,
    _reserved50: [u8; 0x14],
    #[doc = "0x640 - Fractional Rate Generator 10 Clock Select"]
    pub frg10clksel: FRG10CLKSEL,
    #[doc = "0x644 - Fractional Rate Generator 10 Control"]
    pub frg10ctl: FRG10CTL,
    #[doc = "0x648 - Flexcomm10 Clock Select"]
    pub fc10fclksel: FC10FCLKSEL,
    _reserved53: [u8; 0x14],
    #[doc = "0x660 - Fractional Rate Generator 11 Clock Select"]
    pub frg11clksel: FRG11CLKSEL,
    #[doc = "0x664 - Fractional Rate Generator 11 Control"]
    pub frg11ctl: FRG11CTL,
    #[doc = "0x668 - Flexcomm11 Clock Select"]
    pub fc11fclksel: FC11FCLKSEL,
    _reserved56: [u8; 0x14],
    #[doc = "0x680 - Fractional Rate Generator 12 Clock Select"]
    pub frg12clksel: FRG12CLKSEL,
    #[doc = "0x684 - Fractional Rate Generator 12 Control"]
    pub frg12ctl: FRG12CTL,
    #[doc = "0x688 - Flexcomm12 Clock Select"]
    pub fc12fclksel: FC12FCLKSEL,
    _reserved59: [u8; 0x14],
    #[doc = "0x6a0 - Fractional Rate Generator 13 Clock Select"]
    pub frg13clksel: FRG13CLKSEL,
    #[doc = "0x6a4 - Fractional Rate Generator 13 Control"]
    pub frg13ctl: FRG13CTL,
    #[doc = "0x6a8 - Flexcomm13 Clock Select"]
    pub fc13fclksel: FC13FCLKSEL,
    _reserved62: [u8; 0x14],
    #[doc = "0x6c0 - Fractional Rate Generator 14 Clock Select"]
    pub frg14clksel: FRG14CLKSEL,
    #[doc = "0x6c4 - Fractional Rate Generator 14 Control"]
    pub frg14ctl: FRG14CTL,
    #[doc = "0x6c8 - Flexcomm14 Clock Select"]
    pub fc14fclksel: FC14FCLKSEL,
    _reserved65: [u8; 0x14],
    #[doc = "0x6e0 - Fractional Rate Generator 15 Clock Select"]
    pub frg15clksel: FRG15CLKSEL,
    #[doc = "0x6e4 - Fractional Rate Generator 15 Control"]
    pub frg15ctl: FRG15CTL,
    #[doc = "0x6e8 - Flexcomm15 Clock Select"]
    pub fc15fclksel: FC15FCLKSEL,
    _reserved68: [u8; 0x14],
    #[doc = "0x700 - Fractional Rate Generator 16 Clock Select"]
    pub frg16clksel: FRG16CLKSEL,
    #[doc = "0x704 - Fractional Rate Generator 16 Control"]
    pub frg16ctl: FRG16CTL,
    #[doc = "0x708 - Flexcomm16 Clock Select"]
    pub fc16fclksel: FC16FCLKSEL,
    _reserved71: [u8; 0x14],
    #[doc = "0x720 - Fractional Rate Generator 17 Clock Select"]
    pub frg17clksel: FRG17CLKSEL,
    #[doc = "0x724 - Fractional Rate Generator 17 Control"]
    pub frg17ctl: FRG17CTL,
    #[doc = "0x728 - FlexIO Clock Select"]
    pub flexioclksel: FLEXIOCLKSEL,
    _reserved74: [u8; 0x14],
    #[doc = "0x740 - FlexIO Clock Divider"]
    pub flexioclkdiv: FLEXIOCLKDIV,
    _reserved75: [u8; 0x1c],
    #[doc = "0x760 - Fractional Rate Generator PLL Clock Divider"]
    pub frgpllclkdiv: FRGPLLCLKDIV,
    _reserved76: [u8; 0x1c],
    #[doc = "0x780 - DMIC0 Functional Clock Select"]
    pub dmic0fclksel: DMIC0FCLKSEL,
    #[doc = "0x784 - DMIC0 Functional Clock Divider"]
    pub dmic0fclkdiv: DMIC0FCLKDIV,
    _reserved78: [u8; 0x18],
    #[doc = "0x7a0..0x7b4 - CT32BIT bit timer index Functional Clock Select"]
    pub ct32bitfclksel: [CT32BITFCLKSEL; 5],
    _reserved79: [u8; 0x0c],
    #[doc = "0x7c0 - Audio MCLK Clock Select"]
    pub audiomclksel: AUDIOMCLKSEL,
    #[doc = "0x7c4 - Audio MCLK Clock Divider"]
    pub audiomclkdiv: AUDIOMCLKDIV,
    _reserved81: [u8; 0x18],
    #[doc = "0x7e0 - CLKOUT Clock Select 0"]
    pub clkoutsel0: CLKOUTSEL0,
    #[doc = "0x7e4 - CLKOUT Clock Select 1"]
    pub clkoutsel1: CLKOUTSEL1,
    #[doc = "0x7e8 - CLKOUT Functional Clock Divider"]
    pub clkoutfclkdiv: CLKOUTFCLKDIV,
    _reserved84: [u8; 0x14],
    #[doc = "0x800 - I3C0, I3C1 Functional Clock Select"]
    pub i3c01fclksel: I3C01FCLKSEL,
    #[doc = "0x804 - I3C0, I3C1 Functional Slow Time Control Clock Select"]
    pub i3c01fclkstcsel: I3C01FCLKSTCSEL,
    #[doc = "0x808 - I3C0, I3C1 Functional Slow Time Control Clock Divider"]
    pub i3c01fclkstcdiv: I3C01FCLKSTCDIV,
    #[doc = "0x80c - I3C0, I3C1 Functional Slow Clock Divider"]
    pub i3c01fclksdiv: I3C01FCLKSDIV,
    #[doc = "0x810 - I3C0, I3C1 Functional Clock Divider"]
    pub i3c01fclkdiv: I3C01FCLKDIV,
    #[doc = "0x814 - I3C01 Functional Clock Select"]
    pub i3c01fclkststclksel: I3C01FCLKSTSTCLKSEL,
    _reserved90: [u8; 0x08],
    #[doc = "0x820 - Watchdog Timer 1 Functional Clock Select"]
    pub wdt1fclksel: WDT1FCLKSEL,
    _reserved91: [u8; 0x0c],
    #[doc = "0x830 - Analog Comparator 0 Clock Select"]
    pub acmp0fclksel: ACMP0FCLKSEL,
    #[doc = "0x834 - Analog comparator 0 FCLK divider"]
    pub acmp0fclkdiv: ACMP0FCLKDIV,
}
#[doc = "PSCCTL0 (rw) register accessor: an alias for `Reg<PSCCTL0_SPEC>`"]
pub type PSCCTL0 = crate::Reg<pscctl0::PSCCTL0_SPEC>;
#[doc = "Clock Control 0"]
pub mod pscctl0;
#[doc = "PSCCTL1 (rw) register accessor: an alias for `Reg<PSCCTL1_SPEC>`"]
pub type PSCCTL1 = crate::Reg<pscctl1::PSCCTL1_SPEC>;
#[doc = "Clock Control 1"]
pub mod pscctl1;
#[doc = "PSCCTL2 (rw) register accessor: an alias for `Reg<PSCCTL2_SPEC>`"]
pub type PSCCTL2 = crate::Reg<pscctl2::PSCCTL2_SPEC>;
#[doc = "Clock Control 2"]
pub mod pscctl2;
#[doc = "PSCCTL0_SET (rw) register accessor: an alias for `Reg<PSCCTL0_SET_SPEC>`"]
pub type PSCCTL0_SET = crate::Reg<pscctl0_set::PSCCTL0_SET_SPEC>;
#[doc = "Clock Set 0"]
pub mod pscctl0_set;
#[doc = "PSCCTL1_SET (rw) register accessor: an alias for `Reg<PSCCTL1_SET_SPEC>`"]
pub type PSCCTL1_SET = crate::Reg<pscctl1_set::PSCCTL1_SET_SPEC>;
#[doc = "Clock Set 1"]
pub mod pscctl1_set;
#[doc = "PSCCTL2_SET (rw) register accessor: an alias for `Reg<PSCCTL2_SET_SPEC>`"]
pub type PSCCTL2_SET = crate::Reg<pscctl2_set::PSCCTL2_SET_SPEC>;
#[doc = "Clock Set 2"]
pub mod pscctl2_set;
#[doc = "PSCCTL0_CLR (rw) register accessor: an alias for `Reg<PSCCTL0_CLR_SPEC>`"]
pub type PSCCTL0_CLR = crate::Reg<pscctl0_clr::PSCCTL0_CLR_SPEC>;
#[doc = "Clock Clear 0"]
pub mod pscctl0_clr;
#[doc = "PSCCTL1_CLR (rw) register accessor: an alias for `Reg<PSCCTL1_CLR_SPEC>`"]
pub type PSCCTL1_CLR = crate::Reg<pscctl1_clr::PSCCTL1_CLR_SPEC>;
#[doc = "Clock Clear 1"]
pub mod pscctl1_clr;
#[doc = "PSCCTL2_CLR (rw) register accessor: an alias for `Reg<PSCCTL2_CLR_SPEC>`"]
pub type PSCCTL2_CLR = crate::Reg<pscctl2_clr::PSCCTL2_CLR_SPEC>;
#[doc = "Clock Clear 2"]
pub mod pscctl2_clr;
#[doc = "AUDIOPLL0CLKSEL (rw) register accessor: an alias for `Reg<AUDIOPLL0CLKSEL_SPEC>`"]
pub type AUDIOPLL0CLKSEL = crate::Reg<audiopll0clksel::AUDIOPLL0CLKSEL_SPEC>;
#[doc = "Audio PLL0 Clock Select"]
pub mod audiopll0clksel;
#[doc = "AUDIOPLL0CTL0 (rw) register accessor: an alias for `Reg<AUDIOPLL0CTL0_SPEC>`"]
pub type AUDIOPLL0CTL0 = crate::Reg<audiopll0ctl0::AUDIOPLL0CTL0_SPEC>;
#[doc = "Audio PLL0 Control 0"]
pub mod audiopll0ctl0;
#[doc = "AUDIOPLL0LOCKTIMEDIV2 (rw) register accessor: an alias for `Reg<AUDIOPLL0LOCKTIMEDIV2_SPEC>`"]
pub type AUDIOPLL0LOCKTIMEDIV2 = crate::Reg<audiopll0locktimediv2::AUDIOPLL0LOCKTIMEDIV2_SPEC>;
#[doc = "Audio PLL0 Lock Time Divide-by-2"]
pub mod audiopll0locktimediv2;
#[doc = "AUDIOPLL0NUM (rw) register accessor: an alias for `Reg<AUDIOPLL0NUM_SPEC>`"]
pub type AUDIOPLL0NUM = crate::Reg<audiopll0num::AUDIOPLL0NUM_SPEC>;
#[doc = "Audio PLL0 Numerator"]
pub mod audiopll0num;
#[doc = "AUDIOPLL0DENOM (rw) register accessor: an alias for `Reg<AUDIOPLL0DENOM_SPEC>`"]
pub type AUDIOPLL0DENOM = crate::Reg<audiopll0denom::AUDIOPLL0DENOM_SPEC>;
#[doc = "Audio PLL0 Denominator"]
pub mod audiopll0denom;
#[doc = "AUDIOPLL0PFD (rw) register accessor: an alias for `Reg<AUDIOPLL0PFD_SPEC>`"]
pub type AUDIOPLL0PFD = crate::Reg<audiopll0pfd::AUDIOPLL0PFD_SPEC>;
#[doc = "Audio PLL0 PFD"]
pub mod audiopll0pfd;
#[doc = "AUDIOPLLCLKDIV (rw) register accessor: an alias for `Reg<AUDIOPLLCLKDIV_SPEC>`"]
pub type AUDIOPLLCLKDIV = crate::Reg<audiopllclkdiv::AUDIOPLLCLKDIV_SPEC>;
#[doc = "Audio PLL Clock Divider"]
pub mod audiopllclkdiv;
#[doc = "DSPCPUCLKDIV (rw) register accessor: an alias for `Reg<DSPCPUCLKDIV_SPEC>`"]
pub type DSPCPUCLKDIV = crate::Reg<dspcpuclkdiv::DSPCPUCLKDIV_SPEC>;
#[doc = "DSP CPU Clock Divider"]
pub mod dspcpuclkdiv;
#[doc = "DSPCPUCLKSELA (rw) register accessor: an alias for `Reg<DSPCPUCLKSELA_SPEC>`"]
pub type DSPCPUCLKSELA = crate::Reg<dspcpuclksela::DSPCPUCLKSELA_SPEC>;
#[doc = "DSP CPU Clock Select A"]
pub mod dspcpuclksela;
#[doc = "DSPCPUCLKSELB (rw) register accessor: an alias for `Reg<DSPCPUCLKSELB_SPEC>`"]
pub type DSPCPUCLKSELB = crate::Reg<dspcpuclkselb::DSPCPUCLKSELB_SPEC>;
#[doc = "DSP CPU Clock Select B"]
pub mod dspcpuclkselb;
#[doc = "OSEVENTTFCLKSEL (rw) register accessor: an alias for `Reg<OSEVENTTFCLKSEL_SPEC>`"]
pub type OSEVENTTFCLKSEL = crate::Reg<oseventtfclksel::OSEVENTTFCLKSEL_SPEC>;
#[doc = "OS Event Timer Functional Clock Select"]
pub mod oseventtfclksel;
#[doc = "FRG0CLKSEL (rw) register accessor: an alias for `Reg<FRG0CLKSEL_SPEC>`"]
pub type FRG0CLKSEL = crate::Reg<frg0clksel::FRG0CLKSEL_SPEC>;
#[doc = "Fractional Rate Generator 0 Clock Select"]
pub mod frg0clksel;
#[doc = "FRG0CTL (rw) register accessor: an alias for `Reg<FRG0CTL_SPEC>`"]
pub type FRG0CTL = crate::Reg<frg0ctl::FRG0CTL_SPEC>;
#[doc = "Fractional Rate Generator 0 Control"]
pub mod frg0ctl;
#[doc = "FC0FCLKSEL (rw) register accessor: an alias for `Reg<FC0FCLKSEL_SPEC>`"]
pub type FC0FCLKSEL = crate::Reg<fc0fclksel::FC0FCLKSEL_SPEC>;
#[doc = "Flexcomm0 Clock Select"]
pub mod fc0fclksel;
#[doc = "FRG1CLKSEL (rw) register accessor: an alias for `Reg<FRG1CLKSEL_SPEC>`"]
pub type FRG1CLKSEL = crate::Reg<frg1clksel::FRG1CLKSEL_SPEC>;
#[doc = "Fractional Rate Generator 1 Clock Select"]
pub mod frg1clksel;
#[doc = "FRG1CTL (rw) register accessor: an alias for `Reg<FRG1CTL_SPEC>`"]
pub type FRG1CTL = crate::Reg<frg1ctl::FRG1CTL_SPEC>;
#[doc = "Fractional Rate Generator 1 Control"]
pub mod frg1ctl;
#[doc = "FC1FCLKSEL (rw) register accessor: an alias for `Reg<FC1FCLKSEL_SPEC>`"]
pub type FC1FCLKSEL = crate::Reg<fc1fclksel::FC1FCLKSEL_SPEC>;
#[doc = "Flexcomm1 Clock Select"]
pub mod fc1fclksel;
#[doc = "FRG2CLKSEL (rw) register accessor: an alias for `Reg<FRG2CLKSEL_SPEC>`"]
pub type FRG2CLKSEL = crate::Reg<frg2clksel::FRG2CLKSEL_SPEC>;
#[doc = "Fractional Rate Generator 2 Clock Select"]
pub mod frg2clksel;
#[doc = "FRG2CTL (rw) register accessor: an alias for `Reg<FRG2CTL_SPEC>`"]
pub type FRG2CTL = crate::Reg<frg2ctl::FRG2CTL_SPEC>;
#[doc = "Fractional Rate Generator 2 Control"]
pub mod frg2ctl;
#[doc = "FC2FCLKSEL (rw) register accessor: an alias for `Reg<FC2FCLKSEL_SPEC>`"]
pub type FC2FCLKSEL = crate::Reg<fc2fclksel::FC2FCLKSEL_SPEC>;
#[doc = "Flexcomm2 Clock Select"]
pub mod fc2fclksel;
#[doc = "FRG3CLKSEL (rw) register accessor: an alias for `Reg<FRG3CLKSEL_SPEC>`"]
pub type FRG3CLKSEL = crate::Reg<frg3clksel::FRG3CLKSEL_SPEC>;
#[doc = "Fractional Rate Generator 3 Clock Select"]
pub mod frg3clksel;
#[doc = "FRG3CTL (rw) register accessor: an alias for `Reg<FRG3CTL_SPEC>`"]
pub type FRG3CTL = crate::Reg<frg3ctl::FRG3CTL_SPEC>;
#[doc = "Fractional Rate Generator 3 Control"]
pub mod frg3ctl;
#[doc = "FC3FCLKSEL (rw) register accessor: an alias for `Reg<FC3FCLKSEL_SPEC>`"]
pub type FC3FCLKSEL = crate::Reg<fc3fclksel::FC3FCLKSEL_SPEC>;
#[doc = "Flexcomm3 Clock Select"]
pub mod fc3fclksel;
#[doc = "FRG4CLKSEL (rw) register accessor: an alias for `Reg<FRG4CLKSEL_SPEC>`"]
pub type FRG4CLKSEL = crate::Reg<frg4clksel::FRG4CLKSEL_SPEC>;
#[doc = "Fractional Rate Generator 4 Clock Select"]
pub mod frg4clksel;
#[doc = "FRG4CTL (rw) register accessor: an alias for `Reg<FRG4CTL_SPEC>`"]
pub type FRG4CTL = crate::Reg<frg4ctl::FRG4CTL_SPEC>;
#[doc = "Fractional Rate Generator 4 Control"]
pub mod frg4ctl;
#[doc = "FC4FCLKSEL (rw) register accessor: an alias for `Reg<FC4FCLKSEL_SPEC>`"]
pub type FC4FCLKSEL = crate::Reg<fc4fclksel::FC4FCLKSEL_SPEC>;
#[doc = "Flexcomm4 Clock Select"]
pub mod fc4fclksel;
#[doc = "FRG5CLKSEL (rw) register accessor: an alias for `Reg<FRG5CLKSEL_SPEC>`"]
pub type FRG5CLKSEL = crate::Reg<frg5clksel::FRG5CLKSEL_SPEC>;
#[doc = "Fractional Rate Generator 5 Clock Select"]
pub mod frg5clksel;
#[doc = "FRG5CTL (rw) register accessor: an alias for `Reg<FRG5CTL_SPEC>`"]
pub type FRG5CTL = crate::Reg<frg5ctl::FRG5CTL_SPEC>;
#[doc = "Fractional Rate Generator 5 Control"]
pub mod frg5ctl;
#[doc = "FC5FCLKSEL (rw) register accessor: an alias for `Reg<FC5FCLKSEL_SPEC>`"]
pub type FC5FCLKSEL = crate::Reg<fc5fclksel::FC5FCLKSEL_SPEC>;
#[doc = "Flexcomm5 Clock Select"]
pub mod fc5fclksel;
#[doc = "FRG6CLKSEL (rw) register accessor: an alias for `Reg<FRG6CLKSEL_SPEC>`"]
pub type FRG6CLKSEL = crate::Reg<frg6clksel::FRG6CLKSEL_SPEC>;
#[doc = "Fractional Rate Generator 6 Clock Select"]
pub mod frg6clksel;
#[doc = "FRG6CTL (rw) register accessor: an alias for `Reg<FRG6CTL_SPEC>`"]
pub type FRG6CTL = crate::Reg<frg6ctl::FRG6CTL_SPEC>;
#[doc = "Fractional Rate Generator 6 Control"]
pub mod frg6ctl;
#[doc = "FC6FCLKSEL (rw) register accessor: an alias for `Reg<FC6FCLKSEL_SPEC>`"]
pub type FC6FCLKSEL = crate::Reg<fc6fclksel::FC6FCLKSEL_SPEC>;
#[doc = "Flexcomm6 Clock Select"]
pub mod fc6fclksel;
#[doc = "FRG7CLKSEL (rw) register accessor: an alias for `Reg<FRG7CLKSEL_SPEC>`"]
pub type FRG7CLKSEL = crate::Reg<frg7clksel::FRG7CLKSEL_SPEC>;
#[doc = "Fractional Rate Generator 7 Clock Select"]
pub mod frg7clksel;
#[doc = "FRG7CTL (rw) register accessor: an alias for `Reg<FRG7CTL_SPEC>`"]
pub type FRG7CTL = crate::Reg<frg7ctl::FRG7CTL_SPEC>;
#[doc = "Fractional Rate Generator 7 Control"]
pub mod frg7ctl;
#[doc = "FC7FCLKSEL (rw) register accessor: an alias for `Reg<FC7FCLKSEL_SPEC>`"]
pub type FC7FCLKSEL = crate::Reg<fc7fclksel::FC7FCLKSEL_SPEC>;
#[doc = "Flexcomm7 Clock Select"]
pub mod fc7fclksel;
#[doc = "FRG8CLKSEL (rw) register accessor: an alias for `Reg<FRG8CLKSEL_SPEC>`"]
pub type FRG8CLKSEL = crate::Reg<frg8clksel::FRG8CLKSEL_SPEC>;
#[doc = "Fractional Rate Generator 8 Clock Select"]
pub mod frg8clksel;
#[doc = "FRG8CTL (rw) register accessor: an alias for `Reg<FRG8CTL_SPEC>`"]
pub type FRG8CTL = crate::Reg<frg8ctl::FRG8CTL_SPEC>;
#[doc = "Fractional Rate Generator 8 Control"]
pub mod frg8ctl;
#[doc = "FC8FCLKSEL (rw) register accessor: an alias for `Reg<FC8FCLKSEL_SPEC>`"]
pub type FC8FCLKSEL = crate::Reg<fc8fclksel::FC8FCLKSEL_SPEC>;
#[doc = "Flexcomm8 Clock Select"]
pub mod fc8fclksel;
#[doc = "FRG9CLKSEL (rw) register accessor: an alias for `Reg<FRG9CLKSEL_SPEC>`"]
pub type FRG9CLKSEL = crate::Reg<frg9clksel::FRG9CLKSEL_SPEC>;
#[doc = "Fractional Rate Generator 9 Clock Select"]
pub mod frg9clksel;
#[doc = "FRG9CTL (rw) register accessor: an alias for `Reg<FRG9CTL_SPEC>`"]
pub type FRG9CTL = crate::Reg<frg9ctl::FRG9CTL_SPEC>;
#[doc = "Fractional Rate Generator 9 Control"]
pub mod frg9ctl;
#[doc = "FC9FCLKSEL (rw) register accessor: an alias for `Reg<FC9FCLKSEL_SPEC>`"]
pub type FC9FCLKSEL = crate::Reg<fc9fclksel::FC9FCLKSEL_SPEC>;
#[doc = "Flexcomm9 Clock Select"]
pub mod fc9fclksel;
#[doc = "FRG10CLKSEL (rw) register accessor: an alias for `Reg<FRG10CLKSEL_SPEC>`"]
pub type FRG10CLKSEL = crate::Reg<frg10clksel::FRG10CLKSEL_SPEC>;
#[doc = "Fractional Rate Generator 10 Clock Select"]
pub mod frg10clksel;
#[doc = "FRG10CTL (rw) register accessor: an alias for `Reg<FRG10CTL_SPEC>`"]
pub type FRG10CTL = crate::Reg<frg10ctl::FRG10CTL_SPEC>;
#[doc = "Fractional Rate Generator 10 Control"]
pub mod frg10ctl;
#[doc = "FC10FCLKSEL (rw) register accessor: an alias for `Reg<FC10FCLKSEL_SPEC>`"]
pub type FC10FCLKSEL = crate::Reg<fc10fclksel::FC10FCLKSEL_SPEC>;
#[doc = "Flexcomm10 Clock Select"]
pub mod fc10fclksel;
#[doc = "FRG11CLKSEL (rw) register accessor: an alias for `Reg<FRG11CLKSEL_SPEC>`"]
pub type FRG11CLKSEL = crate::Reg<frg11clksel::FRG11CLKSEL_SPEC>;
#[doc = "Fractional Rate Generator 11 Clock Select"]
pub mod frg11clksel;
#[doc = "FRG11CTL (rw) register accessor: an alias for `Reg<FRG11CTL_SPEC>`"]
pub type FRG11CTL = crate::Reg<frg11ctl::FRG11CTL_SPEC>;
#[doc = "Fractional Rate Generator 11 Control"]
pub mod frg11ctl;
#[doc = "FC11FCLKSEL (rw) register accessor: an alias for `Reg<FC11FCLKSEL_SPEC>`"]
pub type FC11FCLKSEL = crate::Reg<fc11fclksel::FC11FCLKSEL_SPEC>;
#[doc = "Flexcomm11 Clock Select"]
pub mod fc11fclksel;
#[doc = "FRG12CLKSEL (rw) register accessor: an alias for `Reg<FRG12CLKSEL_SPEC>`"]
pub type FRG12CLKSEL = crate::Reg<frg12clksel::FRG12CLKSEL_SPEC>;
#[doc = "Fractional Rate Generator 12 Clock Select"]
pub mod frg12clksel;
#[doc = "FRG12CTL (rw) register accessor: an alias for `Reg<FRG12CTL_SPEC>`"]
pub type FRG12CTL = crate::Reg<frg12ctl::FRG12CTL_SPEC>;
#[doc = "Fractional Rate Generator 12 Control"]
pub mod frg12ctl;
#[doc = "FC12FCLKSEL (rw) register accessor: an alias for `Reg<FC12FCLKSEL_SPEC>`"]
pub type FC12FCLKSEL = crate::Reg<fc12fclksel::FC12FCLKSEL_SPEC>;
#[doc = "Flexcomm12 Clock Select"]
pub mod fc12fclksel;
#[doc = "FRG13CLKSEL (rw) register accessor: an alias for `Reg<FRG13CLKSEL_SPEC>`"]
pub type FRG13CLKSEL = crate::Reg<frg13clksel::FRG13CLKSEL_SPEC>;
#[doc = "Fractional Rate Generator 13 Clock Select"]
pub mod frg13clksel;
#[doc = "FRG13CTL (rw) register accessor: an alias for `Reg<FRG13CTL_SPEC>`"]
pub type FRG13CTL = crate::Reg<frg13ctl::FRG13CTL_SPEC>;
#[doc = "Fractional Rate Generator 13 Control"]
pub mod frg13ctl;
#[doc = "FC13FCLKSEL (rw) register accessor: an alias for `Reg<FC13FCLKSEL_SPEC>`"]
pub type FC13FCLKSEL = crate::Reg<fc13fclksel::FC13FCLKSEL_SPEC>;
#[doc = "Flexcomm13 Clock Select"]
pub mod fc13fclksel;
#[doc = "FRG14CLKSEL (rw) register accessor: an alias for `Reg<FRG14CLKSEL_SPEC>`"]
pub type FRG14CLKSEL = crate::Reg<frg14clksel::FRG14CLKSEL_SPEC>;
#[doc = "Fractional Rate Generator 14 Clock Select"]
pub mod frg14clksel;
#[doc = "FRG14CTL (rw) register accessor: an alias for `Reg<FRG14CTL_SPEC>`"]
pub type FRG14CTL = crate::Reg<frg14ctl::FRG14CTL_SPEC>;
#[doc = "Fractional Rate Generator 14 Control"]
pub mod frg14ctl;
#[doc = "FC14FCLKSEL (rw) register accessor: an alias for `Reg<FC14FCLKSEL_SPEC>`"]
pub type FC14FCLKSEL = crate::Reg<fc14fclksel::FC14FCLKSEL_SPEC>;
#[doc = "Flexcomm14 Clock Select"]
pub mod fc14fclksel;
#[doc = "FRG15CLKSEL (rw) register accessor: an alias for `Reg<FRG15CLKSEL_SPEC>`"]
pub type FRG15CLKSEL = crate::Reg<frg15clksel::FRG15CLKSEL_SPEC>;
#[doc = "Fractional Rate Generator 15 Clock Select"]
pub mod frg15clksel;
#[doc = "FRG15CTL (rw) register accessor: an alias for `Reg<FRG15CTL_SPEC>`"]
pub type FRG15CTL = crate::Reg<frg15ctl::FRG15CTL_SPEC>;
#[doc = "Fractional Rate Generator 15 Control"]
pub mod frg15ctl;
#[doc = "FC15FCLKSEL (rw) register accessor: an alias for `Reg<FC15FCLKSEL_SPEC>`"]
pub type FC15FCLKSEL = crate::Reg<fc15fclksel::FC15FCLKSEL_SPEC>;
#[doc = "Flexcomm15 Clock Select"]
pub mod fc15fclksel;
#[doc = "FRG16CLKSEL (rw) register accessor: an alias for `Reg<FRG16CLKSEL_SPEC>`"]
pub type FRG16CLKSEL = crate::Reg<frg16clksel::FRG16CLKSEL_SPEC>;
#[doc = "Fractional Rate Generator 16 Clock Select"]
pub mod frg16clksel;
#[doc = "FRG16CTL (rw) register accessor: an alias for `Reg<FRG16CTL_SPEC>`"]
pub type FRG16CTL = crate::Reg<frg16ctl::FRG16CTL_SPEC>;
#[doc = "Fractional Rate Generator 16 Control"]
pub mod frg16ctl;
#[doc = "FC16FCLKSEL (rw) register accessor: an alias for `Reg<FC16FCLKSEL_SPEC>`"]
pub type FC16FCLKSEL = crate::Reg<fc16fclksel::FC16FCLKSEL_SPEC>;
#[doc = "Flexcomm16 Clock Select"]
pub mod fc16fclksel;
#[doc = "FRG17CLKSEL (rw) register accessor: an alias for `Reg<FRG17CLKSEL_SPEC>`"]
pub type FRG17CLKSEL = crate::Reg<frg17clksel::FRG17CLKSEL_SPEC>;
#[doc = "Fractional Rate Generator 17 Clock Select"]
pub mod frg17clksel;
#[doc = "FRG17CTL (rw) register accessor: an alias for `Reg<FRG17CTL_SPEC>`"]
pub type FRG17CTL = crate::Reg<frg17ctl::FRG17CTL_SPEC>;
#[doc = "Fractional Rate Generator 17 Control"]
pub mod frg17ctl;
#[doc = "FLEXIOCLKSEL (rw) register accessor: an alias for `Reg<FLEXIOCLKSEL_SPEC>`"]
pub type FLEXIOCLKSEL = crate::Reg<flexioclksel::FLEXIOCLKSEL_SPEC>;
#[doc = "FlexIO Clock Select"]
pub mod flexioclksel;
#[doc = "FLEXIOCLKDIV (rw) register accessor: an alias for `Reg<FLEXIOCLKDIV_SPEC>`"]
pub type FLEXIOCLKDIV = crate::Reg<flexioclkdiv::FLEXIOCLKDIV_SPEC>;
#[doc = "FlexIO Clock Divider"]
pub mod flexioclkdiv;
#[doc = "FRGPLLCLKDIV (rw) register accessor: an alias for `Reg<FRGPLLCLKDIV_SPEC>`"]
pub type FRGPLLCLKDIV = crate::Reg<frgpllclkdiv::FRGPLLCLKDIV_SPEC>;
#[doc = "Fractional Rate Generator PLL Clock Divider"]
pub mod frgpllclkdiv;
#[doc = "DMIC0FCLKSEL (rw) register accessor: an alias for `Reg<DMIC0FCLKSEL_SPEC>`"]
pub type DMIC0FCLKSEL = crate::Reg<dmic0fclksel::DMIC0FCLKSEL_SPEC>;
#[doc = "DMIC0 Functional Clock Select"]
pub mod dmic0fclksel;
#[doc = "DMIC0FCLKDIV (rw) register accessor: an alias for `Reg<DMIC0FCLKDIV_SPEC>`"]
pub type DMIC0FCLKDIV = crate::Reg<dmic0fclkdiv::DMIC0FCLKDIV_SPEC>;
#[doc = "DMIC0 Functional Clock Divider"]
pub mod dmic0fclkdiv;
#[doc = "CT32BITFCLKSEL (rw) register accessor: an alias for `Reg<CT32BITFCLKSEL_SPEC>`"]
pub type CT32BITFCLKSEL = crate::Reg<ct32bitfclksel::CT32BITFCLKSEL_SPEC>;
#[doc = "CT32BIT bit timer index Functional Clock Select"]
pub mod ct32bitfclksel;
#[doc = "AUDIOMCLKSEL (rw) register accessor: an alias for `Reg<AUDIOMCLKSEL_SPEC>`"]
pub type AUDIOMCLKSEL = crate::Reg<audiomclksel::AUDIOMCLKSEL_SPEC>;
#[doc = "Audio MCLK Clock Select"]
pub mod audiomclksel;
#[doc = "AUDIOMCLKDIV (rw) register accessor: an alias for `Reg<AUDIOMCLKDIV_SPEC>`"]
pub type AUDIOMCLKDIV = crate::Reg<audiomclkdiv::AUDIOMCLKDIV_SPEC>;
#[doc = "Audio MCLK Clock Divider"]
pub mod audiomclkdiv;
#[doc = "CLKOUTSEL0 (rw) register accessor: an alias for `Reg<CLKOUTSEL0_SPEC>`"]
pub type CLKOUTSEL0 = crate::Reg<clkoutsel0::CLKOUTSEL0_SPEC>;
#[doc = "CLKOUT Clock Select 0"]
pub mod clkoutsel0;
#[doc = "CLKOUTSEL1 (rw) register accessor: an alias for `Reg<CLKOUTSEL1_SPEC>`"]
pub type CLKOUTSEL1 = crate::Reg<clkoutsel1::CLKOUTSEL1_SPEC>;
#[doc = "CLKOUT Clock Select 1"]
pub mod clkoutsel1;
#[doc = "CLKOUTFCLKDIV (rw) register accessor: an alias for `Reg<CLKOUTFCLKDIV_SPEC>`"]
pub type CLKOUTFCLKDIV = crate::Reg<clkoutfclkdiv::CLKOUTFCLKDIV_SPEC>;
#[doc = "CLKOUT Functional Clock Divider"]
pub mod clkoutfclkdiv;
#[doc = "I3C01FCLKSEL (rw) register accessor: an alias for `Reg<I3C01FCLKSEL_SPEC>`"]
pub type I3C01FCLKSEL = crate::Reg<i3c01fclksel::I3C01FCLKSEL_SPEC>;
#[doc = "I3C0, I3C1 Functional Clock Select"]
pub mod i3c01fclksel;
#[doc = "I3C01FCLKSTCSEL (rw) register accessor: an alias for `Reg<I3C01FCLKSTCSEL_SPEC>`"]
pub type I3C01FCLKSTCSEL = crate::Reg<i3c01fclkstcsel::I3C01FCLKSTCSEL_SPEC>;
#[doc = "I3C0, I3C1 Functional Slow Time Control Clock Select"]
pub mod i3c01fclkstcsel;
#[doc = "I3C01FCLKSTCDIV (rw) register accessor: an alias for `Reg<I3C01FCLKSTCDIV_SPEC>`"]
pub type I3C01FCLKSTCDIV = crate::Reg<i3c01fclkstcdiv::I3C01FCLKSTCDIV_SPEC>;
#[doc = "I3C0, I3C1 Functional Slow Time Control Clock Divider"]
pub mod i3c01fclkstcdiv;
#[doc = "I3C01FCLKSDIV (rw) register accessor: an alias for `Reg<I3C01FCLKSDIV_SPEC>`"]
pub type I3C01FCLKSDIV = crate::Reg<i3c01fclksdiv::I3C01FCLKSDIV_SPEC>;
#[doc = "I3C0, I3C1 Functional Slow Clock Divider"]
pub mod i3c01fclksdiv;
#[doc = "I3C01FCLKDIV (rw) register accessor: an alias for `Reg<I3C01FCLKDIV_SPEC>`"]
pub type I3C01FCLKDIV = crate::Reg<i3c01fclkdiv::I3C01FCLKDIV_SPEC>;
#[doc = "I3C0, I3C1 Functional Clock Divider"]
pub mod i3c01fclkdiv;
#[doc = "I3C01FCLKSTSTCLKSEL (rw) register accessor: an alias for `Reg<I3C01FCLKSTSTCLKSEL_SPEC>`"]
pub type I3C01FCLKSTSTCLKSEL = crate::Reg<i3c01fclkststclksel::I3C01FCLKSTSTCLKSEL_SPEC>;
#[doc = "I3C01 Functional Clock Select"]
pub mod i3c01fclkststclksel;
#[doc = "WDT1FCLKSEL (rw) register accessor: an alias for `Reg<WDT1FCLKSEL_SPEC>`"]
pub type WDT1FCLKSEL = crate::Reg<wdt1fclksel::WDT1FCLKSEL_SPEC>;
#[doc = "Watchdog Timer 1 Functional Clock Select"]
pub mod wdt1fclksel;
#[doc = "ACMP0FCLKSEL (rw) register accessor: an alias for `Reg<ACMP0FCLKSEL_SPEC>`"]
pub type ACMP0FCLKSEL = crate::Reg<acmp0fclksel::ACMP0FCLKSEL_SPEC>;
#[doc = "Analog Comparator 0 Clock Select"]
pub mod acmp0fclksel;
#[doc = "ACMP0FCLKDIV (rw) register accessor: an alias for `Reg<ACMP0FCLKDIV_SPEC>`"]
pub type ACMP0FCLKDIV = crate::Reg<acmp0fclkdiv::ACMP0FCLKDIV_SPEC>;
#[doc = "Analog comparator 0 FCLK divider"]
pub mod acmp0fclkdiv;
