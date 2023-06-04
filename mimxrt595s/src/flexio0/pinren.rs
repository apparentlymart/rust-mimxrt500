#[doc = "Register `PINREN` reader"]
pub struct R(crate::R<PINREN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PINREN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PINREN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PINREN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PINREN` writer"]
pub struct W(crate::W<PINREN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PINREN_SPEC>;
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
impl From<crate::W<PINREN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PINREN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PRE` reader - Pin Rising Edge"]
pub type PRE_R = crate::FieldReader<u16, u16>;
#[doc = "Field `PRE` writer - Pin Rising Edge"]
pub type PRE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PINREN_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15 - Pin Rising Edge"]
    #[inline(always)]
    pub fn pre(&self) -> PRE_R {
        PRE_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Pin Rising Edge"]
    #[inline(always)]
    #[must_use]
    pub fn pre(&mut self) -> PRE_W<0> {
        PRE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Pin Rising Edge Enable Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pinren](index.html) module"]
pub struct PINREN_SPEC;
impl crate::RegisterSpec for PINREN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pinren::R](R) reader structure"]
impl crate::Readable for PINREN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pinren::W](W) writer structure"]
impl crate::Writable for PINREN_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PINREN to value 0"]
impl crate::Resettable for PINREN_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
