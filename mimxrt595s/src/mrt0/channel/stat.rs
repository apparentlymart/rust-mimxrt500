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
#[doc = "Field `INTFLAG` reader - Monitors the interrupt flag. Writing 0 indicates no pending interrupt or no operation. Writing 1 indicates pending interrupt, because TIMERn has reached the end of the time interval. If CTRLn\\[INTEN\\]
in the CONTROLn also gets 1, then the interrupt for timer channel n and the global interrupt are generated. Writing 1 to this field bit clears the interrupt request."]
pub type INTFLAG_R = crate::BitReader<INTFLAG_A>;
#[doc = "Monitors the interrupt flag. Writing 0 indicates no pending interrupt or no operation. Writing 1 indicates pending interrupt, because TIMERn has reached the end of the time interval. If CTRLn\\[INTEN\\]
in the CONTROLn also gets 1, then the interrupt for timer channel n and the global interrupt are generated. Writing 1 to this field bit clears the interrupt request.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum INTFLAG_A {
    #[doc = "0: No pending interrupt."]
    NO_PENDING_INTERRUPT = 0,
    #[doc = "1: Pending interrupt."]
    PENDING_INTERRUPT = 1,
}
impl From<INTFLAG_A> for bool {
    #[inline(always)]
    fn from(variant: INTFLAG_A) -> Self {
        variant as u8 != 0
    }
}
impl INTFLAG_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INTFLAG_A {
        match self.bits {
            false => INTFLAG_A::NO_PENDING_INTERRUPT,
            true => INTFLAG_A::PENDING_INTERRUPT,
        }
    }
    #[doc = "Checks if the value of the field is `NO_PENDING_INTERRUPT`"]
    #[inline(always)]
    pub fn is_no_pending_interrupt(&self) -> bool {
        *self == INTFLAG_A::NO_PENDING_INTERRUPT
    }
    #[doc = "Checks if the value of the field is `PENDING_INTERRUPT`"]
    #[inline(always)]
    pub fn is_pending_interrupt(&self) -> bool {
        *self == INTFLAG_A::PENDING_INTERRUPT
    }
}
#[doc = "Field `INTFLAG` writer - Monitors the interrupt flag. Writing 0 indicates no pending interrupt or no operation. Writing 1 indicates pending interrupt, because TIMERn has reached the end of the time interval. If CTRLn\\[INTEN\\]
in the CONTROLn also gets 1, then the interrupt for timer channel n and the global interrupt are generated. Writing 1 to this field bit clears the interrupt request."]
pub type INTFLAG_W<'a, const O: u8> = crate::BitWriter<'a, u32, STAT_SPEC, INTFLAG_A, O>;
impl<'a, const O: u8> INTFLAG_W<'a, O> {
    #[doc = "No pending interrupt."]
    #[inline(always)]
    pub fn no_pending_interrupt(self) -> &'a mut W {
        self.variant(INTFLAG_A::NO_PENDING_INTERRUPT)
    }
    #[doc = "Pending interrupt."]
    #[inline(always)]
    pub fn pending_interrupt(self) -> &'a mut W {
        self.variant(INTFLAG_A::PENDING_INTERRUPT)
    }
}
#[doc = "Field `RUN` reader - Timer n State"]
pub type RUN_R = crate::BitReader<RUN_A>;
#[doc = "Timer n State\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RUN_A {
    #[doc = "0: Idle state."]
    IDLE_STATE = 0,
    #[doc = "1: Running."]
    RUNNING = 1,
}
impl From<RUN_A> for bool {
    #[inline(always)]
    fn from(variant: RUN_A) -> Self {
        variant as u8 != 0
    }
}
impl RUN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RUN_A {
        match self.bits {
            false => RUN_A::IDLE_STATE,
            true => RUN_A::RUNNING,
        }
    }
    #[doc = "Checks if the value of the field is `IDLE_STATE`"]
    #[inline(always)]
    pub fn is_idle_state(&self) -> bool {
        *self == RUN_A::IDLE_STATE
    }
    #[doc = "Checks if the value of the field is `RUNNING`"]
    #[inline(always)]
    pub fn is_running(&self) -> bool {
        *self == RUN_A::RUNNING
    }
}
#[doc = "Field `INUSE` reader - Channel-In-Use flag"]
pub type INUSE_R = crate::BitReader<INUSE_A>;
#[doc = "Channel-In-Use flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum INUSE_A {
    #[doc = "0: This timer channel is not in use."]
    NO = 0,
    #[doc = "1: This timer channel is in use."]
    YES = 1,
}
impl From<INUSE_A> for bool {
    #[inline(always)]
    fn from(variant: INUSE_A) -> Self {
        variant as u8 != 0
    }
}
impl INUSE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INUSE_A {
        match self.bits {
            false => INUSE_A::NO,
            true => INUSE_A::YES,
        }
    }
    #[doc = "Checks if the value of the field is `NO`"]
    #[inline(always)]
    pub fn is_no(&self) -> bool {
        *self == INUSE_A::NO
    }
    #[doc = "Checks if the value of the field is `YES`"]
    #[inline(always)]
    pub fn is_yes(&self) -> bool {
        *self == INUSE_A::YES
    }
}
#[doc = "Field `INUSE` writer - Channel-In-Use flag"]
pub type INUSE_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, STAT_SPEC, INUSE_A, O>;
impl<'a, const O: u8> INUSE_W<'a, O> {
    #[doc = "This timer channel is not in use."]
    #[inline(always)]
    pub fn no(self) -> &'a mut W {
        self.variant(INUSE_A::NO)
    }
    #[doc = "This timer channel is in use."]
    #[inline(always)]
    pub fn yes(self) -> &'a mut W {
        self.variant(INUSE_A::YES)
    }
}
impl R {
    #[doc = "Bit 0 - Monitors the interrupt flag. Writing 0 indicates no pending interrupt or no operation. Writing 1 indicates pending interrupt, because TIMERn has reached the end of the time interval. If CTRLn\\[INTEN\\]
in the CONTROLn also gets 1, then the interrupt for timer channel n and the global interrupt are generated. Writing 1 to this field bit clears the interrupt request."]
    #[inline(always)]
    pub fn intflag(&self) -> INTFLAG_R {
        INTFLAG_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Timer n State"]
    #[inline(always)]
    pub fn run(&self) -> RUN_R {
        RUN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Channel-In-Use flag"]
    #[inline(always)]
    pub fn inuse(&self) -> INUSE_R {
        INUSE_R::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Monitors the interrupt flag. Writing 0 indicates no pending interrupt or no operation. Writing 1 indicates pending interrupt, because TIMERn has reached the end of the time interval. If CTRLn\\[INTEN\\]
in the CONTROLn also gets 1, then the interrupt for timer channel n and the global interrupt are generated. Writing 1 to this field bit clears the interrupt request."]
    #[inline(always)]
    #[must_use]
    pub fn intflag(&mut self) -> INTFLAG_W<0> {
        INTFLAG_W::new(self)
    }
    #[doc = "Bit 2 - Channel-In-Use flag"]
    #[inline(always)]
    #[must_use]
    pub fn inuse(&mut self) -> INUSE_W<2> {
        INUSE_W::new(self)
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
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0x04;
}
#[doc = "`reset()` method sets STAT to value 0"]
impl crate::Resettable for STAT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
