#[doc = "Register `STS2` reader"]
pub struct R(crate::R<STS2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<STS2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<STS2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<STS2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `ASLVLOCK` reader - Flash A sample clock slave delay line locked."]
pub type ASLVLOCK_R = crate::BitReader<ASLVLOCK_A>;
#[doc = "Flash A sample clock slave delay line locked.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ASLVLOCK_A {
    #[doc = "0: Flash A sample clock slave delay line is not locked"]
    VAL0 = 0,
    #[doc = "1: Flash A sample clock slave delay line is locked"]
    VAL1 = 1,
}
impl From<ASLVLOCK_A> for bool {
    #[inline(always)]
    fn from(variant: ASLVLOCK_A) -> Self {
        variant as u8 != 0
    }
}
impl ASLVLOCK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ASLVLOCK_A {
        match self.bits {
            false => ASLVLOCK_A::VAL0,
            true => ASLVLOCK_A::VAL1,
        }
    }
    #[doc = "Checks if the value of the field is `VAL0`"]
    #[inline(always)]
    pub fn is_val0(&self) -> bool {
        *self == ASLVLOCK_A::VAL0
    }
    #[doc = "Checks if the value of the field is `VAL1`"]
    #[inline(always)]
    pub fn is_val1(&self) -> bool {
        *self == ASLVLOCK_A::VAL1
    }
}
#[doc = "Field `AREFLOCK` reader - Flash A sample clock reference delay line locked."]
pub type AREFLOCK_R = crate::BitReader<AREFLOCK_A>;
#[doc = "Flash A sample clock reference delay line locked.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AREFLOCK_A {
    #[doc = "0: Flash A sample clock reference delay line is not locked"]
    VAL0 = 0,
    #[doc = "1: Flash A sample clock reference delay line is locked"]
    VAL1 = 1,
}
impl From<AREFLOCK_A> for bool {
    #[inline(always)]
    fn from(variant: AREFLOCK_A) -> Self {
        variant as u8 != 0
    }
}
impl AREFLOCK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AREFLOCK_A {
        match self.bits {
            false => AREFLOCK_A::VAL0,
            true => AREFLOCK_A::VAL1,
        }
    }
    #[doc = "Checks if the value of the field is `VAL0`"]
    #[inline(always)]
    pub fn is_val0(&self) -> bool {
        *self == AREFLOCK_A::VAL0
    }
    #[doc = "Checks if the value of the field is `VAL1`"]
    #[inline(always)]
    pub fn is_val1(&self) -> bool {
        *self == AREFLOCK_A::VAL1
    }
}
#[doc = "Field `ASLVSEL` reader - Flash A sample clock slave delay line delay cell number selection ."]
pub type ASLVSEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `AREFSEL` reader - Flash A sample clock reference delay line delay cell number selection."]
pub type AREFSEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `BSLVLOCK` reader - Flash B sample clock slave delay line locked."]
pub type BSLVLOCK_R = crate::BitReader<BSLVLOCK_A>;
#[doc = "Flash B sample clock slave delay line locked.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BSLVLOCK_A {
    #[doc = "0: Flash B sample clock slave delay line is not locked."]
    VAL0 = 0,
    #[doc = "1: Flash B sample clock slave delay line is locked."]
    VAL1 = 1,
}
impl From<BSLVLOCK_A> for bool {
    #[inline(always)]
    fn from(variant: BSLVLOCK_A) -> Self {
        variant as u8 != 0
    }
}
impl BSLVLOCK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BSLVLOCK_A {
        match self.bits {
            false => BSLVLOCK_A::VAL0,
            true => BSLVLOCK_A::VAL1,
        }
    }
    #[doc = "Checks if the value of the field is `VAL0`"]
    #[inline(always)]
    pub fn is_val0(&self) -> bool {
        *self == BSLVLOCK_A::VAL0
    }
    #[doc = "Checks if the value of the field is `VAL1`"]
    #[inline(always)]
    pub fn is_val1(&self) -> bool {
        *self == BSLVLOCK_A::VAL1
    }
}
#[doc = "Field `BREFLOCK` reader - Flash B sample clock reference delay line locked."]
pub type BREFLOCK_R = crate::BitReader<BREFLOCK_A>;
#[doc = "Flash B sample clock reference delay line locked.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BREFLOCK_A {
    #[doc = "0: Flash B sample clock reference delay line is not locked."]
    VAL0 = 0,
    #[doc = "1: Flash B sample clock reference delay line is locked."]
    VAL1 = 1,
}
impl From<BREFLOCK_A> for bool {
    #[inline(always)]
    fn from(variant: BREFLOCK_A) -> Self {
        variant as u8 != 0
    }
}
impl BREFLOCK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BREFLOCK_A {
        match self.bits {
            false => BREFLOCK_A::VAL0,
            true => BREFLOCK_A::VAL1,
        }
    }
    #[doc = "Checks if the value of the field is `VAL0`"]
    #[inline(always)]
    pub fn is_val0(&self) -> bool {
        *self == BREFLOCK_A::VAL0
    }
    #[doc = "Checks if the value of the field is `VAL1`"]
    #[inline(always)]
    pub fn is_val1(&self) -> bool {
        *self == BREFLOCK_A::VAL1
    }
}
#[doc = "Field `BSLVSEL` reader - Flash B sample clock slave delay line delay cell number selection."]
pub type BSLVSEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `BREFSEL` reader - Flash B sample clock reference delay line delay cell number selection."]
pub type BREFSEL_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bit 0 - Flash A sample clock slave delay line locked."]
    #[inline(always)]
    pub fn aslvlock(&self) -> ASLVLOCK_R {
        ASLVLOCK_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Flash A sample clock reference delay line locked."]
    #[inline(always)]
    pub fn areflock(&self) -> AREFLOCK_R {
        AREFLOCK_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:7 - Flash A sample clock slave delay line delay cell number selection ."]
    #[inline(always)]
    pub fn aslvsel(&self) -> ASLVSEL_R {
        ASLVSEL_R::new(((self.bits >> 2) & 0x3f) as u8)
    }
    #[doc = "Bits 8:13 - Flash A sample clock reference delay line delay cell number selection."]
    #[inline(always)]
    pub fn arefsel(&self) -> AREFSEL_R {
        AREFSEL_R::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bit 16 - Flash B sample clock slave delay line locked."]
    #[inline(always)]
    pub fn bslvlock(&self) -> BSLVLOCK_R {
        BSLVLOCK_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Flash B sample clock reference delay line locked."]
    #[inline(always)]
    pub fn breflock(&self) -> BREFLOCK_R {
        BREFLOCK_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bits 18:23 - Flash B sample clock slave delay line delay cell number selection."]
    #[inline(always)]
    pub fn bslvsel(&self) -> BSLVSEL_R {
        BSLVSEL_R::new(((self.bits >> 18) & 0x3f) as u8)
    }
    #[doc = "Bits 24:29 - Flash B sample clock reference delay line delay cell number selection."]
    #[inline(always)]
    pub fn brefsel(&self) -> BREFSEL_R {
        BREFSEL_R::new(((self.bits >> 24) & 0x3f) as u8)
    }
}
#[doc = "Status Register 2\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sts2](index.html) module"]
pub struct STS2_SPEC;
impl crate::RegisterSpec for STS2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sts2::R](R) reader structure"]
impl crate::Readable for STS2_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets STS2 to value 0x0100_0100"]
impl crate::Resettable for STS2_SPEC {
    const RESET_VALUE: Self::Ux = 0x0100_0100;
}
