#[doc = "Register `HCCPARAMS` reader"]
pub struct R(crate::R<HCCPARAMS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HCCPARAMS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HCCPARAMS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HCCPARAMS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `HCCPARAMS` writer"]
pub struct W(crate::W<HCCPARAMS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HCCPARAMS_SPEC>;
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
impl From<crate::W<HCCPARAMS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HCCPARAMS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LPMC` reader - Link Power Management Capability"]
pub type LPMC_R = crate::BitReader<bool>;
#[doc = "Field `LPMC` writer - Link Power Management Capability"]
pub type LPMC_W<'a, const O: u8> = crate::BitWriter<'a, u32, HCCPARAMS_SPEC, bool, O>;
impl R {
    #[doc = "Bit 17 - Link Power Management Capability"]
    #[inline(always)]
    pub fn lpmc(&self) -> LPMC_R {
        LPMC_R::new(((self.bits >> 17) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 17 - Link Power Management Capability"]
    #[inline(always)]
    #[must_use]
    pub fn lpmc(&mut self) -> LPMC_W<17> {
        LPMC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "INT PTD Base Address\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hccparams](index.html) module"]
pub struct HCCPARAMS_SPEC;
impl crate::RegisterSpec for HCCPARAMS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [hccparams::R](R) reader structure"]
impl crate::Readable for HCCPARAMS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [hccparams::W](W) writer structure"]
impl crate::Writable for HCCPARAMS_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets HCCPARAMS to value 0"]
impl crate::Resettable for HCCPARAMS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
