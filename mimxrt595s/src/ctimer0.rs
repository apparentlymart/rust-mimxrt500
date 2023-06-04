#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Interrupt Register."]
    pub ir: IR,
    #[doc = "0x04 - Timer Control Register"]
    pub tcr: TCR,
    #[doc = "0x08 - Timer Counter"]
    pub tc: TC,
    #[doc = "0x0c - Prescale Register"]
    pub pr: PR,
    #[doc = "0x10 - Prescale Counter."]
    pub pc: PC,
    #[doc = "0x14 - Match Control Register."]
    pub mcr: MCR,
    #[doc = "0x18..0x28 - Match Register"]
    pub mr: [MR; 4],
    #[doc = "0x28 - Capture Control Register"]
    pub ccr: CCR,
    #[doc = "0x2c..0x3c - Capture Register"]
    pub cr: [CR; 4],
    #[doc = "0x3c - External Match Register"]
    pub emr: EMR,
    _reserved10: [u8; 0x30],
    #[doc = "0x70 - Count Control Register"]
    pub ctcr: CTCR,
    #[doc = "0x74 - PWM Control Register"]
    pub pwmc: PWMC,
    #[doc = "0x78..0x88 - Match Shadow Register"]
    pub msr: [MSR; 4],
}
#[doc = "IR (rw) register accessor: an alias for `Reg<IR_SPEC>`"]
pub type IR = crate::Reg<ir::IR_SPEC>;
#[doc = "Interrupt Register."]
pub mod ir;
#[doc = "TCR (rw) register accessor: an alias for `Reg<TCR_SPEC>`"]
pub type TCR = crate::Reg<tcr::TCR_SPEC>;
#[doc = "Timer Control Register"]
pub mod tcr;
#[doc = "TC (rw) register accessor: an alias for `Reg<TC_SPEC>`"]
pub type TC = crate::Reg<tc::TC_SPEC>;
#[doc = "Timer Counter"]
pub mod tc;
#[doc = "PR (rw) register accessor: an alias for `Reg<PR_SPEC>`"]
pub type PR = crate::Reg<pr::PR_SPEC>;
#[doc = "Prescale Register"]
pub mod pr;
#[doc = "PC (rw) register accessor: an alias for `Reg<PC_SPEC>`"]
pub type PC = crate::Reg<pc::PC_SPEC>;
#[doc = "Prescale Counter."]
pub mod pc;
#[doc = "MCR (rw) register accessor: an alias for `Reg<MCR_SPEC>`"]
pub type MCR = crate::Reg<mcr::MCR_SPEC>;
#[doc = "Match Control Register."]
pub mod mcr;
#[doc = "MR (rw) register accessor: an alias for `Reg<MR_SPEC>`"]
pub type MR = crate::Reg<mr::MR_SPEC>;
#[doc = "Match Register"]
pub mod mr;
#[doc = "CCR (rw) register accessor: an alias for `Reg<CCR_SPEC>`"]
pub type CCR = crate::Reg<ccr::CCR_SPEC>;
#[doc = "Capture Control Register"]
pub mod ccr;
#[doc = "CR (r) register accessor: an alias for `Reg<CR_SPEC>`"]
pub type CR = crate::Reg<cr::CR_SPEC>;
#[doc = "Capture Register"]
pub mod cr;
#[doc = "EMR (rw) register accessor: an alias for `Reg<EMR_SPEC>`"]
pub type EMR = crate::Reg<emr::EMR_SPEC>;
#[doc = "External Match Register"]
pub mod emr;
#[doc = "CTCR (rw) register accessor: an alias for `Reg<CTCR_SPEC>`"]
pub type CTCR = crate::Reg<ctcr::CTCR_SPEC>;
#[doc = "Count Control Register"]
pub mod ctcr;
#[doc = "PWMC (rw) register accessor: an alias for `Reg<PWMC_SPEC>`"]
pub type PWMC = crate::Reg<pwmc::PWMC_SPEC>;
#[doc = "PWM Control Register"]
pub mod pwmc;
#[doc = "MSR (rw) register accessor: an alias for `Reg<MSR_SPEC>`"]
pub type MSR = crate::Reg<msr::MSR_SPEC>;
#[doc = "Match Shadow Register"]
pub mod msr;
