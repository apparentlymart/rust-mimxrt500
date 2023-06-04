#[doc = "Register `PINOUTCLR` reader"]
pub struct R(crate::R<PINOUTCLR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PINOUTCLR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PINOUTCLR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PINOUTCLR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PINOUTCLR` writer"]
pub struct W(crate::W<PINOUTCLR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PINOUTCLR_SPEC>;
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
impl From<crate::W<PINOUTCLR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PINOUTCLR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `OUTCLR` reader - Output Clear"]
pub type OUTCLR_R = crate::FieldReader<u16, u16>;
#[doc = "Field `OUTCLR` writer - Output Clear"]
pub type OUTCLR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PINOUTCLR_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15 - Output Clear"]
    #[inline(always)]
    pub fn outclr(&self) -> OUTCLR_R {
        OUTCLR_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Output Clear"]
    #[inline(always)]
    #[must_use]
    pub fn outclr(&mut self) -> OUTCLR_W<0> {
        OUTCLR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Pin Output Clear Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pinoutclr](index.html) module"]
pub struct PINOUTCLR_SPEC;
impl crate::RegisterSpec for PINOUTCLR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pinoutclr::R](R) reader structure"]
impl crate::Readable for PINOUTCLR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pinoutclr::W](W) writer structure"]
impl crate::Writable for PINOUTCLR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PINOUTCLR to value 0"]
impl crate::Resettable for PINOUTCLR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
