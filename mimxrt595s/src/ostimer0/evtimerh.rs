#[doc = "Register `EVTIMERH` reader"]
pub struct R(crate::R<EVTIMERH_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EVTIMERH_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EVTIMERH_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EVTIMERH_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `EVTIMER_COUNT_VALUE` reader - EVTimer Count Value"]
pub type EVTIMER_COUNT_VALUE_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - EVTimer Count Value"]
    #[inline(always)]
    pub fn evtimer_count_value(&self) -> EVTIMER_COUNT_VALUE_R {
        EVTIMER_COUNT_VALUE_R::new(self.bits)
    }
}
#[doc = "EVTIMER High\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [evtimerh](index.html) module"]
pub struct EVTIMERH_SPEC;
impl crate::RegisterSpec for EVTIMERH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [evtimerh::R](R) reader structure"]
impl crate::Readable for EVTIMERH_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets EVTIMERH to value 0"]
impl crate::Resettable for EVTIMERH_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
