#[doc = "Register `PRNG_OUT` writer"]
pub struct W(crate::W<PRNG_OUT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PRNG_OUT_SPEC>;
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
impl From<crate::W<PRNG_OUT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PRNG_OUT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PRNG_OUT_R` writer - SHA Digest word to reload."]
pub type PRNG_OUT_R_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, PRNG_OUT_SPEC, u32, u32, 32, O>;
impl W {
    #[doc = "Bits 0:31 - SHA Digest word to reload."]
    #[inline(always)]
    #[must_use]
    pub fn prng_out_r(&mut self) -> PRNG_OUT_R_W<0> {
        PRNG_OUT_R_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PRNG Output\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [prng_out](index.html) module"]
pub struct PRNG_OUT_SPEC;
impl crate::RegisterSpec for PRNG_OUT_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [prng_out::W](W) writer structure"]
impl crate::Writable for PRNG_OUT_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PRNG_OUT to value 0"]
impl crate::Resettable for PRNG_OUT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
