#[doc = "Register `PSCCTL1` reader"]
pub struct R(crate::R<PSCCTL1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PSCCTL1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PSCCTL1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PSCCTL1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PSCCTL1` writer"]
pub struct W(crate::W<PSCCTL1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PSCCTL1_SPEC>;
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
impl From<crate::W<PSCCTL1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PSCCTL1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SDIO0_CLK` reader - SDIO0 clock control"]
pub type SDIO0_CLK_R = crate::BitReader<SDIO0_CLK_A>;
#[doc = "SDIO0 clock control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SDIO0_CLK_A {
    #[doc = "0: Disable clock"]
    CLK_DISABLE = 0,
    #[doc = "1: Enable clock"]
    CLK_ENABLE = 1,
}
impl From<SDIO0_CLK_A> for bool {
    #[inline(always)]
    fn from(variant: SDIO0_CLK_A) -> Self {
        variant as u8 != 0
    }
}
impl SDIO0_CLK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SDIO0_CLK_A {
        match self.bits {
            false => SDIO0_CLK_A::CLK_DISABLE,
            true => SDIO0_CLK_A::CLK_ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `CLK_DISABLE`"]
    #[inline(always)]
    pub fn is_clk_disable(&self) -> bool {
        *self == SDIO0_CLK_A::CLK_DISABLE
    }
    #[doc = "Checks if the value of the field is `CLK_ENABLE`"]
    #[inline(always)]
    pub fn is_clk_enable(&self) -> bool {
        *self == SDIO0_CLK_A::CLK_ENABLE
    }
}
#[doc = "Field `SDIO0_CLK` writer - SDIO0 clock control"]
pub type SDIO0_CLK_W<'a, const O: u8> = crate::BitWriter<'a, u32, PSCCTL1_SPEC, SDIO0_CLK_A, O>;
impl<'a, const O: u8> SDIO0_CLK_W<'a, O> {
    #[doc = "Disable clock"]
    #[inline(always)]
    pub fn clk_disable(self) -> &'a mut W {
        self.variant(SDIO0_CLK_A::CLK_DISABLE)
    }
    #[doc = "Enable clock"]
    #[inline(always)]
    pub fn clk_enable(self) -> &'a mut W {
        self.variant(SDIO0_CLK_A::CLK_ENABLE)
    }
}
#[doc = "Field `SDIO1_CLK` reader - SDIO1 clock control"]
pub type SDIO1_CLK_R = crate::BitReader<SDIO1_CLK_A>;
#[doc = "SDIO1 clock control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SDIO1_CLK_A {
    #[doc = "0: Disable clock"]
    CLK_DISABLE = 0,
    #[doc = "1: Enable clock"]
    CLK_ENABLE = 1,
}
impl From<SDIO1_CLK_A> for bool {
    #[inline(always)]
    fn from(variant: SDIO1_CLK_A) -> Self {
        variant as u8 != 0
    }
}
impl SDIO1_CLK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SDIO1_CLK_A {
        match self.bits {
            false => SDIO1_CLK_A::CLK_DISABLE,
            true => SDIO1_CLK_A::CLK_ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `CLK_DISABLE`"]
    #[inline(always)]
    pub fn is_clk_disable(&self) -> bool {
        *self == SDIO1_CLK_A::CLK_DISABLE
    }
    #[doc = "Checks if the value of the field is `CLK_ENABLE`"]
    #[inline(always)]
    pub fn is_clk_enable(&self) -> bool {
        *self == SDIO1_CLK_A::CLK_ENABLE
    }
}
#[doc = "Field `SDIO1_CLK` writer - SDIO1 clock control"]
pub type SDIO1_CLK_W<'a, const O: u8> = crate::BitWriter<'a, u32, PSCCTL1_SPEC, SDIO1_CLK_A, O>;
impl<'a, const O: u8> SDIO1_CLK_W<'a, O> {
    #[doc = "Disable clock"]
    #[inline(always)]
    pub fn clk_disable(self) -> &'a mut W {
        self.variant(SDIO1_CLK_A::CLK_DISABLE)
    }
    #[doc = "Enable clock"]
    #[inline(always)]
    pub fn clk_enable(self) -> &'a mut W {
        self.variant(SDIO1_CLK_A::CLK_ENABLE)
    }
}
#[doc = "Field `ACMP0_CLK` reader - ACMP0 clock control"]
pub type ACMP0_CLK_R = crate::BitReader<ACMP0_CLK_A>;
#[doc = "ACMP0 clock control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ACMP0_CLK_A {
    #[doc = "0: Disable clock"]
    CLK_DISABLE = 0,
    #[doc = "1: Enable clock"]
    CLK_ENABLE = 1,
}
impl From<ACMP0_CLK_A> for bool {
    #[inline(always)]
    fn from(variant: ACMP0_CLK_A) -> Self {
        variant as u8 != 0
    }
}
impl ACMP0_CLK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ACMP0_CLK_A {
        match self.bits {
            false => ACMP0_CLK_A::CLK_DISABLE,
            true => ACMP0_CLK_A::CLK_ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `CLK_DISABLE`"]
    #[inline(always)]
    pub fn is_clk_disable(&self) -> bool {
        *self == ACMP0_CLK_A::CLK_DISABLE
    }
    #[doc = "Checks if the value of the field is `CLK_ENABLE`"]
    #[inline(always)]
    pub fn is_clk_enable(&self) -> bool {
        *self == ACMP0_CLK_A::CLK_ENABLE
    }
}
#[doc = "Field `ACMP0_CLK` writer - ACMP0 clock control"]
pub type ACMP0_CLK_W<'a, const O: u8> = crate::BitWriter<'a, u32, PSCCTL1_SPEC, ACMP0_CLK_A, O>;
impl<'a, const O: u8> ACMP0_CLK_W<'a, O> {
    #[doc = "Disable clock"]
    #[inline(always)]
    pub fn clk_disable(self) -> &'a mut W {
        self.variant(ACMP0_CLK_A::CLK_DISABLE)
    }
    #[doc = "Enable clock"]
    #[inline(always)]
    pub fn clk_enable(self) -> &'a mut W {
        self.variant(ACMP0_CLK_A::CLK_ENABLE)
    }
}
#[doc = "Field `ADC0_CLK` reader - ADC0 clock control"]
pub type ADC0_CLK_R = crate::BitReader<ADC0_CLK_A>;
#[doc = "ADC0 clock control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADC0_CLK_A {
    #[doc = "0: Disable clock"]
    CLK_DISABLE = 0,
    #[doc = "1: Enable clock"]
    CLK_ENABLE = 1,
}
impl From<ADC0_CLK_A> for bool {
    #[inline(always)]
    fn from(variant: ADC0_CLK_A) -> Self {
        variant as u8 != 0
    }
}
impl ADC0_CLK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADC0_CLK_A {
        match self.bits {
            false => ADC0_CLK_A::CLK_DISABLE,
            true => ADC0_CLK_A::CLK_ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `CLK_DISABLE`"]
    #[inline(always)]
    pub fn is_clk_disable(&self) -> bool {
        *self == ADC0_CLK_A::CLK_DISABLE
    }
    #[doc = "Checks if the value of the field is `CLK_ENABLE`"]
    #[inline(always)]
    pub fn is_clk_enable(&self) -> bool {
        *self == ADC0_CLK_A::CLK_ENABLE
    }
}
#[doc = "Field `ADC0_CLK` writer - ADC0 clock control"]
pub type ADC0_CLK_W<'a, const O: u8> = crate::BitWriter<'a, u32, PSCCTL1_SPEC, ADC0_CLK_A, O>;
impl<'a, const O: u8> ADC0_CLK_W<'a, O> {
    #[doc = "Disable clock"]
    #[inline(always)]
    pub fn clk_disable(self) -> &'a mut W {
        self.variant(ADC0_CLK_A::CLK_DISABLE)
    }
    #[doc = "Enable clock"]
    #[inline(always)]
    pub fn clk_enable(self) -> &'a mut W {
        self.variant(ADC0_CLK_A::CLK_ENABLE)
    }
}
#[doc = "Field `SHSGPIO0_CLK` reader - SHSGPIO0 clock control"]
pub type SHSGPIO0_CLK_R = crate::BitReader<SHSGPIO0_CLK_A>;
#[doc = "SHSGPIO0 clock control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SHSGPIO0_CLK_A {
    #[doc = "0: Disable clock"]
    CLK_DISABLE = 0,
    #[doc = "1: Enable clock"]
    CLK_ENABLE = 1,
}
impl From<SHSGPIO0_CLK_A> for bool {
    #[inline(always)]
    fn from(variant: SHSGPIO0_CLK_A) -> Self {
        variant as u8 != 0
    }
}
impl SHSGPIO0_CLK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SHSGPIO0_CLK_A {
        match self.bits {
            false => SHSGPIO0_CLK_A::CLK_DISABLE,
            true => SHSGPIO0_CLK_A::CLK_ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `CLK_DISABLE`"]
    #[inline(always)]
    pub fn is_clk_disable(&self) -> bool {
        *self == SHSGPIO0_CLK_A::CLK_DISABLE
    }
    #[doc = "Checks if the value of the field is `CLK_ENABLE`"]
    #[inline(always)]
    pub fn is_clk_enable(&self) -> bool {
        *self == SHSGPIO0_CLK_A::CLK_ENABLE
    }
}
#[doc = "Field `SHSGPIO0_CLK` writer - SHSGPIO0 clock control"]
pub type SHSGPIO0_CLK_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, PSCCTL1_SPEC, SHSGPIO0_CLK_A, O>;
impl<'a, const O: u8> SHSGPIO0_CLK_W<'a, O> {
    #[doc = "Disable clock"]
    #[inline(always)]
    pub fn clk_disable(self) -> &'a mut W {
        self.variant(SHSGPIO0_CLK_A::CLK_DISABLE)
    }
    #[doc = "Enable clock"]
    #[inline(always)]
    pub fn clk_enable(self) -> &'a mut W {
        self.variant(SHSGPIO0_CLK_A::CLK_ENABLE)
    }
}
impl R {
    #[doc = "Bit 2 - SDIO0 clock control"]
    #[inline(always)]
    pub fn sdio0_clk(&self) -> SDIO0_CLK_R {
        SDIO0_CLK_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - SDIO1 clock control"]
    #[inline(always)]
    pub fn sdio1_clk(&self) -> SDIO1_CLK_R {
        SDIO1_CLK_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 15 - ACMP0 clock control"]
    #[inline(always)]
    pub fn acmp0_clk(&self) -> ACMP0_CLK_R {
        ACMP0_CLK_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - ADC0 clock control"]
    #[inline(always)]
    pub fn adc0_clk(&self) -> ADC0_CLK_R {
        ADC0_CLK_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 24 - SHSGPIO0 clock control"]
    #[inline(always)]
    pub fn shsgpio0_clk(&self) -> SHSGPIO0_CLK_R {
        SHSGPIO0_CLK_R::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 2 - SDIO0 clock control"]
    #[inline(always)]
    #[must_use]
    pub fn sdio0_clk(&mut self) -> SDIO0_CLK_W<2> {
        SDIO0_CLK_W::new(self)
    }
    #[doc = "Bit 3 - SDIO1 clock control"]
    #[inline(always)]
    #[must_use]
    pub fn sdio1_clk(&mut self) -> SDIO1_CLK_W<3> {
        SDIO1_CLK_W::new(self)
    }
    #[doc = "Bit 15 - ACMP0 clock control"]
    #[inline(always)]
    #[must_use]
    pub fn acmp0_clk(&mut self) -> ACMP0_CLK_W<15> {
        ACMP0_CLK_W::new(self)
    }
    #[doc = "Bit 16 - ADC0 clock control"]
    #[inline(always)]
    #[must_use]
    pub fn adc0_clk(&mut self) -> ADC0_CLK_W<16> {
        ADC0_CLK_W::new(self)
    }
    #[doc = "Bit 24 - SHSGPIO0 clock control"]
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
#[doc = "Clock Control 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pscctl1](index.html) module"]
pub struct PSCCTL1_SPEC;
impl crate::RegisterSpec for PSCCTL1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pscctl1::R](R) reader structure"]
impl crate::Readable for PSCCTL1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pscctl1::W](W) writer structure"]
impl crate::Writable for PSCCTL1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PSCCTL1 to value 0"]
impl crate::Resettable for PSCCTL1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
