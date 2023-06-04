#[doc = "Register `OTP_WRITE_DATA` reader"]
pub struct R(crate::R<OTP_WRITE_DATA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OTP_WRITE_DATA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OTP_WRITE_DATA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OTP_WRITE_DATA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `OTP_WRITE_DATA` writer"]
pub struct W(crate::W<OTP_WRITE_DATA_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OTP_WRITE_DATA_SPEC>;
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
impl From<crate::W<OTP_WRITE_DATA_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<OTP_WRITE_DATA_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `WRITE_DATA` reader - Write data"]
pub type WRITE_DATA_R = crate::FieldReader<u32, u32>;
#[doc = "Field `WRITE_DATA` writer - Write data"]
pub type WRITE_DATA_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, OTP_WRITE_DATA_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - Write data"]
    #[inline(always)]
    pub fn write_data(&self) -> WRITE_DATA_R {
        WRITE_DATA_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Write data"]
    #[inline(always)]
    #[must_use]
    pub fn write_data(&mut self) -> WRITE_DATA_W<0> {
        WRITE_DATA_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "OTP programming data\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otp_write_data](index.html) module"]
pub struct OTP_WRITE_DATA_SPEC;
impl crate::RegisterSpec for OTP_WRITE_DATA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [otp_write_data::R](R) reader structure"]
impl crate::Readable for OTP_WRITE_DATA_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [otp_write_data::W](W) writer structure"]
impl crate::Writable for OTP_WRITE_DATA_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets OTP_WRITE_DATA to value 0"]
impl crate::Resettable for OTP_WRITE_DATA_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
