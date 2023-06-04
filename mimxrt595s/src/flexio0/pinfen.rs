#[doc = "Register `PINFEN` reader"]
pub struct R(crate::R<PINFEN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PINFEN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PINFEN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PINFEN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PINFEN` writer"]
pub struct W(crate::W<PINFEN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PINFEN_SPEC>;
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
impl From<crate::W<PINFEN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PINFEN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PFE` reader - Pin Falling Edge"]
pub type PFE_R = crate::FieldReader<u16, u16>;
#[doc = "Field `PFE` writer - Pin Falling Edge"]
pub type PFE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PINFEN_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15 - Pin Falling Edge"]
    #[inline(always)]
    pub fn pfe(&self) -> PFE_R {
        PFE_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Pin Falling Edge"]
    #[inline(always)]
    #[must_use]
    pub fn pfe(&mut self) -> PFE_W<0> {
        PFE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Pin Falling Edge Enable Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pinfen](index.html) module"]
pub struct PINFEN_SPEC;
impl crate::RegisterSpec for PINFEN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pinfen::R](R) reader structure"]
impl crate::Readable for PINFEN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pinfen::W](W) writer structure"]
impl crate::Writable for PINFEN_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PINFEN to value 0"]
impl crate::Resettable for PINFEN_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
