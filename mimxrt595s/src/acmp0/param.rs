#[doc = "Register `PARAM` reader"]
pub struct R(crate::R<PARAM_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PARAM_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PARAM_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PARAM_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `PARAM` reader - Parameter Registers. This read only filed returns the feature parameters implemented along with the Version ID register."]
pub type PARAM_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Parameter Registers. This read only filed returns the feature parameters implemented along with the Version ID register."]
    #[inline(always)]
    pub fn param(&self) -> PARAM_R {
        PARAM_R::new(self.bits)
    }
}
#[doc = "Parameter Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [param](index.html) module"]
pub struct PARAM_SPEC;
impl crate::RegisterSpec for PARAM_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [param::R](R) reader structure"]
impl crate::Readable for PARAM_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets PARAM to value 0"]
impl crate::Resettable for PARAM_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
