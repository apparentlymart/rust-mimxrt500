#[doc = "Register `INABASE` reader"]
pub struct R(crate::R<INABASE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INABASE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INABASE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INABASE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `INABASE` writer"]
pub struct W(crate::W<INABASE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INABASE_SPEC>;
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
impl From<crate::W<INABASE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INABASE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `INABASE` reader - Input A base"]
pub type INABASE_R = crate::FieldReader<u32, u32>;
#[doc = "Field `INABASE` writer - Input A base"]
pub type INABASE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, INABASE_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - Input A base"]
    #[inline(always)]
    pub fn inabase(&self) -> INABASE_R {
        INABASE_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Input A base"]
    #[inline(always)]
    #[must_use]
    pub fn inabase(&mut self) -> INABASE_W<0> {
        INABASE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Input A base\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [inabase](index.html) module"]
pub struct INABASE_SPEC;
impl crate::RegisterSpec for INABASE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [inabase::R](R) reader structure"]
impl crate::Readable for INABASE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [inabase::W](W) writer structure"]
impl crate::Writable for INABASE_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets INABASE to value 0"]
impl crate::Resettable for INABASE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
