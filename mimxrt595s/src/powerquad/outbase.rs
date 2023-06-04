#[doc = "Register `OUTBASE` reader"]
pub struct R(crate::R<OUTBASE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OUTBASE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OUTBASE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OUTBASE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `OUTBASE` writer"]
pub struct W(crate::W<OUTBASE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OUTBASE_SPEC>;
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
impl From<crate::W<OUTBASE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<OUTBASE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `OUTBASE` reader - Base address for the output region"]
pub type OUTBASE_R = crate::FieldReader<u32, u32>;
#[doc = "Field `OUTBASE` writer - Base address for the output region"]
pub type OUTBASE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, OUTBASE_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - Base address for the output region"]
    #[inline(always)]
    pub fn outbase(&self) -> OUTBASE_R {
        OUTBASE_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Base address for the output region"]
    #[inline(always)]
    #[must_use]
    pub fn outbase(&mut self) -> OUTBASE_W<0> {
        OUTBASE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Output Base\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [outbase](index.html) module"]
pub struct OUTBASE_SPEC;
impl crate::RegisterSpec for OUTBASE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [outbase::R](R) reader structure"]
impl crate::Readable for OUTBASE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [outbase::W](W) writer structure"]
impl crate::Writable for OUTBASE_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets OUTBASE to value 0"]
impl crate::Resettable for OUTBASE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
