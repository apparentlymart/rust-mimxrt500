#[doc = "Register `OTP_CRC_VALUE` reader"]
pub struct R(crate::R<OTP_CRC_VALUE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OTP_CRC_VALUE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OTP_CRC_VALUE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OTP_CRC_VALUE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `CRC_VALUE` reader - CRC result value. When it is locked, reading from it returns value 0xBADA_BADA."]
pub type CRC_VALUE_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - CRC result value. When it is locked, reading from it returns value 0xBADA_BADA."]
    #[inline(always)]
    pub fn crc_value(&self) -> CRC_VALUE_R {
        CRC_VALUE_R::new(self.bits)
    }
}
#[doc = "CRC result\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otp_crc_value](index.html) module"]
pub struct OTP_CRC_VALUE_SPEC;
impl crate::RegisterSpec for OTP_CRC_VALUE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [otp_crc_value::R](R) reader structure"]
impl crate::Readable for OTP_CRC_VALUE_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets OTP_CRC_VALUE to value 0"]
impl crate::Resettable for OTP_CRC_VALUE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
