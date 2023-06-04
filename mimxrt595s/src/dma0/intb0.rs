#[doc = "Register `INTB0` reader"]
pub struct R(crate::R<INTB0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INTB0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INTB0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INTB0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `INTB0` writer"]
pub struct W(crate::W<INTB0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INTB0_SPEC>;
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
impl From<crate::W<INTB0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INTB0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `INTB0` reader - Interrupt B status for DMA channel."]
pub type INTB0_R = crate::BitReader<INTB0_A>;
#[doc = "Interrupt B status for DMA channel.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum INTB0_A {
    #[doc = "0: The DMA channel interrupt B is not active."]
    NOT_ACTIVE = 0,
    #[doc = "1: The DMA channel interrupt B is active."]
    ACTIVE = 1,
}
impl From<INTB0_A> for bool {
    #[inline(always)]
    fn from(variant: INTB0_A) -> Self {
        variant as u8 != 0
    }
}
impl INTB0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INTB0_A {
        match self.bits {
            false => INTB0_A::NOT_ACTIVE,
            true => INTB0_A::ACTIVE,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_ACTIVE`"]
    #[inline(always)]
    pub fn is_not_active(&self) -> bool {
        *self == INTB0_A::NOT_ACTIVE
    }
    #[doc = "Checks if the value of the field is `ACTIVE`"]
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        *self == INTB0_A::ACTIVE
    }
}
#[doc = "Field `INTB0` writer - Interrupt B status for DMA channel."]
pub type INTB0_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTB0_SPEC, INTB0_A, O>;
impl<'a, const O: u8> INTB0_W<'a, O> {
    #[doc = "The DMA channel interrupt B is not active."]
    #[inline(always)]
    pub fn not_active(self) -> &'a mut W {
        self.variant(INTB0_A::NOT_ACTIVE)
    }
    #[doc = "The DMA channel interrupt B is active."]
    #[inline(always)]
    pub fn active(self) -> &'a mut W {
        self.variant(INTB0_A::ACTIVE)
    }
}
#[doc = "Field `INTB1` reader - Interrupt B status for DMA channel."]
pub type INTB1_R = crate::BitReader<INTB1_A>;
#[doc = "Interrupt B status for DMA channel.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum INTB1_A {
    #[doc = "0: The DMA channel interrupt B is not active."]
    NOT_ACTIVE = 0,
    #[doc = "1: The DMA channel interrupt B is active."]
    ACTIVE = 1,
}
impl From<INTB1_A> for bool {
    #[inline(always)]
    fn from(variant: INTB1_A) -> Self {
        variant as u8 != 0
    }
}
impl INTB1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INTB1_A {
        match self.bits {
            false => INTB1_A::NOT_ACTIVE,
            true => INTB1_A::ACTIVE,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_ACTIVE`"]
    #[inline(always)]
    pub fn is_not_active(&self) -> bool {
        *self == INTB1_A::NOT_ACTIVE
    }
    #[doc = "Checks if the value of the field is `ACTIVE`"]
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        *self == INTB1_A::ACTIVE
    }
}
#[doc = "Field `INTB1` writer - Interrupt B status for DMA channel."]
pub type INTB1_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTB0_SPEC, INTB1_A, O>;
impl<'a, const O: u8> INTB1_W<'a, O> {
    #[doc = "The DMA channel interrupt B is not active."]
    #[inline(always)]
    pub fn not_active(self) -> &'a mut W {
        self.variant(INTB1_A::NOT_ACTIVE)
    }
    #[doc = "The DMA channel interrupt B is active."]
    #[inline(always)]
    pub fn active(self) -> &'a mut W {
        self.variant(INTB1_A::ACTIVE)
    }
}
#[doc = "Field `INTB2` reader - Interrupt B status for DMA channel."]
pub type INTB2_R = crate::BitReader<INTB2_A>;
#[doc = "Interrupt B status for DMA channel.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum INTB2_A {
    #[doc = "0: The DMA channel interrupt B is not active."]
    NOT_ACTIVE = 0,
    #[doc = "1: The DMA channel interrupt B is active."]
    ACTIVE = 1,
}
impl From<INTB2_A> for bool {
    #[inline(always)]
    fn from(variant: INTB2_A) -> Self {
        variant as u8 != 0
    }
}
impl INTB2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INTB2_A {
        match self.bits {
            false => INTB2_A::NOT_ACTIVE,
            true => INTB2_A::ACTIVE,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_ACTIVE`"]
    #[inline(always)]
    pub fn is_not_active(&self) -> bool {
        *self == INTB2_A::NOT_ACTIVE
    }
    #[doc = "Checks if the value of the field is `ACTIVE`"]
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        *self == INTB2_A::ACTIVE
    }
}
#[doc = "Field `INTB2` writer - Interrupt B status for DMA channel."]
pub type INTB2_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTB0_SPEC, INTB2_A, O>;
impl<'a, const O: u8> INTB2_W<'a, O> {
    #[doc = "The DMA channel interrupt B is not active."]
    #[inline(always)]
    pub fn not_active(self) -> &'a mut W {
        self.variant(INTB2_A::NOT_ACTIVE)
    }
    #[doc = "The DMA channel interrupt B is active."]
    #[inline(always)]
    pub fn active(self) -> &'a mut W {
        self.variant(INTB2_A::ACTIVE)
    }
}
#[doc = "Field `INTB3` reader - Interrupt B status for DMA channel."]
pub type INTB3_R = crate::BitReader<INTB3_A>;
#[doc = "Interrupt B status for DMA channel.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum INTB3_A {
    #[doc = "0: The DMA channel interrupt B is not active."]
    NOT_ACTIVE = 0,
    #[doc = "1: The DMA channel interrupt B is active."]
    ACTIVE = 1,
}
impl From<INTB3_A> for bool {
    #[inline(always)]
    fn from(variant: INTB3_A) -> Self {
        variant as u8 != 0
    }
}
impl INTB3_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INTB3_A {
        match self.bits {
            false => INTB3_A::NOT_ACTIVE,
            true => INTB3_A::ACTIVE,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_ACTIVE`"]
    #[inline(always)]
    pub fn is_not_active(&self) -> bool {
        *self == INTB3_A::NOT_ACTIVE
    }
    #[doc = "Checks if the value of the field is `ACTIVE`"]
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        *self == INTB3_A::ACTIVE
    }
}
#[doc = "Field `INTB3` writer - Interrupt B status for DMA channel."]
pub type INTB3_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTB0_SPEC, INTB3_A, O>;
impl<'a, const O: u8> INTB3_W<'a, O> {
    #[doc = "The DMA channel interrupt B is not active."]
    #[inline(always)]
    pub fn not_active(self) -> &'a mut W {
        self.variant(INTB3_A::NOT_ACTIVE)
    }
    #[doc = "The DMA channel interrupt B is active."]
    #[inline(always)]
    pub fn active(self) -> &'a mut W {
        self.variant(INTB3_A::ACTIVE)
    }
}
#[doc = "Field `INTB4` reader - Interrupt B status for DMA channel."]
pub type INTB4_R = crate::BitReader<INTB4_A>;
#[doc = "Interrupt B status for DMA channel.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum INTB4_A {
    #[doc = "0: The DMA channel interrupt B is not active."]
    NOT_ACTIVE = 0,
    #[doc = "1: The DMA channel interrupt B is active."]
    ACTIVE = 1,
}
impl From<INTB4_A> for bool {
    #[inline(always)]
    fn from(variant: INTB4_A) -> Self {
        variant as u8 != 0
    }
}
impl INTB4_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INTB4_A {
        match self.bits {
            false => INTB4_A::NOT_ACTIVE,
            true => INTB4_A::ACTIVE,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_ACTIVE`"]
    #[inline(always)]
    pub fn is_not_active(&self) -> bool {
        *self == INTB4_A::NOT_ACTIVE
    }
    #[doc = "Checks if the value of the field is `ACTIVE`"]
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        *self == INTB4_A::ACTIVE
    }
}
#[doc = "Field `INTB4` writer - Interrupt B status for DMA channel."]
pub type INTB4_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTB0_SPEC, INTB4_A, O>;
impl<'a, const O: u8> INTB4_W<'a, O> {
    #[doc = "The DMA channel interrupt B is not active."]
    #[inline(always)]
    pub fn not_active(self) -> &'a mut W {
        self.variant(INTB4_A::NOT_ACTIVE)
    }
    #[doc = "The DMA channel interrupt B is active."]
    #[inline(always)]
    pub fn active(self) -> &'a mut W {
        self.variant(INTB4_A::ACTIVE)
    }
}
#[doc = "Field `INTB5` reader - Interrupt B status for DMA channel."]
pub type INTB5_R = crate::BitReader<INTB5_A>;
#[doc = "Interrupt B status for DMA channel.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum INTB5_A {
    #[doc = "0: The DMA channel interrupt B is not active."]
    NOT_ACTIVE = 0,
    #[doc = "1: The DMA channel interrupt B is active."]
    ACTIVE = 1,
}
impl From<INTB5_A> for bool {
    #[inline(always)]
    fn from(variant: INTB5_A) -> Self {
        variant as u8 != 0
    }
}
impl INTB5_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INTB5_A {
        match self.bits {
            false => INTB5_A::NOT_ACTIVE,
            true => INTB5_A::ACTIVE,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_ACTIVE`"]
    #[inline(always)]
    pub fn is_not_active(&self) -> bool {
        *self == INTB5_A::NOT_ACTIVE
    }
    #[doc = "Checks if the value of the field is `ACTIVE`"]
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        *self == INTB5_A::ACTIVE
    }
}
#[doc = "Field `INTB5` writer - Interrupt B status for DMA channel."]
pub type INTB5_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTB0_SPEC, INTB5_A, O>;
impl<'a, const O: u8> INTB5_W<'a, O> {
    #[doc = "The DMA channel interrupt B is not active."]
    #[inline(always)]
    pub fn not_active(self) -> &'a mut W {
        self.variant(INTB5_A::NOT_ACTIVE)
    }
    #[doc = "The DMA channel interrupt B is active."]
    #[inline(always)]
    pub fn active(self) -> &'a mut W {
        self.variant(INTB5_A::ACTIVE)
    }
}
#[doc = "Field `INTB6` reader - Interrupt B status for DMA channel."]
pub type INTB6_R = crate::BitReader<INTB6_A>;
#[doc = "Interrupt B status for DMA channel.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum INTB6_A {
    #[doc = "0: The DMA channel interrupt B is not active."]
    NOT_ACTIVE = 0,
    #[doc = "1: The DMA channel interrupt B is active."]
    ACTIVE = 1,
}
impl From<INTB6_A> for bool {
    #[inline(always)]
    fn from(variant: INTB6_A) -> Self {
        variant as u8 != 0
    }
}
impl INTB6_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INTB6_A {
        match self.bits {
            false => INTB6_A::NOT_ACTIVE,
            true => INTB6_A::ACTIVE,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_ACTIVE`"]
    #[inline(always)]
    pub fn is_not_active(&self) -> bool {
        *self == INTB6_A::NOT_ACTIVE
    }
    #[doc = "Checks if the value of the field is `ACTIVE`"]
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        *self == INTB6_A::ACTIVE
    }
}
#[doc = "Field `INTB6` writer - Interrupt B status for DMA channel."]
pub type INTB6_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTB0_SPEC, INTB6_A, O>;
impl<'a, const O: u8> INTB6_W<'a, O> {
    #[doc = "The DMA channel interrupt B is not active."]
    #[inline(always)]
    pub fn not_active(self) -> &'a mut W {
        self.variant(INTB6_A::NOT_ACTIVE)
    }
    #[doc = "The DMA channel interrupt B is active."]
    #[inline(always)]
    pub fn active(self) -> &'a mut W {
        self.variant(INTB6_A::ACTIVE)
    }
}
#[doc = "Field `INTB7` reader - Interrupt B status for DMA channel."]
pub type INTB7_R = crate::BitReader<INTB7_A>;
#[doc = "Interrupt B status for DMA channel.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum INTB7_A {
    #[doc = "0: The DMA channel interrupt B is not active."]
    NOT_ACTIVE = 0,
    #[doc = "1: The DMA channel interrupt B is active."]
    ACTIVE = 1,
}
impl From<INTB7_A> for bool {
    #[inline(always)]
    fn from(variant: INTB7_A) -> Self {
        variant as u8 != 0
    }
}
impl INTB7_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INTB7_A {
        match self.bits {
            false => INTB7_A::NOT_ACTIVE,
            true => INTB7_A::ACTIVE,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_ACTIVE`"]
    #[inline(always)]
    pub fn is_not_active(&self) -> bool {
        *self == INTB7_A::NOT_ACTIVE
    }
    #[doc = "Checks if the value of the field is `ACTIVE`"]
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        *self == INTB7_A::ACTIVE
    }
}
#[doc = "Field `INTB7` writer - Interrupt B status for DMA channel."]
pub type INTB7_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTB0_SPEC, INTB7_A, O>;
impl<'a, const O: u8> INTB7_W<'a, O> {
    #[doc = "The DMA channel interrupt B is not active."]
    #[inline(always)]
    pub fn not_active(self) -> &'a mut W {
        self.variant(INTB7_A::NOT_ACTIVE)
    }
    #[doc = "The DMA channel interrupt B is active."]
    #[inline(always)]
    pub fn active(self) -> &'a mut W {
        self.variant(INTB7_A::ACTIVE)
    }
}
#[doc = "Field `INTB8` reader - Interrupt B status for DMA channel."]
pub type INTB8_R = crate::BitReader<INTB8_A>;
#[doc = "Interrupt B status for DMA channel.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum INTB8_A {
    #[doc = "0: The DMA channel interrupt B is not active."]
    NOT_ACTIVE = 0,
    #[doc = "1: The DMA channel interrupt B is active."]
    ACTIVE = 1,
}
impl From<INTB8_A> for bool {
    #[inline(always)]
    fn from(variant: INTB8_A) -> Self {
        variant as u8 != 0
    }
}
impl INTB8_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INTB8_A {
        match self.bits {
            false => INTB8_A::NOT_ACTIVE,
            true => INTB8_A::ACTIVE,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_ACTIVE`"]
    #[inline(always)]
    pub fn is_not_active(&self) -> bool {
        *self == INTB8_A::NOT_ACTIVE
    }
    #[doc = "Checks if the value of the field is `ACTIVE`"]
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        *self == INTB8_A::ACTIVE
    }
}
#[doc = "Field `INTB8` writer - Interrupt B status for DMA channel."]
pub type INTB8_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTB0_SPEC, INTB8_A, O>;
impl<'a, const O: u8> INTB8_W<'a, O> {
    #[doc = "The DMA channel interrupt B is not active."]
    #[inline(always)]
    pub fn not_active(self) -> &'a mut W {
        self.variant(INTB8_A::NOT_ACTIVE)
    }
    #[doc = "The DMA channel interrupt B is active."]
    #[inline(always)]
    pub fn active(self) -> &'a mut W {
        self.variant(INTB8_A::ACTIVE)
    }
}
#[doc = "Field `INTB9` reader - Interrupt B status for DMA channel."]
pub type INTB9_R = crate::BitReader<INTB9_A>;
#[doc = "Interrupt B status for DMA channel.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum INTB9_A {
    #[doc = "0: The DMA channel interrupt B is not active."]
    NOT_ACTIVE = 0,
    #[doc = "1: The DMA channel interrupt B is active."]
    ACTIVE = 1,
}
impl From<INTB9_A> for bool {
    #[inline(always)]
    fn from(variant: INTB9_A) -> Self {
        variant as u8 != 0
    }
}
impl INTB9_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INTB9_A {
        match self.bits {
            false => INTB9_A::NOT_ACTIVE,
            true => INTB9_A::ACTIVE,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_ACTIVE`"]
    #[inline(always)]
    pub fn is_not_active(&self) -> bool {
        *self == INTB9_A::NOT_ACTIVE
    }
    #[doc = "Checks if the value of the field is `ACTIVE`"]
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        *self == INTB9_A::ACTIVE
    }
}
#[doc = "Field `INTB9` writer - Interrupt B status for DMA channel."]
pub type INTB9_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTB0_SPEC, INTB9_A, O>;
impl<'a, const O: u8> INTB9_W<'a, O> {
    #[doc = "The DMA channel interrupt B is not active."]
    #[inline(always)]
    pub fn not_active(self) -> &'a mut W {
        self.variant(INTB9_A::NOT_ACTIVE)
    }
    #[doc = "The DMA channel interrupt B is active."]
    #[inline(always)]
    pub fn active(self) -> &'a mut W {
        self.variant(INTB9_A::ACTIVE)
    }
}
#[doc = "Field `INTB10` reader - Interrupt B status for DMA channel."]
pub type INTB10_R = crate::BitReader<INTB10_A>;
#[doc = "Interrupt B status for DMA channel.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum INTB10_A {
    #[doc = "0: The DMA channel interrupt B is not active."]
    NOT_ACTIVE = 0,
    #[doc = "1: The DMA channel interrupt B is active."]
    ACTIVE = 1,
}
impl From<INTB10_A> for bool {
    #[inline(always)]
    fn from(variant: INTB10_A) -> Self {
        variant as u8 != 0
    }
}
impl INTB10_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INTB10_A {
        match self.bits {
            false => INTB10_A::NOT_ACTIVE,
            true => INTB10_A::ACTIVE,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_ACTIVE`"]
    #[inline(always)]
    pub fn is_not_active(&self) -> bool {
        *self == INTB10_A::NOT_ACTIVE
    }
    #[doc = "Checks if the value of the field is `ACTIVE`"]
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        *self == INTB10_A::ACTIVE
    }
}
#[doc = "Field `INTB10` writer - Interrupt B status for DMA channel."]
pub type INTB10_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTB0_SPEC, INTB10_A, O>;
impl<'a, const O: u8> INTB10_W<'a, O> {
    #[doc = "The DMA channel interrupt B is not active."]
    #[inline(always)]
    pub fn not_active(self) -> &'a mut W {
        self.variant(INTB10_A::NOT_ACTIVE)
    }
    #[doc = "The DMA channel interrupt B is active."]
    #[inline(always)]
    pub fn active(self) -> &'a mut W {
        self.variant(INTB10_A::ACTIVE)
    }
}
#[doc = "Field `INTB11` reader - Interrupt B status for DMA channel."]
pub type INTB11_R = crate::BitReader<INTB11_A>;
#[doc = "Interrupt B status for DMA channel.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum INTB11_A {
    #[doc = "0: The DMA channel interrupt B is not active."]
    NOT_ACTIVE = 0,
    #[doc = "1: The DMA channel interrupt B is active."]
    ACTIVE = 1,
}
impl From<INTB11_A> for bool {
    #[inline(always)]
    fn from(variant: INTB11_A) -> Self {
        variant as u8 != 0
    }
}
impl INTB11_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INTB11_A {
        match self.bits {
            false => INTB11_A::NOT_ACTIVE,
            true => INTB11_A::ACTIVE,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_ACTIVE`"]
    #[inline(always)]
    pub fn is_not_active(&self) -> bool {
        *self == INTB11_A::NOT_ACTIVE
    }
    #[doc = "Checks if the value of the field is `ACTIVE`"]
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        *self == INTB11_A::ACTIVE
    }
}
#[doc = "Field `INTB11` writer - Interrupt B status for DMA channel."]
pub type INTB11_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTB0_SPEC, INTB11_A, O>;
impl<'a, const O: u8> INTB11_W<'a, O> {
    #[doc = "The DMA channel interrupt B is not active."]
    #[inline(always)]
    pub fn not_active(self) -> &'a mut W {
        self.variant(INTB11_A::NOT_ACTIVE)
    }
    #[doc = "The DMA channel interrupt B is active."]
    #[inline(always)]
    pub fn active(self) -> &'a mut W {
        self.variant(INTB11_A::ACTIVE)
    }
}
#[doc = "Field `INTB12` reader - Interrupt B status for DMA channel."]
pub type INTB12_R = crate::BitReader<INTB12_A>;
#[doc = "Interrupt B status for DMA channel.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum INTB12_A {
    #[doc = "0: The DMA channel interrupt B is not active."]
    NOT_ACTIVE = 0,
    #[doc = "1: The DMA channel interrupt B is active."]
    ACTIVE = 1,
}
impl From<INTB12_A> for bool {
    #[inline(always)]
    fn from(variant: INTB12_A) -> Self {
        variant as u8 != 0
    }
}
impl INTB12_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INTB12_A {
        match self.bits {
            false => INTB12_A::NOT_ACTIVE,
            true => INTB12_A::ACTIVE,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_ACTIVE`"]
    #[inline(always)]
    pub fn is_not_active(&self) -> bool {
        *self == INTB12_A::NOT_ACTIVE
    }
    #[doc = "Checks if the value of the field is `ACTIVE`"]
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        *self == INTB12_A::ACTIVE
    }
}
#[doc = "Field `INTB12` writer - Interrupt B status for DMA channel."]
pub type INTB12_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTB0_SPEC, INTB12_A, O>;
impl<'a, const O: u8> INTB12_W<'a, O> {
    #[doc = "The DMA channel interrupt B is not active."]
    #[inline(always)]
    pub fn not_active(self) -> &'a mut W {
        self.variant(INTB12_A::NOT_ACTIVE)
    }
    #[doc = "The DMA channel interrupt B is active."]
    #[inline(always)]
    pub fn active(self) -> &'a mut W {
        self.variant(INTB12_A::ACTIVE)
    }
}
#[doc = "Field `INTB13` reader - Interrupt B status for DMA channel."]
pub type INTB13_R = crate::BitReader<INTB13_A>;
#[doc = "Interrupt B status for DMA channel.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum INTB13_A {
    #[doc = "0: The DMA channel interrupt B is not active."]
    NOT_ACTIVE = 0,
    #[doc = "1: The DMA channel interrupt B is active."]
    ACTIVE = 1,
}
impl From<INTB13_A> for bool {
    #[inline(always)]
    fn from(variant: INTB13_A) -> Self {
        variant as u8 != 0
    }
}
impl INTB13_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INTB13_A {
        match self.bits {
            false => INTB13_A::NOT_ACTIVE,
            true => INTB13_A::ACTIVE,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_ACTIVE`"]
    #[inline(always)]
    pub fn is_not_active(&self) -> bool {
        *self == INTB13_A::NOT_ACTIVE
    }
    #[doc = "Checks if the value of the field is `ACTIVE`"]
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        *self == INTB13_A::ACTIVE
    }
}
#[doc = "Field `INTB13` writer - Interrupt B status for DMA channel."]
pub type INTB13_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTB0_SPEC, INTB13_A, O>;
impl<'a, const O: u8> INTB13_W<'a, O> {
    #[doc = "The DMA channel interrupt B is not active."]
    #[inline(always)]
    pub fn not_active(self) -> &'a mut W {
        self.variant(INTB13_A::NOT_ACTIVE)
    }
    #[doc = "The DMA channel interrupt B is active."]
    #[inline(always)]
    pub fn active(self) -> &'a mut W {
        self.variant(INTB13_A::ACTIVE)
    }
}
#[doc = "Field `INTB14` reader - Interrupt B status for DMA channel."]
pub type INTB14_R = crate::BitReader<INTB14_A>;
#[doc = "Interrupt B status for DMA channel.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum INTB14_A {
    #[doc = "0: The DMA channel interrupt B is not active."]
    NOT_ACTIVE = 0,
    #[doc = "1: The DMA channel interrupt B is active."]
    ACTIVE = 1,
}
impl From<INTB14_A> for bool {
    #[inline(always)]
    fn from(variant: INTB14_A) -> Self {
        variant as u8 != 0
    }
}
impl INTB14_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INTB14_A {
        match self.bits {
            false => INTB14_A::NOT_ACTIVE,
            true => INTB14_A::ACTIVE,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_ACTIVE`"]
    #[inline(always)]
    pub fn is_not_active(&self) -> bool {
        *self == INTB14_A::NOT_ACTIVE
    }
    #[doc = "Checks if the value of the field is `ACTIVE`"]
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        *self == INTB14_A::ACTIVE
    }
}
#[doc = "Field `INTB14` writer - Interrupt B status for DMA channel."]
pub type INTB14_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTB0_SPEC, INTB14_A, O>;
impl<'a, const O: u8> INTB14_W<'a, O> {
    #[doc = "The DMA channel interrupt B is not active."]
    #[inline(always)]
    pub fn not_active(self) -> &'a mut W {
        self.variant(INTB14_A::NOT_ACTIVE)
    }
    #[doc = "The DMA channel interrupt B is active."]
    #[inline(always)]
    pub fn active(self) -> &'a mut W {
        self.variant(INTB14_A::ACTIVE)
    }
}
#[doc = "Field `INTB15` reader - Interrupt B status for DMA channel."]
pub type INTB15_R = crate::BitReader<INTB15_A>;
#[doc = "Interrupt B status for DMA channel.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum INTB15_A {
    #[doc = "0: The DMA channel interrupt B is not active."]
    NOT_ACTIVE = 0,
    #[doc = "1: The DMA channel interrupt B is active."]
    ACTIVE = 1,
}
impl From<INTB15_A> for bool {
    #[inline(always)]
    fn from(variant: INTB15_A) -> Self {
        variant as u8 != 0
    }
}
impl INTB15_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INTB15_A {
        match self.bits {
            false => INTB15_A::NOT_ACTIVE,
            true => INTB15_A::ACTIVE,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_ACTIVE`"]
    #[inline(always)]
    pub fn is_not_active(&self) -> bool {
        *self == INTB15_A::NOT_ACTIVE
    }
    #[doc = "Checks if the value of the field is `ACTIVE`"]
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        *self == INTB15_A::ACTIVE
    }
}
#[doc = "Field `INTB15` writer - Interrupt B status for DMA channel."]
pub type INTB15_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTB0_SPEC, INTB15_A, O>;
impl<'a, const O: u8> INTB15_W<'a, O> {
    #[doc = "The DMA channel interrupt B is not active."]
    #[inline(always)]
    pub fn not_active(self) -> &'a mut W {
        self.variant(INTB15_A::NOT_ACTIVE)
    }
    #[doc = "The DMA channel interrupt B is active."]
    #[inline(always)]
    pub fn active(self) -> &'a mut W {
        self.variant(INTB15_A::ACTIVE)
    }
}
#[doc = "Field `INTB16` reader - Interrupt B status for DMA channel."]
pub type INTB16_R = crate::BitReader<INTB16_A>;
#[doc = "Interrupt B status for DMA channel.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum INTB16_A {
    #[doc = "0: The DMA channel interrupt B is not active."]
    NOT_ACTIVE = 0,
    #[doc = "1: The DMA channel interrupt B is active."]
    ACTIVE = 1,
}
impl From<INTB16_A> for bool {
    #[inline(always)]
    fn from(variant: INTB16_A) -> Self {
        variant as u8 != 0
    }
}
impl INTB16_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INTB16_A {
        match self.bits {
            false => INTB16_A::NOT_ACTIVE,
            true => INTB16_A::ACTIVE,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_ACTIVE`"]
    #[inline(always)]
    pub fn is_not_active(&self) -> bool {
        *self == INTB16_A::NOT_ACTIVE
    }
    #[doc = "Checks if the value of the field is `ACTIVE`"]
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        *self == INTB16_A::ACTIVE
    }
}
#[doc = "Field `INTB16` writer - Interrupt B status for DMA channel."]
pub type INTB16_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTB0_SPEC, INTB16_A, O>;
impl<'a, const O: u8> INTB16_W<'a, O> {
    #[doc = "The DMA channel interrupt B is not active."]
    #[inline(always)]
    pub fn not_active(self) -> &'a mut W {
        self.variant(INTB16_A::NOT_ACTIVE)
    }
    #[doc = "The DMA channel interrupt B is active."]
    #[inline(always)]
    pub fn active(self) -> &'a mut W {
        self.variant(INTB16_A::ACTIVE)
    }
}
#[doc = "Field `INTB17` reader - Interrupt B status for DMA channel."]
pub type INTB17_R = crate::BitReader<INTB17_A>;
#[doc = "Interrupt B status for DMA channel.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum INTB17_A {
    #[doc = "0: The DMA channel interrupt B is not active."]
    NOT_ACTIVE = 0,
    #[doc = "1: The DMA channel interrupt B is active."]
    ACTIVE = 1,
}
impl From<INTB17_A> for bool {
    #[inline(always)]
    fn from(variant: INTB17_A) -> Self {
        variant as u8 != 0
    }
}
impl INTB17_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INTB17_A {
        match self.bits {
            false => INTB17_A::NOT_ACTIVE,
            true => INTB17_A::ACTIVE,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_ACTIVE`"]
    #[inline(always)]
    pub fn is_not_active(&self) -> bool {
        *self == INTB17_A::NOT_ACTIVE
    }
    #[doc = "Checks if the value of the field is `ACTIVE`"]
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        *self == INTB17_A::ACTIVE
    }
}
#[doc = "Field `INTB17` writer - Interrupt B status for DMA channel."]
pub type INTB17_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTB0_SPEC, INTB17_A, O>;
impl<'a, const O: u8> INTB17_W<'a, O> {
    #[doc = "The DMA channel interrupt B is not active."]
    #[inline(always)]
    pub fn not_active(self) -> &'a mut W {
        self.variant(INTB17_A::NOT_ACTIVE)
    }
    #[doc = "The DMA channel interrupt B is active."]
    #[inline(always)]
    pub fn active(self) -> &'a mut W {
        self.variant(INTB17_A::ACTIVE)
    }
}
#[doc = "Field `INTB18` reader - Interrupt B status for DMA channel."]
pub type INTB18_R = crate::BitReader<INTB18_A>;
#[doc = "Interrupt B status for DMA channel.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum INTB18_A {
    #[doc = "0: The DMA channel interrupt B is not active."]
    NOT_ACTIVE = 0,
    #[doc = "1: The DMA channel interrupt B is active."]
    ACTIVE = 1,
}
impl From<INTB18_A> for bool {
    #[inline(always)]
    fn from(variant: INTB18_A) -> Self {
        variant as u8 != 0
    }
}
impl INTB18_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INTB18_A {
        match self.bits {
            false => INTB18_A::NOT_ACTIVE,
            true => INTB18_A::ACTIVE,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_ACTIVE`"]
    #[inline(always)]
    pub fn is_not_active(&self) -> bool {
        *self == INTB18_A::NOT_ACTIVE
    }
    #[doc = "Checks if the value of the field is `ACTIVE`"]
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        *self == INTB18_A::ACTIVE
    }
}
#[doc = "Field `INTB18` writer - Interrupt B status for DMA channel."]
pub type INTB18_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTB0_SPEC, INTB18_A, O>;
impl<'a, const O: u8> INTB18_W<'a, O> {
    #[doc = "The DMA channel interrupt B is not active."]
    #[inline(always)]
    pub fn not_active(self) -> &'a mut W {
        self.variant(INTB18_A::NOT_ACTIVE)
    }
    #[doc = "The DMA channel interrupt B is active."]
    #[inline(always)]
    pub fn active(self) -> &'a mut W {
        self.variant(INTB18_A::ACTIVE)
    }
}
#[doc = "Field `INTB19` reader - Interrupt B status for DMA channel."]
pub type INTB19_R = crate::BitReader<INTB19_A>;
#[doc = "Interrupt B status for DMA channel.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum INTB19_A {
    #[doc = "0: The DMA channel interrupt B is not active."]
    NOT_ACTIVE = 0,
    #[doc = "1: The DMA channel interrupt B is active."]
    ACTIVE = 1,
}
impl From<INTB19_A> for bool {
    #[inline(always)]
    fn from(variant: INTB19_A) -> Self {
        variant as u8 != 0
    }
}
impl INTB19_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INTB19_A {
        match self.bits {
            false => INTB19_A::NOT_ACTIVE,
            true => INTB19_A::ACTIVE,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_ACTIVE`"]
    #[inline(always)]
    pub fn is_not_active(&self) -> bool {
        *self == INTB19_A::NOT_ACTIVE
    }
    #[doc = "Checks if the value of the field is `ACTIVE`"]
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        *self == INTB19_A::ACTIVE
    }
}
#[doc = "Field `INTB19` writer - Interrupt B status for DMA channel."]
pub type INTB19_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTB0_SPEC, INTB19_A, O>;
impl<'a, const O: u8> INTB19_W<'a, O> {
    #[doc = "The DMA channel interrupt B is not active."]
    #[inline(always)]
    pub fn not_active(self) -> &'a mut W {
        self.variant(INTB19_A::NOT_ACTIVE)
    }
    #[doc = "The DMA channel interrupt B is active."]
    #[inline(always)]
    pub fn active(self) -> &'a mut W {
        self.variant(INTB19_A::ACTIVE)
    }
}
#[doc = "Field `INTB20` reader - Interrupt B status for DMA channel."]
pub type INTB20_R = crate::BitReader<INTB20_A>;
#[doc = "Interrupt B status for DMA channel.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum INTB20_A {
    #[doc = "0: The DMA channel interrupt B is not active."]
    NOT_ACTIVE = 0,
    #[doc = "1: The DMA channel interrupt B is active."]
    ACTIVE = 1,
}
impl From<INTB20_A> for bool {
    #[inline(always)]
    fn from(variant: INTB20_A) -> Self {
        variant as u8 != 0
    }
}
impl INTB20_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INTB20_A {
        match self.bits {
            false => INTB20_A::NOT_ACTIVE,
            true => INTB20_A::ACTIVE,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_ACTIVE`"]
    #[inline(always)]
    pub fn is_not_active(&self) -> bool {
        *self == INTB20_A::NOT_ACTIVE
    }
    #[doc = "Checks if the value of the field is `ACTIVE`"]
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        *self == INTB20_A::ACTIVE
    }
}
#[doc = "Field `INTB20` writer - Interrupt B status for DMA channel."]
pub type INTB20_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTB0_SPEC, INTB20_A, O>;
impl<'a, const O: u8> INTB20_W<'a, O> {
    #[doc = "The DMA channel interrupt B is not active."]
    #[inline(always)]
    pub fn not_active(self) -> &'a mut W {
        self.variant(INTB20_A::NOT_ACTIVE)
    }
    #[doc = "The DMA channel interrupt B is active."]
    #[inline(always)]
    pub fn active(self) -> &'a mut W {
        self.variant(INTB20_A::ACTIVE)
    }
}
#[doc = "Field `INTB21` reader - Interrupt B status for DMA channel."]
pub type INTB21_R = crate::BitReader<INTB21_A>;
#[doc = "Interrupt B status for DMA channel.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum INTB21_A {
    #[doc = "0: The DMA channel interrupt B is not active."]
    NOT_ACTIVE = 0,
    #[doc = "1: The DMA channel interrupt B is active."]
    ACTIVE = 1,
}
impl From<INTB21_A> for bool {
    #[inline(always)]
    fn from(variant: INTB21_A) -> Self {
        variant as u8 != 0
    }
}
impl INTB21_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INTB21_A {
        match self.bits {
            false => INTB21_A::NOT_ACTIVE,
            true => INTB21_A::ACTIVE,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_ACTIVE`"]
    #[inline(always)]
    pub fn is_not_active(&self) -> bool {
        *self == INTB21_A::NOT_ACTIVE
    }
    #[doc = "Checks if the value of the field is `ACTIVE`"]
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        *self == INTB21_A::ACTIVE
    }
}
#[doc = "Field `INTB21` writer - Interrupt B status for DMA channel."]
pub type INTB21_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTB0_SPEC, INTB21_A, O>;
impl<'a, const O: u8> INTB21_W<'a, O> {
    #[doc = "The DMA channel interrupt B is not active."]
    #[inline(always)]
    pub fn not_active(self) -> &'a mut W {
        self.variant(INTB21_A::NOT_ACTIVE)
    }
    #[doc = "The DMA channel interrupt B is active."]
    #[inline(always)]
    pub fn active(self) -> &'a mut W {
        self.variant(INTB21_A::ACTIVE)
    }
}
#[doc = "Field `INTB22` reader - Interrupt B status for DMA channel."]
pub type INTB22_R = crate::BitReader<INTB22_A>;
#[doc = "Interrupt B status for DMA channel.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum INTB22_A {
    #[doc = "0: The DMA channel interrupt B is not active."]
    NOT_ACTIVE = 0,
    #[doc = "1: The DMA channel interrupt B is active."]
    ACTIVE = 1,
}
impl From<INTB22_A> for bool {
    #[inline(always)]
    fn from(variant: INTB22_A) -> Self {
        variant as u8 != 0
    }
}
impl INTB22_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INTB22_A {
        match self.bits {
            false => INTB22_A::NOT_ACTIVE,
            true => INTB22_A::ACTIVE,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_ACTIVE`"]
    #[inline(always)]
    pub fn is_not_active(&self) -> bool {
        *self == INTB22_A::NOT_ACTIVE
    }
    #[doc = "Checks if the value of the field is `ACTIVE`"]
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        *self == INTB22_A::ACTIVE
    }
}
#[doc = "Field `INTB22` writer - Interrupt B status for DMA channel."]
pub type INTB22_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTB0_SPEC, INTB22_A, O>;
impl<'a, const O: u8> INTB22_W<'a, O> {
    #[doc = "The DMA channel interrupt B is not active."]
    #[inline(always)]
    pub fn not_active(self) -> &'a mut W {
        self.variant(INTB22_A::NOT_ACTIVE)
    }
    #[doc = "The DMA channel interrupt B is active."]
    #[inline(always)]
    pub fn active(self) -> &'a mut W {
        self.variant(INTB22_A::ACTIVE)
    }
}
#[doc = "Field `INTB23` reader - Interrupt B status for DMA channel."]
pub type INTB23_R = crate::BitReader<INTB23_A>;
#[doc = "Interrupt B status for DMA channel.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum INTB23_A {
    #[doc = "0: The DMA channel interrupt B is not active."]
    NOT_ACTIVE = 0,
    #[doc = "1: The DMA channel interrupt B is active."]
    ACTIVE = 1,
}
impl From<INTB23_A> for bool {
    #[inline(always)]
    fn from(variant: INTB23_A) -> Self {
        variant as u8 != 0
    }
}
impl INTB23_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INTB23_A {
        match self.bits {
            false => INTB23_A::NOT_ACTIVE,
            true => INTB23_A::ACTIVE,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_ACTIVE`"]
    #[inline(always)]
    pub fn is_not_active(&self) -> bool {
        *self == INTB23_A::NOT_ACTIVE
    }
    #[doc = "Checks if the value of the field is `ACTIVE`"]
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        *self == INTB23_A::ACTIVE
    }
}
#[doc = "Field `INTB23` writer - Interrupt B status for DMA channel."]
pub type INTB23_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTB0_SPEC, INTB23_A, O>;
impl<'a, const O: u8> INTB23_W<'a, O> {
    #[doc = "The DMA channel interrupt B is not active."]
    #[inline(always)]
    pub fn not_active(self) -> &'a mut W {
        self.variant(INTB23_A::NOT_ACTIVE)
    }
    #[doc = "The DMA channel interrupt B is active."]
    #[inline(always)]
    pub fn active(self) -> &'a mut W {
        self.variant(INTB23_A::ACTIVE)
    }
}
#[doc = "Field `INTB24` reader - Interrupt B status for DMA channel."]
pub type INTB24_R = crate::BitReader<INTB24_A>;
#[doc = "Interrupt B status for DMA channel.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum INTB24_A {
    #[doc = "0: The DMA channel interrupt B is not active."]
    NOT_ACTIVE = 0,
    #[doc = "1: The DMA channel interrupt B is active."]
    ACTIVE = 1,
}
impl From<INTB24_A> for bool {
    #[inline(always)]
    fn from(variant: INTB24_A) -> Self {
        variant as u8 != 0
    }
}
impl INTB24_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INTB24_A {
        match self.bits {
            false => INTB24_A::NOT_ACTIVE,
            true => INTB24_A::ACTIVE,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_ACTIVE`"]
    #[inline(always)]
    pub fn is_not_active(&self) -> bool {
        *self == INTB24_A::NOT_ACTIVE
    }
    #[doc = "Checks if the value of the field is `ACTIVE`"]
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        *self == INTB24_A::ACTIVE
    }
}
#[doc = "Field `INTB24` writer - Interrupt B status for DMA channel."]
pub type INTB24_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTB0_SPEC, INTB24_A, O>;
impl<'a, const O: u8> INTB24_W<'a, O> {
    #[doc = "The DMA channel interrupt B is not active."]
    #[inline(always)]
    pub fn not_active(self) -> &'a mut W {
        self.variant(INTB24_A::NOT_ACTIVE)
    }
    #[doc = "The DMA channel interrupt B is active."]
    #[inline(always)]
    pub fn active(self) -> &'a mut W {
        self.variant(INTB24_A::ACTIVE)
    }
}
#[doc = "Field `INTB25` reader - Interrupt B status for DMA channel."]
pub type INTB25_R = crate::BitReader<INTB25_A>;
#[doc = "Interrupt B status for DMA channel.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum INTB25_A {
    #[doc = "0: The DMA channel interrupt B is not active."]
    NOT_ACTIVE = 0,
    #[doc = "1: The DMA channel interrupt B is active."]
    ACTIVE = 1,
}
impl From<INTB25_A> for bool {
    #[inline(always)]
    fn from(variant: INTB25_A) -> Self {
        variant as u8 != 0
    }
}
impl INTB25_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INTB25_A {
        match self.bits {
            false => INTB25_A::NOT_ACTIVE,
            true => INTB25_A::ACTIVE,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_ACTIVE`"]
    #[inline(always)]
    pub fn is_not_active(&self) -> bool {
        *self == INTB25_A::NOT_ACTIVE
    }
    #[doc = "Checks if the value of the field is `ACTIVE`"]
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        *self == INTB25_A::ACTIVE
    }
}
#[doc = "Field `INTB25` writer - Interrupt B status for DMA channel."]
pub type INTB25_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTB0_SPEC, INTB25_A, O>;
impl<'a, const O: u8> INTB25_W<'a, O> {
    #[doc = "The DMA channel interrupt B is not active."]
    #[inline(always)]
    pub fn not_active(self) -> &'a mut W {
        self.variant(INTB25_A::NOT_ACTIVE)
    }
    #[doc = "The DMA channel interrupt B is active."]
    #[inline(always)]
    pub fn active(self) -> &'a mut W {
        self.variant(INTB25_A::ACTIVE)
    }
}
#[doc = "Field `INTB26` reader - Interrupt B status for DMA channel."]
pub type INTB26_R = crate::BitReader<INTB26_A>;
#[doc = "Interrupt B status for DMA channel.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum INTB26_A {
    #[doc = "0: The DMA channel interrupt B is not active."]
    NOT_ACTIVE = 0,
    #[doc = "1: The DMA channel interrupt B is active."]
    ACTIVE = 1,
}
impl From<INTB26_A> for bool {
    #[inline(always)]
    fn from(variant: INTB26_A) -> Self {
        variant as u8 != 0
    }
}
impl INTB26_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INTB26_A {
        match self.bits {
            false => INTB26_A::NOT_ACTIVE,
            true => INTB26_A::ACTIVE,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_ACTIVE`"]
    #[inline(always)]
    pub fn is_not_active(&self) -> bool {
        *self == INTB26_A::NOT_ACTIVE
    }
    #[doc = "Checks if the value of the field is `ACTIVE`"]
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        *self == INTB26_A::ACTIVE
    }
}
#[doc = "Field `INTB26` writer - Interrupt B status for DMA channel."]
pub type INTB26_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTB0_SPEC, INTB26_A, O>;
impl<'a, const O: u8> INTB26_W<'a, O> {
    #[doc = "The DMA channel interrupt B is not active."]
    #[inline(always)]
    pub fn not_active(self) -> &'a mut W {
        self.variant(INTB26_A::NOT_ACTIVE)
    }
    #[doc = "The DMA channel interrupt B is active."]
    #[inline(always)]
    pub fn active(self) -> &'a mut W {
        self.variant(INTB26_A::ACTIVE)
    }
}
#[doc = "Field `INTB27` reader - Interrupt B status for DMA channel."]
pub type INTB27_R = crate::BitReader<INTB27_A>;
#[doc = "Interrupt B status for DMA channel.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum INTB27_A {
    #[doc = "0: The DMA channel interrupt B is not active."]
    NOT_ACTIVE = 0,
    #[doc = "1: The DMA channel interrupt B is active."]
    ACTIVE = 1,
}
impl From<INTB27_A> for bool {
    #[inline(always)]
    fn from(variant: INTB27_A) -> Self {
        variant as u8 != 0
    }
}
impl INTB27_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INTB27_A {
        match self.bits {
            false => INTB27_A::NOT_ACTIVE,
            true => INTB27_A::ACTIVE,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_ACTIVE`"]
    #[inline(always)]
    pub fn is_not_active(&self) -> bool {
        *self == INTB27_A::NOT_ACTIVE
    }
    #[doc = "Checks if the value of the field is `ACTIVE`"]
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        *self == INTB27_A::ACTIVE
    }
}
#[doc = "Field `INTB27` writer - Interrupt B status for DMA channel."]
pub type INTB27_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTB0_SPEC, INTB27_A, O>;
impl<'a, const O: u8> INTB27_W<'a, O> {
    #[doc = "The DMA channel interrupt B is not active."]
    #[inline(always)]
    pub fn not_active(self) -> &'a mut W {
        self.variant(INTB27_A::NOT_ACTIVE)
    }
    #[doc = "The DMA channel interrupt B is active."]
    #[inline(always)]
    pub fn active(self) -> &'a mut W {
        self.variant(INTB27_A::ACTIVE)
    }
}
#[doc = "Field `INTB28` reader - Interrupt B status for DMA channel."]
pub type INTB28_R = crate::BitReader<INTB28_A>;
#[doc = "Interrupt B status for DMA channel.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum INTB28_A {
    #[doc = "0: The DMA channel interrupt B is not active."]
    NOT_ACTIVE = 0,
    #[doc = "1: The DMA channel interrupt B is active."]
    ACTIVE = 1,
}
impl From<INTB28_A> for bool {
    #[inline(always)]
    fn from(variant: INTB28_A) -> Self {
        variant as u8 != 0
    }
}
impl INTB28_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INTB28_A {
        match self.bits {
            false => INTB28_A::NOT_ACTIVE,
            true => INTB28_A::ACTIVE,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_ACTIVE`"]
    #[inline(always)]
    pub fn is_not_active(&self) -> bool {
        *self == INTB28_A::NOT_ACTIVE
    }
    #[doc = "Checks if the value of the field is `ACTIVE`"]
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        *self == INTB28_A::ACTIVE
    }
}
#[doc = "Field `INTB28` writer - Interrupt B status for DMA channel."]
pub type INTB28_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTB0_SPEC, INTB28_A, O>;
impl<'a, const O: u8> INTB28_W<'a, O> {
    #[doc = "The DMA channel interrupt B is not active."]
    #[inline(always)]
    pub fn not_active(self) -> &'a mut W {
        self.variant(INTB28_A::NOT_ACTIVE)
    }
    #[doc = "The DMA channel interrupt B is active."]
    #[inline(always)]
    pub fn active(self) -> &'a mut W {
        self.variant(INTB28_A::ACTIVE)
    }
}
#[doc = "Field `INTB29` reader - Interrupt B status for DMA channel."]
pub type INTB29_R = crate::BitReader<INTB29_A>;
#[doc = "Interrupt B status for DMA channel.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum INTB29_A {
    #[doc = "0: The DMA channel interrupt B is not active."]
    NOT_ACTIVE = 0,
    #[doc = "1: The DMA channel interrupt B is active."]
    ACTIVE = 1,
}
impl From<INTB29_A> for bool {
    #[inline(always)]
    fn from(variant: INTB29_A) -> Self {
        variant as u8 != 0
    }
}
impl INTB29_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INTB29_A {
        match self.bits {
            false => INTB29_A::NOT_ACTIVE,
            true => INTB29_A::ACTIVE,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_ACTIVE`"]
    #[inline(always)]
    pub fn is_not_active(&self) -> bool {
        *self == INTB29_A::NOT_ACTIVE
    }
    #[doc = "Checks if the value of the field is `ACTIVE`"]
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        *self == INTB29_A::ACTIVE
    }
}
#[doc = "Field `INTB29` writer - Interrupt B status for DMA channel."]
pub type INTB29_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTB0_SPEC, INTB29_A, O>;
impl<'a, const O: u8> INTB29_W<'a, O> {
    #[doc = "The DMA channel interrupt B is not active."]
    #[inline(always)]
    pub fn not_active(self) -> &'a mut W {
        self.variant(INTB29_A::NOT_ACTIVE)
    }
    #[doc = "The DMA channel interrupt B is active."]
    #[inline(always)]
    pub fn active(self) -> &'a mut W {
        self.variant(INTB29_A::ACTIVE)
    }
}
#[doc = "Field `INTB30` reader - Interrupt B status for DMA channel."]
pub type INTB30_R = crate::BitReader<INTB30_A>;
#[doc = "Interrupt B status for DMA channel.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum INTB30_A {
    #[doc = "0: The DMA channel interrupt B is not active."]
    NOT_ACTIVE = 0,
    #[doc = "1: The DMA channel interrupt B is active."]
    ACTIVE = 1,
}
impl From<INTB30_A> for bool {
    #[inline(always)]
    fn from(variant: INTB30_A) -> Self {
        variant as u8 != 0
    }
}
impl INTB30_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INTB30_A {
        match self.bits {
            false => INTB30_A::NOT_ACTIVE,
            true => INTB30_A::ACTIVE,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_ACTIVE`"]
    #[inline(always)]
    pub fn is_not_active(&self) -> bool {
        *self == INTB30_A::NOT_ACTIVE
    }
    #[doc = "Checks if the value of the field is `ACTIVE`"]
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        *self == INTB30_A::ACTIVE
    }
}
#[doc = "Field `INTB30` writer - Interrupt B status for DMA channel."]
pub type INTB30_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTB0_SPEC, INTB30_A, O>;
impl<'a, const O: u8> INTB30_W<'a, O> {
    #[doc = "The DMA channel interrupt B is not active."]
    #[inline(always)]
    pub fn not_active(self) -> &'a mut W {
        self.variant(INTB30_A::NOT_ACTIVE)
    }
    #[doc = "The DMA channel interrupt B is active."]
    #[inline(always)]
    pub fn active(self) -> &'a mut W {
        self.variant(INTB30_A::ACTIVE)
    }
}
#[doc = "Field `INTB31` reader - Interrupt B status for DMA channel."]
pub type INTB31_R = crate::BitReader<INTB31_A>;
#[doc = "Interrupt B status for DMA channel.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum INTB31_A {
    #[doc = "0: The DMA channel interrupt B is not active."]
    NOT_ACTIVE = 0,
    #[doc = "1: The DMA channel interrupt B is active."]
    ACTIVE = 1,
}
impl From<INTB31_A> for bool {
    #[inline(always)]
    fn from(variant: INTB31_A) -> Self {
        variant as u8 != 0
    }
}
impl INTB31_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INTB31_A {
        match self.bits {
            false => INTB31_A::NOT_ACTIVE,
            true => INTB31_A::ACTIVE,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_ACTIVE`"]
    #[inline(always)]
    pub fn is_not_active(&self) -> bool {
        *self == INTB31_A::NOT_ACTIVE
    }
    #[doc = "Checks if the value of the field is `ACTIVE`"]
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        *self == INTB31_A::ACTIVE
    }
}
#[doc = "Field `INTB31` writer - Interrupt B status for DMA channel."]
pub type INTB31_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTB0_SPEC, INTB31_A, O>;
impl<'a, const O: u8> INTB31_W<'a, O> {
    #[doc = "The DMA channel interrupt B is not active."]
    #[inline(always)]
    pub fn not_active(self) -> &'a mut W {
        self.variant(INTB31_A::NOT_ACTIVE)
    }
    #[doc = "The DMA channel interrupt B is active."]
    #[inline(always)]
    pub fn active(self) -> &'a mut W {
        self.variant(INTB31_A::ACTIVE)
    }
}
impl R {
    #[doc = "Bit 0 - Interrupt B status for DMA channel."]
    #[inline(always)]
    pub fn intb0(&self) -> INTB0_R {
        INTB0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Interrupt B status for DMA channel."]
    #[inline(always)]
    pub fn intb1(&self) -> INTB1_R {
        INTB1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Interrupt B status for DMA channel."]
    #[inline(always)]
    pub fn intb2(&self) -> INTB2_R {
        INTB2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Interrupt B status for DMA channel."]
    #[inline(always)]
    pub fn intb3(&self) -> INTB3_R {
        INTB3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Interrupt B status for DMA channel."]
    #[inline(always)]
    pub fn intb4(&self) -> INTB4_R {
        INTB4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Interrupt B status for DMA channel."]
    #[inline(always)]
    pub fn intb5(&self) -> INTB5_R {
        INTB5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Interrupt B status for DMA channel."]
    #[inline(always)]
    pub fn intb6(&self) -> INTB6_R {
        INTB6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Interrupt B status for DMA channel."]
    #[inline(always)]
    pub fn intb7(&self) -> INTB7_R {
        INTB7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Interrupt B status for DMA channel."]
    #[inline(always)]
    pub fn intb8(&self) -> INTB8_R {
        INTB8_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Interrupt B status for DMA channel."]
    #[inline(always)]
    pub fn intb9(&self) -> INTB9_R {
        INTB9_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Interrupt B status for DMA channel."]
    #[inline(always)]
    pub fn intb10(&self) -> INTB10_R {
        INTB10_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Interrupt B status for DMA channel."]
    #[inline(always)]
    pub fn intb11(&self) -> INTB11_R {
        INTB11_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Interrupt B status for DMA channel."]
    #[inline(always)]
    pub fn intb12(&self) -> INTB12_R {
        INTB12_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Interrupt B status for DMA channel."]
    #[inline(always)]
    pub fn intb13(&self) -> INTB13_R {
        INTB13_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Interrupt B status for DMA channel."]
    #[inline(always)]
    pub fn intb14(&self) -> INTB14_R {
        INTB14_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Interrupt B status for DMA channel."]
    #[inline(always)]
    pub fn intb15(&self) -> INTB15_R {
        INTB15_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Interrupt B status for DMA channel."]
    #[inline(always)]
    pub fn intb16(&self) -> INTB16_R {
        INTB16_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Interrupt B status for DMA channel."]
    #[inline(always)]
    pub fn intb17(&self) -> INTB17_R {
        INTB17_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Interrupt B status for DMA channel."]
    #[inline(always)]
    pub fn intb18(&self) -> INTB18_R {
        INTB18_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Interrupt B status for DMA channel."]
    #[inline(always)]
    pub fn intb19(&self) -> INTB19_R {
        INTB19_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Interrupt B status for DMA channel."]
    #[inline(always)]
    pub fn intb20(&self) -> INTB20_R {
        INTB20_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Interrupt B status for DMA channel."]
    #[inline(always)]
    pub fn intb21(&self) -> INTB21_R {
        INTB21_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Interrupt B status for DMA channel."]
    #[inline(always)]
    pub fn intb22(&self) -> INTB22_R {
        INTB22_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Interrupt B status for DMA channel."]
    #[inline(always)]
    pub fn intb23(&self) -> INTB23_R {
        INTB23_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Interrupt B status for DMA channel."]
    #[inline(always)]
    pub fn intb24(&self) -> INTB24_R {
        INTB24_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Interrupt B status for DMA channel."]
    #[inline(always)]
    pub fn intb25(&self) -> INTB25_R {
        INTB25_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Interrupt B status for DMA channel."]
    #[inline(always)]
    pub fn intb26(&self) -> INTB26_R {
        INTB26_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Interrupt B status for DMA channel."]
    #[inline(always)]
    pub fn intb27(&self) -> INTB27_R {
        INTB27_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Interrupt B status for DMA channel."]
    #[inline(always)]
    pub fn intb28(&self) -> INTB28_R {
        INTB28_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Interrupt B status for DMA channel."]
    #[inline(always)]
    pub fn intb29(&self) -> INTB29_R {
        INTB29_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Interrupt B status for DMA channel."]
    #[inline(always)]
    pub fn intb30(&self) -> INTB30_R {
        INTB30_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Interrupt B status for DMA channel."]
    #[inline(always)]
    pub fn intb31(&self) -> INTB31_R {
        INTB31_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Interrupt B status for DMA channel."]
    #[inline(always)]
    #[must_use]
    pub fn intb0(&mut self) -> INTB0_W<0> {
        INTB0_W::new(self)
    }
    #[doc = "Bit 1 - Interrupt B status for DMA channel."]
    #[inline(always)]
    #[must_use]
    pub fn intb1(&mut self) -> INTB1_W<1> {
        INTB1_W::new(self)
    }
    #[doc = "Bit 2 - Interrupt B status for DMA channel."]
    #[inline(always)]
    #[must_use]
    pub fn intb2(&mut self) -> INTB2_W<2> {
        INTB2_W::new(self)
    }
    #[doc = "Bit 3 - Interrupt B status for DMA channel."]
    #[inline(always)]
    #[must_use]
    pub fn intb3(&mut self) -> INTB3_W<3> {
        INTB3_W::new(self)
    }
    #[doc = "Bit 4 - Interrupt B status for DMA channel."]
    #[inline(always)]
    #[must_use]
    pub fn intb4(&mut self) -> INTB4_W<4> {
        INTB4_W::new(self)
    }
    #[doc = "Bit 5 - Interrupt B status for DMA channel."]
    #[inline(always)]
    #[must_use]
    pub fn intb5(&mut self) -> INTB5_W<5> {
        INTB5_W::new(self)
    }
    #[doc = "Bit 6 - Interrupt B status for DMA channel."]
    #[inline(always)]
    #[must_use]
    pub fn intb6(&mut self) -> INTB6_W<6> {
        INTB6_W::new(self)
    }
    #[doc = "Bit 7 - Interrupt B status for DMA channel."]
    #[inline(always)]
    #[must_use]
    pub fn intb7(&mut self) -> INTB7_W<7> {
        INTB7_W::new(self)
    }
    #[doc = "Bit 8 - Interrupt B status for DMA channel."]
    #[inline(always)]
    #[must_use]
    pub fn intb8(&mut self) -> INTB8_W<8> {
        INTB8_W::new(self)
    }
    #[doc = "Bit 9 - Interrupt B status for DMA channel."]
    #[inline(always)]
    #[must_use]
    pub fn intb9(&mut self) -> INTB9_W<9> {
        INTB9_W::new(self)
    }
    #[doc = "Bit 10 - Interrupt B status for DMA channel."]
    #[inline(always)]
    #[must_use]
    pub fn intb10(&mut self) -> INTB10_W<10> {
        INTB10_W::new(self)
    }
    #[doc = "Bit 11 - Interrupt B status for DMA channel."]
    #[inline(always)]
    #[must_use]
    pub fn intb11(&mut self) -> INTB11_W<11> {
        INTB11_W::new(self)
    }
    #[doc = "Bit 12 - Interrupt B status for DMA channel."]
    #[inline(always)]
    #[must_use]
    pub fn intb12(&mut self) -> INTB12_W<12> {
        INTB12_W::new(self)
    }
    #[doc = "Bit 13 - Interrupt B status for DMA channel."]
    #[inline(always)]
    #[must_use]
    pub fn intb13(&mut self) -> INTB13_W<13> {
        INTB13_W::new(self)
    }
    #[doc = "Bit 14 - Interrupt B status for DMA channel."]
    #[inline(always)]
    #[must_use]
    pub fn intb14(&mut self) -> INTB14_W<14> {
        INTB14_W::new(self)
    }
    #[doc = "Bit 15 - Interrupt B status for DMA channel."]
    #[inline(always)]
    #[must_use]
    pub fn intb15(&mut self) -> INTB15_W<15> {
        INTB15_W::new(self)
    }
    #[doc = "Bit 16 - Interrupt B status for DMA channel."]
    #[inline(always)]
    #[must_use]
    pub fn intb16(&mut self) -> INTB16_W<16> {
        INTB16_W::new(self)
    }
    #[doc = "Bit 17 - Interrupt B status for DMA channel."]
    #[inline(always)]
    #[must_use]
    pub fn intb17(&mut self) -> INTB17_W<17> {
        INTB17_W::new(self)
    }
    #[doc = "Bit 18 - Interrupt B status for DMA channel."]
    #[inline(always)]
    #[must_use]
    pub fn intb18(&mut self) -> INTB18_W<18> {
        INTB18_W::new(self)
    }
    #[doc = "Bit 19 - Interrupt B status for DMA channel."]
    #[inline(always)]
    #[must_use]
    pub fn intb19(&mut self) -> INTB19_W<19> {
        INTB19_W::new(self)
    }
    #[doc = "Bit 20 - Interrupt B status for DMA channel."]
    #[inline(always)]
    #[must_use]
    pub fn intb20(&mut self) -> INTB20_W<20> {
        INTB20_W::new(self)
    }
    #[doc = "Bit 21 - Interrupt B status for DMA channel."]
    #[inline(always)]
    #[must_use]
    pub fn intb21(&mut self) -> INTB21_W<21> {
        INTB21_W::new(self)
    }
    #[doc = "Bit 22 - Interrupt B status for DMA channel."]
    #[inline(always)]
    #[must_use]
    pub fn intb22(&mut self) -> INTB22_W<22> {
        INTB22_W::new(self)
    }
    #[doc = "Bit 23 - Interrupt B status for DMA channel."]
    #[inline(always)]
    #[must_use]
    pub fn intb23(&mut self) -> INTB23_W<23> {
        INTB23_W::new(self)
    }
    #[doc = "Bit 24 - Interrupt B status for DMA channel."]
    #[inline(always)]
    #[must_use]
    pub fn intb24(&mut self) -> INTB24_W<24> {
        INTB24_W::new(self)
    }
    #[doc = "Bit 25 - Interrupt B status for DMA channel."]
    #[inline(always)]
    #[must_use]
    pub fn intb25(&mut self) -> INTB25_W<25> {
        INTB25_W::new(self)
    }
    #[doc = "Bit 26 - Interrupt B status for DMA channel."]
    #[inline(always)]
    #[must_use]
    pub fn intb26(&mut self) -> INTB26_W<26> {
        INTB26_W::new(self)
    }
    #[doc = "Bit 27 - Interrupt B status for DMA channel."]
    #[inline(always)]
    #[must_use]
    pub fn intb27(&mut self) -> INTB27_W<27> {
        INTB27_W::new(self)
    }
    #[doc = "Bit 28 - Interrupt B status for DMA channel."]
    #[inline(always)]
    #[must_use]
    pub fn intb28(&mut self) -> INTB28_W<28> {
        INTB28_W::new(self)
    }
    #[doc = "Bit 29 - Interrupt B status for DMA channel."]
    #[inline(always)]
    #[must_use]
    pub fn intb29(&mut self) -> INTB29_W<29> {
        INTB29_W::new(self)
    }
    #[doc = "Bit 30 - Interrupt B status for DMA channel."]
    #[inline(always)]
    #[must_use]
    pub fn intb30(&mut self) -> INTB30_W<30> {
        INTB30_W::new(self)
    }
    #[doc = "Bit 31 - Interrupt B status for DMA channel."]
    #[inline(always)]
    #[must_use]
    pub fn intb31(&mut self) -> INTB31_W<31> {
        INTB31_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt B status for all DMA channels\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intb0](index.html) module"]
pub struct INTB0_SPEC;
impl crate::RegisterSpec for INTB0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [intb0::R](R) reader structure"]
impl crate::Readable for INTB0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [intb0::W](W) writer structure"]
impl crate::Writable for INTB0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets INTB0 to value 0"]
impl crate::Resettable for INTB0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
