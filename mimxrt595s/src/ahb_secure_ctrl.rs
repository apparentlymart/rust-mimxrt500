#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 0x10],
    #[doc = "0x10..0x20 - Memory ROM Rule(n) Register"]
    pub rom_mem_rule: [ROM_MEM_RULE; 4],
    _reserved1: [u8; 0x10],
    #[doc = "0x30..0x40 - FLEXSPI0 Region 0 Rule(n) Register"]
    pub flexspi0_region0_rule: [FLEXSPI0_REGION0_RULE; 4],
    #[doc = "0x40 - no description available"]
    pub flexspi0_region1_4_rule0: FLEXSPI0_REGION1_4_RULE,
    _reserved3: [u8; 0x0c],
    #[doc = "0x50 - no description available"]
    pub flexspi0_region1_4_rule1: FLEXSPI0_REGION1_4_RULE,
    _reserved4: [u8; 0x0c],
    #[doc = "0x60 - no description available"]
    pub flexspi0_region1_4_rule2: FLEXSPI0_REGION1_4_RULE,
    _reserved5: [u8; 0x0c],
    #[doc = "0x70 - no description available"]
    pub flexspi0_region1_4_rule3: FLEXSPI0_REGION1_4_RULE,
    _reserved6: [u8; 0x1c],
    #[doc = "0x90..0xa0 - SRAM Partition 00 Rule(n) Register"]
    pub ram00_rule: [RAM00_RULE; 4],
    #[doc = "0xa0..0xb0 - SRAM Partition 01 Rule(n) Register"]
    pub ram01_rule: [RAM01_RULE; 4],
    _reserved8: [u8; 0x10],
    #[doc = "0xc0..0xd0 - SRAM Partition 02 Rule(n) Register"]
    pub ram02_rule: [RAM02_RULE; 4],
    #[doc = "0xd0..0xe0 - SRAM Partition 03 Rule(n) Register"]
    pub ram03_rule: [RAM03_RULE; 4],
    _reserved10: [u8; 0x10],
    #[doc = "0xf0..0x100 - SRAM Partition 04 Rule(n) Register"]
    pub ram04_rule: [RAM04_RULE; 4],
    #[doc = "0x100..0x110 - SRAM Partition 05 Rule(n) Register"]
    pub ram05_rule: [RAM05_RULE; 4],
    #[doc = "0x110..0x120 - SRAM Partition 06 Rule(n) Register"]
    pub ram06_rule: [RAM06_RULE; 4],
    #[doc = "0x120..0x130 - SRAM Partition 07 Rule(n) Register"]
    pub ram07_rule: [RAM07_RULE; 4],
    _reserved14: [u8; 0x10],
    #[doc = "0x140..0x150 - SRAM Partition 08 Rule(n) Register"]
    pub ram08_rule: [RAM08_RULE; 4],
    #[doc = "0x150..0x160 - SRAM Partition 09 Rule(n) Register"]
    pub ram09_rule: [RAM09_RULE; 4],
    #[doc = "0x160..0x170 - SRAM Partition 10 Rule(n) Register"]
    pub ram10_rule: [RAM10_RULE; 4],
    #[doc = "0x170..0x180 - SRAM Partition 11 Rule(n) Register"]
    pub ram11_rule: [RAM11_RULE; 4],
    _reserved18: [u8; 0x10],
    #[doc = "0x190..0x1a0 - SRAM Partition 12 Rule(n) Register"]
    pub ram12_rule: [RAM12_RULE; 4],
    #[doc = "0x1a0..0x1b0 - SRAM Partition 13 Rule(n) Register"]
    pub ram13_rule: [RAM13_RULE; 4],
    #[doc = "0x1b0..0x1c0 - SRAM Partition 14 Rule(n) Register"]
    pub ram14_rule: [RAM14_RULE; 4],
    #[doc = "0x1c0..0x1d0 - SRAM Partition 15 Rule(n) Register"]
    pub ram15_rule: [RAM15_RULE; 4],
    _reserved22: [u8; 0x10],
    #[doc = "0x1e0..0x1f0 - SRAM Partition 16 Rule(n) Register"]
    pub ram16_rule: [RAM16_RULE; 4],
    #[doc = "0x1f0..0x200 - SRAM Partition 17 Rule(n) Register"]
    pub ram17_rule: [RAM17_RULE; 4],
    #[doc = "0x200..0x210 - SRAM Partition 18 Rule(n) Register"]
    pub ram18_rule: [RAM18_RULE; 4],
    #[doc = "0x210..0x220 - SRAM Partition 19 Rule(n) Register"]
    pub ram19_rule: [RAM19_RULE; 4],
    _reserved26: [u8; 0x10],
    #[doc = "0x230..0x240 - SRAM Partition 20 Rule(n) Register"]
    pub ram20_rule: [RAM20_RULE; 4],
    #[doc = "0x240..0x250 - SRAM Partition 21 Rule(n) Register"]
    pub ram21_rule: [RAM21_RULE; 4],
    #[doc = "0x250..0x260 - SRAM Partition 22 Rule(n) Register"]
    pub ram22_rule: [RAM22_RULE; 4],
    #[doc = "0x260..0x270 - SRAM Partition 23 Rule(n) Register"]
    pub ram23_rule: [RAM23_RULE; 4],
    _reserved30: [u8; 0x10],
    #[doc = "0x280..0x290 - SRAM Partition 24 Rule(n) Register"]
    pub ram24_rule: [RAM24_RULE; 4],
    #[doc = "0x290..0x2a0 - SRAM Partition 25 Rule(n) Register"]
    pub ram25_rule: [RAM25_RULE; 4],
    #[doc = "0x2a0..0x2b0 - SRAM Partition 26 Rule(n) Register"]
    pub ram26_rule: [RAM26_RULE; 4],
    #[doc = "0x2b0..0x2c0 - SRAM Partition 27 Rule(n) Register"]
    pub ram27_rule: [RAM27_RULE; 4],
    _reserved34: [u8; 0x10],
    #[doc = "0x2d0..0x2e0 - SRAM Partition 28 Rule(n) Register"]
    pub ram28_rule: [RAM28_RULE; 4],
    #[doc = "0x2e0..0x2f0 - SRAM Partition 29 Rule(n) Register"]
    pub ram29_rule: [RAM29_RULE; 4],
    #[doc = "0x2f0..0x300 - SRAM Partition 30 Rule(n) Register"]
    pub ram30_rule: [RAM30_RULE; 4],
    #[doc = "0x300..0x310 - SRAM Partition 31 Rule(n) Register"]
    pub ram31_rule: [RAM31_RULE; 4],
    _reserved38: [u8; 0x10],
    #[doc = "0x320..0x330 - Smart DMA (SDMA) RAM Rule(n) Register"]
    pub sdma_ram_rule: [SDMA_RAM_RULE; 4],
    _reserved39: [u8; 0x10],
    #[doc = "0x340..0x350 - FlexSPI1 Region 0 Rule(n) Register"]
    pub flexspi1_region0_rule: [FLEXSPI1_REGION0_RULE; 4],
    #[doc = "0x350 - no description available"]
    pub flexspi1_regionn_rule00: FLEXSPI1_REGIONN_RULE0,
    _reserved41: [u8; 0x0c],
    #[doc = "0x360 - no description available"]
    pub flexspi1_regionn_rule01: FLEXSPI1_REGIONN_RULE0,
    _reserved42: [u8; 0x0c],
    #[doc = "0x370 - no description available"]
    pub flexspi1_regionn_rule02: FLEXSPI1_REGIONN_RULE0,
    _reserved43: [u8; 0x0c],
    #[doc = "0x380 - no description available"]
    pub flexspi1_regionn_rule03: FLEXSPI1_REGIONN_RULE0,
    _reserved44: [u8; 0x1c],
    #[doc = "0x3a0 - APB Bridge Peripheral 0 Rule 0 Register"]
    pub apb_bridge_per0_rule0: APB_BRIDGE_PER0_RULE0,
    #[doc = "0x3a4 - APB Bridge Peripheral 0 Rule 1 Register"]
    pub apb_bridge_per0_rule1: APB_BRIDGE_PER0_RULE1,
    _reserved46: [u8; 0x04],
    #[doc = "0x3ac - APB Bridge Peripheral 0 Rule 3 Register"]
    pub apb_bridge_per0_rule3: APB_BRIDGE_PER0_RULE3,
    #[doc = "0x3b0 - APB Bridge Peripheral 1 Rule 0 Register"]
    pub apb_bridge_per1_rule0: APB_BRIDGE_PER1_RULE0,
    #[doc = "0x3b4 - APB Bridge Peripheral 1 Rule 1 Register"]
    pub apb_bridge_per1_rule1: APB_BRIDGE_PER1_RULE1,
    #[doc = "0x3b8 - APB Bridge Peripheral 1 Rule 2 Register"]
    pub apb_bridge_per1_rule2: APB_BRIDGE_PER1_RULE2,
    #[doc = "0x3bc - APB Bridge Peripheral 1 Rule 3 Register"]
    pub apb_bridge_per1_rule3: APB_BRIDGE_PER1_RULE3,
    #[doc = "0x3c0 - AHB Peripheral 0 Slave Rule 0 Register"]
    pub ahb_periph0_slave_rule0: AHB_PERIPH0_SLAVE_RULE0,
    _reserved52: [u8; 0x0c],
    #[doc = "0x3d0 - AIPS Bridge Peripheral 0 Rule 0 Register"]
    pub aips_bridge0_per_rule0: AIPS_BRIDGE0_PER_RULE0,
    _reserved53: [u8; 0x0c],
    #[doc = "0x3e0 - AHB Peripheral 1 Slave Rule 0 Register"]
    pub ahb_periph1_slave_rule0: AHB_PERIPH1_SLAVE_RULE0,
    #[doc = "0x3e4 - AHB Peripheral 1 Slave Rule 1 Register"]
    pub ahb_periph1_slave_rule1: AHB_PERIPH1_SLAVE_RULE1,
    _reserved55: [u8; 0x18],
    #[doc = "0x400 - AIPS Bridge Peripheral 1 Rule 0 Register"]
    pub aips_bridge1_per_rule0: AIPS_BRIDGE1_PER_RULE0,
    #[doc = "0x404 - AIPS Bridge Peripheral 1 Rule 1 Register"]
    pub aips_bridge1_per_rule1: AIPS_BRIDGE1_PER_RULE1,
    _reserved57: [u8; 0x08],
    #[doc = "0x410 - AHB Peripheral 2 Slave Rule 0 Register"]
    pub ahb_periph2_slave_rule0: AHB_PERIPH2_SLAVE_RULE0,
    _reserved58: [u8; 0x0c],
    #[doc = "0x420 - AHB Secure Control Peripheral Rule 0 Register"]
    pub ahb_secure_ctrl_periph_rule0: AHB_SECURE_CTRL_PERIPH_RULE0,
    _reserved59: [u8; 0x0c],
    #[doc = "0x430 - AHB Peripheral 3 Slave Rule 0 Register"]
    pub ahb_periph3_slave_rule0: AHB_PERIPH3_SLAVE_RULE0,
    #[doc = "0x434 - AHB Peripheral 3 Slave Rule 1 Register"]
    pub ahb_periph3_slave_rule1: AHB_PERIPH3_SLAVE_RULE1,
    _reserved61: [u8; 0x09c8],
    #[doc = "0xe00..0xe48 - Security Violation Address(n) Register"]
    pub sec_vio_addr: [SEC_VIO_ADDR; 18],
    _reserved62: [u8; 0x38],
    #[doc = "0xe80..0xec8 - Security Violation Miscellaneous Information at Address(n) Register"]
    pub sec_vio_misc_info: [SEC_VIO_MISC_INFO; 18],
    _reserved63: [u8; 0x38],
    #[doc = "0xf00 - Security Violation Info Validity for Address(n) Register"]
    pub sec_vio_info_valid: SEC_VIO_INFO_VALID,
    _reserved64: [u8; 0x7c],
    #[doc = "0xf80..0xf8c - GPIO Mask for Port index Register"]
    pub sec_gpio_mask: [SEC_GPIO_MASK; 3],
    _reserved65: [u8; 0x14],
    #[doc = "0xfa0 - Secure Interrupt Mask for DSP Register"]
    pub dsp_int_mask0: DSP_INT_MASK0,
    _reserved66: [u8; 0x18],
    #[doc = "0xfbc - Secure Mask Lock Register"]
    pub sec_mask_lock: SEC_MASK_LOCK,
    _reserved67: [u8; 0x10],
    #[doc = "0xfd0 - Master Secure Level Register"]
    pub master_sec_level: MASTER_SEC_LEVEL,
    #[doc = "0xfd4 - Master Secure Level Register"]
    pub master_sec_anti_pol_reg: MASTER_SEC_ANTI_POL_REG,
    _reserved69: [u8; 0x14],
    #[doc = "0xfec - Miscellaneous CPU0 Control Signals Register"]
    pub cm33_lock_reg: CM33_LOCK_REG,
    _reserved70: [u8; 0x08],
    #[doc = "0xff8 - Secure Control Duplicate Register"]
    pub misc_ctrl_dp_reg: MISC_CTRL_DP_REG,
    #[doc = "0xffc - Secure Control Register"]
    pub misc_ctrl_reg: MISC_CTRL_REG,
}
#[doc = "ROM_MEM_RULE (rw) register accessor: an alias for `Reg<ROM_MEM_RULE_SPEC>`"]
pub type ROM_MEM_RULE = crate::Reg<rom_mem_rule::ROM_MEM_RULE_SPEC>;
#[doc = "Memory ROM Rule(n) Register"]
pub mod rom_mem_rule;
#[doc = "FLEXSPI0_REGION0_RULE (rw) register accessor: an alias for `Reg<FLEXSPI0_REGION0_RULE_SPEC>`"]
pub type FLEXSPI0_REGION0_RULE = crate::Reg<flexspi0_region0_rule::FLEXSPI0_REGION0_RULE_SPEC>;
#[doc = "FLEXSPI0 Region 0 Rule(n) Register"]
pub mod flexspi0_region0_rule;
#[doc = "no description available"]
pub use self::flexspi0_region1_4_rule::FLEXSPI0_REGION1_4_RULE;
#[doc = r"Cluster"]
#[doc = "no description available"]
pub mod flexspi0_region1_4_rule;
#[doc = "RAM00_RULE (rw) register accessor: an alias for `Reg<RAM00_RULE_SPEC>`"]
pub type RAM00_RULE = crate::Reg<ram00_rule::RAM00_RULE_SPEC>;
#[doc = "SRAM Partition 00 Rule(n) Register"]
pub mod ram00_rule;
#[doc = "RAM01_RULE (rw) register accessor: an alias for `Reg<RAM01_RULE_SPEC>`"]
pub type RAM01_RULE = crate::Reg<ram01_rule::RAM01_RULE_SPEC>;
#[doc = "SRAM Partition 01 Rule(n) Register"]
pub mod ram01_rule;
#[doc = "RAM02_RULE (rw) register accessor: an alias for `Reg<RAM02_RULE_SPEC>`"]
pub type RAM02_RULE = crate::Reg<ram02_rule::RAM02_RULE_SPEC>;
#[doc = "SRAM Partition 02 Rule(n) Register"]
pub mod ram02_rule;
#[doc = "RAM03_RULE (rw) register accessor: an alias for `Reg<RAM03_RULE_SPEC>`"]
pub type RAM03_RULE = crate::Reg<ram03_rule::RAM03_RULE_SPEC>;
#[doc = "SRAM Partition 03 Rule(n) Register"]
pub mod ram03_rule;
#[doc = "RAM04_RULE (rw) register accessor: an alias for `Reg<RAM04_RULE_SPEC>`"]
pub type RAM04_RULE = crate::Reg<ram04_rule::RAM04_RULE_SPEC>;
#[doc = "SRAM Partition 04 Rule(n) Register"]
pub mod ram04_rule;
#[doc = "RAM05_RULE (rw) register accessor: an alias for `Reg<RAM05_RULE_SPEC>`"]
pub type RAM05_RULE = crate::Reg<ram05_rule::RAM05_RULE_SPEC>;
#[doc = "SRAM Partition 05 Rule(n) Register"]
pub mod ram05_rule;
#[doc = "RAM06_RULE (rw) register accessor: an alias for `Reg<RAM06_RULE_SPEC>`"]
pub type RAM06_RULE = crate::Reg<ram06_rule::RAM06_RULE_SPEC>;
#[doc = "SRAM Partition 06 Rule(n) Register"]
pub mod ram06_rule;
#[doc = "RAM07_RULE (rw) register accessor: an alias for `Reg<RAM07_RULE_SPEC>`"]
pub type RAM07_RULE = crate::Reg<ram07_rule::RAM07_RULE_SPEC>;
#[doc = "SRAM Partition 07 Rule(n) Register"]
pub mod ram07_rule;
#[doc = "RAM08_RULE (rw) register accessor: an alias for `Reg<RAM08_RULE_SPEC>`"]
pub type RAM08_RULE = crate::Reg<ram08_rule::RAM08_RULE_SPEC>;
#[doc = "SRAM Partition 08 Rule(n) Register"]
pub mod ram08_rule;
#[doc = "RAM09_RULE (rw) register accessor: an alias for `Reg<RAM09_RULE_SPEC>`"]
pub type RAM09_RULE = crate::Reg<ram09_rule::RAM09_RULE_SPEC>;
#[doc = "SRAM Partition 09 Rule(n) Register"]
pub mod ram09_rule;
#[doc = "RAM10_RULE (rw) register accessor: an alias for `Reg<RAM10_RULE_SPEC>`"]
pub type RAM10_RULE = crate::Reg<ram10_rule::RAM10_RULE_SPEC>;
#[doc = "SRAM Partition 10 Rule(n) Register"]
pub mod ram10_rule;
#[doc = "RAM11_RULE (rw) register accessor: an alias for `Reg<RAM11_RULE_SPEC>`"]
pub type RAM11_RULE = crate::Reg<ram11_rule::RAM11_RULE_SPEC>;
#[doc = "SRAM Partition 11 Rule(n) Register"]
pub mod ram11_rule;
#[doc = "RAM12_RULE (rw) register accessor: an alias for `Reg<RAM12_RULE_SPEC>`"]
pub type RAM12_RULE = crate::Reg<ram12_rule::RAM12_RULE_SPEC>;
#[doc = "SRAM Partition 12 Rule(n) Register"]
pub mod ram12_rule;
#[doc = "RAM13_RULE (rw) register accessor: an alias for `Reg<RAM13_RULE_SPEC>`"]
pub type RAM13_RULE = crate::Reg<ram13_rule::RAM13_RULE_SPEC>;
#[doc = "SRAM Partition 13 Rule(n) Register"]
pub mod ram13_rule;
#[doc = "RAM14_RULE (rw) register accessor: an alias for `Reg<RAM14_RULE_SPEC>`"]
pub type RAM14_RULE = crate::Reg<ram14_rule::RAM14_RULE_SPEC>;
#[doc = "SRAM Partition 14 Rule(n) Register"]
pub mod ram14_rule;
#[doc = "RAM15_RULE (rw) register accessor: an alias for `Reg<RAM15_RULE_SPEC>`"]
pub type RAM15_RULE = crate::Reg<ram15_rule::RAM15_RULE_SPEC>;
#[doc = "SRAM Partition 15 Rule(n) Register"]
pub mod ram15_rule;
#[doc = "RAM16_RULE (rw) register accessor: an alias for `Reg<RAM16_RULE_SPEC>`"]
pub type RAM16_RULE = crate::Reg<ram16_rule::RAM16_RULE_SPEC>;
#[doc = "SRAM Partition 16 Rule(n) Register"]
pub mod ram16_rule;
#[doc = "RAM17_RULE (rw) register accessor: an alias for `Reg<RAM17_RULE_SPEC>`"]
pub type RAM17_RULE = crate::Reg<ram17_rule::RAM17_RULE_SPEC>;
#[doc = "SRAM Partition 17 Rule(n) Register"]
pub mod ram17_rule;
#[doc = "RAM18_RULE (rw) register accessor: an alias for `Reg<RAM18_RULE_SPEC>`"]
pub type RAM18_RULE = crate::Reg<ram18_rule::RAM18_RULE_SPEC>;
#[doc = "SRAM Partition 18 Rule(n) Register"]
pub mod ram18_rule;
#[doc = "RAM19_RULE (rw) register accessor: an alias for `Reg<RAM19_RULE_SPEC>`"]
pub type RAM19_RULE = crate::Reg<ram19_rule::RAM19_RULE_SPEC>;
#[doc = "SRAM Partition 19 Rule(n) Register"]
pub mod ram19_rule;
#[doc = "RAM20_RULE (rw) register accessor: an alias for `Reg<RAM20_RULE_SPEC>`"]
pub type RAM20_RULE = crate::Reg<ram20_rule::RAM20_RULE_SPEC>;
#[doc = "SRAM Partition 20 Rule(n) Register"]
pub mod ram20_rule;
#[doc = "RAM21_RULE (rw) register accessor: an alias for `Reg<RAM21_RULE_SPEC>`"]
pub type RAM21_RULE = crate::Reg<ram21_rule::RAM21_RULE_SPEC>;
#[doc = "SRAM Partition 21 Rule(n) Register"]
pub mod ram21_rule;
#[doc = "RAM22_RULE (rw) register accessor: an alias for `Reg<RAM22_RULE_SPEC>`"]
pub type RAM22_RULE = crate::Reg<ram22_rule::RAM22_RULE_SPEC>;
#[doc = "SRAM Partition 22 Rule(n) Register"]
pub mod ram22_rule;
#[doc = "RAM23_RULE (rw) register accessor: an alias for `Reg<RAM23_RULE_SPEC>`"]
pub type RAM23_RULE = crate::Reg<ram23_rule::RAM23_RULE_SPEC>;
#[doc = "SRAM Partition 23 Rule(n) Register"]
pub mod ram23_rule;
#[doc = "RAM24_RULE (rw) register accessor: an alias for `Reg<RAM24_RULE_SPEC>`"]
pub type RAM24_RULE = crate::Reg<ram24_rule::RAM24_RULE_SPEC>;
#[doc = "SRAM Partition 24 Rule(n) Register"]
pub mod ram24_rule;
#[doc = "RAM25_RULE (rw) register accessor: an alias for `Reg<RAM25_RULE_SPEC>`"]
pub type RAM25_RULE = crate::Reg<ram25_rule::RAM25_RULE_SPEC>;
#[doc = "SRAM Partition 25 Rule(n) Register"]
pub mod ram25_rule;
#[doc = "RAM26_RULE (rw) register accessor: an alias for `Reg<RAM26_RULE_SPEC>`"]
pub type RAM26_RULE = crate::Reg<ram26_rule::RAM26_RULE_SPEC>;
#[doc = "SRAM Partition 26 Rule(n) Register"]
pub mod ram26_rule;
#[doc = "RAM27_RULE (rw) register accessor: an alias for `Reg<RAM27_RULE_SPEC>`"]
pub type RAM27_RULE = crate::Reg<ram27_rule::RAM27_RULE_SPEC>;
#[doc = "SRAM Partition 27 Rule(n) Register"]
pub mod ram27_rule;
#[doc = "RAM28_RULE (rw) register accessor: an alias for `Reg<RAM28_RULE_SPEC>`"]
pub type RAM28_RULE = crate::Reg<ram28_rule::RAM28_RULE_SPEC>;
#[doc = "SRAM Partition 28 Rule(n) Register"]
pub mod ram28_rule;
#[doc = "RAM29_RULE (rw) register accessor: an alias for `Reg<RAM29_RULE_SPEC>`"]
pub type RAM29_RULE = crate::Reg<ram29_rule::RAM29_RULE_SPEC>;
#[doc = "SRAM Partition 29 Rule(n) Register"]
pub mod ram29_rule;
#[doc = "RAM30_RULE (rw) register accessor: an alias for `Reg<RAM30_RULE_SPEC>`"]
pub type RAM30_RULE = crate::Reg<ram30_rule::RAM30_RULE_SPEC>;
#[doc = "SRAM Partition 30 Rule(n) Register"]
pub mod ram30_rule;
#[doc = "RAM31_RULE (rw) register accessor: an alias for `Reg<RAM31_RULE_SPEC>`"]
pub type RAM31_RULE = crate::Reg<ram31_rule::RAM31_RULE_SPEC>;
#[doc = "SRAM Partition 31 Rule(n) Register"]
pub mod ram31_rule;
#[doc = "SDMA_RAM_RULE (rw) register accessor: an alias for `Reg<SDMA_RAM_RULE_SPEC>`"]
pub type SDMA_RAM_RULE = crate::Reg<sdma_ram_rule::SDMA_RAM_RULE_SPEC>;
#[doc = "Smart DMA (SDMA) RAM Rule(n) Register"]
pub mod sdma_ram_rule;
#[doc = "FLEXSPI1_REGION0_RULE (rw) register accessor: an alias for `Reg<FLEXSPI1_REGION0_RULE_SPEC>`"]
pub type FLEXSPI1_REGION0_RULE = crate::Reg<flexspi1_region0_rule::FLEXSPI1_REGION0_RULE_SPEC>;
#[doc = "FlexSPI1 Region 0 Rule(n) Register"]
pub mod flexspi1_region0_rule;
#[doc = "no description available"]
pub use self::flexspi1_regionn_rule0::FLEXSPI1_REGIONN_RULE0;
#[doc = r"Cluster"]
#[doc = "no description available"]
pub mod flexspi1_regionn_rule0;
#[doc = "APB_BRIDGE_PER0_RULE0 (rw) register accessor: an alias for `Reg<APB_BRIDGE_PER0_RULE0_SPEC>`"]
pub type APB_BRIDGE_PER0_RULE0 = crate::Reg<apb_bridge_per0_rule0::APB_BRIDGE_PER0_RULE0_SPEC>;
#[doc = "APB Bridge Peripheral 0 Rule 0 Register"]
pub mod apb_bridge_per0_rule0;
#[doc = "APB_BRIDGE_PER0_RULE1 (rw) register accessor: an alias for `Reg<APB_BRIDGE_PER0_RULE1_SPEC>`"]
pub type APB_BRIDGE_PER0_RULE1 = crate::Reg<apb_bridge_per0_rule1::APB_BRIDGE_PER0_RULE1_SPEC>;
#[doc = "APB Bridge Peripheral 0 Rule 1 Register"]
pub mod apb_bridge_per0_rule1;
#[doc = "APB_BRIDGE_PER0_RULE3 (rw) register accessor: an alias for `Reg<APB_BRIDGE_PER0_RULE3_SPEC>`"]
pub type APB_BRIDGE_PER0_RULE3 = crate::Reg<apb_bridge_per0_rule3::APB_BRIDGE_PER0_RULE3_SPEC>;
#[doc = "APB Bridge Peripheral 0 Rule 3 Register"]
pub mod apb_bridge_per0_rule3;
#[doc = "APB_BRIDGE_PER1_RULE0 (rw) register accessor: an alias for `Reg<APB_BRIDGE_PER1_RULE0_SPEC>`"]
pub type APB_BRIDGE_PER1_RULE0 = crate::Reg<apb_bridge_per1_rule0::APB_BRIDGE_PER1_RULE0_SPEC>;
#[doc = "APB Bridge Peripheral 1 Rule 0 Register"]
pub mod apb_bridge_per1_rule0;
#[doc = "APB_BRIDGE_PER1_RULE1 (rw) register accessor: an alias for `Reg<APB_BRIDGE_PER1_RULE1_SPEC>`"]
pub type APB_BRIDGE_PER1_RULE1 = crate::Reg<apb_bridge_per1_rule1::APB_BRIDGE_PER1_RULE1_SPEC>;
#[doc = "APB Bridge Peripheral 1 Rule 1 Register"]
pub mod apb_bridge_per1_rule1;
#[doc = "APB_BRIDGE_PER1_RULE2 (rw) register accessor: an alias for `Reg<APB_BRIDGE_PER1_RULE2_SPEC>`"]
pub type APB_BRIDGE_PER1_RULE2 = crate::Reg<apb_bridge_per1_rule2::APB_BRIDGE_PER1_RULE2_SPEC>;
#[doc = "APB Bridge Peripheral 1 Rule 2 Register"]
pub mod apb_bridge_per1_rule2;
#[doc = "APB_BRIDGE_PER1_RULE3 (rw) register accessor: an alias for `Reg<APB_BRIDGE_PER1_RULE3_SPEC>`"]
pub type APB_BRIDGE_PER1_RULE3 = crate::Reg<apb_bridge_per1_rule3::APB_BRIDGE_PER1_RULE3_SPEC>;
#[doc = "APB Bridge Peripheral 1 Rule 3 Register"]
pub mod apb_bridge_per1_rule3;
#[doc = "AHB_PERIPH0_SLAVE_RULE0 (rw) register accessor: an alias for `Reg<AHB_PERIPH0_SLAVE_RULE0_SPEC>`"]
pub type AHB_PERIPH0_SLAVE_RULE0 =
    crate::Reg<ahb_periph0_slave_rule0::AHB_PERIPH0_SLAVE_RULE0_SPEC>;
#[doc = "AHB Peripheral 0 Slave Rule 0 Register"]
pub mod ahb_periph0_slave_rule0;
#[doc = "AIPS_BRIDGE0_PER_RULE0 (rw) register accessor: an alias for `Reg<AIPS_BRIDGE0_PER_RULE0_SPEC>`"]
pub type AIPS_BRIDGE0_PER_RULE0 = crate::Reg<aips_bridge0_per_rule0::AIPS_BRIDGE0_PER_RULE0_SPEC>;
#[doc = "AIPS Bridge Peripheral 0 Rule 0 Register"]
pub mod aips_bridge0_per_rule0;
#[doc = "AHB_PERIPH1_SLAVE_RULE0 (rw) register accessor: an alias for `Reg<AHB_PERIPH1_SLAVE_RULE0_SPEC>`"]
pub type AHB_PERIPH1_SLAVE_RULE0 =
    crate::Reg<ahb_periph1_slave_rule0::AHB_PERIPH1_SLAVE_RULE0_SPEC>;
#[doc = "AHB Peripheral 1 Slave Rule 0 Register"]
pub mod ahb_periph1_slave_rule0;
#[doc = "AHB_PERIPH1_SLAVE_RULE1 (rw) register accessor: an alias for `Reg<AHB_PERIPH1_SLAVE_RULE1_SPEC>`"]
pub type AHB_PERIPH1_SLAVE_RULE1 =
    crate::Reg<ahb_periph1_slave_rule1::AHB_PERIPH1_SLAVE_RULE1_SPEC>;
#[doc = "AHB Peripheral 1 Slave Rule 1 Register"]
pub mod ahb_periph1_slave_rule1;
#[doc = "AIPS_BRIDGE1_PER_RULE0 (rw) register accessor: an alias for `Reg<AIPS_BRIDGE1_PER_RULE0_SPEC>`"]
pub type AIPS_BRIDGE1_PER_RULE0 = crate::Reg<aips_bridge1_per_rule0::AIPS_BRIDGE1_PER_RULE0_SPEC>;
#[doc = "AIPS Bridge Peripheral 1 Rule 0 Register"]
pub mod aips_bridge1_per_rule0;
#[doc = "AIPS_BRIDGE1_PER_RULE1 (rw) register accessor: an alias for `Reg<AIPS_BRIDGE1_PER_RULE1_SPEC>`"]
pub type AIPS_BRIDGE1_PER_RULE1 = crate::Reg<aips_bridge1_per_rule1::AIPS_BRIDGE1_PER_RULE1_SPEC>;
#[doc = "AIPS Bridge Peripheral 1 Rule 1 Register"]
pub mod aips_bridge1_per_rule1;
#[doc = "AHB_PERIPH2_SLAVE_RULE0 (rw) register accessor: an alias for `Reg<AHB_PERIPH2_SLAVE_RULE0_SPEC>`"]
pub type AHB_PERIPH2_SLAVE_RULE0 =
    crate::Reg<ahb_periph2_slave_rule0::AHB_PERIPH2_SLAVE_RULE0_SPEC>;
#[doc = "AHB Peripheral 2 Slave Rule 0 Register"]
pub mod ahb_periph2_slave_rule0;
#[doc = "AHB_SECURE_CTRL_PERIPH_RULE0 (rw) register accessor: an alias for `Reg<AHB_SECURE_CTRL_PERIPH_RULE0_SPEC>`"]
pub type AHB_SECURE_CTRL_PERIPH_RULE0 =
    crate::Reg<ahb_secure_ctrl_periph_rule0::AHB_SECURE_CTRL_PERIPH_RULE0_SPEC>;
#[doc = "AHB Secure Control Peripheral Rule 0 Register"]
pub mod ahb_secure_ctrl_periph_rule0;
#[doc = "AHB_PERIPH3_SLAVE_RULE0 (rw) register accessor: an alias for `Reg<AHB_PERIPH3_SLAVE_RULE0_SPEC>`"]
pub type AHB_PERIPH3_SLAVE_RULE0 =
    crate::Reg<ahb_periph3_slave_rule0::AHB_PERIPH3_SLAVE_RULE0_SPEC>;
#[doc = "AHB Peripheral 3 Slave Rule 0 Register"]
pub mod ahb_periph3_slave_rule0;
#[doc = "AHB_PERIPH3_SLAVE_RULE1 (rw) register accessor: an alias for `Reg<AHB_PERIPH3_SLAVE_RULE1_SPEC>`"]
pub type AHB_PERIPH3_SLAVE_RULE1 =
    crate::Reg<ahb_periph3_slave_rule1::AHB_PERIPH3_SLAVE_RULE1_SPEC>;
#[doc = "AHB Peripheral 3 Slave Rule 1 Register"]
pub mod ahb_periph3_slave_rule1;
#[doc = "SEC_VIO_ADDR (r) register accessor: an alias for `Reg<SEC_VIO_ADDR_SPEC>`"]
pub type SEC_VIO_ADDR = crate::Reg<sec_vio_addr::SEC_VIO_ADDR_SPEC>;
#[doc = "Security Violation Address(n) Register"]
pub mod sec_vio_addr;
#[doc = "SEC_VIO_MISC_INFO (r) register accessor: an alias for `Reg<SEC_VIO_MISC_INFO_SPEC>`"]
pub type SEC_VIO_MISC_INFO = crate::Reg<sec_vio_misc_info::SEC_VIO_MISC_INFO_SPEC>;
#[doc = "Security Violation Miscellaneous Information at Address(n) Register"]
pub mod sec_vio_misc_info;
#[doc = "SEC_VIO_INFO_VALID (rw) register accessor: an alias for `Reg<SEC_VIO_INFO_VALID_SPEC>`"]
pub type SEC_VIO_INFO_VALID = crate::Reg<sec_vio_info_valid::SEC_VIO_INFO_VALID_SPEC>;
#[doc = "Security Violation Info Validity for Address(n) Register"]
pub mod sec_vio_info_valid;
#[doc = "SEC_GPIO_MASK (rw) register accessor: an alias for `Reg<SEC_GPIO_MASK_SPEC>`"]
pub type SEC_GPIO_MASK = crate::Reg<sec_gpio_mask::SEC_GPIO_MASK_SPEC>;
#[doc = "GPIO Mask for Port index Register"]
pub mod sec_gpio_mask;
#[doc = "DSP_INT_MASK0 (rw) register accessor: an alias for `Reg<DSP_INT_MASK0_SPEC>`"]
pub type DSP_INT_MASK0 = crate::Reg<dsp_int_mask0::DSP_INT_MASK0_SPEC>;
#[doc = "Secure Interrupt Mask for DSP Register"]
pub mod dsp_int_mask0;
#[doc = "SEC_MASK_LOCK (rw) register accessor: an alias for `Reg<SEC_MASK_LOCK_SPEC>`"]
pub type SEC_MASK_LOCK = crate::Reg<sec_mask_lock::SEC_MASK_LOCK_SPEC>;
#[doc = "Secure Mask Lock Register"]
pub mod sec_mask_lock;
#[doc = "MASTER_SEC_LEVEL (rw) register accessor: an alias for `Reg<MASTER_SEC_LEVEL_SPEC>`"]
pub type MASTER_SEC_LEVEL = crate::Reg<master_sec_level::MASTER_SEC_LEVEL_SPEC>;
#[doc = "Master Secure Level Register"]
pub mod master_sec_level;
#[doc = "MASTER_SEC_ANTI_POL_REG (rw) register accessor: an alias for `Reg<MASTER_SEC_ANTI_POL_REG_SPEC>`"]
pub type MASTER_SEC_ANTI_POL_REG =
    crate::Reg<master_sec_anti_pol_reg::MASTER_SEC_ANTI_POL_REG_SPEC>;
#[doc = "Master Secure Level Register"]
pub mod master_sec_anti_pol_reg;
#[doc = "CM33_LOCK_REG (rw) register accessor: an alias for `Reg<CM33_LOCK_REG_SPEC>`"]
pub type CM33_LOCK_REG = crate::Reg<cm33_lock_reg::CM33_LOCK_REG_SPEC>;
#[doc = "Miscellaneous CPU0 Control Signals Register"]
pub mod cm33_lock_reg;
#[doc = "MISC_CTRL_DP_REG (rw) register accessor: an alias for `Reg<MISC_CTRL_DP_REG_SPEC>`"]
pub type MISC_CTRL_DP_REG = crate::Reg<misc_ctrl_dp_reg::MISC_CTRL_DP_REG_SPEC>;
#[doc = "Secure Control Duplicate Register"]
pub mod misc_ctrl_dp_reg;
#[doc = "MISC_CTRL_REG (rw) register accessor: an alias for `Reg<MISC_CTRL_REG_SPEC>`"]
pub type MISC_CTRL_REG = crate::Reg<misc_ctrl_reg::MISC_CTRL_REG_SPEC>;
#[doc = "Secure Control Register"]
pub mod misc_ctrl_reg;
