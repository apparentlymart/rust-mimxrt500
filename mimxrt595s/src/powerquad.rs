#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Output Base"]
    pub outbase: OUTBASE,
    #[doc = "0x04 - Output Format"]
    pub outformat: OUTFORMAT,
    #[doc = "0x08 - Temporary Base"]
    pub tmpbase: TMPBASE,
    #[doc = "0x0c - Temporary Format"]
    pub tmpformat: TMPFORMAT,
    #[doc = "0x10 - Input A base"]
    pub inabase: INABASE,
    #[doc = "0x14 - Input A format"]
    pub inaformat: INAFORMAT,
    #[doc = "0x18 - Input B base"]
    pub inbbase: INBBASE,
    #[doc = "0x1c - Input B format"]
    pub inbformat: INBFORMAT,
    _reserved8: [u8; 0xe0],
    #[doc = "0x100 - Control"]
    pub control: CONTROL,
    #[doc = "0x104 - Length"]
    pub length: LENGTH,
    #[doc = "0x108 - Coprocessor Pre-scale"]
    pub cppre: CPPRE,
    #[doc = "0x10c - Miscellaneous"]
    pub misc: MISC,
    #[doc = "0x110 - Cursory"]
    pub cursory: CURSORY,
    _reserved13: [u8; 0x6c],
    #[doc = "0x180 - CORDIC input X"]
    pub cordic_x: CORDIC_X,
    #[doc = "0x184 - CORDIC input Y"]
    pub cordic_y: CORDIC_Y,
    #[doc = "0x188 - CORDIC input Z"]
    pub cordic_z: CORDIC_Z,
    #[doc = "0x18c - Error Status"]
    pub errstat: ERRSTAT,
    #[doc = "0x190 - Interrupt Enable"]
    pub intren: INTREN,
    #[doc = "0x194 - Event Enable"]
    pub eventen: EVENTEN,
    #[doc = "0x198 - Interrupt Status"]
    pub intrstat: INTRSTAT,
    _reserved20: [u8; 0x64],
    #[doc = "0x200..0x240 - General Purpose Register Bank n"]
    pub gpreg: [GPREG; 16],
    #[doc = "0x240..0x260 - Compute Register Bank n"]
    pub compreg: [COMPREG; 8],
}
#[doc = "OUTBASE (rw) register accessor: an alias for `Reg<OUTBASE_SPEC>`"]
pub type OUTBASE = crate::Reg<outbase::OUTBASE_SPEC>;
#[doc = "Output Base"]
pub mod outbase;
#[doc = "OUTFORMAT (rw) register accessor: an alias for `Reg<OUTFORMAT_SPEC>`"]
pub type OUTFORMAT = crate::Reg<outformat::OUTFORMAT_SPEC>;
#[doc = "Output Format"]
pub mod outformat;
#[doc = "TMPBASE (rw) register accessor: an alias for `Reg<TMPBASE_SPEC>`"]
pub type TMPBASE = crate::Reg<tmpbase::TMPBASE_SPEC>;
#[doc = "Temporary Base"]
pub mod tmpbase;
#[doc = "TMPFORMAT (rw) register accessor: an alias for `Reg<TMPFORMAT_SPEC>`"]
pub type TMPFORMAT = crate::Reg<tmpformat::TMPFORMAT_SPEC>;
#[doc = "Temporary Format"]
pub mod tmpformat;
#[doc = "INABASE (rw) register accessor: an alias for `Reg<INABASE_SPEC>`"]
pub type INABASE = crate::Reg<inabase::INABASE_SPEC>;
#[doc = "Input A base"]
pub mod inabase;
#[doc = "INAFORMAT (rw) register accessor: an alias for `Reg<INAFORMAT_SPEC>`"]
pub type INAFORMAT = crate::Reg<inaformat::INAFORMAT_SPEC>;
#[doc = "Input A format"]
pub mod inaformat;
#[doc = "INBBASE (rw) register accessor: an alias for `Reg<INBBASE_SPEC>`"]
pub type INBBASE = crate::Reg<inbbase::INBBASE_SPEC>;
#[doc = "Input B base"]
pub mod inbbase;
#[doc = "INBFORMAT (rw) register accessor: an alias for `Reg<INBFORMAT_SPEC>`"]
pub type INBFORMAT = crate::Reg<inbformat::INBFORMAT_SPEC>;
#[doc = "Input B format"]
pub mod inbformat;
#[doc = "CONTROL (rw) register accessor: an alias for `Reg<CONTROL_SPEC>`"]
pub type CONTROL = crate::Reg<control::CONTROL_SPEC>;
#[doc = "Control"]
pub mod control;
#[doc = "LENGTH (rw) register accessor: an alias for `Reg<LENGTH_SPEC>`"]
pub type LENGTH = crate::Reg<length::LENGTH_SPEC>;
#[doc = "Length"]
pub mod length;
#[doc = "CPPRE (rw) register accessor: an alias for `Reg<CPPRE_SPEC>`"]
pub type CPPRE = crate::Reg<cppre::CPPRE_SPEC>;
#[doc = "Coprocessor Pre-scale"]
pub mod cppre;
#[doc = "MISC (rw) register accessor: an alias for `Reg<MISC_SPEC>`"]
pub type MISC = crate::Reg<misc::MISC_SPEC>;
#[doc = "Miscellaneous"]
pub mod misc;
#[doc = "CURSORY (rw) register accessor: an alias for `Reg<CURSORY_SPEC>`"]
pub type CURSORY = crate::Reg<cursory::CURSORY_SPEC>;
#[doc = "Cursory"]
pub mod cursory;
#[doc = "CORDIC_X (rw) register accessor: an alias for `Reg<CORDIC_X_SPEC>`"]
pub type CORDIC_X = crate::Reg<cordic_x::CORDIC_X_SPEC>;
#[doc = "CORDIC input X"]
pub mod cordic_x;
#[doc = "CORDIC_Y (rw) register accessor: an alias for `Reg<CORDIC_Y_SPEC>`"]
pub type CORDIC_Y = crate::Reg<cordic_y::CORDIC_Y_SPEC>;
#[doc = "CORDIC input Y"]
pub mod cordic_y;
#[doc = "CORDIC_Z (rw) register accessor: an alias for `Reg<CORDIC_Z_SPEC>`"]
pub type CORDIC_Z = crate::Reg<cordic_z::CORDIC_Z_SPEC>;
#[doc = "CORDIC input Z"]
pub mod cordic_z;
#[doc = "ERRSTAT (rw) register accessor: an alias for `Reg<ERRSTAT_SPEC>`"]
pub type ERRSTAT = crate::Reg<errstat::ERRSTAT_SPEC>;
#[doc = "Error Status"]
pub mod errstat;
#[doc = "INTREN (rw) register accessor: an alias for `Reg<INTREN_SPEC>`"]
pub type INTREN = crate::Reg<intren::INTREN_SPEC>;
#[doc = "Interrupt Enable"]
pub mod intren;
#[doc = "EVENTEN (rw) register accessor: an alias for `Reg<EVENTEN_SPEC>`"]
pub type EVENTEN = crate::Reg<eventen::EVENTEN_SPEC>;
#[doc = "Event Enable"]
pub mod eventen;
#[doc = "INTRSTAT (rw) register accessor: an alias for `Reg<INTRSTAT_SPEC>`"]
pub type INTRSTAT = crate::Reg<intrstat::INTRSTAT_SPEC>;
#[doc = "Interrupt Status"]
pub mod intrstat;
#[doc = "GPREG (rw) register accessor: an alias for `Reg<GPREG_SPEC>`"]
pub type GPREG = crate::Reg<gpreg::GPREG_SPEC>;
#[doc = "General Purpose Register Bank n"]
pub mod gpreg;
#[doc = "COMPREG (rw) register accessor: an alias for `Reg<COMPREG_SPEC>`"]
pub type COMPREG = crate::Reg<compreg::COMPREG_SPEC>;
#[doc = "Compute Register Bank n"]
pub mod compreg;
