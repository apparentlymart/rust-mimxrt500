#[doc = "Register `OTP_SHADOW[%s]` reader"]
pub struct R(crate::R<OTP_SHADOW_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OTP_SHADOW_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OTP_SHADOW_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OTP_SHADOW_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `OTP_SHADOW[%s]` writer"]
pub struct W(crate::W<OTP_SHADOW_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OTP_SHADOW_SPEC>;
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
impl From<crate::W<OTP_SHADOW_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<OTP_SHADOW_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SHADOW` reader - OTP shadow"]
pub type SHADOW_R = crate::FieldReader<u32, u32>;
#[doc = "Field `SHADOW` writer - OTP shadow"]
pub type SHADOW_W<'a, const O: u8> = crate::FieldWriter<'a, u32, OTP_SHADOW_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - OTP shadow"]
    #[inline(always)]
    pub fn shadow(&self) -> SHADOW_R {
        SHADOW_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - OTP shadow"]
    #[inline(always)]
    #[must_use]
    pub fn shadow(&mut self) -> SHADOW_W<0> {
        SHADOW_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "One Time Programmable Controller shadow\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otp_shadow](index.html) module"]
pub struct OTP_SHADOW_SPEC;
impl crate::RegisterSpec for OTP_SHADOW_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [otp_shadow::R](R) reader structure"]
impl crate::Readable for OTP_SHADOW_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [otp_shadow::W](W) writer structure"]
impl crate::Writable for OTP_SHADOW_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets OTP_SHADOW[%s]
to value 0"]
impl crate::Resettable for OTP_SHADOW_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
