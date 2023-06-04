#[doc = "Register `MRMSG_SDR` reader"]
pub struct R(crate::R<MRMSG_SDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MRMSG_SDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MRMSG_SDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MRMSG_SDR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `DATA` reader - Data"]
pub type DATA_R = crate::FieldReader<u16, u16>;
impl R {
    #[doc = "Bits 0:15 - Data"]
    #[inline(always)]
    pub fn data(&self) -> DATA_R {
        DATA_R::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "Master Read Message in SDR mode\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mrmsg_sdr](index.html) module"]
pub struct MRMSG_SDR_SPEC;
impl crate::RegisterSpec for MRMSG_SDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mrmsg_sdr::R](R) reader structure"]
impl crate::Readable for MRMSG_SDR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets MRMSG_SDR to value 0"]
impl crate::Resettable for MRMSG_SDR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
