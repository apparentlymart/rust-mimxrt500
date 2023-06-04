#[doc = "Register `CAPTURE_L` reader"]
pub struct R(crate::R<CAPTURE_L_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CAPTURE_L_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CAPTURE_L_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CAPTURE_L_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `CAPTURE_VALUE` reader - EVTimer Capture Value"]
pub type CAPTURE_VALUE_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - EVTimer Capture Value"]
    #[inline(always)]
    pub fn capture_value(&self) -> CAPTURE_VALUE_R {
        CAPTURE_VALUE_R::new(self.bits)
    }
}
#[doc = "Local Capture Low for CPU\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [capture_l](index.html) module"]
pub struct CAPTURE_L_SPEC;
impl crate::RegisterSpec for CAPTURE_L_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [capture_l::R](R) reader structure"]
impl crate::Readable for CAPTURE_L_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets CAPTURE_L to value 0"]
impl crate::Resettable for CAPTURE_L_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
