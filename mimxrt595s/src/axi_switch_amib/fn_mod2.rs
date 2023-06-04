#[doc = "Register `FN_MOD2` reader"]
pub struct R(crate::R<FN_MOD2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FN_MOD2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FN_MOD2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FN_MOD2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FN_MOD2` writer"]
pub struct W(crate::W<FN_MOD2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FN_MOD2_SPEC>;
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
impl From<crate::W<FN_MOD2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FN_MOD2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FN_MOD2` reader - Bypass Merge"]
pub type FN_MOD2_R = crate::BitReader<bool>;
#[doc = "Field `FN_MOD2` writer - Bypass Merge"]
pub type FN_MOD2_W<'a, const O: u8> = crate::BitWriter<'a, u32, FN_MOD2_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Bypass Merge"]
    #[inline(always)]
    pub fn fn_mod2(&self) -> FN_MOD2_R {
        FN_MOD2_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Bypass Merge"]
    #[inline(always)]
    #[must_use]
    pub fn fn_mod2(&mut self) -> FN_MOD2_W<0> {
        FN_MOD2_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Bypass Merge\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fn_mod2](index.html) module"]
pub struct FN_MOD2_SPEC;
impl crate::RegisterSpec for FN_MOD2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fn_mod2::R](R) reader structure"]
impl crate::Readable for FN_MOD2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fn_mod2::W](W) writer structure"]
impl crate::Writable for FN_MOD2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FN_MOD2 to value 0"]
impl crate::Resettable for FN_MOD2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
