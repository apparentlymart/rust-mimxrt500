#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00..0x94 - no description available"]
    pub channel0: CHANNEL,
    _reserved1: [u8; 0x6c],
    #[doc = "0x100..0x194 - no description available"]
    pub channel1: CHANNEL,
    _reserved2: [u8; 0x6c],
    #[doc = "0x200..0x294 - no description available"]
    pub channel2: CHANNEL,
    _reserved3: [u8; 0x6c],
    #[doc = "0x300..0x394 - no description available"]
    pub channel3: CHANNEL,
    _reserved4: [u8; 0x6c],
    #[doc = "0x400..0x494 - no description available"]
    pub channel4: CHANNEL,
    _reserved5: [u8; 0x6c],
    #[doc = "0x500..0x594 - no description available"]
    pub channel5: CHANNEL,
    _reserved6: [u8; 0x6c],
    #[doc = "0x600..0x694 - no description available"]
    pub channel6: CHANNEL,
    _reserved7: [u8; 0x6c],
    #[doc = "0x700..0x794 - no description available"]
    pub channel7: CHANNEL,
    _reserved8: [u8; 0x076c],
    #[doc = "0xf00 - Channel Enable"]
    pub chanen: CHANEN,
    _reserved9: [u8; 0x0c],
    #[doc = "0xf10 - Use 2 FS register"]
    pub use2fs: USE2FS,
    #[doc = "0xf14 - Global Channel Synchronization Enable"]
    pub global_sync_en: GLOBAL_SYNC_EN,
    #[doc = "0xf18 - Global channel synchronization counter value"]
    pub global_count_val: GLOBAL_COUNT_VAL,
    #[doc = "0xf1c - DMIC decimator reset"]
    pub decreset: DECRESET,
    _reserved13: [u8; 0x60],
    #[doc = "0xf80 - HWVAD Input Gain"]
    pub hwvadgain: HWVADGAIN,
    #[doc = "0xf84 - HWVAD Filter Control"]
    pub hwvadhpfs: HWVADHPFS,
    #[doc = "0xf88 - HWVAD Control"]
    pub hwvadst10: HWVADST10,
    #[doc = "0xf8c - HWVAD Filter Reset"]
    pub hwvadrstt: HWVADRSTT,
    #[doc = "0xf90 - HWVAD Noise Estimator Gain"]
    pub hwvadthgn: HWVADTHGN,
    #[doc = "0xf94 - HWVAD Signal Estimator Gain"]
    pub hwvadthgs: HWVADTHGS,
    #[doc = "0xf98 - HWVAD Noise Envelope Estimator"]
    pub hwvadlowz: HWVADLOWZ,
}
#[doc = "no description available"]
pub use self::channel::CHANNEL;
#[doc = r"Cluster"]
#[doc = "no description available"]
pub mod channel;
#[doc = "CHANEN (rw) register accessor: an alias for `Reg<CHANEN_SPEC>`"]
pub type CHANEN = crate::Reg<chanen::CHANEN_SPEC>;
#[doc = "Channel Enable"]
pub mod chanen;
#[doc = "USE2FS (rw) register accessor: an alias for `Reg<USE2FS_SPEC>`"]
pub type USE2FS = crate::Reg<use2fs::USE2FS_SPEC>;
#[doc = "Use 2 FS register"]
pub mod use2fs;
#[doc = "GLOBAL_SYNC_EN (rw) register accessor: an alias for `Reg<GLOBAL_SYNC_EN_SPEC>`"]
pub type GLOBAL_SYNC_EN = crate::Reg<global_sync_en::GLOBAL_SYNC_EN_SPEC>;
#[doc = "Global Channel Synchronization Enable"]
pub mod global_sync_en;
#[doc = "GLOBAL_COUNT_VAL (rw) register accessor: an alias for `Reg<GLOBAL_COUNT_VAL_SPEC>`"]
pub type GLOBAL_COUNT_VAL = crate::Reg<global_count_val::GLOBAL_COUNT_VAL_SPEC>;
#[doc = "Global channel synchronization counter value"]
pub mod global_count_val;
#[doc = "DECRESET (rw) register accessor: an alias for `Reg<DECRESET_SPEC>`"]
pub type DECRESET = crate::Reg<decreset::DECRESET_SPEC>;
#[doc = "DMIC decimator reset"]
pub mod decreset;
#[doc = "HWVADGAIN (rw) register accessor: an alias for `Reg<HWVADGAIN_SPEC>`"]
pub type HWVADGAIN = crate::Reg<hwvadgain::HWVADGAIN_SPEC>;
#[doc = "HWVAD Input Gain"]
pub mod hwvadgain;
#[doc = "HWVADHPFS (rw) register accessor: an alias for `Reg<HWVADHPFS_SPEC>`"]
pub type HWVADHPFS = crate::Reg<hwvadhpfs::HWVADHPFS_SPEC>;
#[doc = "HWVAD Filter Control"]
pub mod hwvadhpfs;
#[doc = "HWVADST10 (rw) register accessor: an alias for `Reg<HWVADST10_SPEC>`"]
pub type HWVADST10 = crate::Reg<hwvadst10::HWVADST10_SPEC>;
#[doc = "HWVAD Control"]
pub mod hwvadst10;
#[doc = "HWVADRSTT (rw) register accessor: an alias for `Reg<HWVADRSTT_SPEC>`"]
pub type HWVADRSTT = crate::Reg<hwvadrstt::HWVADRSTT_SPEC>;
#[doc = "HWVAD Filter Reset"]
pub mod hwvadrstt;
#[doc = "HWVADTHGN (rw) register accessor: an alias for `Reg<HWVADTHGN_SPEC>`"]
pub type HWVADTHGN = crate::Reg<hwvadthgn::HWVADTHGN_SPEC>;
#[doc = "HWVAD Noise Estimator Gain"]
pub mod hwvadthgn;
#[doc = "HWVADTHGS (rw) register accessor: an alias for `Reg<HWVADTHGS_SPEC>`"]
pub type HWVADTHGS = crate::Reg<hwvadthgs::HWVADTHGS_SPEC>;
#[doc = "HWVAD Signal Estimator Gain"]
pub mod hwvadthgs;
#[doc = "HWVADLOWZ (r) register accessor: an alias for `Reg<HWVADLOWZ_SPEC>`"]
pub type HWVADLOWZ = crate::Reg<hwvadlowz::HWVADLOWZ_SPEC>;
#[doc = "HWVAD Noise Envelope Estimator"]
pub mod hwvadlowz;
