#[doc = "Register `RSTGT_R` reader"]
pub struct R(crate::R<RSTGT_RSTGT_R_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RSTGT_RSTGT_R_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RSTGT_RSTGT_R_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RSTGT_RSTGT_R_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `RSTGTN` reader - Reset Gate Number"]
pub type RSTGTN_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RSTGMS` reader - Reset Gate Domain"]
pub type RSTGMS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RSTGSM` reader - Reset Gate Finite State Machine"]
pub type RSTGSM_R = crate::FieldReader<u8, RSTGSM_A>;
#[doc = "Reset Gate Finite State Machine\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum RSTGSM_A {
    #[doc = "0: Idle, waiting for the first data pattern write."]
    IDLE = 0,
    #[doc = "1: Waiting for the second data pattern write"]
    WAITING = 1,
    #[doc = "2: The 2-write sequence has completed. Generate the specified gate reset(s). After the reset is performed, this machine returns to the idle (waiting for first data pattern write) state."]
    TWO_WRITE_DONE = 2,
}
impl From<RSTGSM_A> for u8 {
    #[inline(always)]
    fn from(variant: RSTGSM_A) -> Self {
        variant as _
    }
}
impl RSTGSM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<RSTGSM_A> {
        match self.bits {
            0 => Some(RSTGSM_A::IDLE),
            1 => Some(RSTGSM_A::WAITING),
            2 => Some(RSTGSM_A::TWO_WRITE_DONE),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `IDLE`"]
    #[inline(always)]
    pub fn is_idle(&self) -> bool {
        *self == RSTGSM_A::IDLE
    }
    #[doc = "Checks if the value of the field is `WAITING`"]
    #[inline(always)]
    pub fn is_waiting(&self) -> bool {
        *self == RSTGSM_A::WAITING
    }
    #[doc = "Checks if the value of the field is `TWO_WRITE_DONE`"]
    #[inline(always)]
    pub fn is_two_write_done(&self) -> bool {
        *self == RSTGSM_A::TWO_WRITE_DONE
    }
}
impl R {
    #[doc = "Bits 0:7 - Reset Gate Number"]
    #[inline(always)]
    pub fn rstgtn(&self) -> RSTGTN_R {
        RSTGTN_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:11 - Reset Gate Domain"]
    #[inline(always)]
    pub fn rstgms(&self) -> RSTGMS_R {
        RSTGMS_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:13 - Reset Gate Finite State Machine"]
    #[inline(always)]
    pub fn rstgsm(&self) -> RSTGSM_R {
        RSTGSM_R::new(((self.bits >> 12) & 3) as u8)
    }
}
#[doc = "Reset Gate Read\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rstgt_rstgt_r](index.html) module"]
pub struct RSTGT_RSTGT_R_SPEC;
impl crate::RegisterSpec for RSTGT_RSTGT_R_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [rstgt_rstgt_r::R](R) reader structure"]
impl crate::Readable for RSTGT_RSTGT_R_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets RSTGT_R to value 0"]
impl crate::Resettable for RSTGT_RSTGT_R_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
