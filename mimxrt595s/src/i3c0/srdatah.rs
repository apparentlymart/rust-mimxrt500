#[doc = "Register `SRDATAH` reader"]
pub struct R(crate::R<SRDATAH_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SRDATAH_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SRDATAH_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SRDATAH_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `LSB` reader - The 1st byte read from the slave"]
pub type LSB_R = crate::FieldReader<u8, u8>;
#[doc = "Field `MSB` reader - The 2nd byte read from the slave"]
pub type MSB_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 0:7 - The 1st byte read from the slave"]
    #[inline(always)]
    pub fn lsb(&self) -> LSB_R {
        LSB_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - The 2nd byte read from the slave"]
    #[inline(always)]
    pub fn msb(&self) -> MSB_R {
        MSB_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
#[doc = "Slave Read Data Half-word Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [srdatah](index.html) module"]
pub struct SRDATAH_SPEC;
impl crate::RegisterSpec for SRDATAH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [srdatah::R](R) reader structure"]
impl crate::Readable for SRDATAH_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets SRDATAH to value 0"]
impl crate::Resettable for SRDATAH_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
