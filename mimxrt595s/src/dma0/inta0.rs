#[doc = "Register `INTA0` reader"]
pub struct R(crate::R<INTA0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INTA0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INTA0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INTA0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `INTA0` writer"]
pub struct W(crate::W<INTA0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INTA0_SPEC>;
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
impl From<crate::W<INTA0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INTA0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `INTA0` reader - Interrupt A status for DMA channel."]
pub type INTA0_R = crate::BitReader<INTA0_A>;
#[doc = "Interrupt A status for DMA channel.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum INTA0_A {
    #[doc = "0: The DMA channel interrupt A is not active."]
    NOT_ACTIVE = 0,
    #[doc = "1: The DMA channel interrupt A is active."]
    ACTIVE = 1,
}
impl From<INTA0_A> for bool {
    #[inline(always)]
    fn from(variant: INTA0_A) -> Self {
        variant as u8 != 0
    }
}
impl INTA0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INTA0_A {
        match self.bits {
            false => INTA0_A::NOT_ACTIVE,
            true => INTA0_A::ACTIVE,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_ACTIVE`"]
    #[inline(always)]
    pub fn is_not_active(&self) -> bool {
        *self == INTA0_A::NOT_ACTIVE
    }
    #[doc = "Checks if the value of the field is `ACTIVE`"]
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        *self == INTA0_A::ACTIVE
    }
}
#[doc = "Field `INTA0` writer - Interrupt A status for DMA channel."]
pub type INTA0_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTA0_SPEC, INTA0_A, O>;
impl<'a, const O: u8> INTA0_W<'a, O> {
    #[doc = "The DMA channel interrupt A is not active."]
    #[inline(always)]
    pub fn not_active(self) -> &'a mut W {
        self.variant(INTA0_A::NOT_ACTIVE)
    }
    #[doc = "The DMA channel interrupt A is active."]
    #[inline(always)]
    pub fn active(self) -> &'a mut W {
        self.variant(INTA0_A::ACTIVE)
    }
}
#[doc = "Field `INTA1` reader - Interrupt A status for DMA channel."]
pub type INTA1_R = crate::BitReader<INTA1_A>;
#[doc = "Interrupt A status for DMA channel.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum INTA1_A {
    #[doc = "0: The DMA channel interrupt A is not active."]
    NOT_ACTIVE = 0,
    #[doc = "1: The DMA channel interrupt A is active."]
    ACTIVE = 1,
}
impl From<INTA1_A> for bool {
    #[inline(always)]
    fn from(variant: INTA1_A) -> Self {
        variant as u8 != 0
    }
}
impl INTA1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INTA1_A {
        match self.bits {
            false => INTA1_A::NOT_ACTIVE,
            true => INTA1_A::ACTIVE,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_ACTIVE`"]
    #[inline(always)]
    pub fn is_not_active(&self) -> bool {
        *self == INTA1_A::NOT_ACTIVE
    }
    #[doc = "Checks if the value of the field is `ACTIVE`"]
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        *self == INTA1_A::ACTIVE
    }
}
#[doc = "Field `INTA1` writer - Interrupt A status for DMA channel."]
pub type INTA1_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTA0_SPEC, INTA1_A, O>;
impl<'a, const O: u8> INTA1_W<'a, O> {
    #[doc = "The DMA channel interrupt A is not active."]
    #[inline(always)]
    pub fn not_active(self) -> &'a mut W {
        self.variant(INTA1_A::NOT_ACTIVE)
    }
    #[doc = "The DMA channel interrupt A is active."]
    #[inline(always)]
    pub fn active(self) -> &'a mut W {
        self.variant(INTA1_A::ACTIVE)
    }
}
#[doc = "Field `INTA2` reader - Interrupt A status for DMA channel."]
pub type INTA2_R = crate::BitReader<INTA2_A>;
#[doc = "Interrupt A status for DMA channel.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum INTA2_A {
    #[doc = "0: The DMA channel interrupt A is not active."]
    NOT_ACTIVE = 0,
    #[doc = "1: The DMA channel interrupt A is active."]
    ACTIVE = 1,
}
impl From<INTA2_A> for bool {
    #[inline(always)]
    fn from(variant: INTA2_A) -> Self {
        variant as u8 != 0
    }
}
impl INTA2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INTA2_A {
        match self.bits {
            false => INTA2_A::NOT_ACTIVE,
            true => INTA2_A::ACTIVE,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_ACTIVE`"]
    #[inline(always)]
    pub fn is_not_active(&self) -> bool {
        *self == INTA2_A::NOT_ACTIVE
    }
    #[doc = "Checks if the value of the field is `ACTIVE`"]
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        *self == INTA2_A::ACTIVE
    }
}
#[doc = "Field `INTA2` writer - Interrupt A status for DMA channel."]
pub type INTA2_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTA0_SPEC, INTA2_A, O>;
impl<'a, const O: u8> INTA2_W<'a, O> {
    #[doc = "The DMA channel interrupt A is not active."]
    #[inline(always)]
    pub fn not_active(self) -> &'a mut W {
        self.variant(INTA2_A::NOT_ACTIVE)
    }
    #[doc = "The DMA channel interrupt A is active."]
    #[inline(always)]
    pub fn active(self) -> &'a mut W {
        self.variant(INTA2_A::ACTIVE)
    }
}
#[doc = "Field `INTA3` reader - Interrupt A status for DMA channel."]
pub type INTA3_R = crate::BitReader<INTA3_A>;
#[doc = "Interrupt A status for DMA channel.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum INTA3_A {
    #[doc = "0: The DMA channel interrupt A is not active."]
    NOT_ACTIVE = 0,
    #[doc = "1: The DMA channel interrupt A is active."]
    ACTIVE = 1,
}
impl From<INTA3_A> for bool {
    #[inline(always)]
    fn from(variant: INTA3_A) -> Self {
        variant as u8 != 0
    }
}
impl INTA3_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INTA3_A {
        match self.bits {
            false => INTA3_A::NOT_ACTIVE,
            true => INTA3_A::ACTIVE,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_ACTIVE`"]
    #[inline(always)]
    pub fn is_not_active(&self) -> bool {
        *self == INTA3_A::NOT_ACTIVE
    }
    #[doc = "Checks if the value of the field is `ACTIVE`"]
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        *self == INTA3_A::ACTIVE
    }
}
#[doc = "Field `INTA3` writer - Interrupt A status for DMA channel."]
pub type INTA3_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTA0_SPEC, INTA3_A, O>;
impl<'a, const O: u8> INTA3_W<'a, O> {
    #[doc = "The DMA channel interrupt A is not active."]
    #[inline(always)]
    pub fn not_active(self) -> &'a mut W {
        self.variant(INTA3_A::NOT_ACTIVE)
    }
    #[doc = "The DMA channel interrupt A is active."]
    #[inline(always)]
    pub fn active(self) -> &'a mut W {
        self.variant(INTA3_A::ACTIVE)
    }
}
#[doc = "Field `INTA4` reader - Interrupt A status for DMA channel."]
pub type INTA4_R = crate::BitReader<INTA4_A>;
#[doc = "Interrupt A status for DMA channel.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum INTA4_A {
    #[doc = "0: The DMA channel interrupt A is not active."]
    NOT_ACTIVE = 0,
    #[doc = "1: The DMA channel interrupt A is active."]
    ACTIVE = 1,
}
impl From<INTA4_A> for bool {
    #[inline(always)]
    fn from(variant: INTA4_A) -> Self {
        variant as u8 != 0
    }
}
impl INTA4_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INTA4_A {
        match self.bits {
            false => INTA4_A::NOT_ACTIVE,
            true => INTA4_A::ACTIVE,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_ACTIVE`"]
    #[inline(always)]
    pub fn is_not_active(&self) -> bool {
        *self == INTA4_A::NOT_ACTIVE
    }
    #[doc = "Checks if the value of the field is `ACTIVE`"]
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        *self == INTA4_A::ACTIVE
    }
}
#[doc = "Field `INTA4` writer - Interrupt A status for DMA channel."]
pub type INTA4_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTA0_SPEC, INTA4_A, O>;
impl<'a, const O: u8> INTA4_W<'a, O> {
    #[doc = "The DMA channel interrupt A is not active."]
    #[inline(always)]
    pub fn not_active(self) -> &'a mut W {
        self.variant(INTA4_A::NOT_ACTIVE)
    }
    #[doc = "The DMA channel interrupt A is active."]
    #[inline(always)]
    pub fn active(self) -> &'a mut W {
        self.variant(INTA4_A::ACTIVE)
    }
}
#[doc = "Field `INTA5` reader - Interrupt A status for DMA channel."]
pub type INTA5_R = crate::BitReader<INTA5_A>;
#[doc = "Interrupt A status for DMA channel.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum INTA5_A {
    #[doc = "0: The DMA channel interrupt A is not active."]
    NOT_ACTIVE = 0,
    #[doc = "1: The DMA channel interrupt A is active."]
    ACTIVE = 1,
}
impl From<INTA5_A> for bool {
    #[inline(always)]
    fn from(variant: INTA5_A) -> Self {
        variant as u8 != 0
    }
}
impl INTA5_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INTA5_A {
        match self.bits {
            false => INTA5_A::NOT_ACTIVE,
            true => INTA5_A::ACTIVE,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_ACTIVE`"]
    #[inline(always)]
    pub fn is_not_active(&self) -> bool {
        *self == INTA5_A::NOT_ACTIVE
    }
    #[doc = "Checks if the value of the field is `ACTIVE`"]
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        *self == INTA5_A::ACTIVE
    }
}
#[doc = "Field `INTA5` writer - Interrupt A status for DMA channel."]
pub type INTA5_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTA0_SPEC, INTA5_A, O>;
impl<'a, const O: u8> INTA5_W<'a, O> {
    #[doc = "The DMA channel interrupt A is not active."]
    #[inline(always)]
    pub fn not_active(self) -> &'a mut W {
        self.variant(INTA5_A::NOT_ACTIVE)
    }
    #[doc = "The DMA channel interrupt A is active."]
    #[inline(always)]
    pub fn active(self) -> &'a mut W {
        self.variant(INTA5_A::ACTIVE)
    }
}
#[doc = "Field `INTA6` reader - Interrupt A status for DMA channel."]
pub type INTA6_R = crate::BitReader<INTA6_A>;
#[doc = "Interrupt A status for DMA channel.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum INTA6_A {
    #[doc = "0: The DMA channel interrupt A is not active."]
    NOT_ACTIVE = 0,
    #[doc = "1: The DMA channel interrupt A is active."]
    ACTIVE = 1,
}
impl From<INTA6_A> for bool {
    #[inline(always)]
    fn from(variant: INTA6_A) -> Self {
        variant as u8 != 0
    }
}
impl INTA6_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INTA6_A {
        match self.bits {
            false => INTA6_A::NOT_ACTIVE,
            true => INTA6_A::ACTIVE,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_ACTIVE`"]
    #[inline(always)]
    pub fn is_not_active(&self) -> bool {
        *self == INTA6_A::NOT_ACTIVE
    }
    #[doc = "Checks if the value of the field is `ACTIVE`"]
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        *self == INTA6_A::ACTIVE
    }
}
#[doc = "Field `INTA6` writer - Interrupt A status for DMA channel."]
pub type INTA6_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTA0_SPEC, INTA6_A, O>;
impl<'a, const O: u8> INTA6_W<'a, O> {
    #[doc = "The DMA channel interrupt A is not active."]
    #[inline(always)]
    pub fn not_active(self) -> &'a mut W {
        self.variant(INTA6_A::NOT_ACTIVE)
    }
    #[doc = "The DMA channel interrupt A is active."]
    #[inline(always)]
    pub fn active(self) -> &'a mut W {
        self.variant(INTA6_A::ACTIVE)
    }
}
#[doc = "Field `INTA7` reader - Interrupt A status for DMA channel."]
pub type INTA7_R = crate::BitReader<INTA7_A>;
#[doc = "Interrupt A status for DMA channel.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum INTA7_A {
    #[doc = "0: The DMA channel interrupt A is not active."]
    NOT_ACTIVE = 0,
    #[doc = "1: The DMA channel interrupt A is active."]
    ACTIVE = 1,
}
impl From<INTA7_A> for bool {
    #[inline(always)]
    fn from(variant: INTA7_A) -> Self {
        variant as u8 != 0
    }
}
impl INTA7_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INTA7_A {
        match self.bits {
            false => INTA7_A::NOT_ACTIVE,
            true => INTA7_A::ACTIVE,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_ACTIVE`"]
    #[inline(always)]
    pub fn is_not_active(&self) -> bool {
        *self == INTA7_A::NOT_ACTIVE
    }
    #[doc = "Checks if the value of the field is `ACTIVE`"]
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        *self == INTA7_A::ACTIVE
    }
}
#[doc = "Field `INTA7` writer - Interrupt A status for DMA channel."]
pub type INTA7_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTA0_SPEC, INTA7_A, O>;
impl<'a, const O: u8> INTA7_W<'a, O> {
    #[doc = "The DMA channel interrupt A is not active."]
    #[inline(always)]
    pub fn not_active(self) -> &'a mut W {
        self.variant(INTA7_A::NOT_ACTIVE)
    }
    #[doc = "The DMA channel interrupt A is active."]
    #[inline(always)]
    pub fn active(self) -> &'a mut W {
        self.variant(INTA7_A::ACTIVE)
    }
}
#[doc = "Field `INTA8` reader - Interrupt A status for DMA channel."]
pub type INTA8_R = crate::BitReader<INTA8_A>;
#[doc = "Interrupt A status for DMA channel.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum INTA8_A {
    #[doc = "0: The DMA channel interrupt A is not active."]
    NOT_ACTIVE = 0,
    #[doc = "1: The DMA channel interrupt A is active."]
    ACTIVE = 1,
}
impl From<INTA8_A> for bool {
    #[inline(always)]
    fn from(variant: INTA8_A) -> Self {
        variant as u8 != 0
    }
}
impl INTA8_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INTA8_A {
        match self.bits {
            false => INTA8_A::NOT_ACTIVE,
            true => INTA8_A::ACTIVE,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_ACTIVE`"]
    #[inline(always)]
    pub fn is_not_active(&self) -> bool {
        *self == INTA8_A::NOT_ACTIVE
    }
    #[doc = "Checks if the value of the field is `ACTIVE`"]
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        *self == INTA8_A::ACTIVE
    }
}
#[doc = "Field `INTA8` writer - Interrupt A status for DMA channel."]
pub type INTA8_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTA0_SPEC, INTA8_A, O>;
impl<'a, const O: u8> INTA8_W<'a, O> {
    #[doc = "The DMA channel interrupt A is not active."]
    #[inline(always)]
    pub fn not_active(self) -> &'a mut W {
        self.variant(INTA8_A::NOT_ACTIVE)
    }
    #[doc = "The DMA channel interrupt A is active."]
    #[inline(always)]
    pub fn active(self) -> &'a mut W {
        self.variant(INTA8_A::ACTIVE)
    }
}
#[doc = "Field `INTA9` reader - Interrupt A status for DMA channel."]
pub type INTA9_R = crate::BitReader<INTA9_A>;
#[doc = "Interrupt A status for DMA channel.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum INTA9_A {
    #[doc = "0: The DMA channel interrupt A is not active."]
    NOT_ACTIVE = 0,
    #[doc = "1: The DMA channel interrupt A is active."]
    ACTIVE = 1,
}
impl From<INTA9_A> for bool {
    #[inline(always)]
    fn from(variant: INTA9_A) -> Self {
        variant as u8 != 0
    }
}
impl INTA9_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INTA9_A {
        match self.bits {
            false => INTA9_A::NOT_ACTIVE,
            true => INTA9_A::ACTIVE,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_ACTIVE`"]
    #[inline(always)]
    pub fn is_not_active(&self) -> bool {
        *self == INTA9_A::NOT_ACTIVE
    }
    #[doc = "Checks if the value of the field is `ACTIVE`"]
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        *self == INTA9_A::ACTIVE
    }
}
#[doc = "Field `INTA9` writer - Interrupt A status for DMA channel."]
pub type INTA9_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTA0_SPEC, INTA9_A, O>;
impl<'a, const O: u8> INTA9_W<'a, O> {
    #[doc = "The DMA channel interrupt A is not active."]
    #[inline(always)]
    pub fn not_active(self) -> &'a mut W {
        self.variant(INTA9_A::NOT_ACTIVE)
    }
    #[doc = "The DMA channel interrupt A is active."]
    #[inline(always)]
    pub fn active(self) -> &'a mut W {
        self.variant(INTA9_A::ACTIVE)
    }
}
#[doc = "Field `INTA10` reader - Interrupt A status for DMA channel."]
pub type INTA10_R = crate::BitReader<INTA10_A>;
#[doc = "Interrupt A status for DMA channel.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum INTA10_A {
    #[doc = "0: The DMA channel interrupt A is not active."]
    NOT_ACTIVE = 0,
    #[doc = "1: The DMA channel interrupt A is active."]
    ACTIVE = 1,
}
impl From<INTA10_A> for bool {
    #[inline(always)]
    fn from(variant: INTA10_A) -> Self {
        variant as u8 != 0
    }
}
impl INTA10_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INTA10_A {
        match self.bits {
            false => INTA10_A::NOT_ACTIVE,
            true => INTA10_A::ACTIVE,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_ACTIVE`"]
    #[inline(always)]
    pub fn is_not_active(&self) -> bool {
        *self == INTA10_A::NOT_ACTIVE
    }
    #[doc = "Checks if the value of the field is `ACTIVE`"]
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        *self == INTA10_A::ACTIVE
    }
}
#[doc = "Field `INTA10` writer - Interrupt A status for DMA channel."]
pub type INTA10_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTA0_SPEC, INTA10_A, O>;
impl<'a, const O: u8> INTA10_W<'a, O> {
    #[doc = "The DMA channel interrupt A is not active."]
    #[inline(always)]
    pub fn not_active(self) -> &'a mut W {
        self.variant(INTA10_A::NOT_ACTIVE)
    }
    #[doc = "The DMA channel interrupt A is active."]
    #[inline(always)]
    pub fn active(self) -> &'a mut W {
        self.variant(INTA10_A::ACTIVE)
    }
}
#[doc = "Field `INTA11` reader - Interrupt A status for DMA channel."]
pub type INTA11_R = crate::BitReader<INTA11_A>;
#[doc = "Interrupt A status for DMA channel.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum INTA11_A {
    #[doc = "0: The DMA channel interrupt A is not active."]
    NOT_ACTIVE = 0,
    #[doc = "1: The DMA channel interrupt A is active."]
    ACTIVE = 1,
}
impl From<INTA11_A> for bool {
    #[inline(always)]
    fn from(variant: INTA11_A) -> Self {
        variant as u8 != 0
    }
}
impl INTA11_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INTA11_A {
        match self.bits {
            false => INTA11_A::NOT_ACTIVE,
            true => INTA11_A::ACTIVE,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_ACTIVE`"]
    #[inline(always)]
    pub fn is_not_active(&self) -> bool {
        *self == INTA11_A::NOT_ACTIVE
    }
    #[doc = "Checks if the value of the field is `ACTIVE`"]
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        *self == INTA11_A::ACTIVE
    }
}
#[doc = "Field `INTA11` writer - Interrupt A status for DMA channel."]
pub type INTA11_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTA0_SPEC, INTA11_A, O>;
impl<'a, const O: u8> INTA11_W<'a, O> {
    #[doc = "The DMA channel interrupt A is not active."]
    #[inline(always)]
    pub fn not_active(self) -> &'a mut W {
        self.variant(INTA11_A::NOT_ACTIVE)
    }
    #[doc = "The DMA channel interrupt A is active."]
    #[inline(always)]
    pub fn active(self) -> &'a mut W {
        self.variant(INTA11_A::ACTIVE)
    }
}
#[doc = "Field `INTA12` reader - Interrupt A status for DMA channel."]
pub type INTA12_R = crate::BitReader<INTA12_A>;
#[doc = "Interrupt A status for DMA channel.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum INTA12_A {
    #[doc = "0: The DMA channel interrupt A is not active."]
    NOT_ACTIVE = 0,
    #[doc = "1: The DMA channel interrupt A is active."]
    ACTIVE = 1,
}
impl From<INTA12_A> for bool {
    #[inline(always)]
    fn from(variant: INTA12_A) -> Self {
        variant as u8 != 0
    }
}
impl INTA12_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INTA12_A {
        match self.bits {
            false => INTA12_A::NOT_ACTIVE,
            true => INTA12_A::ACTIVE,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_ACTIVE`"]
    #[inline(always)]
    pub fn is_not_active(&self) -> bool {
        *self == INTA12_A::NOT_ACTIVE
    }
    #[doc = "Checks if the value of the field is `ACTIVE`"]
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        *self == INTA12_A::ACTIVE
    }
}
#[doc = "Field `INTA12` writer - Interrupt A status for DMA channel."]
pub type INTA12_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTA0_SPEC, INTA12_A, O>;
impl<'a, const O: u8> INTA12_W<'a, O> {
    #[doc = "The DMA channel interrupt A is not active."]
    #[inline(always)]
    pub fn not_active(self) -> &'a mut W {
        self.variant(INTA12_A::NOT_ACTIVE)
    }
    #[doc = "The DMA channel interrupt A is active."]
    #[inline(always)]
    pub fn active(self) -> &'a mut W {
        self.variant(INTA12_A::ACTIVE)
    }
}
#[doc = "Field `INTA13` reader - Interrupt A status for DMA channel."]
pub type INTA13_R = crate::BitReader<INTA13_A>;
#[doc = "Interrupt A status for DMA channel.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum INTA13_A {
    #[doc = "0: The DMA channel interrupt A is not active."]
    NOT_ACTIVE = 0,
    #[doc = "1: The DMA channel interrupt A is active."]
    ACTIVE = 1,
}
impl From<INTA13_A> for bool {
    #[inline(always)]
    fn from(variant: INTA13_A) -> Self {
        variant as u8 != 0
    }
}
impl INTA13_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INTA13_A {
        match self.bits {
            false => INTA13_A::NOT_ACTIVE,
            true => INTA13_A::ACTIVE,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_ACTIVE`"]
    #[inline(always)]
    pub fn is_not_active(&self) -> bool {
        *self == INTA13_A::NOT_ACTIVE
    }
    #[doc = "Checks if the value of the field is `ACTIVE`"]
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        *self == INTA13_A::ACTIVE
    }
}
#[doc = "Field `INTA13` writer - Interrupt A status for DMA channel."]
pub type INTA13_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTA0_SPEC, INTA13_A, O>;
impl<'a, const O: u8> INTA13_W<'a, O> {
    #[doc = "The DMA channel interrupt A is not active."]
    #[inline(always)]
    pub fn not_active(self) -> &'a mut W {
        self.variant(INTA13_A::NOT_ACTIVE)
    }
    #[doc = "The DMA channel interrupt A is active."]
    #[inline(always)]
    pub fn active(self) -> &'a mut W {
        self.variant(INTA13_A::ACTIVE)
    }
}
#[doc = "Field `INTA14` reader - Interrupt A status for DMA channel."]
pub type INTA14_R = crate::BitReader<INTA14_A>;
#[doc = "Interrupt A status for DMA channel.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum INTA14_A {
    #[doc = "0: The DMA channel interrupt A is not active."]
    NOT_ACTIVE = 0,
    #[doc = "1: The DMA channel interrupt A is active."]
    ACTIVE = 1,
}
impl From<INTA14_A> for bool {
    #[inline(always)]
    fn from(variant: INTA14_A) -> Self {
        variant as u8 != 0
    }
}
impl INTA14_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INTA14_A {
        match self.bits {
            false => INTA14_A::NOT_ACTIVE,
            true => INTA14_A::ACTIVE,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_ACTIVE`"]
    #[inline(always)]
    pub fn is_not_active(&self) -> bool {
        *self == INTA14_A::NOT_ACTIVE
    }
    #[doc = "Checks if the value of the field is `ACTIVE`"]
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        *self == INTA14_A::ACTIVE
    }
}
#[doc = "Field `INTA14` writer - Interrupt A status for DMA channel."]
pub type INTA14_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTA0_SPEC, INTA14_A, O>;
impl<'a, const O: u8> INTA14_W<'a, O> {
    #[doc = "The DMA channel interrupt A is not active."]
    #[inline(always)]
    pub fn not_active(self) -> &'a mut W {
        self.variant(INTA14_A::NOT_ACTIVE)
    }
    #[doc = "The DMA channel interrupt A is active."]
    #[inline(always)]
    pub fn active(self) -> &'a mut W {
        self.variant(INTA14_A::ACTIVE)
    }
}
#[doc = "Field `INTA15` reader - Interrupt A status for DMA channel."]
pub type INTA15_R = crate::BitReader<INTA15_A>;
#[doc = "Interrupt A status for DMA channel.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum INTA15_A {
    #[doc = "0: The DMA channel interrupt A is not active."]
    NOT_ACTIVE = 0,
    #[doc = "1: The DMA channel interrupt A is active."]
    ACTIVE = 1,
}
impl From<INTA15_A> for bool {
    #[inline(always)]
    fn from(variant: INTA15_A) -> Self {
        variant as u8 != 0
    }
}
impl INTA15_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INTA15_A {
        match self.bits {
            false => INTA15_A::NOT_ACTIVE,
            true => INTA15_A::ACTIVE,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_ACTIVE`"]
    #[inline(always)]
    pub fn is_not_active(&self) -> bool {
        *self == INTA15_A::NOT_ACTIVE
    }
    #[doc = "Checks if the value of the field is `ACTIVE`"]
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        *self == INTA15_A::ACTIVE
    }
}
#[doc = "Field `INTA15` writer - Interrupt A status for DMA channel."]
pub type INTA15_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTA0_SPEC, INTA15_A, O>;
impl<'a, const O: u8> INTA15_W<'a, O> {
    #[doc = "The DMA channel interrupt A is not active."]
    #[inline(always)]
    pub fn not_active(self) -> &'a mut W {
        self.variant(INTA15_A::NOT_ACTIVE)
    }
    #[doc = "The DMA channel interrupt A is active."]
    #[inline(always)]
    pub fn active(self) -> &'a mut W {
        self.variant(INTA15_A::ACTIVE)
    }
}
#[doc = "Field `INTA16` reader - Interrupt A status for DMA channel."]
pub type INTA16_R = crate::BitReader<INTA16_A>;
#[doc = "Interrupt A status for DMA channel.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum INTA16_A {
    #[doc = "0: The DMA channel interrupt A is not active."]
    NOT_ACTIVE = 0,
    #[doc = "1: The DMA channel interrupt A is active."]
    ACTIVE = 1,
}
impl From<INTA16_A> for bool {
    #[inline(always)]
    fn from(variant: INTA16_A) -> Self {
        variant as u8 != 0
    }
}
impl INTA16_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INTA16_A {
        match self.bits {
            false => INTA16_A::NOT_ACTIVE,
            true => INTA16_A::ACTIVE,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_ACTIVE`"]
    #[inline(always)]
    pub fn is_not_active(&self) -> bool {
        *self == INTA16_A::NOT_ACTIVE
    }
    #[doc = "Checks if the value of the field is `ACTIVE`"]
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        *self == INTA16_A::ACTIVE
    }
}
#[doc = "Field `INTA16` writer - Interrupt A status for DMA channel."]
pub type INTA16_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTA0_SPEC, INTA16_A, O>;
impl<'a, const O: u8> INTA16_W<'a, O> {
    #[doc = "The DMA channel interrupt A is not active."]
    #[inline(always)]
    pub fn not_active(self) -> &'a mut W {
        self.variant(INTA16_A::NOT_ACTIVE)
    }
    #[doc = "The DMA channel interrupt A is active."]
    #[inline(always)]
    pub fn active(self) -> &'a mut W {
        self.variant(INTA16_A::ACTIVE)
    }
}
#[doc = "Field `INTA17` reader - Interrupt A status for DMA channel."]
pub type INTA17_R = crate::BitReader<INTA17_A>;
#[doc = "Interrupt A status for DMA channel.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum INTA17_A {
    #[doc = "0: The DMA channel interrupt A is not active."]
    NOT_ACTIVE = 0,
    #[doc = "1: The DMA channel interrupt A is active."]
    ACTIVE = 1,
}
impl From<INTA17_A> for bool {
    #[inline(always)]
    fn from(variant: INTA17_A) -> Self {
        variant as u8 != 0
    }
}
impl INTA17_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INTA17_A {
        match self.bits {
            false => INTA17_A::NOT_ACTIVE,
            true => INTA17_A::ACTIVE,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_ACTIVE`"]
    #[inline(always)]
    pub fn is_not_active(&self) -> bool {
        *self == INTA17_A::NOT_ACTIVE
    }
    #[doc = "Checks if the value of the field is `ACTIVE`"]
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        *self == INTA17_A::ACTIVE
    }
}
#[doc = "Field `INTA17` writer - Interrupt A status for DMA channel."]
pub type INTA17_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTA0_SPEC, INTA17_A, O>;
impl<'a, const O: u8> INTA17_W<'a, O> {
    #[doc = "The DMA channel interrupt A is not active."]
    #[inline(always)]
    pub fn not_active(self) -> &'a mut W {
        self.variant(INTA17_A::NOT_ACTIVE)
    }
    #[doc = "The DMA channel interrupt A is active."]
    #[inline(always)]
    pub fn active(self) -> &'a mut W {
        self.variant(INTA17_A::ACTIVE)
    }
}
#[doc = "Field `INTA18` reader - Interrupt A status for DMA channel."]
pub type INTA18_R = crate::BitReader<INTA18_A>;
#[doc = "Interrupt A status for DMA channel.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum INTA18_A {
    #[doc = "0: The DMA channel interrupt A is not active."]
    NOT_ACTIVE = 0,
    #[doc = "1: The DMA channel interrupt A is active."]
    ACTIVE = 1,
}
impl From<INTA18_A> for bool {
    #[inline(always)]
    fn from(variant: INTA18_A) -> Self {
        variant as u8 != 0
    }
}
impl INTA18_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INTA18_A {
        match self.bits {
            false => INTA18_A::NOT_ACTIVE,
            true => INTA18_A::ACTIVE,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_ACTIVE`"]
    #[inline(always)]
    pub fn is_not_active(&self) -> bool {
        *self == INTA18_A::NOT_ACTIVE
    }
    #[doc = "Checks if the value of the field is `ACTIVE`"]
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        *self == INTA18_A::ACTIVE
    }
}
#[doc = "Field `INTA18` writer - Interrupt A status for DMA channel."]
pub type INTA18_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTA0_SPEC, INTA18_A, O>;
impl<'a, const O: u8> INTA18_W<'a, O> {
    #[doc = "The DMA channel interrupt A is not active."]
    #[inline(always)]
    pub fn not_active(self) -> &'a mut W {
        self.variant(INTA18_A::NOT_ACTIVE)
    }
    #[doc = "The DMA channel interrupt A is active."]
    #[inline(always)]
    pub fn active(self) -> &'a mut W {
        self.variant(INTA18_A::ACTIVE)
    }
}
#[doc = "Field `INTA19` reader - Interrupt A status for DMA channel."]
pub type INTA19_R = crate::BitReader<INTA19_A>;
#[doc = "Interrupt A status for DMA channel.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum INTA19_A {
    #[doc = "0: The DMA channel interrupt A is not active."]
    NOT_ACTIVE = 0,
    #[doc = "1: The DMA channel interrupt A is active."]
    ACTIVE = 1,
}
impl From<INTA19_A> for bool {
    #[inline(always)]
    fn from(variant: INTA19_A) -> Self {
        variant as u8 != 0
    }
}
impl INTA19_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INTA19_A {
        match self.bits {
            false => INTA19_A::NOT_ACTIVE,
            true => INTA19_A::ACTIVE,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_ACTIVE`"]
    #[inline(always)]
    pub fn is_not_active(&self) -> bool {
        *self == INTA19_A::NOT_ACTIVE
    }
    #[doc = "Checks if the value of the field is `ACTIVE`"]
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        *self == INTA19_A::ACTIVE
    }
}
#[doc = "Field `INTA19` writer - Interrupt A status for DMA channel."]
pub type INTA19_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTA0_SPEC, INTA19_A, O>;
impl<'a, const O: u8> INTA19_W<'a, O> {
    #[doc = "The DMA channel interrupt A is not active."]
    #[inline(always)]
    pub fn not_active(self) -> &'a mut W {
        self.variant(INTA19_A::NOT_ACTIVE)
    }
    #[doc = "The DMA channel interrupt A is active."]
    #[inline(always)]
    pub fn active(self) -> &'a mut W {
        self.variant(INTA19_A::ACTIVE)
    }
}
#[doc = "Field `INTA20` reader - Interrupt A status for DMA channel."]
pub type INTA20_R = crate::BitReader<INTA20_A>;
#[doc = "Interrupt A status for DMA channel.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum INTA20_A {
    #[doc = "0: The DMA channel interrupt A is not active."]
    NOT_ACTIVE = 0,
    #[doc = "1: The DMA channel interrupt A is active."]
    ACTIVE = 1,
}
impl From<INTA20_A> for bool {
    #[inline(always)]
    fn from(variant: INTA20_A) -> Self {
        variant as u8 != 0
    }
}
impl INTA20_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INTA20_A {
        match self.bits {
            false => INTA20_A::NOT_ACTIVE,
            true => INTA20_A::ACTIVE,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_ACTIVE`"]
    #[inline(always)]
    pub fn is_not_active(&self) -> bool {
        *self == INTA20_A::NOT_ACTIVE
    }
    #[doc = "Checks if the value of the field is `ACTIVE`"]
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        *self == INTA20_A::ACTIVE
    }
}
#[doc = "Field `INTA20` writer - Interrupt A status for DMA channel."]
pub type INTA20_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTA0_SPEC, INTA20_A, O>;
impl<'a, const O: u8> INTA20_W<'a, O> {
    #[doc = "The DMA channel interrupt A is not active."]
    #[inline(always)]
    pub fn not_active(self) -> &'a mut W {
        self.variant(INTA20_A::NOT_ACTIVE)
    }
    #[doc = "The DMA channel interrupt A is active."]
    #[inline(always)]
    pub fn active(self) -> &'a mut W {
        self.variant(INTA20_A::ACTIVE)
    }
}
#[doc = "Field `INTA21` reader - Interrupt A status for DMA channel."]
pub type INTA21_R = crate::BitReader<INTA21_A>;
#[doc = "Interrupt A status for DMA channel.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum INTA21_A {
    #[doc = "0: The DMA channel interrupt A is not active."]
    NOT_ACTIVE = 0,
    #[doc = "1: The DMA channel interrupt A is active."]
    ACTIVE = 1,
}
impl From<INTA21_A> for bool {
    #[inline(always)]
    fn from(variant: INTA21_A) -> Self {
        variant as u8 != 0
    }
}
impl INTA21_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INTA21_A {
        match self.bits {
            false => INTA21_A::NOT_ACTIVE,
            true => INTA21_A::ACTIVE,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_ACTIVE`"]
    #[inline(always)]
    pub fn is_not_active(&self) -> bool {
        *self == INTA21_A::NOT_ACTIVE
    }
    #[doc = "Checks if the value of the field is `ACTIVE`"]
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        *self == INTA21_A::ACTIVE
    }
}
#[doc = "Field `INTA21` writer - Interrupt A status for DMA channel."]
pub type INTA21_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTA0_SPEC, INTA21_A, O>;
impl<'a, const O: u8> INTA21_W<'a, O> {
    #[doc = "The DMA channel interrupt A is not active."]
    #[inline(always)]
    pub fn not_active(self) -> &'a mut W {
        self.variant(INTA21_A::NOT_ACTIVE)
    }
    #[doc = "The DMA channel interrupt A is active."]
    #[inline(always)]
    pub fn active(self) -> &'a mut W {
        self.variant(INTA21_A::ACTIVE)
    }
}
#[doc = "Field `INTA22` reader - Interrupt A status for DMA channel."]
pub type INTA22_R = crate::BitReader<INTA22_A>;
#[doc = "Interrupt A status for DMA channel.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum INTA22_A {
    #[doc = "0: The DMA channel interrupt A is not active."]
    NOT_ACTIVE = 0,
    #[doc = "1: The DMA channel interrupt A is active."]
    ACTIVE = 1,
}
impl From<INTA22_A> for bool {
    #[inline(always)]
    fn from(variant: INTA22_A) -> Self {
        variant as u8 != 0
    }
}
impl INTA22_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INTA22_A {
        match self.bits {
            false => INTA22_A::NOT_ACTIVE,
            true => INTA22_A::ACTIVE,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_ACTIVE`"]
    #[inline(always)]
    pub fn is_not_active(&self) -> bool {
        *self == INTA22_A::NOT_ACTIVE
    }
    #[doc = "Checks if the value of the field is `ACTIVE`"]
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        *self == INTA22_A::ACTIVE
    }
}
#[doc = "Field `INTA22` writer - Interrupt A status for DMA channel."]
pub type INTA22_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTA0_SPEC, INTA22_A, O>;
impl<'a, const O: u8> INTA22_W<'a, O> {
    #[doc = "The DMA channel interrupt A is not active."]
    #[inline(always)]
    pub fn not_active(self) -> &'a mut W {
        self.variant(INTA22_A::NOT_ACTIVE)
    }
    #[doc = "The DMA channel interrupt A is active."]
    #[inline(always)]
    pub fn active(self) -> &'a mut W {
        self.variant(INTA22_A::ACTIVE)
    }
}
#[doc = "Field `INTA23` reader - Interrupt A status for DMA channel."]
pub type INTA23_R = crate::BitReader<INTA23_A>;
#[doc = "Interrupt A status for DMA channel.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum INTA23_A {
    #[doc = "0: The DMA channel interrupt A is not active."]
    NOT_ACTIVE = 0,
    #[doc = "1: The DMA channel interrupt A is active."]
    ACTIVE = 1,
}
impl From<INTA23_A> for bool {
    #[inline(always)]
    fn from(variant: INTA23_A) -> Self {
        variant as u8 != 0
    }
}
impl INTA23_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INTA23_A {
        match self.bits {
            false => INTA23_A::NOT_ACTIVE,
            true => INTA23_A::ACTIVE,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_ACTIVE`"]
    #[inline(always)]
    pub fn is_not_active(&self) -> bool {
        *self == INTA23_A::NOT_ACTIVE
    }
    #[doc = "Checks if the value of the field is `ACTIVE`"]
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        *self == INTA23_A::ACTIVE
    }
}
#[doc = "Field `INTA23` writer - Interrupt A status for DMA channel."]
pub type INTA23_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTA0_SPEC, INTA23_A, O>;
impl<'a, const O: u8> INTA23_W<'a, O> {
    #[doc = "The DMA channel interrupt A is not active."]
    #[inline(always)]
    pub fn not_active(self) -> &'a mut W {
        self.variant(INTA23_A::NOT_ACTIVE)
    }
    #[doc = "The DMA channel interrupt A is active."]
    #[inline(always)]
    pub fn active(self) -> &'a mut W {
        self.variant(INTA23_A::ACTIVE)
    }
}
#[doc = "Field `INTA24` reader - Interrupt A status for DMA channel."]
pub type INTA24_R = crate::BitReader<INTA24_A>;
#[doc = "Interrupt A status for DMA channel.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum INTA24_A {
    #[doc = "0: The DMA channel interrupt A is not active."]
    NOT_ACTIVE = 0,
    #[doc = "1: The DMA channel interrupt A is active."]
    ACTIVE = 1,
}
impl From<INTA24_A> for bool {
    #[inline(always)]
    fn from(variant: INTA24_A) -> Self {
        variant as u8 != 0
    }
}
impl INTA24_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INTA24_A {
        match self.bits {
            false => INTA24_A::NOT_ACTIVE,
            true => INTA24_A::ACTIVE,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_ACTIVE`"]
    #[inline(always)]
    pub fn is_not_active(&self) -> bool {
        *self == INTA24_A::NOT_ACTIVE
    }
    #[doc = "Checks if the value of the field is `ACTIVE`"]
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        *self == INTA24_A::ACTIVE
    }
}
#[doc = "Field `INTA24` writer - Interrupt A status for DMA channel."]
pub type INTA24_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTA0_SPEC, INTA24_A, O>;
impl<'a, const O: u8> INTA24_W<'a, O> {
    #[doc = "The DMA channel interrupt A is not active."]
    #[inline(always)]
    pub fn not_active(self) -> &'a mut W {
        self.variant(INTA24_A::NOT_ACTIVE)
    }
    #[doc = "The DMA channel interrupt A is active."]
    #[inline(always)]
    pub fn active(self) -> &'a mut W {
        self.variant(INTA24_A::ACTIVE)
    }
}
#[doc = "Field `INTA25` reader - Interrupt A status for DMA channel."]
pub type INTA25_R = crate::BitReader<INTA25_A>;
#[doc = "Interrupt A status for DMA channel.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum INTA25_A {
    #[doc = "0: The DMA channel interrupt A is not active."]
    NOT_ACTIVE = 0,
    #[doc = "1: The DMA channel interrupt A is active."]
    ACTIVE = 1,
}
impl From<INTA25_A> for bool {
    #[inline(always)]
    fn from(variant: INTA25_A) -> Self {
        variant as u8 != 0
    }
}
impl INTA25_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INTA25_A {
        match self.bits {
            false => INTA25_A::NOT_ACTIVE,
            true => INTA25_A::ACTIVE,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_ACTIVE`"]
    #[inline(always)]
    pub fn is_not_active(&self) -> bool {
        *self == INTA25_A::NOT_ACTIVE
    }
    #[doc = "Checks if the value of the field is `ACTIVE`"]
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        *self == INTA25_A::ACTIVE
    }
}
#[doc = "Field `INTA25` writer - Interrupt A status for DMA channel."]
pub type INTA25_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTA0_SPEC, INTA25_A, O>;
impl<'a, const O: u8> INTA25_W<'a, O> {
    #[doc = "The DMA channel interrupt A is not active."]
    #[inline(always)]
    pub fn not_active(self) -> &'a mut W {
        self.variant(INTA25_A::NOT_ACTIVE)
    }
    #[doc = "The DMA channel interrupt A is active."]
    #[inline(always)]
    pub fn active(self) -> &'a mut W {
        self.variant(INTA25_A::ACTIVE)
    }
}
#[doc = "Field `INTA26` reader - Interrupt A status for DMA channel."]
pub type INTA26_R = crate::BitReader<INTA26_A>;
#[doc = "Interrupt A status for DMA channel.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum INTA26_A {
    #[doc = "0: The DMA channel interrupt A is not active."]
    NOT_ACTIVE = 0,
    #[doc = "1: The DMA channel interrupt A is active."]
    ACTIVE = 1,
}
impl From<INTA26_A> for bool {
    #[inline(always)]
    fn from(variant: INTA26_A) -> Self {
        variant as u8 != 0
    }
}
impl INTA26_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INTA26_A {
        match self.bits {
            false => INTA26_A::NOT_ACTIVE,
            true => INTA26_A::ACTIVE,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_ACTIVE`"]
    #[inline(always)]
    pub fn is_not_active(&self) -> bool {
        *self == INTA26_A::NOT_ACTIVE
    }
    #[doc = "Checks if the value of the field is `ACTIVE`"]
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        *self == INTA26_A::ACTIVE
    }
}
#[doc = "Field `INTA26` writer - Interrupt A status for DMA channel."]
pub type INTA26_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTA0_SPEC, INTA26_A, O>;
impl<'a, const O: u8> INTA26_W<'a, O> {
    #[doc = "The DMA channel interrupt A is not active."]
    #[inline(always)]
    pub fn not_active(self) -> &'a mut W {
        self.variant(INTA26_A::NOT_ACTIVE)
    }
    #[doc = "The DMA channel interrupt A is active."]
    #[inline(always)]
    pub fn active(self) -> &'a mut W {
        self.variant(INTA26_A::ACTIVE)
    }
}
#[doc = "Field `INTA27` reader - Interrupt A status for DMA channel."]
pub type INTA27_R = crate::BitReader<INTA27_A>;
#[doc = "Interrupt A status for DMA channel.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum INTA27_A {
    #[doc = "0: The DMA channel interrupt A is not active."]
    NOT_ACTIVE = 0,
    #[doc = "1: The DMA channel interrupt A is active."]
    ACTIVE = 1,
}
impl From<INTA27_A> for bool {
    #[inline(always)]
    fn from(variant: INTA27_A) -> Self {
        variant as u8 != 0
    }
}
impl INTA27_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INTA27_A {
        match self.bits {
            false => INTA27_A::NOT_ACTIVE,
            true => INTA27_A::ACTIVE,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_ACTIVE`"]
    #[inline(always)]
    pub fn is_not_active(&self) -> bool {
        *self == INTA27_A::NOT_ACTIVE
    }
    #[doc = "Checks if the value of the field is `ACTIVE`"]
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        *self == INTA27_A::ACTIVE
    }
}
#[doc = "Field `INTA27` writer - Interrupt A status for DMA channel."]
pub type INTA27_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTA0_SPEC, INTA27_A, O>;
impl<'a, const O: u8> INTA27_W<'a, O> {
    #[doc = "The DMA channel interrupt A is not active."]
    #[inline(always)]
    pub fn not_active(self) -> &'a mut W {
        self.variant(INTA27_A::NOT_ACTIVE)
    }
    #[doc = "The DMA channel interrupt A is active."]
    #[inline(always)]
    pub fn active(self) -> &'a mut W {
        self.variant(INTA27_A::ACTIVE)
    }
}
#[doc = "Field `INTA28` reader - Interrupt A status for DMA channel."]
pub type INTA28_R = crate::BitReader<INTA28_A>;
#[doc = "Interrupt A status for DMA channel.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum INTA28_A {
    #[doc = "0: The DMA channel interrupt A is not active."]
    NOT_ACTIVE = 0,
    #[doc = "1: The DMA channel interrupt A is active."]
    ACTIVE = 1,
}
impl From<INTA28_A> for bool {
    #[inline(always)]
    fn from(variant: INTA28_A) -> Self {
        variant as u8 != 0
    }
}
impl INTA28_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INTA28_A {
        match self.bits {
            false => INTA28_A::NOT_ACTIVE,
            true => INTA28_A::ACTIVE,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_ACTIVE`"]
    #[inline(always)]
    pub fn is_not_active(&self) -> bool {
        *self == INTA28_A::NOT_ACTIVE
    }
    #[doc = "Checks if the value of the field is `ACTIVE`"]
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        *self == INTA28_A::ACTIVE
    }
}
#[doc = "Field `INTA28` writer - Interrupt A status for DMA channel."]
pub type INTA28_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTA0_SPEC, INTA28_A, O>;
impl<'a, const O: u8> INTA28_W<'a, O> {
    #[doc = "The DMA channel interrupt A is not active."]
    #[inline(always)]
    pub fn not_active(self) -> &'a mut W {
        self.variant(INTA28_A::NOT_ACTIVE)
    }
    #[doc = "The DMA channel interrupt A is active."]
    #[inline(always)]
    pub fn active(self) -> &'a mut W {
        self.variant(INTA28_A::ACTIVE)
    }
}
#[doc = "Field `INTA29` reader - Interrupt A status for DMA channel."]
pub type INTA29_R = crate::BitReader<INTA29_A>;
#[doc = "Interrupt A status for DMA channel.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum INTA29_A {
    #[doc = "0: The DMA channel interrupt A is not active."]
    NOT_ACTIVE = 0,
    #[doc = "1: The DMA channel interrupt A is active."]
    ACTIVE = 1,
}
impl From<INTA29_A> for bool {
    #[inline(always)]
    fn from(variant: INTA29_A) -> Self {
        variant as u8 != 0
    }
}
impl INTA29_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INTA29_A {
        match self.bits {
            false => INTA29_A::NOT_ACTIVE,
            true => INTA29_A::ACTIVE,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_ACTIVE`"]
    #[inline(always)]
    pub fn is_not_active(&self) -> bool {
        *self == INTA29_A::NOT_ACTIVE
    }
    #[doc = "Checks if the value of the field is `ACTIVE`"]
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        *self == INTA29_A::ACTIVE
    }
}
#[doc = "Field `INTA29` writer - Interrupt A status for DMA channel."]
pub type INTA29_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTA0_SPEC, INTA29_A, O>;
impl<'a, const O: u8> INTA29_W<'a, O> {
    #[doc = "The DMA channel interrupt A is not active."]
    #[inline(always)]
    pub fn not_active(self) -> &'a mut W {
        self.variant(INTA29_A::NOT_ACTIVE)
    }
    #[doc = "The DMA channel interrupt A is active."]
    #[inline(always)]
    pub fn active(self) -> &'a mut W {
        self.variant(INTA29_A::ACTIVE)
    }
}
#[doc = "Field `INTA30` reader - Interrupt A status for DMA channel."]
pub type INTA30_R = crate::BitReader<INTA30_A>;
#[doc = "Interrupt A status for DMA channel.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum INTA30_A {
    #[doc = "0: The DMA channel interrupt A is not active."]
    NOT_ACTIVE = 0,
    #[doc = "1: The DMA channel interrupt A is active."]
    ACTIVE = 1,
}
impl From<INTA30_A> for bool {
    #[inline(always)]
    fn from(variant: INTA30_A) -> Self {
        variant as u8 != 0
    }
}
impl INTA30_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INTA30_A {
        match self.bits {
            false => INTA30_A::NOT_ACTIVE,
            true => INTA30_A::ACTIVE,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_ACTIVE`"]
    #[inline(always)]
    pub fn is_not_active(&self) -> bool {
        *self == INTA30_A::NOT_ACTIVE
    }
    #[doc = "Checks if the value of the field is `ACTIVE`"]
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        *self == INTA30_A::ACTIVE
    }
}
#[doc = "Field `INTA30` writer - Interrupt A status for DMA channel."]
pub type INTA30_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTA0_SPEC, INTA30_A, O>;
impl<'a, const O: u8> INTA30_W<'a, O> {
    #[doc = "The DMA channel interrupt A is not active."]
    #[inline(always)]
    pub fn not_active(self) -> &'a mut W {
        self.variant(INTA30_A::NOT_ACTIVE)
    }
    #[doc = "The DMA channel interrupt A is active."]
    #[inline(always)]
    pub fn active(self) -> &'a mut W {
        self.variant(INTA30_A::ACTIVE)
    }
}
#[doc = "Field `INTA31` reader - Interrupt A status for DMA channel."]
pub type INTA31_R = crate::BitReader<INTA31_A>;
#[doc = "Interrupt A status for DMA channel.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum INTA31_A {
    #[doc = "0: The DMA channel interrupt A is not active."]
    NOT_ACTIVE = 0,
    #[doc = "1: The DMA channel interrupt A is active."]
    ACTIVE = 1,
}
impl From<INTA31_A> for bool {
    #[inline(always)]
    fn from(variant: INTA31_A) -> Self {
        variant as u8 != 0
    }
}
impl INTA31_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INTA31_A {
        match self.bits {
            false => INTA31_A::NOT_ACTIVE,
            true => INTA31_A::ACTIVE,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_ACTIVE`"]
    #[inline(always)]
    pub fn is_not_active(&self) -> bool {
        *self == INTA31_A::NOT_ACTIVE
    }
    #[doc = "Checks if the value of the field is `ACTIVE`"]
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        *self == INTA31_A::ACTIVE
    }
}
#[doc = "Field `INTA31` writer - Interrupt A status for DMA channel."]
pub type INTA31_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTA0_SPEC, INTA31_A, O>;
impl<'a, const O: u8> INTA31_W<'a, O> {
    #[doc = "The DMA channel interrupt A is not active."]
    #[inline(always)]
    pub fn not_active(self) -> &'a mut W {
        self.variant(INTA31_A::NOT_ACTIVE)
    }
    #[doc = "The DMA channel interrupt A is active."]
    #[inline(always)]
    pub fn active(self) -> &'a mut W {
        self.variant(INTA31_A::ACTIVE)
    }
}
impl R {
    #[doc = "Bit 0 - Interrupt A status for DMA channel."]
    #[inline(always)]
    pub fn inta0(&self) -> INTA0_R {
        INTA0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Interrupt A status for DMA channel."]
    #[inline(always)]
    pub fn inta1(&self) -> INTA1_R {
        INTA1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Interrupt A status for DMA channel."]
    #[inline(always)]
    pub fn inta2(&self) -> INTA2_R {
        INTA2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Interrupt A status for DMA channel."]
    #[inline(always)]
    pub fn inta3(&self) -> INTA3_R {
        INTA3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Interrupt A status for DMA channel."]
    #[inline(always)]
    pub fn inta4(&self) -> INTA4_R {
        INTA4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Interrupt A status for DMA channel."]
    #[inline(always)]
    pub fn inta5(&self) -> INTA5_R {
        INTA5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Interrupt A status for DMA channel."]
    #[inline(always)]
    pub fn inta6(&self) -> INTA6_R {
        INTA6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Interrupt A status for DMA channel."]
    #[inline(always)]
    pub fn inta7(&self) -> INTA7_R {
        INTA7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Interrupt A status for DMA channel."]
    #[inline(always)]
    pub fn inta8(&self) -> INTA8_R {
        INTA8_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Interrupt A status for DMA channel."]
    #[inline(always)]
    pub fn inta9(&self) -> INTA9_R {
        INTA9_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Interrupt A status for DMA channel."]
    #[inline(always)]
    pub fn inta10(&self) -> INTA10_R {
        INTA10_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Interrupt A status for DMA channel."]
    #[inline(always)]
    pub fn inta11(&self) -> INTA11_R {
        INTA11_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Interrupt A status for DMA channel."]
    #[inline(always)]
    pub fn inta12(&self) -> INTA12_R {
        INTA12_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Interrupt A status for DMA channel."]
    #[inline(always)]
    pub fn inta13(&self) -> INTA13_R {
        INTA13_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Interrupt A status for DMA channel."]
    #[inline(always)]
    pub fn inta14(&self) -> INTA14_R {
        INTA14_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Interrupt A status for DMA channel."]
    #[inline(always)]
    pub fn inta15(&self) -> INTA15_R {
        INTA15_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Interrupt A status for DMA channel."]
    #[inline(always)]
    pub fn inta16(&self) -> INTA16_R {
        INTA16_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Interrupt A status for DMA channel."]
    #[inline(always)]
    pub fn inta17(&self) -> INTA17_R {
        INTA17_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Interrupt A status for DMA channel."]
    #[inline(always)]
    pub fn inta18(&self) -> INTA18_R {
        INTA18_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Interrupt A status for DMA channel."]
    #[inline(always)]
    pub fn inta19(&self) -> INTA19_R {
        INTA19_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Interrupt A status for DMA channel."]
    #[inline(always)]
    pub fn inta20(&self) -> INTA20_R {
        INTA20_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Interrupt A status for DMA channel."]
    #[inline(always)]
    pub fn inta21(&self) -> INTA21_R {
        INTA21_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Interrupt A status for DMA channel."]
    #[inline(always)]
    pub fn inta22(&self) -> INTA22_R {
        INTA22_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Interrupt A status for DMA channel."]
    #[inline(always)]
    pub fn inta23(&self) -> INTA23_R {
        INTA23_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Interrupt A status for DMA channel."]
    #[inline(always)]
    pub fn inta24(&self) -> INTA24_R {
        INTA24_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Interrupt A status for DMA channel."]
    #[inline(always)]
    pub fn inta25(&self) -> INTA25_R {
        INTA25_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Interrupt A status for DMA channel."]
    #[inline(always)]
    pub fn inta26(&self) -> INTA26_R {
        INTA26_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Interrupt A status for DMA channel."]
    #[inline(always)]
    pub fn inta27(&self) -> INTA27_R {
        INTA27_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Interrupt A status for DMA channel."]
    #[inline(always)]
    pub fn inta28(&self) -> INTA28_R {
        INTA28_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Interrupt A status for DMA channel."]
    #[inline(always)]
    pub fn inta29(&self) -> INTA29_R {
        INTA29_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Interrupt A status for DMA channel."]
    #[inline(always)]
    pub fn inta30(&self) -> INTA30_R {
        INTA30_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Interrupt A status for DMA channel."]
    #[inline(always)]
    pub fn inta31(&self) -> INTA31_R {
        INTA31_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Interrupt A status for DMA channel."]
    #[inline(always)]
    #[must_use]
    pub fn inta0(&mut self) -> INTA0_W<0> {
        INTA0_W::new(self)
    }
    #[doc = "Bit 1 - Interrupt A status for DMA channel."]
    #[inline(always)]
    #[must_use]
    pub fn inta1(&mut self) -> INTA1_W<1> {
        INTA1_W::new(self)
    }
    #[doc = "Bit 2 - Interrupt A status for DMA channel."]
    #[inline(always)]
    #[must_use]
    pub fn inta2(&mut self) -> INTA2_W<2> {
        INTA2_W::new(self)
    }
    #[doc = "Bit 3 - Interrupt A status for DMA channel."]
    #[inline(always)]
    #[must_use]
    pub fn inta3(&mut self) -> INTA3_W<3> {
        INTA3_W::new(self)
    }
    #[doc = "Bit 4 - Interrupt A status for DMA channel."]
    #[inline(always)]
    #[must_use]
    pub fn inta4(&mut self) -> INTA4_W<4> {
        INTA4_W::new(self)
    }
    #[doc = "Bit 5 - Interrupt A status for DMA channel."]
    #[inline(always)]
    #[must_use]
    pub fn inta5(&mut self) -> INTA5_W<5> {
        INTA5_W::new(self)
    }
    #[doc = "Bit 6 - Interrupt A status for DMA channel."]
    #[inline(always)]
    #[must_use]
    pub fn inta6(&mut self) -> INTA6_W<6> {
        INTA6_W::new(self)
    }
    #[doc = "Bit 7 - Interrupt A status for DMA channel."]
    #[inline(always)]
    #[must_use]
    pub fn inta7(&mut self) -> INTA7_W<7> {
        INTA7_W::new(self)
    }
    #[doc = "Bit 8 - Interrupt A status for DMA channel."]
    #[inline(always)]
    #[must_use]
    pub fn inta8(&mut self) -> INTA8_W<8> {
        INTA8_W::new(self)
    }
    #[doc = "Bit 9 - Interrupt A status for DMA channel."]
    #[inline(always)]
    #[must_use]
    pub fn inta9(&mut self) -> INTA9_W<9> {
        INTA9_W::new(self)
    }
    #[doc = "Bit 10 - Interrupt A status for DMA channel."]
    #[inline(always)]
    #[must_use]
    pub fn inta10(&mut self) -> INTA10_W<10> {
        INTA10_W::new(self)
    }
    #[doc = "Bit 11 - Interrupt A status for DMA channel."]
    #[inline(always)]
    #[must_use]
    pub fn inta11(&mut self) -> INTA11_W<11> {
        INTA11_W::new(self)
    }
    #[doc = "Bit 12 - Interrupt A status for DMA channel."]
    #[inline(always)]
    #[must_use]
    pub fn inta12(&mut self) -> INTA12_W<12> {
        INTA12_W::new(self)
    }
    #[doc = "Bit 13 - Interrupt A status for DMA channel."]
    #[inline(always)]
    #[must_use]
    pub fn inta13(&mut self) -> INTA13_W<13> {
        INTA13_W::new(self)
    }
    #[doc = "Bit 14 - Interrupt A status for DMA channel."]
    #[inline(always)]
    #[must_use]
    pub fn inta14(&mut self) -> INTA14_W<14> {
        INTA14_W::new(self)
    }
    #[doc = "Bit 15 - Interrupt A status for DMA channel."]
    #[inline(always)]
    #[must_use]
    pub fn inta15(&mut self) -> INTA15_W<15> {
        INTA15_W::new(self)
    }
    #[doc = "Bit 16 - Interrupt A status for DMA channel."]
    #[inline(always)]
    #[must_use]
    pub fn inta16(&mut self) -> INTA16_W<16> {
        INTA16_W::new(self)
    }
    #[doc = "Bit 17 - Interrupt A status for DMA channel."]
    #[inline(always)]
    #[must_use]
    pub fn inta17(&mut self) -> INTA17_W<17> {
        INTA17_W::new(self)
    }
    #[doc = "Bit 18 - Interrupt A status for DMA channel."]
    #[inline(always)]
    #[must_use]
    pub fn inta18(&mut self) -> INTA18_W<18> {
        INTA18_W::new(self)
    }
    #[doc = "Bit 19 - Interrupt A status for DMA channel."]
    #[inline(always)]
    #[must_use]
    pub fn inta19(&mut self) -> INTA19_W<19> {
        INTA19_W::new(self)
    }
    #[doc = "Bit 20 - Interrupt A status for DMA channel."]
    #[inline(always)]
    #[must_use]
    pub fn inta20(&mut self) -> INTA20_W<20> {
        INTA20_W::new(self)
    }
    #[doc = "Bit 21 - Interrupt A status for DMA channel."]
    #[inline(always)]
    #[must_use]
    pub fn inta21(&mut self) -> INTA21_W<21> {
        INTA21_W::new(self)
    }
    #[doc = "Bit 22 - Interrupt A status for DMA channel."]
    #[inline(always)]
    #[must_use]
    pub fn inta22(&mut self) -> INTA22_W<22> {
        INTA22_W::new(self)
    }
    #[doc = "Bit 23 - Interrupt A status for DMA channel."]
    #[inline(always)]
    #[must_use]
    pub fn inta23(&mut self) -> INTA23_W<23> {
        INTA23_W::new(self)
    }
    #[doc = "Bit 24 - Interrupt A status for DMA channel."]
    #[inline(always)]
    #[must_use]
    pub fn inta24(&mut self) -> INTA24_W<24> {
        INTA24_W::new(self)
    }
    #[doc = "Bit 25 - Interrupt A status for DMA channel."]
    #[inline(always)]
    #[must_use]
    pub fn inta25(&mut self) -> INTA25_W<25> {
        INTA25_W::new(self)
    }
    #[doc = "Bit 26 - Interrupt A status for DMA channel."]
    #[inline(always)]
    #[must_use]
    pub fn inta26(&mut self) -> INTA26_W<26> {
        INTA26_W::new(self)
    }
    #[doc = "Bit 27 - Interrupt A status for DMA channel."]
    #[inline(always)]
    #[must_use]
    pub fn inta27(&mut self) -> INTA27_W<27> {
        INTA27_W::new(self)
    }
    #[doc = "Bit 28 - Interrupt A status for DMA channel."]
    #[inline(always)]
    #[must_use]
    pub fn inta28(&mut self) -> INTA28_W<28> {
        INTA28_W::new(self)
    }
    #[doc = "Bit 29 - Interrupt A status for DMA channel."]
    #[inline(always)]
    #[must_use]
    pub fn inta29(&mut self) -> INTA29_W<29> {
        INTA29_W::new(self)
    }
    #[doc = "Bit 30 - Interrupt A status for DMA channel."]
    #[inline(always)]
    #[must_use]
    pub fn inta30(&mut self) -> INTA30_W<30> {
        INTA30_W::new(self)
    }
    #[doc = "Bit 31 - Interrupt A status for DMA channel."]
    #[inline(always)]
    #[must_use]
    pub fn inta31(&mut self) -> INTA31_W<31> {
        INTA31_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt A status for all DMA channels\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [inta0](index.html) module"]
pub struct INTA0_SPEC;
impl crate::RegisterSpec for INTA0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [inta0::R](R) reader structure"]
impl crate::Readable for INTA0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [inta0::W](W) writer structure"]
impl crate::Writable for INTA0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets INTA0 to value 0"]
impl crate::Resettable for INTA0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
