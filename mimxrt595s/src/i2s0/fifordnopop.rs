#[doc = "Register `FIFORDNOPOP` reader"]
pub struct R(crate::R<FIFORDNOPOP_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FIFORDNOPOP_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FIFORDNOPOP_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FIFORDNOPOP_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `RXDATA` reader - Received Data from the FIFO"]
pub type RXDATA_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Received Data from the FIFO"]
    #[inline(always)]
    pub fn rxdata(&self) -> RXDATA_R {
        RXDATA_R::new(self.bits)
    }
}
#[doc = "FIFO Data Read with No FIFO Pop\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fifordnopop](index.html) module"]
pub struct FIFORDNOPOP_SPEC;
impl crate::RegisterSpec for FIFORDNOPOP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fifordnopop::R](R) reader structure"]
impl crate::Readable for FIFORDNOPOP_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets FIFORDNOPOP to value 0"]
impl crate::Resettable for FIFORDNOPOP_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
