#[doc = "Register `SRDATAB` reader"]
pub struct R(crate::R<SRDATAB_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SRDATAB_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SRDATAB_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SRDATAB_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `DATA0` reader - Byte read from the master"]
pub type DATA0_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 0:7 - Byte read from the master"]
    #[inline(always)]
    pub fn data0(&self) -> DATA0_R {
        DATA0_R::new((self.bits & 0xff) as u8)
    }
}
#[doc = "Slave Read Data Byte Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [srdatab](index.html) module"]
pub struct SRDATAB_SPEC;
impl crate::RegisterSpec for SRDATAB_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [srdatab::R](R) reader structure"]
impl crate::Readable for SRDATAB_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets SRDATAB to value 0"]
impl crate::Resettable for SRDATAB_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
