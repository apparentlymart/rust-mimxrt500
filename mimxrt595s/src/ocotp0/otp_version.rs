#[doc = "Register `OTP_VERSION` reader"]
pub struct R(crate::R<OTP_VERSION_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OTP_VERSION_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OTP_VERSION_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OTP_VERSION_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `STEP_VER` reader - Step version"]
pub type STEP_VER_R = crate::FieldReader<u16, u16>;
#[doc = "Field `MINOR_VER` reader - Minor version"]
pub type MINOR_VER_R = crate::FieldReader<u8, u8>;
#[doc = "Field `MAJOR_VER` reader - Major version"]
pub type MAJOR_VER_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 0:15 - Step version"]
    #[inline(always)]
    pub fn step_ver(&self) -> STEP_VER_R {
        STEP_VER_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:23 - Minor version"]
    #[inline(always)]
    pub fn minor_ver(&self) -> MINOR_VER_R {
        MINOR_VER_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Major version"]
    #[inline(always)]
    pub fn major_ver(&self) -> MAJOR_VER_R {
        MAJOR_VER_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
#[doc = "VERSION ID\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otp_version](index.html) module"]
pub struct OTP_VERSION_SPEC;
impl crate::RegisterSpec for OTP_VERSION_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [otp_version::R](R) reader structure"]
impl crate::Readable for OTP_VERSION_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets OTP_VERSION to value 0x0800_0000"]
impl crate::Resettable for OTP_VERSION_SPEC {
    const RESET_VALUE: Self::Ux = 0x0800_0000;
}
