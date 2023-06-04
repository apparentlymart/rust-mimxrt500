#[doc = "Register `FIFO_DATA` reader"]
pub struct R(crate::R<FIFO_DATA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FIFO_DATA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FIFO_DATA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FIFO_DATA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `DATA` reader - PCM Data"]
pub type DATA_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:23 - PCM Data"]
    #[inline(always)]
    pub fn data(&self) -> DATA_R {
        DATA_R::new(self.bits & 0x00ff_ffff)
    }
}
#[doc = "FIFO Data\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fifo_data](index.html) module"]
pub struct FIFO_DATA_SPEC;
impl crate::RegisterSpec for FIFO_DATA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fifo_data::R](R) reader structure"]
impl crate::Readable for FIFO_DATA_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets FIFO_DATA to value 0"]
impl crate::Resettable for FIFO_DATA_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
