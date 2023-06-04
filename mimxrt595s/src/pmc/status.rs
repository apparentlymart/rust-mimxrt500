#[doc = "Register `STATUS` reader"]
pub struct R(crate::R<STATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<STATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<STATUS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<STATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `ACTIVEFSM` reader - General sequencer and finite state machine status"]
pub type ACTIVEFSM_R = crate::BitReader<ACTIVEFSM_A>;
#[doc = "General sequencer and finite state machine status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ACTIVEFSM_A {
    #[doc = "0: All PMC finite state machines are idle. OK to set APPLYCFG to trigger the PMC state machines."]
    DISABLE = 0,
    #[doc = "1: One or more PMC finite state machines are active, do not set APPLYCFG or write to any PDRUNCFG or CTRL register values that are used by the PMC state machines."]
    ENABLE = 1,
}
impl From<ACTIVEFSM_A> for bool {
    #[inline(always)]
    fn from(variant: ACTIVEFSM_A) -> Self {
        variant as u8 != 0
    }
}
impl ACTIVEFSM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ACTIVEFSM_A {
        match self.bits {
            false => ACTIVEFSM_A::DISABLE,
            true => ACTIVEFSM_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == ACTIVEFSM_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == ACTIVEFSM_A::ENABLE
    }
}
impl R {
    #[doc = "Bit 0 - General sequencer and finite state machine status"]
    #[inline(always)]
    pub fn activefsm(&self) -> ACTIVEFSM_R {
        ACTIVEFSM_R::new((self.bits & 1) != 0)
    }
}
#[doc = "Status\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [status](index.html) module"]
pub struct STATUS_SPEC;
impl crate::RegisterSpec for STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [status::R](R) reader structure"]
impl crate::Readable for STATUS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets STATUS to value 0"]
impl crate::Resettable for STATUS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
