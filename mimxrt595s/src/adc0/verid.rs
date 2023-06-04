#[doc = "Register `VERID` reader"]
pub struct R(crate::R<VERID_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<VERID_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<VERID_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<VERID_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `RES` reader - Resolution"]
pub type RES_R = crate::BitReader<RES_A>;
#[doc = "Resolution\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RES_A {
    #[doc = "0: Up to 13-bit differential or 12-bit single-ended resolution supported."]
    MAX_13_BIT = 0,
    #[doc = "1: Up to 16-bit differential or 15-bit single-ended resolution supported. CMDLn\\[MODE\\]
available for selecting the resolution of conversions for the associated command."]
    MAX_16_BIT = 1,
}
impl From<RES_A> for bool {
    #[inline(always)]
    fn from(variant: RES_A) -> Self {
        variant as u8 != 0
    }
}
impl RES_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RES_A {
        match self.bits {
            false => RES_A::MAX_13_BIT,
            true => RES_A::MAX_16_BIT,
        }
    }
    #[doc = "Checks if the value of the field is `MAX_13_BIT`"]
    #[inline(always)]
    pub fn is_max_13_bit(&self) -> bool {
        *self == RES_A::MAX_13_BIT
    }
    #[doc = "Checks if the value of the field is `MAX_16_BIT`"]
    #[inline(always)]
    pub fn is_max_16_bit(&self) -> bool {
        *self == RES_A::MAX_16_BIT
    }
}
#[doc = "Field `DIFFEN` reader - Differential Supported"]
pub type DIFFEN_R = crate::BitReader<DIFFEN_A>;
#[doc = "Differential Supported\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DIFFEN_A {
    #[doc = "0: Not supported"]
    DIFFERENTIAL_NOT_SUPPORTED = 0,
    #[doc = "1: Supported. CMDLn\\[DIFF\\]
and CMDLn\\[ABSEL\\]
control fields implemented."]
    DIFFERENTIAL_SUPPORTED = 1,
}
impl From<DIFFEN_A> for bool {
    #[inline(always)]
    fn from(variant: DIFFEN_A) -> Self {
        variant as u8 != 0
    }
}
impl DIFFEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DIFFEN_A {
        match self.bits {
            false => DIFFEN_A::DIFFERENTIAL_NOT_SUPPORTED,
            true => DIFFEN_A::DIFFERENTIAL_SUPPORTED,
        }
    }
    #[doc = "Checks if the value of the field is `DIFFERENTIAL_NOT_SUPPORTED`"]
    #[inline(always)]
    pub fn is_differential_not_supported(&self) -> bool {
        *self == DIFFEN_A::DIFFERENTIAL_NOT_SUPPORTED
    }
    #[doc = "Checks if the value of the field is `DIFFERENTIAL_SUPPORTED`"]
    #[inline(always)]
    pub fn is_differential_supported(&self) -> bool {
        *self == DIFFEN_A::DIFFERENTIAL_SUPPORTED
    }
}
#[doc = "Field `MVI` reader - Multiple Vref Implemented"]
pub type MVI_R = crate::BitReader<MVI_A>;
#[doc = "Multiple Vref Implemented\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MVI_A {
    #[doc = "0: Single VREFH input supported."]
    MULTIPLE_REF_NOT_SUPPORTED = 0,
    #[doc = "1: Multiple VREFH inputs supported."]
    MULTIPLE_REF_SUPPORTED = 1,
}
impl From<MVI_A> for bool {
    #[inline(always)]
    fn from(variant: MVI_A) -> Self {
        variant as u8 != 0
    }
}
impl MVI_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MVI_A {
        match self.bits {
            false => MVI_A::MULTIPLE_REF_NOT_SUPPORTED,
            true => MVI_A::MULTIPLE_REF_SUPPORTED,
        }
    }
    #[doc = "Checks if the value of the field is `MULTIPLE_REF_NOT_SUPPORTED`"]
    #[inline(always)]
    pub fn is_multiple_ref_not_supported(&self) -> bool {
        *self == MVI_A::MULTIPLE_REF_NOT_SUPPORTED
    }
    #[doc = "Checks if the value of the field is `MULTIPLE_REF_SUPPORTED`"]
    #[inline(always)]
    pub fn is_multiple_ref_supported(&self) -> bool {
        *self == MVI_A::MULTIPLE_REF_SUPPORTED
    }
}
#[doc = "Field `CSW` reader - Channel Scale Width"]
pub type CSW_R = crate::FieldReader<u8, CSW_A>;
#[doc = "Channel Scale Width\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CSW_A {
    #[doc = "0: Not supported."]
    CSCALE_NOT_SUPPORTED = 0,
    #[doc = "1: Supported with one-bit CSCALE control field."]
    BIT_WIDTH_1 = 1,
    #[doc = "6: Supported with six-bit CSCALE control field."]
    BIT_WIDTH_6 = 6,
}
impl From<CSW_A> for u8 {
    #[inline(always)]
    fn from(variant: CSW_A) -> Self {
        variant as _
    }
}
impl CSW_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<CSW_A> {
        match self.bits {
            0 => Some(CSW_A::CSCALE_NOT_SUPPORTED),
            1 => Some(CSW_A::BIT_WIDTH_1),
            6 => Some(CSW_A::BIT_WIDTH_6),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `CSCALE_NOT_SUPPORTED`"]
    #[inline(always)]
    pub fn is_cscale_not_supported(&self) -> bool {
        *self == CSW_A::CSCALE_NOT_SUPPORTED
    }
    #[doc = "Checks if the value of the field is `BIT_WIDTH_1`"]
    #[inline(always)]
    pub fn is_bit_width_1(&self) -> bool {
        *self == CSW_A::BIT_WIDTH_1
    }
    #[doc = "Checks if the value of the field is `BIT_WIDTH_6`"]
    #[inline(always)]
    pub fn is_bit_width_6(&self) -> bool {
        *self == CSW_A::BIT_WIDTH_6
    }
}
#[doc = "Field `VR1RNGI` reader - Voltage Reference 1 Range Control Bit Implemented"]
pub type VR1RNGI_R = crate::BitReader<VR1RNGI_A>;
#[doc = "Voltage Reference 1 Range Control Bit Implemented\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VR1RNGI_A {
    #[doc = "0: Range control not required."]
    REF1_FIXED_VOLTAGE_RANGE = 0,
    #[doc = "1: Range control required."]
    REF1_SELECTABLE_VOLTAGE_RANGE = 1,
}
impl From<VR1RNGI_A> for bool {
    #[inline(always)]
    fn from(variant: VR1RNGI_A) -> Self {
        variant as u8 != 0
    }
}
impl VR1RNGI_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> VR1RNGI_A {
        match self.bits {
            false => VR1RNGI_A::REF1_FIXED_VOLTAGE_RANGE,
            true => VR1RNGI_A::REF1_SELECTABLE_VOLTAGE_RANGE,
        }
    }
    #[doc = "Checks if the value of the field is `REF1_FIXED_VOLTAGE_RANGE`"]
    #[inline(always)]
    pub fn is_ref1_fixed_voltage_range(&self) -> bool {
        *self == VR1RNGI_A::REF1_FIXED_VOLTAGE_RANGE
    }
    #[doc = "Checks if the value of the field is `REF1_SELECTABLE_VOLTAGE_RANGE`"]
    #[inline(always)]
    pub fn is_ref1_selectable_voltage_range(&self) -> bool {
        *self == VR1RNGI_A::REF1_SELECTABLE_VOLTAGE_RANGE
    }
}
#[doc = "Field `IADCKI` reader - Internal ADC Clock Implemented"]
pub type IADCKI_R = crate::BitReader<IADCKI_A>;
#[doc = "Internal ADC Clock Implemented\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IADCKI_A {
    #[doc = "0: Not implemented"]
    INTERNAL_CLK_NOT_AVAILABLE = 0,
    #[doc = "1: Implemented"]
    INTERNAL_CLK_AVAILABLE = 1,
}
impl From<IADCKI_A> for bool {
    #[inline(always)]
    fn from(variant: IADCKI_A) -> Self {
        variant as u8 != 0
    }
}
impl IADCKI_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IADCKI_A {
        match self.bits {
            false => IADCKI_A::INTERNAL_CLK_NOT_AVAILABLE,
            true => IADCKI_A::INTERNAL_CLK_AVAILABLE,
        }
    }
    #[doc = "Checks if the value of the field is `INTERNAL_CLK_NOT_AVAILABLE`"]
    #[inline(always)]
    pub fn is_internal_clk_not_available(&self) -> bool {
        *self == IADCKI_A::INTERNAL_CLK_NOT_AVAILABLE
    }
    #[doc = "Checks if the value of the field is `INTERNAL_CLK_AVAILABLE`"]
    #[inline(always)]
    pub fn is_internal_clk_available(&self) -> bool {
        *self == IADCKI_A::INTERNAL_CLK_AVAILABLE
    }
}
#[doc = "Field `CALOFSI` reader - Calibration Function Implemented"]
pub type CALOFSI_R = crate::BitReader<CALOFSI_A>;
#[doc = "Calibration Function Implemented\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CALOFSI_A {
    #[doc = "0: Not implemented"]
    CAL_FUNCTION_NOT_AVAILABLE = 0,
    #[doc = "1: Implemented"]
    CAL_FUNCTION_AVAILABLE = 1,
}
impl From<CALOFSI_A> for bool {
    #[inline(always)]
    fn from(variant: CALOFSI_A) -> Self {
        variant as u8 != 0
    }
}
impl CALOFSI_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CALOFSI_A {
        match self.bits {
            false => CALOFSI_A::CAL_FUNCTION_NOT_AVAILABLE,
            true => CALOFSI_A::CAL_FUNCTION_AVAILABLE,
        }
    }
    #[doc = "Checks if the value of the field is `CAL_FUNCTION_NOT_AVAILABLE`"]
    #[inline(always)]
    pub fn is_cal_function_not_available(&self) -> bool {
        *self == CALOFSI_A::CAL_FUNCTION_NOT_AVAILABLE
    }
    #[doc = "Checks if the value of the field is `CAL_FUNCTION_AVAILABLE`"]
    #[inline(always)]
    pub fn is_cal_function_available(&self) -> bool {
        *self == CALOFSI_A::CAL_FUNCTION_AVAILABLE
    }
}
#[doc = "Field `NUM_SEC` reader - Number of Single-Ended Outputs Supported"]
pub type NUM_SEC_R = crate::BitReader<NUM_SEC_A>;
#[doc = "Number of Single-Ended Outputs Supported\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NUM_SEC_A {
    #[doc = "0: One"]
    SINGLE_CONVERTOR = 0,
    #[doc = "1: Two"]
    DUAL_CONVERTOR = 1,
}
impl From<NUM_SEC_A> for bool {
    #[inline(always)]
    fn from(variant: NUM_SEC_A) -> Self {
        variant as u8 != 0
    }
}
impl NUM_SEC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> NUM_SEC_A {
        match self.bits {
            false => NUM_SEC_A::SINGLE_CONVERTOR,
            true => NUM_SEC_A::DUAL_CONVERTOR,
        }
    }
    #[doc = "Checks if the value of the field is `SINGLE_CONVERTOR`"]
    #[inline(always)]
    pub fn is_single_convertor(&self) -> bool {
        *self == NUM_SEC_A::SINGLE_CONVERTOR
    }
    #[doc = "Checks if the value of the field is `DUAL_CONVERTOR`"]
    #[inline(always)]
    pub fn is_dual_convertor(&self) -> bool {
        *self == NUM_SEC_A::DUAL_CONVERTOR
    }
}
#[doc = "Field `NUM_FIFO` reader - Number of FIFOs"]
pub type NUM_FIFO_R = crate::FieldReader<u8, NUM_FIFO_A>;
#[doc = "Number of FIFOs\n\nValue on reset: 2"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum NUM_FIFO_A {
    #[doc = "0: N/A"]
    NO_FIFO_IMPLEMENTED = 0,
    #[doc = "1: One"]
    CNT_1 = 1,
    #[doc = "2: Two"]
    CNT_2 = 2,
    #[doc = "3: Three"]
    CNT_3 = 3,
    #[doc = "4: Four"]
    CNT_4 = 4,
}
impl From<NUM_FIFO_A> for u8 {
    #[inline(always)]
    fn from(variant: NUM_FIFO_A) -> Self {
        variant as _
    }
}
impl NUM_FIFO_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<NUM_FIFO_A> {
        match self.bits {
            0 => Some(NUM_FIFO_A::NO_FIFO_IMPLEMENTED),
            1 => Some(NUM_FIFO_A::CNT_1),
            2 => Some(NUM_FIFO_A::CNT_2),
            3 => Some(NUM_FIFO_A::CNT_3),
            4 => Some(NUM_FIFO_A::CNT_4),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `NO_FIFO_IMPLEMENTED`"]
    #[inline(always)]
    pub fn is_no_fifo_implemented(&self) -> bool {
        *self == NUM_FIFO_A::NO_FIFO_IMPLEMENTED
    }
    #[doc = "Checks if the value of the field is `CNT_1`"]
    #[inline(always)]
    pub fn is_cnt_1(&self) -> bool {
        *self == NUM_FIFO_A::CNT_1
    }
    #[doc = "Checks if the value of the field is `CNT_2`"]
    #[inline(always)]
    pub fn is_cnt_2(&self) -> bool {
        *self == NUM_FIFO_A::CNT_2
    }
    #[doc = "Checks if the value of the field is `CNT_3`"]
    #[inline(always)]
    pub fn is_cnt_3(&self) -> bool {
        *self == NUM_FIFO_A::CNT_3
    }
    #[doc = "Checks if the value of the field is `CNT_4`"]
    #[inline(always)]
    pub fn is_cnt_4(&self) -> bool {
        *self == NUM_FIFO_A::CNT_4
    }
}
#[doc = "Field `MINOR` reader - Minor Version Number"]
pub type MINOR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `MAJOR` reader - Major Version Number"]
pub type MAJOR_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bit 0 - Resolution"]
    #[inline(always)]
    pub fn res(&self) -> RES_R {
        RES_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Differential Supported"]
    #[inline(always)]
    pub fn diffen(&self) -> DIFFEN_R {
        DIFFEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 3 - Multiple Vref Implemented"]
    #[inline(always)]
    pub fn mvi(&self) -> MVI_R {
        MVI_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:6 - Channel Scale Width"]
    #[inline(always)]
    pub fn csw(&self) -> CSW_R {
        CSW_R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bit 8 - Voltage Reference 1 Range Control Bit Implemented"]
    #[inline(always)]
    pub fn vr1rngi(&self) -> VR1RNGI_R {
        VR1RNGI_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Internal ADC Clock Implemented"]
    #[inline(always)]
    pub fn iadcki(&self) -> IADCKI_R {
        IADCKI_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Calibration Function Implemented"]
    #[inline(always)]
    pub fn calofsi(&self) -> CALOFSI_R {
        CALOFSI_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Number of Single-Ended Outputs Supported"]
    #[inline(always)]
    pub fn num_sec(&self) -> NUM_SEC_R {
        NUM_SEC_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 12:14 - Number of FIFOs"]
    #[inline(always)]
    pub fn num_fifo(&self) -> NUM_FIFO_R {
        NUM_FIFO_R::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bits 16:23 - Minor Version Number"]
    #[inline(always)]
    pub fn minor(&self) -> MINOR_R {
        MINOR_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Major Version Number"]
    #[inline(always)]
    pub fn major(&self) -> MAJOR_R {
        MAJOR_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
#[doc = "Version ID Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [verid](index.html) module"]
pub struct VERID_SPEC;
impl crate::RegisterSpec for VERID_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [verid::R](R) reader structure"]
impl crate::Readable for VERID_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets VERID to value 0x0200_201a"]
impl crate::Resettable for VERID_SPEC {
    const RESET_VALUE: Self::Ux = 0x0200_201a;
}
