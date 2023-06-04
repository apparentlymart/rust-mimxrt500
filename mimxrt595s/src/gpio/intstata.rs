#[doc = "Register `INTSTATA[%s]` reader"]
pub struct R(crate::R<INTSTATA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INTSTATA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INTSTATA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INTSTATA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `INTSTATA[%s]` writer"]
pub struct W(crate::W<INTSTATA_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INTSTATA_SPEC>;
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
impl From<crate::W<INTSTATA_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INTSTATA_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `STATUS` reader - Interrupt status."]
pub type STATUS_R = crate::FieldReader<u32, u32>;
#[doc = "Field `STATUS` writer - Interrupt status."]
pub type STATUS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, INTSTATA_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - Interrupt status."]
    #[inline(always)]
    pub fn status(&self) -> STATUS_R {
        STATUS_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Interrupt status."]
    #[inline(always)]
    #[must_use]
    pub fn status(&mut self) -> STATUS_W<0> {
        STATUS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt status for interrupt A\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intstata](index.html) module"]
pub struct INTSTATA_SPEC;
impl crate::RegisterSpec for INTSTATA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [intstata::R](R) reader structure"]
impl crate::Readable for INTSTATA_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [intstata::W](W) writer structure"]
impl crate::Writable for INTSTATA_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0xffff_ffff;
}
#[doc = "`reset()` method sets INTSTATA[%s]
to value 0"]
impl crate::Resettable for INTSTATA_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
