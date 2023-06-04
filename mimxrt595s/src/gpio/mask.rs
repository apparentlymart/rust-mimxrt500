#[doc = "Register `MASK[%s]` reader"]
pub struct R(crate::R<MASK_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MASK_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MASK_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MASK_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MASK[%s]` writer"]
pub struct W(crate::W<MASK_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MASK_SPEC>;
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
impl From<crate::W<MASK_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MASK_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MASKP0` reader - Port Mask"]
pub type MASKP0_R = crate::BitReader<MASKP0_A>;
#[doc = "Port Mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MASKP0_A {
    #[doc = "0: Read MPIN: pin state; write MPIN: load output bit"]
    MASKP_0 = 0,
    #[doc = "1: Read MPIN: 0; write MPIN: output bit not affected"]
    MASKP_1 = 1,
}
impl From<MASKP0_A> for bool {
    #[inline(always)]
    fn from(variant: MASKP0_A) -> Self {
        variant as u8 != 0
    }
}
impl MASKP0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MASKP0_A {
        match self.bits {
            false => MASKP0_A::MASKP_0,
            true => MASKP0_A::MASKP_1,
        }
    }
    #[doc = "Checks if the value of the field is `MASKP_0`"]
    #[inline(always)]
    pub fn is_maskp_0(&self) -> bool {
        *self == MASKP0_A::MASKP_0
    }
    #[doc = "Checks if the value of the field is `MASKP_1`"]
    #[inline(always)]
    pub fn is_maskp_1(&self) -> bool {
        *self == MASKP0_A::MASKP_1
    }
}
#[doc = "Field `MASKP0` writer - Port Mask"]
pub type MASKP0_W<'a, const O: u8> = crate::BitWriter<'a, u32, MASK_SPEC, MASKP0_A, O>;
impl<'a, const O: u8> MASKP0_W<'a, O> {
    #[doc = "Read MPIN: pin state; write MPIN: load output bit"]
    #[inline(always)]
    pub fn maskp_0(self) -> &'a mut W {
        self.variant(MASKP0_A::MASKP_0)
    }
    #[doc = "Read MPIN: 0; write MPIN: output bit not affected"]
    #[inline(always)]
    pub fn maskp_1(self) -> &'a mut W {
        self.variant(MASKP0_A::MASKP_1)
    }
}
#[doc = "Field `MASKP1` reader - Port Mask"]
pub type MASKP1_R = crate::BitReader<MASKP1_A>;
#[doc = "Port Mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MASKP1_A {
    #[doc = "0: Read MPIN: pin state; write MPIN: load output bit"]
    MASKP_0 = 0,
    #[doc = "1: Read MPIN: 0; write MPIN: output bit not affected"]
    MASKP_1 = 1,
}
impl From<MASKP1_A> for bool {
    #[inline(always)]
    fn from(variant: MASKP1_A) -> Self {
        variant as u8 != 0
    }
}
impl MASKP1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MASKP1_A {
        match self.bits {
            false => MASKP1_A::MASKP_0,
            true => MASKP1_A::MASKP_1,
        }
    }
    #[doc = "Checks if the value of the field is `MASKP_0`"]
    #[inline(always)]
    pub fn is_maskp_0(&self) -> bool {
        *self == MASKP1_A::MASKP_0
    }
    #[doc = "Checks if the value of the field is `MASKP_1`"]
    #[inline(always)]
    pub fn is_maskp_1(&self) -> bool {
        *self == MASKP1_A::MASKP_1
    }
}
#[doc = "Field `MASKP1` writer - Port Mask"]
pub type MASKP1_W<'a, const O: u8> = crate::BitWriter<'a, u32, MASK_SPEC, MASKP1_A, O>;
impl<'a, const O: u8> MASKP1_W<'a, O> {
    #[doc = "Read MPIN: pin state; write MPIN: load output bit"]
    #[inline(always)]
    pub fn maskp_0(self) -> &'a mut W {
        self.variant(MASKP1_A::MASKP_0)
    }
    #[doc = "Read MPIN: 0; write MPIN: output bit not affected"]
    #[inline(always)]
    pub fn maskp_1(self) -> &'a mut W {
        self.variant(MASKP1_A::MASKP_1)
    }
}
#[doc = "Field `MASKP2` reader - Port Mask"]
pub type MASKP2_R = crate::BitReader<MASKP2_A>;
#[doc = "Port Mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MASKP2_A {
    #[doc = "0: Read MPIN: pin state; write MPIN: load output bit"]
    MASKP_0 = 0,
    #[doc = "1: Read MPIN: 0; write MPIN: output bit not affected"]
    MASKP_1 = 1,
}
impl From<MASKP2_A> for bool {
    #[inline(always)]
    fn from(variant: MASKP2_A) -> Self {
        variant as u8 != 0
    }
}
impl MASKP2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MASKP2_A {
        match self.bits {
            false => MASKP2_A::MASKP_0,
            true => MASKP2_A::MASKP_1,
        }
    }
    #[doc = "Checks if the value of the field is `MASKP_0`"]
    #[inline(always)]
    pub fn is_maskp_0(&self) -> bool {
        *self == MASKP2_A::MASKP_0
    }
    #[doc = "Checks if the value of the field is `MASKP_1`"]
    #[inline(always)]
    pub fn is_maskp_1(&self) -> bool {
        *self == MASKP2_A::MASKP_1
    }
}
#[doc = "Field `MASKP2` writer - Port Mask"]
pub type MASKP2_W<'a, const O: u8> = crate::BitWriter<'a, u32, MASK_SPEC, MASKP2_A, O>;
impl<'a, const O: u8> MASKP2_W<'a, O> {
    #[doc = "Read MPIN: pin state; write MPIN: load output bit"]
    #[inline(always)]
    pub fn maskp_0(self) -> &'a mut W {
        self.variant(MASKP2_A::MASKP_0)
    }
    #[doc = "Read MPIN: 0; write MPIN: output bit not affected"]
    #[inline(always)]
    pub fn maskp_1(self) -> &'a mut W {
        self.variant(MASKP2_A::MASKP_1)
    }
}
#[doc = "Field `MASKP3` reader - Port Mask"]
pub type MASKP3_R = crate::BitReader<MASKP3_A>;
#[doc = "Port Mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MASKP3_A {
    #[doc = "0: Read MPIN: pin state; write MPIN: load output bit"]
    MASKP_0 = 0,
    #[doc = "1: Read MPIN: 0; write MPIN: output bit not affected"]
    MASKP_1 = 1,
}
impl From<MASKP3_A> for bool {
    #[inline(always)]
    fn from(variant: MASKP3_A) -> Self {
        variant as u8 != 0
    }
}
impl MASKP3_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MASKP3_A {
        match self.bits {
            false => MASKP3_A::MASKP_0,
            true => MASKP3_A::MASKP_1,
        }
    }
    #[doc = "Checks if the value of the field is `MASKP_0`"]
    #[inline(always)]
    pub fn is_maskp_0(&self) -> bool {
        *self == MASKP3_A::MASKP_0
    }
    #[doc = "Checks if the value of the field is `MASKP_1`"]
    #[inline(always)]
    pub fn is_maskp_1(&self) -> bool {
        *self == MASKP3_A::MASKP_1
    }
}
#[doc = "Field `MASKP3` writer - Port Mask"]
pub type MASKP3_W<'a, const O: u8> = crate::BitWriter<'a, u32, MASK_SPEC, MASKP3_A, O>;
impl<'a, const O: u8> MASKP3_W<'a, O> {
    #[doc = "Read MPIN: pin state; write MPIN: load output bit"]
    #[inline(always)]
    pub fn maskp_0(self) -> &'a mut W {
        self.variant(MASKP3_A::MASKP_0)
    }
    #[doc = "Read MPIN: 0; write MPIN: output bit not affected"]
    #[inline(always)]
    pub fn maskp_1(self) -> &'a mut W {
        self.variant(MASKP3_A::MASKP_1)
    }
}
#[doc = "Field `MASKP4` reader - Port Mask"]
pub type MASKP4_R = crate::BitReader<MASKP4_A>;
#[doc = "Port Mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MASKP4_A {
    #[doc = "0: Read MPIN: pin state; write MPIN: load output bit"]
    MASKP_0 = 0,
    #[doc = "1: Read MPIN: 0; write MPIN: output bit not affected"]
    MASKP_1 = 1,
}
impl From<MASKP4_A> for bool {
    #[inline(always)]
    fn from(variant: MASKP4_A) -> Self {
        variant as u8 != 0
    }
}
impl MASKP4_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MASKP4_A {
        match self.bits {
            false => MASKP4_A::MASKP_0,
            true => MASKP4_A::MASKP_1,
        }
    }
    #[doc = "Checks if the value of the field is `MASKP_0`"]
    #[inline(always)]
    pub fn is_maskp_0(&self) -> bool {
        *self == MASKP4_A::MASKP_0
    }
    #[doc = "Checks if the value of the field is `MASKP_1`"]
    #[inline(always)]
    pub fn is_maskp_1(&self) -> bool {
        *self == MASKP4_A::MASKP_1
    }
}
#[doc = "Field `MASKP4` writer - Port Mask"]
pub type MASKP4_W<'a, const O: u8> = crate::BitWriter<'a, u32, MASK_SPEC, MASKP4_A, O>;
impl<'a, const O: u8> MASKP4_W<'a, O> {
    #[doc = "Read MPIN: pin state; write MPIN: load output bit"]
    #[inline(always)]
    pub fn maskp_0(self) -> &'a mut W {
        self.variant(MASKP4_A::MASKP_0)
    }
    #[doc = "Read MPIN: 0; write MPIN: output bit not affected"]
    #[inline(always)]
    pub fn maskp_1(self) -> &'a mut W {
        self.variant(MASKP4_A::MASKP_1)
    }
}
#[doc = "Field `MASKP5` reader - Port Mask"]
pub type MASKP5_R = crate::BitReader<MASKP5_A>;
#[doc = "Port Mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MASKP5_A {
    #[doc = "0: Read MPIN: pin state; write MPIN: load output bit"]
    MASKP_0 = 0,
    #[doc = "1: Read MPIN: 0; write MPIN: output bit not affected"]
    MASKP_1 = 1,
}
impl From<MASKP5_A> for bool {
    #[inline(always)]
    fn from(variant: MASKP5_A) -> Self {
        variant as u8 != 0
    }
}
impl MASKP5_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MASKP5_A {
        match self.bits {
            false => MASKP5_A::MASKP_0,
            true => MASKP5_A::MASKP_1,
        }
    }
    #[doc = "Checks if the value of the field is `MASKP_0`"]
    #[inline(always)]
    pub fn is_maskp_0(&self) -> bool {
        *self == MASKP5_A::MASKP_0
    }
    #[doc = "Checks if the value of the field is `MASKP_1`"]
    #[inline(always)]
    pub fn is_maskp_1(&self) -> bool {
        *self == MASKP5_A::MASKP_1
    }
}
#[doc = "Field `MASKP5` writer - Port Mask"]
pub type MASKP5_W<'a, const O: u8> = crate::BitWriter<'a, u32, MASK_SPEC, MASKP5_A, O>;
impl<'a, const O: u8> MASKP5_W<'a, O> {
    #[doc = "Read MPIN: pin state; write MPIN: load output bit"]
    #[inline(always)]
    pub fn maskp_0(self) -> &'a mut W {
        self.variant(MASKP5_A::MASKP_0)
    }
    #[doc = "Read MPIN: 0; write MPIN: output bit not affected"]
    #[inline(always)]
    pub fn maskp_1(self) -> &'a mut W {
        self.variant(MASKP5_A::MASKP_1)
    }
}
#[doc = "Field `MASKP6` reader - Port Mask"]
pub type MASKP6_R = crate::BitReader<MASKP6_A>;
#[doc = "Port Mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MASKP6_A {
    #[doc = "0: Read MPIN: pin state; write MPIN: load output bit"]
    MASKP_0 = 0,
    #[doc = "1: Read MPIN: 0; write MPIN: output bit not affected"]
    MASKP_1 = 1,
}
impl From<MASKP6_A> for bool {
    #[inline(always)]
    fn from(variant: MASKP6_A) -> Self {
        variant as u8 != 0
    }
}
impl MASKP6_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MASKP6_A {
        match self.bits {
            false => MASKP6_A::MASKP_0,
            true => MASKP6_A::MASKP_1,
        }
    }
    #[doc = "Checks if the value of the field is `MASKP_0`"]
    #[inline(always)]
    pub fn is_maskp_0(&self) -> bool {
        *self == MASKP6_A::MASKP_0
    }
    #[doc = "Checks if the value of the field is `MASKP_1`"]
    #[inline(always)]
    pub fn is_maskp_1(&self) -> bool {
        *self == MASKP6_A::MASKP_1
    }
}
#[doc = "Field `MASKP6` writer - Port Mask"]
pub type MASKP6_W<'a, const O: u8> = crate::BitWriter<'a, u32, MASK_SPEC, MASKP6_A, O>;
impl<'a, const O: u8> MASKP6_W<'a, O> {
    #[doc = "Read MPIN: pin state; write MPIN: load output bit"]
    #[inline(always)]
    pub fn maskp_0(self) -> &'a mut W {
        self.variant(MASKP6_A::MASKP_0)
    }
    #[doc = "Read MPIN: 0; write MPIN: output bit not affected"]
    #[inline(always)]
    pub fn maskp_1(self) -> &'a mut W {
        self.variant(MASKP6_A::MASKP_1)
    }
}
#[doc = "Field `MASKP7` reader - Port Mask"]
pub type MASKP7_R = crate::BitReader<MASKP7_A>;
#[doc = "Port Mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MASKP7_A {
    #[doc = "0: Read MPIN: pin state; write MPIN: load output bit"]
    MASKP_0 = 0,
    #[doc = "1: Read MPIN: 0; write MPIN: output bit not affected"]
    MASKP_1 = 1,
}
impl From<MASKP7_A> for bool {
    #[inline(always)]
    fn from(variant: MASKP7_A) -> Self {
        variant as u8 != 0
    }
}
impl MASKP7_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MASKP7_A {
        match self.bits {
            false => MASKP7_A::MASKP_0,
            true => MASKP7_A::MASKP_1,
        }
    }
    #[doc = "Checks if the value of the field is `MASKP_0`"]
    #[inline(always)]
    pub fn is_maskp_0(&self) -> bool {
        *self == MASKP7_A::MASKP_0
    }
    #[doc = "Checks if the value of the field is `MASKP_1`"]
    #[inline(always)]
    pub fn is_maskp_1(&self) -> bool {
        *self == MASKP7_A::MASKP_1
    }
}
#[doc = "Field `MASKP7` writer - Port Mask"]
pub type MASKP7_W<'a, const O: u8> = crate::BitWriter<'a, u32, MASK_SPEC, MASKP7_A, O>;
impl<'a, const O: u8> MASKP7_W<'a, O> {
    #[doc = "Read MPIN: pin state; write MPIN: load output bit"]
    #[inline(always)]
    pub fn maskp_0(self) -> &'a mut W {
        self.variant(MASKP7_A::MASKP_0)
    }
    #[doc = "Read MPIN: 0; write MPIN: output bit not affected"]
    #[inline(always)]
    pub fn maskp_1(self) -> &'a mut W {
        self.variant(MASKP7_A::MASKP_1)
    }
}
#[doc = "Field `MASKP8` reader - Port Mask"]
pub type MASKP8_R = crate::BitReader<MASKP8_A>;
#[doc = "Port Mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MASKP8_A {
    #[doc = "0: Read MPIN: pin state; write MPIN: load output bit"]
    MASKP_0 = 0,
    #[doc = "1: Read MPIN: 0; write MPIN: output bit not affected"]
    MASKP_1 = 1,
}
impl From<MASKP8_A> for bool {
    #[inline(always)]
    fn from(variant: MASKP8_A) -> Self {
        variant as u8 != 0
    }
}
impl MASKP8_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MASKP8_A {
        match self.bits {
            false => MASKP8_A::MASKP_0,
            true => MASKP8_A::MASKP_1,
        }
    }
    #[doc = "Checks if the value of the field is `MASKP_0`"]
    #[inline(always)]
    pub fn is_maskp_0(&self) -> bool {
        *self == MASKP8_A::MASKP_0
    }
    #[doc = "Checks if the value of the field is `MASKP_1`"]
    #[inline(always)]
    pub fn is_maskp_1(&self) -> bool {
        *self == MASKP8_A::MASKP_1
    }
}
#[doc = "Field `MASKP8` writer - Port Mask"]
pub type MASKP8_W<'a, const O: u8> = crate::BitWriter<'a, u32, MASK_SPEC, MASKP8_A, O>;
impl<'a, const O: u8> MASKP8_W<'a, O> {
    #[doc = "Read MPIN: pin state; write MPIN: load output bit"]
    #[inline(always)]
    pub fn maskp_0(self) -> &'a mut W {
        self.variant(MASKP8_A::MASKP_0)
    }
    #[doc = "Read MPIN: 0; write MPIN: output bit not affected"]
    #[inline(always)]
    pub fn maskp_1(self) -> &'a mut W {
        self.variant(MASKP8_A::MASKP_1)
    }
}
#[doc = "Field `MASKP9` reader - Port Mask"]
pub type MASKP9_R = crate::BitReader<MASKP9_A>;
#[doc = "Port Mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MASKP9_A {
    #[doc = "0: Read MPIN: pin state; write MPIN: load output bit"]
    MASKP_0 = 0,
    #[doc = "1: Read MPIN: 0; write MPIN: output bit not affected"]
    MASKP_1 = 1,
}
impl From<MASKP9_A> for bool {
    #[inline(always)]
    fn from(variant: MASKP9_A) -> Self {
        variant as u8 != 0
    }
}
impl MASKP9_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MASKP9_A {
        match self.bits {
            false => MASKP9_A::MASKP_0,
            true => MASKP9_A::MASKP_1,
        }
    }
    #[doc = "Checks if the value of the field is `MASKP_0`"]
    #[inline(always)]
    pub fn is_maskp_0(&self) -> bool {
        *self == MASKP9_A::MASKP_0
    }
    #[doc = "Checks if the value of the field is `MASKP_1`"]
    #[inline(always)]
    pub fn is_maskp_1(&self) -> bool {
        *self == MASKP9_A::MASKP_1
    }
}
#[doc = "Field `MASKP9` writer - Port Mask"]
pub type MASKP9_W<'a, const O: u8> = crate::BitWriter<'a, u32, MASK_SPEC, MASKP9_A, O>;
impl<'a, const O: u8> MASKP9_W<'a, O> {
    #[doc = "Read MPIN: pin state; write MPIN: load output bit"]
    #[inline(always)]
    pub fn maskp_0(self) -> &'a mut W {
        self.variant(MASKP9_A::MASKP_0)
    }
    #[doc = "Read MPIN: 0; write MPIN: output bit not affected"]
    #[inline(always)]
    pub fn maskp_1(self) -> &'a mut W {
        self.variant(MASKP9_A::MASKP_1)
    }
}
#[doc = "Field `MASKP10` reader - Port Mask"]
pub type MASKP10_R = crate::BitReader<MASKP10_A>;
#[doc = "Port Mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MASKP10_A {
    #[doc = "0: Read MPIN: pin state; write MPIN: load output bit"]
    MASKP_0 = 0,
    #[doc = "1: Read MPIN: 0; write MPIN: output bit not affected"]
    MASKP_1 = 1,
}
impl From<MASKP10_A> for bool {
    #[inline(always)]
    fn from(variant: MASKP10_A) -> Self {
        variant as u8 != 0
    }
}
impl MASKP10_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MASKP10_A {
        match self.bits {
            false => MASKP10_A::MASKP_0,
            true => MASKP10_A::MASKP_1,
        }
    }
    #[doc = "Checks if the value of the field is `MASKP_0`"]
    #[inline(always)]
    pub fn is_maskp_0(&self) -> bool {
        *self == MASKP10_A::MASKP_0
    }
    #[doc = "Checks if the value of the field is `MASKP_1`"]
    #[inline(always)]
    pub fn is_maskp_1(&self) -> bool {
        *self == MASKP10_A::MASKP_1
    }
}
#[doc = "Field `MASKP10` writer - Port Mask"]
pub type MASKP10_W<'a, const O: u8> = crate::BitWriter<'a, u32, MASK_SPEC, MASKP10_A, O>;
impl<'a, const O: u8> MASKP10_W<'a, O> {
    #[doc = "Read MPIN: pin state; write MPIN: load output bit"]
    #[inline(always)]
    pub fn maskp_0(self) -> &'a mut W {
        self.variant(MASKP10_A::MASKP_0)
    }
    #[doc = "Read MPIN: 0; write MPIN: output bit not affected"]
    #[inline(always)]
    pub fn maskp_1(self) -> &'a mut W {
        self.variant(MASKP10_A::MASKP_1)
    }
}
#[doc = "Field `MASKP11` reader - Port Mask"]
pub type MASKP11_R = crate::BitReader<MASKP11_A>;
#[doc = "Port Mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MASKP11_A {
    #[doc = "0: Read MPIN: pin state; write MPIN: load output bit"]
    MASKP_0 = 0,
    #[doc = "1: Read MPIN: 0; write MPIN: output bit not affected"]
    MASKP_1 = 1,
}
impl From<MASKP11_A> for bool {
    #[inline(always)]
    fn from(variant: MASKP11_A) -> Self {
        variant as u8 != 0
    }
}
impl MASKP11_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MASKP11_A {
        match self.bits {
            false => MASKP11_A::MASKP_0,
            true => MASKP11_A::MASKP_1,
        }
    }
    #[doc = "Checks if the value of the field is `MASKP_0`"]
    #[inline(always)]
    pub fn is_maskp_0(&self) -> bool {
        *self == MASKP11_A::MASKP_0
    }
    #[doc = "Checks if the value of the field is `MASKP_1`"]
    #[inline(always)]
    pub fn is_maskp_1(&self) -> bool {
        *self == MASKP11_A::MASKP_1
    }
}
#[doc = "Field `MASKP11` writer - Port Mask"]
pub type MASKP11_W<'a, const O: u8> = crate::BitWriter<'a, u32, MASK_SPEC, MASKP11_A, O>;
impl<'a, const O: u8> MASKP11_W<'a, O> {
    #[doc = "Read MPIN: pin state; write MPIN: load output bit"]
    #[inline(always)]
    pub fn maskp_0(self) -> &'a mut W {
        self.variant(MASKP11_A::MASKP_0)
    }
    #[doc = "Read MPIN: 0; write MPIN: output bit not affected"]
    #[inline(always)]
    pub fn maskp_1(self) -> &'a mut W {
        self.variant(MASKP11_A::MASKP_1)
    }
}
#[doc = "Field `MASKP12` reader - Port Mask"]
pub type MASKP12_R = crate::BitReader<MASKP12_A>;
#[doc = "Port Mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MASKP12_A {
    #[doc = "0: Read MPIN: pin state; write MPIN: load output bit"]
    MASKP_0 = 0,
    #[doc = "1: Read MPIN: 0; write MPIN: output bit not affected"]
    MASKP_1 = 1,
}
impl From<MASKP12_A> for bool {
    #[inline(always)]
    fn from(variant: MASKP12_A) -> Self {
        variant as u8 != 0
    }
}
impl MASKP12_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MASKP12_A {
        match self.bits {
            false => MASKP12_A::MASKP_0,
            true => MASKP12_A::MASKP_1,
        }
    }
    #[doc = "Checks if the value of the field is `MASKP_0`"]
    #[inline(always)]
    pub fn is_maskp_0(&self) -> bool {
        *self == MASKP12_A::MASKP_0
    }
    #[doc = "Checks if the value of the field is `MASKP_1`"]
    #[inline(always)]
    pub fn is_maskp_1(&self) -> bool {
        *self == MASKP12_A::MASKP_1
    }
}
#[doc = "Field `MASKP12` writer - Port Mask"]
pub type MASKP12_W<'a, const O: u8> = crate::BitWriter<'a, u32, MASK_SPEC, MASKP12_A, O>;
impl<'a, const O: u8> MASKP12_W<'a, O> {
    #[doc = "Read MPIN: pin state; write MPIN: load output bit"]
    #[inline(always)]
    pub fn maskp_0(self) -> &'a mut W {
        self.variant(MASKP12_A::MASKP_0)
    }
    #[doc = "Read MPIN: 0; write MPIN: output bit not affected"]
    #[inline(always)]
    pub fn maskp_1(self) -> &'a mut W {
        self.variant(MASKP12_A::MASKP_1)
    }
}
#[doc = "Field `MASKP13` reader - Port Mask"]
pub type MASKP13_R = crate::BitReader<MASKP13_A>;
#[doc = "Port Mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MASKP13_A {
    #[doc = "0: Read MPIN: pin state; write MPIN: load output bit"]
    MASKP_0 = 0,
    #[doc = "1: Read MPIN: 0; write MPIN: output bit not affected"]
    MASKP_1 = 1,
}
impl From<MASKP13_A> for bool {
    #[inline(always)]
    fn from(variant: MASKP13_A) -> Self {
        variant as u8 != 0
    }
}
impl MASKP13_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MASKP13_A {
        match self.bits {
            false => MASKP13_A::MASKP_0,
            true => MASKP13_A::MASKP_1,
        }
    }
    #[doc = "Checks if the value of the field is `MASKP_0`"]
    #[inline(always)]
    pub fn is_maskp_0(&self) -> bool {
        *self == MASKP13_A::MASKP_0
    }
    #[doc = "Checks if the value of the field is `MASKP_1`"]
    #[inline(always)]
    pub fn is_maskp_1(&self) -> bool {
        *self == MASKP13_A::MASKP_1
    }
}
#[doc = "Field `MASKP13` writer - Port Mask"]
pub type MASKP13_W<'a, const O: u8> = crate::BitWriter<'a, u32, MASK_SPEC, MASKP13_A, O>;
impl<'a, const O: u8> MASKP13_W<'a, O> {
    #[doc = "Read MPIN: pin state; write MPIN: load output bit"]
    #[inline(always)]
    pub fn maskp_0(self) -> &'a mut W {
        self.variant(MASKP13_A::MASKP_0)
    }
    #[doc = "Read MPIN: 0; write MPIN: output bit not affected"]
    #[inline(always)]
    pub fn maskp_1(self) -> &'a mut W {
        self.variant(MASKP13_A::MASKP_1)
    }
}
#[doc = "Field `MASKP14` reader - Port Mask"]
pub type MASKP14_R = crate::BitReader<MASKP14_A>;
#[doc = "Port Mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MASKP14_A {
    #[doc = "0: Read MPIN: pin state; write MPIN: load output bit"]
    MASKP_0 = 0,
    #[doc = "1: Read MPIN: 0; write MPIN: output bit not affected"]
    MASKP_1 = 1,
}
impl From<MASKP14_A> for bool {
    #[inline(always)]
    fn from(variant: MASKP14_A) -> Self {
        variant as u8 != 0
    }
}
impl MASKP14_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MASKP14_A {
        match self.bits {
            false => MASKP14_A::MASKP_0,
            true => MASKP14_A::MASKP_1,
        }
    }
    #[doc = "Checks if the value of the field is `MASKP_0`"]
    #[inline(always)]
    pub fn is_maskp_0(&self) -> bool {
        *self == MASKP14_A::MASKP_0
    }
    #[doc = "Checks if the value of the field is `MASKP_1`"]
    #[inline(always)]
    pub fn is_maskp_1(&self) -> bool {
        *self == MASKP14_A::MASKP_1
    }
}
#[doc = "Field `MASKP14` writer - Port Mask"]
pub type MASKP14_W<'a, const O: u8> = crate::BitWriter<'a, u32, MASK_SPEC, MASKP14_A, O>;
impl<'a, const O: u8> MASKP14_W<'a, O> {
    #[doc = "Read MPIN: pin state; write MPIN: load output bit"]
    #[inline(always)]
    pub fn maskp_0(self) -> &'a mut W {
        self.variant(MASKP14_A::MASKP_0)
    }
    #[doc = "Read MPIN: 0; write MPIN: output bit not affected"]
    #[inline(always)]
    pub fn maskp_1(self) -> &'a mut W {
        self.variant(MASKP14_A::MASKP_1)
    }
}
#[doc = "Field `MASKP15` reader - Port Mask"]
pub type MASKP15_R = crate::BitReader<MASKP15_A>;
#[doc = "Port Mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MASKP15_A {
    #[doc = "0: Read MPIN: pin state; write MPIN: load output bit"]
    MASKP_0 = 0,
    #[doc = "1: Read MPIN: 0; write MPIN: output bit not affected"]
    MASKP_1 = 1,
}
impl From<MASKP15_A> for bool {
    #[inline(always)]
    fn from(variant: MASKP15_A) -> Self {
        variant as u8 != 0
    }
}
impl MASKP15_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MASKP15_A {
        match self.bits {
            false => MASKP15_A::MASKP_0,
            true => MASKP15_A::MASKP_1,
        }
    }
    #[doc = "Checks if the value of the field is `MASKP_0`"]
    #[inline(always)]
    pub fn is_maskp_0(&self) -> bool {
        *self == MASKP15_A::MASKP_0
    }
    #[doc = "Checks if the value of the field is `MASKP_1`"]
    #[inline(always)]
    pub fn is_maskp_1(&self) -> bool {
        *self == MASKP15_A::MASKP_1
    }
}
#[doc = "Field `MASKP15` writer - Port Mask"]
pub type MASKP15_W<'a, const O: u8> = crate::BitWriter<'a, u32, MASK_SPEC, MASKP15_A, O>;
impl<'a, const O: u8> MASKP15_W<'a, O> {
    #[doc = "Read MPIN: pin state; write MPIN: load output bit"]
    #[inline(always)]
    pub fn maskp_0(self) -> &'a mut W {
        self.variant(MASKP15_A::MASKP_0)
    }
    #[doc = "Read MPIN: 0; write MPIN: output bit not affected"]
    #[inline(always)]
    pub fn maskp_1(self) -> &'a mut W {
        self.variant(MASKP15_A::MASKP_1)
    }
}
#[doc = "Field `MASKP16` reader - Port Mask"]
pub type MASKP16_R = crate::BitReader<MASKP16_A>;
#[doc = "Port Mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MASKP16_A {
    #[doc = "0: Read MPIN: pin state; write MPIN: load output bit"]
    MASKP_0 = 0,
    #[doc = "1: Read MPIN: 0; write MPIN: output bit not affected"]
    MASKP_1 = 1,
}
impl From<MASKP16_A> for bool {
    #[inline(always)]
    fn from(variant: MASKP16_A) -> Self {
        variant as u8 != 0
    }
}
impl MASKP16_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MASKP16_A {
        match self.bits {
            false => MASKP16_A::MASKP_0,
            true => MASKP16_A::MASKP_1,
        }
    }
    #[doc = "Checks if the value of the field is `MASKP_0`"]
    #[inline(always)]
    pub fn is_maskp_0(&self) -> bool {
        *self == MASKP16_A::MASKP_0
    }
    #[doc = "Checks if the value of the field is `MASKP_1`"]
    #[inline(always)]
    pub fn is_maskp_1(&self) -> bool {
        *self == MASKP16_A::MASKP_1
    }
}
#[doc = "Field `MASKP16` writer - Port Mask"]
pub type MASKP16_W<'a, const O: u8> = crate::BitWriter<'a, u32, MASK_SPEC, MASKP16_A, O>;
impl<'a, const O: u8> MASKP16_W<'a, O> {
    #[doc = "Read MPIN: pin state; write MPIN: load output bit"]
    #[inline(always)]
    pub fn maskp_0(self) -> &'a mut W {
        self.variant(MASKP16_A::MASKP_0)
    }
    #[doc = "Read MPIN: 0; write MPIN: output bit not affected"]
    #[inline(always)]
    pub fn maskp_1(self) -> &'a mut W {
        self.variant(MASKP16_A::MASKP_1)
    }
}
#[doc = "Field `MASKP17` reader - Port Mask"]
pub type MASKP17_R = crate::BitReader<MASKP17_A>;
#[doc = "Port Mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MASKP17_A {
    #[doc = "0: Read MPIN: pin state; write MPIN: load output bit"]
    MASKP_0 = 0,
    #[doc = "1: Read MPIN: 0; write MPIN: output bit not affected"]
    MASKP_1 = 1,
}
impl From<MASKP17_A> for bool {
    #[inline(always)]
    fn from(variant: MASKP17_A) -> Self {
        variant as u8 != 0
    }
}
impl MASKP17_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MASKP17_A {
        match self.bits {
            false => MASKP17_A::MASKP_0,
            true => MASKP17_A::MASKP_1,
        }
    }
    #[doc = "Checks if the value of the field is `MASKP_0`"]
    #[inline(always)]
    pub fn is_maskp_0(&self) -> bool {
        *self == MASKP17_A::MASKP_0
    }
    #[doc = "Checks if the value of the field is `MASKP_1`"]
    #[inline(always)]
    pub fn is_maskp_1(&self) -> bool {
        *self == MASKP17_A::MASKP_1
    }
}
#[doc = "Field `MASKP17` writer - Port Mask"]
pub type MASKP17_W<'a, const O: u8> = crate::BitWriter<'a, u32, MASK_SPEC, MASKP17_A, O>;
impl<'a, const O: u8> MASKP17_W<'a, O> {
    #[doc = "Read MPIN: pin state; write MPIN: load output bit"]
    #[inline(always)]
    pub fn maskp_0(self) -> &'a mut W {
        self.variant(MASKP17_A::MASKP_0)
    }
    #[doc = "Read MPIN: 0; write MPIN: output bit not affected"]
    #[inline(always)]
    pub fn maskp_1(self) -> &'a mut W {
        self.variant(MASKP17_A::MASKP_1)
    }
}
#[doc = "Field `MASKP18` reader - Port Mask"]
pub type MASKP18_R = crate::BitReader<MASKP18_A>;
#[doc = "Port Mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MASKP18_A {
    #[doc = "0: Read MPIN: pin state; write MPIN: load output bit"]
    MASKP_0 = 0,
    #[doc = "1: Read MPIN: 0; write MPIN: output bit not affected"]
    MASKP_1 = 1,
}
impl From<MASKP18_A> for bool {
    #[inline(always)]
    fn from(variant: MASKP18_A) -> Self {
        variant as u8 != 0
    }
}
impl MASKP18_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MASKP18_A {
        match self.bits {
            false => MASKP18_A::MASKP_0,
            true => MASKP18_A::MASKP_1,
        }
    }
    #[doc = "Checks if the value of the field is `MASKP_0`"]
    #[inline(always)]
    pub fn is_maskp_0(&self) -> bool {
        *self == MASKP18_A::MASKP_0
    }
    #[doc = "Checks if the value of the field is `MASKP_1`"]
    #[inline(always)]
    pub fn is_maskp_1(&self) -> bool {
        *self == MASKP18_A::MASKP_1
    }
}
#[doc = "Field `MASKP18` writer - Port Mask"]
pub type MASKP18_W<'a, const O: u8> = crate::BitWriter<'a, u32, MASK_SPEC, MASKP18_A, O>;
impl<'a, const O: u8> MASKP18_W<'a, O> {
    #[doc = "Read MPIN: pin state; write MPIN: load output bit"]
    #[inline(always)]
    pub fn maskp_0(self) -> &'a mut W {
        self.variant(MASKP18_A::MASKP_0)
    }
    #[doc = "Read MPIN: 0; write MPIN: output bit not affected"]
    #[inline(always)]
    pub fn maskp_1(self) -> &'a mut W {
        self.variant(MASKP18_A::MASKP_1)
    }
}
#[doc = "Field `MASKP19` reader - Port Mask"]
pub type MASKP19_R = crate::BitReader<MASKP19_A>;
#[doc = "Port Mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MASKP19_A {
    #[doc = "0: Read MPIN: pin state; write MPIN: load output bit"]
    MASKP_0 = 0,
    #[doc = "1: Read MPIN: 0; write MPIN: output bit not affected"]
    MASKP_1 = 1,
}
impl From<MASKP19_A> for bool {
    #[inline(always)]
    fn from(variant: MASKP19_A) -> Self {
        variant as u8 != 0
    }
}
impl MASKP19_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MASKP19_A {
        match self.bits {
            false => MASKP19_A::MASKP_0,
            true => MASKP19_A::MASKP_1,
        }
    }
    #[doc = "Checks if the value of the field is `MASKP_0`"]
    #[inline(always)]
    pub fn is_maskp_0(&self) -> bool {
        *self == MASKP19_A::MASKP_0
    }
    #[doc = "Checks if the value of the field is `MASKP_1`"]
    #[inline(always)]
    pub fn is_maskp_1(&self) -> bool {
        *self == MASKP19_A::MASKP_1
    }
}
#[doc = "Field `MASKP19` writer - Port Mask"]
pub type MASKP19_W<'a, const O: u8> = crate::BitWriter<'a, u32, MASK_SPEC, MASKP19_A, O>;
impl<'a, const O: u8> MASKP19_W<'a, O> {
    #[doc = "Read MPIN: pin state; write MPIN: load output bit"]
    #[inline(always)]
    pub fn maskp_0(self) -> &'a mut W {
        self.variant(MASKP19_A::MASKP_0)
    }
    #[doc = "Read MPIN: 0; write MPIN: output bit not affected"]
    #[inline(always)]
    pub fn maskp_1(self) -> &'a mut W {
        self.variant(MASKP19_A::MASKP_1)
    }
}
#[doc = "Field `MASKP20` reader - Port Mask"]
pub type MASKP20_R = crate::BitReader<MASKP20_A>;
#[doc = "Port Mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MASKP20_A {
    #[doc = "0: Read MPIN: pin state; write MPIN: load output bit"]
    MASKP_0 = 0,
    #[doc = "1: Read MPIN: 0; write MPIN: output bit not affected"]
    MASKP_1 = 1,
}
impl From<MASKP20_A> for bool {
    #[inline(always)]
    fn from(variant: MASKP20_A) -> Self {
        variant as u8 != 0
    }
}
impl MASKP20_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MASKP20_A {
        match self.bits {
            false => MASKP20_A::MASKP_0,
            true => MASKP20_A::MASKP_1,
        }
    }
    #[doc = "Checks if the value of the field is `MASKP_0`"]
    #[inline(always)]
    pub fn is_maskp_0(&self) -> bool {
        *self == MASKP20_A::MASKP_0
    }
    #[doc = "Checks if the value of the field is `MASKP_1`"]
    #[inline(always)]
    pub fn is_maskp_1(&self) -> bool {
        *self == MASKP20_A::MASKP_1
    }
}
#[doc = "Field `MASKP20` writer - Port Mask"]
pub type MASKP20_W<'a, const O: u8> = crate::BitWriter<'a, u32, MASK_SPEC, MASKP20_A, O>;
impl<'a, const O: u8> MASKP20_W<'a, O> {
    #[doc = "Read MPIN: pin state; write MPIN: load output bit"]
    #[inline(always)]
    pub fn maskp_0(self) -> &'a mut W {
        self.variant(MASKP20_A::MASKP_0)
    }
    #[doc = "Read MPIN: 0; write MPIN: output bit not affected"]
    #[inline(always)]
    pub fn maskp_1(self) -> &'a mut W {
        self.variant(MASKP20_A::MASKP_1)
    }
}
#[doc = "Field `MASKP21` reader - Port Mask"]
pub type MASKP21_R = crate::BitReader<MASKP21_A>;
#[doc = "Port Mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MASKP21_A {
    #[doc = "0: Read MPIN: pin state; write MPIN: load output bit"]
    MASKP_0 = 0,
    #[doc = "1: Read MPIN: 0; write MPIN: output bit not affected"]
    MASKP_1 = 1,
}
impl From<MASKP21_A> for bool {
    #[inline(always)]
    fn from(variant: MASKP21_A) -> Self {
        variant as u8 != 0
    }
}
impl MASKP21_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MASKP21_A {
        match self.bits {
            false => MASKP21_A::MASKP_0,
            true => MASKP21_A::MASKP_1,
        }
    }
    #[doc = "Checks if the value of the field is `MASKP_0`"]
    #[inline(always)]
    pub fn is_maskp_0(&self) -> bool {
        *self == MASKP21_A::MASKP_0
    }
    #[doc = "Checks if the value of the field is `MASKP_1`"]
    #[inline(always)]
    pub fn is_maskp_1(&self) -> bool {
        *self == MASKP21_A::MASKP_1
    }
}
#[doc = "Field `MASKP21` writer - Port Mask"]
pub type MASKP21_W<'a, const O: u8> = crate::BitWriter<'a, u32, MASK_SPEC, MASKP21_A, O>;
impl<'a, const O: u8> MASKP21_W<'a, O> {
    #[doc = "Read MPIN: pin state; write MPIN: load output bit"]
    #[inline(always)]
    pub fn maskp_0(self) -> &'a mut W {
        self.variant(MASKP21_A::MASKP_0)
    }
    #[doc = "Read MPIN: 0; write MPIN: output bit not affected"]
    #[inline(always)]
    pub fn maskp_1(self) -> &'a mut W {
        self.variant(MASKP21_A::MASKP_1)
    }
}
#[doc = "Field `MASKP22` reader - Port Mask"]
pub type MASKP22_R = crate::BitReader<MASKP22_A>;
#[doc = "Port Mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MASKP22_A {
    #[doc = "0: Read MPIN: pin state; write MPIN: load output bit"]
    MASKP_0 = 0,
    #[doc = "1: Read MPIN: 0; write MPIN: output bit not affected"]
    MASKP_1 = 1,
}
impl From<MASKP22_A> for bool {
    #[inline(always)]
    fn from(variant: MASKP22_A) -> Self {
        variant as u8 != 0
    }
}
impl MASKP22_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MASKP22_A {
        match self.bits {
            false => MASKP22_A::MASKP_0,
            true => MASKP22_A::MASKP_1,
        }
    }
    #[doc = "Checks if the value of the field is `MASKP_0`"]
    #[inline(always)]
    pub fn is_maskp_0(&self) -> bool {
        *self == MASKP22_A::MASKP_0
    }
    #[doc = "Checks if the value of the field is `MASKP_1`"]
    #[inline(always)]
    pub fn is_maskp_1(&self) -> bool {
        *self == MASKP22_A::MASKP_1
    }
}
#[doc = "Field `MASKP22` writer - Port Mask"]
pub type MASKP22_W<'a, const O: u8> = crate::BitWriter<'a, u32, MASK_SPEC, MASKP22_A, O>;
impl<'a, const O: u8> MASKP22_W<'a, O> {
    #[doc = "Read MPIN: pin state; write MPIN: load output bit"]
    #[inline(always)]
    pub fn maskp_0(self) -> &'a mut W {
        self.variant(MASKP22_A::MASKP_0)
    }
    #[doc = "Read MPIN: 0; write MPIN: output bit not affected"]
    #[inline(always)]
    pub fn maskp_1(self) -> &'a mut W {
        self.variant(MASKP22_A::MASKP_1)
    }
}
#[doc = "Field `MASKP23` reader - Port Mask"]
pub type MASKP23_R = crate::BitReader<MASKP23_A>;
#[doc = "Port Mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MASKP23_A {
    #[doc = "0: Read MPIN: pin state; write MPIN: load output bit"]
    MASKP_0 = 0,
    #[doc = "1: Read MPIN: 0; write MPIN: output bit not affected"]
    MASKP_1 = 1,
}
impl From<MASKP23_A> for bool {
    #[inline(always)]
    fn from(variant: MASKP23_A) -> Self {
        variant as u8 != 0
    }
}
impl MASKP23_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MASKP23_A {
        match self.bits {
            false => MASKP23_A::MASKP_0,
            true => MASKP23_A::MASKP_1,
        }
    }
    #[doc = "Checks if the value of the field is `MASKP_0`"]
    #[inline(always)]
    pub fn is_maskp_0(&self) -> bool {
        *self == MASKP23_A::MASKP_0
    }
    #[doc = "Checks if the value of the field is `MASKP_1`"]
    #[inline(always)]
    pub fn is_maskp_1(&self) -> bool {
        *self == MASKP23_A::MASKP_1
    }
}
#[doc = "Field `MASKP23` writer - Port Mask"]
pub type MASKP23_W<'a, const O: u8> = crate::BitWriter<'a, u32, MASK_SPEC, MASKP23_A, O>;
impl<'a, const O: u8> MASKP23_W<'a, O> {
    #[doc = "Read MPIN: pin state; write MPIN: load output bit"]
    #[inline(always)]
    pub fn maskp_0(self) -> &'a mut W {
        self.variant(MASKP23_A::MASKP_0)
    }
    #[doc = "Read MPIN: 0; write MPIN: output bit not affected"]
    #[inline(always)]
    pub fn maskp_1(self) -> &'a mut W {
        self.variant(MASKP23_A::MASKP_1)
    }
}
#[doc = "Field `MASKP24` reader - Port Mask"]
pub type MASKP24_R = crate::BitReader<MASKP24_A>;
#[doc = "Port Mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MASKP24_A {
    #[doc = "0: Read MPIN: pin state; write MPIN: load output bit"]
    MASKP_0 = 0,
    #[doc = "1: Read MPIN: 0; write MPIN: output bit not affected"]
    MASKP_1 = 1,
}
impl From<MASKP24_A> for bool {
    #[inline(always)]
    fn from(variant: MASKP24_A) -> Self {
        variant as u8 != 0
    }
}
impl MASKP24_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MASKP24_A {
        match self.bits {
            false => MASKP24_A::MASKP_0,
            true => MASKP24_A::MASKP_1,
        }
    }
    #[doc = "Checks if the value of the field is `MASKP_0`"]
    #[inline(always)]
    pub fn is_maskp_0(&self) -> bool {
        *self == MASKP24_A::MASKP_0
    }
    #[doc = "Checks if the value of the field is `MASKP_1`"]
    #[inline(always)]
    pub fn is_maskp_1(&self) -> bool {
        *self == MASKP24_A::MASKP_1
    }
}
#[doc = "Field `MASKP24` writer - Port Mask"]
pub type MASKP24_W<'a, const O: u8> = crate::BitWriter<'a, u32, MASK_SPEC, MASKP24_A, O>;
impl<'a, const O: u8> MASKP24_W<'a, O> {
    #[doc = "Read MPIN: pin state; write MPIN: load output bit"]
    #[inline(always)]
    pub fn maskp_0(self) -> &'a mut W {
        self.variant(MASKP24_A::MASKP_0)
    }
    #[doc = "Read MPIN: 0; write MPIN: output bit not affected"]
    #[inline(always)]
    pub fn maskp_1(self) -> &'a mut W {
        self.variant(MASKP24_A::MASKP_1)
    }
}
#[doc = "Field `MASKP25` reader - Port Mask"]
pub type MASKP25_R = crate::BitReader<MASKP25_A>;
#[doc = "Port Mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MASKP25_A {
    #[doc = "0: Read MPIN: pin state; write MPIN: load output bit"]
    MASKP_0 = 0,
    #[doc = "1: Read MPIN: 0; write MPIN: output bit not affected"]
    MASKP_1 = 1,
}
impl From<MASKP25_A> for bool {
    #[inline(always)]
    fn from(variant: MASKP25_A) -> Self {
        variant as u8 != 0
    }
}
impl MASKP25_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MASKP25_A {
        match self.bits {
            false => MASKP25_A::MASKP_0,
            true => MASKP25_A::MASKP_1,
        }
    }
    #[doc = "Checks if the value of the field is `MASKP_0`"]
    #[inline(always)]
    pub fn is_maskp_0(&self) -> bool {
        *self == MASKP25_A::MASKP_0
    }
    #[doc = "Checks if the value of the field is `MASKP_1`"]
    #[inline(always)]
    pub fn is_maskp_1(&self) -> bool {
        *self == MASKP25_A::MASKP_1
    }
}
#[doc = "Field `MASKP25` writer - Port Mask"]
pub type MASKP25_W<'a, const O: u8> = crate::BitWriter<'a, u32, MASK_SPEC, MASKP25_A, O>;
impl<'a, const O: u8> MASKP25_W<'a, O> {
    #[doc = "Read MPIN: pin state; write MPIN: load output bit"]
    #[inline(always)]
    pub fn maskp_0(self) -> &'a mut W {
        self.variant(MASKP25_A::MASKP_0)
    }
    #[doc = "Read MPIN: 0; write MPIN: output bit not affected"]
    #[inline(always)]
    pub fn maskp_1(self) -> &'a mut W {
        self.variant(MASKP25_A::MASKP_1)
    }
}
#[doc = "Field `MASKP26` reader - Port Mask"]
pub type MASKP26_R = crate::BitReader<MASKP26_A>;
#[doc = "Port Mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MASKP26_A {
    #[doc = "0: Read MPIN: pin state; write MPIN: load output bit"]
    MASKP_0 = 0,
    #[doc = "1: Read MPIN: 0; write MPIN: output bit not affected"]
    MASKP_1 = 1,
}
impl From<MASKP26_A> for bool {
    #[inline(always)]
    fn from(variant: MASKP26_A) -> Self {
        variant as u8 != 0
    }
}
impl MASKP26_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MASKP26_A {
        match self.bits {
            false => MASKP26_A::MASKP_0,
            true => MASKP26_A::MASKP_1,
        }
    }
    #[doc = "Checks if the value of the field is `MASKP_0`"]
    #[inline(always)]
    pub fn is_maskp_0(&self) -> bool {
        *self == MASKP26_A::MASKP_0
    }
    #[doc = "Checks if the value of the field is `MASKP_1`"]
    #[inline(always)]
    pub fn is_maskp_1(&self) -> bool {
        *self == MASKP26_A::MASKP_1
    }
}
#[doc = "Field `MASKP26` writer - Port Mask"]
pub type MASKP26_W<'a, const O: u8> = crate::BitWriter<'a, u32, MASK_SPEC, MASKP26_A, O>;
impl<'a, const O: u8> MASKP26_W<'a, O> {
    #[doc = "Read MPIN: pin state; write MPIN: load output bit"]
    #[inline(always)]
    pub fn maskp_0(self) -> &'a mut W {
        self.variant(MASKP26_A::MASKP_0)
    }
    #[doc = "Read MPIN: 0; write MPIN: output bit not affected"]
    #[inline(always)]
    pub fn maskp_1(self) -> &'a mut W {
        self.variant(MASKP26_A::MASKP_1)
    }
}
#[doc = "Field `MASKP27` reader - Port Mask"]
pub type MASKP27_R = crate::BitReader<MASKP27_A>;
#[doc = "Port Mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MASKP27_A {
    #[doc = "0: Read MPIN: pin state; write MPIN: load output bit"]
    MASKP_0 = 0,
    #[doc = "1: Read MPIN: 0; write MPIN: output bit not affected"]
    MASKP_1 = 1,
}
impl From<MASKP27_A> for bool {
    #[inline(always)]
    fn from(variant: MASKP27_A) -> Self {
        variant as u8 != 0
    }
}
impl MASKP27_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MASKP27_A {
        match self.bits {
            false => MASKP27_A::MASKP_0,
            true => MASKP27_A::MASKP_1,
        }
    }
    #[doc = "Checks if the value of the field is `MASKP_0`"]
    #[inline(always)]
    pub fn is_maskp_0(&self) -> bool {
        *self == MASKP27_A::MASKP_0
    }
    #[doc = "Checks if the value of the field is `MASKP_1`"]
    #[inline(always)]
    pub fn is_maskp_1(&self) -> bool {
        *self == MASKP27_A::MASKP_1
    }
}
#[doc = "Field `MASKP27` writer - Port Mask"]
pub type MASKP27_W<'a, const O: u8> = crate::BitWriter<'a, u32, MASK_SPEC, MASKP27_A, O>;
impl<'a, const O: u8> MASKP27_W<'a, O> {
    #[doc = "Read MPIN: pin state; write MPIN: load output bit"]
    #[inline(always)]
    pub fn maskp_0(self) -> &'a mut W {
        self.variant(MASKP27_A::MASKP_0)
    }
    #[doc = "Read MPIN: 0; write MPIN: output bit not affected"]
    #[inline(always)]
    pub fn maskp_1(self) -> &'a mut W {
        self.variant(MASKP27_A::MASKP_1)
    }
}
#[doc = "Field `MASKP28` reader - Port Mask"]
pub type MASKP28_R = crate::BitReader<MASKP28_A>;
#[doc = "Port Mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MASKP28_A {
    #[doc = "0: Read MPIN: pin state; write MPIN: load output bit"]
    MASKP_0 = 0,
    #[doc = "1: Read MPIN: 0; write MPIN: output bit not affected"]
    MASKP_1 = 1,
}
impl From<MASKP28_A> for bool {
    #[inline(always)]
    fn from(variant: MASKP28_A) -> Self {
        variant as u8 != 0
    }
}
impl MASKP28_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MASKP28_A {
        match self.bits {
            false => MASKP28_A::MASKP_0,
            true => MASKP28_A::MASKP_1,
        }
    }
    #[doc = "Checks if the value of the field is `MASKP_0`"]
    #[inline(always)]
    pub fn is_maskp_0(&self) -> bool {
        *self == MASKP28_A::MASKP_0
    }
    #[doc = "Checks if the value of the field is `MASKP_1`"]
    #[inline(always)]
    pub fn is_maskp_1(&self) -> bool {
        *self == MASKP28_A::MASKP_1
    }
}
#[doc = "Field `MASKP28` writer - Port Mask"]
pub type MASKP28_W<'a, const O: u8> = crate::BitWriter<'a, u32, MASK_SPEC, MASKP28_A, O>;
impl<'a, const O: u8> MASKP28_W<'a, O> {
    #[doc = "Read MPIN: pin state; write MPIN: load output bit"]
    #[inline(always)]
    pub fn maskp_0(self) -> &'a mut W {
        self.variant(MASKP28_A::MASKP_0)
    }
    #[doc = "Read MPIN: 0; write MPIN: output bit not affected"]
    #[inline(always)]
    pub fn maskp_1(self) -> &'a mut W {
        self.variant(MASKP28_A::MASKP_1)
    }
}
#[doc = "Field `MASKP29` reader - Port Mask"]
pub type MASKP29_R = crate::BitReader<MASKP29_A>;
#[doc = "Port Mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MASKP29_A {
    #[doc = "0: Read MPIN: pin state; write MPIN: load output bit"]
    MASKP_0 = 0,
    #[doc = "1: Read MPIN: 0; write MPIN: output bit not affected"]
    MASKP_1 = 1,
}
impl From<MASKP29_A> for bool {
    #[inline(always)]
    fn from(variant: MASKP29_A) -> Self {
        variant as u8 != 0
    }
}
impl MASKP29_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MASKP29_A {
        match self.bits {
            false => MASKP29_A::MASKP_0,
            true => MASKP29_A::MASKP_1,
        }
    }
    #[doc = "Checks if the value of the field is `MASKP_0`"]
    #[inline(always)]
    pub fn is_maskp_0(&self) -> bool {
        *self == MASKP29_A::MASKP_0
    }
    #[doc = "Checks if the value of the field is `MASKP_1`"]
    #[inline(always)]
    pub fn is_maskp_1(&self) -> bool {
        *self == MASKP29_A::MASKP_1
    }
}
#[doc = "Field `MASKP29` writer - Port Mask"]
pub type MASKP29_W<'a, const O: u8> = crate::BitWriter<'a, u32, MASK_SPEC, MASKP29_A, O>;
impl<'a, const O: u8> MASKP29_W<'a, O> {
    #[doc = "Read MPIN: pin state; write MPIN: load output bit"]
    #[inline(always)]
    pub fn maskp_0(self) -> &'a mut W {
        self.variant(MASKP29_A::MASKP_0)
    }
    #[doc = "Read MPIN: 0; write MPIN: output bit not affected"]
    #[inline(always)]
    pub fn maskp_1(self) -> &'a mut W {
        self.variant(MASKP29_A::MASKP_1)
    }
}
#[doc = "Field `MASKP30` reader - Port Mask"]
pub type MASKP30_R = crate::BitReader<MASKP30_A>;
#[doc = "Port Mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MASKP30_A {
    #[doc = "0: Read MPIN: pin state; write MPIN: load output bit"]
    MASKP_0 = 0,
    #[doc = "1: Read MPIN: 0; write MPIN: output bit not affected"]
    MASKP_1 = 1,
}
impl From<MASKP30_A> for bool {
    #[inline(always)]
    fn from(variant: MASKP30_A) -> Self {
        variant as u8 != 0
    }
}
impl MASKP30_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MASKP30_A {
        match self.bits {
            false => MASKP30_A::MASKP_0,
            true => MASKP30_A::MASKP_1,
        }
    }
    #[doc = "Checks if the value of the field is `MASKP_0`"]
    #[inline(always)]
    pub fn is_maskp_0(&self) -> bool {
        *self == MASKP30_A::MASKP_0
    }
    #[doc = "Checks if the value of the field is `MASKP_1`"]
    #[inline(always)]
    pub fn is_maskp_1(&self) -> bool {
        *self == MASKP30_A::MASKP_1
    }
}
#[doc = "Field `MASKP30` writer - Port Mask"]
pub type MASKP30_W<'a, const O: u8> = crate::BitWriter<'a, u32, MASK_SPEC, MASKP30_A, O>;
impl<'a, const O: u8> MASKP30_W<'a, O> {
    #[doc = "Read MPIN: pin state; write MPIN: load output bit"]
    #[inline(always)]
    pub fn maskp_0(self) -> &'a mut W {
        self.variant(MASKP30_A::MASKP_0)
    }
    #[doc = "Read MPIN: 0; write MPIN: output bit not affected"]
    #[inline(always)]
    pub fn maskp_1(self) -> &'a mut W {
        self.variant(MASKP30_A::MASKP_1)
    }
}
#[doc = "Field `MASKP31` reader - Port Mask"]
pub type MASKP31_R = crate::BitReader<MASKP31_A>;
#[doc = "Port Mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MASKP31_A {
    #[doc = "0: Read MPIN: pin state; write MPIN: load output bit"]
    MASKP_0 = 0,
    #[doc = "1: Read MPIN: 0; write MPIN: output bit not affected"]
    MASKP_1 = 1,
}
impl From<MASKP31_A> for bool {
    #[inline(always)]
    fn from(variant: MASKP31_A) -> Self {
        variant as u8 != 0
    }
}
impl MASKP31_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MASKP31_A {
        match self.bits {
            false => MASKP31_A::MASKP_0,
            true => MASKP31_A::MASKP_1,
        }
    }
    #[doc = "Checks if the value of the field is `MASKP_0`"]
    #[inline(always)]
    pub fn is_maskp_0(&self) -> bool {
        *self == MASKP31_A::MASKP_0
    }
    #[doc = "Checks if the value of the field is `MASKP_1`"]
    #[inline(always)]
    pub fn is_maskp_1(&self) -> bool {
        *self == MASKP31_A::MASKP_1
    }
}
#[doc = "Field `MASKP31` writer - Port Mask"]
pub type MASKP31_W<'a, const O: u8> = crate::BitWriter<'a, u32, MASK_SPEC, MASKP31_A, O>;
impl<'a, const O: u8> MASKP31_W<'a, O> {
    #[doc = "Read MPIN: pin state; write MPIN: load output bit"]
    #[inline(always)]
    pub fn maskp_0(self) -> &'a mut W {
        self.variant(MASKP31_A::MASKP_0)
    }
    #[doc = "Read MPIN: 0; write MPIN: output bit not affected"]
    #[inline(always)]
    pub fn maskp_1(self) -> &'a mut W {
        self.variant(MASKP31_A::MASKP_1)
    }
}
impl R {
    #[doc = "Bit 0 - Port Mask"]
    #[inline(always)]
    pub fn maskp0(&self) -> MASKP0_R {
        MASKP0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Port Mask"]
    #[inline(always)]
    pub fn maskp1(&self) -> MASKP1_R {
        MASKP1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Port Mask"]
    #[inline(always)]
    pub fn maskp2(&self) -> MASKP2_R {
        MASKP2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Port Mask"]
    #[inline(always)]
    pub fn maskp3(&self) -> MASKP3_R {
        MASKP3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Port Mask"]
    #[inline(always)]
    pub fn maskp4(&self) -> MASKP4_R {
        MASKP4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Port Mask"]
    #[inline(always)]
    pub fn maskp5(&self) -> MASKP5_R {
        MASKP5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Port Mask"]
    #[inline(always)]
    pub fn maskp6(&self) -> MASKP6_R {
        MASKP6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Port Mask"]
    #[inline(always)]
    pub fn maskp7(&self) -> MASKP7_R {
        MASKP7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Port Mask"]
    #[inline(always)]
    pub fn maskp8(&self) -> MASKP8_R {
        MASKP8_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Port Mask"]
    #[inline(always)]
    pub fn maskp9(&self) -> MASKP9_R {
        MASKP9_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Port Mask"]
    #[inline(always)]
    pub fn maskp10(&self) -> MASKP10_R {
        MASKP10_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Port Mask"]
    #[inline(always)]
    pub fn maskp11(&self) -> MASKP11_R {
        MASKP11_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Port Mask"]
    #[inline(always)]
    pub fn maskp12(&self) -> MASKP12_R {
        MASKP12_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Port Mask"]
    #[inline(always)]
    pub fn maskp13(&self) -> MASKP13_R {
        MASKP13_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Port Mask"]
    #[inline(always)]
    pub fn maskp14(&self) -> MASKP14_R {
        MASKP14_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Port Mask"]
    #[inline(always)]
    pub fn maskp15(&self) -> MASKP15_R {
        MASKP15_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Port Mask"]
    #[inline(always)]
    pub fn maskp16(&self) -> MASKP16_R {
        MASKP16_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Port Mask"]
    #[inline(always)]
    pub fn maskp17(&self) -> MASKP17_R {
        MASKP17_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Port Mask"]
    #[inline(always)]
    pub fn maskp18(&self) -> MASKP18_R {
        MASKP18_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Port Mask"]
    #[inline(always)]
    pub fn maskp19(&self) -> MASKP19_R {
        MASKP19_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Port Mask"]
    #[inline(always)]
    pub fn maskp20(&self) -> MASKP20_R {
        MASKP20_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Port Mask"]
    #[inline(always)]
    pub fn maskp21(&self) -> MASKP21_R {
        MASKP21_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Port Mask"]
    #[inline(always)]
    pub fn maskp22(&self) -> MASKP22_R {
        MASKP22_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Port Mask"]
    #[inline(always)]
    pub fn maskp23(&self) -> MASKP23_R {
        MASKP23_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Port Mask"]
    #[inline(always)]
    pub fn maskp24(&self) -> MASKP24_R {
        MASKP24_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Port Mask"]
    #[inline(always)]
    pub fn maskp25(&self) -> MASKP25_R {
        MASKP25_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Port Mask"]
    #[inline(always)]
    pub fn maskp26(&self) -> MASKP26_R {
        MASKP26_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Port Mask"]
    #[inline(always)]
    pub fn maskp27(&self) -> MASKP27_R {
        MASKP27_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Port Mask"]
    #[inline(always)]
    pub fn maskp28(&self) -> MASKP28_R {
        MASKP28_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Port Mask"]
    #[inline(always)]
    pub fn maskp29(&self) -> MASKP29_R {
        MASKP29_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Port Mask"]
    #[inline(always)]
    pub fn maskp30(&self) -> MASKP30_R {
        MASKP30_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Port Mask"]
    #[inline(always)]
    pub fn maskp31(&self) -> MASKP31_R {
        MASKP31_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Port Mask"]
    #[inline(always)]
    #[must_use]
    pub fn maskp0(&mut self) -> MASKP0_W<0> {
        MASKP0_W::new(self)
    }
    #[doc = "Bit 1 - Port Mask"]
    #[inline(always)]
    #[must_use]
    pub fn maskp1(&mut self) -> MASKP1_W<1> {
        MASKP1_W::new(self)
    }
    #[doc = "Bit 2 - Port Mask"]
    #[inline(always)]
    #[must_use]
    pub fn maskp2(&mut self) -> MASKP2_W<2> {
        MASKP2_W::new(self)
    }
    #[doc = "Bit 3 - Port Mask"]
    #[inline(always)]
    #[must_use]
    pub fn maskp3(&mut self) -> MASKP3_W<3> {
        MASKP3_W::new(self)
    }
    #[doc = "Bit 4 - Port Mask"]
    #[inline(always)]
    #[must_use]
    pub fn maskp4(&mut self) -> MASKP4_W<4> {
        MASKP4_W::new(self)
    }
    #[doc = "Bit 5 - Port Mask"]
    #[inline(always)]
    #[must_use]
    pub fn maskp5(&mut self) -> MASKP5_W<5> {
        MASKP5_W::new(self)
    }
    #[doc = "Bit 6 - Port Mask"]
    #[inline(always)]
    #[must_use]
    pub fn maskp6(&mut self) -> MASKP6_W<6> {
        MASKP6_W::new(self)
    }
    #[doc = "Bit 7 - Port Mask"]
    #[inline(always)]
    #[must_use]
    pub fn maskp7(&mut self) -> MASKP7_W<7> {
        MASKP7_W::new(self)
    }
    #[doc = "Bit 8 - Port Mask"]
    #[inline(always)]
    #[must_use]
    pub fn maskp8(&mut self) -> MASKP8_W<8> {
        MASKP8_W::new(self)
    }
    #[doc = "Bit 9 - Port Mask"]
    #[inline(always)]
    #[must_use]
    pub fn maskp9(&mut self) -> MASKP9_W<9> {
        MASKP9_W::new(self)
    }
    #[doc = "Bit 10 - Port Mask"]
    #[inline(always)]
    #[must_use]
    pub fn maskp10(&mut self) -> MASKP10_W<10> {
        MASKP10_W::new(self)
    }
    #[doc = "Bit 11 - Port Mask"]
    #[inline(always)]
    #[must_use]
    pub fn maskp11(&mut self) -> MASKP11_W<11> {
        MASKP11_W::new(self)
    }
    #[doc = "Bit 12 - Port Mask"]
    #[inline(always)]
    #[must_use]
    pub fn maskp12(&mut self) -> MASKP12_W<12> {
        MASKP12_W::new(self)
    }
    #[doc = "Bit 13 - Port Mask"]
    #[inline(always)]
    #[must_use]
    pub fn maskp13(&mut self) -> MASKP13_W<13> {
        MASKP13_W::new(self)
    }
    #[doc = "Bit 14 - Port Mask"]
    #[inline(always)]
    #[must_use]
    pub fn maskp14(&mut self) -> MASKP14_W<14> {
        MASKP14_W::new(self)
    }
    #[doc = "Bit 15 - Port Mask"]
    #[inline(always)]
    #[must_use]
    pub fn maskp15(&mut self) -> MASKP15_W<15> {
        MASKP15_W::new(self)
    }
    #[doc = "Bit 16 - Port Mask"]
    #[inline(always)]
    #[must_use]
    pub fn maskp16(&mut self) -> MASKP16_W<16> {
        MASKP16_W::new(self)
    }
    #[doc = "Bit 17 - Port Mask"]
    #[inline(always)]
    #[must_use]
    pub fn maskp17(&mut self) -> MASKP17_W<17> {
        MASKP17_W::new(self)
    }
    #[doc = "Bit 18 - Port Mask"]
    #[inline(always)]
    #[must_use]
    pub fn maskp18(&mut self) -> MASKP18_W<18> {
        MASKP18_W::new(self)
    }
    #[doc = "Bit 19 - Port Mask"]
    #[inline(always)]
    #[must_use]
    pub fn maskp19(&mut self) -> MASKP19_W<19> {
        MASKP19_W::new(self)
    }
    #[doc = "Bit 20 - Port Mask"]
    #[inline(always)]
    #[must_use]
    pub fn maskp20(&mut self) -> MASKP20_W<20> {
        MASKP20_W::new(self)
    }
    #[doc = "Bit 21 - Port Mask"]
    #[inline(always)]
    #[must_use]
    pub fn maskp21(&mut self) -> MASKP21_W<21> {
        MASKP21_W::new(self)
    }
    #[doc = "Bit 22 - Port Mask"]
    #[inline(always)]
    #[must_use]
    pub fn maskp22(&mut self) -> MASKP22_W<22> {
        MASKP22_W::new(self)
    }
    #[doc = "Bit 23 - Port Mask"]
    #[inline(always)]
    #[must_use]
    pub fn maskp23(&mut self) -> MASKP23_W<23> {
        MASKP23_W::new(self)
    }
    #[doc = "Bit 24 - Port Mask"]
    #[inline(always)]
    #[must_use]
    pub fn maskp24(&mut self) -> MASKP24_W<24> {
        MASKP24_W::new(self)
    }
    #[doc = "Bit 25 - Port Mask"]
    #[inline(always)]
    #[must_use]
    pub fn maskp25(&mut self) -> MASKP25_W<25> {
        MASKP25_W::new(self)
    }
    #[doc = "Bit 26 - Port Mask"]
    #[inline(always)]
    #[must_use]
    pub fn maskp26(&mut self) -> MASKP26_W<26> {
        MASKP26_W::new(self)
    }
    #[doc = "Bit 27 - Port Mask"]
    #[inline(always)]
    #[must_use]
    pub fn maskp27(&mut self) -> MASKP27_W<27> {
        MASKP27_W::new(self)
    }
    #[doc = "Bit 28 - Port Mask"]
    #[inline(always)]
    #[must_use]
    pub fn maskp28(&mut self) -> MASKP28_W<28> {
        MASKP28_W::new(self)
    }
    #[doc = "Bit 29 - Port Mask"]
    #[inline(always)]
    #[must_use]
    pub fn maskp29(&mut self) -> MASKP29_W<29> {
        MASKP29_W::new(self)
    }
    #[doc = "Bit 30 - Port Mask"]
    #[inline(always)]
    #[must_use]
    pub fn maskp30(&mut self) -> MASKP30_W<30> {
        MASKP30_W::new(self)
    }
    #[doc = "Bit 31 - Port Mask"]
    #[inline(always)]
    #[must_use]
    pub fn maskp31(&mut self) -> MASKP31_W<31> {
        MASKP31_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Port mask\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mask](index.html) module"]
pub struct MASK_SPEC;
impl crate::RegisterSpec for MASK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mask::R](R) reader structure"]
impl crate::Readable for MASK_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mask::W](W) writer structure"]
impl crate::Writable for MASK_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MASK[%s]
to value 0"]
impl crate::Resettable for MASK_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
