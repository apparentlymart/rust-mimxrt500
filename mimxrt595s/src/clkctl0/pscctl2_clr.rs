#[doc = "Register `PSCCTL2_CLR` writer"]
pub struct W(crate::W<PSCCTL2_CLR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PSCCTL2_CLR_SPEC>;
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
impl From<crate::W<PSCCTL2_CLR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PSCCTL2_CLR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Micro-Tick Timer 0 clock clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UTICK0_CLK_AW {
    #[doc = "0: No effect"]
    NO_EFFECT = 0,
    #[doc = "1: Clears the corresponding bit in PSCCTL2 register"]
    CLK_ENABLE_CLEAR = 1,
}
impl From<UTICK0_CLK_AW> for bool {
    #[inline(always)]
    fn from(variant: UTICK0_CLK_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `UTICK0_CLK` writer - Micro-Tick Timer 0 clock clear"]
pub type UTICK0_CLK_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, PSCCTL2_CLR_SPEC, UTICK0_CLK_AW, O>;
impl<'a, const O: u8> UTICK0_CLK_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(UTICK0_CLK_AW::NO_EFFECT)
    }
    #[doc = "Clears the corresponding bit in PSCCTL2 register"]
    #[inline(always)]
    pub fn clk_enable_clear(self) -> &'a mut W {
        self.variant(UTICK0_CLK_AW::CLK_ENABLE_CLEAR)
    }
}
#[doc = "Watchdog Timer 0 clock clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WWDT0_CLK_AW {
    #[doc = "0: No effect"]
    NO_EFFECT = 0,
    #[doc = "1: Clears the corresponding bit in PSCCTL2 register"]
    CLK_ENABLE_CLEAR = 1,
}
impl From<WWDT0_CLK_AW> for bool {
    #[inline(always)]
    fn from(variant: WWDT0_CLK_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WWDT0_CLK` writer - Watchdog Timer 0 clock clear"]
pub type WWDT0_CLK_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, PSCCTL2_CLR_SPEC, WWDT0_CLK_AW, O>;
impl<'a, const O: u8> WWDT0_CLK_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(WWDT0_CLK_AW::NO_EFFECT)
    }
    #[doc = "Clears the corresponding bit in PSCCTL2 register"]
    #[inline(always)]
    pub fn clk_enable_clear(self) -> &'a mut W {
        self.variant(WWDT0_CLK_AW::CLK_ENABLE_CLEAR)
    }
}
#[doc = "Power Management Controller clock clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PMC_CLK_AW {
    #[doc = "0: No effect"]
    NO_EFFECT = 0,
    #[doc = "1: Clears the corresponding bit in PSCCTL2 register"]
    CLK_ENABLE_CLEAR = 1,
}
impl From<PMC_CLK_AW> for bool {
    #[inline(always)]
    fn from(variant: PMC_CLK_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PMC_CLK` writer - Power Management Controller clock clear"]
pub type PMC_CLK_W<'a, const O: u8> = crate::BitWriter<'a, u32, PSCCTL2_CLR_SPEC, PMC_CLK_AW, O>;
impl<'a, const O: u8> PMC_CLK_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(PMC_CLK_AW::NO_EFFECT)
    }
    #[doc = "Clears the corresponding bit in PSCCTL2 register"]
    #[inline(always)]
    pub fn clk_enable_clear(self) -> &'a mut W {
        self.variant(PMC_CLK_AW::CLK_ENABLE_CLEAR)
    }
}
impl W {
    #[doc = "Bit 0 - Micro-Tick Timer 0 clock clear"]
    #[inline(always)]
    #[must_use]
    pub fn utick0_clk(&mut self) -> UTICK0_CLK_W<0> {
        UTICK0_CLK_W::new(self)
    }
    #[doc = "Bit 1 - Watchdog Timer 0 clock clear"]
    #[inline(always)]
    #[must_use]
    pub fn wwdt0_clk(&mut self) -> WWDT0_CLK_W<1> {
        WWDT0_CLK_W::new(self)
    }
    #[doc = "Bit 29 - Power Management Controller clock clear"]
    #[inline(always)]
    #[must_use]
    pub fn pmc_clk(&mut self) -> PMC_CLK_W<29> {
        PMC_CLK_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Clock Control 2 Clear\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pscctl2_clr](index.html) module"]
pub struct PSCCTL2_CLR_SPEC;
impl crate::RegisterSpec for PSCCTL2_CLR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [pscctl2_clr::W](W) writer structure"]
impl crate::Writable for PSCCTL2_CLR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PSCCTL2_CLR to value 0"]
impl crate::Resettable for PSCCTL2_CLR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
