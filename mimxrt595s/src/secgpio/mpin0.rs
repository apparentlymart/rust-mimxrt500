#[doc = "Register `MPIN0` reader"]
pub struct R(crate::R<MPIN0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MPIN0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MPIN0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MPIN0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MPIN0` writer"]
pub struct W(crate::W<MPIN0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MPIN0_SPEC>;
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
impl From<crate::W<MPIN0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MPIN0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MPORTP0` reader - Mask bits for port pins"]
pub type MPORTP0_R = crate::BitReader<MPORTP0_A>;
#[doc = "Mask bits for port pins\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MPORTP0_A {
    #[doc = "0: Read- pin is LOW and/or the corresponding bit in the MASK register is 1; write- clear output bit if the corresponding bit in the MASK register is 0"]
    MPORTP_0 = 0,
    #[doc = "1: Read- pin is HIGH and the corresponding bit in the MASK register is 0; write- set output bit if the corresponding bit in the MASK register is 0"]
    MPORTP_1 = 1,
}
impl From<MPORTP0_A> for bool {
    #[inline(always)]
    fn from(variant: MPORTP0_A) -> Self {
        variant as u8 != 0
    }
}
impl MPORTP0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MPORTP0_A {
        match self.bits {
            false => MPORTP0_A::MPORTP_0,
            true => MPORTP0_A::MPORTP_1,
        }
    }
    #[doc = "Checks if the value of the field is `MPORTP_0`"]
    #[inline(always)]
    pub fn is_mportp_0(&self) -> bool {
        *self == MPORTP0_A::MPORTP_0
    }
    #[doc = "Checks if the value of the field is `MPORTP_1`"]
    #[inline(always)]
    pub fn is_mportp_1(&self) -> bool {
        *self == MPORTP0_A::MPORTP_1
    }
}
#[doc = "Field `MPORTP0` writer - Mask bits for port pins"]
pub type MPORTP0_W<'a, const O: u8> = crate::BitWriter<'a, u32, MPIN0_SPEC, MPORTP0_A, O>;
impl<'a, const O: u8> MPORTP0_W<'a, O> {
    #[doc = "Read- pin is LOW and/or the corresponding bit in the MASK register is 1; write- clear output bit if the corresponding bit in the MASK register is 0"]
    #[inline(always)]
    pub fn mportp_0(self) -> &'a mut W {
        self.variant(MPORTP0_A::MPORTP_0)
    }
    #[doc = "Read- pin is HIGH and the corresponding bit in the MASK register is 0; write- set output bit if the corresponding bit in the MASK register is 0"]
    #[inline(always)]
    pub fn mportp_1(self) -> &'a mut W {
        self.variant(MPORTP0_A::MPORTP_1)
    }
}
#[doc = "Field `MPORTP1` reader - Mask bits for port pins"]
pub type MPORTP1_R = crate::BitReader<MPORTP1_A>;
#[doc = "Mask bits for port pins\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MPORTP1_A {
    #[doc = "0: Read- pin is LOW and/or the corresponding bit in the MASK register is 1; write- clear output bit if the corresponding bit in the MASK register is 0"]
    MPORTP_0 = 0,
    #[doc = "1: Read- pin is HIGH and the corresponding bit in the MASK register is 0; write- set output bit if the corresponding bit in the MASK register is 0"]
    MPORTP_1 = 1,
}
impl From<MPORTP1_A> for bool {
    #[inline(always)]
    fn from(variant: MPORTP1_A) -> Self {
        variant as u8 != 0
    }
}
impl MPORTP1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MPORTP1_A {
        match self.bits {
            false => MPORTP1_A::MPORTP_0,
            true => MPORTP1_A::MPORTP_1,
        }
    }
    #[doc = "Checks if the value of the field is `MPORTP_0`"]
    #[inline(always)]
    pub fn is_mportp_0(&self) -> bool {
        *self == MPORTP1_A::MPORTP_0
    }
    #[doc = "Checks if the value of the field is `MPORTP_1`"]
    #[inline(always)]
    pub fn is_mportp_1(&self) -> bool {
        *self == MPORTP1_A::MPORTP_1
    }
}
#[doc = "Field `MPORTP1` writer - Mask bits for port pins"]
pub type MPORTP1_W<'a, const O: u8> = crate::BitWriter<'a, u32, MPIN0_SPEC, MPORTP1_A, O>;
impl<'a, const O: u8> MPORTP1_W<'a, O> {
    #[doc = "Read- pin is LOW and/or the corresponding bit in the MASK register is 1; write- clear output bit if the corresponding bit in the MASK register is 0"]
    #[inline(always)]
    pub fn mportp_0(self) -> &'a mut W {
        self.variant(MPORTP1_A::MPORTP_0)
    }
    #[doc = "Read- pin is HIGH and the corresponding bit in the MASK register is 0; write- set output bit if the corresponding bit in the MASK register is 0"]
    #[inline(always)]
    pub fn mportp_1(self) -> &'a mut W {
        self.variant(MPORTP1_A::MPORTP_1)
    }
}
#[doc = "Field `MPORTP2` reader - Mask bits for port pins"]
pub type MPORTP2_R = crate::BitReader<MPORTP2_A>;
#[doc = "Mask bits for port pins\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MPORTP2_A {
    #[doc = "0: Read- pin is LOW and/or the corresponding bit in the MASK register is 1; write- clear output bit if the corresponding bit in the MASK register is 0"]
    MPORTP_0 = 0,
    #[doc = "1: Read- pin is HIGH and the corresponding bit in the MASK register is 0; write- set output bit if the corresponding bit in the MASK register is 0"]
    MPORTP_1 = 1,
}
impl From<MPORTP2_A> for bool {
    #[inline(always)]
    fn from(variant: MPORTP2_A) -> Self {
        variant as u8 != 0
    }
}
impl MPORTP2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MPORTP2_A {
        match self.bits {
            false => MPORTP2_A::MPORTP_0,
            true => MPORTP2_A::MPORTP_1,
        }
    }
    #[doc = "Checks if the value of the field is `MPORTP_0`"]
    #[inline(always)]
    pub fn is_mportp_0(&self) -> bool {
        *self == MPORTP2_A::MPORTP_0
    }
    #[doc = "Checks if the value of the field is `MPORTP_1`"]
    #[inline(always)]
    pub fn is_mportp_1(&self) -> bool {
        *self == MPORTP2_A::MPORTP_1
    }
}
#[doc = "Field `MPORTP2` writer - Mask bits for port pins"]
pub type MPORTP2_W<'a, const O: u8> = crate::BitWriter<'a, u32, MPIN0_SPEC, MPORTP2_A, O>;
impl<'a, const O: u8> MPORTP2_W<'a, O> {
    #[doc = "Read- pin is LOW and/or the corresponding bit in the MASK register is 1; write- clear output bit if the corresponding bit in the MASK register is 0"]
    #[inline(always)]
    pub fn mportp_0(self) -> &'a mut W {
        self.variant(MPORTP2_A::MPORTP_0)
    }
    #[doc = "Read- pin is HIGH and the corresponding bit in the MASK register is 0; write- set output bit if the corresponding bit in the MASK register is 0"]
    #[inline(always)]
    pub fn mportp_1(self) -> &'a mut W {
        self.variant(MPORTP2_A::MPORTP_1)
    }
}
#[doc = "Field `MPORTP3` reader - Mask bits for port pins"]
pub type MPORTP3_R = crate::BitReader<MPORTP3_A>;
#[doc = "Mask bits for port pins\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MPORTP3_A {
    #[doc = "0: Read- pin is LOW and/or the corresponding bit in the MASK register is 1; write- clear output bit if the corresponding bit in the MASK register is 0"]
    MPORTP_0 = 0,
    #[doc = "1: Read- pin is HIGH and the corresponding bit in the MASK register is 0; write- set output bit if the corresponding bit in the MASK register is 0"]
    MPORTP_1 = 1,
}
impl From<MPORTP3_A> for bool {
    #[inline(always)]
    fn from(variant: MPORTP3_A) -> Self {
        variant as u8 != 0
    }
}
impl MPORTP3_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MPORTP3_A {
        match self.bits {
            false => MPORTP3_A::MPORTP_0,
            true => MPORTP3_A::MPORTP_1,
        }
    }
    #[doc = "Checks if the value of the field is `MPORTP_0`"]
    #[inline(always)]
    pub fn is_mportp_0(&self) -> bool {
        *self == MPORTP3_A::MPORTP_0
    }
    #[doc = "Checks if the value of the field is `MPORTP_1`"]
    #[inline(always)]
    pub fn is_mportp_1(&self) -> bool {
        *self == MPORTP3_A::MPORTP_1
    }
}
#[doc = "Field `MPORTP3` writer - Mask bits for port pins"]
pub type MPORTP3_W<'a, const O: u8> = crate::BitWriter<'a, u32, MPIN0_SPEC, MPORTP3_A, O>;
impl<'a, const O: u8> MPORTP3_W<'a, O> {
    #[doc = "Read- pin is LOW and/or the corresponding bit in the MASK register is 1; write- clear output bit if the corresponding bit in the MASK register is 0"]
    #[inline(always)]
    pub fn mportp_0(self) -> &'a mut W {
        self.variant(MPORTP3_A::MPORTP_0)
    }
    #[doc = "Read- pin is HIGH and the corresponding bit in the MASK register is 0; write- set output bit if the corresponding bit in the MASK register is 0"]
    #[inline(always)]
    pub fn mportp_1(self) -> &'a mut W {
        self.variant(MPORTP3_A::MPORTP_1)
    }
}
#[doc = "Field `MPORTP4` reader - Mask bits for port pins"]
pub type MPORTP4_R = crate::BitReader<MPORTP4_A>;
#[doc = "Mask bits for port pins\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MPORTP4_A {
    #[doc = "0: Read- pin is LOW and/or the corresponding bit in the MASK register is 1; write- clear output bit if the corresponding bit in the MASK register is 0"]
    MPORTP_0 = 0,
    #[doc = "1: Read- pin is HIGH and the corresponding bit in the MASK register is 0; write- set output bit if the corresponding bit in the MASK register is 0"]
    MPORTP_1 = 1,
}
impl From<MPORTP4_A> for bool {
    #[inline(always)]
    fn from(variant: MPORTP4_A) -> Self {
        variant as u8 != 0
    }
}
impl MPORTP4_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MPORTP4_A {
        match self.bits {
            false => MPORTP4_A::MPORTP_0,
            true => MPORTP4_A::MPORTP_1,
        }
    }
    #[doc = "Checks if the value of the field is `MPORTP_0`"]
    #[inline(always)]
    pub fn is_mportp_0(&self) -> bool {
        *self == MPORTP4_A::MPORTP_0
    }
    #[doc = "Checks if the value of the field is `MPORTP_1`"]
    #[inline(always)]
    pub fn is_mportp_1(&self) -> bool {
        *self == MPORTP4_A::MPORTP_1
    }
}
#[doc = "Field `MPORTP4` writer - Mask bits for port pins"]
pub type MPORTP4_W<'a, const O: u8> = crate::BitWriter<'a, u32, MPIN0_SPEC, MPORTP4_A, O>;
impl<'a, const O: u8> MPORTP4_W<'a, O> {
    #[doc = "Read- pin is LOW and/or the corresponding bit in the MASK register is 1; write- clear output bit if the corresponding bit in the MASK register is 0"]
    #[inline(always)]
    pub fn mportp_0(self) -> &'a mut W {
        self.variant(MPORTP4_A::MPORTP_0)
    }
    #[doc = "Read- pin is HIGH and the corresponding bit in the MASK register is 0; write- set output bit if the corresponding bit in the MASK register is 0"]
    #[inline(always)]
    pub fn mportp_1(self) -> &'a mut W {
        self.variant(MPORTP4_A::MPORTP_1)
    }
}
#[doc = "Field `MPORTP5` reader - Mask bits for port pins"]
pub type MPORTP5_R = crate::BitReader<MPORTP5_A>;
#[doc = "Mask bits for port pins\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MPORTP5_A {
    #[doc = "0: Read- pin is LOW and/or the corresponding bit in the MASK register is 1; write- clear output bit if the corresponding bit in the MASK register is 0"]
    MPORTP_0 = 0,
    #[doc = "1: Read- pin is HIGH and the corresponding bit in the MASK register is 0; write- set output bit if the corresponding bit in the MASK register is 0"]
    MPORTP_1 = 1,
}
impl From<MPORTP5_A> for bool {
    #[inline(always)]
    fn from(variant: MPORTP5_A) -> Self {
        variant as u8 != 0
    }
}
impl MPORTP5_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MPORTP5_A {
        match self.bits {
            false => MPORTP5_A::MPORTP_0,
            true => MPORTP5_A::MPORTP_1,
        }
    }
    #[doc = "Checks if the value of the field is `MPORTP_0`"]
    #[inline(always)]
    pub fn is_mportp_0(&self) -> bool {
        *self == MPORTP5_A::MPORTP_0
    }
    #[doc = "Checks if the value of the field is `MPORTP_1`"]
    #[inline(always)]
    pub fn is_mportp_1(&self) -> bool {
        *self == MPORTP5_A::MPORTP_1
    }
}
#[doc = "Field `MPORTP5` writer - Mask bits for port pins"]
pub type MPORTP5_W<'a, const O: u8> = crate::BitWriter<'a, u32, MPIN0_SPEC, MPORTP5_A, O>;
impl<'a, const O: u8> MPORTP5_W<'a, O> {
    #[doc = "Read- pin is LOW and/or the corresponding bit in the MASK register is 1; write- clear output bit if the corresponding bit in the MASK register is 0"]
    #[inline(always)]
    pub fn mportp_0(self) -> &'a mut W {
        self.variant(MPORTP5_A::MPORTP_0)
    }
    #[doc = "Read- pin is HIGH and the corresponding bit in the MASK register is 0; write- set output bit if the corresponding bit in the MASK register is 0"]
    #[inline(always)]
    pub fn mportp_1(self) -> &'a mut W {
        self.variant(MPORTP5_A::MPORTP_1)
    }
}
#[doc = "Field `MPORTP6` reader - Mask bits for port pins"]
pub type MPORTP6_R = crate::BitReader<MPORTP6_A>;
#[doc = "Mask bits for port pins\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MPORTP6_A {
    #[doc = "0: Read- pin is LOW and/or the corresponding bit in the MASK register is 1; write- clear output bit if the corresponding bit in the MASK register is 0"]
    MPORTP_0 = 0,
    #[doc = "1: Read- pin is HIGH and the corresponding bit in the MASK register is 0; write- set output bit if the corresponding bit in the MASK register is 0"]
    MPORTP_1 = 1,
}
impl From<MPORTP6_A> for bool {
    #[inline(always)]
    fn from(variant: MPORTP6_A) -> Self {
        variant as u8 != 0
    }
}
impl MPORTP6_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MPORTP6_A {
        match self.bits {
            false => MPORTP6_A::MPORTP_0,
            true => MPORTP6_A::MPORTP_1,
        }
    }
    #[doc = "Checks if the value of the field is `MPORTP_0`"]
    #[inline(always)]
    pub fn is_mportp_0(&self) -> bool {
        *self == MPORTP6_A::MPORTP_0
    }
    #[doc = "Checks if the value of the field is `MPORTP_1`"]
    #[inline(always)]
    pub fn is_mportp_1(&self) -> bool {
        *self == MPORTP6_A::MPORTP_1
    }
}
#[doc = "Field `MPORTP6` writer - Mask bits for port pins"]
pub type MPORTP6_W<'a, const O: u8> = crate::BitWriter<'a, u32, MPIN0_SPEC, MPORTP6_A, O>;
impl<'a, const O: u8> MPORTP6_W<'a, O> {
    #[doc = "Read- pin is LOW and/or the corresponding bit in the MASK register is 1; write- clear output bit if the corresponding bit in the MASK register is 0"]
    #[inline(always)]
    pub fn mportp_0(self) -> &'a mut W {
        self.variant(MPORTP6_A::MPORTP_0)
    }
    #[doc = "Read- pin is HIGH and the corresponding bit in the MASK register is 0; write- set output bit if the corresponding bit in the MASK register is 0"]
    #[inline(always)]
    pub fn mportp_1(self) -> &'a mut W {
        self.variant(MPORTP6_A::MPORTP_1)
    }
}
#[doc = "Field `MPORTP7` reader - Mask bits for port pins"]
pub type MPORTP7_R = crate::BitReader<MPORTP7_A>;
#[doc = "Mask bits for port pins\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MPORTP7_A {
    #[doc = "0: Read- pin is LOW and/or the corresponding bit in the MASK register is 1; write- clear output bit if the corresponding bit in the MASK register is 0"]
    MPORTP_0 = 0,
    #[doc = "1: Read- pin is HIGH and the corresponding bit in the MASK register is 0; write- set output bit if the corresponding bit in the MASK register is 0"]
    MPORTP_1 = 1,
}
impl From<MPORTP7_A> for bool {
    #[inline(always)]
    fn from(variant: MPORTP7_A) -> Self {
        variant as u8 != 0
    }
}
impl MPORTP7_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MPORTP7_A {
        match self.bits {
            false => MPORTP7_A::MPORTP_0,
            true => MPORTP7_A::MPORTP_1,
        }
    }
    #[doc = "Checks if the value of the field is `MPORTP_0`"]
    #[inline(always)]
    pub fn is_mportp_0(&self) -> bool {
        *self == MPORTP7_A::MPORTP_0
    }
    #[doc = "Checks if the value of the field is `MPORTP_1`"]
    #[inline(always)]
    pub fn is_mportp_1(&self) -> bool {
        *self == MPORTP7_A::MPORTP_1
    }
}
#[doc = "Field `MPORTP7` writer - Mask bits for port pins"]
pub type MPORTP7_W<'a, const O: u8> = crate::BitWriter<'a, u32, MPIN0_SPEC, MPORTP7_A, O>;
impl<'a, const O: u8> MPORTP7_W<'a, O> {
    #[doc = "Read- pin is LOW and/or the corresponding bit in the MASK register is 1; write- clear output bit if the corresponding bit in the MASK register is 0"]
    #[inline(always)]
    pub fn mportp_0(self) -> &'a mut W {
        self.variant(MPORTP7_A::MPORTP_0)
    }
    #[doc = "Read- pin is HIGH and the corresponding bit in the MASK register is 0; write- set output bit if the corresponding bit in the MASK register is 0"]
    #[inline(always)]
    pub fn mportp_1(self) -> &'a mut W {
        self.variant(MPORTP7_A::MPORTP_1)
    }
}
#[doc = "Field `MPORTP8` reader - Mask bits for port pins"]
pub type MPORTP8_R = crate::BitReader<MPORTP8_A>;
#[doc = "Mask bits for port pins\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MPORTP8_A {
    #[doc = "0: Read- pin is LOW and/or the corresponding bit in the MASK register is 1; write- clear output bit if the corresponding bit in the MASK register is 0"]
    MPORTP_0 = 0,
    #[doc = "1: Read- pin is HIGH and the corresponding bit in the MASK register is 0; write- set output bit if the corresponding bit in the MASK register is 0"]
    MPORTP_1 = 1,
}
impl From<MPORTP8_A> for bool {
    #[inline(always)]
    fn from(variant: MPORTP8_A) -> Self {
        variant as u8 != 0
    }
}
impl MPORTP8_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MPORTP8_A {
        match self.bits {
            false => MPORTP8_A::MPORTP_0,
            true => MPORTP8_A::MPORTP_1,
        }
    }
    #[doc = "Checks if the value of the field is `MPORTP_0`"]
    #[inline(always)]
    pub fn is_mportp_0(&self) -> bool {
        *self == MPORTP8_A::MPORTP_0
    }
    #[doc = "Checks if the value of the field is `MPORTP_1`"]
    #[inline(always)]
    pub fn is_mportp_1(&self) -> bool {
        *self == MPORTP8_A::MPORTP_1
    }
}
#[doc = "Field `MPORTP8` writer - Mask bits for port pins"]
pub type MPORTP8_W<'a, const O: u8> = crate::BitWriter<'a, u32, MPIN0_SPEC, MPORTP8_A, O>;
impl<'a, const O: u8> MPORTP8_W<'a, O> {
    #[doc = "Read- pin is LOW and/or the corresponding bit in the MASK register is 1; write- clear output bit if the corresponding bit in the MASK register is 0"]
    #[inline(always)]
    pub fn mportp_0(self) -> &'a mut W {
        self.variant(MPORTP8_A::MPORTP_0)
    }
    #[doc = "Read- pin is HIGH and the corresponding bit in the MASK register is 0; write- set output bit if the corresponding bit in the MASK register is 0"]
    #[inline(always)]
    pub fn mportp_1(self) -> &'a mut W {
        self.variant(MPORTP8_A::MPORTP_1)
    }
}
#[doc = "Field `MPORTP9` reader - Mask bits for port pins"]
pub type MPORTP9_R = crate::BitReader<MPORTP9_A>;
#[doc = "Mask bits for port pins\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MPORTP9_A {
    #[doc = "0: Read- pin is LOW and/or the corresponding bit in the MASK register is 1; write- clear output bit if the corresponding bit in the MASK register is 0"]
    MPORTP_0 = 0,
    #[doc = "1: Read- pin is HIGH and the corresponding bit in the MASK register is 0; write- set output bit if the corresponding bit in the MASK register is 0"]
    MPORTP_1 = 1,
}
impl From<MPORTP9_A> for bool {
    #[inline(always)]
    fn from(variant: MPORTP9_A) -> Self {
        variant as u8 != 0
    }
}
impl MPORTP9_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MPORTP9_A {
        match self.bits {
            false => MPORTP9_A::MPORTP_0,
            true => MPORTP9_A::MPORTP_1,
        }
    }
    #[doc = "Checks if the value of the field is `MPORTP_0`"]
    #[inline(always)]
    pub fn is_mportp_0(&self) -> bool {
        *self == MPORTP9_A::MPORTP_0
    }
    #[doc = "Checks if the value of the field is `MPORTP_1`"]
    #[inline(always)]
    pub fn is_mportp_1(&self) -> bool {
        *self == MPORTP9_A::MPORTP_1
    }
}
#[doc = "Field `MPORTP9` writer - Mask bits for port pins"]
pub type MPORTP9_W<'a, const O: u8> = crate::BitWriter<'a, u32, MPIN0_SPEC, MPORTP9_A, O>;
impl<'a, const O: u8> MPORTP9_W<'a, O> {
    #[doc = "Read- pin is LOW and/or the corresponding bit in the MASK register is 1; write- clear output bit if the corresponding bit in the MASK register is 0"]
    #[inline(always)]
    pub fn mportp_0(self) -> &'a mut W {
        self.variant(MPORTP9_A::MPORTP_0)
    }
    #[doc = "Read- pin is HIGH and the corresponding bit in the MASK register is 0; write- set output bit if the corresponding bit in the MASK register is 0"]
    #[inline(always)]
    pub fn mportp_1(self) -> &'a mut W {
        self.variant(MPORTP9_A::MPORTP_1)
    }
}
#[doc = "Field `MPORTP10` reader - Mask bits for port pins"]
pub type MPORTP10_R = crate::BitReader<MPORTP10_A>;
#[doc = "Mask bits for port pins\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MPORTP10_A {
    #[doc = "0: Read- pin is LOW and/or the corresponding bit in the MASK register is 1; write- clear output bit if the corresponding bit in the MASK register is 0"]
    MPORTP_0 = 0,
    #[doc = "1: Read- pin is HIGH and the corresponding bit in the MASK register is 0; write- set output bit if the corresponding bit in the MASK register is 0"]
    MPORTP_1 = 1,
}
impl From<MPORTP10_A> for bool {
    #[inline(always)]
    fn from(variant: MPORTP10_A) -> Self {
        variant as u8 != 0
    }
}
impl MPORTP10_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MPORTP10_A {
        match self.bits {
            false => MPORTP10_A::MPORTP_0,
            true => MPORTP10_A::MPORTP_1,
        }
    }
    #[doc = "Checks if the value of the field is `MPORTP_0`"]
    #[inline(always)]
    pub fn is_mportp_0(&self) -> bool {
        *self == MPORTP10_A::MPORTP_0
    }
    #[doc = "Checks if the value of the field is `MPORTP_1`"]
    #[inline(always)]
    pub fn is_mportp_1(&self) -> bool {
        *self == MPORTP10_A::MPORTP_1
    }
}
#[doc = "Field `MPORTP10` writer - Mask bits for port pins"]
pub type MPORTP10_W<'a, const O: u8> = crate::BitWriter<'a, u32, MPIN0_SPEC, MPORTP10_A, O>;
impl<'a, const O: u8> MPORTP10_W<'a, O> {
    #[doc = "Read- pin is LOW and/or the corresponding bit in the MASK register is 1; write- clear output bit if the corresponding bit in the MASK register is 0"]
    #[inline(always)]
    pub fn mportp_0(self) -> &'a mut W {
        self.variant(MPORTP10_A::MPORTP_0)
    }
    #[doc = "Read- pin is HIGH and the corresponding bit in the MASK register is 0; write- set output bit if the corresponding bit in the MASK register is 0"]
    #[inline(always)]
    pub fn mportp_1(self) -> &'a mut W {
        self.variant(MPORTP10_A::MPORTP_1)
    }
}
#[doc = "Field `MPORTP11` reader - Mask bits for port pins"]
pub type MPORTP11_R = crate::BitReader<MPORTP11_A>;
#[doc = "Mask bits for port pins\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MPORTP11_A {
    #[doc = "0: Read- pin is LOW and/or the corresponding bit in the MASK register is 1; write- clear output bit if the corresponding bit in the MASK register is 0"]
    MPORTP_0 = 0,
    #[doc = "1: Read- pin is HIGH and the corresponding bit in the MASK register is 0; write- set output bit if the corresponding bit in the MASK register is 0"]
    MPORTP_1 = 1,
}
impl From<MPORTP11_A> for bool {
    #[inline(always)]
    fn from(variant: MPORTP11_A) -> Self {
        variant as u8 != 0
    }
}
impl MPORTP11_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MPORTP11_A {
        match self.bits {
            false => MPORTP11_A::MPORTP_0,
            true => MPORTP11_A::MPORTP_1,
        }
    }
    #[doc = "Checks if the value of the field is `MPORTP_0`"]
    #[inline(always)]
    pub fn is_mportp_0(&self) -> bool {
        *self == MPORTP11_A::MPORTP_0
    }
    #[doc = "Checks if the value of the field is `MPORTP_1`"]
    #[inline(always)]
    pub fn is_mportp_1(&self) -> bool {
        *self == MPORTP11_A::MPORTP_1
    }
}
#[doc = "Field `MPORTP11` writer - Mask bits for port pins"]
pub type MPORTP11_W<'a, const O: u8> = crate::BitWriter<'a, u32, MPIN0_SPEC, MPORTP11_A, O>;
impl<'a, const O: u8> MPORTP11_W<'a, O> {
    #[doc = "Read- pin is LOW and/or the corresponding bit in the MASK register is 1; write- clear output bit if the corresponding bit in the MASK register is 0"]
    #[inline(always)]
    pub fn mportp_0(self) -> &'a mut W {
        self.variant(MPORTP11_A::MPORTP_0)
    }
    #[doc = "Read- pin is HIGH and the corresponding bit in the MASK register is 0; write- set output bit if the corresponding bit in the MASK register is 0"]
    #[inline(always)]
    pub fn mportp_1(self) -> &'a mut W {
        self.variant(MPORTP11_A::MPORTP_1)
    }
}
#[doc = "Field `MPORTP12` reader - Mask bits for port pins"]
pub type MPORTP12_R = crate::BitReader<MPORTP12_A>;
#[doc = "Mask bits for port pins\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MPORTP12_A {
    #[doc = "0: Read- pin is LOW and/or the corresponding bit in the MASK register is 1; write- clear output bit if the corresponding bit in the MASK register is 0"]
    MPORTP_0 = 0,
    #[doc = "1: Read- pin is HIGH and the corresponding bit in the MASK register is 0; write- set output bit if the corresponding bit in the MASK register is 0"]
    MPORTP_1 = 1,
}
impl From<MPORTP12_A> for bool {
    #[inline(always)]
    fn from(variant: MPORTP12_A) -> Self {
        variant as u8 != 0
    }
}
impl MPORTP12_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MPORTP12_A {
        match self.bits {
            false => MPORTP12_A::MPORTP_0,
            true => MPORTP12_A::MPORTP_1,
        }
    }
    #[doc = "Checks if the value of the field is `MPORTP_0`"]
    #[inline(always)]
    pub fn is_mportp_0(&self) -> bool {
        *self == MPORTP12_A::MPORTP_0
    }
    #[doc = "Checks if the value of the field is `MPORTP_1`"]
    #[inline(always)]
    pub fn is_mportp_1(&self) -> bool {
        *self == MPORTP12_A::MPORTP_1
    }
}
#[doc = "Field `MPORTP12` writer - Mask bits for port pins"]
pub type MPORTP12_W<'a, const O: u8> = crate::BitWriter<'a, u32, MPIN0_SPEC, MPORTP12_A, O>;
impl<'a, const O: u8> MPORTP12_W<'a, O> {
    #[doc = "Read- pin is LOW and/or the corresponding bit in the MASK register is 1; write- clear output bit if the corresponding bit in the MASK register is 0"]
    #[inline(always)]
    pub fn mportp_0(self) -> &'a mut W {
        self.variant(MPORTP12_A::MPORTP_0)
    }
    #[doc = "Read- pin is HIGH and the corresponding bit in the MASK register is 0; write- set output bit if the corresponding bit in the MASK register is 0"]
    #[inline(always)]
    pub fn mportp_1(self) -> &'a mut W {
        self.variant(MPORTP12_A::MPORTP_1)
    }
}
#[doc = "Field `MPORTP13` reader - Mask bits for port pins"]
pub type MPORTP13_R = crate::BitReader<MPORTP13_A>;
#[doc = "Mask bits for port pins\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MPORTP13_A {
    #[doc = "0: Read- pin is LOW and/or the corresponding bit in the MASK register is 1; write- clear output bit if the corresponding bit in the MASK register is 0"]
    MPORTP_0 = 0,
    #[doc = "1: Read- pin is HIGH and the corresponding bit in the MASK register is 0; write- set output bit if the corresponding bit in the MASK register is 0"]
    MPORTP_1 = 1,
}
impl From<MPORTP13_A> for bool {
    #[inline(always)]
    fn from(variant: MPORTP13_A) -> Self {
        variant as u8 != 0
    }
}
impl MPORTP13_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MPORTP13_A {
        match self.bits {
            false => MPORTP13_A::MPORTP_0,
            true => MPORTP13_A::MPORTP_1,
        }
    }
    #[doc = "Checks if the value of the field is `MPORTP_0`"]
    #[inline(always)]
    pub fn is_mportp_0(&self) -> bool {
        *self == MPORTP13_A::MPORTP_0
    }
    #[doc = "Checks if the value of the field is `MPORTP_1`"]
    #[inline(always)]
    pub fn is_mportp_1(&self) -> bool {
        *self == MPORTP13_A::MPORTP_1
    }
}
#[doc = "Field `MPORTP13` writer - Mask bits for port pins"]
pub type MPORTP13_W<'a, const O: u8> = crate::BitWriter<'a, u32, MPIN0_SPEC, MPORTP13_A, O>;
impl<'a, const O: u8> MPORTP13_W<'a, O> {
    #[doc = "Read- pin is LOW and/or the corresponding bit in the MASK register is 1; write- clear output bit if the corresponding bit in the MASK register is 0"]
    #[inline(always)]
    pub fn mportp_0(self) -> &'a mut W {
        self.variant(MPORTP13_A::MPORTP_0)
    }
    #[doc = "Read- pin is HIGH and the corresponding bit in the MASK register is 0; write- set output bit if the corresponding bit in the MASK register is 0"]
    #[inline(always)]
    pub fn mportp_1(self) -> &'a mut W {
        self.variant(MPORTP13_A::MPORTP_1)
    }
}
#[doc = "Field `MPORTP14` reader - Mask bits for port pins"]
pub type MPORTP14_R = crate::BitReader<MPORTP14_A>;
#[doc = "Mask bits for port pins\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MPORTP14_A {
    #[doc = "0: Read- pin is LOW and/or the corresponding bit in the MASK register is 1; write- clear output bit if the corresponding bit in the MASK register is 0"]
    MPORTP_0 = 0,
    #[doc = "1: Read- pin is HIGH and the corresponding bit in the MASK register is 0; write- set output bit if the corresponding bit in the MASK register is 0"]
    MPORTP_1 = 1,
}
impl From<MPORTP14_A> for bool {
    #[inline(always)]
    fn from(variant: MPORTP14_A) -> Self {
        variant as u8 != 0
    }
}
impl MPORTP14_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MPORTP14_A {
        match self.bits {
            false => MPORTP14_A::MPORTP_0,
            true => MPORTP14_A::MPORTP_1,
        }
    }
    #[doc = "Checks if the value of the field is `MPORTP_0`"]
    #[inline(always)]
    pub fn is_mportp_0(&self) -> bool {
        *self == MPORTP14_A::MPORTP_0
    }
    #[doc = "Checks if the value of the field is `MPORTP_1`"]
    #[inline(always)]
    pub fn is_mportp_1(&self) -> bool {
        *self == MPORTP14_A::MPORTP_1
    }
}
#[doc = "Field `MPORTP14` writer - Mask bits for port pins"]
pub type MPORTP14_W<'a, const O: u8> = crate::BitWriter<'a, u32, MPIN0_SPEC, MPORTP14_A, O>;
impl<'a, const O: u8> MPORTP14_W<'a, O> {
    #[doc = "Read- pin is LOW and/or the corresponding bit in the MASK register is 1; write- clear output bit if the corresponding bit in the MASK register is 0"]
    #[inline(always)]
    pub fn mportp_0(self) -> &'a mut W {
        self.variant(MPORTP14_A::MPORTP_0)
    }
    #[doc = "Read- pin is HIGH and the corresponding bit in the MASK register is 0; write- set output bit if the corresponding bit in the MASK register is 0"]
    #[inline(always)]
    pub fn mportp_1(self) -> &'a mut W {
        self.variant(MPORTP14_A::MPORTP_1)
    }
}
#[doc = "Field `MPORTP15` reader - Mask bits for port pins"]
pub type MPORTP15_R = crate::BitReader<MPORTP15_A>;
#[doc = "Mask bits for port pins\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MPORTP15_A {
    #[doc = "0: Read- pin is LOW and/or the corresponding bit in the MASK register is 1; write- clear output bit if the corresponding bit in the MASK register is 0"]
    MPORTP_0 = 0,
    #[doc = "1: Read- pin is HIGH and the corresponding bit in the MASK register is 0; write- set output bit if the corresponding bit in the MASK register is 0"]
    MPORTP_1 = 1,
}
impl From<MPORTP15_A> for bool {
    #[inline(always)]
    fn from(variant: MPORTP15_A) -> Self {
        variant as u8 != 0
    }
}
impl MPORTP15_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MPORTP15_A {
        match self.bits {
            false => MPORTP15_A::MPORTP_0,
            true => MPORTP15_A::MPORTP_1,
        }
    }
    #[doc = "Checks if the value of the field is `MPORTP_0`"]
    #[inline(always)]
    pub fn is_mportp_0(&self) -> bool {
        *self == MPORTP15_A::MPORTP_0
    }
    #[doc = "Checks if the value of the field is `MPORTP_1`"]
    #[inline(always)]
    pub fn is_mportp_1(&self) -> bool {
        *self == MPORTP15_A::MPORTP_1
    }
}
#[doc = "Field `MPORTP15` writer - Mask bits for port pins"]
pub type MPORTP15_W<'a, const O: u8> = crate::BitWriter<'a, u32, MPIN0_SPEC, MPORTP15_A, O>;
impl<'a, const O: u8> MPORTP15_W<'a, O> {
    #[doc = "Read- pin is LOW and/or the corresponding bit in the MASK register is 1; write- clear output bit if the corresponding bit in the MASK register is 0"]
    #[inline(always)]
    pub fn mportp_0(self) -> &'a mut W {
        self.variant(MPORTP15_A::MPORTP_0)
    }
    #[doc = "Read- pin is HIGH and the corresponding bit in the MASK register is 0; write- set output bit if the corresponding bit in the MASK register is 0"]
    #[inline(always)]
    pub fn mportp_1(self) -> &'a mut W {
        self.variant(MPORTP15_A::MPORTP_1)
    }
}
#[doc = "Field `MPORTP16` reader - Mask bits for port pins"]
pub type MPORTP16_R = crate::BitReader<MPORTP16_A>;
#[doc = "Mask bits for port pins\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MPORTP16_A {
    #[doc = "0: Read- pin is LOW and/or the corresponding bit in the MASK register is 1; write- clear output bit if the corresponding bit in the MASK register is 0"]
    MPORTP_0 = 0,
    #[doc = "1: Read- pin is HIGH and the corresponding bit in the MASK register is 0; write- set output bit if the corresponding bit in the MASK register is 0"]
    MPORTP_1 = 1,
}
impl From<MPORTP16_A> for bool {
    #[inline(always)]
    fn from(variant: MPORTP16_A) -> Self {
        variant as u8 != 0
    }
}
impl MPORTP16_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MPORTP16_A {
        match self.bits {
            false => MPORTP16_A::MPORTP_0,
            true => MPORTP16_A::MPORTP_1,
        }
    }
    #[doc = "Checks if the value of the field is `MPORTP_0`"]
    #[inline(always)]
    pub fn is_mportp_0(&self) -> bool {
        *self == MPORTP16_A::MPORTP_0
    }
    #[doc = "Checks if the value of the field is `MPORTP_1`"]
    #[inline(always)]
    pub fn is_mportp_1(&self) -> bool {
        *self == MPORTP16_A::MPORTP_1
    }
}
#[doc = "Field `MPORTP16` writer - Mask bits for port pins"]
pub type MPORTP16_W<'a, const O: u8> = crate::BitWriter<'a, u32, MPIN0_SPEC, MPORTP16_A, O>;
impl<'a, const O: u8> MPORTP16_W<'a, O> {
    #[doc = "Read- pin is LOW and/or the corresponding bit in the MASK register is 1; write- clear output bit if the corresponding bit in the MASK register is 0"]
    #[inline(always)]
    pub fn mportp_0(self) -> &'a mut W {
        self.variant(MPORTP16_A::MPORTP_0)
    }
    #[doc = "Read- pin is HIGH and the corresponding bit in the MASK register is 0; write- set output bit if the corresponding bit in the MASK register is 0"]
    #[inline(always)]
    pub fn mportp_1(self) -> &'a mut W {
        self.variant(MPORTP16_A::MPORTP_1)
    }
}
#[doc = "Field `MPORTP17` reader - Mask bits for port pins"]
pub type MPORTP17_R = crate::BitReader<MPORTP17_A>;
#[doc = "Mask bits for port pins\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MPORTP17_A {
    #[doc = "0: Read- pin is LOW and/or the corresponding bit in the MASK register is 1; write- clear output bit if the corresponding bit in the MASK register is 0"]
    MPORTP_0 = 0,
    #[doc = "1: Read- pin is HIGH and the corresponding bit in the MASK register is 0; write- set output bit if the corresponding bit in the MASK register is 0"]
    MPORTP_1 = 1,
}
impl From<MPORTP17_A> for bool {
    #[inline(always)]
    fn from(variant: MPORTP17_A) -> Self {
        variant as u8 != 0
    }
}
impl MPORTP17_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MPORTP17_A {
        match self.bits {
            false => MPORTP17_A::MPORTP_0,
            true => MPORTP17_A::MPORTP_1,
        }
    }
    #[doc = "Checks if the value of the field is `MPORTP_0`"]
    #[inline(always)]
    pub fn is_mportp_0(&self) -> bool {
        *self == MPORTP17_A::MPORTP_0
    }
    #[doc = "Checks if the value of the field is `MPORTP_1`"]
    #[inline(always)]
    pub fn is_mportp_1(&self) -> bool {
        *self == MPORTP17_A::MPORTP_1
    }
}
#[doc = "Field `MPORTP17` writer - Mask bits for port pins"]
pub type MPORTP17_W<'a, const O: u8> = crate::BitWriter<'a, u32, MPIN0_SPEC, MPORTP17_A, O>;
impl<'a, const O: u8> MPORTP17_W<'a, O> {
    #[doc = "Read- pin is LOW and/or the corresponding bit in the MASK register is 1; write- clear output bit if the corresponding bit in the MASK register is 0"]
    #[inline(always)]
    pub fn mportp_0(self) -> &'a mut W {
        self.variant(MPORTP17_A::MPORTP_0)
    }
    #[doc = "Read- pin is HIGH and the corresponding bit in the MASK register is 0; write- set output bit if the corresponding bit in the MASK register is 0"]
    #[inline(always)]
    pub fn mportp_1(self) -> &'a mut W {
        self.variant(MPORTP17_A::MPORTP_1)
    }
}
#[doc = "Field `MPORTP18` reader - Mask bits for port pins"]
pub type MPORTP18_R = crate::BitReader<MPORTP18_A>;
#[doc = "Mask bits for port pins\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MPORTP18_A {
    #[doc = "0: Read- pin is LOW and/or the corresponding bit in the MASK register is 1; write- clear output bit if the corresponding bit in the MASK register is 0"]
    MPORTP_0 = 0,
    #[doc = "1: Read- pin is HIGH and the corresponding bit in the MASK register is 0; write- set output bit if the corresponding bit in the MASK register is 0"]
    MPORTP_1 = 1,
}
impl From<MPORTP18_A> for bool {
    #[inline(always)]
    fn from(variant: MPORTP18_A) -> Self {
        variant as u8 != 0
    }
}
impl MPORTP18_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MPORTP18_A {
        match self.bits {
            false => MPORTP18_A::MPORTP_0,
            true => MPORTP18_A::MPORTP_1,
        }
    }
    #[doc = "Checks if the value of the field is `MPORTP_0`"]
    #[inline(always)]
    pub fn is_mportp_0(&self) -> bool {
        *self == MPORTP18_A::MPORTP_0
    }
    #[doc = "Checks if the value of the field is `MPORTP_1`"]
    #[inline(always)]
    pub fn is_mportp_1(&self) -> bool {
        *self == MPORTP18_A::MPORTP_1
    }
}
#[doc = "Field `MPORTP18` writer - Mask bits for port pins"]
pub type MPORTP18_W<'a, const O: u8> = crate::BitWriter<'a, u32, MPIN0_SPEC, MPORTP18_A, O>;
impl<'a, const O: u8> MPORTP18_W<'a, O> {
    #[doc = "Read- pin is LOW and/or the corresponding bit in the MASK register is 1; write- clear output bit if the corresponding bit in the MASK register is 0"]
    #[inline(always)]
    pub fn mportp_0(self) -> &'a mut W {
        self.variant(MPORTP18_A::MPORTP_0)
    }
    #[doc = "Read- pin is HIGH and the corresponding bit in the MASK register is 0; write- set output bit if the corresponding bit in the MASK register is 0"]
    #[inline(always)]
    pub fn mportp_1(self) -> &'a mut W {
        self.variant(MPORTP18_A::MPORTP_1)
    }
}
#[doc = "Field `MPORTP19` reader - Mask bits for port pins"]
pub type MPORTP19_R = crate::BitReader<MPORTP19_A>;
#[doc = "Mask bits for port pins\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MPORTP19_A {
    #[doc = "0: Read- pin is LOW and/or the corresponding bit in the MASK register is 1; write- clear output bit if the corresponding bit in the MASK register is 0"]
    MPORTP_0 = 0,
    #[doc = "1: Read- pin is HIGH and the corresponding bit in the MASK register is 0; write- set output bit if the corresponding bit in the MASK register is 0"]
    MPORTP_1 = 1,
}
impl From<MPORTP19_A> for bool {
    #[inline(always)]
    fn from(variant: MPORTP19_A) -> Self {
        variant as u8 != 0
    }
}
impl MPORTP19_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MPORTP19_A {
        match self.bits {
            false => MPORTP19_A::MPORTP_0,
            true => MPORTP19_A::MPORTP_1,
        }
    }
    #[doc = "Checks if the value of the field is `MPORTP_0`"]
    #[inline(always)]
    pub fn is_mportp_0(&self) -> bool {
        *self == MPORTP19_A::MPORTP_0
    }
    #[doc = "Checks if the value of the field is `MPORTP_1`"]
    #[inline(always)]
    pub fn is_mportp_1(&self) -> bool {
        *self == MPORTP19_A::MPORTP_1
    }
}
#[doc = "Field `MPORTP19` writer - Mask bits for port pins"]
pub type MPORTP19_W<'a, const O: u8> = crate::BitWriter<'a, u32, MPIN0_SPEC, MPORTP19_A, O>;
impl<'a, const O: u8> MPORTP19_W<'a, O> {
    #[doc = "Read- pin is LOW and/or the corresponding bit in the MASK register is 1; write- clear output bit if the corresponding bit in the MASK register is 0"]
    #[inline(always)]
    pub fn mportp_0(self) -> &'a mut W {
        self.variant(MPORTP19_A::MPORTP_0)
    }
    #[doc = "Read- pin is HIGH and the corresponding bit in the MASK register is 0; write- set output bit if the corresponding bit in the MASK register is 0"]
    #[inline(always)]
    pub fn mportp_1(self) -> &'a mut W {
        self.variant(MPORTP19_A::MPORTP_1)
    }
}
#[doc = "Field `MPORTP20` reader - Mask bits for port pins"]
pub type MPORTP20_R = crate::BitReader<MPORTP20_A>;
#[doc = "Mask bits for port pins\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MPORTP20_A {
    #[doc = "0: Read- pin is LOW and/or the corresponding bit in the MASK register is 1; write- clear output bit if the corresponding bit in the MASK register is 0"]
    MPORTP_0 = 0,
    #[doc = "1: Read- pin is HIGH and the corresponding bit in the MASK register is 0; write- set output bit if the corresponding bit in the MASK register is 0"]
    MPORTP_1 = 1,
}
impl From<MPORTP20_A> for bool {
    #[inline(always)]
    fn from(variant: MPORTP20_A) -> Self {
        variant as u8 != 0
    }
}
impl MPORTP20_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MPORTP20_A {
        match self.bits {
            false => MPORTP20_A::MPORTP_0,
            true => MPORTP20_A::MPORTP_1,
        }
    }
    #[doc = "Checks if the value of the field is `MPORTP_0`"]
    #[inline(always)]
    pub fn is_mportp_0(&self) -> bool {
        *self == MPORTP20_A::MPORTP_0
    }
    #[doc = "Checks if the value of the field is `MPORTP_1`"]
    #[inline(always)]
    pub fn is_mportp_1(&self) -> bool {
        *self == MPORTP20_A::MPORTP_1
    }
}
#[doc = "Field `MPORTP20` writer - Mask bits for port pins"]
pub type MPORTP20_W<'a, const O: u8> = crate::BitWriter<'a, u32, MPIN0_SPEC, MPORTP20_A, O>;
impl<'a, const O: u8> MPORTP20_W<'a, O> {
    #[doc = "Read- pin is LOW and/or the corresponding bit in the MASK register is 1; write- clear output bit if the corresponding bit in the MASK register is 0"]
    #[inline(always)]
    pub fn mportp_0(self) -> &'a mut W {
        self.variant(MPORTP20_A::MPORTP_0)
    }
    #[doc = "Read- pin is HIGH and the corresponding bit in the MASK register is 0; write- set output bit if the corresponding bit in the MASK register is 0"]
    #[inline(always)]
    pub fn mportp_1(self) -> &'a mut W {
        self.variant(MPORTP20_A::MPORTP_1)
    }
}
#[doc = "Field `MPORTP21` reader - Mask bits for port pins"]
pub type MPORTP21_R = crate::BitReader<MPORTP21_A>;
#[doc = "Mask bits for port pins\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MPORTP21_A {
    #[doc = "0: Read- pin is LOW and/or the corresponding bit in the MASK register is 1; write- clear output bit if the corresponding bit in the MASK register is 0"]
    MPORTP_0 = 0,
    #[doc = "1: Read- pin is HIGH and the corresponding bit in the MASK register is 0; write- set output bit if the corresponding bit in the MASK register is 0"]
    MPORTP_1 = 1,
}
impl From<MPORTP21_A> for bool {
    #[inline(always)]
    fn from(variant: MPORTP21_A) -> Self {
        variant as u8 != 0
    }
}
impl MPORTP21_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MPORTP21_A {
        match self.bits {
            false => MPORTP21_A::MPORTP_0,
            true => MPORTP21_A::MPORTP_1,
        }
    }
    #[doc = "Checks if the value of the field is `MPORTP_0`"]
    #[inline(always)]
    pub fn is_mportp_0(&self) -> bool {
        *self == MPORTP21_A::MPORTP_0
    }
    #[doc = "Checks if the value of the field is `MPORTP_1`"]
    #[inline(always)]
    pub fn is_mportp_1(&self) -> bool {
        *self == MPORTP21_A::MPORTP_1
    }
}
#[doc = "Field `MPORTP21` writer - Mask bits for port pins"]
pub type MPORTP21_W<'a, const O: u8> = crate::BitWriter<'a, u32, MPIN0_SPEC, MPORTP21_A, O>;
impl<'a, const O: u8> MPORTP21_W<'a, O> {
    #[doc = "Read- pin is LOW and/or the corresponding bit in the MASK register is 1; write- clear output bit if the corresponding bit in the MASK register is 0"]
    #[inline(always)]
    pub fn mportp_0(self) -> &'a mut W {
        self.variant(MPORTP21_A::MPORTP_0)
    }
    #[doc = "Read- pin is HIGH and the corresponding bit in the MASK register is 0; write- set output bit if the corresponding bit in the MASK register is 0"]
    #[inline(always)]
    pub fn mportp_1(self) -> &'a mut W {
        self.variant(MPORTP21_A::MPORTP_1)
    }
}
#[doc = "Field `MPORTP22` reader - Mask bits for port pins"]
pub type MPORTP22_R = crate::BitReader<MPORTP22_A>;
#[doc = "Mask bits for port pins\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MPORTP22_A {
    #[doc = "0: Read- pin is LOW and/or the corresponding bit in the MASK register is 1; write- clear output bit if the corresponding bit in the MASK register is 0"]
    MPORTP_0 = 0,
    #[doc = "1: Read- pin is HIGH and the corresponding bit in the MASK register is 0; write- set output bit if the corresponding bit in the MASK register is 0"]
    MPORTP_1 = 1,
}
impl From<MPORTP22_A> for bool {
    #[inline(always)]
    fn from(variant: MPORTP22_A) -> Self {
        variant as u8 != 0
    }
}
impl MPORTP22_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MPORTP22_A {
        match self.bits {
            false => MPORTP22_A::MPORTP_0,
            true => MPORTP22_A::MPORTP_1,
        }
    }
    #[doc = "Checks if the value of the field is `MPORTP_0`"]
    #[inline(always)]
    pub fn is_mportp_0(&self) -> bool {
        *self == MPORTP22_A::MPORTP_0
    }
    #[doc = "Checks if the value of the field is `MPORTP_1`"]
    #[inline(always)]
    pub fn is_mportp_1(&self) -> bool {
        *self == MPORTP22_A::MPORTP_1
    }
}
#[doc = "Field `MPORTP22` writer - Mask bits for port pins"]
pub type MPORTP22_W<'a, const O: u8> = crate::BitWriter<'a, u32, MPIN0_SPEC, MPORTP22_A, O>;
impl<'a, const O: u8> MPORTP22_W<'a, O> {
    #[doc = "Read- pin is LOW and/or the corresponding bit in the MASK register is 1; write- clear output bit if the corresponding bit in the MASK register is 0"]
    #[inline(always)]
    pub fn mportp_0(self) -> &'a mut W {
        self.variant(MPORTP22_A::MPORTP_0)
    }
    #[doc = "Read- pin is HIGH and the corresponding bit in the MASK register is 0; write- set output bit if the corresponding bit in the MASK register is 0"]
    #[inline(always)]
    pub fn mportp_1(self) -> &'a mut W {
        self.variant(MPORTP22_A::MPORTP_1)
    }
}
#[doc = "Field `MPORTP23` reader - Mask bits for port pins"]
pub type MPORTP23_R = crate::BitReader<MPORTP23_A>;
#[doc = "Mask bits for port pins\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MPORTP23_A {
    #[doc = "0: Read- pin is LOW and/or the corresponding bit in the MASK register is 1; write- clear output bit if the corresponding bit in the MASK register is 0"]
    MPORTP_0 = 0,
    #[doc = "1: Read- pin is HIGH and the corresponding bit in the MASK register is 0; write- set output bit if the corresponding bit in the MASK register is 0"]
    MPORTP_1 = 1,
}
impl From<MPORTP23_A> for bool {
    #[inline(always)]
    fn from(variant: MPORTP23_A) -> Self {
        variant as u8 != 0
    }
}
impl MPORTP23_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MPORTP23_A {
        match self.bits {
            false => MPORTP23_A::MPORTP_0,
            true => MPORTP23_A::MPORTP_1,
        }
    }
    #[doc = "Checks if the value of the field is `MPORTP_0`"]
    #[inline(always)]
    pub fn is_mportp_0(&self) -> bool {
        *self == MPORTP23_A::MPORTP_0
    }
    #[doc = "Checks if the value of the field is `MPORTP_1`"]
    #[inline(always)]
    pub fn is_mportp_1(&self) -> bool {
        *self == MPORTP23_A::MPORTP_1
    }
}
#[doc = "Field `MPORTP23` writer - Mask bits for port pins"]
pub type MPORTP23_W<'a, const O: u8> = crate::BitWriter<'a, u32, MPIN0_SPEC, MPORTP23_A, O>;
impl<'a, const O: u8> MPORTP23_W<'a, O> {
    #[doc = "Read- pin is LOW and/or the corresponding bit in the MASK register is 1; write- clear output bit if the corresponding bit in the MASK register is 0"]
    #[inline(always)]
    pub fn mportp_0(self) -> &'a mut W {
        self.variant(MPORTP23_A::MPORTP_0)
    }
    #[doc = "Read- pin is HIGH and the corresponding bit in the MASK register is 0; write- set output bit if the corresponding bit in the MASK register is 0"]
    #[inline(always)]
    pub fn mportp_1(self) -> &'a mut W {
        self.variant(MPORTP23_A::MPORTP_1)
    }
}
#[doc = "Field `MPORTP24` reader - Mask bits for port pins"]
pub type MPORTP24_R = crate::BitReader<MPORTP24_A>;
#[doc = "Mask bits for port pins\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MPORTP24_A {
    #[doc = "0: Read- pin is LOW and/or the corresponding bit in the MASK register is 1; write- clear output bit if the corresponding bit in the MASK register is 0"]
    MPORTP_0 = 0,
    #[doc = "1: Read- pin is HIGH and the corresponding bit in the MASK register is 0; write- set output bit if the corresponding bit in the MASK register is 0"]
    MPORTP_1 = 1,
}
impl From<MPORTP24_A> for bool {
    #[inline(always)]
    fn from(variant: MPORTP24_A) -> Self {
        variant as u8 != 0
    }
}
impl MPORTP24_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MPORTP24_A {
        match self.bits {
            false => MPORTP24_A::MPORTP_0,
            true => MPORTP24_A::MPORTP_1,
        }
    }
    #[doc = "Checks if the value of the field is `MPORTP_0`"]
    #[inline(always)]
    pub fn is_mportp_0(&self) -> bool {
        *self == MPORTP24_A::MPORTP_0
    }
    #[doc = "Checks if the value of the field is `MPORTP_1`"]
    #[inline(always)]
    pub fn is_mportp_1(&self) -> bool {
        *self == MPORTP24_A::MPORTP_1
    }
}
#[doc = "Field `MPORTP24` writer - Mask bits for port pins"]
pub type MPORTP24_W<'a, const O: u8> = crate::BitWriter<'a, u32, MPIN0_SPEC, MPORTP24_A, O>;
impl<'a, const O: u8> MPORTP24_W<'a, O> {
    #[doc = "Read- pin is LOW and/or the corresponding bit in the MASK register is 1; write- clear output bit if the corresponding bit in the MASK register is 0"]
    #[inline(always)]
    pub fn mportp_0(self) -> &'a mut W {
        self.variant(MPORTP24_A::MPORTP_0)
    }
    #[doc = "Read- pin is HIGH and the corresponding bit in the MASK register is 0; write- set output bit if the corresponding bit in the MASK register is 0"]
    #[inline(always)]
    pub fn mportp_1(self) -> &'a mut W {
        self.variant(MPORTP24_A::MPORTP_1)
    }
}
#[doc = "Field `MPORTP25` reader - Mask bits for port pins"]
pub type MPORTP25_R = crate::BitReader<MPORTP25_A>;
#[doc = "Mask bits for port pins\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MPORTP25_A {
    #[doc = "0: Read- pin is LOW and/or the corresponding bit in the MASK register is 1; write- clear output bit if the corresponding bit in the MASK register is 0"]
    MPORTP_0 = 0,
    #[doc = "1: Read- pin is HIGH and the corresponding bit in the MASK register is 0; write- set output bit if the corresponding bit in the MASK register is 0"]
    MPORTP_1 = 1,
}
impl From<MPORTP25_A> for bool {
    #[inline(always)]
    fn from(variant: MPORTP25_A) -> Self {
        variant as u8 != 0
    }
}
impl MPORTP25_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MPORTP25_A {
        match self.bits {
            false => MPORTP25_A::MPORTP_0,
            true => MPORTP25_A::MPORTP_1,
        }
    }
    #[doc = "Checks if the value of the field is `MPORTP_0`"]
    #[inline(always)]
    pub fn is_mportp_0(&self) -> bool {
        *self == MPORTP25_A::MPORTP_0
    }
    #[doc = "Checks if the value of the field is `MPORTP_1`"]
    #[inline(always)]
    pub fn is_mportp_1(&self) -> bool {
        *self == MPORTP25_A::MPORTP_1
    }
}
#[doc = "Field `MPORTP25` writer - Mask bits for port pins"]
pub type MPORTP25_W<'a, const O: u8> = crate::BitWriter<'a, u32, MPIN0_SPEC, MPORTP25_A, O>;
impl<'a, const O: u8> MPORTP25_W<'a, O> {
    #[doc = "Read- pin is LOW and/or the corresponding bit in the MASK register is 1; write- clear output bit if the corresponding bit in the MASK register is 0"]
    #[inline(always)]
    pub fn mportp_0(self) -> &'a mut W {
        self.variant(MPORTP25_A::MPORTP_0)
    }
    #[doc = "Read- pin is HIGH and the corresponding bit in the MASK register is 0; write- set output bit if the corresponding bit in the MASK register is 0"]
    #[inline(always)]
    pub fn mportp_1(self) -> &'a mut W {
        self.variant(MPORTP25_A::MPORTP_1)
    }
}
#[doc = "Field `MPORTP26` reader - Mask bits for port pins"]
pub type MPORTP26_R = crate::BitReader<MPORTP26_A>;
#[doc = "Mask bits for port pins\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MPORTP26_A {
    #[doc = "0: Read- pin is LOW and/or the corresponding bit in the MASK register is 1; write- clear output bit if the corresponding bit in the MASK register is 0"]
    MPORTP_0 = 0,
    #[doc = "1: Read- pin is HIGH and the corresponding bit in the MASK register is 0; write- set output bit if the corresponding bit in the MASK register is 0"]
    MPORTP_1 = 1,
}
impl From<MPORTP26_A> for bool {
    #[inline(always)]
    fn from(variant: MPORTP26_A) -> Self {
        variant as u8 != 0
    }
}
impl MPORTP26_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MPORTP26_A {
        match self.bits {
            false => MPORTP26_A::MPORTP_0,
            true => MPORTP26_A::MPORTP_1,
        }
    }
    #[doc = "Checks if the value of the field is `MPORTP_0`"]
    #[inline(always)]
    pub fn is_mportp_0(&self) -> bool {
        *self == MPORTP26_A::MPORTP_0
    }
    #[doc = "Checks if the value of the field is `MPORTP_1`"]
    #[inline(always)]
    pub fn is_mportp_1(&self) -> bool {
        *self == MPORTP26_A::MPORTP_1
    }
}
#[doc = "Field `MPORTP26` writer - Mask bits for port pins"]
pub type MPORTP26_W<'a, const O: u8> = crate::BitWriter<'a, u32, MPIN0_SPEC, MPORTP26_A, O>;
impl<'a, const O: u8> MPORTP26_W<'a, O> {
    #[doc = "Read- pin is LOW and/or the corresponding bit in the MASK register is 1; write- clear output bit if the corresponding bit in the MASK register is 0"]
    #[inline(always)]
    pub fn mportp_0(self) -> &'a mut W {
        self.variant(MPORTP26_A::MPORTP_0)
    }
    #[doc = "Read- pin is HIGH and the corresponding bit in the MASK register is 0; write- set output bit if the corresponding bit in the MASK register is 0"]
    #[inline(always)]
    pub fn mportp_1(self) -> &'a mut W {
        self.variant(MPORTP26_A::MPORTP_1)
    }
}
#[doc = "Field `MPORTP27` reader - Mask bits for port pins"]
pub type MPORTP27_R = crate::BitReader<MPORTP27_A>;
#[doc = "Mask bits for port pins\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MPORTP27_A {
    #[doc = "0: Read- pin is LOW and/or the corresponding bit in the MASK register is 1; write- clear output bit if the corresponding bit in the MASK register is 0"]
    MPORTP_0 = 0,
    #[doc = "1: Read- pin is HIGH and the corresponding bit in the MASK register is 0; write- set output bit if the corresponding bit in the MASK register is 0"]
    MPORTP_1 = 1,
}
impl From<MPORTP27_A> for bool {
    #[inline(always)]
    fn from(variant: MPORTP27_A) -> Self {
        variant as u8 != 0
    }
}
impl MPORTP27_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MPORTP27_A {
        match self.bits {
            false => MPORTP27_A::MPORTP_0,
            true => MPORTP27_A::MPORTP_1,
        }
    }
    #[doc = "Checks if the value of the field is `MPORTP_0`"]
    #[inline(always)]
    pub fn is_mportp_0(&self) -> bool {
        *self == MPORTP27_A::MPORTP_0
    }
    #[doc = "Checks if the value of the field is `MPORTP_1`"]
    #[inline(always)]
    pub fn is_mportp_1(&self) -> bool {
        *self == MPORTP27_A::MPORTP_1
    }
}
#[doc = "Field `MPORTP27` writer - Mask bits for port pins"]
pub type MPORTP27_W<'a, const O: u8> = crate::BitWriter<'a, u32, MPIN0_SPEC, MPORTP27_A, O>;
impl<'a, const O: u8> MPORTP27_W<'a, O> {
    #[doc = "Read- pin is LOW and/or the corresponding bit in the MASK register is 1; write- clear output bit if the corresponding bit in the MASK register is 0"]
    #[inline(always)]
    pub fn mportp_0(self) -> &'a mut W {
        self.variant(MPORTP27_A::MPORTP_0)
    }
    #[doc = "Read- pin is HIGH and the corresponding bit in the MASK register is 0; write- set output bit if the corresponding bit in the MASK register is 0"]
    #[inline(always)]
    pub fn mportp_1(self) -> &'a mut W {
        self.variant(MPORTP27_A::MPORTP_1)
    }
}
#[doc = "Field `MPORTP28` reader - Mask bits for port pins"]
pub type MPORTP28_R = crate::BitReader<MPORTP28_A>;
#[doc = "Mask bits for port pins\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MPORTP28_A {
    #[doc = "0: Read- pin is LOW and/or the corresponding bit in the MASK register is 1; write- clear output bit if the corresponding bit in the MASK register is 0"]
    MPORTP_0 = 0,
    #[doc = "1: Read- pin is HIGH and the corresponding bit in the MASK register is 0; write- set output bit if the corresponding bit in the MASK register is 0"]
    MPORTP_1 = 1,
}
impl From<MPORTP28_A> for bool {
    #[inline(always)]
    fn from(variant: MPORTP28_A) -> Self {
        variant as u8 != 0
    }
}
impl MPORTP28_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MPORTP28_A {
        match self.bits {
            false => MPORTP28_A::MPORTP_0,
            true => MPORTP28_A::MPORTP_1,
        }
    }
    #[doc = "Checks if the value of the field is `MPORTP_0`"]
    #[inline(always)]
    pub fn is_mportp_0(&self) -> bool {
        *self == MPORTP28_A::MPORTP_0
    }
    #[doc = "Checks if the value of the field is `MPORTP_1`"]
    #[inline(always)]
    pub fn is_mportp_1(&self) -> bool {
        *self == MPORTP28_A::MPORTP_1
    }
}
#[doc = "Field `MPORTP28` writer - Mask bits for port pins"]
pub type MPORTP28_W<'a, const O: u8> = crate::BitWriter<'a, u32, MPIN0_SPEC, MPORTP28_A, O>;
impl<'a, const O: u8> MPORTP28_W<'a, O> {
    #[doc = "Read- pin is LOW and/or the corresponding bit in the MASK register is 1; write- clear output bit if the corresponding bit in the MASK register is 0"]
    #[inline(always)]
    pub fn mportp_0(self) -> &'a mut W {
        self.variant(MPORTP28_A::MPORTP_0)
    }
    #[doc = "Read- pin is HIGH and the corresponding bit in the MASK register is 0; write- set output bit if the corresponding bit in the MASK register is 0"]
    #[inline(always)]
    pub fn mportp_1(self) -> &'a mut W {
        self.variant(MPORTP28_A::MPORTP_1)
    }
}
#[doc = "Field `MPORTP29` reader - Mask bits for port pins"]
pub type MPORTP29_R = crate::BitReader<MPORTP29_A>;
#[doc = "Mask bits for port pins\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MPORTP29_A {
    #[doc = "0: Read- pin is LOW and/or the corresponding bit in the MASK register is 1; write- clear output bit if the corresponding bit in the MASK register is 0"]
    MPORTP_0 = 0,
    #[doc = "1: Read- pin is HIGH and the corresponding bit in the MASK register is 0; write- set output bit if the corresponding bit in the MASK register is 0"]
    MPORTP_1 = 1,
}
impl From<MPORTP29_A> for bool {
    #[inline(always)]
    fn from(variant: MPORTP29_A) -> Self {
        variant as u8 != 0
    }
}
impl MPORTP29_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MPORTP29_A {
        match self.bits {
            false => MPORTP29_A::MPORTP_0,
            true => MPORTP29_A::MPORTP_1,
        }
    }
    #[doc = "Checks if the value of the field is `MPORTP_0`"]
    #[inline(always)]
    pub fn is_mportp_0(&self) -> bool {
        *self == MPORTP29_A::MPORTP_0
    }
    #[doc = "Checks if the value of the field is `MPORTP_1`"]
    #[inline(always)]
    pub fn is_mportp_1(&self) -> bool {
        *self == MPORTP29_A::MPORTP_1
    }
}
#[doc = "Field `MPORTP29` writer - Mask bits for port pins"]
pub type MPORTP29_W<'a, const O: u8> = crate::BitWriter<'a, u32, MPIN0_SPEC, MPORTP29_A, O>;
impl<'a, const O: u8> MPORTP29_W<'a, O> {
    #[doc = "Read- pin is LOW and/or the corresponding bit in the MASK register is 1; write- clear output bit if the corresponding bit in the MASK register is 0"]
    #[inline(always)]
    pub fn mportp_0(self) -> &'a mut W {
        self.variant(MPORTP29_A::MPORTP_0)
    }
    #[doc = "Read- pin is HIGH and the corresponding bit in the MASK register is 0; write- set output bit if the corresponding bit in the MASK register is 0"]
    #[inline(always)]
    pub fn mportp_1(self) -> &'a mut W {
        self.variant(MPORTP29_A::MPORTP_1)
    }
}
#[doc = "Field `MPORTP30` reader - Mask bits for port pins"]
pub type MPORTP30_R = crate::BitReader<MPORTP30_A>;
#[doc = "Mask bits for port pins\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MPORTP30_A {
    #[doc = "0: Read- pin is LOW and/or the corresponding bit in the MASK register is 1; write- clear output bit if the corresponding bit in the MASK register is 0"]
    MPORTP_0 = 0,
    #[doc = "1: Read- pin is HIGH and the corresponding bit in the MASK register is 0; write- set output bit if the corresponding bit in the MASK register is 0"]
    MPORTP_1 = 1,
}
impl From<MPORTP30_A> for bool {
    #[inline(always)]
    fn from(variant: MPORTP30_A) -> Self {
        variant as u8 != 0
    }
}
impl MPORTP30_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MPORTP30_A {
        match self.bits {
            false => MPORTP30_A::MPORTP_0,
            true => MPORTP30_A::MPORTP_1,
        }
    }
    #[doc = "Checks if the value of the field is `MPORTP_0`"]
    #[inline(always)]
    pub fn is_mportp_0(&self) -> bool {
        *self == MPORTP30_A::MPORTP_0
    }
    #[doc = "Checks if the value of the field is `MPORTP_1`"]
    #[inline(always)]
    pub fn is_mportp_1(&self) -> bool {
        *self == MPORTP30_A::MPORTP_1
    }
}
#[doc = "Field `MPORTP30` writer - Mask bits for port pins"]
pub type MPORTP30_W<'a, const O: u8> = crate::BitWriter<'a, u32, MPIN0_SPEC, MPORTP30_A, O>;
impl<'a, const O: u8> MPORTP30_W<'a, O> {
    #[doc = "Read- pin is LOW and/or the corresponding bit in the MASK register is 1; write- clear output bit if the corresponding bit in the MASK register is 0"]
    #[inline(always)]
    pub fn mportp_0(self) -> &'a mut W {
        self.variant(MPORTP30_A::MPORTP_0)
    }
    #[doc = "Read- pin is HIGH and the corresponding bit in the MASK register is 0; write- set output bit if the corresponding bit in the MASK register is 0"]
    #[inline(always)]
    pub fn mportp_1(self) -> &'a mut W {
        self.variant(MPORTP30_A::MPORTP_1)
    }
}
#[doc = "Field `MPORTP31` reader - Mask bits for port pins"]
pub type MPORTP31_R = crate::BitReader<MPORTP31_A>;
#[doc = "Mask bits for port pins\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MPORTP31_A {
    #[doc = "0: Read- pin is LOW and/or the corresponding bit in the MASK register is 1; write- clear output bit if the corresponding bit in the MASK register is 0"]
    MPORTP_0 = 0,
    #[doc = "1: Read- pin is HIGH and the corresponding bit in the MASK register is 0; write- set output bit if the corresponding bit in the MASK register is 0"]
    MPORTP_1 = 1,
}
impl From<MPORTP31_A> for bool {
    #[inline(always)]
    fn from(variant: MPORTP31_A) -> Self {
        variant as u8 != 0
    }
}
impl MPORTP31_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MPORTP31_A {
        match self.bits {
            false => MPORTP31_A::MPORTP_0,
            true => MPORTP31_A::MPORTP_1,
        }
    }
    #[doc = "Checks if the value of the field is `MPORTP_0`"]
    #[inline(always)]
    pub fn is_mportp_0(&self) -> bool {
        *self == MPORTP31_A::MPORTP_0
    }
    #[doc = "Checks if the value of the field is `MPORTP_1`"]
    #[inline(always)]
    pub fn is_mportp_1(&self) -> bool {
        *self == MPORTP31_A::MPORTP_1
    }
}
#[doc = "Field `MPORTP31` writer - Mask bits for port pins"]
pub type MPORTP31_W<'a, const O: u8> = crate::BitWriter<'a, u32, MPIN0_SPEC, MPORTP31_A, O>;
impl<'a, const O: u8> MPORTP31_W<'a, O> {
    #[doc = "Read- pin is LOW and/or the corresponding bit in the MASK register is 1; write- clear output bit if the corresponding bit in the MASK register is 0"]
    #[inline(always)]
    pub fn mportp_0(self) -> &'a mut W {
        self.variant(MPORTP31_A::MPORTP_0)
    }
    #[doc = "Read- pin is HIGH and the corresponding bit in the MASK register is 0; write- set output bit if the corresponding bit in the MASK register is 0"]
    #[inline(always)]
    pub fn mportp_1(self) -> &'a mut W {
        self.variant(MPORTP31_A::MPORTP_1)
    }
}
impl R {
    #[doc = "Bit 0 - Mask bits for port pins"]
    #[inline(always)]
    pub fn mportp0(&self) -> MPORTP0_R {
        MPORTP0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Mask bits for port pins"]
    #[inline(always)]
    pub fn mportp1(&self) -> MPORTP1_R {
        MPORTP1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Mask bits for port pins"]
    #[inline(always)]
    pub fn mportp2(&self) -> MPORTP2_R {
        MPORTP2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Mask bits for port pins"]
    #[inline(always)]
    pub fn mportp3(&self) -> MPORTP3_R {
        MPORTP3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Mask bits for port pins"]
    #[inline(always)]
    pub fn mportp4(&self) -> MPORTP4_R {
        MPORTP4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Mask bits for port pins"]
    #[inline(always)]
    pub fn mportp5(&self) -> MPORTP5_R {
        MPORTP5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Mask bits for port pins"]
    #[inline(always)]
    pub fn mportp6(&self) -> MPORTP6_R {
        MPORTP6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Mask bits for port pins"]
    #[inline(always)]
    pub fn mportp7(&self) -> MPORTP7_R {
        MPORTP7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Mask bits for port pins"]
    #[inline(always)]
    pub fn mportp8(&self) -> MPORTP8_R {
        MPORTP8_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Mask bits for port pins"]
    #[inline(always)]
    pub fn mportp9(&self) -> MPORTP9_R {
        MPORTP9_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Mask bits for port pins"]
    #[inline(always)]
    pub fn mportp10(&self) -> MPORTP10_R {
        MPORTP10_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Mask bits for port pins"]
    #[inline(always)]
    pub fn mportp11(&self) -> MPORTP11_R {
        MPORTP11_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Mask bits for port pins"]
    #[inline(always)]
    pub fn mportp12(&self) -> MPORTP12_R {
        MPORTP12_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Mask bits for port pins"]
    #[inline(always)]
    pub fn mportp13(&self) -> MPORTP13_R {
        MPORTP13_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Mask bits for port pins"]
    #[inline(always)]
    pub fn mportp14(&self) -> MPORTP14_R {
        MPORTP14_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Mask bits for port pins"]
    #[inline(always)]
    pub fn mportp15(&self) -> MPORTP15_R {
        MPORTP15_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Mask bits for port pins"]
    #[inline(always)]
    pub fn mportp16(&self) -> MPORTP16_R {
        MPORTP16_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Mask bits for port pins"]
    #[inline(always)]
    pub fn mportp17(&self) -> MPORTP17_R {
        MPORTP17_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Mask bits for port pins"]
    #[inline(always)]
    pub fn mportp18(&self) -> MPORTP18_R {
        MPORTP18_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Mask bits for port pins"]
    #[inline(always)]
    pub fn mportp19(&self) -> MPORTP19_R {
        MPORTP19_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Mask bits for port pins"]
    #[inline(always)]
    pub fn mportp20(&self) -> MPORTP20_R {
        MPORTP20_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Mask bits for port pins"]
    #[inline(always)]
    pub fn mportp21(&self) -> MPORTP21_R {
        MPORTP21_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Mask bits for port pins"]
    #[inline(always)]
    pub fn mportp22(&self) -> MPORTP22_R {
        MPORTP22_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Mask bits for port pins"]
    #[inline(always)]
    pub fn mportp23(&self) -> MPORTP23_R {
        MPORTP23_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Mask bits for port pins"]
    #[inline(always)]
    pub fn mportp24(&self) -> MPORTP24_R {
        MPORTP24_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Mask bits for port pins"]
    #[inline(always)]
    pub fn mportp25(&self) -> MPORTP25_R {
        MPORTP25_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Mask bits for port pins"]
    #[inline(always)]
    pub fn mportp26(&self) -> MPORTP26_R {
        MPORTP26_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Mask bits for port pins"]
    #[inline(always)]
    pub fn mportp27(&self) -> MPORTP27_R {
        MPORTP27_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Mask bits for port pins"]
    #[inline(always)]
    pub fn mportp28(&self) -> MPORTP28_R {
        MPORTP28_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Mask bits for port pins"]
    #[inline(always)]
    pub fn mportp29(&self) -> MPORTP29_R {
        MPORTP29_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Mask bits for port pins"]
    #[inline(always)]
    pub fn mportp30(&self) -> MPORTP30_R {
        MPORTP30_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Mask bits for port pins"]
    #[inline(always)]
    pub fn mportp31(&self) -> MPORTP31_R {
        MPORTP31_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Mask bits for port pins"]
    #[inline(always)]
    #[must_use]
    pub fn mportp0(&mut self) -> MPORTP0_W<0> {
        MPORTP0_W::new(self)
    }
    #[doc = "Bit 1 - Mask bits for port pins"]
    #[inline(always)]
    #[must_use]
    pub fn mportp1(&mut self) -> MPORTP1_W<1> {
        MPORTP1_W::new(self)
    }
    #[doc = "Bit 2 - Mask bits for port pins"]
    #[inline(always)]
    #[must_use]
    pub fn mportp2(&mut self) -> MPORTP2_W<2> {
        MPORTP2_W::new(self)
    }
    #[doc = "Bit 3 - Mask bits for port pins"]
    #[inline(always)]
    #[must_use]
    pub fn mportp3(&mut self) -> MPORTP3_W<3> {
        MPORTP3_W::new(self)
    }
    #[doc = "Bit 4 - Mask bits for port pins"]
    #[inline(always)]
    #[must_use]
    pub fn mportp4(&mut self) -> MPORTP4_W<4> {
        MPORTP4_W::new(self)
    }
    #[doc = "Bit 5 - Mask bits for port pins"]
    #[inline(always)]
    #[must_use]
    pub fn mportp5(&mut self) -> MPORTP5_W<5> {
        MPORTP5_W::new(self)
    }
    #[doc = "Bit 6 - Mask bits for port pins"]
    #[inline(always)]
    #[must_use]
    pub fn mportp6(&mut self) -> MPORTP6_W<6> {
        MPORTP6_W::new(self)
    }
    #[doc = "Bit 7 - Mask bits for port pins"]
    #[inline(always)]
    #[must_use]
    pub fn mportp7(&mut self) -> MPORTP7_W<7> {
        MPORTP7_W::new(self)
    }
    #[doc = "Bit 8 - Mask bits for port pins"]
    #[inline(always)]
    #[must_use]
    pub fn mportp8(&mut self) -> MPORTP8_W<8> {
        MPORTP8_W::new(self)
    }
    #[doc = "Bit 9 - Mask bits for port pins"]
    #[inline(always)]
    #[must_use]
    pub fn mportp9(&mut self) -> MPORTP9_W<9> {
        MPORTP9_W::new(self)
    }
    #[doc = "Bit 10 - Mask bits for port pins"]
    #[inline(always)]
    #[must_use]
    pub fn mportp10(&mut self) -> MPORTP10_W<10> {
        MPORTP10_W::new(self)
    }
    #[doc = "Bit 11 - Mask bits for port pins"]
    #[inline(always)]
    #[must_use]
    pub fn mportp11(&mut self) -> MPORTP11_W<11> {
        MPORTP11_W::new(self)
    }
    #[doc = "Bit 12 - Mask bits for port pins"]
    #[inline(always)]
    #[must_use]
    pub fn mportp12(&mut self) -> MPORTP12_W<12> {
        MPORTP12_W::new(self)
    }
    #[doc = "Bit 13 - Mask bits for port pins"]
    #[inline(always)]
    #[must_use]
    pub fn mportp13(&mut self) -> MPORTP13_W<13> {
        MPORTP13_W::new(self)
    }
    #[doc = "Bit 14 - Mask bits for port pins"]
    #[inline(always)]
    #[must_use]
    pub fn mportp14(&mut self) -> MPORTP14_W<14> {
        MPORTP14_W::new(self)
    }
    #[doc = "Bit 15 - Mask bits for port pins"]
    #[inline(always)]
    #[must_use]
    pub fn mportp15(&mut self) -> MPORTP15_W<15> {
        MPORTP15_W::new(self)
    }
    #[doc = "Bit 16 - Mask bits for port pins"]
    #[inline(always)]
    #[must_use]
    pub fn mportp16(&mut self) -> MPORTP16_W<16> {
        MPORTP16_W::new(self)
    }
    #[doc = "Bit 17 - Mask bits for port pins"]
    #[inline(always)]
    #[must_use]
    pub fn mportp17(&mut self) -> MPORTP17_W<17> {
        MPORTP17_W::new(self)
    }
    #[doc = "Bit 18 - Mask bits for port pins"]
    #[inline(always)]
    #[must_use]
    pub fn mportp18(&mut self) -> MPORTP18_W<18> {
        MPORTP18_W::new(self)
    }
    #[doc = "Bit 19 - Mask bits for port pins"]
    #[inline(always)]
    #[must_use]
    pub fn mportp19(&mut self) -> MPORTP19_W<19> {
        MPORTP19_W::new(self)
    }
    #[doc = "Bit 20 - Mask bits for port pins"]
    #[inline(always)]
    #[must_use]
    pub fn mportp20(&mut self) -> MPORTP20_W<20> {
        MPORTP20_W::new(self)
    }
    #[doc = "Bit 21 - Mask bits for port pins"]
    #[inline(always)]
    #[must_use]
    pub fn mportp21(&mut self) -> MPORTP21_W<21> {
        MPORTP21_W::new(self)
    }
    #[doc = "Bit 22 - Mask bits for port pins"]
    #[inline(always)]
    #[must_use]
    pub fn mportp22(&mut self) -> MPORTP22_W<22> {
        MPORTP22_W::new(self)
    }
    #[doc = "Bit 23 - Mask bits for port pins"]
    #[inline(always)]
    #[must_use]
    pub fn mportp23(&mut self) -> MPORTP23_W<23> {
        MPORTP23_W::new(self)
    }
    #[doc = "Bit 24 - Mask bits for port pins"]
    #[inline(always)]
    #[must_use]
    pub fn mportp24(&mut self) -> MPORTP24_W<24> {
        MPORTP24_W::new(self)
    }
    #[doc = "Bit 25 - Mask bits for port pins"]
    #[inline(always)]
    #[must_use]
    pub fn mportp25(&mut self) -> MPORTP25_W<25> {
        MPORTP25_W::new(self)
    }
    #[doc = "Bit 26 - Mask bits for port pins"]
    #[inline(always)]
    #[must_use]
    pub fn mportp26(&mut self) -> MPORTP26_W<26> {
        MPORTP26_W::new(self)
    }
    #[doc = "Bit 27 - Mask bits for port pins"]
    #[inline(always)]
    #[must_use]
    pub fn mportp27(&mut self) -> MPORTP27_W<27> {
        MPORTP27_W::new(self)
    }
    #[doc = "Bit 28 - Mask bits for port pins"]
    #[inline(always)]
    #[must_use]
    pub fn mportp28(&mut self) -> MPORTP28_W<28> {
        MPORTP28_W::new(self)
    }
    #[doc = "Bit 29 - Mask bits for port pins"]
    #[inline(always)]
    #[must_use]
    pub fn mportp29(&mut self) -> MPORTP29_W<29> {
        MPORTP29_W::new(self)
    }
    #[doc = "Bit 30 - Mask bits for port pins"]
    #[inline(always)]
    #[must_use]
    pub fn mportp30(&mut self) -> MPORTP30_W<30> {
        MPORTP30_W::new(self)
    }
    #[doc = "Bit 31 - Mask bits for port pins"]
    #[inline(always)]
    #[must_use]
    pub fn mportp31(&mut self) -> MPORTP31_W<31> {
        MPORTP31_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Masked Port Pin\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mpin0](index.html) module"]
pub struct MPIN0_SPEC;
impl crate::RegisterSpec for MPIN0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mpin0::R](R) reader structure"]
impl crate::Readable for MPIN0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mpin0::W](W) writer structure"]
impl crate::Writable for MPIN0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MPIN0 to value 0"]
impl crate::Resettable for MPIN0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
