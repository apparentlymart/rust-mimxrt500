#[doc = "Register `HWVADTHGS` reader"]
pub struct R(crate::R<HWVADTHGS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HWVADTHGS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HWVADTHGS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HWVADTHGS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `HWVADTHGS` writer"]
pub struct W(crate::W<HWVADTHGS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HWVADTHGS_SPEC>;
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
impl From<crate::W<HWVADTHGS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HWVADTHGS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `THGS` reader - Signal Gain Factor"]
pub type THGS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `THGS` writer - Signal Gain Factor"]
pub type THGS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, HWVADTHGS_SPEC, u8, u8, 4, O>;
impl R {
    #[doc = "Bits 0:3 - Signal Gain Factor"]
    #[inline(always)]
    pub fn thgs(&self) -> THGS_R {
        THGS_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Signal Gain Factor"]
    #[inline(always)]
    #[must_use]
    pub fn thgs(&mut self) -> THGS_W<0> {
        THGS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "HWVAD Signal Estimator Gain\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hwvadthgs](index.html) module"]
pub struct HWVADTHGS_SPEC;
impl crate::RegisterSpec for HWVADTHGS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [hwvadthgs::R](R) reader structure"]
impl crate::Readable for HWVADTHGS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [hwvadthgs::W](W) writer structure"]
impl crate::Writable for HWVADTHGS_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets HWVADTHGS to value 0x04"]
impl crate::Resettable for HWVADTHGS_SPEC {
    const RESET_VALUE: Self::Ux = 0x04;
}
