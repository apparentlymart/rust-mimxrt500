#[doc = "Register `RELOAD[%s]` reader"]
pub struct R(crate::R<RELOAD_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RELOAD_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RELOAD_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RELOAD_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RELOAD[%s]` writer"]
pub struct W(crate::W<RELOAD_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RELOAD_SPEC>;
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
impl From<crate::W<RELOAD_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RELOAD_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DIGEST` reader - SHA Digest word to reload."]
pub type DIGEST_R = crate::FieldReader<u32, u32>;
#[doc = "Field `DIGEST` writer - SHA Digest word to reload."]
pub type DIGEST_W<'a, const O: u8> = crate::FieldWriter<'a, u32, RELOAD_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - SHA Digest word to reload."]
    #[inline(always)]
    pub fn digest(&self) -> DIGEST_R {
        DIGEST_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - SHA Digest word to reload."]
    #[inline(always)]
    #[must_use]
    pub fn digest(&mut self) -> DIGEST_W<0> {
        DIGEST_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DIGEST/OUTDATA Reload\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [reload](index.html) module"]
pub struct RELOAD_SPEC;
impl crate::RegisterSpec for RELOAD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [reload::R](R) reader structure"]
impl crate::Readable for RELOAD_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [reload::W](W) writer structure"]
impl crate::Writable for RELOAD_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RELOAD[%s]
to value 0"]
impl crate::Resettable for RELOAD_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
