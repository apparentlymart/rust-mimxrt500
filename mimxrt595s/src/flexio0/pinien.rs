#[doc = "Register `PINIEN` reader"]
pub struct R(crate::R<PINIEN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PINIEN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PINIEN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PINIEN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PINIEN` writer"]
pub struct W(crate::W<PINIEN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PINIEN_SPEC>;
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
impl From<crate::W<PINIEN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PINIEN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PSIE` reader - Pin Status Interrupt Enable"]
pub type PSIE_R = crate::FieldReader<u16, u16>;
#[doc = "Field `PSIE` writer - Pin Status Interrupt Enable"]
pub type PSIE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PINIEN_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15 - Pin Status Interrupt Enable"]
    #[inline(always)]
    pub fn psie(&self) -> PSIE_R {
        PSIE_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Pin Status Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn psie(&mut self) -> PSIE_W<0> {
        PSIE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Pin Interrupt Enable Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pinien](index.html) module"]
pub struct PINIEN_SPEC;
impl crate::RegisterSpec for PINIEN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pinien::R](R) reader structure"]
impl crate::Readable for PINIEN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pinien::W](W) writer structure"]
impl crate::Writable for PINIEN_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PINIEN to value 0"]
impl crate::Resettable for PINIEN_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
