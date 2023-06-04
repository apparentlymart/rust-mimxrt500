#[doc = "Register `MRDATAB` reader"]
pub struct R(crate::R<MRDATAB_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MRDATAB_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MRDATAB_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MRDATAB_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `VALUE` reader - VALUE"]
pub type VALUE_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 0:7 - VALUE"]
    #[inline(always)]
    pub fn value(&self) -> VALUE_R {
        VALUE_R::new((self.bits & 0xff) as u8)
    }
}
#[doc = "Master Read Data Byte Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mrdatab](index.html) module"]
pub struct MRDATAB_SPEC;
impl crate::RegisterSpec for MRDATAB_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mrdatab::R](R) reader structure"]
impl crate::Readable for MRDATAB_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets MRDATAB to value 0"]
impl crate::Resettable for MRDATAB_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
