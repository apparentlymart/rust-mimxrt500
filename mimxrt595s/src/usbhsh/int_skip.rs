#[doc = "Register `INT_SKIP` reader"]
pub struct R(crate::R<INT_SKIP_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INT_SKIP_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INT_SKIP_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INT_SKIP_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `INT_SKIP` writer"]
pub struct W(crate::W<INT_SKIP_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INT_SKIP_SPEC>;
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
impl From<crate::W<INT_SKIP_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INT_SKIP_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `INT_SKIP` reader - INT Skip"]
pub type INT_SKIP_R = crate::FieldReader<u32, u32>;
#[doc = "Field `INT_SKIP` writer - INT Skip"]
pub type INT_SKIP_W<'a, const O: u8> = crate::FieldWriter<'a, u32, INT_SKIP_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - INT Skip"]
    #[inline(always)]
    pub fn int_skip(&self) -> INT_SKIP_R {
        INT_SKIP_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - INT Skip"]
    #[inline(always)]
    #[must_use]
    pub fn int_skip(&mut self) -> INT_SKIP_W<0> {
        INT_SKIP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "INT PTD Skip Map\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [int_skip](index.html) module"]
pub struct INT_SKIP_SPEC;
impl crate::RegisterSpec for INT_SKIP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [int_skip::R](R) reader structure"]
impl crate::Readable for INT_SKIP_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [int_skip::W](W) writer structure"]
impl crate::Writable for INT_SKIP_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets INT_SKIP to value 0"]
impl crate::Resettable for INT_SKIP_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
