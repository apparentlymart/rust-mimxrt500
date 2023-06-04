#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - PUF Control"]
    pub ctrl: CTRL,
    #[doc = "0x04 - PUF Key Index"]
    pub keyindex: KEYINDEX,
    #[doc = "0x08 - PUF Key Size"]
    pub keysize: KEYSIZE,
    _reserved3: [u8; 0x14],
    #[doc = "0x20 - PUF Status"]
    pub stat: STAT,
    _reserved4: [u8; 0x04],
    #[doc = "0x28 - PUF Allow"]
    pub allow: ALLOW,
    _reserved5: [u8; 0x14],
    #[doc = "0x40 - PUF Key Input"]
    pub keyinput: KEYINPUT,
    #[doc = "0x44 - PUF Code Input"]
    pub codeinput: CODEINPUT,
    #[doc = "0x48 - PUF Code Output"]
    pub codeoutput: CODEOUTPUT,
    _reserved8: [u8; 0x14],
    #[doc = "0x60 - PUF Key Output Index"]
    pub keyoutindex: KEYOUTINDEX,
    #[doc = "0x64 - PUF Key Output"]
    pub keyoutput: KEYOUTPUT,
    _reserved10: [u8; 0x74],
    #[doc = "0xdc - PUF Interface Status and Clear"]
    pub ifstat: IFSTAT,
    _reserved11: [u8; 0x20],
    #[doc = "0x100 - PUF Interrupt Enable"]
    pub inten: INTEN,
    #[doc = "0x104 - PUF Interrupt Status"]
    pub intstat: INTSTAT,
    #[doc = "0x108 - PUF Power Control"]
    pub pwrctrl: PWRCTRL,
    #[doc = "0x10c - PUF Configuration"]
    pub cfg: CFG,
    _reserved15: [u8; 0xf0],
    #[doc = "0x200 - Key Lock"]
    pub keylock: KEYLOCK,
    #[doc = "0x204 - Key Enable"]
    pub keyenable: KEYENABLE,
    #[doc = "0x208 - Key Reset"]
    pub keyreset: KEYRESET,
    #[doc = "0x20c - Index Block Low"]
    pub idxblk_l: IDXBLK_L,
    #[doc = "0x210 - Index Block High Duplicate"]
    pub idxblk_h_dp: IDXBLK_H_DP,
    #[doc = "0x214..0x224 - Key Mask x"]
    pub keymask: [KEYMASK; 4],
    _reserved21: [u8; 0x30],
    #[doc = "0x254 - Index Block High"]
    pub idxblk_h: IDXBLK_H,
    #[doc = "0x258 - Index Block Low Duplicate"]
    pub idxblk_l_dp: IDXBLK_L_DP,
}
#[doc = "CTRL (rw) register accessor: an alias for `Reg<CTRL_SPEC>`"]
pub type CTRL = crate::Reg<ctrl::CTRL_SPEC>;
#[doc = "PUF Control"]
pub mod ctrl;
#[doc = "KEYINDEX (rw) register accessor: an alias for `Reg<KEYINDEX_SPEC>`"]
pub type KEYINDEX = crate::Reg<keyindex::KEYINDEX_SPEC>;
#[doc = "PUF Key Index"]
pub mod keyindex;
#[doc = "KEYSIZE (rw) register accessor: an alias for `Reg<KEYSIZE_SPEC>`"]
pub type KEYSIZE = crate::Reg<keysize::KEYSIZE_SPEC>;
#[doc = "PUF Key Size"]
pub mod keysize;
#[doc = "STAT (r) register accessor: an alias for `Reg<STAT_SPEC>`"]
pub type STAT = crate::Reg<stat::STAT_SPEC>;
#[doc = "PUF Status"]
pub mod stat;
#[doc = "ALLOW (r) register accessor: an alias for `Reg<ALLOW_SPEC>`"]
pub type ALLOW = crate::Reg<allow::ALLOW_SPEC>;
#[doc = "PUF Allow"]
pub mod allow;
#[doc = "KEYINPUT (w) register accessor: an alias for `Reg<KEYINPUT_SPEC>`"]
pub type KEYINPUT = crate::Reg<keyinput::KEYINPUT_SPEC>;
#[doc = "PUF Key Input"]
pub mod keyinput;
#[doc = "CODEINPUT (w) register accessor: an alias for `Reg<CODEINPUT_SPEC>`"]
pub type CODEINPUT = crate::Reg<codeinput::CODEINPUT_SPEC>;
#[doc = "PUF Code Input"]
pub mod codeinput;
#[doc = "CODEOUTPUT (r) register accessor: an alias for `Reg<CODEOUTPUT_SPEC>`"]
pub type CODEOUTPUT = crate::Reg<codeoutput::CODEOUTPUT_SPEC>;
#[doc = "PUF Code Output"]
pub mod codeoutput;
#[doc = "KEYOUTINDEX (r) register accessor: an alias for `Reg<KEYOUTINDEX_SPEC>`"]
pub type KEYOUTINDEX = crate::Reg<keyoutindex::KEYOUTINDEX_SPEC>;
#[doc = "PUF Key Output Index"]
pub mod keyoutindex;
#[doc = "KEYOUTPUT (r) register accessor: an alias for `Reg<KEYOUTPUT_SPEC>`"]
pub type KEYOUTPUT = crate::Reg<keyoutput::KEYOUTPUT_SPEC>;
#[doc = "PUF Key Output"]
pub mod keyoutput;
#[doc = "IFSTAT (rw) register accessor: an alias for `Reg<IFSTAT_SPEC>`"]
pub type IFSTAT = crate::Reg<ifstat::IFSTAT_SPEC>;
#[doc = "PUF Interface Status and Clear"]
pub mod ifstat;
#[doc = "INTEN (rw) register accessor: an alias for `Reg<INTEN_SPEC>`"]
pub type INTEN = crate::Reg<inten::INTEN_SPEC>;
#[doc = "PUF Interrupt Enable"]
pub mod inten;
#[doc = "INTSTAT (rw) register accessor: an alias for `Reg<INTSTAT_SPEC>`"]
pub type INTSTAT = crate::Reg<intstat::INTSTAT_SPEC>;
#[doc = "PUF Interrupt Status"]
pub mod intstat;
#[doc = "PWRCTRL (rw) register accessor: an alias for `Reg<PWRCTRL_SPEC>`"]
pub type PWRCTRL = crate::Reg<pwrctrl::PWRCTRL_SPEC>;
#[doc = "PUF Power Control"]
pub mod pwrctrl;
#[doc = "CFG (rw) register accessor: an alias for `Reg<CFG_SPEC>`"]
pub type CFG = crate::Reg<cfg::CFG_SPEC>;
#[doc = "PUF Configuration"]
pub mod cfg;
#[doc = "KEYLOCK (rw) register accessor: an alias for `Reg<KEYLOCK_SPEC>`"]
pub type KEYLOCK = crate::Reg<keylock::KEYLOCK_SPEC>;
#[doc = "Key Lock"]
pub mod keylock;
#[doc = "KEYENABLE (rw) register accessor: an alias for `Reg<KEYENABLE_SPEC>`"]
pub type KEYENABLE = crate::Reg<keyenable::KEYENABLE_SPEC>;
#[doc = "Key Enable"]
pub mod keyenable;
#[doc = "KEYRESET (w) register accessor: an alias for `Reg<KEYRESET_SPEC>`"]
pub type KEYRESET = crate::Reg<keyreset::KEYRESET_SPEC>;
#[doc = "Key Reset"]
pub mod keyreset;
#[doc = "IDXBLK_L (rw) register accessor: an alias for `Reg<IDXBLK_L_SPEC>`"]
pub type IDXBLK_L = crate::Reg<idxblk_l::IDXBLK_L_SPEC>;
#[doc = "Index Block Low"]
pub mod idxblk_l;
#[doc = "IDXBLK_H_DP (rw) register accessor: an alias for `Reg<IDXBLK_H_DP_SPEC>`"]
pub type IDXBLK_H_DP = crate::Reg<idxblk_h_dp::IDXBLK_H_DP_SPEC>;
#[doc = "Index Block High Duplicate"]
pub mod idxblk_h_dp;
#[doc = "KEYMASK (w) register accessor: an alias for `Reg<KEYMASK_SPEC>`"]
pub type KEYMASK = crate::Reg<keymask::KEYMASK_SPEC>;
#[doc = "Key Mask x"]
pub mod keymask;
#[doc = "IDXBLK_H (rw) register accessor: an alias for `Reg<IDXBLK_H_SPEC>`"]
pub type IDXBLK_H = crate::Reg<idxblk_h::IDXBLK_H_SPEC>;
#[doc = "Index Block High"]
pub mod idxblk_h;
#[doc = "IDXBLK_L_DP (rw) register accessor: an alias for `Reg<IDXBLK_L_DP_SPEC>`"]
pub type IDXBLK_L_DP = crate::Reg<idxblk_l_dp::IDXBLK_L_DP_SPEC>;
#[doc = "Index Block Low Duplicate"]
pub mod idxblk_l_dp;
