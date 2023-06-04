#[doc = "Register `CLR0` reader"]
pub struct R(crate::R<CLR0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CLR0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CLR0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CLR0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CLR0` writer"]
pub struct W(crate::W<CLR0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CLR0_SPEC>;
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
impl From<crate::W<CLR0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CLR0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CLRP0` reader - Clear output bits"]
pub type CLRP0_R = crate::BitReader<CLRP0_A>;
#[doc = "Clear output bits\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CLRP0_A {
    #[doc = "0: No operation"]
    CLRP_0 = 0,
    #[doc = "1: Clears output bit"]
    CLRP_1 = 1,
}
impl From<CLRP0_A> for bool {
    #[inline(always)]
    fn from(variant: CLRP0_A) -> Self {
        variant as u8 != 0
    }
}
impl CLRP0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CLRP0_A {
        match self.bits {
            false => CLRP0_A::CLRP_0,
            true => CLRP0_A::CLRP_1,
        }
    }
    #[doc = "Checks if the value of the field is `CLRP_0`"]
    #[inline(always)]
    pub fn is_clrp_0(&self) -> bool {
        *self == CLRP0_A::CLRP_0
    }
    #[doc = "Checks if the value of the field is `CLRP_1`"]
    #[inline(always)]
    pub fn is_clrp_1(&self) -> bool {
        *self == CLRP0_A::CLRP_1
    }
}
#[doc = "Field `CLRP0` writer - Clear output bits"]
pub type CLRP0_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, CLR0_SPEC, CLRP0_A, O>;
impl<'a, const O: u8> CLRP0_W<'a, O> {
    #[doc = "No operation"]
    #[inline(always)]
    pub fn clrp_0(self) -> &'a mut W {
        self.variant(CLRP0_A::CLRP_0)
    }
    #[doc = "Clears output bit"]
    #[inline(always)]
    pub fn clrp_1(self) -> &'a mut W {
        self.variant(CLRP0_A::CLRP_1)
    }
}
#[doc = "Field `CLRP1` reader - Clear output bits"]
pub type CLRP1_R = crate::BitReader<CLRP1_A>;
#[doc = "Clear output bits\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CLRP1_A {
    #[doc = "0: No operation"]
    CLRP_0 = 0,
    #[doc = "1: Clears output bit"]
    CLRP_1 = 1,
}
impl From<CLRP1_A> for bool {
    #[inline(always)]
    fn from(variant: CLRP1_A) -> Self {
        variant as u8 != 0
    }
}
impl CLRP1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CLRP1_A {
        match self.bits {
            false => CLRP1_A::CLRP_0,
            true => CLRP1_A::CLRP_1,
        }
    }
    #[doc = "Checks if the value of the field is `CLRP_0`"]
    #[inline(always)]
    pub fn is_clrp_0(&self) -> bool {
        *self == CLRP1_A::CLRP_0
    }
    #[doc = "Checks if the value of the field is `CLRP_1`"]
    #[inline(always)]
    pub fn is_clrp_1(&self) -> bool {
        *self == CLRP1_A::CLRP_1
    }
}
#[doc = "Field `CLRP1` writer - Clear output bits"]
pub type CLRP1_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, CLR0_SPEC, CLRP1_A, O>;
impl<'a, const O: u8> CLRP1_W<'a, O> {
    #[doc = "No operation"]
    #[inline(always)]
    pub fn clrp_0(self) -> &'a mut W {
        self.variant(CLRP1_A::CLRP_0)
    }
    #[doc = "Clears output bit"]
    #[inline(always)]
    pub fn clrp_1(self) -> &'a mut W {
        self.variant(CLRP1_A::CLRP_1)
    }
}
#[doc = "Field `CLRP2` reader - Clear output bits"]
pub type CLRP2_R = crate::BitReader<CLRP2_A>;
#[doc = "Clear output bits\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CLRP2_A {
    #[doc = "0: No operation"]
    CLRP_0 = 0,
    #[doc = "1: Clears output bit"]
    CLRP_1 = 1,
}
impl From<CLRP2_A> for bool {
    #[inline(always)]
    fn from(variant: CLRP2_A) -> Self {
        variant as u8 != 0
    }
}
impl CLRP2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CLRP2_A {
        match self.bits {
            false => CLRP2_A::CLRP_0,
            true => CLRP2_A::CLRP_1,
        }
    }
    #[doc = "Checks if the value of the field is `CLRP_0`"]
    #[inline(always)]
    pub fn is_clrp_0(&self) -> bool {
        *self == CLRP2_A::CLRP_0
    }
    #[doc = "Checks if the value of the field is `CLRP_1`"]
    #[inline(always)]
    pub fn is_clrp_1(&self) -> bool {
        *self == CLRP2_A::CLRP_1
    }
}
#[doc = "Field `CLRP2` writer - Clear output bits"]
pub type CLRP2_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, CLR0_SPEC, CLRP2_A, O>;
impl<'a, const O: u8> CLRP2_W<'a, O> {
    #[doc = "No operation"]
    #[inline(always)]
    pub fn clrp_0(self) -> &'a mut W {
        self.variant(CLRP2_A::CLRP_0)
    }
    #[doc = "Clears output bit"]
    #[inline(always)]
    pub fn clrp_1(self) -> &'a mut W {
        self.variant(CLRP2_A::CLRP_1)
    }
}
#[doc = "Field `CLRP3` reader - Clear output bits"]
pub type CLRP3_R = crate::BitReader<CLRP3_A>;
#[doc = "Clear output bits\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CLRP3_A {
    #[doc = "0: No operation"]
    CLRP_0 = 0,
    #[doc = "1: Clears output bit"]
    CLRP_1 = 1,
}
impl From<CLRP3_A> for bool {
    #[inline(always)]
    fn from(variant: CLRP3_A) -> Self {
        variant as u8 != 0
    }
}
impl CLRP3_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CLRP3_A {
        match self.bits {
            false => CLRP3_A::CLRP_0,
            true => CLRP3_A::CLRP_1,
        }
    }
    #[doc = "Checks if the value of the field is `CLRP_0`"]
    #[inline(always)]
    pub fn is_clrp_0(&self) -> bool {
        *self == CLRP3_A::CLRP_0
    }
    #[doc = "Checks if the value of the field is `CLRP_1`"]
    #[inline(always)]
    pub fn is_clrp_1(&self) -> bool {
        *self == CLRP3_A::CLRP_1
    }
}
#[doc = "Field `CLRP3` writer - Clear output bits"]
pub type CLRP3_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, CLR0_SPEC, CLRP3_A, O>;
impl<'a, const O: u8> CLRP3_W<'a, O> {
    #[doc = "No operation"]
    #[inline(always)]
    pub fn clrp_0(self) -> &'a mut W {
        self.variant(CLRP3_A::CLRP_0)
    }
    #[doc = "Clears output bit"]
    #[inline(always)]
    pub fn clrp_1(self) -> &'a mut W {
        self.variant(CLRP3_A::CLRP_1)
    }
}
#[doc = "Field `CLRP4` reader - Clear output bits"]
pub type CLRP4_R = crate::BitReader<CLRP4_A>;
#[doc = "Clear output bits\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CLRP4_A {
    #[doc = "0: No operation"]
    CLRP_0 = 0,
    #[doc = "1: Clears output bit"]
    CLRP_1 = 1,
}
impl From<CLRP4_A> for bool {
    #[inline(always)]
    fn from(variant: CLRP4_A) -> Self {
        variant as u8 != 0
    }
}
impl CLRP4_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CLRP4_A {
        match self.bits {
            false => CLRP4_A::CLRP_0,
            true => CLRP4_A::CLRP_1,
        }
    }
    #[doc = "Checks if the value of the field is `CLRP_0`"]
    #[inline(always)]
    pub fn is_clrp_0(&self) -> bool {
        *self == CLRP4_A::CLRP_0
    }
    #[doc = "Checks if the value of the field is `CLRP_1`"]
    #[inline(always)]
    pub fn is_clrp_1(&self) -> bool {
        *self == CLRP4_A::CLRP_1
    }
}
#[doc = "Field `CLRP4` writer - Clear output bits"]
pub type CLRP4_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, CLR0_SPEC, CLRP4_A, O>;
impl<'a, const O: u8> CLRP4_W<'a, O> {
    #[doc = "No operation"]
    #[inline(always)]
    pub fn clrp_0(self) -> &'a mut W {
        self.variant(CLRP4_A::CLRP_0)
    }
    #[doc = "Clears output bit"]
    #[inline(always)]
    pub fn clrp_1(self) -> &'a mut W {
        self.variant(CLRP4_A::CLRP_1)
    }
}
#[doc = "Field `CLRP5` reader - Clear output bits"]
pub type CLRP5_R = crate::BitReader<CLRP5_A>;
#[doc = "Clear output bits\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CLRP5_A {
    #[doc = "0: No operation"]
    CLRP_0 = 0,
    #[doc = "1: Clears output bit"]
    CLRP_1 = 1,
}
impl From<CLRP5_A> for bool {
    #[inline(always)]
    fn from(variant: CLRP5_A) -> Self {
        variant as u8 != 0
    }
}
impl CLRP5_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CLRP5_A {
        match self.bits {
            false => CLRP5_A::CLRP_0,
            true => CLRP5_A::CLRP_1,
        }
    }
    #[doc = "Checks if the value of the field is `CLRP_0`"]
    #[inline(always)]
    pub fn is_clrp_0(&self) -> bool {
        *self == CLRP5_A::CLRP_0
    }
    #[doc = "Checks if the value of the field is `CLRP_1`"]
    #[inline(always)]
    pub fn is_clrp_1(&self) -> bool {
        *self == CLRP5_A::CLRP_1
    }
}
#[doc = "Field `CLRP5` writer - Clear output bits"]
pub type CLRP5_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, CLR0_SPEC, CLRP5_A, O>;
impl<'a, const O: u8> CLRP5_W<'a, O> {
    #[doc = "No operation"]
    #[inline(always)]
    pub fn clrp_0(self) -> &'a mut W {
        self.variant(CLRP5_A::CLRP_0)
    }
    #[doc = "Clears output bit"]
    #[inline(always)]
    pub fn clrp_1(self) -> &'a mut W {
        self.variant(CLRP5_A::CLRP_1)
    }
}
#[doc = "Field `CLRP6` reader - Clear output bits"]
pub type CLRP6_R = crate::BitReader<CLRP6_A>;
#[doc = "Clear output bits\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CLRP6_A {
    #[doc = "0: No operation"]
    CLRP_0 = 0,
    #[doc = "1: Clears output bit"]
    CLRP_1 = 1,
}
impl From<CLRP6_A> for bool {
    #[inline(always)]
    fn from(variant: CLRP6_A) -> Self {
        variant as u8 != 0
    }
}
impl CLRP6_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CLRP6_A {
        match self.bits {
            false => CLRP6_A::CLRP_0,
            true => CLRP6_A::CLRP_1,
        }
    }
    #[doc = "Checks if the value of the field is `CLRP_0`"]
    #[inline(always)]
    pub fn is_clrp_0(&self) -> bool {
        *self == CLRP6_A::CLRP_0
    }
    #[doc = "Checks if the value of the field is `CLRP_1`"]
    #[inline(always)]
    pub fn is_clrp_1(&self) -> bool {
        *self == CLRP6_A::CLRP_1
    }
}
#[doc = "Field `CLRP6` writer - Clear output bits"]
pub type CLRP6_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, CLR0_SPEC, CLRP6_A, O>;
impl<'a, const O: u8> CLRP6_W<'a, O> {
    #[doc = "No operation"]
    #[inline(always)]
    pub fn clrp_0(self) -> &'a mut W {
        self.variant(CLRP6_A::CLRP_0)
    }
    #[doc = "Clears output bit"]
    #[inline(always)]
    pub fn clrp_1(self) -> &'a mut W {
        self.variant(CLRP6_A::CLRP_1)
    }
}
#[doc = "Field `CLRP7` reader - Clear output bits"]
pub type CLRP7_R = crate::BitReader<CLRP7_A>;
#[doc = "Clear output bits\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CLRP7_A {
    #[doc = "0: No operation"]
    CLRP_0 = 0,
    #[doc = "1: Clears output bit"]
    CLRP_1 = 1,
}
impl From<CLRP7_A> for bool {
    #[inline(always)]
    fn from(variant: CLRP7_A) -> Self {
        variant as u8 != 0
    }
}
impl CLRP7_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CLRP7_A {
        match self.bits {
            false => CLRP7_A::CLRP_0,
            true => CLRP7_A::CLRP_1,
        }
    }
    #[doc = "Checks if the value of the field is `CLRP_0`"]
    #[inline(always)]
    pub fn is_clrp_0(&self) -> bool {
        *self == CLRP7_A::CLRP_0
    }
    #[doc = "Checks if the value of the field is `CLRP_1`"]
    #[inline(always)]
    pub fn is_clrp_1(&self) -> bool {
        *self == CLRP7_A::CLRP_1
    }
}
#[doc = "Field `CLRP7` writer - Clear output bits"]
pub type CLRP7_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, CLR0_SPEC, CLRP7_A, O>;
impl<'a, const O: u8> CLRP7_W<'a, O> {
    #[doc = "No operation"]
    #[inline(always)]
    pub fn clrp_0(self) -> &'a mut W {
        self.variant(CLRP7_A::CLRP_0)
    }
    #[doc = "Clears output bit"]
    #[inline(always)]
    pub fn clrp_1(self) -> &'a mut W {
        self.variant(CLRP7_A::CLRP_1)
    }
}
#[doc = "Field `CLRP8` reader - Clear output bits"]
pub type CLRP8_R = crate::BitReader<CLRP8_A>;
#[doc = "Clear output bits\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CLRP8_A {
    #[doc = "0: No operation"]
    CLRP_0 = 0,
    #[doc = "1: Clears output bit"]
    CLRP_1 = 1,
}
impl From<CLRP8_A> for bool {
    #[inline(always)]
    fn from(variant: CLRP8_A) -> Self {
        variant as u8 != 0
    }
}
impl CLRP8_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CLRP8_A {
        match self.bits {
            false => CLRP8_A::CLRP_0,
            true => CLRP8_A::CLRP_1,
        }
    }
    #[doc = "Checks if the value of the field is `CLRP_0`"]
    #[inline(always)]
    pub fn is_clrp_0(&self) -> bool {
        *self == CLRP8_A::CLRP_0
    }
    #[doc = "Checks if the value of the field is `CLRP_1`"]
    #[inline(always)]
    pub fn is_clrp_1(&self) -> bool {
        *self == CLRP8_A::CLRP_1
    }
}
#[doc = "Field `CLRP8` writer - Clear output bits"]
pub type CLRP8_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, CLR0_SPEC, CLRP8_A, O>;
impl<'a, const O: u8> CLRP8_W<'a, O> {
    #[doc = "No operation"]
    #[inline(always)]
    pub fn clrp_0(self) -> &'a mut W {
        self.variant(CLRP8_A::CLRP_0)
    }
    #[doc = "Clears output bit"]
    #[inline(always)]
    pub fn clrp_1(self) -> &'a mut W {
        self.variant(CLRP8_A::CLRP_1)
    }
}
#[doc = "Field `CLRP9` reader - Clear output bits"]
pub type CLRP9_R = crate::BitReader<CLRP9_A>;
#[doc = "Clear output bits\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CLRP9_A {
    #[doc = "0: No operation"]
    CLRP_0 = 0,
    #[doc = "1: Clears output bit"]
    CLRP_1 = 1,
}
impl From<CLRP9_A> for bool {
    #[inline(always)]
    fn from(variant: CLRP9_A) -> Self {
        variant as u8 != 0
    }
}
impl CLRP9_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CLRP9_A {
        match self.bits {
            false => CLRP9_A::CLRP_0,
            true => CLRP9_A::CLRP_1,
        }
    }
    #[doc = "Checks if the value of the field is `CLRP_0`"]
    #[inline(always)]
    pub fn is_clrp_0(&self) -> bool {
        *self == CLRP9_A::CLRP_0
    }
    #[doc = "Checks if the value of the field is `CLRP_1`"]
    #[inline(always)]
    pub fn is_clrp_1(&self) -> bool {
        *self == CLRP9_A::CLRP_1
    }
}
#[doc = "Field `CLRP9` writer - Clear output bits"]
pub type CLRP9_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, CLR0_SPEC, CLRP9_A, O>;
impl<'a, const O: u8> CLRP9_W<'a, O> {
    #[doc = "No operation"]
    #[inline(always)]
    pub fn clrp_0(self) -> &'a mut W {
        self.variant(CLRP9_A::CLRP_0)
    }
    #[doc = "Clears output bit"]
    #[inline(always)]
    pub fn clrp_1(self) -> &'a mut W {
        self.variant(CLRP9_A::CLRP_1)
    }
}
#[doc = "Field `CLRP10` reader - Clear output bits"]
pub type CLRP10_R = crate::BitReader<CLRP10_A>;
#[doc = "Clear output bits\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CLRP10_A {
    #[doc = "0: No operation"]
    CLRP_0 = 0,
    #[doc = "1: Clears output bit"]
    CLRP_1 = 1,
}
impl From<CLRP10_A> for bool {
    #[inline(always)]
    fn from(variant: CLRP10_A) -> Self {
        variant as u8 != 0
    }
}
impl CLRP10_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CLRP10_A {
        match self.bits {
            false => CLRP10_A::CLRP_0,
            true => CLRP10_A::CLRP_1,
        }
    }
    #[doc = "Checks if the value of the field is `CLRP_0`"]
    #[inline(always)]
    pub fn is_clrp_0(&self) -> bool {
        *self == CLRP10_A::CLRP_0
    }
    #[doc = "Checks if the value of the field is `CLRP_1`"]
    #[inline(always)]
    pub fn is_clrp_1(&self) -> bool {
        *self == CLRP10_A::CLRP_1
    }
}
#[doc = "Field `CLRP10` writer - Clear output bits"]
pub type CLRP10_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, CLR0_SPEC, CLRP10_A, O>;
impl<'a, const O: u8> CLRP10_W<'a, O> {
    #[doc = "No operation"]
    #[inline(always)]
    pub fn clrp_0(self) -> &'a mut W {
        self.variant(CLRP10_A::CLRP_0)
    }
    #[doc = "Clears output bit"]
    #[inline(always)]
    pub fn clrp_1(self) -> &'a mut W {
        self.variant(CLRP10_A::CLRP_1)
    }
}
#[doc = "Field `CLRP11` reader - Clear output bits"]
pub type CLRP11_R = crate::BitReader<CLRP11_A>;
#[doc = "Clear output bits\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CLRP11_A {
    #[doc = "0: No operation"]
    CLRP_0 = 0,
    #[doc = "1: Clears output bit"]
    CLRP_1 = 1,
}
impl From<CLRP11_A> for bool {
    #[inline(always)]
    fn from(variant: CLRP11_A) -> Self {
        variant as u8 != 0
    }
}
impl CLRP11_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CLRP11_A {
        match self.bits {
            false => CLRP11_A::CLRP_0,
            true => CLRP11_A::CLRP_1,
        }
    }
    #[doc = "Checks if the value of the field is `CLRP_0`"]
    #[inline(always)]
    pub fn is_clrp_0(&self) -> bool {
        *self == CLRP11_A::CLRP_0
    }
    #[doc = "Checks if the value of the field is `CLRP_1`"]
    #[inline(always)]
    pub fn is_clrp_1(&self) -> bool {
        *self == CLRP11_A::CLRP_1
    }
}
#[doc = "Field `CLRP11` writer - Clear output bits"]
pub type CLRP11_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, CLR0_SPEC, CLRP11_A, O>;
impl<'a, const O: u8> CLRP11_W<'a, O> {
    #[doc = "No operation"]
    #[inline(always)]
    pub fn clrp_0(self) -> &'a mut W {
        self.variant(CLRP11_A::CLRP_0)
    }
    #[doc = "Clears output bit"]
    #[inline(always)]
    pub fn clrp_1(self) -> &'a mut W {
        self.variant(CLRP11_A::CLRP_1)
    }
}
#[doc = "Field `CLRP12` reader - Clear output bits"]
pub type CLRP12_R = crate::BitReader<CLRP12_A>;
#[doc = "Clear output bits\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CLRP12_A {
    #[doc = "0: No operation"]
    CLRP_0 = 0,
    #[doc = "1: Clears output bit"]
    CLRP_1 = 1,
}
impl From<CLRP12_A> for bool {
    #[inline(always)]
    fn from(variant: CLRP12_A) -> Self {
        variant as u8 != 0
    }
}
impl CLRP12_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CLRP12_A {
        match self.bits {
            false => CLRP12_A::CLRP_0,
            true => CLRP12_A::CLRP_1,
        }
    }
    #[doc = "Checks if the value of the field is `CLRP_0`"]
    #[inline(always)]
    pub fn is_clrp_0(&self) -> bool {
        *self == CLRP12_A::CLRP_0
    }
    #[doc = "Checks if the value of the field is `CLRP_1`"]
    #[inline(always)]
    pub fn is_clrp_1(&self) -> bool {
        *self == CLRP12_A::CLRP_1
    }
}
#[doc = "Field `CLRP12` writer - Clear output bits"]
pub type CLRP12_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, CLR0_SPEC, CLRP12_A, O>;
impl<'a, const O: u8> CLRP12_W<'a, O> {
    #[doc = "No operation"]
    #[inline(always)]
    pub fn clrp_0(self) -> &'a mut W {
        self.variant(CLRP12_A::CLRP_0)
    }
    #[doc = "Clears output bit"]
    #[inline(always)]
    pub fn clrp_1(self) -> &'a mut W {
        self.variant(CLRP12_A::CLRP_1)
    }
}
#[doc = "Field `CLRP13` reader - Clear output bits"]
pub type CLRP13_R = crate::BitReader<CLRP13_A>;
#[doc = "Clear output bits\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CLRP13_A {
    #[doc = "0: No operation"]
    CLRP_0 = 0,
    #[doc = "1: Clears output bit"]
    CLRP_1 = 1,
}
impl From<CLRP13_A> for bool {
    #[inline(always)]
    fn from(variant: CLRP13_A) -> Self {
        variant as u8 != 0
    }
}
impl CLRP13_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CLRP13_A {
        match self.bits {
            false => CLRP13_A::CLRP_0,
            true => CLRP13_A::CLRP_1,
        }
    }
    #[doc = "Checks if the value of the field is `CLRP_0`"]
    #[inline(always)]
    pub fn is_clrp_0(&self) -> bool {
        *self == CLRP13_A::CLRP_0
    }
    #[doc = "Checks if the value of the field is `CLRP_1`"]
    #[inline(always)]
    pub fn is_clrp_1(&self) -> bool {
        *self == CLRP13_A::CLRP_1
    }
}
#[doc = "Field `CLRP13` writer - Clear output bits"]
pub type CLRP13_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, CLR0_SPEC, CLRP13_A, O>;
impl<'a, const O: u8> CLRP13_W<'a, O> {
    #[doc = "No operation"]
    #[inline(always)]
    pub fn clrp_0(self) -> &'a mut W {
        self.variant(CLRP13_A::CLRP_0)
    }
    #[doc = "Clears output bit"]
    #[inline(always)]
    pub fn clrp_1(self) -> &'a mut W {
        self.variant(CLRP13_A::CLRP_1)
    }
}
#[doc = "Field `CLRP14` reader - Clear output bits"]
pub type CLRP14_R = crate::BitReader<CLRP14_A>;
#[doc = "Clear output bits\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CLRP14_A {
    #[doc = "0: No operation"]
    CLRP_0 = 0,
    #[doc = "1: Clears output bit"]
    CLRP_1 = 1,
}
impl From<CLRP14_A> for bool {
    #[inline(always)]
    fn from(variant: CLRP14_A) -> Self {
        variant as u8 != 0
    }
}
impl CLRP14_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CLRP14_A {
        match self.bits {
            false => CLRP14_A::CLRP_0,
            true => CLRP14_A::CLRP_1,
        }
    }
    #[doc = "Checks if the value of the field is `CLRP_0`"]
    #[inline(always)]
    pub fn is_clrp_0(&self) -> bool {
        *self == CLRP14_A::CLRP_0
    }
    #[doc = "Checks if the value of the field is `CLRP_1`"]
    #[inline(always)]
    pub fn is_clrp_1(&self) -> bool {
        *self == CLRP14_A::CLRP_1
    }
}
#[doc = "Field `CLRP14` writer - Clear output bits"]
pub type CLRP14_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, CLR0_SPEC, CLRP14_A, O>;
impl<'a, const O: u8> CLRP14_W<'a, O> {
    #[doc = "No operation"]
    #[inline(always)]
    pub fn clrp_0(self) -> &'a mut W {
        self.variant(CLRP14_A::CLRP_0)
    }
    #[doc = "Clears output bit"]
    #[inline(always)]
    pub fn clrp_1(self) -> &'a mut W {
        self.variant(CLRP14_A::CLRP_1)
    }
}
#[doc = "Field `CLRP15` reader - Clear output bits"]
pub type CLRP15_R = crate::BitReader<CLRP15_A>;
#[doc = "Clear output bits\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CLRP15_A {
    #[doc = "0: No operation"]
    CLRP_0 = 0,
    #[doc = "1: Clears output bit"]
    CLRP_1 = 1,
}
impl From<CLRP15_A> for bool {
    #[inline(always)]
    fn from(variant: CLRP15_A) -> Self {
        variant as u8 != 0
    }
}
impl CLRP15_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CLRP15_A {
        match self.bits {
            false => CLRP15_A::CLRP_0,
            true => CLRP15_A::CLRP_1,
        }
    }
    #[doc = "Checks if the value of the field is `CLRP_0`"]
    #[inline(always)]
    pub fn is_clrp_0(&self) -> bool {
        *self == CLRP15_A::CLRP_0
    }
    #[doc = "Checks if the value of the field is `CLRP_1`"]
    #[inline(always)]
    pub fn is_clrp_1(&self) -> bool {
        *self == CLRP15_A::CLRP_1
    }
}
#[doc = "Field `CLRP15` writer - Clear output bits"]
pub type CLRP15_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, CLR0_SPEC, CLRP15_A, O>;
impl<'a, const O: u8> CLRP15_W<'a, O> {
    #[doc = "No operation"]
    #[inline(always)]
    pub fn clrp_0(self) -> &'a mut W {
        self.variant(CLRP15_A::CLRP_0)
    }
    #[doc = "Clears output bit"]
    #[inline(always)]
    pub fn clrp_1(self) -> &'a mut W {
        self.variant(CLRP15_A::CLRP_1)
    }
}
#[doc = "Field `CLRP16` reader - Clear output bits"]
pub type CLRP16_R = crate::BitReader<CLRP16_A>;
#[doc = "Clear output bits\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CLRP16_A {
    #[doc = "0: No operation"]
    CLRP_0 = 0,
    #[doc = "1: Clears output bit"]
    CLRP_1 = 1,
}
impl From<CLRP16_A> for bool {
    #[inline(always)]
    fn from(variant: CLRP16_A) -> Self {
        variant as u8 != 0
    }
}
impl CLRP16_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CLRP16_A {
        match self.bits {
            false => CLRP16_A::CLRP_0,
            true => CLRP16_A::CLRP_1,
        }
    }
    #[doc = "Checks if the value of the field is `CLRP_0`"]
    #[inline(always)]
    pub fn is_clrp_0(&self) -> bool {
        *self == CLRP16_A::CLRP_0
    }
    #[doc = "Checks if the value of the field is `CLRP_1`"]
    #[inline(always)]
    pub fn is_clrp_1(&self) -> bool {
        *self == CLRP16_A::CLRP_1
    }
}
#[doc = "Field `CLRP16` writer - Clear output bits"]
pub type CLRP16_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, CLR0_SPEC, CLRP16_A, O>;
impl<'a, const O: u8> CLRP16_W<'a, O> {
    #[doc = "No operation"]
    #[inline(always)]
    pub fn clrp_0(self) -> &'a mut W {
        self.variant(CLRP16_A::CLRP_0)
    }
    #[doc = "Clears output bit"]
    #[inline(always)]
    pub fn clrp_1(self) -> &'a mut W {
        self.variant(CLRP16_A::CLRP_1)
    }
}
#[doc = "Field `CLRP17` reader - Clear output bits"]
pub type CLRP17_R = crate::BitReader<CLRP17_A>;
#[doc = "Clear output bits\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CLRP17_A {
    #[doc = "0: No operation"]
    CLRP_0 = 0,
    #[doc = "1: Clears output bit"]
    CLRP_1 = 1,
}
impl From<CLRP17_A> for bool {
    #[inline(always)]
    fn from(variant: CLRP17_A) -> Self {
        variant as u8 != 0
    }
}
impl CLRP17_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CLRP17_A {
        match self.bits {
            false => CLRP17_A::CLRP_0,
            true => CLRP17_A::CLRP_1,
        }
    }
    #[doc = "Checks if the value of the field is `CLRP_0`"]
    #[inline(always)]
    pub fn is_clrp_0(&self) -> bool {
        *self == CLRP17_A::CLRP_0
    }
    #[doc = "Checks if the value of the field is `CLRP_1`"]
    #[inline(always)]
    pub fn is_clrp_1(&self) -> bool {
        *self == CLRP17_A::CLRP_1
    }
}
#[doc = "Field `CLRP17` writer - Clear output bits"]
pub type CLRP17_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, CLR0_SPEC, CLRP17_A, O>;
impl<'a, const O: u8> CLRP17_W<'a, O> {
    #[doc = "No operation"]
    #[inline(always)]
    pub fn clrp_0(self) -> &'a mut W {
        self.variant(CLRP17_A::CLRP_0)
    }
    #[doc = "Clears output bit"]
    #[inline(always)]
    pub fn clrp_1(self) -> &'a mut W {
        self.variant(CLRP17_A::CLRP_1)
    }
}
#[doc = "Field `CLRP18` reader - Clear output bits"]
pub type CLRP18_R = crate::BitReader<CLRP18_A>;
#[doc = "Clear output bits\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CLRP18_A {
    #[doc = "0: No operation"]
    CLRP_0 = 0,
    #[doc = "1: Clears output bit"]
    CLRP_1 = 1,
}
impl From<CLRP18_A> for bool {
    #[inline(always)]
    fn from(variant: CLRP18_A) -> Self {
        variant as u8 != 0
    }
}
impl CLRP18_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CLRP18_A {
        match self.bits {
            false => CLRP18_A::CLRP_0,
            true => CLRP18_A::CLRP_1,
        }
    }
    #[doc = "Checks if the value of the field is `CLRP_0`"]
    #[inline(always)]
    pub fn is_clrp_0(&self) -> bool {
        *self == CLRP18_A::CLRP_0
    }
    #[doc = "Checks if the value of the field is `CLRP_1`"]
    #[inline(always)]
    pub fn is_clrp_1(&self) -> bool {
        *self == CLRP18_A::CLRP_1
    }
}
#[doc = "Field `CLRP18` writer - Clear output bits"]
pub type CLRP18_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, CLR0_SPEC, CLRP18_A, O>;
impl<'a, const O: u8> CLRP18_W<'a, O> {
    #[doc = "No operation"]
    #[inline(always)]
    pub fn clrp_0(self) -> &'a mut W {
        self.variant(CLRP18_A::CLRP_0)
    }
    #[doc = "Clears output bit"]
    #[inline(always)]
    pub fn clrp_1(self) -> &'a mut W {
        self.variant(CLRP18_A::CLRP_1)
    }
}
#[doc = "Field `CLRP19` reader - Clear output bits"]
pub type CLRP19_R = crate::BitReader<CLRP19_A>;
#[doc = "Clear output bits\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CLRP19_A {
    #[doc = "0: No operation"]
    CLRP_0 = 0,
    #[doc = "1: Clears output bit"]
    CLRP_1 = 1,
}
impl From<CLRP19_A> for bool {
    #[inline(always)]
    fn from(variant: CLRP19_A) -> Self {
        variant as u8 != 0
    }
}
impl CLRP19_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CLRP19_A {
        match self.bits {
            false => CLRP19_A::CLRP_0,
            true => CLRP19_A::CLRP_1,
        }
    }
    #[doc = "Checks if the value of the field is `CLRP_0`"]
    #[inline(always)]
    pub fn is_clrp_0(&self) -> bool {
        *self == CLRP19_A::CLRP_0
    }
    #[doc = "Checks if the value of the field is `CLRP_1`"]
    #[inline(always)]
    pub fn is_clrp_1(&self) -> bool {
        *self == CLRP19_A::CLRP_1
    }
}
#[doc = "Field `CLRP19` writer - Clear output bits"]
pub type CLRP19_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, CLR0_SPEC, CLRP19_A, O>;
impl<'a, const O: u8> CLRP19_W<'a, O> {
    #[doc = "No operation"]
    #[inline(always)]
    pub fn clrp_0(self) -> &'a mut W {
        self.variant(CLRP19_A::CLRP_0)
    }
    #[doc = "Clears output bit"]
    #[inline(always)]
    pub fn clrp_1(self) -> &'a mut W {
        self.variant(CLRP19_A::CLRP_1)
    }
}
#[doc = "Field `CLRP20` reader - Clear output bits"]
pub type CLRP20_R = crate::BitReader<CLRP20_A>;
#[doc = "Clear output bits\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CLRP20_A {
    #[doc = "0: No operation"]
    CLRP_0 = 0,
    #[doc = "1: Clears output bit"]
    CLRP_1 = 1,
}
impl From<CLRP20_A> for bool {
    #[inline(always)]
    fn from(variant: CLRP20_A) -> Self {
        variant as u8 != 0
    }
}
impl CLRP20_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CLRP20_A {
        match self.bits {
            false => CLRP20_A::CLRP_0,
            true => CLRP20_A::CLRP_1,
        }
    }
    #[doc = "Checks if the value of the field is `CLRP_0`"]
    #[inline(always)]
    pub fn is_clrp_0(&self) -> bool {
        *self == CLRP20_A::CLRP_0
    }
    #[doc = "Checks if the value of the field is `CLRP_1`"]
    #[inline(always)]
    pub fn is_clrp_1(&self) -> bool {
        *self == CLRP20_A::CLRP_1
    }
}
#[doc = "Field `CLRP20` writer - Clear output bits"]
pub type CLRP20_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, CLR0_SPEC, CLRP20_A, O>;
impl<'a, const O: u8> CLRP20_W<'a, O> {
    #[doc = "No operation"]
    #[inline(always)]
    pub fn clrp_0(self) -> &'a mut W {
        self.variant(CLRP20_A::CLRP_0)
    }
    #[doc = "Clears output bit"]
    #[inline(always)]
    pub fn clrp_1(self) -> &'a mut W {
        self.variant(CLRP20_A::CLRP_1)
    }
}
#[doc = "Field `CLRP21` reader - Clear output bits"]
pub type CLRP21_R = crate::BitReader<CLRP21_A>;
#[doc = "Clear output bits\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CLRP21_A {
    #[doc = "0: No operation"]
    CLRP_0 = 0,
    #[doc = "1: Clears output bit"]
    CLRP_1 = 1,
}
impl From<CLRP21_A> for bool {
    #[inline(always)]
    fn from(variant: CLRP21_A) -> Self {
        variant as u8 != 0
    }
}
impl CLRP21_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CLRP21_A {
        match self.bits {
            false => CLRP21_A::CLRP_0,
            true => CLRP21_A::CLRP_1,
        }
    }
    #[doc = "Checks if the value of the field is `CLRP_0`"]
    #[inline(always)]
    pub fn is_clrp_0(&self) -> bool {
        *self == CLRP21_A::CLRP_0
    }
    #[doc = "Checks if the value of the field is `CLRP_1`"]
    #[inline(always)]
    pub fn is_clrp_1(&self) -> bool {
        *self == CLRP21_A::CLRP_1
    }
}
#[doc = "Field `CLRP21` writer - Clear output bits"]
pub type CLRP21_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, CLR0_SPEC, CLRP21_A, O>;
impl<'a, const O: u8> CLRP21_W<'a, O> {
    #[doc = "No operation"]
    #[inline(always)]
    pub fn clrp_0(self) -> &'a mut W {
        self.variant(CLRP21_A::CLRP_0)
    }
    #[doc = "Clears output bit"]
    #[inline(always)]
    pub fn clrp_1(self) -> &'a mut W {
        self.variant(CLRP21_A::CLRP_1)
    }
}
#[doc = "Field `CLRP22` reader - Clear output bits"]
pub type CLRP22_R = crate::BitReader<CLRP22_A>;
#[doc = "Clear output bits\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CLRP22_A {
    #[doc = "0: No operation"]
    CLRP_0 = 0,
    #[doc = "1: Clears output bit"]
    CLRP_1 = 1,
}
impl From<CLRP22_A> for bool {
    #[inline(always)]
    fn from(variant: CLRP22_A) -> Self {
        variant as u8 != 0
    }
}
impl CLRP22_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CLRP22_A {
        match self.bits {
            false => CLRP22_A::CLRP_0,
            true => CLRP22_A::CLRP_1,
        }
    }
    #[doc = "Checks if the value of the field is `CLRP_0`"]
    #[inline(always)]
    pub fn is_clrp_0(&self) -> bool {
        *self == CLRP22_A::CLRP_0
    }
    #[doc = "Checks if the value of the field is `CLRP_1`"]
    #[inline(always)]
    pub fn is_clrp_1(&self) -> bool {
        *self == CLRP22_A::CLRP_1
    }
}
#[doc = "Field `CLRP22` writer - Clear output bits"]
pub type CLRP22_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, CLR0_SPEC, CLRP22_A, O>;
impl<'a, const O: u8> CLRP22_W<'a, O> {
    #[doc = "No operation"]
    #[inline(always)]
    pub fn clrp_0(self) -> &'a mut W {
        self.variant(CLRP22_A::CLRP_0)
    }
    #[doc = "Clears output bit"]
    #[inline(always)]
    pub fn clrp_1(self) -> &'a mut W {
        self.variant(CLRP22_A::CLRP_1)
    }
}
#[doc = "Field `CLRP23` reader - Clear output bits"]
pub type CLRP23_R = crate::BitReader<CLRP23_A>;
#[doc = "Clear output bits\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CLRP23_A {
    #[doc = "0: No operation"]
    CLRP_0 = 0,
    #[doc = "1: Clears output bit"]
    CLRP_1 = 1,
}
impl From<CLRP23_A> for bool {
    #[inline(always)]
    fn from(variant: CLRP23_A) -> Self {
        variant as u8 != 0
    }
}
impl CLRP23_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CLRP23_A {
        match self.bits {
            false => CLRP23_A::CLRP_0,
            true => CLRP23_A::CLRP_1,
        }
    }
    #[doc = "Checks if the value of the field is `CLRP_0`"]
    #[inline(always)]
    pub fn is_clrp_0(&self) -> bool {
        *self == CLRP23_A::CLRP_0
    }
    #[doc = "Checks if the value of the field is `CLRP_1`"]
    #[inline(always)]
    pub fn is_clrp_1(&self) -> bool {
        *self == CLRP23_A::CLRP_1
    }
}
#[doc = "Field `CLRP23` writer - Clear output bits"]
pub type CLRP23_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, CLR0_SPEC, CLRP23_A, O>;
impl<'a, const O: u8> CLRP23_W<'a, O> {
    #[doc = "No operation"]
    #[inline(always)]
    pub fn clrp_0(self) -> &'a mut W {
        self.variant(CLRP23_A::CLRP_0)
    }
    #[doc = "Clears output bit"]
    #[inline(always)]
    pub fn clrp_1(self) -> &'a mut W {
        self.variant(CLRP23_A::CLRP_1)
    }
}
#[doc = "Field `CLRP24` reader - Clear output bits"]
pub type CLRP24_R = crate::BitReader<CLRP24_A>;
#[doc = "Clear output bits\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CLRP24_A {
    #[doc = "0: No operation"]
    CLRP_0 = 0,
    #[doc = "1: Clears output bit"]
    CLRP_1 = 1,
}
impl From<CLRP24_A> for bool {
    #[inline(always)]
    fn from(variant: CLRP24_A) -> Self {
        variant as u8 != 0
    }
}
impl CLRP24_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CLRP24_A {
        match self.bits {
            false => CLRP24_A::CLRP_0,
            true => CLRP24_A::CLRP_1,
        }
    }
    #[doc = "Checks if the value of the field is `CLRP_0`"]
    #[inline(always)]
    pub fn is_clrp_0(&self) -> bool {
        *self == CLRP24_A::CLRP_0
    }
    #[doc = "Checks if the value of the field is `CLRP_1`"]
    #[inline(always)]
    pub fn is_clrp_1(&self) -> bool {
        *self == CLRP24_A::CLRP_1
    }
}
#[doc = "Field `CLRP24` writer - Clear output bits"]
pub type CLRP24_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, CLR0_SPEC, CLRP24_A, O>;
impl<'a, const O: u8> CLRP24_W<'a, O> {
    #[doc = "No operation"]
    #[inline(always)]
    pub fn clrp_0(self) -> &'a mut W {
        self.variant(CLRP24_A::CLRP_0)
    }
    #[doc = "Clears output bit"]
    #[inline(always)]
    pub fn clrp_1(self) -> &'a mut W {
        self.variant(CLRP24_A::CLRP_1)
    }
}
#[doc = "Field `CLRP25` reader - Clear output bits"]
pub type CLRP25_R = crate::BitReader<CLRP25_A>;
#[doc = "Clear output bits\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CLRP25_A {
    #[doc = "0: No operation"]
    CLRP_0 = 0,
    #[doc = "1: Clears output bit"]
    CLRP_1 = 1,
}
impl From<CLRP25_A> for bool {
    #[inline(always)]
    fn from(variant: CLRP25_A) -> Self {
        variant as u8 != 0
    }
}
impl CLRP25_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CLRP25_A {
        match self.bits {
            false => CLRP25_A::CLRP_0,
            true => CLRP25_A::CLRP_1,
        }
    }
    #[doc = "Checks if the value of the field is `CLRP_0`"]
    #[inline(always)]
    pub fn is_clrp_0(&self) -> bool {
        *self == CLRP25_A::CLRP_0
    }
    #[doc = "Checks if the value of the field is `CLRP_1`"]
    #[inline(always)]
    pub fn is_clrp_1(&self) -> bool {
        *self == CLRP25_A::CLRP_1
    }
}
#[doc = "Field `CLRP25` writer - Clear output bits"]
pub type CLRP25_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, CLR0_SPEC, CLRP25_A, O>;
impl<'a, const O: u8> CLRP25_W<'a, O> {
    #[doc = "No operation"]
    #[inline(always)]
    pub fn clrp_0(self) -> &'a mut W {
        self.variant(CLRP25_A::CLRP_0)
    }
    #[doc = "Clears output bit"]
    #[inline(always)]
    pub fn clrp_1(self) -> &'a mut W {
        self.variant(CLRP25_A::CLRP_1)
    }
}
#[doc = "Field `CLRP26` reader - Clear output bits"]
pub type CLRP26_R = crate::BitReader<CLRP26_A>;
#[doc = "Clear output bits\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CLRP26_A {
    #[doc = "0: No operation"]
    CLRP_0 = 0,
    #[doc = "1: Clears output bit"]
    CLRP_1 = 1,
}
impl From<CLRP26_A> for bool {
    #[inline(always)]
    fn from(variant: CLRP26_A) -> Self {
        variant as u8 != 0
    }
}
impl CLRP26_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CLRP26_A {
        match self.bits {
            false => CLRP26_A::CLRP_0,
            true => CLRP26_A::CLRP_1,
        }
    }
    #[doc = "Checks if the value of the field is `CLRP_0`"]
    #[inline(always)]
    pub fn is_clrp_0(&self) -> bool {
        *self == CLRP26_A::CLRP_0
    }
    #[doc = "Checks if the value of the field is `CLRP_1`"]
    #[inline(always)]
    pub fn is_clrp_1(&self) -> bool {
        *self == CLRP26_A::CLRP_1
    }
}
#[doc = "Field `CLRP26` writer - Clear output bits"]
pub type CLRP26_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, CLR0_SPEC, CLRP26_A, O>;
impl<'a, const O: u8> CLRP26_W<'a, O> {
    #[doc = "No operation"]
    #[inline(always)]
    pub fn clrp_0(self) -> &'a mut W {
        self.variant(CLRP26_A::CLRP_0)
    }
    #[doc = "Clears output bit"]
    #[inline(always)]
    pub fn clrp_1(self) -> &'a mut W {
        self.variant(CLRP26_A::CLRP_1)
    }
}
#[doc = "Field `CLRP27` reader - Clear output bits"]
pub type CLRP27_R = crate::BitReader<CLRP27_A>;
#[doc = "Clear output bits\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CLRP27_A {
    #[doc = "0: No operation"]
    CLRP_0 = 0,
    #[doc = "1: Clears output bit"]
    CLRP_1 = 1,
}
impl From<CLRP27_A> for bool {
    #[inline(always)]
    fn from(variant: CLRP27_A) -> Self {
        variant as u8 != 0
    }
}
impl CLRP27_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CLRP27_A {
        match self.bits {
            false => CLRP27_A::CLRP_0,
            true => CLRP27_A::CLRP_1,
        }
    }
    #[doc = "Checks if the value of the field is `CLRP_0`"]
    #[inline(always)]
    pub fn is_clrp_0(&self) -> bool {
        *self == CLRP27_A::CLRP_0
    }
    #[doc = "Checks if the value of the field is `CLRP_1`"]
    #[inline(always)]
    pub fn is_clrp_1(&self) -> bool {
        *self == CLRP27_A::CLRP_1
    }
}
#[doc = "Field `CLRP27` writer - Clear output bits"]
pub type CLRP27_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, CLR0_SPEC, CLRP27_A, O>;
impl<'a, const O: u8> CLRP27_W<'a, O> {
    #[doc = "No operation"]
    #[inline(always)]
    pub fn clrp_0(self) -> &'a mut W {
        self.variant(CLRP27_A::CLRP_0)
    }
    #[doc = "Clears output bit"]
    #[inline(always)]
    pub fn clrp_1(self) -> &'a mut W {
        self.variant(CLRP27_A::CLRP_1)
    }
}
#[doc = "Field `CLRP28` reader - Clear output bits"]
pub type CLRP28_R = crate::BitReader<CLRP28_A>;
#[doc = "Clear output bits\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CLRP28_A {
    #[doc = "0: No operation"]
    CLRP_0 = 0,
    #[doc = "1: Clears output bit"]
    CLRP_1 = 1,
}
impl From<CLRP28_A> for bool {
    #[inline(always)]
    fn from(variant: CLRP28_A) -> Self {
        variant as u8 != 0
    }
}
impl CLRP28_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CLRP28_A {
        match self.bits {
            false => CLRP28_A::CLRP_0,
            true => CLRP28_A::CLRP_1,
        }
    }
    #[doc = "Checks if the value of the field is `CLRP_0`"]
    #[inline(always)]
    pub fn is_clrp_0(&self) -> bool {
        *self == CLRP28_A::CLRP_0
    }
    #[doc = "Checks if the value of the field is `CLRP_1`"]
    #[inline(always)]
    pub fn is_clrp_1(&self) -> bool {
        *self == CLRP28_A::CLRP_1
    }
}
#[doc = "Field `CLRP28` writer - Clear output bits"]
pub type CLRP28_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, CLR0_SPEC, CLRP28_A, O>;
impl<'a, const O: u8> CLRP28_W<'a, O> {
    #[doc = "No operation"]
    #[inline(always)]
    pub fn clrp_0(self) -> &'a mut W {
        self.variant(CLRP28_A::CLRP_0)
    }
    #[doc = "Clears output bit"]
    #[inline(always)]
    pub fn clrp_1(self) -> &'a mut W {
        self.variant(CLRP28_A::CLRP_1)
    }
}
#[doc = "Field `CLRP29` reader - Clear output bits"]
pub type CLRP29_R = crate::BitReader<CLRP29_A>;
#[doc = "Clear output bits\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CLRP29_A {
    #[doc = "0: No operation"]
    CLRP_0 = 0,
    #[doc = "1: Clears output bit"]
    CLRP_1 = 1,
}
impl From<CLRP29_A> for bool {
    #[inline(always)]
    fn from(variant: CLRP29_A) -> Self {
        variant as u8 != 0
    }
}
impl CLRP29_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CLRP29_A {
        match self.bits {
            false => CLRP29_A::CLRP_0,
            true => CLRP29_A::CLRP_1,
        }
    }
    #[doc = "Checks if the value of the field is `CLRP_0`"]
    #[inline(always)]
    pub fn is_clrp_0(&self) -> bool {
        *self == CLRP29_A::CLRP_0
    }
    #[doc = "Checks if the value of the field is `CLRP_1`"]
    #[inline(always)]
    pub fn is_clrp_1(&self) -> bool {
        *self == CLRP29_A::CLRP_1
    }
}
#[doc = "Field `CLRP29` writer - Clear output bits"]
pub type CLRP29_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, CLR0_SPEC, CLRP29_A, O>;
impl<'a, const O: u8> CLRP29_W<'a, O> {
    #[doc = "No operation"]
    #[inline(always)]
    pub fn clrp_0(self) -> &'a mut W {
        self.variant(CLRP29_A::CLRP_0)
    }
    #[doc = "Clears output bit"]
    #[inline(always)]
    pub fn clrp_1(self) -> &'a mut W {
        self.variant(CLRP29_A::CLRP_1)
    }
}
#[doc = "Field `CLRP30` reader - Clear output bits"]
pub type CLRP30_R = crate::BitReader<CLRP30_A>;
#[doc = "Clear output bits\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CLRP30_A {
    #[doc = "0: No operation"]
    CLRP_0 = 0,
    #[doc = "1: Clears output bit"]
    CLRP_1 = 1,
}
impl From<CLRP30_A> for bool {
    #[inline(always)]
    fn from(variant: CLRP30_A) -> Self {
        variant as u8 != 0
    }
}
impl CLRP30_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CLRP30_A {
        match self.bits {
            false => CLRP30_A::CLRP_0,
            true => CLRP30_A::CLRP_1,
        }
    }
    #[doc = "Checks if the value of the field is `CLRP_0`"]
    #[inline(always)]
    pub fn is_clrp_0(&self) -> bool {
        *self == CLRP30_A::CLRP_0
    }
    #[doc = "Checks if the value of the field is `CLRP_1`"]
    #[inline(always)]
    pub fn is_clrp_1(&self) -> bool {
        *self == CLRP30_A::CLRP_1
    }
}
#[doc = "Field `CLRP30` writer - Clear output bits"]
pub type CLRP30_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, CLR0_SPEC, CLRP30_A, O>;
impl<'a, const O: u8> CLRP30_W<'a, O> {
    #[doc = "No operation"]
    #[inline(always)]
    pub fn clrp_0(self) -> &'a mut W {
        self.variant(CLRP30_A::CLRP_0)
    }
    #[doc = "Clears output bit"]
    #[inline(always)]
    pub fn clrp_1(self) -> &'a mut W {
        self.variant(CLRP30_A::CLRP_1)
    }
}
#[doc = "Field `CLRP31` reader - Clear output bits"]
pub type CLRP31_R = crate::BitReader<CLRP31_A>;
#[doc = "Clear output bits\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CLRP31_A {
    #[doc = "0: No operation"]
    CLRP_0 = 0,
    #[doc = "1: Clears output bit"]
    CLRP_1 = 1,
}
impl From<CLRP31_A> for bool {
    #[inline(always)]
    fn from(variant: CLRP31_A) -> Self {
        variant as u8 != 0
    }
}
impl CLRP31_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CLRP31_A {
        match self.bits {
            false => CLRP31_A::CLRP_0,
            true => CLRP31_A::CLRP_1,
        }
    }
    #[doc = "Checks if the value of the field is `CLRP_0`"]
    #[inline(always)]
    pub fn is_clrp_0(&self) -> bool {
        *self == CLRP31_A::CLRP_0
    }
    #[doc = "Checks if the value of the field is `CLRP_1`"]
    #[inline(always)]
    pub fn is_clrp_1(&self) -> bool {
        *self == CLRP31_A::CLRP_1
    }
}
#[doc = "Field `CLRP31` writer - Clear output bits"]
pub type CLRP31_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, CLR0_SPEC, CLRP31_A, O>;
impl<'a, const O: u8> CLRP31_W<'a, O> {
    #[doc = "No operation"]
    #[inline(always)]
    pub fn clrp_0(self) -> &'a mut W {
        self.variant(CLRP31_A::CLRP_0)
    }
    #[doc = "Clears output bit"]
    #[inline(always)]
    pub fn clrp_1(self) -> &'a mut W {
        self.variant(CLRP31_A::CLRP_1)
    }
}
impl R {
    #[doc = "Bit 0 - Clear output bits"]
    #[inline(always)]
    pub fn clrp0(&self) -> CLRP0_R {
        CLRP0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Clear output bits"]
    #[inline(always)]
    pub fn clrp1(&self) -> CLRP1_R {
        CLRP1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Clear output bits"]
    #[inline(always)]
    pub fn clrp2(&self) -> CLRP2_R {
        CLRP2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Clear output bits"]
    #[inline(always)]
    pub fn clrp3(&self) -> CLRP3_R {
        CLRP3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Clear output bits"]
    #[inline(always)]
    pub fn clrp4(&self) -> CLRP4_R {
        CLRP4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Clear output bits"]
    #[inline(always)]
    pub fn clrp5(&self) -> CLRP5_R {
        CLRP5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Clear output bits"]
    #[inline(always)]
    pub fn clrp6(&self) -> CLRP6_R {
        CLRP6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Clear output bits"]
    #[inline(always)]
    pub fn clrp7(&self) -> CLRP7_R {
        CLRP7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Clear output bits"]
    #[inline(always)]
    pub fn clrp8(&self) -> CLRP8_R {
        CLRP8_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Clear output bits"]
    #[inline(always)]
    pub fn clrp9(&self) -> CLRP9_R {
        CLRP9_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Clear output bits"]
    #[inline(always)]
    pub fn clrp10(&self) -> CLRP10_R {
        CLRP10_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Clear output bits"]
    #[inline(always)]
    pub fn clrp11(&self) -> CLRP11_R {
        CLRP11_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Clear output bits"]
    #[inline(always)]
    pub fn clrp12(&self) -> CLRP12_R {
        CLRP12_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Clear output bits"]
    #[inline(always)]
    pub fn clrp13(&self) -> CLRP13_R {
        CLRP13_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Clear output bits"]
    #[inline(always)]
    pub fn clrp14(&self) -> CLRP14_R {
        CLRP14_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Clear output bits"]
    #[inline(always)]
    pub fn clrp15(&self) -> CLRP15_R {
        CLRP15_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Clear output bits"]
    #[inline(always)]
    pub fn clrp16(&self) -> CLRP16_R {
        CLRP16_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Clear output bits"]
    #[inline(always)]
    pub fn clrp17(&self) -> CLRP17_R {
        CLRP17_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Clear output bits"]
    #[inline(always)]
    pub fn clrp18(&self) -> CLRP18_R {
        CLRP18_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Clear output bits"]
    #[inline(always)]
    pub fn clrp19(&self) -> CLRP19_R {
        CLRP19_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Clear output bits"]
    #[inline(always)]
    pub fn clrp20(&self) -> CLRP20_R {
        CLRP20_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Clear output bits"]
    #[inline(always)]
    pub fn clrp21(&self) -> CLRP21_R {
        CLRP21_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Clear output bits"]
    #[inline(always)]
    pub fn clrp22(&self) -> CLRP22_R {
        CLRP22_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Clear output bits"]
    #[inline(always)]
    pub fn clrp23(&self) -> CLRP23_R {
        CLRP23_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Clear output bits"]
    #[inline(always)]
    pub fn clrp24(&self) -> CLRP24_R {
        CLRP24_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Clear output bits"]
    #[inline(always)]
    pub fn clrp25(&self) -> CLRP25_R {
        CLRP25_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Clear output bits"]
    #[inline(always)]
    pub fn clrp26(&self) -> CLRP26_R {
        CLRP26_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Clear output bits"]
    #[inline(always)]
    pub fn clrp27(&self) -> CLRP27_R {
        CLRP27_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Clear output bits"]
    #[inline(always)]
    pub fn clrp28(&self) -> CLRP28_R {
        CLRP28_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Clear output bits"]
    #[inline(always)]
    pub fn clrp29(&self) -> CLRP29_R {
        CLRP29_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Clear output bits"]
    #[inline(always)]
    pub fn clrp30(&self) -> CLRP30_R {
        CLRP30_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Clear output bits"]
    #[inline(always)]
    pub fn clrp31(&self) -> CLRP31_R {
        CLRP31_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Clear output bits"]
    #[inline(always)]
    #[must_use]
    pub fn clrp0(&mut self) -> CLRP0_W<0> {
        CLRP0_W::new(self)
    }
    #[doc = "Bit 1 - Clear output bits"]
    #[inline(always)]
    #[must_use]
    pub fn clrp1(&mut self) -> CLRP1_W<1> {
        CLRP1_W::new(self)
    }
    #[doc = "Bit 2 - Clear output bits"]
    #[inline(always)]
    #[must_use]
    pub fn clrp2(&mut self) -> CLRP2_W<2> {
        CLRP2_W::new(self)
    }
    #[doc = "Bit 3 - Clear output bits"]
    #[inline(always)]
    #[must_use]
    pub fn clrp3(&mut self) -> CLRP3_W<3> {
        CLRP3_W::new(self)
    }
    #[doc = "Bit 4 - Clear output bits"]
    #[inline(always)]
    #[must_use]
    pub fn clrp4(&mut self) -> CLRP4_W<4> {
        CLRP4_W::new(self)
    }
    #[doc = "Bit 5 - Clear output bits"]
    #[inline(always)]
    #[must_use]
    pub fn clrp5(&mut self) -> CLRP5_W<5> {
        CLRP5_W::new(self)
    }
    #[doc = "Bit 6 - Clear output bits"]
    #[inline(always)]
    #[must_use]
    pub fn clrp6(&mut self) -> CLRP6_W<6> {
        CLRP6_W::new(self)
    }
    #[doc = "Bit 7 - Clear output bits"]
    #[inline(always)]
    #[must_use]
    pub fn clrp7(&mut self) -> CLRP7_W<7> {
        CLRP7_W::new(self)
    }
    #[doc = "Bit 8 - Clear output bits"]
    #[inline(always)]
    #[must_use]
    pub fn clrp8(&mut self) -> CLRP8_W<8> {
        CLRP8_W::new(self)
    }
    #[doc = "Bit 9 - Clear output bits"]
    #[inline(always)]
    #[must_use]
    pub fn clrp9(&mut self) -> CLRP9_W<9> {
        CLRP9_W::new(self)
    }
    #[doc = "Bit 10 - Clear output bits"]
    #[inline(always)]
    #[must_use]
    pub fn clrp10(&mut self) -> CLRP10_W<10> {
        CLRP10_W::new(self)
    }
    #[doc = "Bit 11 - Clear output bits"]
    #[inline(always)]
    #[must_use]
    pub fn clrp11(&mut self) -> CLRP11_W<11> {
        CLRP11_W::new(self)
    }
    #[doc = "Bit 12 - Clear output bits"]
    #[inline(always)]
    #[must_use]
    pub fn clrp12(&mut self) -> CLRP12_W<12> {
        CLRP12_W::new(self)
    }
    #[doc = "Bit 13 - Clear output bits"]
    #[inline(always)]
    #[must_use]
    pub fn clrp13(&mut self) -> CLRP13_W<13> {
        CLRP13_W::new(self)
    }
    #[doc = "Bit 14 - Clear output bits"]
    #[inline(always)]
    #[must_use]
    pub fn clrp14(&mut self) -> CLRP14_W<14> {
        CLRP14_W::new(self)
    }
    #[doc = "Bit 15 - Clear output bits"]
    #[inline(always)]
    #[must_use]
    pub fn clrp15(&mut self) -> CLRP15_W<15> {
        CLRP15_W::new(self)
    }
    #[doc = "Bit 16 - Clear output bits"]
    #[inline(always)]
    #[must_use]
    pub fn clrp16(&mut self) -> CLRP16_W<16> {
        CLRP16_W::new(self)
    }
    #[doc = "Bit 17 - Clear output bits"]
    #[inline(always)]
    #[must_use]
    pub fn clrp17(&mut self) -> CLRP17_W<17> {
        CLRP17_W::new(self)
    }
    #[doc = "Bit 18 - Clear output bits"]
    #[inline(always)]
    #[must_use]
    pub fn clrp18(&mut self) -> CLRP18_W<18> {
        CLRP18_W::new(self)
    }
    #[doc = "Bit 19 - Clear output bits"]
    #[inline(always)]
    #[must_use]
    pub fn clrp19(&mut self) -> CLRP19_W<19> {
        CLRP19_W::new(self)
    }
    #[doc = "Bit 20 - Clear output bits"]
    #[inline(always)]
    #[must_use]
    pub fn clrp20(&mut self) -> CLRP20_W<20> {
        CLRP20_W::new(self)
    }
    #[doc = "Bit 21 - Clear output bits"]
    #[inline(always)]
    #[must_use]
    pub fn clrp21(&mut self) -> CLRP21_W<21> {
        CLRP21_W::new(self)
    }
    #[doc = "Bit 22 - Clear output bits"]
    #[inline(always)]
    #[must_use]
    pub fn clrp22(&mut self) -> CLRP22_W<22> {
        CLRP22_W::new(self)
    }
    #[doc = "Bit 23 - Clear output bits"]
    #[inline(always)]
    #[must_use]
    pub fn clrp23(&mut self) -> CLRP23_W<23> {
        CLRP23_W::new(self)
    }
    #[doc = "Bit 24 - Clear output bits"]
    #[inline(always)]
    #[must_use]
    pub fn clrp24(&mut self) -> CLRP24_W<24> {
        CLRP24_W::new(self)
    }
    #[doc = "Bit 25 - Clear output bits"]
    #[inline(always)]
    #[must_use]
    pub fn clrp25(&mut self) -> CLRP25_W<25> {
        CLRP25_W::new(self)
    }
    #[doc = "Bit 26 - Clear output bits"]
    #[inline(always)]
    #[must_use]
    pub fn clrp26(&mut self) -> CLRP26_W<26> {
        CLRP26_W::new(self)
    }
    #[doc = "Bit 27 - Clear output bits"]
    #[inline(always)]
    #[must_use]
    pub fn clrp27(&mut self) -> CLRP27_W<27> {
        CLRP27_W::new(self)
    }
    #[doc = "Bit 28 - Clear output bits"]
    #[inline(always)]
    #[must_use]
    pub fn clrp28(&mut self) -> CLRP28_W<28> {
        CLRP28_W::new(self)
    }
    #[doc = "Bit 29 - Clear output bits"]
    #[inline(always)]
    #[must_use]
    pub fn clrp29(&mut self) -> CLRP29_W<29> {
        CLRP29_W::new(self)
    }
    #[doc = "Bit 30 - Clear output bits"]
    #[inline(always)]
    #[must_use]
    pub fn clrp30(&mut self) -> CLRP30_W<30> {
        CLRP30_W::new(self)
    }
    #[doc = "Bit 31 - Clear output bits"]
    #[inline(always)]
    #[must_use]
    pub fn clrp31(&mut self) -> CLRP31_W<31> {
        CLRP31_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Port clear\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clr0](index.html) module"]
pub struct CLR0_SPEC;
impl crate::RegisterSpec for CLR0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [clr0::R](R) reader structure"]
impl crate::Readable for CLR0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [clr0::W](W) writer structure"]
impl crate::Writable for CLR0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0xffff_ffff;
}
#[doc = "`reset()` method sets CLR0 to value 0"]
impl crate::Resettable for CLR0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
