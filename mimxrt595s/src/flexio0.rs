#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Version ID Register"]
    pub verid: VERID,
    #[doc = "0x04 - Parameter Register"]
    pub param: PARAM,
    #[doc = "0x08 - FlexIO Control Register"]
    pub ctrl: CTRL,
    #[doc = "0x0c - Pin State Register"]
    pub pin: PIN,
    #[doc = "0x10 - Shifter Status Register"]
    pub shiftstat: SHIFTSTAT,
    #[doc = "0x14 - Shifter Error Register"]
    pub shifterr: SHIFTERR,
    #[doc = "0x18 - Timer Status Register"]
    pub timstat: TIMSTAT,
    _reserved7: [u8; 0x04],
    #[doc = "0x20 - Shifter Status Interrupt Enable"]
    pub shiftsien: SHIFTSIEN,
    #[doc = "0x24 - Shifter Error Interrupt Enable"]
    pub shifteien: SHIFTEIEN,
    #[doc = "0x28 - Timer Interrupt Enable Register"]
    pub timien: TIMIEN,
    _reserved10: [u8; 0x04],
    #[doc = "0x30 - Shifter Status DMA Enable"]
    pub shiftsden: SHIFTSDEN,
    _reserved11: [u8; 0x04],
    #[doc = "0x38 - Timer Status DMA Enable"]
    pub timersden: TIMERSDEN,
    _reserved12: [u8; 0x04],
    #[doc = "0x40 - Shifter State Register"]
    pub shiftstate: SHIFTSTATE,
    _reserved13: [u8; 0x04],
    #[doc = "0x48 - Trigger Status Register"]
    pub trgstat: TRGSTAT,
    #[doc = "0x4c - External Trigger Interrupt Enable Register"]
    pub trigien: TRIGIEN,
    #[doc = "0x50 - Pin Status Register"]
    pub pinstat: PINSTAT,
    #[doc = "0x54 - Pin Interrupt Enable Register"]
    pub pinien: PINIEN,
    #[doc = "0x58 - Pin Rising Edge Enable Register"]
    pub pinren: PINREN,
    #[doc = "0x5c - Pin Falling Edge Enable Register"]
    pub pinfen: PINFEN,
    #[doc = "0x60 - Pin Output Data Register"]
    pub pinoutd: PINOUTD,
    #[doc = "0x64 - Pin Output Enable Register"]
    pub pinoute: PINOUTE,
    #[doc = "0x68 - Pin Output Disable Register"]
    pub pinoutdis: PINOUTDIS,
    #[doc = "0x6c - Pin Output Clear Register"]
    pub pinoutclr: PINOUTCLR,
    #[doc = "0x70 - Pin Output Set Register"]
    pub pinoutset: PINOUTSET,
    #[doc = "0x74 - Pin Output Toggle Register"]
    pub pinouttog: PINOUTTOG,
    _reserved25: [u8; 0x08],
    #[doc = "0x80..0xa0 - Shifter Control N Register"]
    pub shiftctl: [SHIFTCTL; 8],
    _reserved26: [u8; 0x60],
    #[doc = "0x100..0x120 - Shifter Configuration N Register"]
    pub shiftcfg: [SHIFTCFG; 8],
    _reserved27: [u8; 0xe0],
    #[doc = "0x200..0x220 - Shifter Buffer N Register"]
    pub shiftbuf: [SHIFTBUF; 8],
    _reserved28: [u8; 0x60],
    #[doc = "0x280..0x2a0 - Shifter Buffer N Bit Swapped Register"]
    pub shiftbufbis: [SHIFTBUFBIS; 8],
    _reserved29: [u8; 0x60],
    #[doc = "0x300..0x320 - Shifter Buffer N Byte Swapped Register"]
    pub shiftbufbys: [SHIFTBUFBYS; 8],
    _reserved30: [u8; 0x60],
    #[doc = "0x380..0x3a0 - Shifter Buffer N Bit Byte Swapped Register"]
    pub shiftbufbbs: [SHIFTBUFBBS; 8],
    _reserved31: [u8; 0x60],
    #[doc = "0x400..0x420 - Timer Control N Register"]
    pub timctl: [TIMCTL; 8],
    _reserved32: [u8; 0x60],
    #[doc = "0x480..0x4a0 - Timer Configuration N Register"]
    pub timcfg: [TIMCFG; 8],
    _reserved33: [u8; 0x60],
    #[doc = "0x500..0x520 - Timer Compare N Register"]
    pub timcmp: [TIMCMP; 8],
    _reserved34: [u8; 0x0160],
    #[doc = "0x680..0x6a0 - Shifter Buffer N Nibble Byte Swapped Register"]
    pub shiftbufnbs: [SHIFTBUFNBS; 8],
    _reserved35: [u8; 0x60],
    #[doc = "0x700..0x720 - Shifter Buffer N Half Word Swapped Register"]
    pub shiftbufhws: [SHIFTBUFHWS; 8],
    _reserved36: [u8; 0x60],
    #[doc = "0x780..0x7a0 - Shifter Buffer N Nibble Swapped Register"]
    pub shiftbufnis: [SHIFTBUFNIS; 8],
    _reserved37: [u8; 0x60],
    #[doc = "0x800..0x820 - Shifter Buffer N Odd Even Swapped Register"]
    pub shiftbufoes: [SHIFTBUFOES; 8],
    _reserved38: [u8; 0x60],
    #[doc = "0x880..0x8a0 - Shifter Buffer N Even Odd Swapped Register"]
    pub shiftbufeos: [SHIFTBUFEOS; 8],
    _reserved39: [u8; 0x60],
    #[doc = "0x900..0x920 - Shifter Buffer N Halfword Byte Swapped Register"]
    pub shiftbufhbs: [SHIFTBUFHBS; 8],
}
#[doc = "VERID (r) register accessor: an alias for `Reg<VERID_SPEC>`"]
pub type VERID = crate::Reg<verid::VERID_SPEC>;
#[doc = "Version ID Register"]
pub mod verid;
#[doc = "PARAM (r) register accessor: an alias for `Reg<PARAM_SPEC>`"]
pub type PARAM = crate::Reg<param::PARAM_SPEC>;
#[doc = "Parameter Register"]
pub mod param;
#[doc = "CTRL (rw) register accessor: an alias for `Reg<CTRL_SPEC>`"]
pub type CTRL = crate::Reg<ctrl::CTRL_SPEC>;
#[doc = "FlexIO Control Register"]
pub mod ctrl;
#[doc = "PIN (r) register accessor: an alias for `Reg<PIN_SPEC>`"]
pub type PIN = crate::Reg<pin::PIN_SPEC>;
#[doc = "Pin State Register"]
pub mod pin;
#[doc = "SHIFTSTAT (rw) register accessor: an alias for `Reg<SHIFTSTAT_SPEC>`"]
pub type SHIFTSTAT = crate::Reg<shiftstat::SHIFTSTAT_SPEC>;
#[doc = "Shifter Status Register"]
pub mod shiftstat;
#[doc = "SHIFTERR (rw) register accessor: an alias for `Reg<SHIFTERR_SPEC>`"]
pub type SHIFTERR = crate::Reg<shifterr::SHIFTERR_SPEC>;
#[doc = "Shifter Error Register"]
pub mod shifterr;
#[doc = "TIMSTAT (rw) register accessor: an alias for `Reg<TIMSTAT_SPEC>`"]
pub type TIMSTAT = crate::Reg<timstat::TIMSTAT_SPEC>;
#[doc = "Timer Status Register"]
pub mod timstat;
#[doc = "SHIFTSIEN (rw) register accessor: an alias for `Reg<SHIFTSIEN_SPEC>`"]
pub type SHIFTSIEN = crate::Reg<shiftsien::SHIFTSIEN_SPEC>;
#[doc = "Shifter Status Interrupt Enable"]
pub mod shiftsien;
#[doc = "SHIFTEIEN (rw) register accessor: an alias for `Reg<SHIFTEIEN_SPEC>`"]
pub type SHIFTEIEN = crate::Reg<shifteien::SHIFTEIEN_SPEC>;
#[doc = "Shifter Error Interrupt Enable"]
pub mod shifteien;
#[doc = "TIMIEN (rw) register accessor: an alias for `Reg<TIMIEN_SPEC>`"]
pub type TIMIEN = crate::Reg<timien::TIMIEN_SPEC>;
#[doc = "Timer Interrupt Enable Register"]
pub mod timien;
#[doc = "SHIFTSDEN (rw) register accessor: an alias for `Reg<SHIFTSDEN_SPEC>`"]
pub type SHIFTSDEN = crate::Reg<shiftsden::SHIFTSDEN_SPEC>;
#[doc = "Shifter Status DMA Enable"]
pub mod shiftsden;
#[doc = "TIMERSDEN (rw) register accessor: an alias for `Reg<TIMERSDEN_SPEC>`"]
pub type TIMERSDEN = crate::Reg<timersden::TIMERSDEN_SPEC>;
#[doc = "Timer Status DMA Enable"]
pub mod timersden;
#[doc = "SHIFTSTATE (rw) register accessor: an alias for `Reg<SHIFTSTATE_SPEC>`"]
pub type SHIFTSTATE = crate::Reg<shiftstate::SHIFTSTATE_SPEC>;
#[doc = "Shifter State Register"]
pub mod shiftstate;
#[doc = "TRGSTAT (rw) register accessor: an alias for `Reg<TRGSTAT_SPEC>`"]
pub type TRGSTAT = crate::Reg<trgstat::TRGSTAT_SPEC>;
#[doc = "Trigger Status Register"]
pub mod trgstat;
#[doc = "TRIGIEN (rw) register accessor: an alias for `Reg<TRIGIEN_SPEC>`"]
pub type TRIGIEN = crate::Reg<trigien::TRIGIEN_SPEC>;
#[doc = "External Trigger Interrupt Enable Register"]
pub mod trigien;
#[doc = "PINSTAT (rw) register accessor: an alias for `Reg<PINSTAT_SPEC>`"]
pub type PINSTAT = crate::Reg<pinstat::PINSTAT_SPEC>;
#[doc = "Pin Status Register"]
pub mod pinstat;
#[doc = "PINIEN (rw) register accessor: an alias for `Reg<PINIEN_SPEC>`"]
pub type PINIEN = crate::Reg<pinien::PINIEN_SPEC>;
#[doc = "Pin Interrupt Enable Register"]
pub mod pinien;
#[doc = "PINREN (rw) register accessor: an alias for `Reg<PINREN_SPEC>`"]
pub type PINREN = crate::Reg<pinren::PINREN_SPEC>;
#[doc = "Pin Rising Edge Enable Register"]
pub mod pinren;
#[doc = "PINFEN (rw) register accessor: an alias for `Reg<PINFEN_SPEC>`"]
pub type PINFEN = crate::Reg<pinfen::PINFEN_SPEC>;
#[doc = "Pin Falling Edge Enable Register"]
pub mod pinfen;
#[doc = "PINOUTD (rw) register accessor: an alias for `Reg<PINOUTD_SPEC>`"]
pub type PINOUTD = crate::Reg<pinoutd::PINOUTD_SPEC>;
#[doc = "Pin Output Data Register"]
pub mod pinoutd;
#[doc = "PINOUTE (rw) register accessor: an alias for `Reg<PINOUTE_SPEC>`"]
pub type PINOUTE = crate::Reg<pinoute::PINOUTE_SPEC>;
#[doc = "Pin Output Enable Register"]
pub mod pinoute;
#[doc = "PINOUTDIS (rw) register accessor: an alias for `Reg<PINOUTDIS_SPEC>`"]
pub type PINOUTDIS = crate::Reg<pinoutdis::PINOUTDIS_SPEC>;
#[doc = "Pin Output Disable Register"]
pub mod pinoutdis;
#[doc = "PINOUTCLR (rw) register accessor: an alias for `Reg<PINOUTCLR_SPEC>`"]
pub type PINOUTCLR = crate::Reg<pinoutclr::PINOUTCLR_SPEC>;
#[doc = "Pin Output Clear Register"]
pub mod pinoutclr;
#[doc = "PINOUTSET (rw) register accessor: an alias for `Reg<PINOUTSET_SPEC>`"]
pub type PINOUTSET = crate::Reg<pinoutset::PINOUTSET_SPEC>;
#[doc = "Pin Output Set Register"]
pub mod pinoutset;
#[doc = "PINOUTTOG (rw) register accessor: an alias for `Reg<PINOUTTOG_SPEC>`"]
pub type PINOUTTOG = crate::Reg<pinouttog::PINOUTTOG_SPEC>;
#[doc = "Pin Output Toggle Register"]
pub mod pinouttog;
#[doc = "SHIFTCTL (rw) register accessor: an alias for `Reg<SHIFTCTL_SPEC>`"]
pub type SHIFTCTL = crate::Reg<shiftctl::SHIFTCTL_SPEC>;
#[doc = "Shifter Control N Register"]
pub mod shiftctl;
#[doc = "SHIFTCFG (rw) register accessor: an alias for `Reg<SHIFTCFG_SPEC>`"]
pub type SHIFTCFG = crate::Reg<shiftcfg::SHIFTCFG_SPEC>;
#[doc = "Shifter Configuration N Register"]
pub mod shiftcfg;
#[doc = "SHIFTBUF (rw) register accessor: an alias for `Reg<SHIFTBUF_SPEC>`"]
pub type SHIFTBUF = crate::Reg<shiftbuf::SHIFTBUF_SPEC>;
#[doc = "Shifter Buffer N Register"]
pub mod shiftbuf;
#[doc = "SHIFTBUFBIS (rw) register accessor: an alias for `Reg<SHIFTBUFBIS_SPEC>`"]
pub type SHIFTBUFBIS = crate::Reg<shiftbufbis::SHIFTBUFBIS_SPEC>;
#[doc = "Shifter Buffer N Bit Swapped Register"]
pub mod shiftbufbis;
#[doc = "SHIFTBUFBYS (rw) register accessor: an alias for `Reg<SHIFTBUFBYS_SPEC>`"]
pub type SHIFTBUFBYS = crate::Reg<shiftbufbys::SHIFTBUFBYS_SPEC>;
#[doc = "Shifter Buffer N Byte Swapped Register"]
pub mod shiftbufbys;
#[doc = "SHIFTBUFBBS (rw) register accessor: an alias for `Reg<SHIFTBUFBBS_SPEC>`"]
pub type SHIFTBUFBBS = crate::Reg<shiftbufbbs::SHIFTBUFBBS_SPEC>;
#[doc = "Shifter Buffer N Bit Byte Swapped Register"]
pub mod shiftbufbbs;
#[doc = "TIMCTL (rw) register accessor: an alias for `Reg<TIMCTL_SPEC>`"]
pub type TIMCTL = crate::Reg<timctl::TIMCTL_SPEC>;
#[doc = "Timer Control N Register"]
pub mod timctl;
#[doc = "TIMCFG (rw) register accessor: an alias for `Reg<TIMCFG_SPEC>`"]
pub type TIMCFG = crate::Reg<timcfg::TIMCFG_SPEC>;
#[doc = "Timer Configuration N Register"]
pub mod timcfg;
#[doc = "TIMCMP (rw) register accessor: an alias for `Reg<TIMCMP_SPEC>`"]
pub type TIMCMP = crate::Reg<timcmp::TIMCMP_SPEC>;
#[doc = "Timer Compare N Register"]
pub mod timcmp;
#[doc = "SHIFTBUFNBS (rw) register accessor: an alias for `Reg<SHIFTBUFNBS_SPEC>`"]
pub type SHIFTBUFNBS = crate::Reg<shiftbufnbs::SHIFTBUFNBS_SPEC>;
#[doc = "Shifter Buffer N Nibble Byte Swapped Register"]
pub mod shiftbufnbs;
#[doc = "SHIFTBUFHWS (rw) register accessor: an alias for `Reg<SHIFTBUFHWS_SPEC>`"]
pub type SHIFTBUFHWS = crate::Reg<shiftbufhws::SHIFTBUFHWS_SPEC>;
#[doc = "Shifter Buffer N Half Word Swapped Register"]
pub mod shiftbufhws;
#[doc = "SHIFTBUFNIS (rw) register accessor: an alias for `Reg<SHIFTBUFNIS_SPEC>`"]
pub type SHIFTBUFNIS = crate::Reg<shiftbufnis::SHIFTBUFNIS_SPEC>;
#[doc = "Shifter Buffer N Nibble Swapped Register"]
pub mod shiftbufnis;
#[doc = "SHIFTBUFOES (rw) register accessor: an alias for `Reg<SHIFTBUFOES_SPEC>`"]
pub type SHIFTBUFOES = crate::Reg<shiftbufoes::SHIFTBUFOES_SPEC>;
#[doc = "Shifter Buffer N Odd Even Swapped Register"]
pub mod shiftbufoes;
#[doc = "SHIFTBUFEOS (rw) register accessor: an alias for `Reg<SHIFTBUFEOS_SPEC>`"]
pub type SHIFTBUFEOS = crate::Reg<shiftbufeos::SHIFTBUFEOS_SPEC>;
#[doc = "Shifter Buffer N Even Odd Swapped Register"]
pub mod shiftbufeos;
#[doc = "SHIFTBUFHBS (rw) register accessor: an alias for `Reg<SHIFTBUFHBS_SPEC>`"]
pub type SHIFTBUFHBS = crate::Reg<shiftbufhbs::SHIFTBUFHBS_SPEC>;
#[doc = "Shifter Buffer N Halfword Byte Swapped Register"]
pub mod shiftbufhbs;
