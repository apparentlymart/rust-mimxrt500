#[doc = "Register `OTP_READ_DATA` reader"]
pub struct R(crate::R<OTP_READ_DATA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OTP_READ_DATA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OTP_READ_DATA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OTP_READ_DATA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `READ_DATA` reader - Fuse word read data from read operation"]
pub type READ_DATA_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Fuse word read data from read operation"]
    #[inline(always)]
    pub fn read_data(&self) -> READ_DATA_R {
        READ_DATA_R::new(self.bits)
    }
}
#[doc = "OTP read data\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otp_read_data](index.html) module"]
pub struct OTP_READ_DATA_SPEC;
impl crate::RegisterSpec for OTP_READ_DATA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [otp_read_data::R](R) reader structure"]
impl crate::Readable for OTP_READ_DATA_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets OTP_READ_DATA to value 0"]
impl crate::Resettable for OTP_READ_DATA_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
