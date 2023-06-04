#[doc = "Register `ISO_DONE` reader"]
pub struct R(crate::R<ISO_DONE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ISO_DONE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ISO_DONE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ISO_DONE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ISO_DONE` writer"]
pub struct W(crate::W<ISO_DONE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ISO_DONE_SPEC>;
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
impl From<crate::W<ISO_DONE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ISO_DONE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ISO_DONE` reader - ISO Done"]
pub type ISO_DONE_R = crate::FieldReader<u32, u32>;
#[doc = "Field `ISO_DONE` writer - ISO Done"]
pub type ISO_DONE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ISO_DONE_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - ISO Done"]
    #[inline(always)]
    pub fn iso_done(&self) -> ISO_DONE_R {
        ISO_DONE_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - ISO Done"]
    #[inline(always)]
    #[must_use]
    pub fn iso_done(&mut self) -> ISO_DONE_W<0> {
        ISO_DONE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ISO PTD Done Map\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [iso_done](index.html) module"]
pub struct ISO_DONE_SPEC;
impl crate::RegisterSpec for ISO_DONE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [iso_done::R](R) reader structure"]
impl crate::Readable for ISO_DONE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [iso_done::W](W) writer structure"]
impl crate::Writable for ISO_DONE_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0xffff_ffff;
}
#[doc = "`reset()` method sets ISO_DONE to value 0"]
impl crate::Resettable for ISO_DONE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
