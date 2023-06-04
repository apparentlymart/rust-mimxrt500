#[doc = "Register `CIENF` writer"]
pub struct W(crate::W<CIENF_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CIENF_SPEC>;
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
impl From<crate::W<CIENF_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CIENF_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Clear bits in the IENF\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CENAF_AW {
    #[doc = "0: No operation"]
    CENAF_0 = 0,
    #[doc = "1: LOW-active interrupt selected or falling edge interrupt disabled"]
    CENAF_1 = 1,
}
impl From<CENAF_AW> for u8 {
    #[inline(always)]
    fn from(variant: CENAF_AW) -> Self {
        variant as _
    }
}
#[doc = "Field `CENAF` writer - Clear bits in the IENF"]
pub type CENAF_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CIENF_SPEC, u8, CENAF_AW, 8, O>;
impl<'a, const O: u8> CENAF_W<'a, O> {
    #[doc = "No operation"]
    #[inline(always)]
    pub fn cenaf_0(self) -> &'a mut W {
        self.variant(CENAF_AW::CENAF_0)
    }
    #[doc = "LOW-active interrupt selected or falling edge interrupt disabled"]
    #[inline(always)]
    pub fn cenaf_1(self) -> &'a mut W {
        self.variant(CENAF_AW::CENAF_1)
    }
}
impl W {
    #[doc = "Bits 0:7 - Clear bits in the IENF"]
    #[inline(always)]
    #[must_use]
    pub fn cenaf(&mut self) -> CENAF_W<0> {
        CENAF_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Pin Interrupt Active Level or Falling Edge Interrupt Clear\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cienf](index.html) module"]
pub struct CIENF_SPEC;
impl crate::RegisterSpec for CIENF_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [cienf::W](W) writer structure"]
impl crate::Writable for CIENF_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CIENF to value 0"]
impl crate::Resettable for CIENF_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
