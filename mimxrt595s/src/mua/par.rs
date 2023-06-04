#[doc = "Register `PAR` reader"]
pub struct R(crate::R<PAR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PAR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PAR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PAR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `PARAMETER` reader - This bitfield contains the parameter settings of MUA."]
pub type PARAMETER_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - This bitfield contains the parameter settings of MUA."]
    #[inline(always)]
    pub fn parameter(&self) -> PARAMETER_R {
        PARAMETER_R::new(self.bits)
    }
}
#[doc = "Parameter Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [par](index.html) module"]
pub struct PAR_SPEC;
impl crate::RegisterSpec for PAR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [par::R](R) reader structure"]
impl crate::Readable for PAR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets PAR to value 0"]
impl crate::Resettable for PAR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
