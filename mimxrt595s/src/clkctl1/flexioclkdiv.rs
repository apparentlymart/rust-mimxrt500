#[doc = "Register `FLEXIOCLKDIV` reader"]
pub struct R(crate::R<FLEXIOCLKDIV_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FLEXIOCLKDIV_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FLEXIOCLKDIV_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FLEXIOCLKDIV_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FLEXIOCLKDIV` writer"]
pub struct W(crate::W<FLEXIOCLKDIV_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FLEXIOCLKDIV_SPEC>;
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
impl From<crate::W<FLEXIOCLKDIV_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FLEXIOCLKDIV_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DIV` reader - FLEXIO Clock Divider Value"]
pub type DIV_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DIV` writer - FLEXIO Clock Divider Value"]
pub type DIV_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FLEXIOCLKDIV_SPEC, u8, u8, 8, O>;
#[doc = "Field `RESET` reader - Reset the Divider Counter"]
pub type RESET_R = crate::BitReader<RESET_A>;
#[doc = "Reset the Divider Counter\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RESET_A {
    #[doc = "0: No effect"]
    DIVIDER_COUNTER_NOT_RESET = 0,
    #[doc = "1: Reset the Divider Counter"]
    DIVIDER_COUNTER_RESET = 1,
}
impl From<RESET_A> for bool {
    #[inline(always)]
    fn from(variant: RESET_A) -> Self {
        variant as u8 != 0
    }
}
impl RESET_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RESET_A {
        match self.bits {
            false => RESET_A::DIVIDER_COUNTER_NOT_RESET,
            true => RESET_A::DIVIDER_COUNTER_RESET,
        }
    }
    #[doc = "Checks if the value of the field is `DIVIDER_COUNTER_NOT_RESET`"]
    #[inline(always)]
    pub fn is_divider_counter_not_reset(&self) -> bool {
        *self == RESET_A::DIVIDER_COUNTER_NOT_RESET
    }
    #[doc = "Checks if the value of the field is `DIVIDER_COUNTER_RESET`"]
    #[inline(always)]
    pub fn is_divider_counter_reset(&self) -> bool {
        *self == RESET_A::DIVIDER_COUNTER_RESET
    }
}
#[doc = "Field `RESET` writer - Reset the Divider Counter"]
pub type RESET_W<'a, const O: u8> = crate::BitWriter<'a, u32, FLEXIOCLKDIV_SPEC, RESET_A, O>;
impl<'a, const O: u8> RESET_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn divider_counter_not_reset(self) -> &'a mut W {
        self.variant(RESET_A::DIVIDER_COUNTER_NOT_RESET)
    }
    #[doc = "Reset the Divider Counter"]
    #[inline(always)]
    pub fn divider_counter_reset(self) -> &'a mut W {
        self.variant(RESET_A::DIVIDER_COUNTER_RESET)
    }
}
#[doc = "Field `HALT` reader - Halt the Divider Counter"]
pub type HALT_R = crate::BitReader<HALT_A>;
#[doc = "Halt the Divider Counter\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HALT_A {
    #[doc = "0: No effect"]
    DIVIDER_COUNTER_NOT_HALT = 0,
    #[doc = "1: Halt (stop) the Divider Counter"]
    DIVIDER_COUNTER_HALT = 1,
}
impl From<HALT_A> for bool {
    #[inline(always)]
    fn from(variant: HALT_A) -> Self {
        variant as u8 != 0
    }
}
impl HALT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HALT_A {
        match self.bits {
            false => HALT_A::DIVIDER_COUNTER_NOT_HALT,
            true => HALT_A::DIVIDER_COUNTER_HALT,
        }
    }
    #[doc = "Checks if the value of the field is `DIVIDER_COUNTER_NOT_HALT`"]
    #[inline(always)]
    pub fn is_divider_counter_not_halt(&self) -> bool {
        *self == HALT_A::DIVIDER_COUNTER_NOT_HALT
    }
    #[doc = "Checks if the value of the field is `DIVIDER_COUNTER_HALT`"]
    #[inline(always)]
    pub fn is_divider_counter_halt(&self) -> bool {
        *self == HALT_A::DIVIDER_COUNTER_HALT
    }
}
#[doc = "Field `HALT` writer - Halt the Divider Counter"]
pub type HALT_W<'a, const O: u8> = crate::BitWriter<'a, u32, FLEXIOCLKDIV_SPEC, HALT_A, O>;
impl<'a, const O: u8> HALT_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn divider_counter_not_halt(self) -> &'a mut W {
        self.variant(HALT_A::DIVIDER_COUNTER_NOT_HALT)
    }
    #[doc = "Halt (stop) the Divider Counter"]
    #[inline(always)]
    pub fn divider_counter_halt(self) -> &'a mut W {
        self.variant(HALT_A::DIVIDER_COUNTER_HALT)
    }
}
#[doc = "Field `REQFLAG` reader - Divider Status Flag"]
pub type REQFLAG_R = crate::BitReader<REQFLAG_A>;
#[doc = "Divider Status Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum REQFLAG_A {
    #[doc = "0: The Divider change has finished (clock being divided must be running for this status to change)."]
    REQFLAG_CHANGE_FINISHED = 0,
    #[doc = "1: The Divider value has changed"]
    REQFLAG_CHANGED = 1,
}
impl From<REQFLAG_A> for bool {
    #[inline(always)]
    fn from(variant: REQFLAG_A) -> Self {
        variant as u8 != 0
    }
}
impl REQFLAG_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REQFLAG_A {
        match self.bits {
            false => REQFLAG_A::REQFLAG_CHANGE_FINISHED,
            true => REQFLAG_A::REQFLAG_CHANGED,
        }
    }
    #[doc = "Checks if the value of the field is `REQFLAG_CHANGE_FINISHED`"]
    #[inline(always)]
    pub fn is_reqflag_change_finished(&self) -> bool {
        *self == REQFLAG_A::REQFLAG_CHANGE_FINISHED
    }
    #[doc = "Checks if the value of the field is `REQFLAG_CHANGED`"]
    #[inline(always)]
    pub fn is_reqflag_changed(&self) -> bool {
        *self == REQFLAG_A::REQFLAG_CHANGED
    }
}
#[doc = "Field `REQFLAG` writer - Divider Status Flag"]
pub type REQFLAG_W<'a, const O: u8> = crate::BitWriter<'a, u32, FLEXIOCLKDIV_SPEC, REQFLAG_A, O>;
impl<'a, const O: u8> REQFLAG_W<'a, O> {
    #[doc = "The Divider change has finished (clock being divided must be running for this status to change)."]
    #[inline(always)]
    pub fn reqflag_change_finished(self) -> &'a mut W {
        self.variant(REQFLAG_A::REQFLAG_CHANGE_FINISHED)
    }
    #[doc = "The Divider value has changed"]
    #[inline(always)]
    pub fn reqflag_changed(self) -> &'a mut W {
        self.variant(REQFLAG_A::REQFLAG_CHANGED)
    }
}
impl R {
    #[doc = "Bits 0:7 - FLEXIO Clock Divider Value"]
    #[inline(always)]
    pub fn div(&self) -> DIV_R {
        DIV_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 29 - Reset the Divider Counter"]
    #[inline(always)]
    pub fn reset(&self) -> RESET_R {
        RESET_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Halt the Divider Counter"]
    #[inline(always)]
    pub fn halt(&self) -> HALT_R {
        HALT_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Divider Status Flag"]
    #[inline(always)]
    pub fn reqflag(&self) -> REQFLAG_R {
        REQFLAG_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - FLEXIO Clock Divider Value"]
    #[inline(always)]
    #[must_use]
    pub fn div(&mut self) -> DIV_W<0> {
        DIV_W::new(self)
    }
    #[doc = "Bit 29 - Reset the Divider Counter"]
    #[inline(always)]
    #[must_use]
    pub fn reset(&mut self) -> RESET_W<29> {
        RESET_W::new(self)
    }
    #[doc = "Bit 30 - Halt the Divider Counter"]
    #[inline(always)]
    #[must_use]
    pub fn halt(&mut self) -> HALT_W<30> {
        HALT_W::new(self)
    }
    #[doc = "Bit 31 - Divider Status Flag"]
    #[inline(always)]
    #[must_use]
    pub fn reqflag(&mut self) -> REQFLAG_W<31> {
        REQFLAG_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "FlexIO Clock Divider\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [flexioclkdiv](index.html) module"]
pub struct FLEXIOCLKDIV_SPEC;
impl crate::RegisterSpec for FLEXIOCLKDIV_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [flexioclkdiv::R](R) reader structure"]
impl crate::Readable for FLEXIOCLKDIV_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [flexioclkdiv::W](W) writer structure"]
impl crate::Writable for FLEXIOCLKDIV_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FLEXIOCLKDIV to value 0x4000_0000"]
impl crate::Resettable for FLEXIOCLKDIV_SPEC {
    const RESET_VALUE: Self::Ux = 0x4000_0000;
}
