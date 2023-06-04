#[doc = "Register `OSEVENT_CTRL` reader"]
pub struct R(crate::R<OSEVENT_CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OSEVENT_CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OSEVENT_CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OSEVENT_CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `OSEVENT_CTRL` writer"]
pub struct W(crate::W<OSEVENT_CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OSEVENT_CTRL_SPEC>;
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
impl From<crate::W<OSEVENT_CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<OSEVENT_CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `OSTIMER_INTRFLAG` reader - Interrupt Flag"]
pub type OSTIMER_INTRFLAG_R = crate::BitReader<bool>;
#[doc = "Field `OSTIMER_INTRFLAG` writer - Interrupt Flag"]
pub type OSTIMER_INTRFLAG_W<'a, const O: u8> =
    crate::BitWriter1C<'a, u32, OSEVENT_CTRL_SPEC, bool, O>;
#[doc = "Field `OSTIMER_INTENA` reader - Interrupt or Wake-Up Request"]
pub type OSTIMER_INTENA_R = crate::BitReader<OSTIMER_INTENA_A>;
#[doc = "Interrupt or Wake-Up Request\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OSTIMER_INTENA_A {
    #[doc = "0: Interrupts blocked"]
    INTERRUPTS_BLOCKED = 0,
    #[doc = "1: Interrupts enabled"]
    INTERRUPTS_ENABLED = 1,
}
impl From<OSTIMER_INTENA_A> for bool {
    #[inline(always)]
    fn from(variant: OSTIMER_INTENA_A) -> Self {
        variant as u8 != 0
    }
}
impl OSTIMER_INTENA_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OSTIMER_INTENA_A {
        match self.bits {
            false => OSTIMER_INTENA_A::INTERRUPTS_BLOCKED,
            true => OSTIMER_INTENA_A::INTERRUPTS_ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `INTERRUPTS_BLOCKED`"]
    #[inline(always)]
    pub fn is_interrupts_blocked(&self) -> bool {
        *self == OSTIMER_INTENA_A::INTERRUPTS_BLOCKED
    }
    #[doc = "Checks if the value of the field is `INTERRUPTS_ENABLED`"]
    #[inline(always)]
    pub fn is_interrupts_enabled(&self) -> bool {
        *self == OSTIMER_INTENA_A::INTERRUPTS_ENABLED
    }
}
#[doc = "Field `OSTIMER_INTENA` writer - Interrupt or Wake-Up Request"]
pub type OSTIMER_INTENA_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, OSEVENT_CTRL_SPEC, OSTIMER_INTENA_A, O>;
impl<'a, const O: u8> OSTIMER_INTENA_W<'a, O> {
    #[doc = "Interrupts blocked"]
    #[inline(always)]
    pub fn interrupts_blocked(self) -> &'a mut W {
        self.variant(OSTIMER_INTENA_A::INTERRUPTS_BLOCKED)
    }
    #[doc = "Interrupts enabled"]
    #[inline(always)]
    pub fn interrupts_enabled(self) -> &'a mut W {
        self.variant(OSTIMER_INTENA_A::INTERRUPTS_ENABLED)
    }
}
#[doc = "Field `MATCH_WR_RDY` reader - EVTimer Match Write Ready"]
pub type MATCH_WR_RDY_R = crate::BitReader<bool>;
#[doc = "Field `MATCH_WR_RDY` writer - EVTimer Match Write Ready"]
pub type MATCH_WR_RDY_W<'a, const O: u8> = crate::BitWriter<'a, u32, OSEVENT_CTRL_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Interrupt Flag"]
    #[inline(always)]
    pub fn ostimer_intrflag(&self) -> OSTIMER_INTRFLAG_R {
        OSTIMER_INTRFLAG_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Interrupt or Wake-Up Request"]
    #[inline(always)]
    pub fn ostimer_intena(&self) -> OSTIMER_INTENA_R {
        OSTIMER_INTENA_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - EVTimer Match Write Ready"]
    #[inline(always)]
    pub fn match_wr_rdy(&self) -> MATCH_WR_RDY_R {
        MATCH_WR_RDY_R::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn ostimer_intrflag(&mut self) -> OSTIMER_INTRFLAG_W<0> {
        OSTIMER_INTRFLAG_W::new(self)
    }
    #[doc = "Bit 1 - Interrupt or Wake-Up Request"]
    #[inline(always)]
    #[must_use]
    pub fn ostimer_intena(&mut self) -> OSTIMER_INTENA_W<1> {
        OSTIMER_INTENA_W::new(self)
    }
    #[doc = "Bit 2 - EVTimer Match Write Ready"]
    #[inline(always)]
    #[must_use]
    pub fn match_wr_rdy(&mut self) -> MATCH_WR_RDY_W<2> {
        MATCH_WR_RDY_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "OSTIMER Control for CPU\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [osevent_ctrl](index.html) module"]
pub struct OSEVENT_CTRL_SPEC;
impl crate::RegisterSpec for OSEVENT_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [osevent_ctrl::R](R) reader structure"]
impl crate::Readable for OSEVENT_CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [osevent_ctrl::W](W) writer structure"]
impl crate::Writable for OSEVENT_CTRL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0x01;
}
#[doc = "`reset()` method sets OSEVENT_CTRL to value 0"]
impl crate::Resettable for OSEVENT_CTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
