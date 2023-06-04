#[doc = "Register `RR_TIMER_CR` reader"]
pub struct R(crate::R<RR_TIMER_CR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RR_TIMER_CR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RR_TIMER_CR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RR_TIMER_CR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RR_TIMER_CR` writer"]
pub struct W(crate::W<RR_TIMER_CR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RR_TIMER_CR_SPEC>;
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
impl From<crate::W<RR_TIMER_CR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RR_TIMER_CR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RR_TIMER_RELOAD` reader - This field establishes the repetitive count rate for the timer. Each time the timer counts down to zero it is reloaded with this value. The rr_trig signal will be generated at a rate of (rr_timer_reload + 1) times the rr_clock period (typically 30.6 uS)"]
pub type RR_TIMER_RELOAD_R = crate::FieldReader<u32, u32>;
#[doc = "Field `RR_TIMER_RELOAD` writer - This field establishes the repetitive count rate for the timer. Each time the timer counts down to zero it is reloaded with this value. The rr_trig signal will be generated at a rate of (rr_timer_reload + 1) times the rr_clock period (typically 30.6 uS)"]
pub type RR_TIMER_RELOAD_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, RR_TIMER_CR_SPEC, u32, u32, 28, O>;
#[doc = "Field `RR_TIMER_ENA` reader - RR_TIMER enable"]
pub type RR_TIMER_ENA_R = crate::BitReader<bool>;
#[doc = "Field `RR_TIMER_ENA` writer - RR_TIMER enable"]
pub type RR_TIMER_ENA_W<'a, const O: u8> = crate::BitWriter<'a, u32, RR_TIMER_CR_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:27 - This field establishes the repetitive count rate for the timer. Each time the timer counts down to zero it is reloaded with this value. The rr_trig signal will be generated at a rate of (rr_timer_reload + 1) times the rr_clock period (typically 30.6 uS)"]
    #[inline(always)]
    pub fn rr_timer_reload(&self) -> RR_TIMER_RELOAD_R {
        RR_TIMER_RELOAD_R::new(self.bits & 0x0fff_ffff)
    }
    #[doc = "Bit 31 - RR_TIMER enable"]
    #[inline(always)]
    pub fn rr_timer_ena(&self) -> RR_TIMER_ENA_R {
        RR_TIMER_ENA_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:27 - This field establishes the repetitive count rate for the timer. Each time the timer counts down to zero it is reloaded with this value. The rr_trig signal will be generated at a rate of (rr_timer_reload + 1) times the rr_clock period (typically 30.6 uS)"]
    #[inline(always)]
    #[must_use]
    pub fn rr_timer_reload(&mut self) -> RR_TIMER_RELOAD_W<0> {
        RR_TIMER_RELOAD_W::new(self)
    }
    #[doc = "Bit 31 - RR_TIMER enable"]
    #[inline(always)]
    #[must_use]
    pub fn rr_timer_ena(&mut self) -> RR_TIMER_ENA_W<31> {
        RR_TIMER_ENA_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Round-Robin Timer Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rr_timer_cr](index.html) module"]
pub struct RR_TIMER_CR_SPEC;
impl crate::RegisterSpec for RR_TIMER_CR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rr_timer_cr::R](R) reader structure"]
impl crate::Readable for RR_TIMER_CR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rr_timer_cr::W](W) writer structure"]
impl crate::Writable for RR_TIMER_CR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RR_TIMER_CR to value 0"]
impl crate::Resettable for RR_TIMER_CR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
