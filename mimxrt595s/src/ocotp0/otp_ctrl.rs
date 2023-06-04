#[doc = "Register `OTP_CTRL` reader"]
pub struct R(crate::R<OTP_CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OTP_CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OTP_CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OTP_CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `OTP_CTRL` writer"]
pub struct W(crate::W<OTP_CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OTP_CTRL_SPEC>;
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
impl From<crate::W<OTP_CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<OTP_CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ADDR` reader - OTP word address"]
pub type ADDR_R = crate::FieldReader<u16, u16>;
#[doc = "Field `ADDR` writer - OTP word address"]
pub type ADDR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, OTP_CTRL_SPEC, u16, u16, 9, O>;
#[doc = "Field `RELOAD_SHADOWS` reader - Reload Shadow registers"]
pub type RELOAD_SHADOWS_R = crate::BitReader<bool>;
#[doc = "Field `RELOAD_SHADOWS` writer - Reload Shadow registers"]
pub type RELOAD_SHADOWS_W<'a, const O: u8> = crate::BitWriter<'a, u32, OTP_CTRL_SPEC, bool, O>;
#[doc = "Field `CRC_TEST` reader - Set to start CRC calculation."]
pub type CRC_TEST_R = crate::BitReader<bool>;
#[doc = "Field `CRC_TEST` writer - Set to start CRC calculation."]
pub type CRC_TEST_W<'a, const O: u8> = crate::BitWriter<'a, u32, OTP_CTRL_SPEC, bool, O>;
#[doc = "Field `WORDLOCK` reader - Wordlock"]
pub type WORDLOCK_R = crate::BitReader<bool>;
#[doc = "Field `WORDLOCK` writer - Wordlock"]
pub type WORDLOCK_W<'a, const O: u8> = crate::BitWriter<'a, u32, OTP_CTRL_SPEC, bool, O>;
#[doc = "Field `WR_UNLOCK` reader - Write unlock"]
pub type WR_UNLOCK_R = crate::FieldReader<u16, u16>;
#[doc = "Field `WR_UNLOCK` writer - Write unlock"]
pub type WR_UNLOCK_W<'a, const O: u8> = crate::FieldWriter<'a, u32, OTP_CTRL_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:8 - OTP word address"]
    #[inline(always)]
    pub fn addr(&self) -> ADDR_R {
        ADDR_R::new((self.bits & 0x01ff) as u16)
    }
    #[doc = "Bit 11 - Reload Shadow registers"]
    #[inline(always)]
    pub fn reload_shadows(&self) -> RELOAD_SHADOWS_R {
        RELOAD_SHADOWS_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Set to start CRC calculation."]
    #[inline(always)]
    pub fn crc_test(&self) -> CRC_TEST_R {
        CRC_TEST_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 15 - Wordlock"]
    #[inline(always)]
    pub fn wordlock(&self) -> WORDLOCK_R {
        WORDLOCK_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:31 - Write unlock"]
    #[inline(always)]
    pub fn wr_unlock(&self) -> WR_UNLOCK_R {
        WR_UNLOCK_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:8 - OTP word address"]
    #[inline(always)]
    #[must_use]
    pub fn addr(&mut self) -> ADDR_W<0> {
        ADDR_W::new(self)
    }
    #[doc = "Bit 11 - Reload Shadow registers"]
    #[inline(always)]
    #[must_use]
    pub fn reload_shadows(&mut self) -> RELOAD_SHADOWS_W<11> {
        RELOAD_SHADOWS_W::new(self)
    }
    #[doc = "Bit 12 - Set to start CRC calculation."]
    #[inline(always)]
    #[must_use]
    pub fn crc_test(&mut self) -> CRC_TEST_W<12> {
        CRC_TEST_W::new(self)
    }
    #[doc = "Bit 15 - Wordlock"]
    #[inline(always)]
    #[must_use]
    pub fn wordlock(&mut self) -> WORDLOCK_W<15> {
        WORDLOCK_W::new(self)
    }
    #[doc = "Bits 16:31 - Write unlock"]
    #[inline(always)]
    #[must_use]
    pub fn wr_unlock(&mut self) -> WR_UNLOCK_W<16> {
        WR_UNLOCK_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Control/Address\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otp_ctrl](index.html) module"]
pub struct OTP_CTRL_SPEC;
impl crate::RegisterSpec for OTP_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [otp_ctrl::R](R) reader structure"]
impl crate::Readable for OTP_CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [otp_ctrl::W](W) writer structure"]
impl crate::Writable for OTP_CTRL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets OTP_CTRL to value 0"]
impl crate::Resettable for OTP_CTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
