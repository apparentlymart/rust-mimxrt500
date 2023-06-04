#[doc = "Register `PINOUTTOG` reader"]
pub struct R(crate::R<PINOUTTOG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PINOUTTOG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PINOUTTOG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PINOUTTOG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PINOUTTOG` writer"]
pub struct W(crate::W<PINOUTTOG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PINOUTTOG_SPEC>;
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
impl From<crate::W<PINOUTTOG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PINOUTTOG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `OUTTOG` reader - Output Toggle"]
pub type OUTTOG_R = crate::FieldReader<u16, u16>;
#[doc = "Field `OUTTOG` writer - Output Toggle"]
pub type OUTTOG_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PINOUTTOG_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15 - Output Toggle"]
    #[inline(always)]
    pub fn outtog(&self) -> OUTTOG_R {
        OUTTOG_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Output Toggle"]
    #[inline(always)]
    #[must_use]
    pub fn outtog(&mut self) -> OUTTOG_W<0> {
        OUTTOG_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Pin Output Toggle Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pinouttog](index.html) module"]
pub struct PINOUTTOG_SPEC;
impl crate::RegisterSpec for PINOUTTOG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pinouttog::R](R) reader structure"]
impl crate::Readable for PINOUTTOG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pinouttog::W](W) writer structure"]
impl crate::Writable for PINOUTTOG_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PINOUTTOG to value 0"]
impl crate::Resettable for PINOUTTOG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
