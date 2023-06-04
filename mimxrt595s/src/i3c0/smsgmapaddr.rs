#[doc = "Register `SMSGMAPADDR` reader"]
pub struct R(crate::R<SMSGMAPADDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SMSGMAPADDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SMSGMAPADDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SMSGMAPADDR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `MAPLAST` reader - Matched address index"]
pub type MAPLAST_R = crate::FieldReader<u8, u8>;
#[doc = "Field `MAPLASTM1` reader - Previous match index 1"]
pub type MAPLASTM1_R = crate::FieldReader<u8, u8>;
#[doc = "Field `MAPLASTM2` reader - Previous match index 2"]
pub type MAPLASTM2_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 0:3 - Matched address index"]
    #[inline(always)]
    pub fn maplast(&self) -> MAPLAST_R {
        MAPLAST_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - Previous match index 1"]
    #[inline(always)]
    pub fn maplastm1(&self) -> MAPLASTM1_R {
        MAPLASTM1_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - Previous match index 2"]
    #[inline(always)]
    pub fn maplastm2(&self) -> MAPLASTM2_R {
        MAPLASTM2_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
}
#[doc = "Slave Message-Mapped Address Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [smsgmapaddr](index.html) module"]
pub struct SMSGMAPADDR_SPEC;
impl crate::RegisterSpec for SMSGMAPADDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [smsgmapaddr::R](R) reader structure"]
impl crate::Readable for SMSGMAPADDR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets SMSGMAPADDR to value 0x0214"]
impl crate::Resettable for SMSGMAPADDR_SPEC {
    const RESET_VALUE: Self::Ux = 0x0214;
}
