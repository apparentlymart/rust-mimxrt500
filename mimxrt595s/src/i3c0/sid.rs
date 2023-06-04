#[doc = "Register `SID` reader"]
pub struct R(crate::R<SID_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SID_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SID_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SID_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `ID` reader - ID"]
pub type ID_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - ID"]
    #[inline(always)]
    pub fn id(&self) -> ID_R {
        ID_R::new(self.bits)
    }
}
#[doc = "Slave Module ID\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sid](index.html) module"]
pub struct SID_SPEC;
impl crate::RegisterSpec for SID_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sid::R](R) reader structure"]
impl crate::Readable for SID_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets SID to value 0"]
impl crate::Resettable for SID_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
