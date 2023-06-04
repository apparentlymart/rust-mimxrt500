#[doc = "Register `PINOUTE` reader"]
pub struct R(crate::R<PINOUTE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PINOUTE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PINOUTE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PINOUTE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PINOUTE` writer"]
pub struct W(crate::W<PINOUTE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PINOUTE_SPEC>;
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
impl From<crate::W<PINOUTE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PINOUTE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `OUTE` reader - Output Enable"]
pub type OUTE_R = crate::FieldReader<u16, u16>;
#[doc = "Field `OUTE` writer - Output Enable"]
pub type OUTE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PINOUTE_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15 - Output Enable"]
    #[inline(always)]
    pub fn oute(&self) -> OUTE_R {
        OUTE_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Output Enable"]
    #[inline(always)]
    #[must_use]
    pub fn oute(&mut self) -> OUTE_W<0> {
        OUTE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Pin Output Enable Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pinoute](index.html) module"]
pub struct PINOUTE_SPEC;
impl crate::RegisterSpec for PINOUTE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pinoute::R](R) reader structure"]
impl crate::Readable for PINOUTE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pinoute::W](W) writer structure"]
impl crate::Writable for PINOUTE_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PINOUTE to value 0"]
impl crate::Resettable for PINOUTE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
