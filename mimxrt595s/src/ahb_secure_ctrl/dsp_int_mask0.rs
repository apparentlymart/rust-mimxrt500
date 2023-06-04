#[doc = "Register `DSP_INT_MASK0` reader"]
pub struct R(crate::R<DSP_INT_MASK0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DSP_INT_MASK0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DSP_INT_MASK0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DSP_INT_MASK0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DSP_INT_MASK0` writer"]
pub struct W(crate::W<DSP_INT_MASK0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DSP_INT_MASK0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<DSP_INT_MASK0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DSP_INT_MASK0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PMUX_OUT0` reader - Mask bit"]
pub type PMUX_OUT0_R = crate::BitReader<PMUX_OUT0_A>;
#[doc = "Mask bit\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PMUX_OUT0_A {
    #[doc = "0: Masked"]
    MASKED = 0,
    #[doc = "1: Not masked"]
    NOT_MASKED = 1,
}
impl From<PMUX_OUT0_A> for bool {
    #[inline(always)]
    fn from(variant: PMUX_OUT0_A) -> Self {
        variant as u8 != 0
    }
}
impl PMUX_OUT0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PMUX_OUT0_A {
        match self.bits {
            false => PMUX_OUT0_A::MASKED,
            true => PMUX_OUT0_A::NOT_MASKED,
        }
    }
    #[doc = "Checks if the value of the field is `MASKED`"]
    #[inline(always)]
    pub fn is_masked(&self) -> bool {
        *self == PMUX_OUT0_A::MASKED
    }
    #[doc = "Checks if the value of the field is `NOT_MASKED`"]
    #[inline(always)]
    pub fn is_not_masked(&self) -> bool {
        *self == PMUX_OUT0_A::NOT_MASKED
    }
}
#[doc = "Field `PMUX_OUT0` writer - Mask bit"]
pub type PMUX_OUT0_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, DSP_INT_MASK0_SPEC, PMUX_OUT0_A, O>;
impl<'a, const O: u8> PMUX_OUT0_W<'a, O> {
    #[doc = "Masked"]
    #[inline(always)]
    pub fn masked(self) -> &'a mut W {
        self.variant(PMUX_OUT0_A::MASKED)
    }
    #[doc = "Not masked"]
    #[inline(always)]
    pub fn not_masked(self) -> &'a mut W {
        self.variant(PMUX_OUT0_A::NOT_MASKED)
    }
}
#[doc = "Field `PMUX_OUT1` reader - Mask bit"]
pub type PMUX_OUT1_R = crate::BitReader<PMUX_OUT1_A>;
#[doc = "Mask bit\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PMUX_OUT1_A {
    #[doc = "0: Masked"]
    MASKED = 0,
    #[doc = "1: Not masked"]
    NOT_MASKED = 1,
}
impl From<PMUX_OUT1_A> for bool {
    #[inline(always)]
    fn from(variant: PMUX_OUT1_A) -> Self {
        variant as u8 != 0
    }
}
impl PMUX_OUT1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PMUX_OUT1_A {
        match self.bits {
            false => PMUX_OUT1_A::MASKED,
            true => PMUX_OUT1_A::NOT_MASKED,
        }
    }
    #[doc = "Checks if the value of the field is `MASKED`"]
    #[inline(always)]
    pub fn is_masked(&self) -> bool {
        *self == PMUX_OUT1_A::MASKED
    }
    #[doc = "Checks if the value of the field is `NOT_MASKED`"]
    #[inline(always)]
    pub fn is_not_masked(&self) -> bool {
        *self == PMUX_OUT1_A::NOT_MASKED
    }
}
#[doc = "Field `PMUX_OUT1` writer - Mask bit"]
pub type PMUX_OUT1_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, DSP_INT_MASK0_SPEC, PMUX_OUT1_A, O>;
impl<'a, const O: u8> PMUX_OUT1_W<'a, O> {
    #[doc = "Masked"]
    #[inline(always)]
    pub fn masked(self) -> &'a mut W {
        self.variant(PMUX_OUT1_A::MASKED)
    }
    #[doc = "Not masked"]
    #[inline(always)]
    pub fn not_masked(self) -> &'a mut W {
        self.variant(PMUX_OUT1_A::NOT_MASKED)
    }
}
#[doc = "Field `PMUX_OUT2` reader - Mask bit"]
pub type PMUX_OUT2_R = crate::BitReader<PMUX_OUT2_A>;
#[doc = "Mask bit\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PMUX_OUT2_A {
    #[doc = "0: Masked"]
    MASKED = 0,
    #[doc = "1: Not masked"]
    NOT_MASKED = 1,
}
impl From<PMUX_OUT2_A> for bool {
    #[inline(always)]
    fn from(variant: PMUX_OUT2_A) -> Self {
        variant as u8 != 0
    }
}
impl PMUX_OUT2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PMUX_OUT2_A {
        match self.bits {
            false => PMUX_OUT2_A::MASKED,
            true => PMUX_OUT2_A::NOT_MASKED,
        }
    }
    #[doc = "Checks if the value of the field is `MASKED`"]
    #[inline(always)]
    pub fn is_masked(&self) -> bool {
        *self == PMUX_OUT2_A::MASKED
    }
    #[doc = "Checks if the value of the field is `NOT_MASKED`"]
    #[inline(always)]
    pub fn is_not_masked(&self) -> bool {
        *self == PMUX_OUT2_A::NOT_MASKED
    }
}
#[doc = "Field `PMUX_OUT2` writer - Mask bit"]
pub type PMUX_OUT2_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, DSP_INT_MASK0_SPEC, PMUX_OUT2_A, O>;
impl<'a, const O: u8> PMUX_OUT2_W<'a, O> {
    #[doc = "Masked"]
    #[inline(always)]
    pub fn masked(self) -> &'a mut W {
        self.variant(PMUX_OUT2_A::MASKED)
    }
    #[doc = "Not masked"]
    #[inline(always)]
    pub fn not_masked(self) -> &'a mut W {
        self.variant(PMUX_OUT2_A::NOT_MASKED)
    }
}
#[doc = "Field `PMUX_OUT3` reader - Mask bit"]
pub type PMUX_OUT3_R = crate::BitReader<PMUX_OUT3_A>;
#[doc = "Mask bit\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PMUX_OUT3_A {
    #[doc = "0: Masked"]
    MASKED = 0,
    #[doc = "1: Not masked"]
    NOT_MASKED = 1,
}
impl From<PMUX_OUT3_A> for bool {
    #[inline(always)]
    fn from(variant: PMUX_OUT3_A) -> Self {
        variant as u8 != 0
    }
}
impl PMUX_OUT3_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PMUX_OUT3_A {
        match self.bits {
            false => PMUX_OUT3_A::MASKED,
            true => PMUX_OUT3_A::NOT_MASKED,
        }
    }
    #[doc = "Checks if the value of the field is `MASKED`"]
    #[inline(always)]
    pub fn is_masked(&self) -> bool {
        *self == PMUX_OUT3_A::MASKED
    }
    #[doc = "Checks if the value of the field is `NOT_MASKED`"]
    #[inline(always)]
    pub fn is_not_masked(&self) -> bool {
        *self == PMUX_OUT3_A::NOT_MASKED
    }
}
#[doc = "Field `PMUX_OUT3` writer - Mask bit"]
pub type PMUX_OUT3_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, DSP_INT_MASK0_SPEC, PMUX_OUT3_A, O>;
impl<'a, const O: u8> PMUX_OUT3_W<'a, O> {
    #[doc = "Masked"]
    #[inline(always)]
    pub fn masked(self) -> &'a mut W {
        self.variant(PMUX_OUT3_A::MASKED)
    }
    #[doc = "Not masked"]
    #[inline(always)]
    pub fn not_masked(self) -> &'a mut W {
        self.variant(PMUX_OUT3_A::NOT_MASKED)
    }
}
#[doc = "Field `PMUX_OUT4` reader - Mask bit"]
pub type PMUX_OUT4_R = crate::BitReader<PMUX_OUT4_A>;
#[doc = "Mask bit\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PMUX_OUT4_A {
    #[doc = "0: Masked"]
    MASKED = 0,
    #[doc = "1: Not masked"]
    NOT_MASKED = 1,
}
impl From<PMUX_OUT4_A> for bool {
    #[inline(always)]
    fn from(variant: PMUX_OUT4_A) -> Self {
        variant as u8 != 0
    }
}
impl PMUX_OUT4_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PMUX_OUT4_A {
        match self.bits {
            false => PMUX_OUT4_A::MASKED,
            true => PMUX_OUT4_A::NOT_MASKED,
        }
    }
    #[doc = "Checks if the value of the field is `MASKED`"]
    #[inline(always)]
    pub fn is_masked(&self) -> bool {
        *self == PMUX_OUT4_A::MASKED
    }
    #[doc = "Checks if the value of the field is `NOT_MASKED`"]
    #[inline(always)]
    pub fn is_not_masked(&self) -> bool {
        *self == PMUX_OUT4_A::NOT_MASKED
    }
}
#[doc = "Field `PMUX_OUT4` writer - Mask bit"]
pub type PMUX_OUT4_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, DSP_INT_MASK0_SPEC, PMUX_OUT4_A, O>;
impl<'a, const O: u8> PMUX_OUT4_W<'a, O> {
    #[doc = "Masked"]
    #[inline(always)]
    pub fn masked(self) -> &'a mut W {
        self.variant(PMUX_OUT4_A::MASKED)
    }
    #[doc = "Not masked"]
    #[inline(always)]
    pub fn not_masked(self) -> &'a mut W {
        self.variant(PMUX_OUT4_A::NOT_MASKED)
    }
}
#[doc = "Field `PMUX_OUT5` reader - Mask bit"]
pub type PMUX_OUT5_R = crate::BitReader<PMUX_OUT5_A>;
#[doc = "Mask bit\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PMUX_OUT5_A {
    #[doc = "0: Masked"]
    MASKED = 0,
    #[doc = "1: Not masked"]
    NOT_MASKED = 1,
}
impl From<PMUX_OUT5_A> for bool {
    #[inline(always)]
    fn from(variant: PMUX_OUT5_A) -> Self {
        variant as u8 != 0
    }
}
impl PMUX_OUT5_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PMUX_OUT5_A {
        match self.bits {
            false => PMUX_OUT5_A::MASKED,
            true => PMUX_OUT5_A::NOT_MASKED,
        }
    }
    #[doc = "Checks if the value of the field is `MASKED`"]
    #[inline(always)]
    pub fn is_masked(&self) -> bool {
        *self == PMUX_OUT5_A::MASKED
    }
    #[doc = "Checks if the value of the field is `NOT_MASKED`"]
    #[inline(always)]
    pub fn is_not_masked(&self) -> bool {
        *self == PMUX_OUT5_A::NOT_MASKED
    }
}
#[doc = "Field `PMUX_OUT5` writer - Mask bit"]
pub type PMUX_OUT5_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, DSP_INT_MASK0_SPEC, PMUX_OUT5_A, O>;
impl<'a, const O: u8> PMUX_OUT5_W<'a, O> {
    #[doc = "Masked"]
    #[inline(always)]
    pub fn masked(self) -> &'a mut W {
        self.variant(PMUX_OUT5_A::MASKED)
    }
    #[doc = "Not masked"]
    #[inline(always)]
    pub fn not_masked(self) -> &'a mut W {
        self.variant(PMUX_OUT5_A::NOT_MASKED)
    }
}
#[doc = "Field `PMUX_OUT6` reader - Mask bit"]
pub type PMUX_OUT6_R = crate::BitReader<PMUX_OUT6_A>;
#[doc = "Mask bit\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PMUX_OUT6_A {
    #[doc = "0: Masked"]
    MASKED = 0,
    #[doc = "1: Not masked"]
    NOT_MASKED = 1,
}
impl From<PMUX_OUT6_A> for bool {
    #[inline(always)]
    fn from(variant: PMUX_OUT6_A) -> Self {
        variant as u8 != 0
    }
}
impl PMUX_OUT6_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PMUX_OUT6_A {
        match self.bits {
            false => PMUX_OUT6_A::MASKED,
            true => PMUX_OUT6_A::NOT_MASKED,
        }
    }
    #[doc = "Checks if the value of the field is `MASKED`"]
    #[inline(always)]
    pub fn is_masked(&self) -> bool {
        *self == PMUX_OUT6_A::MASKED
    }
    #[doc = "Checks if the value of the field is `NOT_MASKED`"]
    #[inline(always)]
    pub fn is_not_masked(&self) -> bool {
        *self == PMUX_OUT6_A::NOT_MASKED
    }
}
#[doc = "Field `PMUX_OUT6` writer - Mask bit"]
pub type PMUX_OUT6_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, DSP_INT_MASK0_SPEC, PMUX_OUT6_A, O>;
impl<'a, const O: u8> PMUX_OUT6_W<'a, O> {
    #[doc = "Masked"]
    #[inline(always)]
    pub fn masked(self) -> &'a mut W {
        self.variant(PMUX_OUT6_A::MASKED)
    }
    #[doc = "Not masked"]
    #[inline(always)]
    pub fn not_masked(self) -> &'a mut W {
        self.variant(PMUX_OUT6_A::NOT_MASKED)
    }
}
#[doc = "Field `PMUX_OUT7` reader - Mask bit"]
pub type PMUX_OUT7_R = crate::BitReader<PMUX_OUT7_A>;
#[doc = "Mask bit\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PMUX_OUT7_A {
    #[doc = "0: Masked"]
    MASKED = 0,
    #[doc = "1: Not masked"]
    NOT_MASKED = 1,
}
impl From<PMUX_OUT7_A> for bool {
    #[inline(always)]
    fn from(variant: PMUX_OUT7_A) -> Self {
        variant as u8 != 0
    }
}
impl PMUX_OUT7_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PMUX_OUT7_A {
        match self.bits {
            false => PMUX_OUT7_A::MASKED,
            true => PMUX_OUT7_A::NOT_MASKED,
        }
    }
    #[doc = "Checks if the value of the field is `MASKED`"]
    #[inline(always)]
    pub fn is_masked(&self) -> bool {
        *self == PMUX_OUT7_A::MASKED
    }
    #[doc = "Checks if the value of the field is `NOT_MASKED`"]
    #[inline(always)]
    pub fn is_not_masked(&self) -> bool {
        *self == PMUX_OUT7_A::NOT_MASKED
    }
}
#[doc = "Field `PMUX_OUT7` writer - Mask bit"]
pub type PMUX_OUT7_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, DSP_INT_MASK0_SPEC, PMUX_OUT7_A, O>;
impl<'a, const O: u8> PMUX_OUT7_W<'a, O> {
    #[doc = "Masked"]
    #[inline(always)]
    pub fn masked(self) -> &'a mut W {
        self.variant(PMUX_OUT7_A::MASKED)
    }
    #[doc = "Not masked"]
    #[inline(always)]
    pub fn not_masked(self) -> &'a mut W {
        self.variant(PMUX_OUT7_A::NOT_MASKED)
    }
}
#[doc = "Field `PMUX_OUT8` reader - Mask bit"]
pub type PMUX_OUT8_R = crate::BitReader<PMUX_OUT8_A>;
#[doc = "Mask bit\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PMUX_OUT8_A {
    #[doc = "0: Masked"]
    MASKED = 0,
    #[doc = "1: Not masked"]
    NOT_MASKED = 1,
}
impl From<PMUX_OUT8_A> for bool {
    #[inline(always)]
    fn from(variant: PMUX_OUT8_A) -> Self {
        variant as u8 != 0
    }
}
impl PMUX_OUT8_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PMUX_OUT8_A {
        match self.bits {
            false => PMUX_OUT8_A::MASKED,
            true => PMUX_OUT8_A::NOT_MASKED,
        }
    }
    #[doc = "Checks if the value of the field is `MASKED`"]
    #[inline(always)]
    pub fn is_masked(&self) -> bool {
        *self == PMUX_OUT8_A::MASKED
    }
    #[doc = "Checks if the value of the field is `NOT_MASKED`"]
    #[inline(always)]
    pub fn is_not_masked(&self) -> bool {
        *self == PMUX_OUT8_A::NOT_MASKED
    }
}
#[doc = "Field `PMUX_OUT8` writer - Mask bit"]
pub type PMUX_OUT8_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, DSP_INT_MASK0_SPEC, PMUX_OUT8_A, O>;
impl<'a, const O: u8> PMUX_OUT8_W<'a, O> {
    #[doc = "Masked"]
    #[inline(always)]
    pub fn masked(self) -> &'a mut W {
        self.variant(PMUX_OUT8_A::MASKED)
    }
    #[doc = "Not masked"]
    #[inline(always)]
    pub fn not_masked(self) -> &'a mut W {
        self.variant(PMUX_OUT8_A::NOT_MASKED)
    }
}
#[doc = "Field `PMUX_OUT9` reader - Mask bit"]
pub type PMUX_OUT9_R = crate::BitReader<PMUX_OUT9_A>;
#[doc = "Mask bit\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PMUX_OUT9_A {
    #[doc = "0: Masked"]
    MASKED = 0,
    #[doc = "1: Not masked"]
    NOT_MASKED = 1,
}
impl From<PMUX_OUT9_A> for bool {
    #[inline(always)]
    fn from(variant: PMUX_OUT9_A) -> Self {
        variant as u8 != 0
    }
}
impl PMUX_OUT9_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PMUX_OUT9_A {
        match self.bits {
            false => PMUX_OUT9_A::MASKED,
            true => PMUX_OUT9_A::NOT_MASKED,
        }
    }
    #[doc = "Checks if the value of the field is `MASKED`"]
    #[inline(always)]
    pub fn is_masked(&self) -> bool {
        *self == PMUX_OUT9_A::MASKED
    }
    #[doc = "Checks if the value of the field is `NOT_MASKED`"]
    #[inline(always)]
    pub fn is_not_masked(&self) -> bool {
        *self == PMUX_OUT9_A::NOT_MASKED
    }
}
#[doc = "Field `PMUX_OUT9` writer - Mask bit"]
pub type PMUX_OUT9_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, DSP_INT_MASK0_SPEC, PMUX_OUT9_A, O>;
impl<'a, const O: u8> PMUX_OUT9_W<'a, O> {
    #[doc = "Masked"]
    #[inline(always)]
    pub fn masked(self) -> &'a mut W {
        self.variant(PMUX_OUT9_A::MASKED)
    }
    #[doc = "Not masked"]
    #[inline(always)]
    pub fn not_masked(self) -> &'a mut W {
        self.variant(PMUX_OUT9_A::NOT_MASKED)
    }
}
#[doc = "Field `PMUX_OUT10` reader - Mask bit"]
pub type PMUX_OUT10_R = crate::BitReader<PMUX_OUT10_A>;
#[doc = "Mask bit\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PMUX_OUT10_A {
    #[doc = "0: Masked"]
    MASKED = 0,
    #[doc = "1: Not masked"]
    NOT_MASKED = 1,
}
impl From<PMUX_OUT10_A> for bool {
    #[inline(always)]
    fn from(variant: PMUX_OUT10_A) -> Self {
        variant as u8 != 0
    }
}
impl PMUX_OUT10_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PMUX_OUT10_A {
        match self.bits {
            false => PMUX_OUT10_A::MASKED,
            true => PMUX_OUT10_A::NOT_MASKED,
        }
    }
    #[doc = "Checks if the value of the field is `MASKED`"]
    #[inline(always)]
    pub fn is_masked(&self) -> bool {
        *self == PMUX_OUT10_A::MASKED
    }
    #[doc = "Checks if the value of the field is `NOT_MASKED`"]
    #[inline(always)]
    pub fn is_not_masked(&self) -> bool {
        *self == PMUX_OUT10_A::NOT_MASKED
    }
}
#[doc = "Field `PMUX_OUT10` writer - Mask bit"]
pub type PMUX_OUT10_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, DSP_INT_MASK0_SPEC, PMUX_OUT10_A, O>;
impl<'a, const O: u8> PMUX_OUT10_W<'a, O> {
    #[doc = "Masked"]
    #[inline(always)]
    pub fn masked(self) -> &'a mut W {
        self.variant(PMUX_OUT10_A::MASKED)
    }
    #[doc = "Not masked"]
    #[inline(always)]
    pub fn not_masked(self) -> &'a mut W {
        self.variant(PMUX_OUT10_A::NOT_MASKED)
    }
}
#[doc = "Field `PMUX_OUT11` reader - Mask bit"]
pub type PMUX_OUT11_R = crate::BitReader<PMUX_OUT11_A>;
#[doc = "Mask bit\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PMUX_OUT11_A {
    #[doc = "0: Masked"]
    MASKED = 0,
    #[doc = "1: Not masked"]
    NOT_MASKED = 1,
}
impl From<PMUX_OUT11_A> for bool {
    #[inline(always)]
    fn from(variant: PMUX_OUT11_A) -> Self {
        variant as u8 != 0
    }
}
impl PMUX_OUT11_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PMUX_OUT11_A {
        match self.bits {
            false => PMUX_OUT11_A::MASKED,
            true => PMUX_OUT11_A::NOT_MASKED,
        }
    }
    #[doc = "Checks if the value of the field is `MASKED`"]
    #[inline(always)]
    pub fn is_masked(&self) -> bool {
        *self == PMUX_OUT11_A::MASKED
    }
    #[doc = "Checks if the value of the field is `NOT_MASKED`"]
    #[inline(always)]
    pub fn is_not_masked(&self) -> bool {
        *self == PMUX_OUT11_A::NOT_MASKED
    }
}
#[doc = "Field `PMUX_OUT11` writer - Mask bit"]
pub type PMUX_OUT11_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, DSP_INT_MASK0_SPEC, PMUX_OUT11_A, O>;
impl<'a, const O: u8> PMUX_OUT11_W<'a, O> {
    #[doc = "Masked"]
    #[inline(always)]
    pub fn masked(self) -> &'a mut W {
        self.variant(PMUX_OUT11_A::MASKED)
    }
    #[doc = "Not masked"]
    #[inline(always)]
    pub fn not_masked(self) -> &'a mut W {
        self.variant(PMUX_OUT11_A::NOT_MASKED)
    }
}
#[doc = "Field `PMUX_OUT12` reader - Mask bit"]
pub type PMUX_OUT12_R = crate::BitReader<PMUX_OUT12_A>;
#[doc = "Mask bit\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PMUX_OUT12_A {
    #[doc = "0: Masked"]
    MASKED = 0,
    #[doc = "1: Not masked"]
    NOT_MASKED = 1,
}
impl From<PMUX_OUT12_A> for bool {
    #[inline(always)]
    fn from(variant: PMUX_OUT12_A) -> Self {
        variant as u8 != 0
    }
}
impl PMUX_OUT12_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PMUX_OUT12_A {
        match self.bits {
            false => PMUX_OUT12_A::MASKED,
            true => PMUX_OUT12_A::NOT_MASKED,
        }
    }
    #[doc = "Checks if the value of the field is `MASKED`"]
    #[inline(always)]
    pub fn is_masked(&self) -> bool {
        *self == PMUX_OUT12_A::MASKED
    }
    #[doc = "Checks if the value of the field is `NOT_MASKED`"]
    #[inline(always)]
    pub fn is_not_masked(&self) -> bool {
        *self == PMUX_OUT12_A::NOT_MASKED
    }
}
#[doc = "Field `PMUX_OUT12` writer - Mask bit"]
pub type PMUX_OUT12_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, DSP_INT_MASK0_SPEC, PMUX_OUT12_A, O>;
impl<'a, const O: u8> PMUX_OUT12_W<'a, O> {
    #[doc = "Masked"]
    #[inline(always)]
    pub fn masked(self) -> &'a mut W {
        self.variant(PMUX_OUT12_A::MASKED)
    }
    #[doc = "Not masked"]
    #[inline(always)]
    pub fn not_masked(self) -> &'a mut W {
        self.variant(PMUX_OUT12_A::NOT_MASKED)
    }
}
#[doc = "Field `PMUX_OUT13` reader - Mask bit"]
pub type PMUX_OUT13_R = crate::BitReader<PMUX_OUT13_A>;
#[doc = "Mask bit\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PMUX_OUT13_A {
    #[doc = "0: Masked"]
    MASKED = 0,
    #[doc = "1: Not masked"]
    NOT_MASKED = 1,
}
impl From<PMUX_OUT13_A> for bool {
    #[inline(always)]
    fn from(variant: PMUX_OUT13_A) -> Self {
        variant as u8 != 0
    }
}
impl PMUX_OUT13_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PMUX_OUT13_A {
        match self.bits {
            false => PMUX_OUT13_A::MASKED,
            true => PMUX_OUT13_A::NOT_MASKED,
        }
    }
    #[doc = "Checks if the value of the field is `MASKED`"]
    #[inline(always)]
    pub fn is_masked(&self) -> bool {
        *self == PMUX_OUT13_A::MASKED
    }
    #[doc = "Checks if the value of the field is `NOT_MASKED`"]
    #[inline(always)]
    pub fn is_not_masked(&self) -> bool {
        *self == PMUX_OUT13_A::NOT_MASKED
    }
}
#[doc = "Field `PMUX_OUT13` writer - Mask bit"]
pub type PMUX_OUT13_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, DSP_INT_MASK0_SPEC, PMUX_OUT13_A, O>;
impl<'a, const O: u8> PMUX_OUT13_W<'a, O> {
    #[doc = "Masked"]
    #[inline(always)]
    pub fn masked(self) -> &'a mut W {
        self.variant(PMUX_OUT13_A::MASKED)
    }
    #[doc = "Not masked"]
    #[inline(always)]
    pub fn not_masked(self) -> &'a mut W {
        self.variant(PMUX_OUT13_A::NOT_MASKED)
    }
}
#[doc = "Field `PMUX_OUT14` reader - Mask bit"]
pub type PMUX_OUT14_R = crate::BitReader<PMUX_OUT14_A>;
#[doc = "Mask bit\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PMUX_OUT14_A {
    #[doc = "0: Masked"]
    MASKED = 0,
    #[doc = "1: Not masked"]
    NOT_MASKED = 1,
}
impl From<PMUX_OUT14_A> for bool {
    #[inline(always)]
    fn from(variant: PMUX_OUT14_A) -> Self {
        variant as u8 != 0
    }
}
impl PMUX_OUT14_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PMUX_OUT14_A {
        match self.bits {
            false => PMUX_OUT14_A::MASKED,
            true => PMUX_OUT14_A::NOT_MASKED,
        }
    }
    #[doc = "Checks if the value of the field is `MASKED`"]
    #[inline(always)]
    pub fn is_masked(&self) -> bool {
        *self == PMUX_OUT14_A::MASKED
    }
    #[doc = "Checks if the value of the field is `NOT_MASKED`"]
    #[inline(always)]
    pub fn is_not_masked(&self) -> bool {
        *self == PMUX_OUT14_A::NOT_MASKED
    }
}
#[doc = "Field `PMUX_OUT14` writer - Mask bit"]
pub type PMUX_OUT14_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, DSP_INT_MASK0_SPEC, PMUX_OUT14_A, O>;
impl<'a, const O: u8> PMUX_OUT14_W<'a, O> {
    #[doc = "Masked"]
    #[inline(always)]
    pub fn masked(self) -> &'a mut W {
        self.variant(PMUX_OUT14_A::MASKED)
    }
    #[doc = "Not masked"]
    #[inline(always)]
    pub fn not_masked(self) -> &'a mut W {
        self.variant(PMUX_OUT14_A::NOT_MASKED)
    }
}
#[doc = "Field `PMUX_OUT15` reader - Mask bit"]
pub type PMUX_OUT15_R = crate::BitReader<PMUX_OUT15_A>;
#[doc = "Mask bit\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PMUX_OUT15_A {
    #[doc = "0: Masked"]
    MASKED = 0,
    #[doc = "1: Not masked"]
    NOT_MASKED = 1,
}
impl From<PMUX_OUT15_A> for bool {
    #[inline(always)]
    fn from(variant: PMUX_OUT15_A) -> Self {
        variant as u8 != 0
    }
}
impl PMUX_OUT15_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PMUX_OUT15_A {
        match self.bits {
            false => PMUX_OUT15_A::MASKED,
            true => PMUX_OUT15_A::NOT_MASKED,
        }
    }
    #[doc = "Checks if the value of the field is `MASKED`"]
    #[inline(always)]
    pub fn is_masked(&self) -> bool {
        *self == PMUX_OUT15_A::MASKED
    }
    #[doc = "Checks if the value of the field is `NOT_MASKED`"]
    #[inline(always)]
    pub fn is_not_masked(&self) -> bool {
        *self == PMUX_OUT15_A::NOT_MASKED
    }
}
#[doc = "Field `PMUX_OUT15` writer - Mask bit"]
pub type PMUX_OUT15_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, DSP_INT_MASK0_SPEC, PMUX_OUT15_A, O>;
impl<'a, const O: u8> PMUX_OUT15_W<'a, O> {
    #[doc = "Masked"]
    #[inline(always)]
    pub fn masked(self) -> &'a mut W {
        self.variant(PMUX_OUT15_A::MASKED)
    }
    #[doc = "Not masked"]
    #[inline(always)]
    pub fn not_masked(self) -> &'a mut W {
        self.variant(PMUX_OUT15_A::NOT_MASKED)
    }
}
#[doc = "Field `PMUX_OUT16` reader - Mask bit"]
pub type PMUX_OUT16_R = crate::BitReader<PMUX_OUT16_A>;
#[doc = "Mask bit\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PMUX_OUT16_A {
    #[doc = "0: Masked"]
    MASKED = 0,
    #[doc = "1: Not masked"]
    NOT_MASKED = 1,
}
impl From<PMUX_OUT16_A> for bool {
    #[inline(always)]
    fn from(variant: PMUX_OUT16_A) -> Self {
        variant as u8 != 0
    }
}
impl PMUX_OUT16_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PMUX_OUT16_A {
        match self.bits {
            false => PMUX_OUT16_A::MASKED,
            true => PMUX_OUT16_A::NOT_MASKED,
        }
    }
    #[doc = "Checks if the value of the field is `MASKED`"]
    #[inline(always)]
    pub fn is_masked(&self) -> bool {
        *self == PMUX_OUT16_A::MASKED
    }
    #[doc = "Checks if the value of the field is `NOT_MASKED`"]
    #[inline(always)]
    pub fn is_not_masked(&self) -> bool {
        *self == PMUX_OUT16_A::NOT_MASKED
    }
}
#[doc = "Field `PMUX_OUT16` writer - Mask bit"]
pub type PMUX_OUT16_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, DSP_INT_MASK0_SPEC, PMUX_OUT16_A, O>;
impl<'a, const O: u8> PMUX_OUT16_W<'a, O> {
    #[doc = "Masked"]
    #[inline(always)]
    pub fn masked(self) -> &'a mut W {
        self.variant(PMUX_OUT16_A::MASKED)
    }
    #[doc = "Not masked"]
    #[inline(always)]
    pub fn not_masked(self) -> &'a mut W {
        self.variant(PMUX_OUT16_A::NOT_MASKED)
    }
}
#[doc = "Field `PMUX_OUT17` reader - Mask bit"]
pub type PMUX_OUT17_R = crate::BitReader<PMUX_OUT17_A>;
#[doc = "Mask bit\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PMUX_OUT17_A {
    #[doc = "0: Masked"]
    MASKED = 0,
    #[doc = "1: Not masked"]
    NOT_MASKED = 1,
}
impl From<PMUX_OUT17_A> for bool {
    #[inline(always)]
    fn from(variant: PMUX_OUT17_A) -> Self {
        variant as u8 != 0
    }
}
impl PMUX_OUT17_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PMUX_OUT17_A {
        match self.bits {
            false => PMUX_OUT17_A::MASKED,
            true => PMUX_OUT17_A::NOT_MASKED,
        }
    }
    #[doc = "Checks if the value of the field is `MASKED`"]
    #[inline(always)]
    pub fn is_masked(&self) -> bool {
        *self == PMUX_OUT17_A::MASKED
    }
    #[doc = "Checks if the value of the field is `NOT_MASKED`"]
    #[inline(always)]
    pub fn is_not_masked(&self) -> bool {
        *self == PMUX_OUT17_A::NOT_MASKED
    }
}
#[doc = "Field `PMUX_OUT17` writer - Mask bit"]
pub type PMUX_OUT17_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, DSP_INT_MASK0_SPEC, PMUX_OUT17_A, O>;
impl<'a, const O: u8> PMUX_OUT17_W<'a, O> {
    #[doc = "Masked"]
    #[inline(always)]
    pub fn masked(self) -> &'a mut W {
        self.variant(PMUX_OUT17_A::MASKED)
    }
    #[doc = "Not masked"]
    #[inline(always)]
    pub fn not_masked(self) -> &'a mut W {
        self.variant(PMUX_OUT17_A::NOT_MASKED)
    }
}
#[doc = "Field `PMUX_OUT18` reader - Mask bit"]
pub type PMUX_OUT18_R = crate::BitReader<PMUX_OUT18_A>;
#[doc = "Mask bit\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PMUX_OUT18_A {
    #[doc = "0: Masked"]
    MASKED = 0,
    #[doc = "1: Not masked"]
    NOT_MASKED = 1,
}
impl From<PMUX_OUT18_A> for bool {
    #[inline(always)]
    fn from(variant: PMUX_OUT18_A) -> Self {
        variant as u8 != 0
    }
}
impl PMUX_OUT18_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PMUX_OUT18_A {
        match self.bits {
            false => PMUX_OUT18_A::MASKED,
            true => PMUX_OUT18_A::NOT_MASKED,
        }
    }
    #[doc = "Checks if the value of the field is `MASKED`"]
    #[inline(always)]
    pub fn is_masked(&self) -> bool {
        *self == PMUX_OUT18_A::MASKED
    }
    #[doc = "Checks if the value of the field is `NOT_MASKED`"]
    #[inline(always)]
    pub fn is_not_masked(&self) -> bool {
        *self == PMUX_OUT18_A::NOT_MASKED
    }
}
#[doc = "Field `PMUX_OUT18` writer - Mask bit"]
pub type PMUX_OUT18_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, DSP_INT_MASK0_SPEC, PMUX_OUT18_A, O>;
impl<'a, const O: u8> PMUX_OUT18_W<'a, O> {
    #[doc = "Masked"]
    #[inline(always)]
    pub fn masked(self) -> &'a mut W {
        self.variant(PMUX_OUT18_A::MASKED)
    }
    #[doc = "Not masked"]
    #[inline(always)]
    pub fn not_masked(self) -> &'a mut W {
        self.variant(PMUX_OUT18_A::NOT_MASKED)
    }
}
#[doc = "Field `PMUX_OUT19` reader - Mask bit"]
pub type PMUX_OUT19_R = crate::BitReader<PMUX_OUT19_A>;
#[doc = "Mask bit\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PMUX_OUT19_A {
    #[doc = "0: Masked"]
    MASKED = 0,
    #[doc = "1: Not masked"]
    NOT_MASKED = 1,
}
impl From<PMUX_OUT19_A> for bool {
    #[inline(always)]
    fn from(variant: PMUX_OUT19_A) -> Self {
        variant as u8 != 0
    }
}
impl PMUX_OUT19_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PMUX_OUT19_A {
        match self.bits {
            false => PMUX_OUT19_A::MASKED,
            true => PMUX_OUT19_A::NOT_MASKED,
        }
    }
    #[doc = "Checks if the value of the field is `MASKED`"]
    #[inline(always)]
    pub fn is_masked(&self) -> bool {
        *self == PMUX_OUT19_A::MASKED
    }
    #[doc = "Checks if the value of the field is `NOT_MASKED`"]
    #[inline(always)]
    pub fn is_not_masked(&self) -> bool {
        *self == PMUX_OUT19_A::NOT_MASKED
    }
}
#[doc = "Field `PMUX_OUT19` writer - Mask bit"]
pub type PMUX_OUT19_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, DSP_INT_MASK0_SPEC, PMUX_OUT19_A, O>;
impl<'a, const O: u8> PMUX_OUT19_W<'a, O> {
    #[doc = "Masked"]
    #[inline(always)]
    pub fn masked(self) -> &'a mut W {
        self.variant(PMUX_OUT19_A::MASKED)
    }
    #[doc = "Not masked"]
    #[inline(always)]
    pub fn not_masked(self) -> &'a mut W {
        self.variant(PMUX_OUT19_A::NOT_MASKED)
    }
}
#[doc = "Field `PMUX_OUT20` reader - Mask bit"]
pub type PMUX_OUT20_R = crate::BitReader<PMUX_OUT20_A>;
#[doc = "Mask bit\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PMUX_OUT20_A {
    #[doc = "0: Masked"]
    MASKED = 0,
    #[doc = "1: Not masked"]
    NOT_MASKED = 1,
}
impl From<PMUX_OUT20_A> for bool {
    #[inline(always)]
    fn from(variant: PMUX_OUT20_A) -> Self {
        variant as u8 != 0
    }
}
impl PMUX_OUT20_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PMUX_OUT20_A {
        match self.bits {
            false => PMUX_OUT20_A::MASKED,
            true => PMUX_OUT20_A::NOT_MASKED,
        }
    }
    #[doc = "Checks if the value of the field is `MASKED`"]
    #[inline(always)]
    pub fn is_masked(&self) -> bool {
        *self == PMUX_OUT20_A::MASKED
    }
    #[doc = "Checks if the value of the field is `NOT_MASKED`"]
    #[inline(always)]
    pub fn is_not_masked(&self) -> bool {
        *self == PMUX_OUT20_A::NOT_MASKED
    }
}
#[doc = "Field `PMUX_OUT20` writer - Mask bit"]
pub type PMUX_OUT20_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, DSP_INT_MASK0_SPEC, PMUX_OUT20_A, O>;
impl<'a, const O: u8> PMUX_OUT20_W<'a, O> {
    #[doc = "Masked"]
    #[inline(always)]
    pub fn masked(self) -> &'a mut W {
        self.variant(PMUX_OUT20_A::MASKED)
    }
    #[doc = "Not masked"]
    #[inline(always)]
    pub fn not_masked(self) -> &'a mut W {
        self.variant(PMUX_OUT20_A::NOT_MASKED)
    }
}
#[doc = "Field `PMUX_OUT21` reader - Mask bit"]
pub type PMUX_OUT21_R = crate::BitReader<PMUX_OUT21_A>;
#[doc = "Mask bit\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PMUX_OUT21_A {
    #[doc = "0: Masked"]
    MASKED = 0,
    #[doc = "1: Not masked"]
    NOT_MASKED = 1,
}
impl From<PMUX_OUT21_A> for bool {
    #[inline(always)]
    fn from(variant: PMUX_OUT21_A) -> Self {
        variant as u8 != 0
    }
}
impl PMUX_OUT21_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PMUX_OUT21_A {
        match self.bits {
            false => PMUX_OUT21_A::MASKED,
            true => PMUX_OUT21_A::NOT_MASKED,
        }
    }
    #[doc = "Checks if the value of the field is `MASKED`"]
    #[inline(always)]
    pub fn is_masked(&self) -> bool {
        *self == PMUX_OUT21_A::MASKED
    }
    #[doc = "Checks if the value of the field is `NOT_MASKED`"]
    #[inline(always)]
    pub fn is_not_masked(&self) -> bool {
        *self == PMUX_OUT21_A::NOT_MASKED
    }
}
#[doc = "Field `PMUX_OUT21` writer - Mask bit"]
pub type PMUX_OUT21_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, DSP_INT_MASK0_SPEC, PMUX_OUT21_A, O>;
impl<'a, const O: u8> PMUX_OUT21_W<'a, O> {
    #[doc = "Masked"]
    #[inline(always)]
    pub fn masked(self) -> &'a mut W {
        self.variant(PMUX_OUT21_A::MASKED)
    }
    #[doc = "Not masked"]
    #[inline(always)]
    pub fn not_masked(self) -> &'a mut W {
        self.variant(PMUX_OUT21_A::NOT_MASKED)
    }
}
#[doc = "Field `PMUX_OUT22` reader - Mask bit"]
pub type PMUX_OUT22_R = crate::BitReader<PMUX_OUT22_A>;
#[doc = "Mask bit\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PMUX_OUT22_A {
    #[doc = "0: Masked"]
    MASKED = 0,
    #[doc = "1: Not masked"]
    NOT_MASKED = 1,
}
impl From<PMUX_OUT22_A> for bool {
    #[inline(always)]
    fn from(variant: PMUX_OUT22_A) -> Self {
        variant as u8 != 0
    }
}
impl PMUX_OUT22_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PMUX_OUT22_A {
        match self.bits {
            false => PMUX_OUT22_A::MASKED,
            true => PMUX_OUT22_A::NOT_MASKED,
        }
    }
    #[doc = "Checks if the value of the field is `MASKED`"]
    #[inline(always)]
    pub fn is_masked(&self) -> bool {
        *self == PMUX_OUT22_A::MASKED
    }
    #[doc = "Checks if the value of the field is `NOT_MASKED`"]
    #[inline(always)]
    pub fn is_not_masked(&self) -> bool {
        *self == PMUX_OUT22_A::NOT_MASKED
    }
}
#[doc = "Field `PMUX_OUT22` writer - Mask bit"]
pub type PMUX_OUT22_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, DSP_INT_MASK0_SPEC, PMUX_OUT22_A, O>;
impl<'a, const O: u8> PMUX_OUT22_W<'a, O> {
    #[doc = "Masked"]
    #[inline(always)]
    pub fn masked(self) -> &'a mut W {
        self.variant(PMUX_OUT22_A::MASKED)
    }
    #[doc = "Not masked"]
    #[inline(always)]
    pub fn not_masked(self) -> &'a mut W {
        self.variant(PMUX_OUT22_A::NOT_MASKED)
    }
}
#[doc = "Field `PMUX_OUT23` reader - Mask bit"]
pub type PMUX_OUT23_R = crate::BitReader<PMUX_OUT23_A>;
#[doc = "Mask bit\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PMUX_OUT23_A {
    #[doc = "0: Masked"]
    MASKED = 0,
    #[doc = "1: Not masked"]
    NOT_MASKED = 1,
}
impl From<PMUX_OUT23_A> for bool {
    #[inline(always)]
    fn from(variant: PMUX_OUT23_A) -> Self {
        variant as u8 != 0
    }
}
impl PMUX_OUT23_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PMUX_OUT23_A {
        match self.bits {
            false => PMUX_OUT23_A::MASKED,
            true => PMUX_OUT23_A::NOT_MASKED,
        }
    }
    #[doc = "Checks if the value of the field is `MASKED`"]
    #[inline(always)]
    pub fn is_masked(&self) -> bool {
        *self == PMUX_OUT23_A::MASKED
    }
    #[doc = "Checks if the value of the field is `NOT_MASKED`"]
    #[inline(always)]
    pub fn is_not_masked(&self) -> bool {
        *self == PMUX_OUT23_A::NOT_MASKED
    }
}
#[doc = "Field `PMUX_OUT23` writer - Mask bit"]
pub type PMUX_OUT23_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, DSP_INT_MASK0_SPEC, PMUX_OUT23_A, O>;
impl<'a, const O: u8> PMUX_OUT23_W<'a, O> {
    #[doc = "Masked"]
    #[inline(always)]
    pub fn masked(self) -> &'a mut W {
        self.variant(PMUX_OUT23_A::MASKED)
    }
    #[doc = "Not masked"]
    #[inline(always)]
    pub fn not_masked(self) -> &'a mut W {
        self.variant(PMUX_OUT23_A::NOT_MASKED)
    }
}
#[doc = "Field `PMUX_OUT24` reader - Mask bit"]
pub type PMUX_OUT24_R = crate::BitReader<PMUX_OUT24_A>;
#[doc = "Mask bit\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PMUX_OUT24_A {
    #[doc = "0: Masked"]
    MASKED = 0,
    #[doc = "1: Not masked"]
    NOT_MASKED = 1,
}
impl From<PMUX_OUT24_A> for bool {
    #[inline(always)]
    fn from(variant: PMUX_OUT24_A) -> Self {
        variant as u8 != 0
    }
}
impl PMUX_OUT24_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PMUX_OUT24_A {
        match self.bits {
            false => PMUX_OUT24_A::MASKED,
            true => PMUX_OUT24_A::NOT_MASKED,
        }
    }
    #[doc = "Checks if the value of the field is `MASKED`"]
    #[inline(always)]
    pub fn is_masked(&self) -> bool {
        *self == PMUX_OUT24_A::MASKED
    }
    #[doc = "Checks if the value of the field is `NOT_MASKED`"]
    #[inline(always)]
    pub fn is_not_masked(&self) -> bool {
        *self == PMUX_OUT24_A::NOT_MASKED
    }
}
#[doc = "Field `PMUX_OUT24` writer - Mask bit"]
pub type PMUX_OUT24_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, DSP_INT_MASK0_SPEC, PMUX_OUT24_A, O>;
impl<'a, const O: u8> PMUX_OUT24_W<'a, O> {
    #[doc = "Masked"]
    #[inline(always)]
    pub fn masked(self) -> &'a mut W {
        self.variant(PMUX_OUT24_A::MASKED)
    }
    #[doc = "Not masked"]
    #[inline(always)]
    pub fn not_masked(self) -> &'a mut W {
        self.variant(PMUX_OUT24_A::NOT_MASKED)
    }
}
#[doc = "Field `PMUX_OUT25` reader - Mask bit"]
pub type PMUX_OUT25_R = crate::BitReader<PMUX_OUT25_A>;
#[doc = "Mask bit\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PMUX_OUT25_A {
    #[doc = "0: Masked"]
    MASKED = 0,
    #[doc = "1: Not masked"]
    NOT_MASKED = 1,
}
impl From<PMUX_OUT25_A> for bool {
    #[inline(always)]
    fn from(variant: PMUX_OUT25_A) -> Self {
        variant as u8 != 0
    }
}
impl PMUX_OUT25_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PMUX_OUT25_A {
        match self.bits {
            false => PMUX_OUT25_A::MASKED,
            true => PMUX_OUT25_A::NOT_MASKED,
        }
    }
    #[doc = "Checks if the value of the field is `MASKED`"]
    #[inline(always)]
    pub fn is_masked(&self) -> bool {
        *self == PMUX_OUT25_A::MASKED
    }
    #[doc = "Checks if the value of the field is `NOT_MASKED`"]
    #[inline(always)]
    pub fn is_not_masked(&self) -> bool {
        *self == PMUX_OUT25_A::NOT_MASKED
    }
}
#[doc = "Field `PMUX_OUT25` writer - Mask bit"]
pub type PMUX_OUT25_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, DSP_INT_MASK0_SPEC, PMUX_OUT25_A, O>;
impl<'a, const O: u8> PMUX_OUT25_W<'a, O> {
    #[doc = "Masked"]
    #[inline(always)]
    pub fn masked(self) -> &'a mut W {
        self.variant(PMUX_OUT25_A::MASKED)
    }
    #[doc = "Not masked"]
    #[inline(always)]
    pub fn not_masked(self) -> &'a mut W {
        self.variant(PMUX_OUT25_A::NOT_MASKED)
    }
}
#[doc = "Field `PMUX_OUT26` reader - Mask bit"]
pub type PMUX_OUT26_R = crate::BitReader<PMUX_OUT26_A>;
#[doc = "Mask bit\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PMUX_OUT26_A {
    #[doc = "0: Masked"]
    MASKED = 0,
    #[doc = "1: Not masked"]
    NOT_MASKED = 1,
}
impl From<PMUX_OUT26_A> for bool {
    #[inline(always)]
    fn from(variant: PMUX_OUT26_A) -> Self {
        variant as u8 != 0
    }
}
impl PMUX_OUT26_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PMUX_OUT26_A {
        match self.bits {
            false => PMUX_OUT26_A::MASKED,
            true => PMUX_OUT26_A::NOT_MASKED,
        }
    }
    #[doc = "Checks if the value of the field is `MASKED`"]
    #[inline(always)]
    pub fn is_masked(&self) -> bool {
        *self == PMUX_OUT26_A::MASKED
    }
    #[doc = "Checks if the value of the field is `NOT_MASKED`"]
    #[inline(always)]
    pub fn is_not_masked(&self) -> bool {
        *self == PMUX_OUT26_A::NOT_MASKED
    }
}
#[doc = "Field `PMUX_OUT26` writer - Mask bit"]
pub type PMUX_OUT26_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, DSP_INT_MASK0_SPEC, PMUX_OUT26_A, O>;
impl<'a, const O: u8> PMUX_OUT26_W<'a, O> {
    #[doc = "Masked"]
    #[inline(always)]
    pub fn masked(self) -> &'a mut W {
        self.variant(PMUX_OUT26_A::MASKED)
    }
    #[doc = "Not masked"]
    #[inline(always)]
    pub fn not_masked(self) -> &'a mut W {
        self.variant(PMUX_OUT26_A::NOT_MASKED)
    }
}
impl R {
    #[doc = "Bit 5 - Mask bit"]
    #[inline(always)]
    pub fn pmux_out0(&self) -> PMUX_OUT0_R {
        PMUX_OUT0_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Mask bit"]
    #[inline(always)]
    pub fn pmux_out1(&self) -> PMUX_OUT1_R {
        PMUX_OUT1_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Mask bit"]
    #[inline(always)]
    pub fn pmux_out2(&self) -> PMUX_OUT2_R {
        PMUX_OUT2_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Mask bit"]
    #[inline(always)]
    pub fn pmux_out3(&self) -> PMUX_OUT3_R {
        PMUX_OUT3_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Mask bit"]
    #[inline(always)]
    pub fn pmux_out4(&self) -> PMUX_OUT4_R {
        PMUX_OUT4_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Mask bit"]
    #[inline(always)]
    pub fn pmux_out5(&self) -> PMUX_OUT5_R {
        PMUX_OUT5_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Mask bit"]
    #[inline(always)]
    pub fn pmux_out6(&self) -> PMUX_OUT6_R {
        PMUX_OUT6_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Mask bit"]
    #[inline(always)]
    pub fn pmux_out7(&self) -> PMUX_OUT7_R {
        PMUX_OUT7_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Mask bit"]
    #[inline(always)]
    pub fn pmux_out8(&self) -> PMUX_OUT8_R {
        PMUX_OUT8_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Mask bit"]
    #[inline(always)]
    pub fn pmux_out9(&self) -> PMUX_OUT9_R {
        PMUX_OUT9_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Mask bit"]
    #[inline(always)]
    pub fn pmux_out10(&self) -> PMUX_OUT10_R {
        PMUX_OUT10_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Mask bit"]
    #[inline(always)]
    pub fn pmux_out11(&self) -> PMUX_OUT11_R {
        PMUX_OUT11_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Mask bit"]
    #[inline(always)]
    pub fn pmux_out12(&self) -> PMUX_OUT12_R {
        PMUX_OUT12_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Mask bit"]
    #[inline(always)]
    pub fn pmux_out13(&self) -> PMUX_OUT13_R {
        PMUX_OUT13_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Mask bit"]
    #[inline(always)]
    pub fn pmux_out14(&self) -> PMUX_OUT14_R {
        PMUX_OUT14_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Mask bit"]
    #[inline(always)]
    pub fn pmux_out15(&self) -> PMUX_OUT15_R {
        PMUX_OUT15_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Mask bit"]
    #[inline(always)]
    pub fn pmux_out16(&self) -> PMUX_OUT16_R {
        PMUX_OUT16_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Mask bit"]
    #[inline(always)]
    pub fn pmux_out17(&self) -> PMUX_OUT17_R {
        PMUX_OUT17_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Mask bit"]
    #[inline(always)]
    pub fn pmux_out18(&self) -> PMUX_OUT18_R {
        PMUX_OUT18_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Mask bit"]
    #[inline(always)]
    pub fn pmux_out19(&self) -> PMUX_OUT19_R {
        PMUX_OUT19_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Mask bit"]
    #[inline(always)]
    pub fn pmux_out20(&self) -> PMUX_OUT20_R {
        PMUX_OUT20_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Mask bit"]
    #[inline(always)]
    pub fn pmux_out21(&self) -> PMUX_OUT21_R {
        PMUX_OUT21_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Mask bit"]
    #[inline(always)]
    pub fn pmux_out22(&self) -> PMUX_OUT22_R {
        PMUX_OUT22_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Mask bit"]
    #[inline(always)]
    pub fn pmux_out23(&self) -> PMUX_OUT23_R {
        PMUX_OUT23_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Mask bit"]
    #[inline(always)]
    pub fn pmux_out24(&self) -> PMUX_OUT24_R {
        PMUX_OUT24_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Mask bit"]
    #[inline(always)]
    pub fn pmux_out25(&self) -> PMUX_OUT25_R {
        PMUX_OUT25_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Mask bit"]
    #[inline(always)]
    pub fn pmux_out26(&self) -> PMUX_OUT26_R {
        PMUX_OUT26_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 5 - Mask bit"]
    #[inline(always)]
    #[must_use]
    pub fn pmux_out0(&mut self) -> PMUX_OUT0_W<5> {
        PMUX_OUT0_W::new(self)
    }
    #[doc = "Bit 6 - Mask bit"]
    #[inline(always)]
    #[must_use]
    pub fn pmux_out1(&mut self) -> PMUX_OUT1_W<6> {
        PMUX_OUT1_W::new(self)
    }
    #[doc = "Bit 7 - Mask bit"]
    #[inline(always)]
    #[must_use]
    pub fn pmux_out2(&mut self) -> PMUX_OUT2_W<7> {
        PMUX_OUT2_W::new(self)
    }
    #[doc = "Bit 8 - Mask bit"]
    #[inline(always)]
    #[must_use]
    pub fn pmux_out3(&mut self) -> PMUX_OUT3_W<8> {
        PMUX_OUT3_W::new(self)
    }
    #[doc = "Bit 9 - Mask bit"]
    #[inline(always)]
    #[must_use]
    pub fn pmux_out4(&mut self) -> PMUX_OUT4_W<9> {
        PMUX_OUT4_W::new(self)
    }
    #[doc = "Bit 10 - Mask bit"]
    #[inline(always)]
    #[must_use]
    pub fn pmux_out5(&mut self) -> PMUX_OUT5_W<10> {
        PMUX_OUT5_W::new(self)
    }
    #[doc = "Bit 11 - Mask bit"]
    #[inline(always)]
    #[must_use]
    pub fn pmux_out6(&mut self) -> PMUX_OUT6_W<11> {
        PMUX_OUT6_W::new(self)
    }
    #[doc = "Bit 12 - Mask bit"]
    #[inline(always)]
    #[must_use]
    pub fn pmux_out7(&mut self) -> PMUX_OUT7_W<12> {
        PMUX_OUT7_W::new(self)
    }
    #[doc = "Bit 13 - Mask bit"]
    #[inline(always)]
    #[must_use]
    pub fn pmux_out8(&mut self) -> PMUX_OUT8_W<13> {
        PMUX_OUT8_W::new(self)
    }
    #[doc = "Bit 14 - Mask bit"]
    #[inline(always)]
    #[must_use]
    pub fn pmux_out9(&mut self) -> PMUX_OUT9_W<14> {
        PMUX_OUT9_W::new(self)
    }
    #[doc = "Bit 15 - Mask bit"]
    #[inline(always)]
    #[must_use]
    pub fn pmux_out10(&mut self) -> PMUX_OUT10_W<15> {
        PMUX_OUT10_W::new(self)
    }
    #[doc = "Bit 16 - Mask bit"]
    #[inline(always)]
    #[must_use]
    pub fn pmux_out11(&mut self) -> PMUX_OUT11_W<16> {
        PMUX_OUT11_W::new(self)
    }
    #[doc = "Bit 17 - Mask bit"]
    #[inline(always)]
    #[must_use]
    pub fn pmux_out12(&mut self) -> PMUX_OUT12_W<17> {
        PMUX_OUT12_W::new(self)
    }
    #[doc = "Bit 18 - Mask bit"]
    #[inline(always)]
    #[must_use]
    pub fn pmux_out13(&mut self) -> PMUX_OUT13_W<18> {
        PMUX_OUT13_W::new(self)
    }
    #[doc = "Bit 19 - Mask bit"]
    #[inline(always)]
    #[must_use]
    pub fn pmux_out14(&mut self) -> PMUX_OUT14_W<19> {
        PMUX_OUT14_W::new(self)
    }
    #[doc = "Bit 20 - Mask bit"]
    #[inline(always)]
    #[must_use]
    pub fn pmux_out15(&mut self) -> PMUX_OUT15_W<20> {
        PMUX_OUT15_W::new(self)
    }
    #[doc = "Bit 21 - Mask bit"]
    #[inline(always)]
    #[must_use]
    pub fn pmux_out16(&mut self) -> PMUX_OUT16_W<21> {
        PMUX_OUT16_W::new(self)
    }
    #[doc = "Bit 22 - Mask bit"]
    #[inline(always)]
    #[must_use]
    pub fn pmux_out17(&mut self) -> PMUX_OUT17_W<22> {
        PMUX_OUT17_W::new(self)
    }
    #[doc = "Bit 23 - Mask bit"]
    #[inline(always)]
    #[must_use]
    pub fn pmux_out18(&mut self) -> PMUX_OUT18_W<23> {
        PMUX_OUT18_W::new(self)
    }
    #[doc = "Bit 24 - Mask bit"]
    #[inline(always)]
    #[must_use]
    pub fn pmux_out19(&mut self) -> PMUX_OUT19_W<24> {
        PMUX_OUT19_W::new(self)
    }
    #[doc = "Bit 25 - Mask bit"]
    #[inline(always)]
    #[must_use]
    pub fn pmux_out20(&mut self) -> PMUX_OUT20_W<25> {
        PMUX_OUT20_W::new(self)
    }
    #[doc = "Bit 26 - Mask bit"]
    #[inline(always)]
    #[must_use]
    pub fn pmux_out21(&mut self) -> PMUX_OUT21_W<26> {
        PMUX_OUT21_W::new(self)
    }
    #[doc = "Bit 27 - Mask bit"]
    #[inline(always)]
    #[must_use]
    pub fn pmux_out22(&mut self) -> PMUX_OUT22_W<27> {
        PMUX_OUT22_W::new(self)
    }
    #[doc = "Bit 28 - Mask bit"]
    #[inline(always)]
    #[must_use]
    pub fn pmux_out23(&mut self) -> PMUX_OUT23_W<28> {
        PMUX_OUT23_W::new(self)
    }
    #[doc = "Bit 29 - Mask bit"]
    #[inline(always)]
    #[must_use]
    pub fn pmux_out24(&mut self) -> PMUX_OUT24_W<29> {
        PMUX_OUT24_W::new(self)
    }
    #[doc = "Bit 30 - Mask bit"]
    #[inline(always)]
    #[must_use]
    pub fn pmux_out25(&mut self) -> PMUX_OUT25_W<30> {
        PMUX_OUT25_W::new(self)
    }
    #[doc = "Bit 31 - Mask bit"]
    #[inline(always)]
    #[must_use]
    pub fn pmux_out26(&mut self) -> PMUX_OUT26_W<31> {
        PMUX_OUT26_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Secure Interrupt Mask for DSP Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dsp_int_mask0](index.html) module"]
pub struct DSP_INT_MASK0_SPEC;
impl crate::RegisterSpec for DSP_INT_MASK0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dsp_int_mask0::R](R) reader structure"]
impl crate::Readable for DSP_INT_MASK0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dsp_int_mask0::W](W) writer structure"]
impl crate::Writable for DSP_INT_MASK0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DSP_INT_MASK0 to value 0xffff_ffff"]
impl crate::Resettable for DSP_INT_MASK0_SPEC {
    const RESET_VALUE: Self::Ux = 0xffff_ffff;
}
