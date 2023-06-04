#[doc = "Register `CTX_CTR%s` reader"]
pub struct R(crate::R<CTX_CTR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CTX_CTR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CTX_CTR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CTX_CTR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CTX_CTR%s` writer"]
pub struct W(crate::W<CTX_CTR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CTX_CTR_SPEC>;
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
impl From<crate::W<CTX_CTR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CTX_CTR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CTR` reader - AES Counter"]
pub type CTR_R = crate::FieldReader<u32, u32>;
#[doc = "Field `CTR` writer - AES Counter"]
pub type CTR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CTX_CTR_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - AES Counter"]
    #[inline(always)]
    pub fn ctr(&self) -> CTR_R {
        CTR_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - AES Counter"]
    #[inline(always)]
    #[must_use]
    pub fn ctr(&mut self) -> CTR_W<0> {
        CTR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "AES Counter Word\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctx_ctr](index.html) module"]
pub struct CTX_CTR_SPEC;
impl crate::RegisterSpec for CTX_CTR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ctx_ctr::R](R) reader structure"]
impl crate::Readable for CTX_CTR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ctx_ctr::W](W) writer structure"]
impl crate::Writable for CTX_CTR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CTX_CTR%s to value 0"]
impl crate::Resettable for CTX_CTR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
