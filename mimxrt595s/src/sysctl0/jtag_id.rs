#[doc = "Register `JTAG_ID` reader"]
pub struct R(crate::R<JTAG_ID_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<JTAG_ID_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<JTAG_ID_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<JTAG_ID_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `FIXBIT` reader - JTAG ID fix bit."]
pub type FIXBIT_R = crate::BitReader<bool>;
#[doc = "Field `MANU` reader - JTAG ID Manufacturer"]
pub type MANU_R = crate::FieldReader<u16, u16>;
#[doc = "Field `PRODUCT_ID` reader - JTAG ID Product ID as defined in the Product ID register"]
pub type PRODUCT_ID_R = crate::FieldReader<u16, u16>;
#[doc = "Field `CHIPREV` reader - JTAG ID 4-Bit Chip Silicon Revision"]
pub type CHIPREV_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bit 0 - JTAG ID fix bit."]
    #[inline(always)]
    pub fn fixbit(&self) -> FIXBIT_R {
        FIXBIT_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:11 - JTAG ID Manufacturer"]
    #[inline(always)]
    pub fn manu(&self) -> MANU_R {
        MANU_R::new(((self.bits >> 1) & 0x07ff) as u16)
    }
    #[doc = "Bits 12:27 - JTAG ID Product ID as defined in the Product ID register"]
    #[inline(always)]
    pub fn product_id(&self) -> PRODUCT_ID_R {
        PRODUCT_ID_R::new(((self.bits >> 12) & 0xffff) as u16)
    }
    #[doc = "Bits 28:31 - JTAG ID 4-Bit Chip Silicon Revision"]
    #[inline(always)]
    pub fn chiprev(&self) -> CHIPREV_R {
        CHIPREV_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
#[doc = "JTAG ID\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [jtag_id](index.html) module"]
pub struct JTAG_ID_SPEC;
impl crate::RegisterSpec for JTAG_ID_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [jtag_id::R](R) reader structure"]
impl crate::Readable for JTAG_ID_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets JTAG_ID to value 0x2b"]
impl crate::Resettable for JTAG_ID_SPEC {
    const RESET_VALUE: Self::Ux = 0x2b;
}
