#[doc = "Register `DIRCLR[%s]` reader"]
pub struct R(crate::R<DIRCLR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DIRCLR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DIRCLR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DIRCLR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DIRCLR[%s]` writer"]
pub struct W(crate::W<DIRCLR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DIRCLR_SPEC>;
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
impl From<crate::W<DIRCLR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DIRCLR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DIRCLRP0` reader - Clear direction bits."]
pub type DIRCLRP0_R = crate::BitReader<DIRCLRP0_A>;
#[doc = "Clear direction bits.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DIRCLRP0_A {
    #[doc = "0: No operation"]
    DIRCLR_0 = 0,
    #[doc = "1: Clears direction bits"]
    DIRCLR_1 = 1,
}
impl From<DIRCLRP0_A> for bool {
    #[inline(always)]
    fn from(variant: DIRCLRP0_A) -> Self {
        variant as u8 != 0
    }
}
impl DIRCLRP0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DIRCLRP0_A {
        match self.bits {
            false => DIRCLRP0_A::DIRCLR_0,
            true => DIRCLRP0_A::DIRCLR_1,
        }
    }
    #[doc = "Checks if the value of the field is `DIRCLR_0`"]
    #[inline(always)]
    pub fn is_dirclr_0(&self) -> bool {
        *self == DIRCLRP0_A::DIRCLR_0
    }
    #[doc = "Checks if the value of the field is `DIRCLR_1`"]
    #[inline(always)]
    pub fn is_dirclr_1(&self) -> bool {
        *self == DIRCLRP0_A::DIRCLR_1
    }
}
#[doc = "Field `DIRCLRP0` writer - Clear direction bits."]
pub type DIRCLRP0_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, DIRCLR_SPEC, DIRCLRP0_A, O>;
impl<'a, const O: u8> DIRCLRP0_W<'a, O> {
    #[doc = "No operation"]
    #[inline(always)]
    pub fn dirclr_0(self) -> &'a mut W {
        self.variant(DIRCLRP0_A::DIRCLR_0)
    }
    #[doc = "Clears direction bits"]
    #[inline(always)]
    pub fn dirclr_1(self) -> &'a mut W {
        self.variant(DIRCLRP0_A::DIRCLR_1)
    }
}
#[doc = "Field `DIRCLRP1` reader - Clear direction bits."]
pub type DIRCLRP1_R = crate::BitReader<DIRCLRP1_A>;
#[doc = "Clear direction bits.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DIRCLRP1_A {
    #[doc = "0: No operation"]
    DIRCLR_0 = 0,
    #[doc = "1: Clears direction bits"]
    DIRCLR_1 = 1,
}
impl From<DIRCLRP1_A> for bool {
    #[inline(always)]
    fn from(variant: DIRCLRP1_A) -> Self {
        variant as u8 != 0
    }
}
impl DIRCLRP1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DIRCLRP1_A {
        match self.bits {
            false => DIRCLRP1_A::DIRCLR_0,
            true => DIRCLRP1_A::DIRCLR_1,
        }
    }
    #[doc = "Checks if the value of the field is `DIRCLR_0`"]
    #[inline(always)]
    pub fn is_dirclr_0(&self) -> bool {
        *self == DIRCLRP1_A::DIRCLR_0
    }
    #[doc = "Checks if the value of the field is `DIRCLR_1`"]
    #[inline(always)]
    pub fn is_dirclr_1(&self) -> bool {
        *self == DIRCLRP1_A::DIRCLR_1
    }
}
#[doc = "Field `DIRCLRP1` writer - Clear direction bits."]
pub type DIRCLRP1_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, DIRCLR_SPEC, DIRCLRP1_A, O>;
impl<'a, const O: u8> DIRCLRP1_W<'a, O> {
    #[doc = "No operation"]
    #[inline(always)]
    pub fn dirclr_0(self) -> &'a mut W {
        self.variant(DIRCLRP1_A::DIRCLR_0)
    }
    #[doc = "Clears direction bits"]
    #[inline(always)]
    pub fn dirclr_1(self) -> &'a mut W {
        self.variant(DIRCLRP1_A::DIRCLR_1)
    }
}
#[doc = "Field `DIRCLRP2` reader - Clear direction bits."]
pub type DIRCLRP2_R = crate::BitReader<DIRCLRP2_A>;
#[doc = "Clear direction bits.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DIRCLRP2_A {
    #[doc = "0: No operation"]
    DIRCLR_0 = 0,
    #[doc = "1: Clears direction bits"]
    DIRCLR_1 = 1,
}
impl From<DIRCLRP2_A> for bool {
    #[inline(always)]
    fn from(variant: DIRCLRP2_A) -> Self {
        variant as u8 != 0
    }
}
impl DIRCLRP2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DIRCLRP2_A {
        match self.bits {
            false => DIRCLRP2_A::DIRCLR_0,
            true => DIRCLRP2_A::DIRCLR_1,
        }
    }
    #[doc = "Checks if the value of the field is `DIRCLR_0`"]
    #[inline(always)]
    pub fn is_dirclr_0(&self) -> bool {
        *self == DIRCLRP2_A::DIRCLR_0
    }
    #[doc = "Checks if the value of the field is `DIRCLR_1`"]
    #[inline(always)]
    pub fn is_dirclr_1(&self) -> bool {
        *self == DIRCLRP2_A::DIRCLR_1
    }
}
#[doc = "Field `DIRCLRP2` writer - Clear direction bits."]
pub type DIRCLRP2_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, DIRCLR_SPEC, DIRCLRP2_A, O>;
impl<'a, const O: u8> DIRCLRP2_W<'a, O> {
    #[doc = "No operation"]
    #[inline(always)]
    pub fn dirclr_0(self) -> &'a mut W {
        self.variant(DIRCLRP2_A::DIRCLR_0)
    }
    #[doc = "Clears direction bits"]
    #[inline(always)]
    pub fn dirclr_1(self) -> &'a mut W {
        self.variant(DIRCLRP2_A::DIRCLR_1)
    }
}
#[doc = "Field `DIRCLRP3` reader - Clear direction bits."]
pub type DIRCLRP3_R = crate::BitReader<DIRCLRP3_A>;
#[doc = "Clear direction bits.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DIRCLRP3_A {
    #[doc = "0: No operation"]
    DIRCLR_0 = 0,
    #[doc = "1: Clears direction bits"]
    DIRCLR_1 = 1,
}
impl From<DIRCLRP3_A> for bool {
    #[inline(always)]
    fn from(variant: DIRCLRP3_A) -> Self {
        variant as u8 != 0
    }
}
impl DIRCLRP3_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DIRCLRP3_A {
        match self.bits {
            false => DIRCLRP3_A::DIRCLR_0,
            true => DIRCLRP3_A::DIRCLR_1,
        }
    }
    #[doc = "Checks if the value of the field is `DIRCLR_0`"]
    #[inline(always)]
    pub fn is_dirclr_0(&self) -> bool {
        *self == DIRCLRP3_A::DIRCLR_0
    }
    #[doc = "Checks if the value of the field is `DIRCLR_1`"]
    #[inline(always)]
    pub fn is_dirclr_1(&self) -> bool {
        *self == DIRCLRP3_A::DIRCLR_1
    }
}
#[doc = "Field `DIRCLRP3` writer - Clear direction bits."]
pub type DIRCLRP3_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, DIRCLR_SPEC, DIRCLRP3_A, O>;
impl<'a, const O: u8> DIRCLRP3_W<'a, O> {
    #[doc = "No operation"]
    #[inline(always)]
    pub fn dirclr_0(self) -> &'a mut W {
        self.variant(DIRCLRP3_A::DIRCLR_0)
    }
    #[doc = "Clears direction bits"]
    #[inline(always)]
    pub fn dirclr_1(self) -> &'a mut W {
        self.variant(DIRCLRP3_A::DIRCLR_1)
    }
}
#[doc = "Field `DIRCLRP4` reader - Clear direction bits."]
pub type DIRCLRP4_R = crate::BitReader<DIRCLRP4_A>;
#[doc = "Clear direction bits.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DIRCLRP4_A {
    #[doc = "0: No operation"]
    DIRCLR_0 = 0,
    #[doc = "1: Clears direction bits"]
    DIRCLR_1 = 1,
}
impl From<DIRCLRP4_A> for bool {
    #[inline(always)]
    fn from(variant: DIRCLRP4_A) -> Self {
        variant as u8 != 0
    }
}
impl DIRCLRP4_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DIRCLRP4_A {
        match self.bits {
            false => DIRCLRP4_A::DIRCLR_0,
            true => DIRCLRP4_A::DIRCLR_1,
        }
    }
    #[doc = "Checks if the value of the field is `DIRCLR_0`"]
    #[inline(always)]
    pub fn is_dirclr_0(&self) -> bool {
        *self == DIRCLRP4_A::DIRCLR_0
    }
    #[doc = "Checks if the value of the field is `DIRCLR_1`"]
    #[inline(always)]
    pub fn is_dirclr_1(&self) -> bool {
        *self == DIRCLRP4_A::DIRCLR_1
    }
}
#[doc = "Field `DIRCLRP4` writer - Clear direction bits."]
pub type DIRCLRP4_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, DIRCLR_SPEC, DIRCLRP4_A, O>;
impl<'a, const O: u8> DIRCLRP4_W<'a, O> {
    #[doc = "No operation"]
    #[inline(always)]
    pub fn dirclr_0(self) -> &'a mut W {
        self.variant(DIRCLRP4_A::DIRCLR_0)
    }
    #[doc = "Clears direction bits"]
    #[inline(always)]
    pub fn dirclr_1(self) -> &'a mut W {
        self.variant(DIRCLRP4_A::DIRCLR_1)
    }
}
#[doc = "Field `DIRCLRP5` reader - Clear direction bits."]
pub type DIRCLRP5_R = crate::BitReader<DIRCLRP5_A>;
#[doc = "Clear direction bits.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DIRCLRP5_A {
    #[doc = "0: No operation"]
    DIRCLR_0 = 0,
    #[doc = "1: Clears direction bits"]
    DIRCLR_1 = 1,
}
impl From<DIRCLRP5_A> for bool {
    #[inline(always)]
    fn from(variant: DIRCLRP5_A) -> Self {
        variant as u8 != 0
    }
}
impl DIRCLRP5_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DIRCLRP5_A {
        match self.bits {
            false => DIRCLRP5_A::DIRCLR_0,
            true => DIRCLRP5_A::DIRCLR_1,
        }
    }
    #[doc = "Checks if the value of the field is `DIRCLR_0`"]
    #[inline(always)]
    pub fn is_dirclr_0(&self) -> bool {
        *self == DIRCLRP5_A::DIRCLR_0
    }
    #[doc = "Checks if the value of the field is `DIRCLR_1`"]
    #[inline(always)]
    pub fn is_dirclr_1(&self) -> bool {
        *self == DIRCLRP5_A::DIRCLR_1
    }
}
#[doc = "Field `DIRCLRP5` writer - Clear direction bits."]
pub type DIRCLRP5_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, DIRCLR_SPEC, DIRCLRP5_A, O>;
impl<'a, const O: u8> DIRCLRP5_W<'a, O> {
    #[doc = "No operation"]
    #[inline(always)]
    pub fn dirclr_0(self) -> &'a mut W {
        self.variant(DIRCLRP5_A::DIRCLR_0)
    }
    #[doc = "Clears direction bits"]
    #[inline(always)]
    pub fn dirclr_1(self) -> &'a mut W {
        self.variant(DIRCLRP5_A::DIRCLR_1)
    }
}
#[doc = "Field `DIRCLRP6` reader - Clear direction bits."]
pub type DIRCLRP6_R = crate::BitReader<DIRCLRP6_A>;
#[doc = "Clear direction bits.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DIRCLRP6_A {
    #[doc = "0: No operation"]
    DIRCLR_0 = 0,
    #[doc = "1: Clears direction bits"]
    DIRCLR_1 = 1,
}
impl From<DIRCLRP6_A> for bool {
    #[inline(always)]
    fn from(variant: DIRCLRP6_A) -> Self {
        variant as u8 != 0
    }
}
impl DIRCLRP6_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DIRCLRP6_A {
        match self.bits {
            false => DIRCLRP6_A::DIRCLR_0,
            true => DIRCLRP6_A::DIRCLR_1,
        }
    }
    #[doc = "Checks if the value of the field is `DIRCLR_0`"]
    #[inline(always)]
    pub fn is_dirclr_0(&self) -> bool {
        *self == DIRCLRP6_A::DIRCLR_0
    }
    #[doc = "Checks if the value of the field is `DIRCLR_1`"]
    #[inline(always)]
    pub fn is_dirclr_1(&self) -> bool {
        *self == DIRCLRP6_A::DIRCLR_1
    }
}
#[doc = "Field `DIRCLRP6` writer - Clear direction bits."]
pub type DIRCLRP6_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, DIRCLR_SPEC, DIRCLRP6_A, O>;
impl<'a, const O: u8> DIRCLRP6_W<'a, O> {
    #[doc = "No operation"]
    #[inline(always)]
    pub fn dirclr_0(self) -> &'a mut W {
        self.variant(DIRCLRP6_A::DIRCLR_0)
    }
    #[doc = "Clears direction bits"]
    #[inline(always)]
    pub fn dirclr_1(self) -> &'a mut W {
        self.variant(DIRCLRP6_A::DIRCLR_1)
    }
}
#[doc = "Field `DIRCLRP7` reader - Clear direction bits."]
pub type DIRCLRP7_R = crate::BitReader<DIRCLRP7_A>;
#[doc = "Clear direction bits.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DIRCLRP7_A {
    #[doc = "0: No operation"]
    DIRCLR_0 = 0,
    #[doc = "1: Clears direction bits"]
    DIRCLR_1 = 1,
}
impl From<DIRCLRP7_A> for bool {
    #[inline(always)]
    fn from(variant: DIRCLRP7_A) -> Self {
        variant as u8 != 0
    }
}
impl DIRCLRP7_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DIRCLRP7_A {
        match self.bits {
            false => DIRCLRP7_A::DIRCLR_0,
            true => DIRCLRP7_A::DIRCLR_1,
        }
    }
    #[doc = "Checks if the value of the field is `DIRCLR_0`"]
    #[inline(always)]
    pub fn is_dirclr_0(&self) -> bool {
        *self == DIRCLRP7_A::DIRCLR_0
    }
    #[doc = "Checks if the value of the field is `DIRCLR_1`"]
    #[inline(always)]
    pub fn is_dirclr_1(&self) -> bool {
        *self == DIRCLRP7_A::DIRCLR_1
    }
}
#[doc = "Field `DIRCLRP7` writer - Clear direction bits."]
pub type DIRCLRP7_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, DIRCLR_SPEC, DIRCLRP7_A, O>;
impl<'a, const O: u8> DIRCLRP7_W<'a, O> {
    #[doc = "No operation"]
    #[inline(always)]
    pub fn dirclr_0(self) -> &'a mut W {
        self.variant(DIRCLRP7_A::DIRCLR_0)
    }
    #[doc = "Clears direction bits"]
    #[inline(always)]
    pub fn dirclr_1(self) -> &'a mut W {
        self.variant(DIRCLRP7_A::DIRCLR_1)
    }
}
#[doc = "Field `DIRCLRP8` reader - Clear direction bits."]
pub type DIRCLRP8_R = crate::BitReader<DIRCLRP8_A>;
#[doc = "Clear direction bits.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DIRCLRP8_A {
    #[doc = "0: No operation"]
    DIRCLR_0 = 0,
    #[doc = "1: Clears direction bits"]
    DIRCLR_1 = 1,
}
impl From<DIRCLRP8_A> for bool {
    #[inline(always)]
    fn from(variant: DIRCLRP8_A) -> Self {
        variant as u8 != 0
    }
}
impl DIRCLRP8_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DIRCLRP8_A {
        match self.bits {
            false => DIRCLRP8_A::DIRCLR_0,
            true => DIRCLRP8_A::DIRCLR_1,
        }
    }
    #[doc = "Checks if the value of the field is `DIRCLR_0`"]
    #[inline(always)]
    pub fn is_dirclr_0(&self) -> bool {
        *self == DIRCLRP8_A::DIRCLR_0
    }
    #[doc = "Checks if the value of the field is `DIRCLR_1`"]
    #[inline(always)]
    pub fn is_dirclr_1(&self) -> bool {
        *self == DIRCLRP8_A::DIRCLR_1
    }
}
#[doc = "Field `DIRCLRP8` writer - Clear direction bits."]
pub type DIRCLRP8_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, DIRCLR_SPEC, DIRCLRP8_A, O>;
impl<'a, const O: u8> DIRCLRP8_W<'a, O> {
    #[doc = "No operation"]
    #[inline(always)]
    pub fn dirclr_0(self) -> &'a mut W {
        self.variant(DIRCLRP8_A::DIRCLR_0)
    }
    #[doc = "Clears direction bits"]
    #[inline(always)]
    pub fn dirclr_1(self) -> &'a mut W {
        self.variant(DIRCLRP8_A::DIRCLR_1)
    }
}
#[doc = "Field `DIRCLRP9` reader - Clear direction bits."]
pub type DIRCLRP9_R = crate::BitReader<DIRCLRP9_A>;
#[doc = "Clear direction bits.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DIRCLRP9_A {
    #[doc = "0: No operation"]
    DIRCLR_0 = 0,
    #[doc = "1: Clears direction bits"]
    DIRCLR_1 = 1,
}
impl From<DIRCLRP9_A> for bool {
    #[inline(always)]
    fn from(variant: DIRCLRP9_A) -> Self {
        variant as u8 != 0
    }
}
impl DIRCLRP9_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DIRCLRP9_A {
        match self.bits {
            false => DIRCLRP9_A::DIRCLR_0,
            true => DIRCLRP9_A::DIRCLR_1,
        }
    }
    #[doc = "Checks if the value of the field is `DIRCLR_0`"]
    #[inline(always)]
    pub fn is_dirclr_0(&self) -> bool {
        *self == DIRCLRP9_A::DIRCLR_0
    }
    #[doc = "Checks if the value of the field is `DIRCLR_1`"]
    #[inline(always)]
    pub fn is_dirclr_1(&self) -> bool {
        *self == DIRCLRP9_A::DIRCLR_1
    }
}
#[doc = "Field `DIRCLRP9` writer - Clear direction bits."]
pub type DIRCLRP9_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, DIRCLR_SPEC, DIRCLRP9_A, O>;
impl<'a, const O: u8> DIRCLRP9_W<'a, O> {
    #[doc = "No operation"]
    #[inline(always)]
    pub fn dirclr_0(self) -> &'a mut W {
        self.variant(DIRCLRP9_A::DIRCLR_0)
    }
    #[doc = "Clears direction bits"]
    #[inline(always)]
    pub fn dirclr_1(self) -> &'a mut W {
        self.variant(DIRCLRP9_A::DIRCLR_1)
    }
}
#[doc = "Field `DIRCLRP10` reader - Clear direction bits."]
pub type DIRCLRP10_R = crate::BitReader<DIRCLRP10_A>;
#[doc = "Clear direction bits.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DIRCLRP10_A {
    #[doc = "0: No operation"]
    DIRCLR_0 = 0,
    #[doc = "1: Clears direction bits"]
    DIRCLR_1 = 1,
}
impl From<DIRCLRP10_A> for bool {
    #[inline(always)]
    fn from(variant: DIRCLRP10_A) -> Self {
        variant as u8 != 0
    }
}
impl DIRCLRP10_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DIRCLRP10_A {
        match self.bits {
            false => DIRCLRP10_A::DIRCLR_0,
            true => DIRCLRP10_A::DIRCLR_1,
        }
    }
    #[doc = "Checks if the value of the field is `DIRCLR_0`"]
    #[inline(always)]
    pub fn is_dirclr_0(&self) -> bool {
        *self == DIRCLRP10_A::DIRCLR_0
    }
    #[doc = "Checks if the value of the field is `DIRCLR_1`"]
    #[inline(always)]
    pub fn is_dirclr_1(&self) -> bool {
        *self == DIRCLRP10_A::DIRCLR_1
    }
}
#[doc = "Field `DIRCLRP10` writer - Clear direction bits."]
pub type DIRCLRP10_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, DIRCLR_SPEC, DIRCLRP10_A, O>;
impl<'a, const O: u8> DIRCLRP10_W<'a, O> {
    #[doc = "No operation"]
    #[inline(always)]
    pub fn dirclr_0(self) -> &'a mut W {
        self.variant(DIRCLRP10_A::DIRCLR_0)
    }
    #[doc = "Clears direction bits"]
    #[inline(always)]
    pub fn dirclr_1(self) -> &'a mut W {
        self.variant(DIRCLRP10_A::DIRCLR_1)
    }
}
#[doc = "Field `DIRCLRP11` reader - Clear direction bits."]
pub type DIRCLRP11_R = crate::BitReader<DIRCLRP11_A>;
#[doc = "Clear direction bits.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DIRCLRP11_A {
    #[doc = "0: No operation"]
    DIRCLR_0 = 0,
    #[doc = "1: Clears direction bits"]
    DIRCLR_1 = 1,
}
impl From<DIRCLRP11_A> for bool {
    #[inline(always)]
    fn from(variant: DIRCLRP11_A) -> Self {
        variant as u8 != 0
    }
}
impl DIRCLRP11_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DIRCLRP11_A {
        match self.bits {
            false => DIRCLRP11_A::DIRCLR_0,
            true => DIRCLRP11_A::DIRCLR_1,
        }
    }
    #[doc = "Checks if the value of the field is `DIRCLR_0`"]
    #[inline(always)]
    pub fn is_dirclr_0(&self) -> bool {
        *self == DIRCLRP11_A::DIRCLR_0
    }
    #[doc = "Checks if the value of the field is `DIRCLR_1`"]
    #[inline(always)]
    pub fn is_dirclr_1(&self) -> bool {
        *self == DIRCLRP11_A::DIRCLR_1
    }
}
#[doc = "Field `DIRCLRP11` writer - Clear direction bits."]
pub type DIRCLRP11_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, DIRCLR_SPEC, DIRCLRP11_A, O>;
impl<'a, const O: u8> DIRCLRP11_W<'a, O> {
    #[doc = "No operation"]
    #[inline(always)]
    pub fn dirclr_0(self) -> &'a mut W {
        self.variant(DIRCLRP11_A::DIRCLR_0)
    }
    #[doc = "Clears direction bits"]
    #[inline(always)]
    pub fn dirclr_1(self) -> &'a mut W {
        self.variant(DIRCLRP11_A::DIRCLR_1)
    }
}
#[doc = "Field `DIRCLRP12` reader - Clear direction bits."]
pub type DIRCLRP12_R = crate::BitReader<DIRCLRP12_A>;
#[doc = "Clear direction bits.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DIRCLRP12_A {
    #[doc = "0: No operation"]
    DIRCLR_0 = 0,
    #[doc = "1: Clears direction bits"]
    DIRCLR_1 = 1,
}
impl From<DIRCLRP12_A> for bool {
    #[inline(always)]
    fn from(variant: DIRCLRP12_A) -> Self {
        variant as u8 != 0
    }
}
impl DIRCLRP12_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DIRCLRP12_A {
        match self.bits {
            false => DIRCLRP12_A::DIRCLR_0,
            true => DIRCLRP12_A::DIRCLR_1,
        }
    }
    #[doc = "Checks if the value of the field is `DIRCLR_0`"]
    #[inline(always)]
    pub fn is_dirclr_0(&self) -> bool {
        *self == DIRCLRP12_A::DIRCLR_0
    }
    #[doc = "Checks if the value of the field is `DIRCLR_1`"]
    #[inline(always)]
    pub fn is_dirclr_1(&self) -> bool {
        *self == DIRCLRP12_A::DIRCLR_1
    }
}
#[doc = "Field `DIRCLRP12` writer - Clear direction bits."]
pub type DIRCLRP12_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, DIRCLR_SPEC, DIRCLRP12_A, O>;
impl<'a, const O: u8> DIRCLRP12_W<'a, O> {
    #[doc = "No operation"]
    #[inline(always)]
    pub fn dirclr_0(self) -> &'a mut W {
        self.variant(DIRCLRP12_A::DIRCLR_0)
    }
    #[doc = "Clears direction bits"]
    #[inline(always)]
    pub fn dirclr_1(self) -> &'a mut W {
        self.variant(DIRCLRP12_A::DIRCLR_1)
    }
}
#[doc = "Field `DIRCLRP13` reader - Clear direction bits."]
pub type DIRCLRP13_R = crate::BitReader<DIRCLRP13_A>;
#[doc = "Clear direction bits.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DIRCLRP13_A {
    #[doc = "0: No operation"]
    DIRCLR_0 = 0,
    #[doc = "1: Clears direction bits"]
    DIRCLR_1 = 1,
}
impl From<DIRCLRP13_A> for bool {
    #[inline(always)]
    fn from(variant: DIRCLRP13_A) -> Self {
        variant as u8 != 0
    }
}
impl DIRCLRP13_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DIRCLRP13_A {
        match self.bits {
            false => DIRCLRP13_A::DIRCLR_0,
            true => DIRCLRP13_A::DIRCLR_1,
        }
    }
    #[doc = "Checks if the value of the field is `DIRCLR_0`"]
    #[inline(always)]
    pub fn is_dirclr_0(&self) -> bool {
        *self == DIRCLRP13_A::DIRCLR_0
    }
    #[doc = "Checks if the value of the field is `DIRCLR_1`"]
    #[inline(always)]
    pub fn is_dirclr_1(&self) -> bool {
        *self == DIRCLRP13_A::DIRCLR_1
    }
}
#[doc = "Field `DIRCLRP13` writer - Clear direction bits."]
pub type DIRCLRP13_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, DIRCLR_SPEC, DIRCLRP13_A, O>;
impl<'a, const O: u8> DIRCLRP13_W<'a, O> {
    #[doc = "No operation"]
    #[inline(always)]
    pub fn dirclr_0(self) -> &'a mut W {
        self.variant(DIRCLRP13_A::DIRCLR_0)
    }
    #[doc = "Clears direction bits"]
    #[inline(always)]
    pub fn dirclr_1(self) -> &'a mut W {
        self.variant(DIRCLRP13_A::DIRCLR_1)
    }
}
#[doc = "Field `DIRCLRP14` reader - Clear direction bits."]
pub type DIRCLRP14_R = crate::BitReader<DIRCLRP14_A>;
#[doc = "Clear direction bits.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DIRCLRP14_A {
    #[doc = "0: No operation"]
    DIRCLR_0 = 0,
    #[doc = "1: Clears direction bits"]
    DIRCLR_1 = 1,
}
impl From<DIRCLRP14_A> for bool {
    #[inline(always)]
    fn from(variant: DIRCLRP14_A) -> Self {
        variant as u8 != 0
    }
}
impl DIRCLRP14_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DIRCLRP14_A {
        match self.bits {
            false => DIRCLRP14_A::DIRCLR_0,
            true => DIRCLRP14_A::DIRCLR_1,
        }
    }
    #[doc = "Checks if the value of the field is `DIRCLR_0`"]
    #[inline(always)]
    pub fn is_dirclr_0(&self) -> bool {
        *self == DIRCLRP14_A::DIRCLR_0
    }
    #[doc = "Checks if the value of the field is `DIRCLR_1`"]
    #[inline(always)]
    pub fn is_dirclr_1(&self) -> bool {
        *self == DIRCLRP14_A::DIRCLR_1
    }
}
#[doc = "Field `DIRCLRP14` writer - Clear direction bits."]
pub type DIRCLRP14_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, DIRCLR_SPEC, DIRCLRP14_A, O>;
impl<'a, const O: u8> DIRCLRP14_W<'a, O> {
    #[doc = "No operation"]
    #[inline(always)]
    pub fn dirclr_0(self) -> &'a mut W {
        self.variant(DIRCLRP14_A::DIRCLR_0)
    }
    #[doc = "Clears direction bits"]
    #[inline(always)]
    pub fn dirclr_1(self) -> &'a mut W {
        self.variant(DIRCLRP14_A::DIRCLR_1)
    }
}
#[doc = "Field `DIRCLRP15` reader - Clear direction bits."]
pub type DIRCLRP15_R = crate::BitReader<DIRCLRP15_A>;
#[doc = "Clear direction bits.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DIRCLRP15_A {
    #[doc = "0: No operation"]
    DIRCLR_0 = 0,
    #[doc = "1: Clears direction bits"]
    DIRCLR_1 = 1,
}
impl From<DIRCLRP15_A> for bool {
    #[inline(always)]
    fn from(variant: DIRCLRP15_A) -> Self {
        variant as u8 != 0
    }
}
impl DIRCLRP15_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DIRCLRP15_A {
        match self.bits {
            false => DIRCLRP15_A::DIRCLR_0,
            true => DIRCLRP15_A::DIRCLR_1,
        }
    }
    #[doc = "Checks if the value of the field is `DIRCLR_0`"]
    #[inline(always)]
    pub fn is_dirclr_0(&self) -> bool {
        *self == DIRCLRP15_A::DIRCLR_0
    }
    #[doc = "Checks if the value of the field is `DIRCLR_1`"]
    #[inline(always)]
    pub fn is_dirclr_1(&self) -> bool {
        *self == DIRCLRP15_A::DIRCLR_1
    }
}
#[doc = "Field `DIRCLRP15` writer - Clear direction bits."]
pub type DIRCLRP15_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, DIRCLR_SPEC, DIRCLRP15_A, O>;
impl<'a, const O: u8> DIRCLRP15_W<'a, O> {
    #[doc = "No operation"]
    #[inline(always)]
    pub fn dirclr_0(self) -> &'a mut W {
        self.variant(DIRCLRP15_A::DIRCLR_0)
    }
    #[doc = "Clears direction bits"]
    #[inline(always)]
    pub fn dirclr_1(self) -> &'a mut W {
        self.variant(DIRCLRP15_A::DIRCLR_1)
    }
}
#[doc = "Field `DIRCLRP16` reader - Clear direction bits."]
pub type DIRCLRP16_R = crate::BitReader<DIRCLRP16_A>;
#[doc = "Clear direction bits.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DIRCLRP16_A {
    #[doc = "0: No operation"]
    DIRCLR_0 = 0,
    #[doc = "1: Clears direction bits"]
    DIRCLR_1 = 1,
}
impl From<DIRCLRP16_A> for bool {
    #[inline(always)]
    fn from(variant: DIRCLRP16_A) -> Self {
        variant as u8 != 0
    }
}
impl DIRCLRP16_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DIRCLRP16_A {
        match self.bits {
            false => DIRCLRP16_A::DIRCLR_0,
            true => DIRCLRP16_A::DIRCLR_1,
        }
    }
    #[doc = "Checks if the value of the field is `DIRCLR_0`"]
    #[inline(always)]
    pub fn is_dirclr_0(&self) -> bool {
        *self == DIRCLRP16_A::DIRCLR_0
    }
    #[doc = "Checks if the value of the field is `DIRCLR_1`"]
    #[inline(always)]
    pub fn is_dirclr_1(&self) -> bool {
        *self == DIRCLRP16_A::DIRCLR_1
    }
}
#[doc = "Field `DIRCLRP16` writer - Clear direction bits."]
pub type DIRCLRP16_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, DIRCLR_SPEC, DIRCLRP16_A, O>;
impl<'a, const O: u8> DIRCLRP16_W<'a, O> {
    #[doc = "No operation"]
    #[inline(always)]
    pub fn dirclr_0(self) -> &'a mut W {
        self.variant(DIRCLRP16_A::DIRCLR_0)
    }
    #[doc = "Clears direction bits"]
    #[inline(always)]
    pub fn dirclr_1(self) -> &'a mut W {
        self.variant(DIRCLRP16_A::DIRCLR_1)
    }
}
#[doc = "Field `DIRCLRP17` reader - Clear direction bits."]
pub type DIRCLRP17_R = crate::BitReader<DIRCLRP17_A>;
#[doc = "Clear direction bits.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DIRCLRP17_A {
    #[doc = "0: No operation"]
    DIRCLR_0 = 0,
    #[doc = "1: Clears direction bits"]
    DIRCLR_1 = 1,
}
impl From<DIRCLRP17_A> for bool {
    #[inline(always)]
    fn from(variant: DIRCLRP17_A) -> Self {
        variant as u8 != 0
    }
}
impl DIRCLRP17_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DIRCLRP17_A {
        match self.bits {
            false => DIRCLRP17_A::DIRCLR_0,
            true => DIRCLRP17_A::DIRCLR_1,
        }
    }
    #[doc = "Checks if the value of the field is `DIRCLR_0`"]
    #[inline(always)]
    pub fn is_dirclr_0(&self) -> bool {
        *self == DIRCLRP17_A::DIRCLR_0
    }
    #[doc = "Checks if the value of the field is `DIRCLR_1`"]
    #[inline(always)]
    pub fn is_dirclr_1(&self) -> bool {
        *self == DIRCLRP17_A::DIRCLR_1
    }
}
#[doc = "Field `DIRCLRP17` writer - Clear direction bits."]
pub type DIRCLRP17_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, DIRCLR_SPEC, DIRCLRP17_A, O>;
impl<'a, const O: u8> DIRCLRP17_W<'a, O> {
    #[doc = "No operation"]
    #[inline(always)]
    pub fn dirclr_0(self) -> &'a mut W {
        self.variant(DIRCLRP17_A::DIRCLR_0)
    }
    #[doc = "Clears direction bits"]
    #[inline(always)]
    pub fn dirclr_1(self) -> &'a mut W {
        self.variant(DIRCLRP17_A::DIRCLR_1)
    }
}
#[doc = "Field `DIRCLRP18` reader - Clear direction bits."]
pub type DIRCLRP18_R = crate::BitReader<DIRCLRP18_A>;
#[doc = "Clear direction bits.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DIRCLRP18_A {
    #[doc = "0: No operation"]
    DIRCLR_0 = 0,
    #[doc = "1: Clears direction bits"]
    DIRCLR_1 = 1,
}
impl From<DIRCLRP18_A> for bool {
    #[inline(always)]
    fn from(variant: DIRCLRP18_A) -> Self {
        variant as u8 != 0
    }
}
impl DIRCLRP18_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DIRCLRP18_A {
        match self.bits {
            false => DIRCLRP18_A::DIRCLR_0,
            true => DIRCLRP18_A::DIRCLR_1,
        }
    }
    #[doc = "Checks if the value of the field is `DIRCLR_0`"]
    #[inline(always)]
    pub fn is_dirclr_0(&self) -> bool {
        *self == DIRCLRP18_A::DIRCLR_0
    }
    #[doc = "Checks if the value of the field is `DIRCLR_1`"]
    #[inline(always)]
    pub fn is_dirclr_1(&self) -> bool {
        *self == DIRCLRP18_A::DIRCLR_1
    }
}
#[doc = "Field `DIRCLRP18` writer - Clear direction bits."]
pub type DIRCLRP18_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, DIRCLR_SPEC, DIRCLRP18_A, O>;
impl<'a, const O: u8> DIRCLRP18_W<'a, O> {
    #[doc = "No operation"]
    #[inline(always)]
    pub fn dirclr_0(self) -> &'a mut W {
        self.variant(DIRCLRP18_A::DIRCLR_0)
    }
    #[doc = "Clears direction bits"]
    #[inline(always)]
    pub fn dirclr_1(self) -> &'a mut W {
        self.variant(DIRCLRP18_A::DIRCLR_1)
    }
}
#[doc = "Field `DIRCLRP19` reader - Clear direction bits."]
pub type DIRCLRP19_R = crate::BitReader<DIRCLRP19_A>;
#[doc = "Clear direction bits.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DIRCLRP19_A {
    #[doc = "0: No operation"]
    DIRCLR_0 = 0,
    #[doc = "1: Clears direction bits"]
    DIRCLR_1 = 1,
}
impl From<DIRCLRP19_A> for bool {
    #[inline(always)]
    fn from(variant: DIRCLRP19_A) -> Self {
        variant as u8 != 0
    }
}
impl DIRCLRP19_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DIRCLRP19_A {
        match self.bits {
            false => DIRCLRP19_A::DIRCLR_0,
            true => DIRCLRP19_A::DIRCLR_1,
        }
    }
    #[doc = "Checks if the value of the field is `DIRCLR_0`"]
    #[inline(always)]
    pub fn is_dirclr_0(&self) -> bool {
        *self == DIRCLRP19_A::DIRCLR_0
    }
    #[doc = "Checks if the value of the field is `DIRCLR_1`"]
    #[inline(always)]
    pub fn is_dirclr_1(&self) -> bool {
        *self == DIRCLRP19_A::DIRCLR_1
    }
}
#[doc = "Field `DIRCLRP19` writer - Clear direction bits."]
pub type DIRCLRP19_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, DIRCLR_SPEC, DIRCLRP19_A, O>;
impl<'a, const O: u8> DIRCLRP19_W<'a, O> {
    #[doc = "No operation"]
    #[inline(always)]
    pub fn dirclr_0(self) -> &'a mut W {
        self.variant(DIRCLRP19_A::DIRCLR_0)
    }
    #[doc = "Clears direction bits"]
    #[inline(always)]
    pub fn dirclr_1(self) -> &'a mut W {
        self.variant(DIRCLRP19_A::DIRCLR_1)
    }
}
#[doc = "Field `DIRCLRP20` reader - Clear direction bits."]
pub type DIRCLRP20_R = crate::BitReader<DIRCLRP20_A>;
#[doc = "Clear direction bits.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DIRCLRP20_A {
    #[doc = "0: No operation"]
    DIRCLR_0 = 0,
    #[doc = "1: Clears direction bits"]
    DIRCLR_1 = 1,
}
impl From<DIRCLRP20_A> for bool {
    #[inline(always)]
    fn from(variant: DIRCLRP20_A) -> Self {
        variant as u8 != 0
    }
}
impl DIRCLRP20_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DIRCLRP20_A {
        match self.bits {
            false => DIRCLRP20_A::DIRCLR_0,
            true => DIRCLRP20_A::DIRCLR_1,
        }
    }
    #[doc = "Checks if the value of the field is `DIRCLR_0`"]
    #[inline(always)]
    pub fn is_dirclr_0(&self) -> bool {
        *self == DIRCLRP20_A::DIRCLR_0
    }
    #[doc = "Checks if the value of the field is `DIRCLR_1`"]
    #[inline(always)]
    pub fn is_dirclr_1(&self) -> bool {
        *self == DIRCLRP20_A::DIRCLR_1
    }
}
#[doc = "Field `DIRCLRP20` writer - Clear direction bits."]
pub type DIRCLRP20_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, DIRCLR_SPEC, DIRCLRP20_A, O>;
impl<'a, const O: u8> DIRCLRP20_W<'a, O> {
    #[doc = "No operation"]
    #[inline(always)]
    pub fn dirclr_0(self) -> &'a mut W {
        self.variant(DIRCLRP20_A::DIRCLR_0)
    }
    #[doc = "Clears direction bits"]
    #[inline(always)]
    pub fn dirclr_1(self) -> &'a mut W {
        self.variant(DIRCLRP20_A::DIRCLR_1)
    }
}
#[doc = "Field `DIRCLRP21` reader - Clear direction bits."]
pub type DIRCLRP21_R = crate::BitReader<DIRCLRP21_A>;
#[doc = "Clear direction bits.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DIRCLRP21_A {
    #[doc = "0: No operation"]
    DIRCLR_0 = 0,
    #[doc = "1: Clears direction bits"]
    DIRCLR_1 = 1,
}
impl From<DIRCLRP21_A> for bool {
    #[inline(always)]
    fn from(variant: DIRCLRP21_A) -> Self {
        variant as u8 != 0
    }
}
impl DIRCLRP21_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DIRCLRP21_A {
        match self.bits {
            false => DIRCLRP21_A::DIRCLR_0,
            true => DIRCLRP21_A::DIRCLR_1,
        }
    }
    #[doc = "Checks if the value of the field is `DIRCLR_0`"]
    #[inline(always)]
    pub fn is_dirclr_0(&self) -> bool {
        *self == DIRCLRP21_A::DIRCLR_0
    }
    #[doc = "Checks if the value of the field is `DIRCLR_1`"]
    #[inline(always)]
    pub fn is_dirclr_1(&self) -> bool {
        *self == DIRCLRP21_A::DIRCLR_1
    }
}
#[doc = "Field `DIRCLRP21` writer - Clear direction bits."]
pub type DIRCLRP21_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, DIRCLR_SPEC, DIRCLRP21_A, O>;
impl<'a, const O: u8> DIRCLRP21_W<'a, O> {
    #[doc = "No operation"]
    #[inline(always)]
    pub fn dirclr_0(self) -> &'a mut W {
        self.variant(DIRCLRP21_A::DIRCLR_0)
    }
    #[doc = "Clears direction bits"]
    #[inline(always)]
    pub fn dirclr_1(self) -> &'a mut W {
        self.variant(DIRCLRP21_A::DIRCLR_1)
    }
}
#[doc = "Field `DIRCLRP22` reader - Clear direction bits."]
pub type DIRCLRP22_R = crate::BitReader<DIRCLRP22_A>;
#[doc = "Clear direction bits.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DIRCLRP22_A {
    #[doc = "0: No operation"]
    DIRCLR_0 = 0,
    #[doc = "1: Clears direction bits"]
    DIRCLR_1 = 1,
}
impl From<DIRCLRP22_A> for bool {
    #[inline(always)]
    fn from(variant: DIRCLRP22_A) -> Self {
        variant as u8 != 0
    }
}
impl DIRCLRP22_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DIRCLRP22_A {
        match self.bits {
            false => DIRCLRP22_A::DIRCLR_0,
            true => DIRCLRP22_A::DIRCLR_1,
        }
    }
    #[doc = "Checks if the value of the field is `DIRCLR_0`"]
    #[inline(always)]
    pub fn is_dirclr_0(&self) -> bool {
        *self == DIRCLRP22_A::DIRCLR_0
    }
    #[doc = "Checks if the value of the field is `DIRCLR_1`"]
    #[inline(always)]
    pub fn is_dirclr_1(&self) -> bool {
        *self == DIRCLRP22_A::DIRCLR_1
    }
}
#[doc = "Field `DIRCLRP22` writer - Clear direction bits."]
pub type DIRCLRP22_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, DIRCLR_SPEC, DIRCLRP22_A, O>;
impl<'a, const O: u8> DIRCLRP22_W<'a, O> {
    #[doc = "No operation"]
    #[inline(always)]
    pub fn dirclr_0(self) -> &'a mut W {
        self.variant(DIRCLRP22_A::DIRCLR_0)
    }
    #[doc = "Clears direction bits"]
    #[inline(always)]
    pub fn dirclr_1(self) -> &'a mut W {
        self.variant(DIRCLRP22_A::DIRCLR_1)
    }
}
#[doc = "Field `DIRCLRP23` reader - Clear direction bits."]
pub type DIRCLRP23_R = crate::BitReader<DIRCLRP23_A>;
#[doc = "Clear direction bits.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DIRCLRP23_A {
    #[doc = "0: No operation"]
    DIRCLR_0 = 0,
    #[doc = "1: Clears direction bits"]
    DIRCLR_1 = 1,
}
impl From<DIRCLRP23_A> for bool {
    #[inline(always)]
    fn from(variant: DIRCLRP23_A) -> Self {
        variant as u8 != 0
    }
}
impl DIRCLRP23_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DIRCLRP23_A {
        match self.bits {
            false => DIRCLRP23_A::DIRCLR_0,
            true => DIRCLRP23_A::DIRCLR_1,
        }
    }
    #[doc = "Checks if the value of the field is `DIRCLR_0`"]
    #[inline(always)]
    pub fn is_dirclr_0(&self) -> bool {
        *self == DIRCLRP23_A::DIRCLR_0
    }
    #[doc = "Checks if the value of the field is `DIRCLR_1`"]
    #[inline(always)]
    pub fn is_dirclr_1(&self) -> bool {
        *self == DIRCLRP23_A::DIRCLR_1
    }
}
#[doc = "Field `DIRCLRP23` writer - Clear direction bits."]
pub type DIRCLRP23_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, DIRCLR_SPEC, DIRCLRP23_A, O>;
impl<'a, const O: u8> DIRCLRP23_W<'a, O> {
    #[doc = "No operation"]
    #[inline(always)]
    pub fn dirclr_0(self) -> &'a mut W {
        self.variant(DIRCLRP23_A::DIRCLR_0)
    }
    #[doc = "Clears direction bits"]
    #[inline(always)]
    pub fn dirclr_1(self) -> &'a mut W {
        self.variant(DIRCLRP23_A::DIRCLR_1)
    }
}
#[doc = "Field `DIRCLRP24` reader - Clear direction bits."]
pub type DIRCLRP24_R = crate::BitReader<DIRCLRP24_A>;
#[doc = "Clear direction bits.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DIRCLRP24_A {
    #[doc = "0: No operation"]
    DIRCLR_0 = 0,
    #[doc = "1: Clears direction bits"]
    DIRCLR_1 = 1,
}
impl From<DIRCLRP24_A> for bool {
    #[inline(always)]
    fn from(variant: DIRCLRP24_A) -> Self {
        variant as u8 != 0
    }
}
impl DIRCLRP24_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DIRCLRP24_A {
        match self.bits {
            false => DIRCLRP24_A::DIRCLR_0,
            true => DIRCLRP24_A::DIRCLR_1,
        }
    }
    #[doc = "Checks if the value of the field is `DIRCLR_0`"]
    #[inline(always)]
    pub fn is_dirclr_0(&self) -> bool {
        *self == DIRCLRP24_A::DIRCLR_0
    }
    #[doc = "Checks if the value of the field is `DIRCLR_1`"]
    #[inline(always)]
    pub fn is_dirclr_1(&self) -> bool {
        *self == DIRCLRP24_A::DIRCLR_1
    }
}
#[doc = "Field `DIRCLRP24` writer - Clear direction bits."]
pub type DIRCLRP24_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, DIRCLR_SPEC, DIRCLRP24_A, O>;
impl<'a, const O: u8> DIRCLRP24_W<'a, O> {
    #[doc = "No operation"]
    #[inline(always)]
    pub fn dirclr_0(self) -> &'a mut W {
        self.variant(DIRCLRP24_A::DIRCLR_0)
    }
    #[doc = "Clears direction bits"]
    #[inline(always)]
    pub fn dirclr_1(self) -> &'a mut W {
        self.variant(DIRCLRP24_A::DIRCLR_1)
    }
}
#[doc = "Field `DIRCLRP25` reader - Clear direction bits."]
pub type DIRCLRP25_R = crate::BitReader<DIRCLRP25_A>;
#[doc = "Clear direction bits.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DIRCLRP25_A {
    #[doc = "0: No operation"]
    DIRCLR_0 = 0,
    #[doc = "1: Clears direction bits"]
    DIRCLR_1 = 1,
}
impl From<DIRCLRP25_A> for bool {
    #[inline(always)]
    fn from(variant: DIRCLRP25_A) -> Self {
        variant as u8 != 0
    }
}
impl DIRCLRP25_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DIRCLRP25_A {
        match self.bits {
            false => DIRCLRP25_A::DIRCLR_0,
            true => DIRCLRP25_A::DIRCLR_1,
        }
    }
    #[doc = "Checks if the value of the field is `DIRCLR_0`"]
    #[inline(always)]
    pub fn is_dirclr_0(&self) -> bool {
        *self == DIRCLRP25_A::DIRCLR_0
    }
    #[doc = "Checks if the value of the field is `DIRCLR_1`"]
    #[inline(always)]
    pub fn is_dirclr_1(&self) -> bool {
        *self == DIRCLRP25_A::DIRCLR_1
    }
}
#[doc = "Field `DIRCLRP25` writer - Clear direction bits."]
pub type DIRCLRP25_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, DIRCLR_SPEC, DIRCLRP25_A, O>;
impl<'a, const O: u8> DIRCLRP25_W<'a, O> {
    #[doc = "No operation"]
    #[inline(always)]
    pub fn dirclr_0(self) -> &'a mut W {
        self.variant(DIRCLRP25_A::DIRCLR_0)
    }
    #[doc = "Clears direction bits"]
    #[inline(always)]
    pub fn dirclr_1(self) -> &'a mut W {
        self.variant(DIRCLRP25_A::DIRCLR_1)
    }
}
#[doc = "Field `DIRCLRP26` reader - Clear direction bits."]
pub type DIRCLRP26_R = crate::BitReader<DIRCLRP26_A>;
#[doc = "Clear direction bits.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DIRCLRP26_A {
    #[doc = "0: No operation"]
    DIRCLR_0 = 0,
    #[doc = "1: Clears direction bits"]
    DIRCLR_1 = 1,
}
impl From<DIRCLRP26_A> for bool {
    #[inline(always)]
    fn from(variant: DIRCLRP26_A) -> Self {
        variant as u8 != 0
    }
}
impl DIRCLRP26_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DIRCLRP26_A {
        match self.bits {
            false => DIRCLRP26_A::DIRCLR_0,
            true => DIRCLRP26_A::DIRCLR_1,
        }
    }
    #[doc = "Checks if the value of the field is `DIRCLR_0`"]
    #[inline(always)]
    pub fn is_dirclr_0(&self) -> bool {
        *self == DIRCLRP26_A::DIRCLR_0
    }
    #[doc = "Checks if the value of the field is `DIRCLR_1`"]
    #[inline(always)]
    pub fn is_dirclr_1(&self) -> bool {
        *self == DIRCLRP26_A::DIRCLR_1
    }
}
#[doc = "Field `DIRCLRP26` writer - Clear direction bits."]
pub type DIRCLRP26_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, DIRCLR_SPEC, DIRCLRP26_A, O>;
impl<'a, const O: u8> DIRCLRP26_W<'a, O> {
    #[doc = "No operation"]
    #[inline(always)]
    pub fn dirclr_0(self) -> &'a mut W {
        self.variant(DIRCLRP26_A::DIRCLR_0)
    }
    #[doc = "Clears direction bits"]
    #[inline(always)]
    pub fn dirclr_1(self) -> &'a mut W {
        self.variant(DIRCLRP26_A::DIRCLR_1)
    }
}
#[doc = "Field `DIRCLRP27` reader - Clear direction bits."]
pub type DIRCLRP27_R = crate::BitReader<DIRCLRP27_A>;
#[doc = "Clear direction bits.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DIRCLRP27_A {
    #[doc = "0: No operation"]
    DIRCLR_0 = 0,
    #[doc = "1: Clears direction bits"]
    DIRCLR_1 = 1,
}
impl From<DIRCLRP27_A> for bool {
    #[inline(always)]
    fn from(variant: DIRCLRP27_A) -> Self {
        variant as u8 != 0
    }
}
impl DIRCLRP27_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DIRCLRP27_A {
        match self.bits {
            false => DIRCLRP27_A::DIRCLR_0,
            true => DIRCLRP27_A::DIRCLR_1,
        }
    }
    #[doc = "Checks if the value of the field is `DIRCLR_0`"]
    #[inline(always)]
    pub fn is_dirclr_0(&self) -> bool {
        *self == DIRCLRP27_A::DIRCLR_0
    }
    #[doc = "Checks if the value of the field is `DIRCLR_1`"]
    #[inline(always)]
    pub fn is_dirclr_1(&self) -> bool {
        *self == DIRCLRP27_A::DIRCLR_1
    }
}
#[doc = "Field `DIRCLRP27` writer - Clear direction bits."]
pub type DIRCLRP27_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, DIRCLR_SPEC, DIRCLRP27_A, O>;
impl<'a, const O: u8> DIRCLRP27_W<'a, O> {
    #[doc = "No operation"]
    #[inline(always)]
    pub fn dirclr_0(self) -> &'a mut W {
        self.variant(DIRCLRP27_A::DIRCLR_0)
    }
    #[doc = "Clears direction bits"]
    #[inline(always)]
    pub fn dirclr_1(self) -> &'a mut W {
        self.variant(DIRCLRP27_A::DIRCLR_1)
    }
}
#[doc = "Field `DIRCLRP28` reader - Clear direction bits."]
pub type DIRCLRP28_R = crate::BitReader<DIRCLRP28_A>;
#[doc = "Clear direction bits.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DIRCLRP28_A {
    #[doc = "0: No operation"]
    DIRCLR_0 = 0,
    #[doc = "1: Clears direction bits"]
    DIRCLR_1 = 1,
}
impl From<DIRCLRP28_A> for bool {
    #[inline(always)]
    fn from(variant: DIRCLRP28_A) -> Self {
        variant as u8 != 0
    }
}
impl DIRCLRP28_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DIRCLRP28_A {
        match self.bits {
            false => DIRCLRP28_A::DIRCLR_0,
            true => DIRCLRP28_A::DIRCLR_1,
        }
    }
    #[doc = "Checks if the value of the field is `DIRCLR_0`"]
    #[inline(always)]
    pub fn is_dirclr_0(&self) -> bool {
        *self == DIRCLRP28_A::DIRCLR_0
    }
    #[doc = "Checks if the value of the field is `DIRCLR_1`"]
    #[inline(always)]
    pub fn is_dirclr_1(&self) -> bool {
        *self == DIRCLRP28_A::DIRCLR_1
    }
}
#[doc = "Field `DIRCLRP28` writer - Clear direction bits."]
pub type DIRCLRP28_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, DIRCLR_SPEC, DIRCLRP28_A, O>;
impl<'a, const O: u8> DIRCLRP28_W<'a, O> {
    #[doc = "No operation"]
    #[inline(always)]
    pub fn dirclr_0(self) -> &'a mut W {
        self.variant(DIRCLRP28_A::DIRCLR_0)
    }
    #[doc = "Clears direction bits"]
    #[inline(always)]
    pub fn dirclr_1(self) -> &'a mut W {
        self.variant(DIRCLRP28_A::DIRCLR_1)
    }
}
#[doc = "Field `DIRCLRP29` reader - Clear direction bits."]
pub type DIRCLRP29_R = crate::BitReader<DIRCLRP29_A>;
#[doc = "Clear direction bits.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DIRCLRP29_A {
    #[doc = "0: No operation"]
    DIRCLR_0 = 0,
    #[doc = "1: Clears direction bits"]
    DIRCLR_1 = 1,
}
impl From<DIRCLRP29_A> for bool {
    #[inline(always)]
    fn from(variant: DIRCLRP29_A) -> Self {
        variant as u8 != 0
    }
}
impl DIRCLRP29_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DIRCLRP29_A {
        match self.bits {
            false => DIRCLRP29_A::DIRCLR_0,
            true => DIRCLRP29_A::DIRCLR_1,
        }
    }
    #[doc = "Checks if the value of the field is `DIRCLR_0`"]
    #[inline(always)]
    pub fn is_dirclr_0(&self) -> bool {
        *self == DIRCLRP29_A::DIRCLR_0
    }
    #[doc = "Checks if the value of the field is `DIRCLR_1`"]
    #[inline(always)]
    pub fn is_dirclr_1(&self) -> bool {
        *self == DIRCLRP29_A::DIRCLR_1
    }
}
#[doc = "Field `DIRCLRP29` writer - Clear direction bits."]
pub type DIRCLRP29_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, DIRCLR_SPEC, DIRCLRP29_A, O>;
impl<'a, const O: u8> DIRCLRP29_W<'a, O> {
    #[doc = "No operation"]
    #[inline(always)]
    pub fn dirclr_0(self) -> &'a mut W {
        self.variant(DIRCLRP29_A::DIRCLR_0)
    }
    #[doc = "Clears direction bits"]
    #[inline(always)]
    pub fn dirclr_1(self) -> &'a mut W {
        self.variant(DIRCLRP29_A::DIRCLR_1)
    }
}
#[doc = "Field `DIRCLRP30` reader - Clear direction bits."]
pub type DIRCLRP30_R = crate::BitReader<DIRCLRP30_A>;
#[doc = "Clear direction bits.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DIRCLRP30_A {
    #[doc = "0: No operation"]
    DIRCLR_0 = 0,
    #[doc = "1: Clears direction bits"]
    DIRCLR_1 = 1,
}
impl From<DIRCLRP30_A> for bool {
    #[inline(always)]
    fn from(variant: DIRCLRP30_A) -> Self {
        variant as u8 != 0
    }
}
impl DIRCLRP30_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DIRCLRP30_A {
        match self.bits {
            false => DIRCLRP30_A::DIRCLR_0,
            true => DIRCLRP30_A::DIRCLR_1,
        }
    }
    #[doc = "Checks if the value of the field is `DIRCLR_0`"]
    #[inline(always)]
    pub fn is_dirclr_0(&self) -> bool {
        *self == DIRCLRP30_A::DIRCLR_0
    }
    #[doc = "Checks if the value of the field is `DIRCLR_1`"]
    #[inline(always)]
    pub fn is_dirclr_1(&self) -> bool {
        *self == DIRCLRP30_A::DIRCLR_1
    }
}
#[doc = "Field `DIRCLRP30` writer - Clear direction bits."]
pub type DIRCLRP30_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, DIRCLR_SPEC, DIRCLRP30_A, O>;
impl<'a, const O: u8> DIRCLRP30_W<'a, O> {
    #[doc = "No operation"]
    #[inline(always)]
    pub fn dirclr_0(self) -> &'a mut W {
        self.variant(DIRCLRP30_A::DIRCLR_0)
    }
    #[doc = "Clears direction bits"]
    #[inline(always)]
    pub fn dirclr_1(self) -> &'a mut W {
        self.variant(DIRCLRP30_A::DIRCLR_1)
    }
}
#[doc = "Field `DIRCLRP31` reader - Clear direction bits."]
pub type DIRCLRP31_R = crate::BitReader<DIRCLRP31_A>;
#[doc = "Clear direction bits.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DIRCLRP31_A {
    #[doc = "0: No operation"]
    DIRCLR_0 = 0,
    #[doc = "1: Clears direction bits"]
    DIRCLR_1 = 1,
}
impl From<DIRCLRP31_A> for bool {
    #[inline(always)]
    fn from(variant: DIRCLRP31_A) -> Self {
        variant as u8 != 0
    }
}
impl DIRCLRP31_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DIRCLRP31_A {
        match self.bits {
            false => DIRCLRP31_A::DIRCLR_0,
            true => DIRCLRP31_A::DIRCLR_1,
        }
    }
    #[doc = "Checks if the value of the field is `DIRCLR_0`"]
    #[inline(always)]
    pub fn is_dirclr_0(&self) -> bool {
        *self == DIRCLRP31_A::DIRCLR_0
    }
    #[doc = "Checks if the value of the field is `DIRCLR_1`"]
    #[inline(always)]
    pub fn is_dirclr_1(&self) -> bool {
        *self == DIRCLRP31_A::DIRCLR_1
    }
}
#[doc = "Field `DIRCLRP31` writer - Clear direction bits."]
pub type DIRCLRP31_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, DIRCLR_SPEC, DIRCLRP31_A, O>;
impl<'a, const O: u8> DIRCLRP31_W<'a, O> {
    #[doc = "No operation"]
    #[inline(always)]
    pub fn dirclr_0(self) -> &'a mut W {
        self.variant(DIRCLRP31_A::DIRCLR_0)
    }
    #[doc = "Clears direction bits"]
    #[inline(always)]
    pub fn dirclr_1(self) -> &'a mut W {
        self.variant(DIRCLRP31_A::DIRCLR_1)
    }
}
impl R {
    #[doc = "Bit 0 - Clear direction bits."]
    #[inline(always)]
    pub fn dirclrp0(&self) -> DIRCLRP0_R {
        DIRCLRP0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Clear direction bits."]
    #[inline(always)]
    pub fn dirclrp1(&self) -> DIRCLRP1_R {
        DIRCLRP1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Clear direction bits."]
    #[inline(always)]
    pub fn dirclrp2(&self) -> DIRCLRP2_R {
        DIRCLRP2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Clear direction bits."]
    #[inline(always)]
    pub fn dirclrp3(&self) -> DIRCLRP3_R {
        DIRCLRP3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Clear direction bits."]
    #[inline(always)]
    pub fn dirclrp4(&self) -> DIRCLRP4_R {
        DIRCLRP4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Clear direction bits."]
    #[inline(always)]
    pub fn dirclrp5(&self) -> DIRCLRP5_R {
        DIRCLRP5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Clear direction bits."]
    #[inline(always)]
    pub fn dirclrp6(&self) -> DIRCLRP6_R {
        DIRCLRP6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Clear direction bits."]
    #[inline(always)]
    pub fn dirclrp7(&self) -> DIRCLRP7_R {
        DIRCLRP7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Clear direction bits."]
    #[inline(always)]
    pub fn dirclrp8(&self) -> DIRCLRP8_R {
        DIRCLRP8_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Clear direction bits."]
    #[inline(always)]
    pub fn dirclrp9(&self) -> DIRCLRP9_R {
        DIRCLRP9_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Clear direction bits."]
    #[inline(always)]
    pub fn dirclrp10(&self) -> DIRCLRP10_R {
        DIRCLRP10_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Clear direction bits."]
    #[inline(always)]
    pub fn dirclrp11(&self) -> DIRCLRP11_R {
        DIRCLRP11_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Clear direction bits."]
    #[inline(always)]
    pub fn dirclrp12(&self) -> DIRCLRP12_R {
        DIRCLRP12_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Clear direction bits."]
    #[inline(always)]
    pub fn dirclrp13(&self) -> DIRCLRP13_R {
        DIRCLRP13_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Clear direction bits."]
    #[inline(always)]
    pub fn dirclrp14(&self) -> DIRCLRP14_R {
        DIRCLRP14_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Clear direction bits."]
    #[inline(always)]
    pub fn dirclrp15(&self) -> DIRCLRP15_R {
        DIRCLRP15_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Clear direction bits."]
    #[inline(always)]
    pub fn dirclrp16(&self) -> DIRCLRP16_R {
        DIRCLRP16_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Clear direction bits."]
    #[inline(always)]
    pub fn dirclrp17(&self) -> DIRCLRP17_R {
        DIRCLRP17_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Clear direction bits."]
    #[inline(always)]
    pub fn dirclrp18(&self) -> DIRCLRP18_R {
        DIRCLRP18_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Clear direction bits."]
    #[inline(always)]
    pub fn dirclrp19(&self) -> DIRCLRP19_R {
        DIRCLRP19_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Clear direction bits."]
    #[inline(always)]
    pub fn dirclrp20(&self) -> DIRCLRP20_R {
        DIRCLRP20_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Clear direction bits."]
    #[inline(always)]
    pub fn dirclrp21(&self) -> DIRCLRP21_R {
        DIRCLRP21_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Clear direction bits."]
    #[inline(always)]
    pub fn dirclrp22(&self) -> DIRCLRP22_R {
        DIRCLRP22_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Clear direction bits."]
    #[inline(always)]
    pub fn dirclrp23(&self) -> DIRCLRP23_R {
        DIRCLRP23_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Clear direction bits."]
    #[inline(always)]
    pub fn dirclrp24(&self) -> DIRCLRP24_R {
        DIRCLRP24_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Clear direction bits."]
    #[inline(always)]
    pub fn dirclrp25(&self) -> DIRCLRP25_R {
        DIRCLRP25_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Clear direction bits."]
    #[inline(always)]
    pub fn dirclrp26(&self) -> DIRCLRP26_R {
        DIRCLRP26_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Clear direction bits."]
    #[inline(always)]
    pub fn dirclrp27(&self) -> DIRCLRP27_R {
        DIRCLRP27_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Clear direction bits."]
    #[inline(always)]
    pub fn dirclrp28(&self) -> DIRCLRP28_R {
        DIRCLRP28_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Clear direction bits."]
    #[inline(always)]
    pub fn dirclrp29(&self) -> DIRCLRP29_R {
        DIRCLRP29_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Clear direction bits."]
    #[inline(always)]
    pub fn dirclrp30(&self) -> DIRCLRP30_R {
        DIRCLRP30_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Clear direction bits."]
    #[inline(always)]
    pub fn dirclrp31(&self) -> DIRCLRP31_R {
        DIRCLRP31_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Clear direction bits."]
    #[inline(always)]
    #[must_use]
    pub fn dirclrp0(&mut self) -> DIRCLRP0_W<0> {
        DIRCLRP0_W::new(self)
    }
    #[doc = "Bit 1 - Clear direction bits."]
    #[inline(always)]
    #[must_use]
    pub fn dirclrp1(&mut self) -> DIRCLRP1_W<1> {
        DIRCLRP1_W::new(self)
    }
    #[doc = "Bit 2 - Clear direction bits."]
    #[inline(always)]
    #[must_use]
    pub fn dirclrp2(&mut self) -> DIRCLRP2_W<2> {
        DIRCLRP2_W::new(self)
    }
    #[doc = "Bit 3 - Clear direction bits."]
    #[inline(always)]
    #[must_use]
    pub fn dirclrp3(&mut self) -> DIRCLRP3_W<3> {
        DIRCLRP3_W::new(self)
    }
    #[doc = "Bit 4 - Clear direction bits."]
    #[inline(always)]
    #[must_use]
    pub fn dirclrp4(&mut self) -> DIRCLRP4_W<4> {
        DIRCLRP4_W::new(self)
    }
    #[doc = "Bit 5 - Clear direction bits."]
    #[inline(always)]
    #[must_use]
    pub fn dirclrp5(&mut self) -> DIRCLRP5_W<5> {
        DIRCLRP5_W::new(self)
    }
    #[doc = "Bit 6 - Clear direction bits."]
    #[inline(always)]
    #[must_use]
    pub fn dirclrp6(&mut self) -> DIRCLRP6_W<6> {
        DIRCLRP6_W::new(self)
    }
    #[doc = "Bit 7 - Clear direction bits."]
    #[inline(always)]
    #[must_use]
    pub fn dirclrp7(&mut self) -> DIRCLRP7_W<7> {
        DIRCLRP7_W::new(self)
    }
    #[doc = "Bit 8 - Clear direction bits."]
    #[inline(always)]
    #[must_use]
    pub fn dirclrp8(&mut self) -> DIRCLRP8_W<8> {
        DIRCLRP8_W::new(self)
    }
    #[doc = "Bit 9 - Clear direction bits."]
    #[inline(always)]
    #[must_use]
    pub fn dirclrp9(&mut self) -> DIRCLRP9_W<9> {
        DIRCLRP9_W::new(self)
    }
    #[doc = "Bit 10 - Clear direction bits."]
    #[inline(always)]
    #[must_use]
    pub fn dirclrp10(&mut self) -> DIRCLRP10_W<10> {
        DIRCLRP10_W::new(self)
    }
    #[doc = "Bit 11 - Clear direction bits."]
    #[inline(always)]
    #[must_use]
    pub fn dirclrp11(&mut self) -> DIRCLRP11_W<11> {
        DIRCLRP11_W::new(self)
    }
    #[doc = "Bit 12 - Clear direction bits."]
    #[inline(always)]
    #[must_use]
    pub fn dirclrp12(&mut self) -> DIRCLRP12_W<12> {
        DIRCLRP12_W::new(self)
    }
    #[doc = "Bit 13 - Clear direction bits."]
    #[inline(always)]
    #[must_use]
    pub fn dirclrp13(&mut self) -> DIRCLRP13_W<13> {
        DIRCLRP13_W::new(self)
    }
    #[doc = "Bit 14 - Clear direction bits."]
    #[inline(always)]
    #[must_use]
    pub fn dirclrp14(&mut self) -> DIRCLRP14_W<14> {
        DIRCLRP14_W::new(self)
    }
    #[doc = "Bit 15 - Clear direction bits."]
    #[inline(always)]
    #[must_use]
    pub fn dirclrp15(&mut self) -> DIRCLRP15_W<15> {
        DIRCLRP15_W::new(self)
    }
    #[doc = "Bit 16 - Clear direction bits."]
    #[inline(always)]
    #[must_use]
    pub fn dirclrp16(&mut self) -> DIRCLRP16_W<16> {
        DIRCLRP16_W::new(self)
    }
    #[doc = "Bit 17 - Clear direction bits."]
    #[inline(always)]
    #[must_use]
    pub fn dirclrp17(&mut self) -> DIRCLRP17_W<17> {
        DIRCLRP17_W::new(self)
    }
    #[doc = "Bit 18 - Clear direction bits."]
    #[inline(always)]
    #[must_use]
    pub fn dirclrp18(&mut self) -> DIRCLRP18_W<18> {
        DIRCLRP18_W::new(self)
    }
    #[doc = "Bit 19 - Clear direction bits."]
    #[inline(always)]
    #[must_use]
    pub fn dirclrp19(&mut self) -> DIRCLRP19_W<19> {
        DIRCLRP19_W::new(self)
    }
    #[doc = "Bit 20 - Clear direction bits."]
    #[inline(always)]
    #[must_use]
    pub fn dirclrp20(&mut self) -> DIRCLRP20_W<20> {
        DIRCLRP20_W::new(self)
    }
    #[doc = "Bit 21 - Clear direction bits."]
    #[inline(always)]
    #[must_use]
    pub fn dirclrp21(&mut self) -> DIRCLRP21_W<21> {
        DIRCLRP21_W::new(self)
    }
    #[doc = "Bit 22 - Clear direction bits."]
    #[inline(always)]
    #[must_use]
    pub fn dirclrp22(&mut self) -> DIRCLRP22_W<22> {
        DIRCLRP22_W::new(self)
    }
    #[doc = "Bit 23 - Clear direction bits."]
    #[inline(always)]
    #[must_use]
    pub fn dirclrp23(&mut self) -> DIRCLRP23_W<23> {
        DIRCLRP23_W::new(self)
    }
    #[doc = "Bit 24 - Clear direction bits."]
    #[inline(always)]
    #[must_use]
    pub fn dirclrp24(&mut self) -> DIRCLRP24_W<24> {
        DIRCLRP24_W::new(self)
    }
    #[doc = "Bit 25 - Clear direction bits."]
    #[inline(always)]
    #[must_use]
    pub fn dirclrp25(&mut self) -> DIRCLRP25_W<25> {
        DIRCLRP25_W::new(self)
    }
    #[doc = "Bit 26 - Clear direction bits."]
    #[inline(always)]
    #[must_use]
    pub fn dirclrp26(&mut self) -> DIRCLRP26_W<26> {
        DIRCLRP26_W::new(self)
    }
    #[doc = "Bit 27 - Clear direction bits."]
    #[inline(always)]
    #[must_use]
    pub fn dirclrp27(&mut self) -> DIRCLRP27_W<27> {
        DIRCLRP27_W::new(self)
    }
    #[doc = "Bit 28 - Clear direction bits."]
    #[inline(always)]
    #[must_use]
    pub fn dirclrp28(&mut self) -> DIRCLRP28_W<28> {
        DIRCLRP28_W::new(self)
    }
    #[doc = "Bit 29 - Clear direction bits."]
    #[inline(always)]
    #[must_use]
    pub fn dirclrp29(&mut self) -> DIRCLRP29_W<29> {
        DIRCLRP29_W::new(self)
    }
    #[doc = "Bit 30 - Clear direction bits."]
    #[inline(always)]
    #[must_use]
    pub fn dirclrp30(&mut self) -> DIRCLRP30_W<30> {
        DIRCLRP30_W::new(self)
    }
    #[doc = "Bit 31 - Clear direction bits."]
    #[inline(always)]
    #[must_use]
    pub fn dirclrp31(&mut self) -> DIRCLRP31_W<31> {
        DIRCLRP31_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Port direction clear\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dirclr](index.html) module"]
pub struct DIRCLR_SPEC;
impl crate::RegisterSpec for DIRCLR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dirclr::R](R) reader structure"]
impl crate::Readable for DIRCLR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dirclr::W](W) writer structure"]
impl crate::Writable for DIRCLR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0xffff_ffff;
}
#[doc = "`reset()` method sets DIRCLR[%s]
to value 0"]
impl crate::Resettable for DIRCLR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
