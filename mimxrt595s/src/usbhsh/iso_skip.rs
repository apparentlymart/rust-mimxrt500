#[doc = "Register `ISO_SKIP` reader"]
pub struct R(crate::R<ISO_SKIP_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ISO_SKIP_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ISO_SKIP_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ISO_SKIP_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ISO_SKIP` writer"]
pub struct W(crate::W<ISO_SKIP_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ISO_SKIP_SPEC>;
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
impl From<crate::W<ISO_SKIP_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ISO_SKIP_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ISO_SKIP` reader - ISO Skip"]
pub type ISO_SKIP_R = crate::FieldReader<u32, u32>;
#[doc = "Field `ISO_SKIP` writer - ISO Skip"]
pub type ISO_SKIP_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ISO_SKIP_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - ISO Skip"]
    #[inline(always)]
    pub fn iso_skip(&self) -> ISO_SKIP_R {
        ISO_SKIP_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - ISO Skip"]
    #[inline(always)]
    #[must_use]
    pub fn iso_skip(&mut self) -> ISO_SKIP_W<0> {
        ISO_SKIP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ISO PTD Skip Map\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [iso_skip](index.html) module"]
pub struct ISO_SKIP_SPEC;
impl crate::RegisterSpec for ISO_SKIP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [iso_skip::R](R) reader structure"]
impl crate::Readable for ISO_SKIP_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [iso_skip::W](W) writer structure"]
impl crate::Writable for ISO_SKIP_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ISO_SKIP to value 0"]
impl crate::Resettable for ISO_SKIP_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
