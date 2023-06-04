#[doc = "Register `DcChipPatchRev` reader"]
pub struct R(crate::R<DC_CHIP_PATCH_REV_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DC_CHIP_PATCH_REV_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DC_CHIP_PATCH_REV_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DC_CHIP_PATCH_REV_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `PATCH_REV` reader - Patch Revision"]
pub type PATCH_REV_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Patch Revision"]
    #[inline(always)]
    pub fn patch_rev(&self) -> PATCH_REV_R {
        PATCH_REV_R::new(self.bits)
    }
}
#[doc = "Patch Revision\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dc_chip_patch_rev](index.html) module"]
pub struct DC_CHIP_PATCH_REV_SPEC;
impl crate::RegisterSpec for DC_CHIP_PATCH_REV_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dc_chip_patch_rev::R](R) reader structure"]
impl crate::Readable for DC_CHIP_PATCH_REV_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets DcChipPatchRev to value 0x03"]
impl crate::Resettable for DC_CHIP_PATCH_REV_SPEC {
    const RESET_VALUE: Self::Ux = 0x03;
}
