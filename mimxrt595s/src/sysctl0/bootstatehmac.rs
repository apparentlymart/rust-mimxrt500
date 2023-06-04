#[doc = "Register `BOOTSTATEHMAC[%s]` reader"]
pub struct R(crate::R<BOOTSTATEHMAC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BOOTSTATEHMAC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BOOTSTATEHMAC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BOOTSTATEHMAC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BOOTSTATEHMAC[%s]` writer"]
pub struct W(crate::W<BOOTSTATEHMAC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BOOTSTATEHMAC_SPEC>;
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
impl From<crate::W<BOOTSTATEHMAC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BOOTSTATEHMAC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BOOTSTATEHMAC` reader - BOOTSTATEHMAC\\[0:7\\]"]
pub type BOOTSTATEHMAC_R = crate::FieldReader<u32, u32>;
#[doc = "Field `BOOTSTATEHMAC` writer - BOOTSTATEHMAC\\[0:7\\]"]
pub type BOOTSTATEHMAC_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, BOOTSTATEHMAC_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - BOOTSTATEHMAC\\[0:7\\]"]
    #[inline(always)]
    pub fn bootstatehmac(&self) -> BOOTSTATEHMAC_R {
        BOOTSTATEHMAC_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - BOOTSTATEHMAC\\[0:7\\]"]
    #[inline(always)]
    #[must_use]
    pub fn bootstatehmac(&mut self) -> BOOTSTATEHMAC_W<0> {
        BOOTSTATEHMAC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "HMAC of boot state used for attestation.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bootstatehmac](index.html) module"]
pub struct BOOTSTATEHMAC_SPEC;
impl crate::RegisterSpec for BOOTSTATEHMAC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [bootstatehmac::R](R) reader structure"]
impl crate::Readable for BOOTSTATEHMAC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [bootstatehmac::W](W) writer structure"]
impl crate::Writable for BOOTSTATEHMAC_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets BOOTSTATEHMAC[%s]
to value 0"]
impl crate::Resettable for BOOTSTATEHMAC_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
