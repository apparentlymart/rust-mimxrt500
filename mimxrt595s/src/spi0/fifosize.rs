#[doc = "Register `FIFOSIZE` reader"]
pub struct R(crate::R<FIFOSIZE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FIFOSIZE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FIFOSIZE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FIFOSIZE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `FIFOSIZE` reader - FIFO Size"]
pub type FIFOSIZE_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 0:4 - FIFO Size"]
    #[inline(always)]
    pub fn fifosize(&self) -> FIFOSIZE_R {
        FIFOSIZE_R::new((self.bits & 0x1f) as u8)
    }
}
#[doc = "FIFO Size Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fifosize](index.html) module"]
pub struct FIFOSIZE_SPEC;
impl crate::RegisterSpec for FIFOSIZE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fifosize::R](R) reader structure"]
impl crate::Readable for FIFOSIZE_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets FIFOSIZE to value 0"]
impl crate::Resettable for FIFOSIZE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
