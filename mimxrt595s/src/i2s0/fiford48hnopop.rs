#[doc = "Register `FIFORD48HNOPOP` reader"]
pub struct R(crate::R<FIFORD48HNOPOP_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FIFORD48HNOPOP_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FIFORD48HNOPOP_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FIFORD48HNOPOP_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `RXDATA` reader - Received Data from the FIFO"]
pub type RXDATA_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:23 - Received Data from the FIFO"]
    #[inline(always)]
    pub fn rxdata(&self) -> RXDATA_R {
        RXDATA_R::new(self.bits & 0x00ff_ffff)
    }
}
#[doc = "FIFO Data Read for Upper Data Bits with No FIFO Pop\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fiford48hnopop](index.html) module"]
pub struct FIFORD48HNOPOP_SPEC;
impl crate::RegisterSpec for FIFORD48HNOPOP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fiford48hnopop::R](R) reader structure"]
impl crate::Readable for FIFORD48HNOPOP_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets FIFORD48HNOPOP to value 0"]
impl crate::Resettable for FIFORD48HNOPOP_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
