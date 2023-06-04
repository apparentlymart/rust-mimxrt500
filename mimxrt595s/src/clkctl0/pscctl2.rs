#[doc = "Register `PSCCTL2` reader"]
pub struct R(crate::R<PSCCTL2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PSCCTL2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PSCCTL2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PSCCTL2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PSCCTL2` writer"]
pub struct W(crate::W<PSCCTL2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PSCCTL2_SPEC>;
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
impl From<crate::W<PSCCTL2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PSCCTL2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `UTICK0_CLK` reader - Micro-Tick Timer 0 clock control"]
pub type UTICK0_CLK_R = crate::BitReader<UTICK0_CLK_A>;
#[doc = "Micro-Tick Timer 0 clock control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UTICK0_CLK_A {
    #[doc = "0: Disable clock"]
    CLK_DISABLE = 0,
    #[doc = "1: Enable clock"]
    CLK_ENABLE = 1,
}
impl From<UTICK0_CLK_A> for bool {
    #[inline(always)]
    fn from(variant: UTICK0_CLK_A) -> Self {
        variant as u8 != 0
    }
}
impl UTICK0_CLK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> UTICK0_CLK_A {
        match self.bits {
            false => UTICK0_CLK_A::CLK_DISABLE,
            true => UTICK0_CLK_A::CLK_ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `CLK_DISABLE`"]
    #[inline(always)]
    pub fn is_clk_disable(&self) -> bool {
        *self == UTICK0_CLK_A::CLK_DISABLE
    }
    #[doc = "Checks if the value of the field is `CLK_ENABLE`"]
    #[inline(always)]
    pub fn is_clk_enable(&self) -> bool {
        *self == UTICK0_CLK_A::CLK_ENABLE
    }
}
#[doc = "Field `UTICK0_CLK` writer - Micro-Tick Timer 0 clock control"]
pub type UTICK0_CLK_W<'a, const O: u8> = crate::BitWriter<'a, u32, PSCCTL2_SPEC, UTICK0_CLK_A, O>;
impl<'a, const O: u8> UTICK0_CLK_W<'a, O> {
    #[doc = "Disable clock"]
    #[inline(always)]
    pub fn clk_disable(self) -> &'a mut W {
        self.variant(UTICK0_CLK_A::CLK_DISABLE)
    }
    #[doc = "Enable clock"]
    #[inline(always)]
    pub fn clk_enable(self) -> &'a mut W {
        self.variant(UTICK0_CLK_A::CLK_ENABLE)
    }
}
#[doc = "Field `WWDT0_CLK` reader - Watchdog Timer 0 clock control"]
pub type WWDT0_CLK_R = crate::BitReader<WWDT0_CLK_A>;
#[doc = "Watchdog Timer 0 clock control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WWDT0_CLK_A {
    #[doc = "0: Disable clock"]
    CLK_DISABLE = 0,
    #[doc = "1: Enable clock"]
    CLK_ENABLE = 1,
}
impl From<WWDT0_CLK_A> for bool {
    #[inline(always)]
    fn from(variant: WWDT0_CLK_A) -> Self {
        variant as u8 != 0
    }
}
impl WWDT0_CLK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WWDT0_CLK_A {
        match self.bits {
            false => WWDT0_CLK_A::CLK_DISABLE,
            true => WWDT0_CLK_A::CLK_ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `CLK_DISABLE`"]
    #[inline(always)]
    pub fn is_clk_disable(&self) -> bool {
        *self == WWDT0_CLK_A::CLK_DISABLE
    }
    #[doc = "Checks if the value of the field is `CLK_ENABLE`"]
    #[inline(always)]
    pub fn is_clk_enable(&self) -> bool {
        *self == WWDT0_CLK_A::CLK_ENABLE
    }
}
#[doc = "Field `WWDT0_CLK` writer - Watchdog Timer 0 clock control"]
pub type WWDT0_CLK_W<'a, const O: u8> = crate::BitWriter<'a, u32, PSCCTL2_SPEC, WWDT0_CLK_A, O>;
impl<'a, const O: u8> WWDT0_CLK_W<'a, O> {
    #[doc = "Disable clock"]
    #[inline(always)]
    pub fn clk_disable(self) -> &'a mut W {
        self.variant(WWDT0_CLK_A::CLK_DISABLE)
    }
    #[doc = "Enable clock"]
    #[inline(always)]
    pub fn clk_enable(self) -> &'a mut W {
        self.variant(WWDT0_CLK_A::CLK_ENABLE)
    }
}
#[doc = "Field `PMC_CLK` reader - Power Management Controller clock control"]
pub type PMC_CLK_R = crate::BitReader<PMC_CLK_A>;
#[doc = "Power Management Controller clock control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PMC_CLK_A {
    #[doc = "0: Disable clock"]
    CLK_DISABLE = 0,
    #[doc = "1: Enable clock"]
    CLK_ENABLE = 1,
}
impl From<PMC_CLK_A> for bool {
    #[inline(always)]
    fn from(variant: PMC_CLK_A) -> Self {
        variant as u8 != 0
    }
}
impl PMC_CLK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PMC_CLK_A {
        match self.bits {
            false => PMC_CLK_A::CLK_DISABLE,
            true => PMC_CLK_A::CLK_ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `CLK_DISABLE`"]
    #[inline(always)]
    pub fn is_clk_disable(&self) -> bool {
        *self == PMC_CLK_A::CLK_DISABLE
    }
    #[doc = "Checks if the value of the field is `CLK_ENABLE`"]
    #[inline(always)]
    pub fn is_clk_enable(&self) -> bool {
        *self == PMC_CLK_A::CLK_ENABLE
    }
}
#[doc = "Field `PMC_CLK` writer - Power Management Controller clock control"]
pub type PMC_CLK_W<'a, const O: u8> = crate::BitWriter<'a, u32, PSCCTL2_SPEC, PMC_CLK_A, O>;
impl<'a, const O: u8> PMC_CLK_W<'a, O> {
    #[doc = "Disable clock"]
    #[inline(always)]
    pub fn clk_disable(self) -> &'a mut W {
        self.variant(PMC_CLK_A::CLK_DISABLE)
    }
    #[doc = "Enable clock"]
    #[inline(always)]
    pub fn clk_enable(self) -> &'a mut W {
        self.variant(PMC_CLK_A::CLK_ENABLE)
    }
}
impl R {
    #[doc = "Bit 0 - Micro-Tick Timer 0 clock control"]
    #[inline(always)]
    pub fn utick0_clk(&self) -> UTICK0_CLK_R {
        UTICK0_CLK_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Watchdog Timer 0 clock control"]
    #[inline(always)]
    pub fn wwdt0_clk(&self) -> WWDT0_CLK_R {
        WWDT0_CLK_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 29 - Power Management Controller clock control"]
    #[inline(always)]
    pub fn pmc_clk(&self) -> PMC_CLK_R {
        PMC_CLK_R::new(((self.bits >> 29) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Micro-Tick Timer 0 clock control"]
    #[inline(always)]
    #[must_use]
    pub fn utick0_clk(&mut self) -> UTICK0_CLK_W<0> {
        UTICK0_CLK_W::new(self)
    }
    #[doc = "Bit 1 - Watchdog Timer 0 clock control"]
    #[inline(always)]
    #[must_use]
    pub fn wwdt0_clk(&mut self) -> WWDT0_CLK_W<1> {
        WWDT0_CLK_W::new(self)
    }
    #[doc = "Bit 29 - Power Management Controller clock control"]
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
#[doc = "Clock Control 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pscctl2](index.html) module"]
pub struct PSCCTL2_SPEC;
impl crate::RegisterSpec for PSCCTL2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pscctl2::R](R) reader structure"]
impl crate::Readable for PSCCTL2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pscctl2::W](W) writer structure"]
impl crate::Writable for PSCCTL2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PSCCTL2 to value 0"]
impl crate::Resettable for PSCCTL2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
