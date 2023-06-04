#[doc = "Register `SYSOSCCTL0` reader"]
pub struct R(crate::R<SYSOSCCTL0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SYSOSCCTL0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SYSOSCCTL0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SYSOSCCTL0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SYSOSCCTL0` writer"]
pub struct W(crate::W<SYSOSCCTL0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SYSOSCCTL0_SPEC>;
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
impl From<crate::W<SYSOSCCTL0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SYSOSCCTL0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LP_ENABLE` reader - Low Power Mode Enable"]
pub type LP_ENABLE_R = crate::BitReader<LP_ENABLE_A>;
#[doc = "Low Power Mode Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LP_ENABLE_A {
    #[doc = "0: Enable High Gain Mode (HP)"]
    HIGH_GAIN_ENABLE = 0,
    #[doc = "1: Enable Low Power mode (LP)"]
    LOW_POWER_ENABLE = 1,
}
impl From<LP_ENABLE_A> for bool {
    #[inline(always)]
    fn from(variant: LP_ENABLE_A) -> Self {
        variant as u8 != 0
    }
}
impl LP_ENABLE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LP_ENABLE_A {
        match self.bits {
            false => LP_ENABLE_A::HIGH_GAIN_ENABLE,
            true => LP_ENABLE_A::LOW_POWER_ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `HIGH_GAIN_ENABLE`"]
    #[inline(always)]
    pub fn is_high_gain_enable(&self) -> bool {
        *self == LP_ENABLE_A::HIGH_GAIN_ENABLE
    }
    #[doc = "Checks if the value of the field is `LOW_POWER_ENABLE`"]
    #[inline(always)]
    pub fn is_low_power_enable(&self) -> bool {
        *self == LP_ENABLE_A::LOW_POWER_ENABLE
    }
}
#[doc = "Field `LP_ENABLE` writer - Low Power Mode Enable"]
pub type LP_ENABLE_W<'a, const O: u8> = crate::BitWriter<'a, u32, SYSOSCCTL0_SPEC, LP_ENABLE_A, O>;
impl<'a, const O: u8> LP_ENABLE_W<'a, O> {
    #[doc = "Enable High Gain Mode (HP)"]
    #[inline(always)]
    pub fn high_gain_enable(self) -> &'a mut W {
        self.variant(LP_ENABLE_A::HIGH_GAIN_ENABLE)
    }
    #[doc = "Enable Low Power mode (LP)"]
    #[inline(always)]
    pub fn low_power_enable(self) -> &'a mut W {
        self.variant(LP_ENABLE_A::LOW_POWER_ENABLE)
    }
}
#[doc = "Field `BYPASS_ENABLE` reader - Bypass Enable"]
pub type BYPASS_ENABLE_R = crate::BitReader<BYPASS_ENABLE_A>;
#[doc = "Bypass Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BYPASS_ENABLE_A {
    #[doc = "0: Enable Normal mode. Oscillation with crystal connected."]
    NORMAL_ENABLE = 0,
    #[doc = "1: Enable Bypass mode. In this mode a clock can be directly input into the XTALIN pin."]
    BYPASS_ENABLE = 1,
}
impl From<BYPASS_ENABLE_A> for bool {
    #[inline(always)]
    fn from(variant: BYPASS_ENABLE_A) -> Self {
        variant as u8 != 0
    }
}
impl BYPASS_ENABLE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BYPASS_ENABLE_A {
        match self.bits {
            false => BYPASS_ENABLE_A::NORMAL_ENABLE,
            true => BYPASS_ENABLE_A::BYPASS_ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `NORMAL_ENABLE`"]
    #[inline(always)]
    pub fn is_normal_enable(&self) -> bool {
        *self == BYPASS_ENABLE_A::NORMAL_ENABLE
    }
    #[doc = "Checks if the value of the field is `BYPASS_ENABLE`"]
    #[inline(always)]
    pub fn is_bypass_enable(&self) -> bool {
        *self == BYPASS_ENABLE_A::BYPASS_ENABLE
    }
}
#[doc = "Field `BYPASS_ENABLE` writer - Bypass Enable"]
pub type BYPASS_ENABLE_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SYSOSCCTL0_SPEC, BYPASS_ENABLE_A, O>;
impl<'a, const O: u8> BYPASS_ENABLE_W<'a, O> {
    #[doc = "Enable Normal mode. Oscillation with crystal connected."]
    #[inline(always)]
    pub fn normal_enable(self) -> &'a mut W {
        self.variant(BYPASS_ENABLE_A::NORMAL_ENABLE)
    }
    #[doc = "Enable Bypass mode. In this mode a clock can be directly input into the XTALIN pin."]
    #[inline(always)]
    pub fn bypass_enable(self) -> &'a mut W {
        self.variant(BYPASS_ENABLE_A::BYPASS_ENABLE)
    }
}
impl R {
    #[doc = "Bit 0 - Low Power Mode Enable"]
    #[inline(always)]
    pub fn lp_enable(&self) -> LP_ENABLE_R {
        LP_ENABLE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Bypass Enable"]
    #[inline(always)]
    pub fn bypass_enable(&self) -> BYPASS_ENABLE_R {
        BYPASS_ENABLE_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Low Power Mode Enable"]
    #[inline(always)]
    #[must_use]
    pub fn lp_enable(&mut self) -> LP_ENABLE_W<0> {
        LP_ENABLE_W::new(self)
    }
    #[doc = "Bit 1 - Bypass Enable"]
    #[inline(always)]
    #[must_use]
    pub fn bypass_enable(&mut self) -> BYPASS_ENABLE_W<1> {
        BYPASS_ENABLE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "System Oscillator Control 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sysoscctl0](index.html) module"]
pub struct SYSOSCCTL0_SPEC;
impl crate::RegisterSpec for SYSOSCCTL0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sysoscctl0::R](R) reader structure"]
impl crate::Readable for SYSOSCCTL0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sysoscctl0::W](W) writer structure"]
impl crate::Writable for SYSOSCCTL0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SYSOSCCTL0 to value 0"]
impl crate::Resettable for SYSOSCCTL0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
