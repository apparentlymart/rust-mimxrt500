#[doc = "Register `DcChipDate` reader"]
pub struct R(crate::R<DC_CHIP_DATE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DC_CHIP_DATE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DC_CHIP_DATE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DC_CHIP_DATE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `DATE` reader - Date"]
pub type DATE_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Date"]
    #[inline(always)]
    pub fn date(&self) -> DATE_R {
        DATE_R::new(self.bits)
    }
}
#[doc = "Shows the release date for the IP in YYYYMMDD (year, month)\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dc_chip_date](index.html) module"]
pub struct DC_CHIP_DATE_SPEC;
impl crate::RegisterSpec for DC_CHIP_DATE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dc_chip_date::R](R) reader structure"]
impl crate::Readable for DC_CHIP_DATE_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets DcChipDate to value 0x2018_0612"]
impl crate::Resettable for DC_CHIP_DATE_SPEC {
    const RESET_VALUE: Self::Ux = 0x2018_0612;
}
