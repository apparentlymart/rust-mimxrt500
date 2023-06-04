#[doc = "Register `INTSTAT` reader"]
pub struct R(crate::R<INTSTAT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INTSTAT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INTSTAT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INTSTAT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `DONE` reader - If set, interrupt is caused by accelerator being done."]
pub type DONE_R = crate::BitReader<DONE_A>;
#[doc = "If set, interrupt is caused by accelerator being done.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DONE_A {
    #[doc = "0: Not caused by accelerator being done"]
    NOT_CAUSED = 0,
    #[doc = "1: Caused by accelerator being done"]
    CAUSED = 1,
}
impl From<DONE_A> for bool {
    #[inline(always)]
    fn from(variant: DONE_A) -> Self {
        variant as u8 != 0
    }
}
impl DONE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DONE_A {
        match self.bits {
            false => DONE_A::NOT_CAUSED,
            true => DONE_A::CAUSED,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_CAUSED`"]
    #[inline(always)]
    pub fn is_not_caused(&self) -> bool {
        *self == DONE_A::NOT_CAUSED
    }
    #[doc = "Checks if the value of the field is `CAUSED`"]
    #[inline(always)]
    pub fn is_caused(&self) -> bool {
        *self == DONE_A::CAUSED
    }
}
impl R {
    #[doc = "Bit 0 - If set, interrupt is caused by accelerator being done."]
    #[inline(always)]
    pub fn done(&self) -> DONE_R {
        DONE_R::new((self.bits & 1) != 0)
    }
}
#[doc = "Interrupt status\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intstat](index.html) module"]
pub struct INTSTAT_SPEC;
impl crate::RegisterSpec for INTSTAT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [intstat::R](R) reader structure"]
impl crate::Readable for INTSTAT_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets INTSTAT to value 0"]
impl crate::Resettable for INTSTAT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
