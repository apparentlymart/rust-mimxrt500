#[doc = "Register `OUT_CLR` reader"]
pub struct R(crate::R<OUT_CLR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OUT_CLR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OUT_CLR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OUT_CLR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `OUT_CLR` writer"]
pub struct W(crate::W<OUT_CLR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OUT_CLR_SPEC>;
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
impl From<crate::W<OUT_CLR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<OUT_CLR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CLR` reader - Clear"]
pub type CLR_R = crate::FieldReader<u16, u16>;
#[doc = "Field `CLR` writer - Clear"]
pub type CLR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, OUT_CLR_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15 - Clear"]
    #[inline(always)]
    pub fn clr(&self) -> CLR_R {
        CLR_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Clear"]
    #[inline(always)]
    #[must_use]
    pub fn clr(&mut self) -> CLR_W<0> {
        CLR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Output n Clear\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [out_clr](index.html) module"]
pub struct OUT_CLR_SPEC;
impl crate::RegisterSpec for OUT_CLR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [out_clr::R](R) reader structure"]
impl crate::Readable for OUT_CLR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [out_clr::W](W) writer structure"]
impl crate::Writable for OUT_CLR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets OUT_CLR to value 0"]
impl crate::Resettable for OUT_CLR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
