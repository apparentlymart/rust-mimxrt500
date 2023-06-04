#[doc = "Register `OTP_PDN` reader"]
pub struct R(crate::R<OTP_PDN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OTP_PDN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OTP_PDN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OTP_PDN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `OTP_PDN` writer"]
pub struct W(crate::W<OTP_PDN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OTP_PDN_SPEC>;
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
impl From<crate::W<OTP_PDN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<OTP_PDN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PDN` reader - Power-down"]
pub type PDN_R = crate::BitReader<bool>;
#[doc = "Field `PDN` writer - Power-down"]
pub type PDN_W<'a, const O: u8> = crate::BitWriter<'a, u32, OTP_PDN_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Power-down"]
    #[inline(always)]
    pub fn pdn(&self) -> PDN_R {
        PDN_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Power-down"]
    #[inline(always)]
    #[must_use]
    pub fn pdn(&mut self) -> PDN_W<0> {
        PDN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Power-down\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otp_pdn](index.html) module"]
pub struct OTP_PDN_SPEC;
impl crate::RegisterSpec for OTP_PDN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [otp_pdn::R](R) reader structure"]
impl crate::Readable for OTP_PDN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [otp_pdn::W](W) writer structure"]
impl crate::Writable for OTP_PDN_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets OTP_PDN to value 0x01"]
impl crate::Resettable for OTP_PDN_SPEC {
    const RESET_VALUE: Self::Ux = 0x01;
}
