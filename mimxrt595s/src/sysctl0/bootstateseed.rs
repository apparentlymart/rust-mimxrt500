#[doc = "Register `BOOTSTATESEED[%s]` reader"]
pub struct R(crate::R<BOOTSTATESEED_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BOOTSTATESEED_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BOOTSTATESEED_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BOOTSTATESEED_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BOOTSTATESEED[%s]` writer"]
pub struct W(crate::W<BOOTSTATESEED_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BOOTSTATESEED_SPEC>;
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
impl From<crate::W<BOOTSTATESEED_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BOOTSTATESEED_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BOOTSTATESEED` reader - BOOTSTATESEED\\[0:7\\]"]
pub type BOOTSTATESEED_R = crate::FieldReader<u32, u32>;
#[doc = "Field `BOOTSTATESEED` writer - BOOTSTATESEED\\[0:7\\]"]
pub type BOOTSTATESEED_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, BOOTSTATESEED_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - BOOTSTATESEED\\[0:7\\]"]
    #[inline(always)]
    pub fn bootstateseed(&self) -> BOOTSTATESEED_R {
        BOOTSTATESEED_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - BOOTSTATESEED\\[0:7\\]"]
    #[inline(always)]
    #[must_use]
    pub fn bootstateseed(&mut self) -> BOOTSTATESEED_W<0> {
        BOOTSTATESEED_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Boot State Seed\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bootstateseed](index.html) module"]
pub struct BOOTSTATESEED_SPEC;
impl crate::RegisterSpec for BOOTSTATESEED_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [bootstateseed::R](R) reader structure"]
impl crate::Readable for BOOTSTATESEED_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [bootstateseed::W](W) writer structure"]
impl crate::Writable for BOOTSTATESEED_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets BOOTSTATESEED[%s]
to value 0"]
impl crate::Resettable for BOOTSTATESEED_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
