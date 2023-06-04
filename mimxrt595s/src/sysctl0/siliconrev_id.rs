#[doc = "Register `SILICONREV_ID` reader"]
pub struct R(crate::R<SILICONREV_ID_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SILICONREV_ID_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SILICONREV_ID_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SILICONREV_ID_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `MINOR` reader - MINOR"]
pub type MINOR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `MAJOR` reader - MAJOR"]
pub type MAJOR_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 0:3 - MINOR"]
    #[inline(always)]
    pub fn minor(&self) -> MINOR_R {
        MINOR_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - MAJOR"]
    #[inline(always)]
    pub fn major(&self) -> MAJOR_R {
        MAJOR_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
}
#[doc = "Silicon Revision ID\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [siliconrev_id](index.html) module"]
pub struct SILICONREV_ID_SPEC;
impl crate::RegisterSpec for SILICONREV_ID_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [siliconrev_id::R](R) reader structure"]
impl crate::Readable for SILICONREV_ID_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets SILICONREV_ID to value 0x000b_000a"]
impl crate::Resettable for SILICONREV_ID_SPEC {
    const RESET_VALUE: Self::Ux = 0x000b_000a;
}
