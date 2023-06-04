#[doc = "Register `SUBSEC` reader"]
pub struct R(crate::R<SUBSEC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SUBSEC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SUBSEC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SUBSEC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `RTC_SUBSEC` reader - RTC Sub-second Counter"]
pub type RTC_SUBSEC_R = crate::FieldReader<u16, u16>;
impl R {
    #[doc = "Bits 0:14 - RTC Sub-second Counter"]
    #[inline(always)]
    pub fn rtc_subsec(&self) -> RTC_SUBSEC_R {
        RTC_SUBSEC_R::new((self.bits & 0x7fff) as u16)
    }
}
#[doc = "RTC Sub-second Counter\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [subsec](index.html) module"]
pub struct SUBSEC_SPEC;
impl crate::RegisterSpec for SUBSEC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [subsec::R](R) reader structure"]
impl crate::Readable for SUBSEC_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets SUBSEC to value 0"]
impl crate::Resettable for SUBSEC_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
