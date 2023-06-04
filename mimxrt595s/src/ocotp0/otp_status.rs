#[doc = "Register `OTP_STATUS` reader"]
pub struct R(crate::R<OTP_STATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OTP_STATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OTP_STATUS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OTP_STATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `OTP_STATUS` writer"]
pub struct W(crate::W<OTP_STATUS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OTP_STATUS_SPEC>;
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
impl From<crate::W<OTP_STATUS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<OTP_STATUS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SEC` reader - OTP Single Error Corrected status of ECC during read operation. Write 1 to clear."]
pub type SEC_R = crate::BitReader<bool>;
#[doc = "Field `SEC` writer - OTP Single Error Corrected status of ECC during read operation. Write 1 to clear."]
pub type SEC_W<'a, const O: u8> = crate::BitWriter<'a, u32, OTP_STATUS_SPEC, bool, O>;
#[doc = "Field `DED` reader - OTP Double Error Detection status of ECC during read operation. Write 1 to clear."]
pub type DED_R = crate::BitReader<bool>;
#[doc = "Field `DED` writer - OTP Double Error Detection status of ECC during read operation. Write 1 to clear."]
pub type DED_W<'a, const O: u8> = crate::BitWriter<'a, u32, OTP_STATUS_SPEC, bool, O>;
#[doc = "Field `LOCKED` reader - OTP LOCKED status during read/write operation. Write 1 to clear."]
pub type LOCKED_R = crate::BitReader<bool>;
#[doc = "Field `LOCKED` writer - OTP LOCKED status during read/write operation. Write 1 to clear."]
pub type LOCKED_W<'a, const O: u8> = crate::BitWriter<'a, u32, OTP_STATUS_SPEC, bool, O>;
#[doc = "Field `PROGFAIL` reader - OTP PROGFAIL status. Write 1 to clear."]
pub type PROGFAIL_R = crate::BitReader<bool>;
#[doc = "Field `PROGFAIL` writer - OTP PROGFAIL status. Write 1 to clear."]
pub type PROGFAIL_W<'a, const O: u8> = crate::BitWriter<'a, u32, OTP_STATUS_SPEC, bool, O>;
#[doc = "Field `ACK` reader - OTP ACK value"]
pub type ACK_R = crate::BitReader<bool>;
#[doc = "Field `PWOK` reader - OTP Power OK status. Indicate that power VDD are in the operating range."]
pub type PWOK_R = crate::BitReader<bool>;
#[doc = "Field `SEC_RELOAD` reader - OTP Single Error Corrected status of ECC during reload process. Write 1 to clear."]
pub type SEC_RELOAD_R = crate::BitReader<bool>;
#[doc = "Field `SEC_RELOAD` writer - OTP Single Error Corrected status of ECC during reload process. Write 1 to clear."]
pub type SEC_RELOAD_W<'a, const O: u8> = crate::BitWriter<'a, u32, OTP_STATUS_SPEC, bool, O>;
#[doc = "Field `DED_RELOAD` reader - OTP Double Error Detect status of ECC during reload process. Write 1 to clear."]
pub type DED_RELOAD_R = crate::BitReader<bool>;
#[doc = "Field `DED_RELOAD` writer - OTP Double Error Detect status of ECC during reload process. Write 1 to clear."]
pub type DED_RELOAD_W<'a, const O: u8> = crate::BitWriter<'a, u32, OTP_STATUS_SPEC, bool, O>;
#[doc = "Field `BUSY` reader - OTP controller status"]
pub type BUSY_R = crate::BitReader<bool>;
#[doc = "Field `ERROR` reader - Error"]
pub type ERROR_R = crate::BitReader<bool>;
#[doc = "Field `ERROR` writer - Error"]
pub type ERROR_W<'a, const O: u8> = crate::BitWriter<'a, u32, OTP_STATUS_SPEC, bool, O>;
#[doc = "Field `CRC_FAIL` reader - CRC failed when set by hardware for CRC operation. Write 1 to clear."]
pub type CRC_FAIL_R = crate::BitReader<bool>;
#[doc = "Field `CRC_FAIL` writer - CRC failed when set by hardware for CRC operation. Write 1 to clear."]
pub type CRC_FAIL_W<'a, const O: u8> = crate::BitWriter<'a, u32, OTP_STATUS_SPEC, bool, O>;
#[doc = "Field `FUSE_LATCHED` reader - Indicate all shadows registers have been loaded with their corresponding fuse words when set by the controller after reset."]
pub type FUSE_LATCHED_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 9 - OTP Single Error Corrected status of ECC during read operation. Write 1 to clear."]
    #[inline(always)]
    pub fn sec(&self) -> SEC_R {
        SEC_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - OTP Double Error Detection status of ECC during read operation. Write 1 to clear."]
    #[inline(always)]
    pub fn ded(&self) -> DED_R {
        DED_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - OTP LOCKED status during read/write operation. Write 1 to clear."]
    #[inline(always)]
    pub fn locked(&self) -> LOCKED_R {
        LOCKED_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - OTP PROGFAIL status. Write 1 to clear."]
    #[inline(always)]
    pub fn progfail(&self) -> PROGFAIL_R {
        PROGFAIL_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - OTP ACK value"]
    #[inline(always)]
    pub fn ack(&self) -> ACK_R {
        ACK_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - OTP Power OK status. Indicate that power VDD are in the operating range."]
    #[inline(always)]
    pub fn pwok(&self) -> PWOK_R {
        PWOK_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 20 - OTP Single Error Corrected status of ECC during reload process. Write 1 to clear."]
    #[inline(always)]
    pub fn sec_reload(&self) -> SEC_RELOAD_R {
        SEC_RELOAD_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - OTP Double Error Detect status of ECC during reload process. Write 1 to clear."]
    #[inline(always)]
    pub fn ded_reload(&self) -> DED_RELOAD_R {
        DED_RELOAD_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - OTP controller status"]
    #[inline(always)]
    pub fn busy(&self) -> BUSY_R {
        BUSY_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Error"]
    #[inline(always)]
    pub fn error(&self) -> ERROR_R {
        ERROR_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - CRC failed when set by hardware for CRC operation. Write 1 to clear."]
    #[inline(always)]
    pub fn crc_fail(&self) -> CRC_FAIL_R {
        CRC_FAIL_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Indicate all shadows registers have been loaded with their corresponding fuse words when set by the controller after reset."]
    #[inline(always)]
    pub fn fuse_latched(&self) -> FUSE_LATCHED_R {
        FUSE_LATCHED_R::new(((self.bits >> 25) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 9 - OTP Single Error Corrected status of ECC during read operation. Write 1 to clear."]
    #[inline(always)]
    #[must_use]
    pub fn sec(&mut self) -> SEC_W<9> {
        SEC_W::new(self)
    }
    #[doc = "Bit 10 - OTP Double Error Detection status of ECC during read operation. Write 1 to clear."]
    #[inline(always)]
    #[must_use]
    pub fn ded(&mut self) -> DED_W<10> {
        DED_W::new(self)
    }
    #[doc = "Bit 11 - OTP LOCKED status during read/write operation. Write 1 to clear."]
    #[inline(always)]
    #[must_use]
    pub fn locked(&mut self) -> LOCKED_W<11> {
        LOCKED_W::new(self)
    }
    #[doc = "Bit 12 - OTP PROGFAIL status. Write 1 to clear."]
    #[inline(always)]
    #[must_use]
    pub fn progfail(&mut self) -> PROGFAIL_W<12> {
        PROGFAIL_W::new(self)
    }
    #[doc = "Bit 20 - OTP Single Error Corrected status of ECC during reload process. Write 1 to clear."]
    #[inline(always)]
    #[must_use]
    pub fn sec_reload(&mut self) -> SEC_RELOAD_W<20> {
        SEC_RELOAD_W::new(self)
    }
    #[doc = "Bit 21 - OTP Double Error Detect status of ECC during reload process. Write 1 to clear."]
    #[inline(always)]
    #[must_use]
    pub fn ded_reload(&mut self) -> DED_RELOAD_W<21> {
        DED_RELOAD_W::new(self)
    }
    #[doc = "Bit 23 - Error"]
    #[inline(always)]
    #[must_use]
    pub fn error(&mut self) -> ERROR_W<23> {
        ERROR_W::new(self)
    }
    #[doc = "Bit 24 - CRC failed when set by hardware for CRC operation. Write 1 to clear."]
    #[inline(always)]
    #[must_use]
    pub fn crc_fail(&mut self) -> CRC_FAIL_W<24> {
        CRC_FAIL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "OTP Status\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otp_status](index.html) module"]
pub struct OTP_STATUS_SPEC;
impl crate::RegisterSpec for OTP_STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [otp_status::R](R) reader structure"]
impl crate::Readable for OTP_STATUS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [otp_status::W](W) writer structure"]
impl crate::Writable for OTP_STATUS_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets OTP_STATUS to value 0x0200_6000"]
impl crate::Resettable for OTP_STATUS_SPEC {
    const RESET_VALUE: Self::Ux = 0x0200_6000;
}
