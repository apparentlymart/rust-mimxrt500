#[doc = "Register `SYSPLL0CTL0` reader"]
pub struct R(crate::R<SYSPLL0CTL0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SYSPLL0CTL0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SYSPLL0CTL0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SYSPLL0CTL0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SYSPLL0CTL0` writer"]
pub struct W(crate::W<SYSPLL0CTL0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SYSPLL0CTL0_SPEC>;
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
impl From<crate::W<SYSPLL0CTL0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SYSPLL0CTL0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BYPASS` reader - SYSPLL0 BYPASS Mode"]
pub type BYPASS_R = crate::BitReader<BYPASS_A>;
#[doc = "SYSPLL0 BYPASS Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BYPASS_A {
    #[doc = "0: PFD outputs are PFD-programmed clocks"]
    PFD = 0,
    #[doc = "1: Bypass Mode: PFD outputs are sourced directly from rhe reference input clock"]
    BYPASS = 1,
}
impl From<BYPASS_A> for bool {
    #[inline(always)]
    fn from(variant: BYPASS_A) -> Self {
        variant as u8 != 0
    }
}
impl BYPASS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BYPASS_A {
        match self.bits {
            false => BYPASS_A::PFD,
            true => BYPASS_A::BYPASS,
        }
    }
    #[doc = "Checks if the value of the field is `PFD`"]
    #[inline(always)]
    pub fn is_pfd(&self) -> bool {
        *self == BYPASS_A::PFD
    }
    #[doc = "Checks if the value of the field is `BYPASS`"]
    #[inline(always)]
    pub fn is_bypass(&self) -> bool {
        *self == BYPASS_A::BYPASS
    }
}
#[doc = "Field `BYPASS` writer - SYSPLL0 BYPASS Mode"]
pub type BYPASS_W<'a, const O: u8> = crate::BitWriter<'a, u32, SYSPLL0CTL0_SPEC, BYPASS_A, O>;
impl<'a, const O: u8> BYPASS_W<'a, O> {
    #[doc = "PFD outputs are PFD-programmed clocks"]
    #[inline(always)]
    pub fn pfd(self) -> &'a mut W {
        self.variant(BYPASS_A::PFD)
    }
    #[doc = "Bypass Mode: PFD outputs are sourced directly from rhe reference input clock"]
    #[inline(always)]
    pub fn bypass(self) -> &'a mut W {
        self.variant(BYPASS_A::BYPASS)
    }
}
#[doc = "Field `RESET` reader - SYSPLL0 Reset"]
pub type RESET_R = crate::BitReader<RESET_A>;
#[doc = "SYSPLL0 Reset\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RESET_A {
    #[doc = "0: SYSPLL0 reset is removed"]
    NO_RESET = 0,
    #[doc = "1: SYSPLL0 is placed into reset"]
    RESET = 1,
}
impl From<RESET_A> for bool {
    #[inline(always)]
    fn from(variant: RESET_A) -> Self {
        variant as u8 != 0
    }
}
impl RESET_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RESET_A {
        match self.bits {
            false => RESET_A::NO_RESET,
            true => RESET_A::RESET,
        }
    }
    #[doc = "Checks if the value of the field is `NO_RESET`"]
    #[inline(always)]
    pub fn is_no_reset(&self) -> bool {
        *self == RESET_A::NO_RESET
    }
    #[doc = "Checks if the value of the field is `RESET`"]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == RESET_A::RESET
    }
}
#[doc = "Field `RESET` writer - SYSPLL0 Reset"]
pub type RESET_W<'a, const O: u8> = crate::BitWriter<'a, u32, SYSPLL0CTL0_SPEC, RESET_A, O>;
impl<'a, const O: u8> RESET_W<'a, O> {
    #[doc = "SYSPLL0 reset is removed"]
    #[inline(always)]
    pub fn no_reset(self) -> &'a mut W {
        self.variant(RESET_A::NO_RESET)
    }
    #[doc = "SYSPLL0 is placed into reset"]
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(RESET_A::RESET)
    }
}
#[doc = "Field `HOLDRINGOFF_ENA` reader - Hold Ring Off Control"]
pub type HOLDRINGOFF_ENA_R = crate::BitReader<HOLDRINGOFF_ENA_A>;
#[doc = "Hold Ring Off Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HOLDRINGOFF_ENA_A {
    #[doc = "0: Disable"]
    DISABLE = 0,
    #[doc = "1: Enable"]
    ENABLE = 1,
}
impl From<HOLDRINGOFF_ENA_A> for bool {
    #[inline(always)]
    fn from(variant: HOLDRINGOFF_ENA_A) -> Self {
        variant as u8 != 0
    }
}
impl HOLDRINGOFF_ENA_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HOLDRINGOFF_ENA_A {
        match self.bits {
            false => HOLDRINGOFF_ENA_A::DISABLE,
            true => HOLDRINGOFF_ENA_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == HOLDRINGOFF_ENA_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == HOLDRINGOFF_ENA_A::ENABLE
    }
}
#[doc = "Field `HOLDRINGOFF_ENA` writer - Hold Ring Off Control"]
pub type HOLDRINGOFF_ENA_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SYSPLL0CTL0_SPEC, HOLDRINGOFF_ENA_A, O>;
impl<'a, const O: u8> HOLDRINGOFF_ENA_W<'a, O> {
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(HOLDRINGOFF_ENA_A::DISABLE)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(HOLDRINGOFF_ENA_A::ENABLE)
    }
}
#[doc = "Field `MULT` reader - Multiplication Factor"]
pub type MULT_R = crate::FieldReader<u8, MULT_A>;
#[doc = "Multiplication Factor\n\nValue on reset: 22"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MULT_A {
    #[doc = "16: Multiply by 16"]
    DIV16 = 16,
    #[doc = "17: Multiply by 17"]
    DIV17 = 17,
    #[doc = "18: Multiply by 18"]
    DIV18 = 18,
    #[doc = "19: Multiply by 19"]
    DIV19 = 19,
    #[doc = "20: Multiply by 20"]
    DIV20 = 20,
    #[doc = "21: Multiply by 21"]
    DIV21 = 21,
    #[doc = "22: Multiply by 22"]
    DIV22 = 22,
}
impl From<MULT_A> for u8 {
    #[inline(always)]
    fn from(variant: MULT_A) -> Self {
        variant as _
    }
}
impl MULT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<MULT_A> {
        match self.bits {
            16 => Some(MULT_A::DIV16),
            17 => Some(MULT_A::DIV17),
            18 => Some(MULT_A::DIV18),
            19 => Some(MULT_A::DIV19),
            20 => Some(MULT_A::DIV20),
            21 => Some(MULT_A::DIV21),
            22 => Some(MULT_A::DIV22),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `DIV16`"]
    #[inline(always)]
    pub fn is_div16(&self) -> bool {
        *self == MULT_A::DIV16
    }
    #[doc = "Checks if the value of the field is `DIV17`"]
    #[inline(always)]
    pub fn is_div17(&self) -> bool {
        *self == MULT_A::DIV17
    }
    #[doc = "Checks if the value of the field is `DIV18`"]
    #[inline(always)]
    pub fn is_div18(&self) -> bool {
        *self == MULT_A::DIV18
    }
    #[doc = "Checks if the value of the field is `DIV19`"]
    #[inline(always)]
    pub fn is_div19(&self) -> bool {
        *self == MULT_A::DIV19
    }
    #[doc = "Checks if the value of the field is `DIV20`"]
    #[inline(always)]
    pub fn is_div20(&self) -> bool {
        *self == MULT_A::DIV20
    }
    #[doc = "Checks if the value of the field is `DIV21`"]
    #[inline(always)]
    pub fn is_div21(&self) -> bool {
        *self == MULT_A::DIV21
    }
    #[doc = "Checks if the value of the field is `DIV22`"]
    #[inline(always)]
    pub fn is_div22(&self) -> bool {
        *self == MULT_A::DIV22
    }
}
#[doc = "Field `MULT` writer - Multiplication Factor"]
pub type MULT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SYSPLL0CTL0_SPEC, u8, MULT_A, 8, O>;
impl<'a, const O: u8> MULT_W<'a, O> {
    #[doc = "Multiply by 16"]
    #[inline(always)]
    pub fn div16(self) -> &'a mut W {
        self.variant(MULT_A::DIV16)
    }
    #[doc = "Multiply by 17"]
    #[inline(always)]
    pub fn div17(self) -> &'a mut W {
        self.variant(MULT_A::DIV17)
    }
    #[doc = "Multiply by 18"]
    #[inline(always)]
    pub fn div18(self) -> &'a mut W {
        self.variant(MULT_A::DIV18)
    }
    #[doc = "Multiply by 19"]
    #[inline(always)]
    pub fn div19(self) -> &'a mut W {
        self.variant(MULT_A::DIV19)
    }
    #[doc = "Multiply by 20"]
    #[inline(always)]
    pub fn div20(self) -> &'a mut W {
        self.variant(MULT_A::DIV20)
    }
    #[doc = "Multiply by 21"]
    #[inline(always)]
    pub fn div21(self) -> &'a mut W {
        self.variant(MULT_A::DIV21)
    }
    #[doc = "Multiply by 22"]
    #[inline(always)]
    pub fn div22(self) -> &'a mut W {
        self.variant(MULT_A::DIV22)
    }
}
impl R {
    #[doc = "Bit 0 - SYSPLL0 BYPASS Mode"]
    #[inline(always)]
    pub fn bypass(&self) -> BYPASS_R {
        BYPASS_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - SYSPLL0 Reset"]
    #[inline(always)]
    pub fn reset(&self) -> RESET_R {
        RESET_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 13 - Hold Ring Off Control"]
    #[inline(always)]
    pub fn holdringoff_ena(&self) -> HOLDRINGOFF_ENA_R {
        HOLDRINGOFF_ENA_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bits 16:23 - Multiplication Factor"]
    #[inline(always)]
    pub fn mult(&self) -> MULT_R {
        MULT_R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - SYSPLL0 BYPASS Mode"]
    #[inline(always)]
    #[must_use]
    pub fn bypass(&mut self) -> BYPASS_W<0> {
        BYPASS_W::new(self)
    }
    #[doc = "Bit 1 - SYSPLL0 Reset"]
    #[inline(always)]
    #[must_use]
    pub fn reset(&mut self) -> RESET_W<1> {
        RESET_W::new(self)
    }
    #[doc = "Bit 13 - Hold Ring Off Control"]
    #[inline(always)]
    #[must_use]
    pub fn holdringoff_ena(&mut self) -> HOLDRINGOFF_ENA_W<13> {
        HOLDRINGOFF_ENA_W::new(self)
    }
    #[doc = "Bits 16:23 - Multiplication Factor"]
    #[inline(always)]
    #[must_use]
    pub fn mult(&mut self) -> MULT_W<16> {
        MULT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "System PLL0 Control 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [syspll0ctl0](index.html) module"]
pub struct SYSPLL0CTL0_SPEC;
impl crate::RegisterSpec for SYSPLL0CTL0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [syspll0ctl0::R](R) reader structure"]
impl crate::Readable for SYSPLL0CTL0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [syspll0ctl0::W](W) writer structure"]
impl crate::Writable for SYSPLL0CTL0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SYSPLL0CTL0 to value 0x0016_0002"]
impl crate::Resettable for SYSPLL0CTL0_SPEC {
    const RESET_VALUE: Self::Ux = 0x0016_0002;
}
