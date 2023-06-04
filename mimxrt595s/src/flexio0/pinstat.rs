#[doc = "Register `PINSTAT` reader"]
pub struct R(crate::R<PINSTAT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PINSTAT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PINSTAT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PINSTAT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PINSTAT` writer"]
pub struct W(crate::W<PINSTAT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PINSTAT_SPEC>;
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
impl From<crate::W<PINSTAT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PINSTAT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PSF` reader - Pin Status Flags"]
pub type PSF_R = crate::FieldReader<u16, u16>;
#[doc = "Field `PSF` writer - Pin Status Flags"]
pub type PSF_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PINSTAT_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15 - Pin Status Flags"]
    #[inline(always)]
    pub fn psf(&self) -> PSF_R {
        PSF_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Pin Status Flags"]
    #[inline(always)]
    #[must_use]
    pub fn psf(&mut self) -> PSF_W<0> {
        PSF_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Pin Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pinstat](index.html) module"]
pub struct PINSTAT_SPEC;
impl crate::RegisterSpec for PINSTAT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pinstat::R](R) reader structure"]
impl crate::Readable for PINSTAT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pinstat::W](W) writer structure"]
impl crate::Writable for PINSTAT_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0xffff;
}
#[doc = "`reset()` method sets PINSTAT to value 0"]
impl crate::Resettable for PINSTAT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
