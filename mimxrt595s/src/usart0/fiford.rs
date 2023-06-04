#[doc = "Register `FIFORD` reader"]
pub struct R(crate::R<FIFORD_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FIFORD_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FIFORD_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FIFORD_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `RXDATA` reader - Received Data from the FIFO"]
pub type RXDATA_R = crate::FieldReader<u16, u16>;
#[doc = "Field `FRAMERR` reader - Framing Error Status Flag"]
pub type FRAMERR_R = crate::BitReader<bool>;
#[doc = "Field `PARITYERR` reader - Parity Error Status Flag"]
pub type PARITYERR_R = crate::BitReader<bool>;
#[doc = "Field `RXNOISE` reader - Received Noise Flag"]
pub type RXNOISE_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bits 0:8 - Received Data from the FIFO"]
    #[inline(always)]
    pub fn rxdata(&self) -> RXDATA_R {
        RXDATA_R::new((self.bits & 0x01ff) as u16)
    }
    #[doc = "Bit 13 - Framing Error Status Flag"]
    #[inline(always)]
    pub fn framerr(&self) -> FRAMERR_R {
        FRAMERR_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Parity Error Status Flag"]
    #[inline(always)]
    pub fn parityerr(&self) -> PARITYERR_R {
        PARITYERR_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Received Noise Flag"]
    #[inline(always)]
    pub fn rxnoise(&self) -> RXNOISE_R {
        RXNOISE_R::new(((self.bits >> 15) & 1) != 0)
    }
}
#[doc = "FIFO Read Data\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fiford](index.html) module"]
pub struct FIFORD_SPEC;
impl crate::RegisterSpec for FIFORD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fiford::R](R) reader structure"]
impl crate::Readable for FIFORD_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets FIFORD to value 0"]
impl crate::Resettable for FIFORD_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
