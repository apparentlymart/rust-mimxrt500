#[doc = "Register `SYSTEM_NSTICK_CALIB` reader"]
pub struct R(crate::R<SYSTEM_NSTICK_CALIB_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SYSTEM_NSTICK_CALIB_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SYSTEM_NSTICK_CALIB_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SYSTEM_NSTICK_CALIB_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SYSTEM_NSTICK_CALIB` writer"]
pub struct W(crate::W<SYSTEM_NSTICK_CALIB_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SYSTEM_NSTICK_CALIB_SPEC>;
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
impl From<crate::W<SYSTEM_NSTICK_CALIB_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SYSTEM_NSTICK_CALIB_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SYSTEM_NSTICK_CALIB` reader - M33 non secure tick calibration"]
pub type SYSTEM_NSTICK_CALIB_R = crate::FieldReader<u32, u32>;
#[doc = "Field `SYSTEM_NSTICK_CALIB` writer - M33 non secure tick calibration"]
pub type SYSTEM_NSTICK_CALIB_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, SYSTEM_NSTICK_CALIB_SPEC, u32, u32, 27, O>;
impl R {
    #[doc = "Bits 0:26 - M33 non secure tick calibration"]
    #[inline(always)]
    pub fn system_nstick_calib(&self) -> SYSTEM_NSTICK_CALIB_R {
        SYSTEM_NSTICK_CALIB_R::new(self.bits & 0x07ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:26 - M33 non secure tick calibration"]
    #[inline(always)]
    #[must_use]
    pub fn system_nstick_calib(&mut self) -> SYSTEM_NSTICK_CALIB_W<0> {
        SYSTEM_NSTICK_CALIB_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "System non-secure tick calibration\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [system_nstick_calib](index.html) module"]
pub struct SYSTEM_NSTICK_CALIB_SPEC;
impl crate::RegisterSpec for SYSTEM_NSTICK_CALIB_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [system_nstick_calib::R](R) reader structure"]
impl crate::Readable for SYSTEM_NSTICK_CALIB_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [system_nstick_calib::W](W) writer structure"]
impl crate::Writable for SYSTEM_NSTICK_CALIB_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SYSTEM_NSTICK_CALIB to value 0"]
impl crate::Resettable for SYSTEM_NSTICK_CALIB_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
