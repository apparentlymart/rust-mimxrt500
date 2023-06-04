#[doc = "Register `MEMADDR` reader"]
pub struct R(crate::R<MEMADDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MEMADDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MEMADDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MEMADDR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MEMADDR` writer"]
pub struct W(crate::W<MEMADDR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MEMADDR_SPEC>;
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
impl From<crate::W<MEMADDR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MEMADDR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BASE` reader - Base"]
pub type BASE_R = crate::FieldReader<u32, u32>;
#[doc = "Field `BASE` writer - Base"]
pub type BASE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MEMADDR_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - Base"]
    #[inline(always)]
    pub fn base(&self) -> BASE_R {
        BASE_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Base"]
    #[inline(always)]
    #[must_use]
    pub fn base(&mut self) -> BASE_W<0> {
        BASE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Memory Address\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [memaddr](index.html) module"]
pub struct MEMADDR_SPEC;
impl crate::RegisterSpec for MEMADDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [memaddr::R](R) reader structure"]
impl crate::Readable for MEMADDR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [memaddr::W](W) writer structure"]
impl crate::Writable for MEMADDR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MEMADDR to value 0"]
impl crate::Resettable for MEMADDR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
