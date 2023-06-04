#[doc = "Register `SEC_VIO_ADDR[%s]` reader"]
pub struct R(crate::R<SEC_VIO_ADDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SEC_VIO_ADDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SEC_VIO_ADDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SEC_VIO_ADDR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `SEC_VIO_INFO_WRITE` reader - Security violation access read/write indicator"]
pub type SEC_VIO_INFO_WRITE_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Security violation access read/write indicator"]
    #[inline(always)]
    pub fn sec_vio_info_write(&self) -> SEC_VIO_INFO_WRITE_R {
        SEC_VIO_INFO_WRITE_R::new(self.bits)
    }
}
#[doc = "Security Violation Address(n) Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sec_vio_addr](index.html) module"]
pub struct SEC_VIO_ADDR_SPEC;
impl crate::RegisterSpec for SEC_VIO_ADDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sec_vio_addr::R](R) reader structure"]
impl crate::Readable for SEC_VIO_ADDR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets SEC_VIO_ADDR[%s]
to value 0"]
impl crate::Resettable for SEC_VIO_ADDR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
