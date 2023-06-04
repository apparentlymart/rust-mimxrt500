#[doc = "Register `OTP_CRC_ADDR` reader"]
pub struct R(crate::R<OTP_CRC_ADDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OTP_CRC_ADDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OTP_CRC_ADDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OTP_CRC_ADDR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `OTP_CRC_ADDR` writer"]
pub struct W(crate::W<OTP_CRC_ADDR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OTP_CRC_ADDR_SPEC>;
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
impl From<crate::W<OTP_CRC_ADDR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<OTP_CRC_ADDR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CRC_START_ADDR` reader - CRC starting fuse word address"]
pub type CRC_START_ADDR_R = crate::FieldReader<u16, u16>;
#[doc = "Field `CRC_START_ADDR` writer - CRC starting fuse word address"]
pub type CRC_START_ADDR_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, OTP_CRC_ADDR_SPEC, u16, u16, 9, O>;
#[doc = "Field `CRC_END_ADDR` reader - CRC ending fuse word address"]
pub type CRC_END_ADDR_R = crate::FieldReader<u16, u16>;
#[doc = "Field `CRC_END_ADDR` writer - CRC ending fuse word address"]
pub type CRC_END_ADDR_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, OTP_CRC_ADDR_SPEC, u16, u16, 9, O>;
#[doc = "Field `CRC_REF_ADDR` reader - CRC reference address"]
pub type CRC_REF_ADDR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CRC_REF_ADDR` writer - CRC reference address"]
pub type CRC_REF_ADDR_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, OTP_CRC_ADDR_SPEC, u8, u8, 3, O>;
impl R {
    #[doc = "Bits 0:8 - CRC starting fuse word address"]
    #[inline(always)]
    pub fn crc_start_addr(&self) -> CRC_START_ADDR_R {
        CRC_START_ADDR_R::new((self.bits & 0x01ff) as u16)
    }
    #[doc = "Bits 12:20 - CRC ending fuse word address"]
    #[inline(always)]
    pub fn crc_end_addr(&self) -> CRC_END_ADDR_R {
        CRC_END_ADDR_R::new(((self.bits >> 12) & 0x01ff) as u16)
    }
    #[doc = "Bits 24:26 - CRC reference address"]
    #[inline(always)]
    pub fn crc_ref_addr(&self) -> CRC_REF_ADDR_R {
        CRC_REF_ADDR_R::new(((self.bits >> 24) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:8 - CRC starting fuse word address"]
    #[inline(always)]
    #[must_use]
    pub fn crc_start_addr(&mut self) -> CRC_START_ADDR_W<0> {
        CRC_START_ADDR_W::new(self)
    }
    #[doc = "Bits 12:20 - CRC ending fuse word address"]
    #[inline(always)]
    #[must_use]
    pub fn crc_end_addr(&mut self) -> CRC_END_ADDR_W<12> {
        CRC_END_ADDR_W::new(self)
    }
    #[doc = "Bits 24:26 - CRC reference address"]
    #[inline(always)]
    #[must_use]
    pub fn crc_ref_addr(&mut self) -> CRC_REF_ADDR_W<24> {
        CRC_REF_ADDR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CRC address range\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otp_crc_addr](index.html) module"]
pub struct OTP_CRC_ADDR_SPEC;
impl crate::RegisterSpec for OTP_CRC_ADDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [otp_crc_addr::R](R) reader structure"]
impl crate::Readable for OTP_CRC_ADDR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [otp_crc_addr::W](W) writer structure"]
impl crate::Writable for OTP_CRC_ADDR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets OTP_CRC_ADDR to value 0"]
impl crate::Resettable for OTP_CRC_ADDR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
