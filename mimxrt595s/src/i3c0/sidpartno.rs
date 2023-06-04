#[doc = "Register `SIDPARTNO` reader"]
pub struct R(crate::R<SIDPARTNO_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SIDPARTNO_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SIDPARTNO_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SIDPARTNO_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SIDPARTNO` writer"]
pub struct W(crate::W<SIDPARTNO_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SIDPARTNO_SPEC>;
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
impl From<crate::W<SIDPARTNO_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SIDPARTNO_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PARTNO` reader - Part number"]
pub type PARTNO_R = crate::FieldReader<u32, u32>;
#[doc = "Field `PARTNO` writer - Part number"]
pub type PARTNO_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SIDPARTNO_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - Part number"]
    #[inline(always)]
    pub fn partno(&self) -> PARTNO_R {
        PARTNO_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Part number"]
    #[inline(always)]
    #[must_use]
    pub fn partno(&mut self) -> PARTNO_W<0> {
        PARTNO_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Slave ID Part Number Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sidpartno](index.html) module"]
pub struct SIDPARTNO_SPEC;
impl crate::RegisterSpec for SIDPARTNO_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sidpartno::R](R) reader structure"]
impl crate::Readable for SIDPARTNO_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sidpartno::W](W) writer structure"]
impl crate::Writable for SIDPARTNO_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SIDPARTNO to value 0"]
impl crate::Resettable for SIDPARTNO_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
