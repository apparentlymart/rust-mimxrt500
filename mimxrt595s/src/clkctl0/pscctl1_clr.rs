#[doc = "Register `PSCCTL1_CLR` writer"]
pub struct W(crate::W<PSCCTL1_CLR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PSCCTL1_CLR_SPEC>;
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
impl From<crate::W<PSCCTL1_CLR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PSCCTL1_CLR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "SDIO0 clock clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SDIO0_CLK_AW {
    #[doc = "0: No effect"]
    NO_EFFECT = 0,
    #[doc = "1: Clears the corresponding bit in PSCCTL1 register"]
    CLK_ENABLE_CLEAR = 1,
}
impl From<SDIO0_CLK_AW> for bool {
    #[inline(always)]
    fn from(variant: SDIO0_CLK_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SDIO0_CLK` writer - SDIO0 clock clear"]
pub type SDIO0_CLK_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, PSCCTL1_CLR_SPEC, SDIO0_CLK_AW, O>;
impl<'a, const O: u8> SDIO0_CLK_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(SDIO0_CLK_AW::NO_EFFECT)
    }
    #[doc = "Clears the corresponding bit in PSCCTL1 register"]
    #[inline(always)]
    pub fn clk_enable_clear(self) -> &'a mut W {
        self.variant(SDIO0_CLK_AW::CLK_ENABLE_CLEAR)
    }
}
#[doc = "SDIO1 clock clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SDIO1_CLK_AW {
    #[doc = "0: No effect"]
    NO_EFFECT = 0,
    #[doc = "1: Clears the corresponding bit in PSCCTL1 register"]
    CLK_ENABLE_CLEAR = 1,
}
impl From<SDIO1_CLK_AW> for bool {
    #[inline(always)]
    fn from(variant: SDIO1_CLK_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SDIO1_CLK` writer - SDIO1 clock clear"]
pub type SDIO1_CLK_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, PSCCTL1_CLR_SPEC, SDIO1_CLK_AW, O>;
impl<'a, const O: u8> SDIO1_CLK_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(SDIO1_CLK_AW::NO_EFFECT)
    }
    #[doc = "Clears the corresponding bit in PSCCTL1 register"]
    #[inline(always)]
    pub fn clk_enable_clear(self) -> &'a mut W {
        self.variant(SDIO1_CLK_AW::CLK_ENABLE_CLEAR)
    }
}
#[doc = "ACMP0 clock clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ACMP0_CLK_AW {
    #[doc = "0: No effect"]
    NO_EFFECT = 0,
    #[doc = "1: Clears the corresponding bit in PSCCTL1 register"]
    CLK_ENABLE_CLEAR = 1,
}
impl From<ACMP0_CLK_AW> for bool {
    #[inline(always)]
    fn from(variant: ACMP0_CLK_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ACMP0_CLK` writer - ACMP0 clock clear"]
pub type ACMP0_CLK_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, PSCCTL1_CLR_SPEC, ACMP0_CLK_AW, O>;
impl<'a, const O: u8> ACMP0_CLK_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(ACMP0_CLK_AW::NO_EFFECT)
    }
    #[doc = "Clears the corresponding bit in PSCCTL1 register"]
    #[inline(always)]
    pub fn clk_enable_clear(self) -> &'a mut W {
        self.variant(ACMP0_CLK_AW::CLK_ENABLE_CLEAR)
    }
}
#[doc = "ADC0 clock clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADC0_CLK_AW {
    #[doc = "0: No effect"]
    NO_EFFECT = 0,
    #[doc = "1: Clears the corresponding bit in PSCCTL1 register"]
    CLK_ENABLE_CLEAR = 1,
}
impl From<ADC0_CLK_AW> for bool {
    #[inline(always)]
    fn from(variant: ADC0_CLK_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADC0_CLK` writer - ADC0 clock clear"]
pub type ADC0_CLK_W<'a, const O: u8> = crate::BitWriter<'a, u32, PSCCTL1_CLR_SPEC, ADC0_CLK_AW, O>;
impl<'a, const O: u8> ADC0_CLK_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(ADC0_CLK_AW::NO_EFFECT)
    }
    #[doc = "Clears the corresponding bit in PSCCTL1 register"]
    #[inline(always)]
    pub fn clk_enable_clear(self) -> &'a mut W {
        self.variant(ADC0_CLK_AW::CLK_ENABLE_CLEAR)
    }
}
#[doc = "SHSGPIO0 clock clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SHSGPIO0_CLK_AW {
    #[doc = "0: No effect"]
    NO_EFFECT = 0,
    #[doc = "1: Clears the corresponding bit in PSCCTL1 register"]
    CLK_ENABLE_CLEAR = 1,
}
impl From<SHSGPIO0_CLK_AW> for bool {
    #[inline(always)]
    fn from(variant: SHSGPIO0_CLK_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SHSGPIO0_CLK` writer - SHSGPIO0 clock clear"]
pub type SHSGPIO0_CLK_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, PSCCTL1_CLR_SPEC, SHSGPIO0_CLK_AW, O>;
impl<'a, const O: u8> SHSGPIO0_CLK_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(SHSGPIO0_CLK_AW::NO_EFFECT)
    }
    #[doc = "Clears the corresponding bit in PSCCTL1 register"]
    #[inline(always)]
    pub fn clk_enable_clear(self) -> &'a mut W {
        self.variant(SHSGPIO0_CLK_AW::CLK_ENABLE_CLEAR)
    }
}
impl W {
    #[doc = "Bit 2 - SDIO0 clock clear"]
    #[inline(always)]
    #[must_use]
    pub fn sdio0_clk(&mut self) -> SDIO0_CLK_W<2> {
        SDIO0_CLK_W::new(self)
    }
    #[doc = "Bit 3 - SDIO1 clock clear"]
    #[inline(always)]
    #[must_use]
    pub fn sdio1_clk(&mut self) -> SDIO1_CLK_W<3> {
        SDIO1_CLK_W::new(self)
    }
    #[doc = "Bit 15 - ACMP0 clock clear"]
    #[inline(always)]
    #[must_use]
    pub fn acmp0_clk(&mut self) -> ACMP0_CLK_W<15> {
        ACMP0_CLK_W::new(self)
    }
    #[doc = "Bit 16 - ADC0 clock clear"]
    #[inline(always)]
    #[must_use]
    pub fn adc0_clk(&mut self) -> ADC0_CLK_W<16> {
        ADC0_CLK_W::new(self)
    }
    #[doc = "Bit 24 - SHSGPIO0 clock clear"]
    #[inline(always)]
    #[must_use]
    pub fn shsgpio0_clk(&mut self) -> SHSGPIO0_CLK_W<24> {
        SHSGPIO0_CLK_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Clock Control 1 Clear\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pscctl1_clr](index.html) module"]
pub struct PSCCTL1_CLR_SPEC;
impl crate::RegisterSpec for PSCCTL1_CLR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [pscctl1_clr::W](W) writer structure"]
impl crate::Writable for PSCCTL1_CLR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PSCCTL1_CLR to value 0"]
impl crate::Resettable for PSCCTL1_CLR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
