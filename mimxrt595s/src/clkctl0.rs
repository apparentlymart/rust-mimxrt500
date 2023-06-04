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
    #[doc = "0x40 - Clock Control 0 Set"]
    pub pscctl0_set: PSCCTL0_SET,
    #[doc = "0x44 - Clock Control 1 Set"]
    pub pscctl1_set: PSCCTL1_SET,
    #[doc = "0x48 - Clock Control 2 Set"]
    pub pscctl2_set: PSCCTL2_SET,
    _reserved6: [u8; 0x24],
    #[doc = "0x70 - Clock Control 0 Clear"]
    pub pscctl0_clr: PSCCTL0_CLR,
    #[doc = "0x74 - Clock Control 1 Clear"]
    pub pscctl1_clr: PSCCTL1_CLR,
    #[doc = "0x78 - Clock Control 2 Clear"]
    pub pscctl2_clr: PSCCTL2_CLR,
    _reserved9: [u8; 0x04],
    #[doc = "0x80 - Free Running Oscillator Control"]
    pub fro_control: FRO_CONTROL,
    #[doc = "0x84 - Free Running Oscillator Captured Value"]
    pub fro_capval: FRO_CAPVAL,
    _reserved11: [u8; 0x04],
    #[doc = "0x8c - Free Running Oscillator Trim"]
    pub fro_rdtrim: FRO_RDTRIM,
    #[doc = "0x90 - Free Running OscillatorSC Trim"]
    pub fro_sctrim: FRO_SCTRIM,
    _reserved13: [u8; 0x74],
    #[doc = "0x108 - FRO Clock Divider"]
    pub frodivsel: FRODIVSEL,
    #[doc = "0x10c - FRO Clock Status"]
    pub froclkstatus: FROCLKSTATUS,
    #[doc = "0x110 - FRO Enable Register"]
    pub frodivoen: FRODIVOEN,
    _reserved16: [u8; 0x1c],
    #[doc = "0x130 - Low Frequency Clock Divider"]
    pub lowfreqclkdiv: LOWFREQCLKDIV,
    _reserved17: [u8; 0x2c],
    #[doc = "0x160 - System Oscillator Control 0"]
    pub sysoscctl0: SYSOSCCTL0,
    _reserved18: [u8; 0x04],
    #[doc = "0x168 - OSC Clock Source Select"]
    pub sysoscbypass: SYSOSCBYPASS,
    _reserved19: [u8; 0x24],
    #[doc = "0x190 - Low Power Oscillator Control 0"]
    pub lposcctl0: LPOSCCTL0,
    _reserved20: [u8; 0x2c],
    #[doc = "0x1c0 - 32 KHz Oscillator Control 0"]
    pub osc32khzctl0: OSC32KHZCTL0,
    _reserved21: [u8; 0x3c],
    #[doc = "0x200 - System PLL 0 Clock Select"]
    pub syspll0clksel: SYSPLL0CLKSEL,
    #[doc = "0x204 - System PLL0 Control 0"]
    pub syspll0ctl0: SYSPLL0CTL0,
    _reserved23: [u8; 0x04],
    #[doc = "0x20c - System PLL0 Lock Time Div2"]
    pub syspll0locktimediv2: SYSPLL0LOCKTIMEDIV2,
    #[doc = "0x210 - System PLL0 Numerator"]
    pub syspll0num: SYSPLL0NUM,
    #[doc = "0x214 - System PLL0 Denominator"]
    pub syspll0denom: SYSPLL0DENOM,
    #[doc = "0x218 - System PLL0 PFD"]
    pub syspll0pfd: SYSPLL0PFD,
    _reserved27: [u8; 0x24],
    #[doc = "0x240 - Main PLL Clock Divider"]
    pub mainpllclkdiv: MAINPLLCLKDIV,
    #[doc = "0x244 - DSP PLL Clock Divider"]
    pub dsppllclkdiv: DSPPLLCLKDIV,
    #[doc = "0x248 - AUX0 PLL Clock Divider"]
    pub aux0pllclkdiv: AUX0PLLCLKDIV,
    #[doc = "0x24c - AUX1 PLL Clock Divider"]
    pub aux1pllclkdiv: AUX1PLLCLKDIV,
    _reserved31: [u8; 0x01b0],
    #[doc = "0x400 - System CPU AHB Clock Divider"]
    pub syscpuahbclkdiv: SYSCPUAHBCLKDIV,
    _reserved32: [u8; 0x2c],
    #[doc = "0x430 - Main Clock Select A"]
    pub mainclksela: MAINCLKSELA,
    #[doc = "0x434 - Main Clock Select B"]
    pub mainclkselb: MAINCLKSELB,
    _reserved34: [u8; 0xc8],
    #[doc = "0x500 - PFC divider 0 (trace clock)"]
    pub pfc0div: PFC0DIV,
    #[doc = "0x504 - PFC divider 1 (USB HS PHY bus clock)"]
    pub pfc1div: PFC1DIV,
    _reserved36: [u8; 0x0118],
    #[doc = "0x620 - FlexSPI0 Functional Clock Select"]
    pub flexspi0fclksel: FLEXSPI0FCLKSEL,
    #[doc = "0x624 - FlexSPI0 Functional Clock Divider"]
    pub flexspi0fclkdiv: FLEXSPI0FCLKDIV,
    _reserved38: [u8; 0x08],
    #[doc = "0x630 - FlexSPI1 Functional Clock Select"]
    pub flexspi1fclksel: FLEXSPI1FCLKSEL,
    #[doc = "0x634 - FlexSPI1 Functional Clock Divider"]
    pub flexspi1fclkdiv: FLEXSPI1FCLKDIV,
    _reserved40: [u8; 0x08],
    #[doc = "0x640 - SCT Functional Clock Select"]
    pub sctfclksel: SCTFCLKSEL,
    #[doc = "0x644 - SCT Functional Clock Divider"]
    pub sctin7clkdiv: SCTIN7CLKDIV,
    _reserved42: [u8; 0x18],
    #[doc = "0x660 - High Speed USB Functional Clock Select"]
    pub usbhsfclksel: USBHSFCLKSEL,
    #[doc = "0x664 - High Speed USB Functional Clock Divider"]
    pub usbhsfclkdiv: USBHSFCLKDIV,
    _reserved44: [u8; 0x18],
    #[doc = "0x680 - SDIO0 Functional Clock Select"]
    pub sdio0fclksel: SDIO0FCLKSEL,
    #[doc = "0x684 - SDIO0 Functional Clock Divider"]
    pub sdio0fclkdiv: SDIO0FCLKDIV,
    _reserved46: [u8; 0x08],
    #[doc = "0x690 - SDIO1 Functional Clock Select"]
    pub sdio1fclksel: SDIO1FCLKSEL,
    #[doc = "0x694 - SDIO1 Functional Clock Divider"]
    pub sdio1fclkdiv: SDIO1FCLKDIV,
    _reserved48: [u8; 0x38],
    #[doc = "0x6d0 - ADC0 Functional Clock Select 0"]
    pub adc0fclksel0: ADC0FCLKSEL0,
    #[doc = "0x6d4 - ADC0 Functional Clock Select 1"]
    pub adc0fclksel1: ADC0FCLKSEL1,
    #[doc = "0x6d8 - ADC0 Functional Clock Divider"]
    pub adc0fclkdiv: ADC0FCLKDIV,
    _reserved51: [u8; 0x24],
    #[doc = "0x700 - UTICK Functional Clock Select"]
    pub utickfclksel: UTICKFCLKSEL,
    _reserved52: [u8; 0x1c],
    #[doc = "0x720 - WDT0 Functional Clock Select"]
    pub wdt0fclksel: WDT0FCLKSEL,
    _reserved53: [u8; 0x0c],
    #[doc = "0x730 - 32 KHz Wake Clock Source Select"]
    pub a32khzwakeclksel: A32KHZWAKECLKSEL,
    #[doc = "0x734 - 32 KHz Wake Clock Divider"]
    pub a32khzwakeclkdiv: A32KHZWAKECLKDIV,
    _reserved55: [u8; 0x28],
    #[doc = "0x760 - SYSTICK Functional Clock Select"]
    pub systickfclksel: SYSTICKFCLKSEL,
    #[doc = "0x764 - SYSTICK Functional Clock Divider"]
    pub systickfclkdiv: SYSTICKFCLKDIV,
    _reserved57: [u8; 0x08],
    #[doc = "0x770 - MIPI-DSI PHY Clock Select"]
    pub dphyclksel: DPHYCLKSEL,
    #[doc = "0x774 - MIPI-DSI PHY Clock Divider"]
    pub dphyclkdiv: DPHYCLKDIV,
    #[doc = "0x778 - MIPI-DSI DPHY Escape Mode Clock Select"]
    pub dphyescclksel: DPHYESCCLKSEL,
    #[doc = "0x77c - MIPI-DSI DPHY Escape Mode Receive Clock Divider"]
    pub dphyescrxclkdiv: DPHYESCRXCLKDIV,
    #[doc = "0x780 - MIPI-DSI DPHY Escape Mode Tramsmit Clock Divider"]
    pub dphyesctxclkdiv: DPHYESCTXCLKDIV,
    _reserved62: [u8; 0x0c],
    #[doc = "0x790 - GPU Clock Select"]
    pub gpuclksel: GPUCLKSEL,
    #[doc = "0x794 - GPU Clock Divider"]
    pub gpuclkdiv: GPUCLKDIV,
    _reserved64: [u8; 0x08],
    #[doc = "0x7a0 - LCDIF Pixel Clock Select"]
    pub dcpixelclksel: DCPIXELCLKSEL,
    #[doc = "0x7a4 - LCDIF Pixel Clock Divider"]
    pub dcpixelclkdiv: DCPIXELCLKDIV,
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
#[doc = "PSCCTL0_SET (w) register accessor: an alias for `Reg<PSCCTL0_SET_SPEC>`"]
pub type PSCCTL0_SET = crate::Reg<pscctl0_set::PSCCTL0_SET_SPEC>;
#[doc = "Clock Control 0 Set"]
pub mod pscctl0_set;
#[doc = "PSCCTL1_SET (w) register accessor: an alias for `Reg<PSCCTL1_SET_SPEC>`"]
pub type PSCCTL1_SET = crate::Reg<pscctl1_set::PSCCTL1_SET_SPEC>;
#[doc = "Clock Control 1 Set"]
pub mod pscctl1_set;
#[doc = "PSCCTL2_SET (w) register accessor: an alias for `Reg<PSCCTL2_SET_SPEC>`"]
pub type PSCCTL2_SET = crate::Reg<pscctl2_set::PSCCTL2_SET_SPEC>;
#[doc = "Clock Control 2 Set"]
pub mod pscctl2_set;
#[doc = "PSCCTL0_CLR (w) register accessor: an alias for `Reg<PSCCTL0_CLR_SPEC>`"]
pub type PSCCTL0_CLR = crate::Reg<pscctl0_clr::PSCCTL0_CLR_SPEC>;
#[doc = "Clock Control 0 Clear"]
pub mod pscctl0_clr;
#[doc = "PSCCTL1_CLR (w) register accessor: an alias for `Reg<PSCCTL1_CLR_SPEC>`"]
pub type PSCCTL1_CLR = crate::Reg<pscctl1_clr::PSCCTL1_CLR_SPEC>;
#[doc = "Clock Control 1 Clear"]
pub mod pscctl1_clr;
#[doc = "PSCCTL2_CLR (w) register accessor: an alias for `Reg<PSCCTL2_CLR_SPEC>`"]
pub type PSCCTL2_CLR = crate::Reg<pscctl2_clr::PSCCTL2_CLR_SPEC>;
#[doc = "Clock Control 2 Clear"]
pub mod pscctl2_clr;
#[doc = "FRO_CONTROL (rw) register accessor: an alias for `Reg<FRO_CONTROL_SPEC>`"]
pub type FRO_CONTROL = crate::Reg<fro_control::FRO_CONTROL_SPEC>;
#[doc = "Free Running Oscillator Control"]
pub mod fro_control;
#[doc = "FRO_CAPVAL (r) register accessor: an alias for `Reg<FRO_CAPVAL_SPEC>`"]
pub type FRO_CAPVAL = crate::Reg<fro_capval::FRO_CAPVAL_SPEC>;
#[doc = "Free Running Oscillator Captured Value"]
pub mod fro_capval;
#[doc = "FRO_RDTRIM (rw) register accessor: an alias for `Reg<FRO_RDTRIM_SPEC>`"]
pub type FRO_RDTRIM = crate::Reg<fro_rdtrim::FRO_RDTRIM_SPEC>;
#[doc = "Free Running Oscillator Trim"]
pub mod fro_rdtrim;
#[doc = "FRO_SCTRIM (rw) register accessor: an alias for `Reg<FRO_SCTRIM_SPEC>`"]
pub type FRO_SCTRIM = crate::Reg<fro_sctrim::FRO_SCTRIM_SPEC>;
#[doc = "Free Running OscillatorSC Trim"]
pub mod fro_sctrim;
#[doc = "FRODIVSEL (rw) register accessor: an alias for `Reg<FRODIVSEL_SPEC>`"]
pub type FRODIVSEL = crate::Reg<frodivsel::FRODIVSEL_SPEC>;
#[doc = "FRO Clock Divider"]
pub mod frodivsel;
#[doc = "FROCLKSTATUS (r) register accessor: an alias for `Reg<FROCLKSTATUS_SPEC>`"]
pub type FROCLKSTATUS = crate::Reg<froclkstatus::FROCLKSTATUS_SPEC>;
#[doc = "FRO Clock Status"]
pub mod froclkstatus;
#[doc = "FRODIVOEN (rw) register accessor: an alias for `Reg<FRODIVOEN_SPEC>`"]
pub type FRODIVOEN = crate::Reg<frodivoen::FRODIVOEN_SPEC>;
#[doc = "FRO Enable Register"]
pub mod frodivoen;
#[doc = "LOWFREQCLKDIV (rw) register accessor: an alias for `Reg<LOWFREQCLKDIV_SPEC>`"]
pub type LOWFREQCLKDIV = crate::Reg<lowfreqclkdiv::LOWFREQCLKDIV_SPEC>;
#[doc = "Low Frequency Clock Divider"]
pub mod lowfreqclkdiv;
#[doc = "SYSOSCCTL0 (rw) register accessor: an alias for `Reg<SYSOSCCTL0_SPEC>`"]
pub type SYSOSCCTL0 = crate::Reg<sysoscctl0::SYSOSCCTL0_SPEC>;
#[doc = "System Oscillator Control 0"]
pub mod sysoscctl0;
#[doc = "SYSOSCBYPASS (rw) register accessor: an alias for `Reg<SYSOSCBYPASS_SPEC>`"]
pub type SYSOSCBYPASS = crate::Reg<sysoscbypass::SYSOSCBYPASS_SPEC>;
#[doc = "OSC Clock Source Select"]
pub mod sysoscbypass;
#[doc = "LPOSCCTL0 (rw) register accessor: an alias for `Reg<LPOSCCTL0_SPEC>`"]
pub type LPOSCCTL0 = crate::Reg<lposcctl0::LPOSCCTL0_SPEC>;
#[doc = "Low Power Oscillator Control 0"]
pub mod lposcctl0;
#[doc = "OSC32KHZCTL0 (rw) register accessor: an alias for `Reg<OSC32KHZCTL0_SPEC>`"]
pub type OSC32KHZCTL0 = crate::Reg<osc32khzctl0::OSC32KHZCTL0_SPEC>;
#[doc = "32 KHz Oscillator Control 0"]
pub mod osc32khzctl0;
#[doc = "SYSPLL0CLKSEL (rw) register accessor: an alias for `Reg<SYSPLL0CLKSEL_SPEC>`"]
pub type SYSPLL0CLKSEL = crate::Reg<syspll0clksel::SYSPLL0CLKSEL_SPEC>;
#[doc = "System PLL 0 Clock Select"]
pub mod syspll0clksel;
#[doc = "SYSPLL0CTL0 (rw) register accessor: an alias for `Reg<SYSPLL0CTL0_SPEC>`"]
pub type SYSPLL0CTL0 = crate::Reg<syspll0ctl0::SYSPLL0CTL0_SPEC>;
#[doc = "System PLL0 Control 0"]
pub mod syspll0ctl0;
#[doc = "SYSPLL0LOCKTIMEDIV2 (rw) register accessor: an alias for `Reg<SYSPLL0LOCKTIMEDIV2_SPEC>`"]
pub type SYSPLL0LOCKTIMEDIV2 = crate::Reg<syspll0locktimediv2::SYSPLL0LOCKTIMEDIV2_SPEC>;
#[doc = "System PLL0 Lock Time Div2"]
pub mod syspll0locktimediv2;
#[doc = "SYSPLL0NUM (rw) register accessor: an alias for `Reg<SYSPLL0NUM_SPEC>`"]
pub type SYSPLL0NUM = crate::Reg<syspll0num::SYSPLL0NUM_SPEC>;
#[doc = "System PLL0 Numerator"]
pub mod syspll0num;
#[doc = "SYSPLL0DENOM (rw) register accessor: an alias for `Reg<SYSPLL0DENOM_SPEC>`"]
pub type SYSPLL0DENOM = crate::Reg<syspll0denom::SYSPLL0DENOM_SPEC>;
#[doc = "System PLL0 Denominator"]
pub mod syspll0denom;
#[doc = "SYSPLL0PFD (rw) register accessor: an alias for `Reg<SYSPLL0PFD_SPEC>`"]
pub type SYSPLL0PFD = crate::Reg<syspll0pfd::SYSPLL0PFD_SPEC>;
#[doc = "System PLL0 PFD"]
pub mod syspll0pfd;
#[doc = "MAINPLLCLKDIV (rw) register accessor: an alias for `Reg<MAINPLLCLKDIV_SPEC>`"]
pub type MAINPLLCLKDIV = crate::Reg<mainpllclkdiv::MAINPLLCLKDIV_SPEC>;
#[doc = "Main PLL Clock Divider"]
pub mod mainpllclkdiv;
#[doc = "DSPPLLCLKDIV (rw) register accessor: an alias for `Reg<DSPPLLCLKDIV_SPEC>`"]
pub type DSPPLLCLKDIV = crate::Reg<dsppllclkdiv::DSPPLLCLKDIV_SPEC>;
#[doc = "DSP PLL Clock Divider"]
pub mod dsppllclkdiv;
#[doc = "AUX0PLLCLKDIV (rw) register accessor: an alias for `Reg<AUX0PLLCLKDIV_SPEC>`"]
pub type AUX0PLLCLKDIV = crate::Reg<aux0pllclkdiv::AUX0PLLCLKDIV_SPEC>;
#[doc = "AUX0 PLL Clock Divider"]
pub mod aux0pllclkdiv;
#[doc = "AUX1PLLCLKDIV (rw) register accessor: an alias for `Reg<AUX1PLLCLKDIV_SPEC>`"]
pub type AUX1PLLCLKDIV = crate::Reg<aux1pllclkdiv::AUX1PLLCLKDIV_SPEC>;
#[doc = "AUX1 PLL Clock Divider"]
pub mod aux1pllclkdiv;
#[doc = "SYSCPUAHBCLKDIV (rw) register accessor: an alias for `Reg<SYSCPUAHBCLKDIV_SPEC>`"]
pub type SYSCPUAHBCLKDIV = crate::Reg<syscpuahbclkdiv::SYSCPUAHBCLKDIV_SPEC>;
#[doc = "System CPU AHB Clock Divider"]
pub mod syscpuahbclkdiv;
#[doc = "MAINCLKSELA (rw) register accessor: an alias for `Reg<MAINCLKSELA_SPEC>`"]
pub type MAINCLKSELA = crate::Reg<mainclksela::MAINCLKSELA_SPEC>;
#[doc = "Main Clock Select A"]
pub mod mainclksela;
#[doc = "MAINCLKSELB (rw) register accessor: an alias for `Reg<MAINCLKSELB_SPEC>`"]
pub type MAINCLKSELB = crate::Reg<mainclkselb::MAINCLKSELB_SPEC>;
#[doc = "Main Clock Select B"]
pub mod mainclkselb;
#[doc = "PFC0DIV (rw) register accessor: an alias for `Reg<PFC0DIV_SPEC>`"]
pub type PFC0DIV = crate::Reg<pfc0div::PFC0DIV_SPEC>;
#[doc = "PFC divider 0 (trace clock)"]
pub mod pfc0div;
#[doc = "PFC1DIV (rw) register accessor: an alias for `Reg<PFC1DIV_SPEC>`"]
pub type PFC1DIV = crate::Reg<pfc1div::PFC1DIV_SPEC>;
#[doc = "PFC divider 1 (USB HS PHY bus clock)"]
pub mod pfc1div;
#[doc = "FLEXSPI0FCLKSEL (rw) register accessor: an alias for `Reg<FLEXSPI0FCLKSEL_SPEC>`"]
pub type FLEXSPI0FCLKSEL = crate::Reg<flexspi0fclksel::FLEXSPI0FCLKSEL_SPEC>;
#[doc = "FlexSPI0 Functional Clock Select"]
pub mod flexspi0fclksel;
#[doc = "FLEXSPI0FCLKDIV (rw) register accessor: an alias for `Reg<FLEXSPI0FCLKDIV_SPEC>`"]
pub type FLEXSPI0FCLKDIV = crate::Reg<flexspi0fclkdiv::FLEXSPI0FCLKDIV_SPEC>;
#[doc = "FlexSPI0 Functional Clock Divider"]
pub mod flexspi0fclkdiv;
#[doc = "FLEXSPI1FCLKSEL (rw) register accessor: an alias for `Reg<FLEXSPI1FCLKSEL_SPEC>`"]
pub type FLEXSPI1FCLKSEL = crate::Reg<flexspi1fclksel::FLEXSPI1FCLKSEL_SPEC>;
#[doc = "FlexSPI1 Functional Clock Select"]
pub mod flexspi1fclksel;
#[doc = "FLEXSPI1FCLKDIV (rw) register accessor: an alias for `Reg<FLEXSPI1FCLKDIV_SPEC>`"]
pub type FLEXSPI1FCLKDIV = crate::Reg<flexspi1fclkdiv::FLEXSPI1FCLKDIV_SPEC>;
#[doc = "FlexSPI1 Functional Clock Divider"]
pub mod flexspi1fclkdiv;
#[doc = "SCTFCLKSEL (rw) register accessor: an alias for `Reg<SCTFCLKSEL_SPEC>`"]
pub type SCTFCLKSEL = crate::Reg<sctfclksel::SCTFCLKSEL_SPEC>;
#[doc = "SCT Functional Clock Select"]
pub mod sctfclksel;
#[doc = "SCTIN7CLKDIV (rw) register accessor: an alias for `Reg<SCTIN7CLKDIV_SPEC>`"]
pub type SCTIN7CLKDIV = crate::Reg<sctin7clkdiv::SCTIN7CLKDIV_SPEC>;
#[doc = "SCT Functional Clock Divider"]
pub mod sctin7clkdiv;
#[doc = "USBHSFCLKSEL (rw) register accessor: an alias for `Reg<USBHSFCLKSEL_SPEC>`"]
pub type USBHSFCLKSEL = crate::Reg<usbhsfclksel::USBHSFCLKSEL_SPEC>;
#[doc = "High Speed USB Functional Clock Select"]
pub mod usbhsfclksel;
#[doc = "USBHSFCLKDIV (rw) register accessor: an alias for `Reg<USBHSFCLKDIV_SPEC>`"]
pub type USBHSFCLKDIV = crate::Reg<usbhsfclkdiv::USBHSFCLKDIV_SPEC>;
#[doc = "High Speed USB Functional Clock Divider"]
pub mod usbhsfclkdiv;
#[doc = "SDIO0FCLKSEL (rw) register accessor: an alias for `Reg<SDIO0FCLKSEL_SPEC>`"]
pub type SDIO0FCLKSEL = crate::Reg<sdio0fclksel::SDIO0FCLKSEL_SPEC>;
#[doc = "SDIO0 Functional Clock Select"]
pub mod sdio0fclksel;
#[doc = "SDIO0FCLKDIV (rw) register accessor: an alias for `Reg<SDIO0FCLKDIV_SPEC>`"]
pub type SDIO0FCLKDIV = crate::Reg<sdio0fclkdiv::SDIO0FCLKDIV_SPEC>;
#[doc = "SDIO0 Functional Clock Divider"]
pub mod sdio0fclkdiv;
#[doc = "SDIO1FCLKSEL (rw) register accessor: an alias for `Reg<SDIO1FCLKSEL_SPEC>`"]
pub type SDIO1FCLKSEL = crate::Reg<sdio1fclksel::SDIO1FCLKSEL_SPEC>;
#[doc = "SDIO1 Functional Clock Select"]
pub mod sdio1fclksel;
#[doc = "SDIO1FCLKDIV (rw) register accessor: an alias for `Reg<SDIO1FCLKDIV_SPEC>`"]
pub type SDIO1FCLKDIV = crate::Reg<sdio1fclkdiv::SDIO1FCLKDIV_SPEC>;
#[doc = "SDIO1 Functional Clock Divider"]
pub mod sdio1fclkdiv;
#[doc = "ADC0FCLKSEL0 (rw) register accessor: an alias for `Reg<ADC0FCLKSEL0_SPEC>`"]
pub type ADC0FCLKSEL0 = crate::Reg<adc0fclksel0::ADC0FCLKSEL0_SPEC>;
#[doc = "ADC0 Functional Clock Select 0"]
pub mod adc0fclksel0;
#[doc = "ADC0FCLKSEL1 (rw) register accessor: an alias for `Reg<ADC0FCLKSEL1_SPEC>`"]
pub type ADC0FCLKSEL1 = crate::Reg<adc0fclksel1::ADC0FCLKSEL1_SPEC>;
#[doc = "ADC0 Functional Clock Select 1"]
pub mod adc0fclksel1;
#[doc = "ADC0FCLKDIV (rw) register accessor: an alias for `Reg<ADC0FCLKDIV_SPEC>`"]
pub type ADC0FCLKDIV = crate::Reg<adc0fclkdiv::ADC0FCLKDIV_SPEC>;
#[doc = "ADC0 Functional Clock Divider"]
pub mod adc0fclkdiv;
#[doc = "UTICKFCLKSEL (rw) register accessor: an alias for `Reg<UTICKFCLKSEL_SPEC>`"]
pub type UTICKFCLKSEL = crate::Reg<utickfclksel::UTICKFCLKSEL_SPEC>;
#[doc = "UTICK Functional Clock Select"]
pub mod utickfclksel;
#[doc = "WDT0FCLKSEL (rw) register accessor: an alias for `Reg<WDT0FCLKSEL_SPEC>`"]
pub type WDT0FCLKSEL = crate::Reg<wdt0fclksel::WDT0FCLKSEL_SPEC>;
#[doc = "WDT0 Functional Clock Select"]
pub mod wdt0fclksel;
#[doc = "A32KHZWAKECLKSEL (rw) register accessor: an alias for `Reg<A32KHZWAKECLKSEL_SPEC>`"]
pub type A32KHZWAKECLKSEL = crate::Reg<a32khzwakeclksel::A32KHZWAKECLKSEL_SPEC>;
#[doc = "32 KHz Wake Clock Source Select"]
pub mod a32khzwakeclksel;
#[doc = "A32KHZWAKECLKDIV (rw) register accessor: an alias for `Reg<A32KHZWAKECLKDIV_SPEC>`"]
pub type A32KHZWAKECLKDIV = crate::Reg<a32khzwakeclkdiv::A32KHZWAKECLKDIV_SPEC>;
#[doc = "32 KHz Wake Clock Divider"]
pub mod a32khzwakeclkdiv;
#[doc = "SYSTICKFCLKSEL (rw) register accessor: an alias for `Reg<SYSTICKFCLKSEL_SPEC>`"]
pub type SYSTICKFCLKSEL = crate::Reg<systickfclksel::SYSTICKFCLKSEL_SPEC>;
#[doc = "SYSTICK Functional Clock Select"]
pub mod systickfclksel;
#[doc = "SYSTICKFCLKDIV (rw) register accessor: an alias for `Reg<SYSTICKFCLKDIV_SPEC>`"]
pub type SYSTICKFCLKDIV = crate::Reg<systickfclkdiv::SYSTICKFCLKDIV_SPEC>;
#[doc = "SYSTICK Functional Clock Divider"]
pub mod systickfclkdiv;
#[doc = "DPHYCLKSEL (rw) register accessor: an alias for `Reg<DPHYCLKSEL_SPEC>`"]
pub type DPHYCLKSEL = crate::Reg<dphyclksel::DPHYCLKSEL_SPEC>;
#[doc = "MIPI-DSI PHY Clock Select"]
pub mod dphyclksel;
#[doc = "DPHYCLKDIV (rw) register accessor: an alias for `Reg<DPHYCLKDIV_SPEC>`"]
pub type DPHYCLKDIV = crate::Reg<dphyclkdiv::DPHYCLKDIV_SPEC>;
#[doc = "MIPI-DSI PHY Clock Divider"]
pub mod dphyclkdiv;
#[doc = "DPHYESCCLKSEL (rw) register accessor: an alias for `Reg<DPHYESCCLKSEL_SPEC>`"]
pub type DPHYESCCLKSEL = crate::Reg<dphyescclksel::DPHYESCCLKSEL_SPEC>;
#[doc = "MIPI-DSI DPHY Escape Mode Clock Select"]
pub mod dphyescclksel;
#[doc = "DPHYESCRXCLKDIV (rw) register accessor: an alias for `Reg<DPHYESCRXCLKDIV_SPEC>`"]
pub type DPHYESCRXCLKDIV = crate::Reg<dphyescrxclkdiv::DPHYESCRXCLKDIV_SPEC>;
#[doc = "MIPI-DSI DPHY Escape Mode Receive Clock Divider"]
pub mod dphyescrxclkdiv;
#[doc = "DPHYESCTXCLKDIV (rw) register accessor: an alias for `Reg<DPHYESCTXCLKDIV_SPEC>`"]
pub type DPHYESCTXCLKDIV = crate::Reg<dphyesctxclkdiv::DPHYESCTXCLKDIV_SPEC>;
#[doc = "MIPI-DSI DPHY Escape Mode Tramsmit Clock Divider"]
pub mod dphyesctxclkdiv;
#[doc = "GPUCLKSEL (rw) register accessor: an alias for `Reg<GPUCLKSEL_SPEC>`"]
pub type GPUCLKSEL = crate::Reg<gpuclksel::GPUCLKSEL_SPEC>;
#[doc = "GPU Clock Select"]
pub mod gpuclksel;
#[doc = "GPUCLKDIV (rw) register accessor: an alias for `Reg<GPUCLKDIV_SPEC>`"]
pub type GPUCLKDIV = crate::Reg<gpuclkdiv::GPUCLKDIV_SPEC>;
#[doc = "GPU Clock Divider"]
pub mod gpuclkdiv;
#[doc = "DCPIXELCLKSEL (rw) register accessor: an alias for `Reg<DCPIXELCLKSEL_SPEC>`"]
pub type DCPIXELCLKSEL = crate::Reg<dcpixelclksel::DCPIXELCLKSEL_SPEC>;
#[doc = "LCDIF Pixel Clock Select"]
pub mod dcpixelclksel;
#[doc = "DCPIXELCLKDIV (rw) register accessor: an alias for `Reg<DCPIXELCLKDIV_SPEC>`"]
pub type DCPIXELCLKDIV = crate::Reg<dcpixelclkdiv::DCPIXELCLKDIV_SPEC>;
#[doc = "LCDIF Pixel Clock Divider"]
pub mod dcpixelclkdiv;
