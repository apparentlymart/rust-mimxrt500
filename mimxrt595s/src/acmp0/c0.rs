#[doc = "Register `C0` reader"]
pub struct R(crate::R<C0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<C0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<C0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<C0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `C0` writer"]
pub struct W(crate::W<C0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<C0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<C0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<C0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `HYSTCTR` reader - Comparator hard block hysteresis control. See chip data sheet to get the actual hystersis value with each level"]
pub type HYSTCTR_R = crate::FieldReader<u8, HYSTCTR_A>;
#[doc = "Comparator hard block hysteresis control. See chip data sheet to get the actual hystersis value with each level\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum HYSTCTR_A {
    #[doc = "0: The hard block output has level 0 hysteresis internally."]
    HYSTCTR_0 = 0,
    #[doc = "1: The hard block output has level 1 hysteresis internally."]
    HYSTCTR_1 = 1,
    #[doc = "2: The hard block output has level 2 hysteresis internally."]
    HYSTCTR_2 = 2,
    #[doc = "3: The hard block output has level 3 hysteresis internally."]
    HYSTCTR_3 = 3,
}
impl From<HYSTCTR_A> for u8 {
    #[inline(always)]
    fn from(variant: HYSTCTR_A) -> Self {
        variant as _
    }
}
impl HYSTCTR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HYSTCTR_A {
        match self.bits {
            0 => HYSTCTR_A::HYSTCTR_0,
            1 => HYSTCTR_A::HYSTCTR_1,
            2 => HYSTCTR_A::HYSTCTR_2,
            3 => HYSTCTR_A::HYSTCTR_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `HYSTCTR_0`"]
    #[inline(always)]
    pub fn is_hystctr_0(&self) -> bool {
        *self == HYSTCTR_A::HYSTCTR_0
    }
    #[doc = "Checks if the value of the field is `HYSTCTR_1`"]
    #[inline(always)]
    pub fn is_hystctr_1(&self) -> bool {
        *self == HYSTCTR_A::HYSTCTR_1
    }
    #[doc = "Checks if the value of the field is `HYSTCTR_2`"]
    #[inline(always)]
    pub fn is_hystctr_2(&self) -> bool {
        *self == HYSTCTR_A::HYSTCTR_2
    }
    #[doc = "Checks if the value of the field is `HYSTCTR_3`"]
    #[inline(always)]
    pub fn is_hystctr_3(&self) -> bool {
        *self == HYSTCTR_A::HYSTCTR_3
    }
}
#[doc = "Field `HYSTCTR` writer - Comparator hard block hysteresis control. See chip data sheet to get the actual hystersis value with each level"]
pub type HYSTCTR_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, C0_SPEC, u8, HYSTCTR_A, 2, O>;
impl<'a, const O: u8> HYSTCTR_W<'a, O> {
    #[doc = "The hard block output has level 0 hysteresis internally."]
    #[inline(always)]
    pub fn hystctr_0(self) -> &'a mut W {
        self.variant(HYSTCTR_A::HYSTCTR_0)
    }
    #[doc = "The hard block output has level 1 hysteresis internally."]
    #[inline(always)]
    pub fn hystctr_1(self) -> &'a mut W {
        self.variant(HYSTCTR_A::HYSTCTR_1)
    }
    #[doc = "The hard block output has level 2 hysteresis internally."]
    #[inline(always)]
    pub fn hystctr_2(self) -> &'a mut W {
        self.variant(HYSTCTR_A::HYSTCTR_2)
    }
    #[doc = "The hard block output has level 3 hysteresis internally."]
    #[inline(always)]
    pub fn hystctr_3(self) -> &'a mut W {
        self.variant(HYSTCTR_A::HYSTCTR_3)
    }
}
#[doc = "Field `FILTER_CNT` reader - Filter Sample Count"]
pub type FILTER_CNT_R = crate::FieldReader<u8, FILTER_CNT_A>;
#[doc = "Filter Sample Count\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum FILTER_CNT_A {
    #[doc = "0: Filter is disabled. If SE = 1, then COUT is a logic zero (this is not a legal state, and is not recommended). If SE = 0, COUT = COUTA."]
    FILTER_CNT_0 = 0,
    #[doc = "1: 1 consecutive sample must agree (comparator output is simply sampled)."]
    FILTER_CNT_1 = 1,
    #[doc = "2: 2 consecutive samples must agree."]
    FILTER_CNT_2 = 2,
    #[doc = "3: 3 consecutive samples must agree."]
    FILTER_CNT_3 = 3,
    #[doc = "4: 4 consecutive samples must agree."]
    FILTER_CNT_4 = 4,
    #[doc = "5: 5 consecutive samples must agree."]
    FILTER_CNT_5 = 5,
    #[doc = "6: 6 consecutive samples must agree."]
    FILTER_CNT_6 = 6,
    #[doc = "7: 7 consecutive samples must agree."]
    FILTER_CNT_7 = 7,
}
impl From<FILTER_CNT_A> for u8 {
    #[inline(always)]
    fn from(variant: FILTER_CNT_A) -> Self {
        variant as _
    }
}
impl FILTER_CNT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FILTER_CNT_A {
        match self.bits {
            0 => FILTER_CNT_A::FILTER_CNT_0,
            1 => FILTER_CNT_A::FILTER_CNT_1,
            2 => FILTER_CNT_A::FILTER_CNT_2,
            3 => FILTER_CNT_A::FILTER_CNT_3,
            4 => FILTER_CNT_A::FILTER_CNT_4,
            5 => FILTER_CNT_A::FILTER_CNT_5,
            6 => FILTER_CNT_A::FILTER_CNT_6,
            7 => FILTER_CNT_A::FILTER_CNT_7,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `FILTER_CNT_0`"]
    #[inline(always)]
    pub fn is_filter_cnt_0(&self) -> bool {
        *self == FILTER_CNT_A::FILTER_CNT_0
    }
    #[doc = "Checks if the value of the field is `FILTER_CNT_1`"]
    #[inline(always)]
    pub fn is_filter_cnt_1(&self) -> bool {
        *self == FILTER_CNT_A::FILTER_CNT_1
    }
    #[doc = "Checks if the value of the field is `FILTER_CNT_2`"]
    #[inline(always)]
    pub fn is_filter_cnt_2(&self) -> bool {
        *self == FILTER_CNT_A::FILTER_CNT_2
    }
    #[doc = "Checks if the value of the field is `FILTER_CNT_3`"]
    #[inline(always)]
    pub fn is_filter_cnt_3(&self) -> bool {
        *self == FILTER_CNT_A::FILTER_CNT_3
    }
    #[doc = "Checks if the value of the field is `FILTER_CNT_4`"]
    #[inline(always)]
    pub fn is_filter_cnt_4(&self) -> bool {
        *self == FILTER_CNT_A::FILTER_CNT_4
    }
    #[doc = "Checks if the value of the field is `FILTER_CNT_5`"]
    #[inline(always)]
    pub fn is_filter_cnt_5(&self) -> bool {
        *self == FILTER_CNT_A::FILTER_CNT_5
    }
    #[doc = "Checks if the value of the field is `FILTER_CNT_6`"]
    #[inline(always)]
    pub fn is_filter_cnt_6(&self) -> bool {
        *self == FILTER_CNT_A::FILTER_CNT_6
    }
    #[doc = "Checks if the value of the field is `FILTER_CNT_7`"]
    #[inline(always)]
    pub fn is_filter_cnt_7(&self) -> bool {
        *self == FILTER_CNT_A::FILTER_CNT_7
    }
}
#[doc = "Field `FILTER_CNT` writer - Filter Sample Count"]
pub type FILTER_CNT_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, C0_SPEC, u8, FILTER_CNT_A, 3, O>;
impl<'a, const O: u8> FILTER_CNT_W<'a, O> {
    #[doc = "Filter is disabled. If SE = 1, then COUT is a logic zero (this is not a legal state, and is not recommended). If SE = 0, COUT = COUTA."]
    #[inline(always)]
    pub fn filter_cnt_0(self) -> &'a mut W {
        self.variant(FILTER_CNT_A::FILTER_CNT_0)
    }
    #[doc = "1 consecutive sample must agree (comparator output is simply sampled)."]
    #[inline(always)]
    pub fn filter_cnt_1(self) -> &'a mut W {
        self.variant(FILTER_CNT_A::FILTER_CNT_1)
    }
    #[doc = "2 consecutive samples must agree."]
    #[inline(always)]
    pub fn filter_cnt_2(self) -> &'a mut W {
        self.variant(FILTER_CNT_A::FILTER_CNT_2)
    }
    #[doc = "3 consecutive samples must agree."]
    #[inline(always)]
    pub fn filter_cnt_3(self) -> &'a mut W {
        self.variant(FILTER_CNT_A::FILTER_CNT_3)
    }
    #[doc = "4 consecutive samples must agree."]
    #[inline(always)]
    pub fn filter_cnt_4(self) -> &'a mut W {
        self.variant(FILTER_CNT_A::FILTER_CNT_4)
    }
    #[doc = "5 consecutive samples must agree."]
    #[inline(always)]
    pub fn filter_cnt_5(self) -> &'a mut W {
        self.variant(FILTER_CNT_A::FILTER_CNT_5)
    }
    #[doc = "6 consecutive samples must agree."]
    #[inline(always)]
    pub fn filter_cnt_6(self) -> &'a mut W {
        self.variant(FILTER_CNT_A::FILTER_CNT_6)
    }
    #[doc = "7 consecutive samples must agree."]
    #[inline(always)]
    pub fn filter_cnt_7(self) -> &'a mut W {
        self.variant(FILTER_CNT_A::FILTER_CNT_7)
    }
}
#[doc = "Field `EN` reader - Comparator Module Enable"]
pub type EN_R = crate::BitReader<EN_A>;
#[doc = "Comparator Module Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EN_A {
    #[doc = "0: Analog Comparator is disabled."]
    EN_0 = 0,
    #[doc = "1: Analog Comparator is enabled."]
    EN_1 = 1,
}
impl From<EN_A> for bool {
    #[inline(always)]
    fn from(variant: EN_A) -> Self {
        variant as u8 != 0
    }
}
impl EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EN_A {
        match self.bits {
            false => EN_A::EN_0,
            true => EN_A::EN_1,
        }
    }
    #[doc = "Checks if the value of the field is `EN_0`"]
    #[inline(always)]
    pub fn is_en_0(&self) -> bool {
        *self == EN_A::EN_0
    }
    #[doc = "Checks if the value of the field is `EN_1`"]
    #[inline(always)]
    pub fn is_en_1(&self) -> bool {
        *self == EN_A::EN_1
    }
}
#[doc = "Field `EN` writer - Comparator Module Enable"]
pub type EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, C0_SPEC, EN_A, O>;
impl<'a, const O: u8> EN_W<'a, O> {
    #[doc = "Analog Comparator is disabled."]
    #[inline(always)]
    pub fn en_0(self) -> &'a mut W {
        self.variant(EN_A::EN_0)
    }
    #[doc = "Analog Comparator is enabled."]
    #[inline(always)]
    pub fn en_1(self) -> &'a mut W {
        self.variant(EN_A::EN_1)
    }
}
#[doc = "Field `OPE` reader - Comparator Output Pin Enable"]
pub type OPE_R = crate::BitReader<OPE_A>;
#[doc = "Comparator Output Pin Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OPE_A {
    #[doc = "0: When OPE is 0, the comparator output (after window/filter settings dependent on software configuration) is not available to a packaged pin."]
    OPE_0 = 0,
    #[doc = "1: When OPE is 1, and if the software has configured the comparator to own a packaged pin, the comparator is available in a packaged pin."]
    OPE_1 = 1,
}
impl From<OPE_A> for bool {
    #[inline(always)]
    fn from(variant: OPE_A) -> Self {
        variant as u8 != 0
    }
}
impl OPE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OPE_A {
        match self.bits {
            false => OPE_A::OPE_0,
            true => OPE_A::OPE_1,
        }
    }
    #[doc = "Checks if the value of the field is `OPE_0`"]
    #[inline(always)]
    pub fn is_ope_0(&self) -> bool {
        *self == OPE_A::OPE_0
    }
    #[doc = "Checks if the value of the field is `OPE_1`"]
    #[inline(always)]
    pub fn is_ope_1(&self) -> bool {
        *self == OPE_A::OPE_1
    }
}
#[doc = "Field `OPE` writer - Comparator Output Pin Enable"]
pub type OPE_W<'a, const O: u8> = crate::BitWriter<'a, u32, C0_SPEC, OPE_A, O>;
impl<'a, const O: u8> OPE_W<'a, O> {
    #[doc = "When OPE is 0, the comparator output (after window/filter settings dependent on software configuration) is not available to a packaged pin."]
    #[inline(always)]
    pub fn ope_0(self) -> &'a mut W {
        self.variant(OPE_A::OPE_0)
    }
    #[doc = "When OPE is 1, and if the software has configured the comparator to own a packaged pin, the comparator is available in a packaged pin."]
    #[inline(always)]
    pub fn ope_1(self) -> &'a mut W {
        self.variant(OPE_A::OPE_1)
    }
}
#[doc = "Field `COS` reader - Comparator Output Select"]
pub type COS_R = crate::BitReader<COS_A>;
#[doc = "Comparator Output Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum COS_A {
    #[doc = "0: Set CMPO to equal COUT (filtered comparator output)."]
    COS_0 = 0,
    #[doc = "1: Set CMPO to equal COUTA (unfiltered comparator output)."]
    COS_1 = 1,
}
impl From<COS_A> for bool {
    #[inline(always)]
    fn from(variant: COS_A) -> Self {
        variant as u8 != 0
    }
}
impl COS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> COS_A {
        match self.bits {
            false => COS_A::COS_0,
            true => COS_A::COS_1,
        }
    }
    #[doc = "Checks if the value of the field is `COS_0`"]
    #[inline(always)]
    pub fn is_cos_0(&self) -> bool {
        *self == COS_A::COS_0
    }
    #[doc = "Checks if the value of the field is `COS_1`"]
    #[inline(always)]
    pub fn is_cos_1(&self) -> bool {
        *self == COS_A::COS_1
    }
}
#[doc = "Field `COS` writer - Comparator Output Select"]
pub type COS_W<'a, const O: u8> = crate::BitWriter<'a, u32, C0_SPEC, COS_A, O>;
impl<'a, const O: u8> COS_W<'a, O> {
    #[doc = "Set CMPO to equal COUT (filtered comparator output)."]
    #[inline(always)]
    pub fn cos_0(self) -> &'a mut W {
        self.variant(COS_A::COS_0)
    }
    #[doc = "Set CMPO to equal COUTA (unfiltered comparator output)."]
    #[inline(always)]
    pub fn cos_1(self) -> &'a mut W {
        self.variant(COS_A::COS_1)
    }
}
#[doc = "Field `INVT` reader - Comparator invert"]
pub type INVT_R = crate::BitReader<INVT_A>;
#[doc = "Comparator invert\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum INVT_A {
    #[doc = "0: Does not invert the comparator output."]
    INVT_0 = 0,
    #[doc = "1: Inverts the comparator output."]
    INVT_1 = 1,
}
impl From<INVT_A> for bool {
    #[inline(always)]
    fn from(variant: INVT_A) -> Self {
        variant as u8 != 0
    }
}
impl INVT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INVT_A {
        match self.bits {
            false => INVT_A::INVT_0,
            true => INVT_A::INVT_1,
        }
    }
    #[doc = "Checks if the value of the field is `INVT_0`"]
    #[inline(always)]
    pub fn is_invt_0(&self) -> bool {
        *self == INVT_A::INVT_0
    }
    #[doc = "Checks if the value of the field is `INVT_1`"]
    #[inline(always)]
    pub fn is_invt_1(&self) -> bool {
        *self == INVT_A::INVT_1
    }
}
#[doc = "Field `INVT` writer - Comparator invert"]
pub type INVT_W<'a, const O: u8> = crate::BitWriter<'a, u32, C0_SPEC, INVT_A, O>;
impl<'a, const O: u8> INVT_W<'a, O> {
    #[doc = "Does not invert the comparator output."]
    #[inline(always)]
    pub fn invt_0(self) -> &'a mut W {
        self.variant(INVT_A::INVT_0)
    }
    #[doc = "Inverts the comparator output."]
    #[inline(always)]
    pub fn invt_1(self) -> &'a mut W {
        self.variant(INVT_A::INVT_1)
    }
}
#[doc = "Field `PMODE` reader - Power Mode Select"]
pub type PMODE_R = crate::BitReader<PMODE_A>;
#[doc = "Power Mode Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PMODE_A {
    #[doc = "0: Low Speed (LS) comparison mode is selected."]
    PMODE_0 = 0,
    #[doc = "1: High Speed (HS) comparison mode is selected."]
    PMODE_1 = 1,
}
impl From<PMODE_A> for bool {
    #[inline(always)]
    fn from(variant: PMODE_A) -> Self {
        variant as u8 != 0
    }
}
impl PMODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PMODE_A {
        match self.bits {
            false => PMODE_A::PMODE_0,
            true => PMODE_A::PMODE_1,
        }
    }
    #[doc = "Checks if the value of the field is `PMODE_0`"]
    #[inline(always)]
    pub fn is_pmode_0(&self) -> bool {
        *self == PMODE_A::PMODE_0
    }
    #[doc = "Checks if the value of the field is `PMODE_1`"]
    #[inline(always)]
    pub fn is_pmode_1(&self) -> bool {
        *self == PMODE_A::PMODE_1
    }
}
#[doc = "Field `PMODE` writer - Power Mode Select"]
pub type PMODE_W<'a, const O: u8> = crate::BitWriter<'a, u32, C0_SPEC, PMODE_A, O>;
impl<'a, const O: u8> PMODE_W<'a, O> {
    #[doc = "Low Speed (LS) comparison mode is selected."]
    #[inline(always)]
    pub fn pmode_0(self) -> &'a mut W {
        self.variant(PMODE_A::PMODE_0)
    }
    #[doc = "High Speed (HS) comparison mode is selected."]
    #[inline(always)]
    pub fn pmode_1(self) -> &'a mut W {
        self.variant(PMODE_A::PMODE_1)
    }
}
#[doc = "Field `WE` reader - Windowing Enable"]
pub type WE_R = crate::BitReader<WE_A>;
#[doc = "Windowing Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WE_A {
    #[doc = "0: Windowing mode is not selected."]
    WE_0 = 0,
    #[doc = "1: Windowing mode is selected."]
    WE_1 = 1,
}
impl From<WE_A> for bool {
    #[inline(always)]
    fn from(variant: WE_A) -> Self {
        variant as u8 != 0
    }
}
impl WE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WE_A {
        match self.bits {
            false => WE_A::WE_0,
            true => WE_A::WE_1,
        }
    }
    #[doc = "Checks if the value of the field is `WE_0`"]
    #[inline(always)]
    pub fn is_we_0(&self) -> bool {
        *self == WE_A::WE_0
    }
    #[doc = "Checks if the value of the field is `WE_1`"]
    #[inline(always)]
    pub fn is_we_1(&self) -> bool {
        *self == WE_A::WE_1
    }
}
#[doc = "Field `WE` writer - Windowing Enable"]
pub type WE_W<'a, const O: u8> = crate::BitWriter<'a, u32, C0_SPEC, WE_A, O>;
impl<'a, const O: u8> WE_W<'a, O> {
    #[doc = "Windowing mode is not selected."]
    #[inline(always)]
    pub fn we_0(self) -> &'a mut W {
        self.variant(WE_A::WE_0)
    }
    #[doc = "Windowing mode is selected."]
    #[inline(always)]
    pub fn we_1(self) -> &'a mut W {
        self.variant(WE_A::WE_1)
    }
}
#[doc = "Field `SE` reader - Sample Enable"]
pub type SE_R = crate::BitReader<SE_A>;
#[doc = "Sample Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SE_A {
    #[doc = "0: Sampling mode is not selected."]
    SE_0 = 0,
    #[doc = "1: Sampling mode is selected."]
    SE_1 = 1,
}
impl From<SE_A> for bool {
    #[inline(always)]
    fn from(variant: SE_A) -> Self {
        variant as u8 != 0
    }
}
impl SE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SE_A {
        match self.bits {
            false => SE_A::SE_0,
            true => SE_A::SE_1,
        }
    }
    #[doc = "Checks if the value of the field is `SE_0`"]
    #[inline(always)]
    pub fn is_se_0(&self) -> bool {
        *self == SE_A::SE_0
    }
    #[doc = "Checks if the value of the field is `SE_1`"]
    #[inline(always)]
    pub fn is_se_1(&self) -> bool {
        *self == SE_A::SE_1
    }
}
#[doc = "Field `SE` writer - Sample Enable"]
pub type SE_W<'a, const O: u8> = crate::BitWriter<'a, u32, C0_SPEC, SE_A, O>;
impl<'a, const O: u8> SE_W<'a, O> {
    #[doc = "Sampling mode is not selected."]
    #[inline(always)]
    pub fn se_0(self) -> &'a mut W {
        self.variant(SE_A::SE_0)
    }
    #[doc = "Sampling mode is selected."]
    #[inline(always)]
    pub fn se_1(self) -> &'a mut W {
        self.variant(SE_A::SE_1)
    }
}
#[doc = "Field `FPR` reader - Filter Sample Period"]
pub type FPR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `FPR` writer - Filter Sample Period"]
pub type FPR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, C0_SPEC, u8, u8, 8, O>;
#[doc = "Field `COUT` reader - Analog Comparator Output"]
pub type COUT_R = crate::BitReader<bool>;
#[doc = "Field `CFF` reader - Analog Comparator Flag Falling"]
pub type CFF_R = crate::BitReader<CFF_A>;
#[doc = "Analog Comparator Flag Falling\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CFF_A {
    #[doc = "0: A falling edge has not been detected on COUT."]
    CFF_0 = 0,
    #[doc = "1: A falling edge on COUT has occurred."]
    CFF_1 = 1,
}
impl From<CFF_A> for bool {
    #[inline(always)]
    fn from(variant: CFF_A) -> Self {
        variant as u8 != 0
    }
}
impl CFF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CFF_A {
        match self.bits {
            false => CFF_A::CFF_0,
            true => CFF_A::CFF_1,
        }
    }
    #[doc = "Checks if the value of the field is `CFF_0`"]
    #[inline(always)]
    pub fn is_cff_0(&self) -> bool {
        *self == CFF_A::CFF_0
    }
    #[doc = "Checks if the value of the field is `CFF_1`"]
    #[inline(always)]
    pub fn is_cff_1(&self) -> bool {
        *self == CFF_A::CFF_1
    }
}
#[doc = "Field `CFF` writer - Analog Comparator Flag Falling"]
pub type CFF_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, C0_SPEC, CFF_A, O>;
impl<'a, const O: u8> CFF_W<'a, O> {
    #[doc = "A falling edge has not been detected on COUT."]
    #[inline(always)]
    pub fn cff_0(self) -> &'a mut W {
        self.variant(CFF_A::CFF_0)
    }
    #[doc = "A falling edge on COUT has occurred."]
    #[inline(always)]
    pub fn cff_1(self) -> &'a mut W {
        self.variant(CFF_A::CFF_1)
    }
}
#[doc = "Field `CFR` reader - Analog Comparator Flag Rising"]
pub type CFR_R = crate::BitReader<CFR_A>;
#[doc = "Analog Comparator Flag Rising\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CFR_A {
    #[doc = "0: A rising edge has not been detected on COUT."]
    CFR_0 = 0,
    #[doc = "1: A rising edge on COUT has occurred."]
    CFR_1 = 1,
}
impl From<CFR_A> for bool {
    #[inline(always)]
    fn from(variant: CFR_A) -> Self {
        variant as u8 != 0
    }
}
impl CFR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CFR_A {
        match self.bits {
            false => CFR_A::CFR_0,
            true => CFR_A::CFR_1,
        }
    }
    #[doc = "Checks if the value of the field is `CFR_0`"]
    #[inline(always)]
    pub fn is_cfr_0(&self) -> bool {
        *self == CFR_A::CFR_0
    }
    #[doc = "Checks if the value of the field is `CFR_1`"]
    #[inline(always)]
    pub fn is_cfr_1(&self) -> bool {
        *self == CFR_A::CFR_1
    }
}
#[doc = "Field `CFR` writer - Analog Comparator Flag Rising"]
pub type CFR_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, C0_SPEC, CFR_A, O>;
impl<'a, const O: u8> CFR_W<'a, O> {
    #[doc = "A rising edge has not been detected on COUT."]
    #[inline(always)]
    pub fn cfr_0(self) -> &'a mut W {
        self.variant(CFR_A::CFR_0)
    }
    #[doc = "A rising edge on COUT has occurred."]
    #[inline(always)]
    pub fn cfr_1(self) -> &'a mut W {
        self.variant(CFR_A::CFR_1)
    }
}
#[doc = "Field `IEF` reader - Comparator Interrupt Enable Falling"]
pub type IEF_R = crate::BitReader<IEF_A>;
#[doc = "Comparator Interrupt Enable Falling\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IEF_A {
    #[doc = "0: Interrupt is disabled."]
    IEF_0 = 0,
    #[doc = "1: Interrupt is enabled."]
    IEF_1 = 1,
}
impl From<IEF_A> for bool {
    #[inline(always)]
    fn from(variant: IEF_A) -> Self {
        variant as u8 != 0
    }
}
impl IEF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IEF_A {
        match self.bits {
            false => IEF_A::IEF_0,
            true => IEF_A::IEF_1,
        }
    }
    #[doc = "Checks if the value of the field is `IEF_0`"]
    #[inline(always)]
    pub fn is_ief_0(&self) -> bool {
        *self == IEF_A::IEF_0
    }
    #[doc = "Checks if the value of the field is `IEF_1`"]
    #[inline(always)]
    pub fn is_ief_1(&self) -> bool {
        *self == IEF_A::IEF_1
    }
}
#[doc = "Field `IEF` writer - Comparator Interrupt Enable Falling"]
pub type IEF_W<'a, const O: u8> = crate::BitWriter<'a, u32, C0_SPEC, IEF_A, O>;
impl<'a, const O: u8> IEF_W<'a, O> {
    #[doc = "Interrupt is disabled."]
    #[inline(always)]
    pub fn ief_0(self) -> &'a mut W {
        self.variant(IEF_A::IEF_0)
    }
    #[doc = "Interrupt is enabled."]
    #[inline(always)]
    pub fn ief_1(self) -> &'a mut W {
        self.variant(IEF_A::IEF_1)
    }
}
#[doc = "Field `IER` reader - Comparator Interrupt Enable Rising"]
pub type IER_R = crate::BitReader<IER_A>;
#[doc = "Comparator Interrupt Enable Rising\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IER_A {
    #[doc = "0: Interrupt is disabled."]
    IER_0 = 0,
    #[doc = "1: Interrupt is enabled."]
    IER_1 = 1,
}
impl From<IER_A> for bool {
    #[inline(always)]
    fn from(variant: IER_A) -> Self {
        variant as u8 != 0
    }
}
impl IER_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IER_A {
        match self.bits {
            false => IER_A::IER_0,
            true => IER_A::IER_1,
        }
    }
    #[doc = "Checks if the value of the field is `IER_0`"]
    #[inline(always)]
    pub fn is_ier_0(&self) -> bool {
        *self == IER_A::IER_0
    }
    #[doc = "Checks if the value of the field is `IER_1`"]
    #[inline(always)]
    pub fn is_ier_1(&self) -> bool {
        *self == IER_A::IER_1
    }
}
#[doc = "Field `IER` writer - Comparator Interrupt Enable Rising"]
pub type IER_W<'a, const O: u8> = crate::BitWriter<'a, u32, C0_SPEC, IER_A, O>;
impl<'a, const O: u8> IER_W<'a, O> {
    #[doc = "Interrupt is disabled."]
    #[inline(always)]
    pub fn ier_0(self) -> &'a mut W {
        self.variant(IER_A::IER_0)
    }
    #[doc = "Interrupt is enabled."]
    #[inline(always)]
    pub fn ier_1(self) -> &'a mut W {
        self.variant(IER_A::IER_1)
    }
}
#[doc = "Field `DMAEN` reader - DMA Enable"]
pub type DMAEN_R = crate::BitReader<DMAEN_A>;
#[doc = "DMA Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DMAEN_A {
    #[doc = "0: DMA is disabled."]
    DMAEN_0 = 0,
    #[doc = "1: DMA is enabled."]
    DMAEN_1 = 1,
}
impl From<DMAEN_A> for bool {
    #[inline(always)]
    fn from(variant: DMAEN_A) -> Self {
        variant as u8 != 0
    }
}
impl DMAEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DMAEN_A {
        match self.bits {
            false => DMAEN_A::DMAEN_0,
            true => DMAEN_A::DMAEN_1,
        }
    }
    #[doc = "Checks if the value of the field is `DMAEN_0`"]
    #[inline(always)]
    pub fn is_dmaen_0(&self) -> bool {
        *self == DMAEN_A::DMAEN_0
    }
    #[doc = "Checks if the value of the field is `DMAEN_1`"]
    #[inline(always)]
    pub fn is_dmaen_1(&self) -> bool {
        *self == DMAEN_A::DMAEN_1
    }
}
#[doc = "Field `DMAEN` writer - DMA Enable"]
pub type DMAEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, C0_SPEC, DMAEN_A, O>;
impl<'a, const O: u8> DMAEN_W<'a, O> {
    #[doc = "DMA is disabled."]
    #[inline(always)]
    pub fn dmaen_0(self) -> &'a mut W {
        self.variant(DMAEN_A::DMAEN_0)
    }
    #[doc = "DMA is enabled."]
    #[inline(always)]
    pub fn dmaen_1(self) -> &'a mut W {
        self.variant(DMAEN_A::DMAEN_1)
    }
}
#[doc = "Field `LINKEN` reader - CMP to DAC link enable."]
pub type LINKEN_R = crate::BitReader<LINKEN_A>;
#[doc = "CMP to DAC link enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LINKEN_A {
    #[doc = "0: CMP to DAC link is disabled"]
    LINKEN_0 = 0,
    #[doc = "1: CMP to DAC link is enabled."]
    LINKEN_1 = 1,
}
impl From<LINKEN_A> for bool {
    #[inline(always)]
    fn from(variant: LINKEN_A) -> Self {
        variant as u8 != 0
    }
}
impl LINKEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LINKEN_A {
        match self.bits {
            false => LINKEN_A::LINKEN_0,
            true => LINKEN_A::LINKEN_1,
        }
    }
    #[doc = "Checks if the value of the field is `LINKEN_0`"]
    #[inline(always)]
    pub fn is_linken_0(&self) -> bool {
        *self == LINKEN_A::LINKEN_0
    }
    #[doc = "Checks if the value of the field is `LINKEN_1`"]
    #[inline(always)]
    pub fn is_linken_1(&self) -> bool {
        *self == LINKEN_A::LINKEN_1
    }
}
#[doc = "Field `LINKEN` writer - CMP to DAC link enable."]
pub type LINKEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, C0_SPEC, LINKEN_A, O>;
impl<'a, const O: u8> LINKEN_W<'a, O> {
    #[doc = "CMP to DAC link is disabled"]
    #[inline(always)]
    pub fn linken_0(self) -> &'a mut W {
        self.variant(LINKEN_A::LINKEN_0)
    }
    #[doc = "CMP to DAC link is enabled."]
    #[inline(always)]
    pub fn linken_1(self) -> &'a mut W {
        self.variant(LINKEN_A::LINKEN_1)
    }
}
impl R {
    #[doc = "Bits 0:1 - Comparator hard block hysteresis control. See chip data sheet to get the actual hystersis value with each level"]
    #[inline(always)]
    pub fn hystctr(&self) -> HYSTCTR_R {
        HYSTCTR_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 4:6 - Filter Sample Count"]
    #[inline(always)]
    pub fn filter_cnt(&self) -> FILTER_CNT_R {
        FILTER_CNT_R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bit 8 - Comparator Module Enable"]
    #[inline(always)]
    pub fn en(&self) -> EN_R {
        EN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Comparator Output Pin Enable"]
    #[inline(always)]
    pub fn ope(&self) -> OPE_R {
        OPE_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Comparator Output Select"]
    #[inline(always)]
    pub fn cos(&self) -> COS_R {
        COS_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Comparator invert"]
    #[inline(always)]
    pub fn invt(&self) -> INVT_R {
        INVT_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Power Mode Select"]
    #[inline(always)]
    pub fn pmode(&self) -> PMODE_R {
        PMODE_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 14 - Windowing Enable"]
    #[inline(always)]
    pub fn we(&self) -> WE_R {
        WE_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Sample Enable"]
    #[inline(always)]
    pub fn se(&self) -> SE_R {
        SE_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:23 - Filter Sample Period"]
    #[inline(always)]
    pub fn fpr(&self) -> FPR_R {
        FPR_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bit 24 - Analog Comparator Output"]
    #[inline(always)]
    pub fn cout(&self) -> COUT_R {
        COUT_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Analog Comparator Flag Falling"]
    #[inline(always)]
    pub fn cff(&self) -> CFF_R {
        CFF_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Analog Comparator Flag Rising"]
    #[inline(always)]
    pub fn cfr(&self) -> CFR_R {
        CFR_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Comparator Interrupt Enable Falling"]
    #[inline(always)]
    pub fn ief(&self) -> IEF_R {
        IEF_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Comparator Interrupt Enable Rising"]
    #[inline(always)]
    pub fn ier(&self) -> IER_R {
        IER_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 30 - DMA Enable"]
    #[inline(always)]
    pub fn dmaen(&self) -> DMAEN_R {
        DMAEN_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - CMP to DAC link enable."]
    #[inline(always)]
    pub fn linken(&self) -> LINKEN_R {
        LINKEN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - Comparator hard block hysteresis control. See chip data sheet to get the actual hystersis value with each level"]
    #[inline(always)]
    #[must_use]
    pub fn hystctr(&mut self) -> HYSTCTR_W<0> {
        HYSTCTR_W::new(self)
    }
    #[doc = "Bits 4:6 - Filter Sample Count"]
    #[inline(always)]
    #[must_use]
    pub fn filter_cnt(&mut self) -> FILTER_CNT_W<4> {
        FILTER_CNT_W::new(self)
    }
    #[doc = "Bit 8 - Comparator Module Enable"]
    #[inline(always)]
    #[must_use]
    pub fn en(&mut self) -> EN_W<8> {
        EN_W::new(self)
    }
    #[doc = "Bit 9 - Comparator Output Pin Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ope(&mut self) -> OPE_W<9> {
        OPE_W::new(self)
    }
    #[doc = "Bit 10 - Comparator Output Select"]
    #[inline(always)]
    #[must_use]
    pub fn cos(&mut self) -> COS_W<10> {
        COS_W::new(self)
    }
    #[doc = "Bit 11 - Comparator invert"]
    #[inline(always)]
    #[must_use]
    pub fn invt(&mut self) -> INVT_W<11> {
        INVT_W::new(self)
    }
    #[doc = "Bit 12 - Power Mode Select"]
    #[inline(always)]
    #[must_use]
    pub fn pmode(&mut self) -> PMODE_W<12> {
        PMODE_W::new(self)
    }
    #[doc = "Bit 14 - Windowing Enable"]
    #[inline(always)]
    #[must_use]
    pub fn we(&mut self) -> WE_W<14> {
        WE_W::new(self)
    }
    #[doc = "Bit 15 - Sample Enable"]
    #[inline(always)]
    #[must_use]
    pub fn se(&mut self) -> SE_W<15> {
        SE_W::new(self)
    }
    #[doc = "Bits 16:23 - Filter Sample Period"]
    #[inline(always)]
    #[must_use]
    pub fn fpr(&mut self) -> FPR_W<16> {
        FPR_W::new(self)
    }
    #[doc = "Bit 25 - Analog Comparator Flag Falling"]
    #[inline(always)]
    #[must_use]
    pub fn cff(&mut self) -> CFF_W<25> {
        CFF_W::new(self)
    }
    #[doc = "Bit 26 - Analog Comparator Flag Rising"]
    #[inline(always)]
    #[must_use]
    pub fn cfr(&mut self) -> CFR_W<26> {
        CFR_W::new(self)
    }
    #[doc = "Bit 27 - Comparator Interrupt Enable Falling"]
    #[inline(always)]
    #[must_use]
    pub fn ief(&mut self) -> IEF_W<27> {
        IEF_W::new(self)
    }
    #[doc = "Bit 28 - Comparator Interrupt Enable Rising"]
    #[inline(always)]
    #[must_use]
    pub fn ier(&mut self) -> IER_W<28> {
        IER_W::new(self)
    }
    #[doc = "Bit 30 - DMA Enable"]
    #[inline(always)]
    #[must_use]
    pub fn dmaen(&mut self) -> DMAEN_W<30> {
        DMAEN_W::new(self)
    }
    #[doc = "Bit 31 - CMP to DAC link enable."]
    #[inline(always)]
    #[must_use]
    pub fn linken(&mut self) -> LINKEN_W<31> {
        LINKEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CMP Control Register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [c0](index.html) module"]
pub struct C0_SPEC;
impl crate::RegisterSpec for C0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [c0::R](R) reader structure"]
impl crate::Readable for C0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [c0::W](W) writer structure"]
impl crate::Writable for C0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0x0600_0000;
}
#[doc = "`reset()` method sets C0 to value 0"]
impl crate::Resettable for C0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
