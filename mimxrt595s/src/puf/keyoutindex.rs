#[doc = "Register `KEYOUTINDEX` reader"]
pub struct R(crate::R<KEYOUTINDEX_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<KEYOUTINDEX_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<KEYOUTINDEX_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<KEYOUTINDEX_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `KEYOUTIDX` reader - Key Output Index"]
pub type KEYOUTIDX_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 0:3 - Key Output Index"]
    #[inline(always)]
    pub fn keyoutidx(&self) -> KEYOUTIDX_R {
        KEYOUTIDX_R::new((self.bits & 0x0f) as u8)
    }
}
#[doc = "PUF Key Output Index\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [keyoutindex](index.html) module"]
pub struct KEYOUTINDEX_SPEC;
impl crate::RegisterSpec for KEYOUTINDEX_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [keyoutindex::R](R) reader structure"]
impl crate::Readable for KEYOUTINDEX_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets KEYOUTINDEX to value 0"]
impl crate::Resettable for KEYOUTINDEX_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
