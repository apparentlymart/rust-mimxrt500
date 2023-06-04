#[doc = "Register `FN_MOD` reader"]
pub struct R(crate::R<FN_MOD_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FN_MOD_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FN_MOD_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FN_MOD_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FN_MOD` writer"]
pub struct W(crate::W<FN_MOD_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FN_MOD_SPEC>;
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
impl From<crate::W<FN_MOD_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FN_MOD_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FN_MOD` reader - Bypass Merge"]
pub type FN_MOD_R = crate::FieldReader<u8, u8>;
#[doc = "Field `FN_MOD` writer - Bypass Merge"]
pub type FN_MOD_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FN_MOD_SPEC, u8, u8, 2, O>;
impl R {
    #[doc = "Bits 0:1 - Bypass Merge"]
    #[inline(always)]
    pub fn fn_mod(&self) -> FN_MOD_R {
        FN_MOD_R::new((self.bits & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Bypass Merge"]
    #[inline(always)]
    #[must_use]
    pub fn fn_mod(&mut self) -> FN_MOD_W<0> {
        FN_MOD_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Issuing Functionality Modification\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fn_mod](index.html) module"]
pub struct FN_MOD_SPEC;
impl crate::RegisterSpec for FN_MOD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fn_mod::R](R) reader structure"]
impl crate::Readable for FN_MOD_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fn_mod::W](W) writer structure"]
impl crate::Writable for FN_MOD_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FN_MOD to value 0"]
impl crate::Resettable for FN_MOD_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
