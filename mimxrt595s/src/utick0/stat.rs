#[doc = "Register `STAT` reader"]
pub struct R(crate::R<STAT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<STAT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<STAT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<STAT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `STAT` writer"]
pub struct W(crate::W<STAT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<STAT_SPEC>;
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
impl From<crate::W<STAT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<STAT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `INTR` reader - Interrupt flag"]
pub type INTR_R = crate::BitReader<INTR_A>;
#[doc = "Interrupt flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum INTR_A {
    #[doc = "0: No interrupt is pending"]
    NOPENDINGINTERRUPT = 0,
    #[doc = "1: An interrupt is pending"]
    PENDINGINTERRUPT = 1,
}
impl From<INTR_A> for bool {
    #[inline(always)]
    fn from(variant: INTR_A) -> Self {
        variant as u8 != 0
    }
}
impl INTR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INTR_A {
        match self.bits {
            false => INTR_A::NOPENDINGINTERRUPT,
            true => INTR_A::PENDINGINTERRUPT,
        }
    }
    #[doc = "Checks if the value of the field is `NOPENDINGINTERRUPT`"]
    #[inline(always)]
    pub fn is_nopendinginterrupt(&self) -> bool {
        *self == INTR_A::NOPENDINGINTERRUPT
    }
    #[doc = "Checks if the value of the field is `PENDINGINTERRUPT`"]
    #[inline(always)]
    pub fn is_pendinginterrupt(&self) -> bool {
        *self == INTR_A::PENDINGINTERRUPT
    }
}
#[doc = "Field `INTR` writer - Interrupt flag"]
pub type INTR_W<'a, const O: u8> = crate::BitWriter<'a, u32, STAT_SPEC, INTR_A, O>;
impl<'a, const O: u8> INTR_W<'a, O> {
    #[doc = "No interrupt is pending"]
    #[inline(always)]
    pub fn nopendinginterrupt(self) -> &'a mut W {
        self.variant(INTR_A::NOPENDINGINTERRUPT)
    }
    #[doc = "An interrupt is pending"]
    #[inline(always)]
    pub fn pendinginterrupt(self) -> &'a mut W {
        self.variant(INTR_A::PENDINGINTERRUPT)
    }
}
#[doc = "Field `ACTIVE` reader - Timer active flag"]
pub type ACTIVE_R = crate::BitReader<ACTIVE_A>;
#[doc = "Timer active flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ACTIVE_A {
    #[doc = "0: The Micro-Tick Timer is not active (stopped)"]
    TIMERISNOTACTIVE = 0,
    #[doc = "1: The Micro-Tick Timer is currently active"]
    TIMERISACTIVE = 1,
}
impl From<ACTIVE_A> for bool {
    #[inline(always)]
    fn from(variant: ACTIVE_A) -> Self {
        variant as u8 != 0
    }
}
impl ACTIVE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ACTIVE_A {
        match self.bits {
            false => ACTIVE_A::TIMERISNOTACTIVE,
            true => ACTIVE_A::TIMERISACTIVE,
        }
    }
    #[doc = "Checks if the value of the field is `TIMERISNOTACTIVE`"]
    #[inline(always)]
    pub fn is_timerisnotactive(&self) -> bool {
        *self == ACTIVE_A::TIMERISNOTACTIVE
    }
    #[doc = "Checks if the value of the field is `TIMERISACTIVE`"]
    #[inline(always)]
    pub fn is_timerisactive(&self) -> bool {
        *self == ACTIVE_A::TIMERISACTIVE
    }
}
#[doc = "Field `ACTIVE` writer - Timer active flag"]
pub type ACTIVE_W<'a, const O: u8> = crate::BitWriter<'a, u32, STAT_SPEC, ACTIVE_A, O>;
impl<'a, const O: u8> ACTIVE_W<'a, O> {
    #[doc = "The Micro-Tick Timer is not active (stopped)"]
    #[inline(always)]
    pub fn timerisnotactive(self) -> &'a mut W {
        self.variant(ACTIVE_A::TIMERISNOTACTIVE)
    }
    #[doc = "The Micro-Tick Timer is currently active"]
    #[inline(always)]
    pub fn timerisactive(self) -> &'a mut W {
        self.variant(ACTIVE_A::TIMERISACTIVE)
    }
}
impl R {
    #[doc = "Bit 0 - Interrupt flag"]
    #[inline(always)]
    pub fn intr(&self) -> INTR_R {
        INTR_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Timer active flag"]
    #[inline(always)]
    pub fn active(&self) -> ACTIVE_R {
        ACTIVE_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Interrupt flag"]
    #[inline(always)]
    #[must_use]
    pub fn intr(&mut self) -> INTR_W<0> {
        INTR_W::new(self)
    }
    #[doc = "Bit 1 - Timer active flag"]
    #[inline(always)]
    #[must_use]
    pub fn active(&mut self) -> ACTIVE_W<1> {
        ACTIVE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Status\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [stat](index.html) module"]
pub struct STAT_SPEC;
impl crate::RegisterSpec for STAT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [stat::R](R) reader structure"]
impl crate::Readable for STAT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [stat::W](W) writer structure"]
impl crate::Writable for STAT_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets STAT to value 0"]
impl crate::Resettable for STAT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
