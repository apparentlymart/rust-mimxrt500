#[doc = "Register `PRSTCTL1_SET` writer"]
pub struct W(crate::W<PRSTCTL1_SET_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PRSTCTL1_SET_SPEC>;
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
impl From<crate::W<PRSTCTL1_SET_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PRSTCTL1_SET_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "SDIO0 reset set\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SDIO0_AW {
    #[doc = "0: No effect"]
    SDIO0_CLR = 0,
    #[doc = "1: Sets the PRSTCTL1 Bit"]
    SDIO0_SET = 1,
}
impl From<SDIO0_AW> for bool {
    #[inline(always)]
    fn from(variant: SDIO0_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SDIO0` writer - SDIO0 reset set"]
pub type SDIO0_W<'a, const O: u8> = crate::BitWriter<'a, u32, PRSTCTL1_SET_SPEC, SDIO0_AW, O>;
impl<'a, const O: u8> SDIO0_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn sdio0_clr(self) -> &'a mut W {
        self.variant(SDIO0_AW::SDIO0_CLR)
    }
    #[doc = "Sets the PRSTCTL1 Bit"]
    #[inline(always)]
    pub fn sdio0_set(self) -> &'a mut W {
        self.variant(SDIO0_AW::SDIO0_SET)
    }
}
#[doc = "SDIO1 reset set\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SDIO1_AW {
    #[doc = "0: No effect"]
    SDIO1_CLR = 0,
    #[doc = "1: Sets the PRSTCTL1 Bit"]
    SDIO1_SET = 1,
}
impl From<SDIO1_AW> for bool {
    #[inline(always)]
    fn from(variant: SDIO1_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SDIO1` writer - SDIO1 reset set"]
pub type SDIO1_W<'a, const O: u8> = crate::BitWriter<'a, u32, PRSTCTL1_SET_SPEC, SDIO1_AW, O>;
impl<'a, const O: u8> SDIO1_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn sdio1_clr(self) -> &'a mut W {
        self.variant(SDIO1_AW::SDIO1_CLR)
    }
    #[doc = "Sets the PRSTCTL1 Bit"]
    #[inline(always)]
    pub fn sdio1_set(self) -> &'a mut W {
        self.variant(SDIO1_AW::SDIO1_SET)
    }
}
#[doc = "ACMP0 reset set\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ACMP0_AW {
    #[doc = "0: No effect"]
    ACMP0_CLR = 0,
    #[doc = "1: Sets the PRSTCTL1 Bit"]
    ACMP0_SET = 1,
}
impl From<ACMP0_AW> for bool {
    #[inline(always)]
    fn from(variant: ACMP0_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ACMP0` writer - ACMP0 reset set"]
pub type ACMP0_W<'a, const O: u8> = crate::BitWriter<'a, u32, PRSTCTL1_SET_SPEC, ACMP0_AW, O>;
impl<'a, const O: u8> ACMP0_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn acmp0_clr(self) -> &'a mut W {
        self.variant(ACMP0_AW::ACMP0_CLR)
    }
    #[doc = "Sets the PRSTCTL1 Bit"]
    #[inline(always)]
    pub fn acmp0_set(self) -> &'a mut W {
        self.variant(ACMP0_AW::ACMP0_SET)
    }
}
#[doc = "ADC0 reset set\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADC0_AW {
    #[doc = "0: No effect"]
    ADC0_CLR = 0,
    #[doc = "1: Sets the PRSTCTL1 Bit"]
    ADC0_SET = 1,
}
impl From<ADC0_AW> for bool {
    #[inline(always)]
    fn from(variant: ADC0_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADC0` writer - ADC0 reset set"]
pub type ADC0_W<'a, const O: u8> = crate::BitWriter<'a, u32, PRSTCTL1_SET_SPEC, ADC0_AW, O>;
impl<'a, const O: u8> ADC0_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn adc0_clr(self) -> &'a mut W {
        self.variant(ADC0_AW::ADC0_CLR)
    }
    #[doc = "Sets the PRSTCTL1 Bit"]
    #[inline(always)]
    pub fn adc0_set(self) -> &'a mut W {
        self.variant(ADC0_AW::ADC0_SET)
    }
}
#[doc = "SHSGPIO0 reset set\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SHSGPIO0_AW {
    #[doc = "0: No effect"]
    SHSGPIO0_CLR = 0,
    #[doc = "1: Sets the PRSTCTL1 Bit"]
    SHSGPIO0_SET = 1,
}
impl From<SHSGPIO0_AW> for bool {
    #[inline(always)]
    fn from(variant: SHSGPIO0_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SHSGPIO0` writer - SHSGPIO0 reset set"]
pub type SHSGPIO0_W<'a, const O: u8> = crate::BitWriter<'a, u32, PRSTCTL1_SET_SPEC, SHSGPIO0_AW, O>;
impl<'a, const O: u8> SHSGPIO0_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn shsgpio0_clr(self) -> &'a mut W {
        self.variant(SHSGPIO0_AW::SHSGPIO0_CLR)
    }
    #[doc = "Sets the PRSTCTL1 Bit"]
    #[inline(always)]
    pub fn shsgpio0_set(self) -> &'a mut W {
        self.variant(SHSGPIO0_AW::SHSGPIO0_SET)
    }
}
impl W {
    #[doc = "Bit 2 - SDIO0 reset set"]
    #[inline(always)]
    #[must_use]
    pub fn sdio0(&mut self) -> SDIO0_W<2> {
        SDIO0_W::new(self)
    }
    #[doc = "Bit 3 - SDIO1 reset set"]
    #[inline(always)]
    #[must_use]
    pub fn sdio1(&mut self) -> SDIO1_W<3> {
        SDIO1_W::new(self)
    }
    #[doc = "Bit 15 - ACMP0 reset set"]
    #[inline(always)]
    #[must_use]
    pub fn acmp0(&mut self) -> ACMP0_W<15> {
        ACMP0_W::new(self)
    }
    #[doc = "Bit 16 - ADC0 reset set"]
    #[inline(always)]
    #[must_use]
    pub fn adc0(&mut self) -> ADC0_W<16> {
        ADC0_W::new(self)
    }
    #[doc = "Bit 24 - SHSGPIO0 reset set"]
    #[inline(always)]
    #[must_use]
    pub fn shsgpio0(&mut self) -> SHSGPIO0_W<24> {
        SHSGPIO0_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Peripheral Reset Control Register 1 SET\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [prstctl1_set](index.html) module"]
pub struct PRSTCTL1_SET_SPEC;
impl crate::RegisterSpec for PRSTCTL1_SET_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [prstctl1_set::W](W) writer structure"]
impl crate::Writable for PRSTCTL1_SET_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PRSTCTL1_SET to value 0"]
impl crate::Resettable for PRSTCTL1_SET_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
