#[doc = "Register `EPTOGGLE` reader"]
pub struct R(crate::R<EPTOGGLE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EPTOGGLE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EPTOGGLE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EPTOGGLE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `TOGGLE` reader - Endpoint data toggle."]
pub type TOGGLE_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:29 - Endpoint data toggle."]
    #[inline(always)]
    pub fn toggle(&self) -> TOGGLE_R {
        TOGGLE_R::new(self.bits & 0x3fff_ffff)
    }
}
#[doc = "USB Endpoint Toggle\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [eptoggle](index.html) module"]
pub struct EPTOGGLE_SPEC;
impl crate::RegisterSpec for EPTOGGLE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [eptoggle::R](R) reader structure"]
impl crate::Readable for EPTOGGLE_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets EPTOGGLE to value 0"]
impl crate::Resettable for EPTOGGLE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
