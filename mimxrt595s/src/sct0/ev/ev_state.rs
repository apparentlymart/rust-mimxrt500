#[doc = "Register `EV_STATE` reader"]
pub struct R(crate::R<EV_STATE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EV_STATE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EV_STATE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EV_STATE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EV_STATE` writer"]
pub struct W(crate::W<EV_STATE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EV_STATE_SPEC>;
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
impl From<crate::W<EV_STATE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EV_STATE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `STATEMSKn` reader - Event State Mask"]
pub type STATEMSKN_R = crate::FieldReader<u32, u32>;
#[doc = "Field `STATEMSKn` writer - Event State Mask"]
pub type STATEMSKN_W<'a, const O: u8> = crate::FieldWriter<'a, u32, EV_STATE_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - Event State Mask"]
    #[inline(always)]
    pub fn statemskn(&self) -> STATEMSKN_R {
        STATEMSKN_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Event State Mask"]
    #[inline(always)]
    #[must_use]
    pub fn statemskn(&mut self) -> STATEMSKN_W<0> {
        STATEMSKN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Event n State\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ev_state](index.html) module"]
pub struct EV_STATE_SPEC;
impl crate::RegisterSpec for EV_STATE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ev_state::R](R) reader structure"]
impl crate::Readable for EV_STATE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ev_state::W](W) writer structure"]
impl crate::Writable for EV_STATE_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets EV_STATE to value 0"]
impl crate::Resettable for EV_STATE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
