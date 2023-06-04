#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved_0_freqme_freqmectrl: [u8; 0x04],
}
impl RegisterBlock {
    #[doc = "0x00 - Frequency Measurement (in Write mode)"]
    #[inline(always)]
    pub const fn freqme_freqmectrl_w(&self) -> &FREQME_FREQMECTRL_W {
        unsafe { &*(self as *const Self).cast::<u8>().add(0usize).cast() }
    }
    #[doc = "0x00 - Frequency Measurement (in Read mode)"]
    #[inline(always)]
    pub const fn freqme_freqmectrl_r(&self) -> &FREQME_FREQMECTRL_R {
        unsafe { &*(self as *const Self).cast::<u8>().add(0usize).cast() }
    }
}
#[doc = "FREQME_FREQMECTRL_R (r) register accessor: an alias for `Reg<FREQME_FREQMECTRL_R_SPEC>`"]
pub type FREQME_FREQMECTRL_R = crate::Reg<freqme_freqmectrl_r::FREQME_FREQMECTRL_R_SPEC>;
#[doc = "Frequency Measurement (in Read mode)"]
pub mod freqme_freqmectrl_r;
#[doc = "FREQME_FREQMECTRL_W (w) register accessor: an alias for `Reg<FREQME_FREQMECTRL_W_SPEC>`"]
pub type FREQME_FREQMECTRL_W = crate::Reg<freqme_freqmectrl_w::FREQME_FREQMECTRL_W_SPEC>;
#[doc = "Frequency Measurement (in Write mode)"]
pub mod freqme_freqmectrl_w;
