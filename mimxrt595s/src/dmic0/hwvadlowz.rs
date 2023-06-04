#[doc = "Register `HWVADLOWZ` reader"]
pub struct R(crate::R<HWVADLOWZ_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HWVADLOWZ_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HWVADLOWZ_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HWVADLOWZ_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `LOWZ` reader - Average Noise-floor Value"]
pub type LOWZ_R = crate::FieldReader<u16, u16>;
impl R {
    #[doc = "Bits 0:15 - Average Noise-floor Value"]
    #[inline(always)]
    pub fn lowz(&self) -> LOWZ_R {
        LOWZ_R::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "HWVAD Noise Envelope Estimator\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hwvadlowz](index.html) module"]
pub struct HWVADLOWZ_SPEC;
impl crate::RegisterSpec for HWVADLOWZ_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [hwvadlowz::R](R) reader structure"]
impl crate::Readable for HWVADLOWZ_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets HWVADLOWZ to value 0"]
impl crate::Resettable for HWVADLOWZ_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
