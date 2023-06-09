#[doc = "Register `MSTDAT` reader"]
pub struct R(crate::R<MSTDAT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MSTDAT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MSTDAT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MSTDAT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MSTDAT` writer"]
pub struct W(crate::W<MSTDAT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MSTDAT_SPEC>;
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
impl From<crate::W<MSTDAT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MSTDAT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DATA` reader - Master function data register"]
pub type DATA_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DATA` writer - Master function data register"]
pub type DATA_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MSTDAT_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - Master function data register"]
    #[inline(always)]
    pub fn data(&self) -> DATA_R {
        DATA_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Master function data register"]
    #[inline(always)]
    #[must_use]
    pub fn data(&mut self) -> DATA_W<0> {
        DATA_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Master Data Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mstdat](index.html) module"]
pub struct MSTDAT_SPEC;
impl crate::RegisterSpec for MSTDAT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mstdat::R](R) reader structure"]
impl crate::Readable for MSTDAT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mstdat::W](W) writer structure"]
impl crate::Writable for MSTDAT_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MSTDAT to value 0"]
impl crate::Resettable for MSTDAT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
