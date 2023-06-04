#[doc = "Register `COMPREG[%s]` reader"]
pub struct R(crate::R<COMPREG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<COMPREG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<COMPREG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<COMPREG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `COMPREG[%s]` writer"]
pub struct W(crate::W<COMPREG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<COMPREG_SPEC>;
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
impl From<crate::W<COMPREG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<COMPREG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `COMPREG` reader - Compute bank"]
pub type COMPREG_R = crate::FieldReader<u32, u32>;
#[doc = "Field `COMPREG` writer - Compute bank"]
pub type COMPREG_W<'a, const O: u8> = crate::FieldWriter<'a, u32, COMPREG_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - Compute bank"]
    #[inline(always)]
    pub fn compreg(&self) -> COMPREG_R {
        COMPREG_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Compute bank"]
    #[inline(always)]
    #[must_use]
    pub fn compreg(&mut self) -> COMPREG_W<0> {
        COMPREG_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Compute Register Bank n\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [compreg](index.html) module"]
pub struct COMPREG_SPEC;
impl crate::RegisterSpec for COMPREG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [compreg::R](R) reader structure"]
impl crate::Readable for COMPREG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [compreg::W](W) writer structure"]
impl crate::Writable for COMPREG_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets COMPREG[%s]
to value 0"]
impl crate::Resettable for COMPREG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
