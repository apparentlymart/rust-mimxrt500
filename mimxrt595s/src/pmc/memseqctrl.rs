#[doc = "Register `MEMSEQCTRL` reader"]
pub struct R(crate::R<MEMSEQCTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MEMSEQCTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MEMSEQCTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MEMSEQCTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MEMSEQCTRL` writer"]
pub struct W(crate::W<MEMSEQCTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MEMSEQCTRL_SPEC>;
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
impl From<crate::W<MEMSEQCTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MEMSEQCTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MEMSEQNUM` reader - Number of memories to turn on/off at a time."]
pub type MEMSEQNUM_R = crate::FieldReader<u8, MEMSEQNUM_A>;
#[doc = "Number of memories to turn on/off at a time.\n\nValue on reset: 63"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MEMSEQNUM_A {
    #[doc = "0: For main system SRAM partitions, 1st array power then periphery power (400ns worst case delay)"]
    DISABLE = 0,
    #[doc = "1: Turn on 1st memory partition at a time, periphery and array power.switches at the same time."]
    ENABLE = 1,
    #[doc = "5: Turn on 5th memory partitions in parallel, periphery and array."]
    VALUE_0B101 = 5,
    #[doc = "63: All memories are switched on/off at the same time"]
    VALUE_0B111111 = 63,
}
impl From<MEMSEQNUM_A> for u8 {
    #[inline(always)]
    fn from(variant: MEMSEQNUM_A) -> Self {
        variant as _
    }
}
impl MEMSEQNUM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<MEMSEQNUM_A> {
        match self.bits {
            0 => Some(MEMSEQNUM_A::DISABLE),
            1 => Some(MEMSEQNUM_A::ENABLE),
            5 => Some(MEMSEQNUM_A::VALUE_0B101),
            63 => Some(MEMSEQNUM_A::VALUE_0B111111),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == MEMSEQNUM_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == MEMSEQNUM_A::ENABLE
    }
    #[doc = "Checks if the value of the field is `VALUE_0B101`"]
    #[inline(always)]
    pub fn is_value_0b101(&self) -> bool {
        *self == MEMSEQNUM_A::VALUE_0B101
    }
    #[doc = "Checks if the value of the field is `VALUE_0B111111`"]
    #[inline(always)]
    pub fn is_value_0b111111(&self) -> bool {
        *self == MEMSEQNUM_A::VALUE_0B111111
    }
}
#[doc = "Field `MEMSEQNUM` writer - Number of memories to turn on/off at a time."]
pub type MEMSEQNUM_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, MEMSEQCTRL_SPEC, u8, MEMSEQNUM_A, 6, O>;
impl<'a, const O: u8> MEMSEQNUM_W<'a, O> {
    #[doc = "For main system SRAM partitions, 1st array power then periphery power (400ns worst case delay)"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(MEMSEQNUM_A::DISABLE)
    }
    #[doc = "Turn on 1st memory partition at a time, periphery and array power.switches at the same time."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(MEMSEQNUM_A::ENABLE)
    }
    #[doc = "Turn on 5th memory partitions in parallel, periphery and array."]
    #[inline(always)]
    pub fn value_0b101(self) -> &'a mut W {
        self.variant(MEMSEQNUM_A::VALUE_0B101)
    }
    #[doc = "All memories are switched on/off at the same time"]
    #[inline(always)]
    pub fn value_0b111111(self) -> &'a mut W {
        self.variant(MEMSEQNUM_A::VALUE_0B111111)
    }
}
impl R {
    #[doc = "Bits 0:5 - Number of memories to turn on/off at a time."]
    #[inline(always)]
    pub fn memseqnum(&self) -> MEMSEQNUM_R {
        MEMSEQNUM_R::new((self.bits & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - Number of memories to turn on/off at a time."]
    #[inline(always)]
    #[must_use]
    pub fn memseqnum(&mut self) -> MEMSEQNUM_W<0> {
        MEMSEQNUM_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PMC Memory sequencer Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [memseqctrl](index.html) module"]
pub struct MEMSEQCTRL_SPEC;
impl crate::RegisterSpec for MEMSEQCTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [memseqctrl::R](R) reader structure"]
impl crate::Readable for MEMSEQCTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [memseqctrl::W](W) writer structure"]
impl crate::Writable for MEMSEQCTRL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MEMSEQCTRL to value 0x3f"]
impl crate::Resettable for MEMSEQCTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0x3f;
}
