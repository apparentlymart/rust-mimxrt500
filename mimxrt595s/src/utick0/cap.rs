#[doc = "Register `CAP[%s]` reader"]
pub struct R(crate::R<CAP_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CAP_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CAP_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CAP_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `CAP_VALUE` reader - Captured value for the related capture event"]
pub type CAP_VALUE_R = crate::FieldReader<u32, u32>;
#[doc = "Field `VALID` reader - Captured value is valid"]
pub type VALID_R = crate::BitReader<VALID_A>;
#[doc = "Captured value is valid\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VALID_A {
    #[doc = "0: A valid value has been not been captured"]
    NOTVALID = 0,
    #[doc = "1: A valid value has been captured, based on a transition of the related UTICK_CAPn pin"]
    VALID = 1,
}
impl From<VALID_A> for bool {
    #[inline(always)]
    fn from(variant: VALID_A) -> Self {
        variant as u8 != 0
    }
}
impl VALID_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> VALID_A {
        match self.bits {
            false => VALID_A::NOTVALID,
            true => VALID_A::VALID,
        }
    }
    #[doc = "Checks if the value of the field is `NOTVALID`"]
    #[inline(always)]
    pub fn is_notvalid(&self) -> bool {
        *self == VALID_A::NOTVALID
    }
    #[doc = "Checks if the value of the field is `VALID`"]
    #[inline(always)]
    pub fn is_valid(&self) -> bool {
        *self == VALID_A::VALID
    }
}
impl R {
    #[doc = "Bits 0:30 - Captured value for the related capture event"]
    #[inline(always)]
    pub fn cap_value(&self) -> CAP_VALUE_R {
        CAP_VALUE_R::new(self.bits & 0x7fff_ffff)
    }
    #[doc = "Bit 31 - Captured value is valid"]
    #[inline(always)]
    pub fn valid(&self) -> VALID_R {
        VALID_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[doc = "Capture\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cap](index.html) module"]
pub struct CAP_SPEC;
impl crate::RegisterSpec for CAP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cap::R](R) reader structure"]
impl crate::Readable for CAP_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets CAP[%s]
to value 0"]
impl crate::Resettable for CAP_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
