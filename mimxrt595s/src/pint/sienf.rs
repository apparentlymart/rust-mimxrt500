#[doc = "Register `SIENF` writer"]
pub struct W(crate::W<SIENF_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SIENF_SPEC>;
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
impl From<crate::W<SIENF_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SIENF_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Set bits in the IENF\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SETENAF_AW {
    #[doc = "0: No operation"]
    SETENAF_0 = 0,
    #[doc = "1: Select HIGH-active interrupt or enable falling edge interrupt"]
    SETENAF_1 = 1,
}
impl From<SETENAF_AW> for u8 {
    #[inline(always)]
    fn from(variant: SETENAF_AW) -> Self {
        variant as _
    }
}
#[doc = "Field `SETENAF` writer - Set bits in the IENF"]
pub type SETENAF_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SIENF_SPEC, u8, SETENAF_AW, 8, O>;
impl<'a, const O: u8> SETENAF_W<'a, O> {
    #[doc = "No operation"]
    #[inline(always)]
    pub fn setenaf_0(self) -> &'a mut W {
        self.variant(SETENAF_AW::SETENAF_0)
    }
    #[doc = "Select HIGH-active interrupt or enable falling edge interrupt"]
    #[inline(always)]
    pub fn setenaf_1(self) -> &'a mut W {
        self.variant(SETENAF_AW::SETENAF_1)
    }
}
impl W {
    #[doc = "Bits 0:7 - Set bits in the IENF"]
    #[inline(always)]
    #[must_use]
    pub fn setenaf(&mut self) -> SETENAF_W<0> {
        SETENAF_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Pin Interrupt Active Level or Falling Edge Interrupt Set\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sienf](index.html) module"]
pub struct SIENF_SPEC;
impl crate::RegisterSpec for SIENF_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [sienf::W](W) writer structure"]
impl crate::Writable for SIENF_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0xff;
}
#[doc = "`reset()` method sets SIENF to value 0"]
impl crate::Resettable for SIENF_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
