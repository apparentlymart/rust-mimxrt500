#[doc = "Register `LPOSCCTL0` reader"]
pub struct R(crate::R<LPOSCCTL0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LPOSCCTL0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LPOSCCTL0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LPOSCCTL0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `LPOSCCTL0` writer"]
pub struct W(crate::W<LPOSCCTL0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LPOSCCTL0_SPEC>;
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
impl From<crate::W<LPOSCCTL0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LPOSCCTL0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CLKRDY` reader - LPOSC Clock Ready"]
pub type CLKRDY_R = crate::BitReader<CLKRDY_A>;
#[doc = "LPOSC Clock Ready\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CLKRDY_A {
    #[doc = "0: LPOSC clock is not ready"]
    CLK_NOT_READY = 0,
    #[doc = "1: LPOSC clock is ready"]
    CLK_READY = 1,
}
impl From<CLKRDY_A> for bool {
    #[inline(always)]
    fn from(variant: CLKRDY_A) -> Self {
        variant as u8 != 0
    }
}
impl CLKRDY_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CLKRDY_A {
        match self.bits {
            false => CLKRDY_A::CLK_NOT_READY,
            true => CLKRDY_A::CLK_READY,
        }
    }
    #[doc = "Checks if the value of the field is `CLK_NOT_READY`"]
    #[inline(always)]
    pub fn is_clk_not_ready(&self) -> bool {
        *self == CLKRDY_A::CLK_NOT_READY
    }
    #[doc = "Checks if the value of the field is `CLK_READY`"]
    #[inline(always)]
    pub fn is_clk_ready(&self) -> bool {
        *self == CLKRDY_A::CLK_READY
    }
}
#[doc = "Field `CLKRDY` writer - LPOSC Clock Ready"]
pub type CLKRDY_W<'a, const O: u8> = crate::BitWriter<'a, u32, LPOSCCTL0_SPEC, CLKRDY_A, O>;
impl<'a, const O: u8> CLKRDY_W<'a, O> {
    #[doc = "LPOSC clock is not ready"]
    #[inline(always)]
    pub fn clk_not_ready(self) -> &'a mut W {
        self.variant(CLKRDY_A::CLK_NOT_READY)
    }
    #[doc = "LPOSC clock is ready"]
    #[inline(always)]
    pub fn clk_ready(self) -> &'a mut W {
        self.variant(CLKRDY_A::CLK_READY)
    }
}
impl R {
    #[doc = "Bit 31 - LPOSC Clock Ready"]
    #[inline(always)]
    pub fn clkrdy(&self) -> CLKRDY_R {
        CLKRDY_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 31 - LPOSC Clock Ready"]
    #[inline(always)]
    #[must_use]
    pub fn clkrdy(&mut self) -> CLKRDY_W<31> {
        CLKRDY_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Low Power Oscillator Control 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lposcctl0](index.html) module"]
pub struct LPOSCCTL0_SPEC;
impl crate::RegisterSpec for LPOSCCTL0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [lposcctl0::R](R) reader structure"]
impl crate::Readable for LPOSCCTL0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [lposcctl0::W](W) writer structure"]
impl crate::Writable for LPOSCCTL0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets LPOSCCTL0 to value 0x807b_c4d4"]
impl crate::Resettable for LPOSCCTL0_SPEC {
    const RESET_VALUE: Self::Ux = 0x807b_c4d4;
}
