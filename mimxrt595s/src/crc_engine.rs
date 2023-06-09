#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - MODE Register"]
    pub mode: MODE,
    #[doc = "0x04 - CRC Seed Register"]
    pub seed: SEED,
    _reserved_2_sum_wr_data: [u8; 0x04],
}
impl RegisterBlock {
    #[doc = "0x08 - CRC Write Data"]
    #[inline(always)]
    pub const fn sum_wr_data_wr_data(&self) -> &SUM_WR_DATA_WR_DATA {
        unsafe { &*(self as *const Self).cast::<u8>().add(8usize).cast() }
    }
    #[doc = "0x08 - CRC Sum"]
    #[inline(always)]
    pub const fn sum_wr_data_sum(&self) -> &SUM_WR_DATA_SUM {
        unsafe { &*(self as *const Self).cast::<u8>().add(8usize).cast() }
    }
}
#[doc = "MODE (rw) register accessor: an alias for `Reg<MODE_SPEC>`"]
pub type MODE = crate::Reg<mode::MODE_SPEC>;
#[doc = "MODE Register"]
pub mod mode;
#[doc = "SEED (rw) register accessor: an alias for `Reg<SEED_SPEC>`"]
pub type SEED = crate::Reg<seed::SEED_SPEC>;
#[doc = "CRC Seed Register"]
pub mod seed;
#[doc = "SUM_WR_DATA_SUM (r) register accessor: an alias for `Reg<SUM_WR_DATA_SUM_SPEC>`"]
pub type SUM_WR_DATA_SUM = crate::Reg<sum_wr_data_sum::SUM_WR_DATA_SUM_SPEC>;
#[doc = "CRC Sum"]
pub mod sum_wr_data_sum;
#[doc = "SUM_WR_DATA_WR_DATA (rw) register accessor: an alias for `Reg<SUM_WR_DATA_WR_DATA_SPEC>`"]
pub type SUM_WR_DATA_WR_DATA = crate::Reg<sum_wr_data_wr_data::SUM_WR_DATA_WR_DATA_SPEC>;
#[doc = "CRC Write Data"]
pub mod sum_wr_data_wr_data;
