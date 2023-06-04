#[doc = "Register `FREQMECTRL_R` reader"]
pub struct R(crate::R<FREQME_FREQMECTRL_R_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FREQME_FREQMECTRL_R_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FREQME_FREQMECTRL_R_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FREQME_FREQMECTRL_R_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `RESULT` reader - Result"]
pub type RESULT_R = crate::FieldReader<u32, u32>;
#[doc = "Field `MEASURE_IN_PROGRESS` reader - Measure in Progress"]
pub type MEASURE_IN_PROGRESS_R = crate::BitReader<MEASURE_IN_PROGRESS_A>;
#[doc = "Measure in Progress\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MEASURE_IN_PROGRESS_A {
    #[doc = "0: Process complete. Measurement cycle is complete. The results are ready in the RESULT field."]
    CYCLE_DONE = 0,
    #[doc = "1: In Progress. Measurement cycle is in progress."]
    IN_PROGRESS = 1,
}
impl From<MEASURE_IN_PROGRESS_A> for bool {
    #[inline(always)]
    fn from(variant: MEASURE_IN_PROGRESS_A) -> Self {
        variant as u8 != 0
    }
}
impl MEASURE_IN_PROGRESS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MEASURE_IN_PROGRESS_A {
        match self.bits {
            false => MEASURE_IN_PROGRESS_A::CYCLE_DONE,
            true => MEASURE_IN_PROGRESS_A::IN_PROGRESS,
        }
    }
    #[doc = "Checks if the value of the field is `CYCLE_DONE`"]
    #[inline(always)]
    pub fn is_cycle_done(&self) -> bool {
        *self == MEASURE_IN_PROGRESS_A::CYCLE_DONE
    }
    #[doc = "Checks if the value of the field is `IN_PROGRESS`"]
    #[inline(always)]
    pub fn is_in_progress(&self) -> bool {
        *self == MEASURE_IN_PROGRESS_A::IN_PROGRESS
    }
}
impl R {
    #[doc = "Bits 0:30 - Result"]
    #[inline(always)]
    pub fn result(&self) -> RESULT_R {
        RESULT_R::new(self.bits & 0x7fff_ffff)
    }
    #[doc = "Bit 31 - Measure in Progress"]
    #[inline(always)]
    pub fn measure_in_progress(&self) -> MEASURE_IN_PROGRESS_R {
        MEASURE_IN_PROGRESS_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[doc = "Frequency Measurement (in Read mode)\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [freqme_freqmectrl_r](index.html) module"]
pub struct FREQME_FREQMECTRL_R_SPEC;
impl crate::RegisterSpec for FREQME_FREQMECTRL_R_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [freqme_freqmectrl_r::R](R) reader structure"]
impl crate::Readable for FREQME_FREQMECTRL_R_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets FREQMECTRL_R to value 0"]
impl crate::Resettable for FREQME_FREQMECTRL_R_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
