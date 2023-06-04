#[doc = "Register `MRDATAH` reader"]
pub struct R(crate::R<MRDATAH_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MRDATAH_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MRDATAH_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MRDATAH_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `LSB` reader - LSB"]
pub type LSB_R = crate::FieldReader<u8, u8>;
#[doc = "Field `MSB` reader - MSB"]
pub type MSB_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 0:7 - LSB"]
    #[inline(always)]
    pub fn lsb(&self) -> LSB_R {
        LSB_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - MSB"]
    #[inline(always)]
    pub fn msb(&self) -> MSB_R {
        MSB_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
#[doc = "Master Read Data Half-word Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mrdatah](index.html) module"]
pub struct MRDATAH_SPEC;
impl crate::RegisterSpec for MRDATAH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mrdatah::R](R) reader structure"]
impl crate::Readable for MRDATAH_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets MRDATAH to value 0"]
impl crate::Resettable for MRDATAH_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
