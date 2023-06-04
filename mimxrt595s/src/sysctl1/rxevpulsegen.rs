#[doc = "Register `RXEVPULSEGEN` writer"]
pub struct W(crate::W<RXEVPULSEGEN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RXEVPULSEGEN_SPEC>;
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
impl From<crate::W<RXEVPULSEGEN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RXEVPULSEGEN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "RX Event Pulse Generator\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RXEVPULSEGEN_AW {
    #[doc = "0: No effect"]
    RXEVPULSEGEN_0 = 0,
    #[doc = "1: Pulse RXEV High for one PSCLK cycle"]
    RXEVPULSEGEN_1 = 1,
}
impl From<RXEVPULSEGEN_AW> for bool {
    #[inline(always)]
    fn from(variant: RXEVPULSEGEN_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RXEVPULSEGEN` writer - RX Event Pulse Generator"]
pub type RXEVPULSEGEN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, RXEVPULSEGEN_SPEC, RXEVPULSEGEN_AW, O>;
impl<'a, const O: u8> RXEVPULSEGEN_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn rxevpulsegen_0(self) -> &'a mut W {
        self.variant(RXEVPULSEGEN_AW::RXEVPULSEGEN_0)
    }
    #[doc = "Pulse RXEV High for one PSCLK cycle"]
    #[inline(always)]
    pub fn rxevpulsegen_1(self) -> &'a mut W {
        self.variant(RXEVPULSEGEN_AW::RXEVPULSEGEN_1)
    }
}
impl W {
    #[doc = "Bit 0 - RX Event Pulse Generator"]
    #[inline(always)]
    #[must_use]
    pub fn rxevpulsegen(&mut self) -> RXEVPULSEGEN_W<0> {
        RXEVPULSEGEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RX Event Pulse Generator\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rxevpulsegen](index.html) module"]
pub struct RXEVPULSEGEN_SPEC;
impl crate::RegisterSpec for RXEVPULSEGEN_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [rxevpulsegen::W](W) writer structure"]
impl crate::Writable for RXEVPULSEGEN_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RXEVPULSEGEN to value 0"]
impl crate::Resettable for RXEVPULSEGEN_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
