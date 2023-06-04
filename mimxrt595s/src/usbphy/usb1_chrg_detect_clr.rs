#[doc = "Register `USB1_CHRG_DETECT_CLR` reader"]
pub struct R(crate::R<USB1_CHRG_DETECT_CLR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<USB1_CHRG_DETECT_CLR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<USB1_CHRG_DETECT_CLR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<USB1_CHRG_DETECT_CLR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `USB1_CHRG_DETECT_CLR` writer"]
pub struct W(crate::W<USB1_CHRG_DETECT_CLR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<USB1_CHRG_DETECT_CLR_SPEC>;
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
impl From<crate::W<USB1_CHRG_DETECT_CLR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<USB1_CHRG_DETECT_CLR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PULLUP_DP` reader - PULLUP_DP"]
pub type PULLUP_DP_R = crate::BitReader<PULLUP_DP_A>;
#[doc = "PULLUP_DP\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PULLUP_DP_A {
    #[doc = "0: No effect"]
    DISABLE = 0,
    #[doc = "1: Clears the corresponding bit"]
    ENABLE = 1,
}
impl From<PULLUP_DP_A> for bool {
    #[inline(always)]
    fn from(variant: PULLUP_DP_A) -> Self {
        variant as u8 != 0
    }
}
impl PULLUP_DP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PULLUP_DP_A {
        match self.bits {
            false => PULLUP_DP_A::DISABLE,
            true => PULLUP_DP_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == PULLUP_DP_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == PULLUP_DP_A::ENABLE
    }
}
#[doc = "Field `PULLUP_DP` writer - PULLUP_DP"]
pub type PULLUP_DP_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, USB1_CHRG_DETECT_CLR_SPEC, PULLUP_DP_A, O>;
impl<'a, const O: u8> PULLUP_DP_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(PULLUP_DP_A::DISABLE)
    }
    #[doc = "Clears the corresponding bit"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(PULLUP_DP_A::ENABLE)
    }
}
#[doc = "Field `BGR_IBIAS` reader - BGR_IBIAS"]
pub type BGR_IBIAS_R = crate::BitReader<BGR_IBIAS_A>;
#[doc = "BGR_IBIAS\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BGR_IBIAS_A {
    #[doc = "0: No effect"]
    DISABLE = 0,
    #[doc = "1: Clears the corresponding bit"]
    ENABLE = 1,
}
impl From<BGR_IBIAS_A> for bool {
    #[inline(always)]
    fn from(variant: BGR_IBIAS_A) -> Self {
        variant as u8 != 0
    }
}
impl BGR_IBIAS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BGR_IBIAS_A {
        match self.bits {
            false => BGR_IBIAS_A::DISABLE,
            true => BGR_IBIAS_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == BGR_IBIAS_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == BGR_IBIAS_A::ENABLE
    }
}
#[doc = "Field `BGR_IBIAS` writer - BGR_IBIAS"]
pub type BGR_IBIAS_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, USB1_CHRG_DETECT_CLR_SPEC, BGR_IBIAS_A, O>;
impl<'a, const O: u8> BGR_IBIAS_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(BGR_IBIAS_A::DISABLE)
    }
    #[doc = "Clears the corresponding bit"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(BGR_IBIAS_A::ENABLE)
    }
}
impl R {
    #[doc = "Bit 2 - PULLUP_DP"]
    #[inline(always)]
    pub fn pullup_dp(&self) -> PULLUP_DP_R {
        PULLUP_DP_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 23 - BGR_IBIAS"]
    #[inline(always)]
    pub fn bgr_ibias(&self) -> BGR_IBIAS_R {
        BGR_IBIAS_R::new(((self.bits >> 23) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 2 - PULLUP_DP"]
    #[inline(always)]
    #[must_use]
    pub fn pullup_dp(&mut self) -> PULLUP_DP_W<2> {
        PULLUP_DP_W::new(self)
    }
    #[doc = "Bit 23 - BGR_IBIAS"]
    #[inline(always)]
    #[must_use]
    pub fn bgr_ibias(&mut self) -> BGR_IBIAS_W<23> {
        BGR_IBIAS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Charger Detect Control Clear\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usb1_chrg_detect_clr](index.html) module"]
pub struct USB1_CHRG_DETECT_CLR_SPEC;
impl crate::RegisterSpec for USB1_CHRG_DETECT_CLR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [usb1_chrg_detect_clr::R](R) reader structure"]
impl crate::Readable for USB1_CHRG_DETECT_CLR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [usb1_chrg_detect_clr::W](W) writer structure"]
impl crate::Writable for USB1_CHRG_DETECT_CLR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets USB1_CHRG_DETECT_CLR to value 0"]
impl crate::Resettable for USB1_CHRG_DETECT_CLR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
