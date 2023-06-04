#[doc = "Register `TRGSTAT` reader"]
pub struct R(crate::R<TRGSTAT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TRGSTAT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TRGSTAT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TRGSTAT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TRGSTAT` writer"]
pub struct W(crate::W<TRGSTAT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TRGSTAT_SPEC>;
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
impl From<crate::W<TRGSTAT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TRGSTAT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ETSF` reader - External Trigger Status Flags"]
pub type ETSF_R = crate::FieldReader<u16, u16>;
#[doc = "Field `ETSF` writer - External Trigger Status Flags"]
pub type ETSF_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TRGSTAT_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15 - External Trigger Status Flags"]
    #[inline(always)]
    pub fn etsf(&self) -> ETSF_R {
        ETSF_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - External Trigger Status Flags"]
    #[inline(always)]
    #[must_use]
    pub fn etsf(&mut self) -> ETSF_W<0> {
        ETSF_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Trigger Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [trgstat](index.html) module"]
pub struct TRGSTAT_SPEC;
impl crate::RegisterSpec for TRGSTAT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [trgstat::R](R) reader structure"]
impl crate::Readable for TRGSTAT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [trgstat::W](W) writer structure"]
impl crate::Writable for TRGSTAT_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0xffff;
}
#[doc = "`reset()` method sets TRGSTAT to value 0"]
impl crate::Resettable for TRGSTAT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
