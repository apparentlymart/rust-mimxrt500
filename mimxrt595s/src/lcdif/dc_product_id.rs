#[doc = "Register `DcProductId` reader"]
pub struct R(crate::R<DC_PRODUCT_ID_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DC_PRODUCT_ID_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DC_PRODUCT_ID_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DC_PRODUCT_ID_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `PRODUCT_ID` reader - Product ID"]
pub type PRODUCT_ID_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Product ID"]
    #[inline(always)]
    pub fn product_id(&self) -> PRODUCT_ID_R {
        PRODUCT_ID_R::new(self.bits)
    }
}
#[doc = "Product ID\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dc_product_id](index.html) module"]
pub struct DC_PRODUCT_ID_SPEC;
impl crate::RegisterSpec for DC_PRODUCT_ID_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dc_product_id::R](R) reader structure"]
impl crate::Readable for DC_PRODUCT_ID_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets DcProductId to value 0x0200_0361"]
impl crate::Resettable for DC_PRODUCT_ID_SPEC {
    const RESET_VALUE: Self::Ux = 0x0200_0361;
}
