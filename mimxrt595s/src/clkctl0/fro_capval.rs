#[doc = "Register `FRO_CAPVAL` reader"]
pub struct R(crate::R<FRO_CAPVAL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FRO_CAPVAL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FRO_CAPVAL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FRO_CAPVAL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `CAPVAL` reader - Captured Value"]
pub type CAPVAL_R = crate::FieldReader<u16, u16>;
#[doc = "Field `DATA_VALID` reader - Data Valid"]
pub type DATA_VALID_R = crate::BitReader<DATA_VALID_A>;
#[doc = "Data Valid\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DATA_VALID_A {
    #[doc = "0: CAPVAL data is not valid"]
    DATA_NOT_VALID = 0,
    #[doc = "1: CAPVAL data is valid"]
    DATA_VALID = 1,
}
impl From<DATA_VALID_A> for bool {
    #[inline(always)]
    fn from(variant: DATA_VALID_A) -> Self {
        variant as u8 != 0
    }
}
impl DATA_VALID_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DATA_VALID_A {
        match self.bits {
            false => DATA_VALID_A::DATA_NOT_VALID,
            true => DATA_VALID_A::DATA_VALID,
        }
    }
    #[doc = "Checks if the value of the field is `DATA_NOT_VALID`"]
    #[inline(always)]
    pub fn is_data_not_valid(&self) -> bool {
        *self == DATA_VALID_A::DATA_NOT_VALID
    }
    #[doc = "Checks if the value of the field is `DATA_VALID`"]
    #[inline(always)]
    pub fn is_data_valid(&self) -> bool {
        *self == DATA_VALID_A::DATA_VALID
    }
}
impl R {
    #[doc = "Bits 0:15 - Captured Value"]
    #[inline(always)]
    pub fn capval(&self) -> CAPVAL_R {
        CAPVAL_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bit 31 - Data Valid"]
    #[inline(always)]
    pub fn data_valid(&self) -> DATA_VALID_R {
        DATA_VALID_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[doc = "Free Running Oscillator Captured Value\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fro_capval](index.html) module"]
pub struct FRO_CAPVAL_SPEC;
impl crate::RegisterSpec for FRO_CAPVAL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fro_capval::R](R) reader structure"]
impl crate::Readable for FRO_CAPVAL_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets FRO_CAPVAL to value 0"]
impl crate::Resettable for FRO_CAPVAL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
