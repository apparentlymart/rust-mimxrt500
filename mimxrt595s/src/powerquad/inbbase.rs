#[doc = "Register `INBBASE` reader"]
pub struct R(crate::R<INBBASE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INBBASE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INBBASE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INBBASE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `INBBASE` writer"]
pub struct W(crate::W<INBBASE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INBBASE_SPEC>;
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
impl From<crate::W<INBBASE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INBBASE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `INBBASE` reader - Input B base"]
pub type INBBASE_R = crate::FieldReader<u32, u32>;
#[doc = "Field `INBBASE` writer - Input B base"]
pub type INBBASE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, INBBASE_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - Input B base"]
    #[inline(always)]
    pub fn inbbase(&self) -> INBBASE_R {
        INBBASE_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Input B base"]
    #[inline(always)]
    #[must_use]
    pub fn inbbase(&mut self) -> INBBASE_W<0> {
        INBBASE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Input B base\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [inbbase](index.html) module"]
pub struct INBBASE_SPEC;
impl crate::RegisterSpec for INBBASE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [inbbase::R](R) reader structure"]
impl crate::Readable for INBBASE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [inbbase::W](W) writer structure"]
impl crate::Writable for INBBASE_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets INBBASE to value 0"]
impl crate::Resettable for INBBASE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
