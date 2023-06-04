#[doc = "Register `ERRINT0` reader"]
pub struct R(crate::R<ERRINT0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ERRINT0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ERRINT0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ERRINT0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ERRINT0` writer"]
pub struct W(crate::W<ERRINT0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ERRINT0_SPEC>;
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
impl From<crate::W<ERRINT0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ERRINT0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ERR0` reader - Error Interrupt flag for DMA channel."]
pub type ERR0_R = crate::BitReader<ERR0_A>;
#[doc = "Error Interrupt flag for DMA channel.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ERR0_A {
    #[doc = "0: The Error Interrupt is not active for DMA channel."]
    NOT_ACTIVE = 0,
    #[doc = "1: The Error Interrupt is pending for DMA channel."]
    PENDING = 1,
}
impl From<ERR0_A> for bool {
    #[inline(always)]
    fn from(variant: ERR0_A) -> Self {
        variant as u8 != 0
    }
}
impl ERR0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ERR0_A {
        match self.bits {
            false => ERR0_A::NOT_ACTIVE,
            true => ERR0_A::PENDING,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_ACTIVE`"]
    #[inline(always)]
    pub fn is_not_active(&self) -> bool {
        *self == ERR0_A::NOT_ACTIVE
    }
    #[doc = "Checks if the value of the field is `PENDING`"]
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        *self == ERR0_A::PENDING
    }
}
#[doc = "Field `ERR0` writer - Error Interrupt flag for DMA channel."]
pub type ERR0_W<'a, const O: u8> = crate::BitWriter<'a, u32, ERRINT0_SPEC, ERR0_A, O>;
impl<'a, const O: u8> ERR0_W<'a, O> {
    #[doc = "The Error Interrupt is not active for DMA channel."]
    #[inline(always)]
    pub fn not_active(self) -> &'a mut W {
        self.variant(ERR0_A::NOT_ACTIVE)
    }
    #[doc = "The Error Interrupt is pending for DMA channel."]
    #[inline(always)]
    pub fn pending(self) -> &'a mut W {
        self.variant(ERR0_A::PENDING)
    }
}
#[doc = "Field `ERR1` reader - Error Interrupt flag for DMA channel."]
pub type ERR1_R = crate::BitReader<ERR1_A>;
#[doc = "Error Interrupt flag for DMA channel.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ERR1_A {
    #[doc = "0: The Error Interrupt is not active for DMA channel."]
    NOT_ACTIVE = 0,
    #[doc = "1: The Error Interrupt is pending for DMA channel."]
    PENDING = 1,
}
impl From<ERR1_A> for bool {
    #[inline(always)]
    fn from(variant: ERR1_A) -> Self {
        variant as u8 != 0
    }
}
impl ERR1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ERR1_A {
        match self.bits {
            false => ERR1_A::NOT_ACTIVE,
            true => ERR1_A::PENDING,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_ACTIVE`"]
    #[inline(always)]
    pub fn is_not_active(&self) -> bool {
        *self == ERR1_A::NOT_ACTIVE
    }
    #[doc = "Checks if the value of the field is `PENDING`"]
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        *self == ERR1_A::PENDING
    }
}
#[doc = "Field `ERR1` writer - Error Interrupt flag for DMA channel."]
pub type ERR1_W<'a, const O: u8> = crate::BitWriter<'a, u32, ERRINT0_SPEC, ERR1_A, O>;
impl<'a, const O: u8> ERR1_W<'a, O> {
    #[doc = "The Error Interrupt is not active for DMA channel."]
    #[inline(always)]
    pub fn not_active(self) -> &'a mut W {
        self.variant(ERR1_A::NOT_ACTIVE)
    }
    #[doc = "The Error Interrupt is pending for DMA channel."]
    #[inline(always)]
    pub fn pending(self) -> &'a mut W {
        self.variant(ERR1_A::PENDING)
    }
}
#[doc = "Field `ERR2` reader - Error Interrupt flag for DMA channel."]
pub type ERR2_R = crate::BitReader<ERR2_A>;
#[doc = "Error Interrupt flag for DMA channel.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ERR2_A {
    #[doc = "0: The Error Interrupt is not active for DMA channel."]
    NOT_ACTIVE = 0,
    #[doc = "1: The Error Interrupt is pending for DMA channel."]
    PENDING = 1,
}
impl From<ERR2_A> for bool {
    #[inline(always)]
    fn from(variant: ERR2_A) -> Self {
        variant as u8 != 0
    }
}
impl ERR2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ERR2_A {
        match self.bits {
            false => ERR2_A::NOT_ACTIVE,
            true => ERR2_A::PENDING,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_ACTIVE`"]
    #[inline(always)]
    pub fn is_not_active(&self) -> bool {
        *self == ERR2_A::NOT_ACTIVE
    }
    #[doc = "Checks if the value of the field is `PENDING`"]
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        *self == ERR2_A::PENDING
    }
}
#[doc = "Field `ERR2` writer - Error Interrupt flag for DMA channel."]
pub type ERR2_W<'a, const O: u8> = crate::BitWriter<'a, u32, ERRINT0_SPEC, ERR2_A, O>;
impl<'a, const O: u8> ERR2_W<'a, O> {
    #[doc = "The Error Interrupt is not active for DMA channel."]
    #[inline(always)]
    pub fn not_active(self) -> &'a mut W {
        self.variant(ERR2_A::NOT_ACTIVE)
    }
    #[doc = "The Error Interrupt is pending for DMA channel."]
    #[inline(always)]
    pub fn pending(self) -> &'a mut W {
        self.variant(ERR2_A::PENDING)
    }
}
#[doc = "Field `ERR3` reader - Error Interrupt flag for DMA channel."]
pub type ERR3_R = crate::BitReader<ERR3_A>;
#[doc = "Error Interrupt flag for DMA channel.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ERR3_A {
    #[doc = "0: The Error Interrupt is not active for DMA channel."]
    NOT_ACTIVE = 0,
    #[doc = "1: The Error Interrupt is pending for DMA channel."]
    PENDING = 1,
}
impl From<ERR3_A> for bool {
    #[inline(always)]
    fn from(variant: ERR3_A) -> Self {
        variant as u8 != 0
    }
}
impl ERR3_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ERR3_A {
        match self.bits {
            false => ERR3_A::NOT_ACTIVE,
            true => ERR3_A::PENDING,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_ACTIVE`"]
    #[inline(always)]
    pub fn is_not_active(&self) -> bool {
        *self == ERR3_A::NOT_ACTIVE
    }
    #[doc = "Checks if the value of the field is `PENDING`"]
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        *self == ERR3_A::PENDING
    }
}
#[doc = "Field `ERR3` writer - Error Interrupt flag for DMA channel."]
pub type ERR3_W<'a, const O: u8> = crate::BitWriter<'a, u32, ERRINT0_SPEC, ERR3_A, O>;
impl<'a, const O: u8> ERR3_W<'a, O> {
    #[doc = "The Error Interrupt is not active for DMA channel."]
    #[inline(always)]
    pub fn not_active(self) -> &'a mut W {
        self.variant(ERR3_A::NOT_ACTIVE)
    }
    #[doc = "The Error Interrupt is pending for DMA channel."]
    #[inline(always)]
    pub fn pending(self) -> &'a mut W {
        self.variant(ERR3_A::PENDING)
    }
}
#[doc = "Field `ERR4` reader - Error Interrupt flag for DMA channel."]
pub type ERR4_R = crate::BitReader<ERR4_A>;
#[doc = "Error Interrupt flag for DMA channel.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ERR4_A {
    #[doc = "0: The Error Interrupt is not active for DMA channel."]
    NOT_ACTIVE = 0,
    #[doc = "1: The Error Interrupt is pending for DMA channel."]
    PENDING = 1,
}
impl From<ERR4_A> for bool {
    #[inline(always)]
    fn from(variant: ERR4_A) -> Self {
        variant as u8 != 0
    }
}
impl ERR4_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ERR4_A {
        match self.bits {
            false => ERR4_A::NOT_ACTIVE,
            true => ERR4_A::PENDING,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_ACTIVE`"]
    #[inline(always)]
    pub fn is_not_active(&self) -> bool {
        *self == ERR4_A::NOT_ACTIVE
    }
    #[doc = "Checks if the value of the field is `PENDING`"]
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        *self == ERR4_A::PENDING
    }
}
#[doc = "Field `ERR4` writer - Error Interrupt flag for DMA channel."]
pub type ERR4_W<'a, const O: u8> = crate::BitWriter<'a, u32, ERRINT0_SPEC, ERR4_A, O>;
impl<'a, const O: u8> ERR4_W<'a, O> {
    #[doc = "The Error Interrupt is not active for DMA channel."]
    #[inline(always)]
    pub fn not_active(self) -> &'a mut W {
        self.variant(ERR4_A::NOT_ACTIVE)
    }
    #[doc = "The Error Interrupt is pending for DMA channel."]
    #[inline(always)]
    pub fn pending(self) -> &'a mut W {
        self.variant(ERR4_A::PENDING)
    }
}
#[doc = "Field `ERR5` reader - Error Interrupt flag for DMA channel."]
pub type ERR5_R = crate::BitReader<ERR5_A>;
#[doc = "Error Interrupt flag for DMA channel.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ERR5_A {
    #[doc = "0: The Error Interrupt is not active for DMA channel."]
    NOT_ACTIVE = 0,
    #[doc = "1: The Error Interrupt is pending for DMA channel."]
    PENDING = 1,
}
impl From<ERR5_A> for bool {
    #[inline(always)]
    fn from(variant: ERR5_A) -> Self {
        variant as u8 != 0
    }
}
impl ERR5_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ERR5_A {
        match self.bits {
            false => ERR5_A::NOT_ACTIVE,
            true => ERR5_A::PENDING,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_ACTIVE`"]
    #[inline(always)]
    pub fn is_not_active(&self) -> bool {
        *self == ERR5_A::NOT_ACTIVE
    }
    #[doc = "Checks if the value of the field is `PENDING`"]
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        *self == ERR5_A::PENDING
    }
}
#[doc = "Field `ERR5` writer - Error Interrupt flag for DMA channel."]
pub type ERR5_W<'a, const O: u8> = crate::BitWriter<'a, u32, ERRINT0_SPEC, ERR5_A, O>;
impl<'a, const O: u8> ERR5_W<'a, O> {
    #[doc = "The Error Interrupt is not active for DMA channel."]
    #[inline(always)]
    pub fn not_active(self) -> &'a mut W {
        self.variant(ERR5_A::NOT_ACTIVE)
    }
    #[doc = "The Error Interrupt is pending for DMA channel."]
    #[inline(always)]
    pub fn pending(self) -> &'a mut W {
        self.variant(ERR5_A::PENDING)
    }
}
#[doc = "Field `ERR6` reader - Error Interrupt flag for DMA channel."]
pub type ERR6_R = crate::BitReader<ERR6_A>;
#[doc = "Error Interrupt flag for DMA channel.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ERR6_A {
    #[doc = "0: The Error Interrupt is not active for DMA channel."]
    NOT_ACTIVE = 0,
    #[doc = "1: The Error Interrupt is pending for DMA channel."]
    PENDING = 1,
}
impl From<ERR6_A> for bool {
    #[inline(always)]
    fn from(variant: ERR6_A) -> Self {
        variant as u8 != 0
    }
}
impl ERR6_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ERR6_A {
        match self.bits {
            false => ERR6_A::NOT_ACTIVE,
            true => ERR6_A::PENDING,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_ACTIVE`"]
    #[inline(always)]
    pub fn is_not_active(&self) -> bool {
        *self == ERR6_A::NOT_ACTIVE
    }
    #[doc = "Checks if the value of the field is `PENDING`"]
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        *self == ERR6_A::PENDING
    }
}
#[doc = "Field `ERR6` writer - Error Interrupt flag for DMA channel."]
pub type ERR6_W<'a, const O: u8> = crate::BitWriter<'a, u32, ERRINT0_SPEC, ERR6_A, O>;
impl<'a, const O: u8> ERR6_W<'a, O> {
    #[doc = "The Error Interrupt is not active for DMA channel."]
    #[inline(always)]
    pub fn not_active(self) -> &'a mut W {
        self.variant(ERR6_A::NOT_ACTIVE)
    }
    #[doc = "The Error Interrupt is pending for DMA channel."]
    #[inline(always)]
    pub fn pending(self) -> &'a mut W {
        self.variant(ERR6_A::PENDING)
    }
}
#[doc = "Field `ERR7` reader - Error Interrupt flag for DMA channel."]
pub type ERR7_R = crate::BitReader<ERR7_A>;
#[doc = "Error Interrupt flag for DMA channel.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ERR7_A {
    #[doc = "0: The Error Interrupt is not active for DMA channel."]
    NOT_ACTIVE = 0,
    #[doc = "1: The Error Interrupt is pending for DMA channel."]
    PENDING = 1,
}
impl From<ERR7_A> for bool {
    #[inline(always)]
    fn from(variant: ERR7_A) -> Self {
        variant as u8 != 0
    }
}
impl ERR7_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ERR7_A {
        match self.bits {
            false => ERR7_A::NOT_ACTIVE,
            true => ERR7_A::PENDING,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_ACTIVE`"]
    #[inline(always)]
    pub fn is_not_active(&self) -> bool {
        *self == ERR7_A::NOT_ACTIVE
    }
    #[doc = "Checks if the value of the field is `PENDING`"]
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        *self == ERR7_A::PENDING
    }
}
#[doc = "Field `ERR7` writer - Error Interrupt flag for DMA channel."]
pub type ERR7_W<'a, const O: u8> = crate::BitWriter<'a, u32, ERRINT0_SPEC, ERR7_A, O>;
impl<'a, const O: u8> ERR7_W<'a, O> {
    #[doc = "The Error Interrupt is not active for DMA channel."]
    #[inline(always)]
    pub fn not_active(self) -> &'a mut W {
        self.variant(ERR7_A::NOT_ACTIVE)
    }
    #[doc = "The Error Interrupt is pending for DMA channel."]
    #[inline(always)]
    pub fn pending(self) -> &'a mut W {
        self.variant(ERR7_A::PENDING)
    }
}
#[doc = "Field `ERR8` reader - Error Interrupt flag for DMA channel."]
pub type ERR8_R = crate::BitReader<ERR8_A>;
#[doc = "Error Interrupt flag for DMA channel.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ERR8_A {
    #[doc = "0: The Error Interrupt is not active for DMA channel."]
    NOT_ACTIVE = 0,
    #[doc = "1: The Error Interrupt is pending for DMA channel."]
    PENDING = 1,
}
impl From<ERR8_A> for bool {
    #[inline(always)]
    fn from(variant: ERR8_A) -> Self {
        variant as u8 != 0
    }
}
impl ERR8_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ERR8_A {
        match self.bits {
            false => ERR8_A::NOT_ACTIVE,
            true => ERR8_A::PENDING,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_ACTIVE`"]
    #[inline(always)]
    pub fn is_not_active(&self) -> bool {
        *self == ERR8_A::NOT_ACTIVE
    }
    #[doc = "Checks if the value of the field is `PENDING`"]
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        *self == ERR8_A::PENDING
    }
}
#[doc = "Field `ERR8` writer - Error Interrupt flag for DMA channel."]
pub type ERR8_W<'a, const O: u8> = crate::BitWriter<'a, u32, ERRINT0_SPEC, ERR8_A, O>;
impl<'a, const O: u8> ERR8_W<'a, O> {
    #[doc = "The Error Interrupt is not active for DMA channel."]
    #[inline(always)]
    pub fn not_active(self) -> &'a mut W {
        self.variant(ERR8_A::NOT_ACTIVE)
    }
    #[doc = "The Error Interrupt is pending for DMA channel."]
    #[inline(always)]
    pub fn pending(self) -> &'a mut W {
        self.variant(ERR8_A::PENDING)
    }
}
#[doc = "Field `ERR9` reader - Error Interrupt flag for DMA channel."]
pub type ERR9_R = crate::BitReader<ERR9_A>;
#[doc = "Error Interrupt flag for DMA channel.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ERR9_A {
    #[doc = "0: The Error Interrupt is not active for DMA channel."]
    NOT_ACTIVE = 0,
    #[doc = "1: The Error Interrupt is pending for DMA channel."]
    PENDING = 1,
}
impl From<ERR9_A> for bool {
    #[inline(always)]
    fn from(variant: ERR9_A) -> Self {
        variant as u8 != 0
    }
}
impl ERR9_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ERR9_A {
        match self.bits {
            false => ERR9_A::NOT_ACTIVE,
            true => ERR9_A::PENDING,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_ACTIVE`"]
    #[inline(always)]
    pub fn is_not_active(&self) -> bool {
        *self == ERR9_A::NOT_ACTIVE
    }
    #[doc = "Checks if the value of the field is `PENDING`"]
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        *self == ERR9_A::PENDING
    }
}
#[doc = "Field `ERR9` writer - Error Interrupt flag for DMA channel."]
pub type ERR9_W<'a, const O: u8> = crate::BitWriter<'a, u32, ERRINT0_SPEC, ERR9_A, O>;
impl<'a, const O: u8> ERR9_W<'a, O> {
    #[doc = "The Error Interrupt is not active for DMA channel."]
    #[inline(always)]
    pub fn not_active(self) -> &'a mut W {
        self.variant(ERR9_A::NOT_ACTIVE)
    }
    #[doc = "The Error Interrupt is pending for DMA channel."]
    #[inline(always)]
    pub fn pending(self) -> &'a mut W {
        self.variant(ERR9_A::PENDING)
    }
}
#[doc = "Field `ERR10` reader - Error Interrupt flag for DMA channel."]
pub type ERR10_R = crate::BitReader<ERR10_A>;
#[doc = "Error Interrupt flag for DMA channel.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ERR10_A {
    #[doc = "0: The Error Interrupt is not active for DMA channel."]
    NOT_ACTIVE = 0,
    #[doc = "1: The Error Interrupt is pending for DMA channel."]
    PENDING = 1,
}
impl From<ERR10_A> for bool {
    #[inline(always)]
    fn from(variant: ERR10_A) -> Self {
        variant as u8 != 0
    }
}
impl ERR10_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ERR10_A {
        match self.bits {
            false => ERR10_A::NOT_ACTIVE,
            true => ERR10_A::PENDING,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_ACTIVE`"]
    #[inline(always)]
    pub fn is_not_active(&self) -> bool {
        *self == ERR10_A::NOT_ACTIVE
    }
    #[doc = "Checks if the value of the field is `PENDING`"]
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        *self == ERR10_A::PENDING
    }
}
#[doc = "Field `ERR10` writer - Error Interrupt flag for DMA channel."]
pub type ERR10_W<'a, const O: u8> = crate::BitWriter<'a, u32, ERRINT0_SPEC, ERR10_A, O>;
impl<'a, const O: u8> ERR10_W<'a, O> {
    #[doc = "The Error Interrupt is not active for DMA channel."]
    #[inline(always)]
    pub fn not_active(self) -> &'a mut W {
        self.variant(ERR10_A::NOT_ACTIVE)
    }
    #[doc = "The Error Interrupt is pending for DMA channel."]
    #[inline(always)]
    pub fn pending(self) -> &'a mut W {
        self.variant(ERR10_A::PENDING)
    }
}
#[doc = "Field `ERR11` reader - Error Interrupt flag for DMA channel."]
pub type ERR11_R = crate::BitReader<ERR11_A>;
#[doc = "Error Interrupt flag for DMA channel.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ERR11_A {
    #[doc = "0: The Error Interrupt is not active for DMA channel."]
    NOT_ACTIVE = 0,
    #[doc = "1: The Error Interrupt is pending for DMA channel."]
    PENDING = 1,
}
impl From<ERR11_A> for bool {
    #[inline(always)]
    fn from(variant: ERR11_A) -> Self {
        variant as u8 != 0
    }
}
impl ERR11_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ERR11_A {
        match self.bits {
            false => ERR11_A::NOT_ACTIVE,
            true => ERR11_A::PENDING,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_ACTIVE`"]
    #[inline(always)]
    pub fn is_not_active(&self) -> bool {
        *self == ERR11_A::NOT_ACTIVE
    }
    #[doc = "Checks if the value of the field is `PENDING`"]
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        *self == ERR11_A::PENDING
    }
}
#[doc = "Field `ERR11` writer - Error Interrupt flag for DMA channel."]
pub type ERR11_W<'a, const O: u8> = crate::BitWriter<'a, u32, ERRINT0_SPEC, ERR11_A, O>;
impl<'a, const O: u8> ERR11_W<'a, O> {
    #[doc = "The Error Interrupt is not active for DMA channel."]
    #[inline(always)]
    pub fn not_active(self) -> &'a mut W {
        self.variant(ERR11_A::NOT_ACTIVE)
    }
    #[doc = "The Error Interrupt is pending for DMA channel."]
    #[inline(always)]
    pub fn pending(self) -> &'a mut W {
        self.variant(ERR11_A::PENDING)
    }
}
#[doc = "Field `ERR12` reader - Error Interrupt flag for DMA channel."]
pub type ERR12_R = crate::BitReader<ERR12_A>;
#[doc = "Error Interrupt flag for DMA channel.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ERR12_A {
    #[doc = "0: The Error Interrupt is not active for DMA channel."]
    NOT_ACTIVE = 0,
    #[doc = "1: The Error Interrupt is pending for DMA channel."]
    PENDING = 1,
}
impl From<ERR12_A> for bool {
    #[inline(always)]
    fn from(variant: ERR12_A) -> Self {
        variant as u8 != 0
    }
}
impl ERR12_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ERR12_A {
        match self.bits {
            false => ERR12_A::NOT_ACTIVE,
            true => ERR12_A::PENDING,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_ACTIVE`"]
    #[inline(always)]
    pub fn is_not_active(&self) -> bool {
        *self == ERR12_A::NOT_ACTIVE
    }
    #[doc = "Checks if the value of the field is `PENDING`"]
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        *self == ERR12_A::PENDING
    }
}
#[doc = "Field `ERR12` writer - Error Interrupt flag for DMA channel."]
pub type ERR12_W<'a, const O: u8> = crate::BitWriter<'a, u32, ERRINT0_SPEC, ERR12_A, O>;
impl<'a, const O: u8> ERR12_W<'a, O> {
    #[doc = "The Error Interrupt is not active for DMA channel."]
    #[inline(always)]
    pub fn not_active(self) -> &'a mut W {
        self.variant(ERR12_A::NOT_ACTIVE)
    }
    #[doc = "The Error Interrupt is pending for DMA channel."]
    #[inline(always)]
    pub fn pending(self) -> &'a mut W {
        self.variant(ERR12_A::PENDING)
    }
}
#[doc = "Field `ERR13` reader - Error Interrupt flag for DMA channel."]
pub type ERR13_R = crate::BitReader<ERR13_A>;
#[doc = "Error Interrupt flag for DMA channel.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ERR13_A {
    #[doc = "0: The Error Interrupt is not active for DMA channel."]
    NOT_ACTIVE = 0,
    #[doc = "1: The Error Interrupt is pending for DMA channel."]
    PENDING = 1,
}
impl From<ERR13_A> for bool {
    #[inline(always)]
    fn from(variant: ERR13_A) -> Self {
        variant as u8 != 0
    }
}
impl ERR13_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ERR13_A {
        match self.bits {
            false => ERR13_A::NOT_ACTIVE,
            true => ERR13_A::PENDING,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_ACTIVE`"]
    #[inline(always)]
    pub fn is_not_active(&self) -> bool {
        *self == ERR13_A::NOT_ACTIVE
    }
    #[doc = "Checks if the value of the field is `PENDING`"]
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        *self == ERR13_A::PENDING
    }
}
#[doc = "Field `ERR13` writer - Error Interrupt flag for DMA channel."]
pub type ERR13_W<'a, const O: u8> = crate::BitWriter<'a, u32, ERRINT0_SPEC, ERR13_A, O>;
impl<'a, const O: u8> ERR13_W<'a, O> {
    #[doc = "The Error Interrupt is not active for DMA channel."]
    #[inline(always)]
    pub fn not_active(self) -> &'a mut W {
        self.variant(ERR13_A::NOT_ACTIVE)
    }
    #[doc = "The Error Interrupt is pending for DMA channel."]
    #[inline(always)]
    pub fn pending(self) -> &'a mut W {
        self.variant(ERR13_A::PENDING)
    }
}
#[doc = "Field `ERR14` reader - Error Interrupt flag for DMA channel."]
pub type ERR14_R = crate::BitReader<ERR14_A>;
#[doc = "Error Interrupt flag for DMA channel.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ERR14_A {
    #[doc = "0: The Error Interrupt is not active for DMA channel."]
    NOT_ACTIVE = 0,
    #[doc = "1: The Error Interrupt is pending for DMA channel."]
    PENDING = 1,
}
impl From<ERR14_A> for bool {
    #[inline(always)]
    fn from(variant: ERR14_A) -> Self {
        variant as u8 != 0
    }
}
impl ERR14_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ERR14_A {
        match self.bits {
            false => ERR14_A::NOT_ACTIVE,
            true => ERR14_A::PENDING,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_ACTIVE`"]
    #[inline(always)]
    pub fn is_not_active(&self) -> bool {
        *self == ERR14_A::NOT_ACTIVE
    }
    #[doc = "Checks if the value of the field is `PENDING`"]
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        *self == ERR14_A::PENDING
    }
}
#[doc = "Field `ERR14` writer - Error Interrupt flag for DMA channel."]
pub type ERR14_W<'a, const O: u8> = crate::BitWriter<'a, u32, ERRINT0_SPEC, ERR14_A, O>;
impl<'a, const O: u8> ERR14_W<'a, O> {
    #[doc = "The Error Interrupt is not active for DMA channel."]
    #[inline(always)]
    pub fn not_active(self) -> &'a mut W {
        self.variant(ERR14_A::NOT_ACTIVE)
    }
    #[doc = "The Error Interrupt is pending for DMA channel."]
    #[inline(always)]
    pub fn pending(self) -> &'a mut W {
        self.variant(ERR14_A::PENDING)
    }
}
#[doc = "Field `ERR15` reader - Error Interrupt flag for DMA channel."]
pub type ERR15_R = crate::BitReader<ERR15_A>;
#[doc = "Error Interrupt flag for DMA channel.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ERR15_A {
    #[doc = "0: The Error Interrupt is not active for DMA channel."]
    NOT_ACTIVE = 0,
    #[doc = "1: The Error Interrupt is pending for DMA channel."]
    PENDING = 1,
}
impl From<ERR15_A> for bool {
    #[inline(always)]
    fn from(variant: ERR15_A) -> Self {
        variant as u8 != 0
    }
}
impl ERR15_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ERR15_A {
        match self.bits {
            false => ERR15_A::NOT_ACTIVE,
            true => ERR15_A::PENDING,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_ACTIVE`"]
    #[inline(always)]
    pub fn is_not_active(&self) -> bool {
        *self == ERR15_A::NOT_ACTIVE
    }
    #[doc = "Checks if the value of the field is `PENDING`"]
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        *self == ERR15_A::PENDING
    }
}
#[doc = "Field `ERR15` writer - Error Interrupt flag for DMA channel."]
pub type ERR15_W<'a, const O: u8> = crate::BitWriter<'a, u32, ERRINT0_SPEC, ERR15_A, O>;
impl<'a, const O: u8> ERR15_W<'a, O> {
    #[doc = "The Error Interrupt is not active for DMA channel."]
    #[inline(always)]
    pub fn not_active(self) -> &'a mut W {
        self.variant(ERR15_A::NOT_ACTIVE)
    }
    #[doc = "The Error Interrupt is pending for DMA channel."]
    #[inline(always)]
    pub fn pending(self) -> &'a mut W {
        self.variant(ERR15_A::PENDING)
    }
}
#[doc = "Field `ERR16` reader - Error Interrupt flag for DMA channel."]
pub type ERR16_R = crate::BitReader<ERR16_A>;
#[doc = "Error Interrupt flag for DMA channel.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ERR16_A {
    #[doc = "0: The Error Interrupt is not active for DMA channel."]
    NOT_ACTIVE = 0,
    #[doc = "1: The Error Interrupt is pending for DMA channel."]
    PENDING = 1,
}
impl From<ERR16_A> for bool {
    #[inline(always)]
    fn from(variant: ERR16_A) -> Self {
        variant as u8 != 0
    }
}
impl ERR16_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ERR16_A {
        match self.bits {
            false => ERR16_A::NOT_ACTIVE,
            true => ERR16_A::PENDING,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_ACTIVE`"]
    #[inline(always)]
    pub fn is_not_active(&self) -> bool {
        *self == ERR16_A::NOT_ACTIVE
    }
    #[doc = "Checks if the value of the field is `PENDING`"]
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        *self == ERR16_A::PENDING
    }
}
#[doc = "Field `ERR16` writer - Error Interrupt flag for DMA channel."]
pub type ERR16_W<'a, const O: u8> = crate::BitWriter<'a, u32, ERRINT0_SPEC, ERR16_A, O>;
impl<'a, const O: u8> ERR16_W<'a, O> {
    #[doc = "The Error Interrupt is not active for DMA channel."]
    #[inline(always)]
    pub fn not_active(self) -> &'a mut W {
        self.variant(ERR16_A::NOT_ACTIVE)
    }
    #[doc = "The Error Interrupt is pending for DMA channel."]
    #[inline(always)]
    pub fn pending(self) -> &'a mut W {
        self.variant(ERR16_A::PENDING)
    }
}
#[doc = "Field `ERR17` reader - Error Interrupt flag for DMA channel."]
pub type ERR17_R = crate::BitReader<ERR17_A>;
#[doc = "Error Interrupt flag for DMA channel.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ERR17_A {
    #[doc = "0: The Error Interrupt is not active for DMA channel."]
    NOT_ACTIVE = 0,
    #[doc = "1: The Error Interrupt is pending for DMA channel."]
    PENDING = 1,
}
impl From<ERR17_A> for bool {
    #[inline(always)]
    fn from(variant: ERR17_A) -> Self {
        variant as u8 != 0
    }
}
impl ERR17_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ERR17_A {
        match self.bits {
            false => ERR17_A::NOT_ACTIVE,
            true => ERR17_A::PENDING,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_ACTIVE`"]
    #[inline(always)]
    pub fn is_not_active(&self) -> bool {
        *self == ERR17_A::NOT_ACTIVE
    }
    #[doc = "Checks if the value of the field is `PENDING`"]
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        *self == ERR17_A::PENDING
    }
}
#[doc = "Field `ERR17` writer - Error Interrupt flag for DMA channel."]
pub type ERR17_W<'a, const O: u8> = crate::BitWriter<'a, u32, ERRINT0_SPEC, ERR17_A, O>;
impl<'a, const O: u8> ERR17_W<'a, O> {
    #[doc = "The Error Interrupt is not active for DMA channel."]
    #[inline(always)]
    pub fn not_active(self) -> &'a mut W {
        self.variant(ERR17_A::NOT_ACTIVE)
    }
    #[doc = "The Error Interrupt is pending for DMA channel."]
    #[inline(always)]
    pub fn pending(self) -> &'a mut W {
        self.variant(ERR17_A::PENDING)
    }
}
#[doc = "Field `ERR18` reader - Error Interrupt flag for DMA channel."]
pub type ERR18_R = crate::BitReader<ERR18_A>;
#[doc = "Error Interrupt flag for DMA channel.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ERR18_A {
    #[doc = "0: The Error Interrupt is not active for DMA channel."]
    NOT_ACTIVE = 0,
    #[doc = "1: The Error Interrupt is pending for DMA channel."]
    PENDING = 1,
}
impl From<ERR18_A> for bool {
    #[inline(always)]
    fn from(variant: ERR18_A) -> Self {
        variant as u8 != 0
    }
}
impl ERR18_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ERR18_A {
        match self.bits {
            false => ERR18_A::NOT_ACTIVE,
            true => ERR18_A::PENDING,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_ACTIVE`"]
    #[inline(always)]
    pub fn is_not_active(&self) -> bool {
        *self == ERR18_A::NOT_ACTIVE
    }
    #[doc = "Checks if the value of the field is `PENDING`"]
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        *self == ERR18_A::PENDING
    }
}
#[doc = "Field `ERR18` writer - Error Interrupt flag for DMA channel."]
pub type ERR18_W<'a, const O: u8> = crate::BitWriter<'a, u32, ERRINT0_SPEC, ERR18_A, O>;
impl<'a, const O: u8> ERR18_W<'a, O> {
    #[doc = "The Error Interrupt is not active for DMA channel."]
    #[inline(always)]
    pub fn not_active(self) -> &'a mut W {
        self.variant(ERR18_A::NOT_ACTIVE)
    }
    #[doc = "The Error Interrupt is pending for DMA channel."]
    #[inline(always)]
    pub fn pending(self) -> &'a mut W {
        self.variant(ERR18_A::PENDING)
    }
}
#[doc = "Field `ERR19` reader - Error Interrupt flag for DMA channel."]
pub type ERR19_R = crate::BitReader<ERR19_A>;
#[doc = "Error Interrupt flag for DMA channel.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ERR19_A {
    #[doc = "0: The Error Interrupt is not active for DMA channel."]
    NOT_ACTIVE = 0,
    #[doc = "1: The Error Interrupt is pending for DMA channel."]
    PENDING = 1,
}
impl From<ERR19_A> for bool {
    #[inline(always)]
    fn from(variant: ERR19_A) -> Self {
        variant as u8 != 0
    }
}
impl ERR19_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ERR19_A {
        match self.bits {
            false => ERR19_A::NOT_ACTIVE,
            true => ERR19_A::PENDING,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_ACTIVE`"]
    #[inline(always)]
    pub fn is_not_active(&self) -> bool {
        *self == ERR19_A::NOT_ACTIVE
    }
    #[doc = "Checks if the value of the field is `PENDING`"]
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        *self == ERR19_A::PENDING
    }
}
#[doc = "Field `ERR19` writer - Error Interrupt flag for DMA channel."]
pub type ERR19_W<'a, const O: u8> = crate::BitWriter<'a, u32, ERRINT0_SPEC, ERR19_A, O>;
impl<'a, const O: u8> ERR19_W<'a, O> {
    #[doc = "The Error Interrupt is not active for DMA channel."]
    #[inline(always)]
    pub fn not_active(self) -> &'a mut W {
        self.variant(ERR19_A::NOT_ACTIVE)
    }
    #[doc = "The Error Interrupt is pending for DMA channel."]
    #[inline(always)]
    pub fn pending(self) -> &'a mut W {
        self.variant(ERR19_A::PENDING)
    }
}
#[doc = "Field `ERR20` reader - Error Interrupt flag for DMA channel."]
pub type ERR20_R = crate::BitReader<ERR20_A>;
#[doc = "Error Interrupt flag for DMA channel.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ERR20_A {
    #[doc = "0: The Error Interrupt is not active for DMA channel."]
    NOT_ACTIVE = 0,
    #[doc = "1: The Error Interrupt is pending for DMA channel."]
    PENDING = 1,
}
impl From<ERR20_A> for bool {
    #[inline(always)]
    fn from(variant: ERR20_A) -> Self {
        variant as u8 != 0
    }
}
impl ERR20_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ERR20_A {
        match self.bits {
            false => ERR20_A::NOT_ACTIVE,
            true => ERR20_A::PENDING,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_ACTIVE`"]
    #[inline(always)]
    pub fn is_not_active(&self) -> bool {
        *self == ERR20_A::NOT_ACTIVE
    }
    #[doc = "Checks if the value of the field is `PENDING`"]
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        *self == ERR20_A::PENDING
    }
}
#[doc = "Field `ERR20` writer - Error Interrupt flag for DMA channel."]
pub type ERR20_W<'a, const O: u8> = crate::BitWriter<'a, u32, ERRINT0_SPEC, ERR20_A, O>;
impl<'a, const O: u8> ERR20_W<'a, O> {
    #[doc = "The Error Interrupt is not active for DMA channel."]
    #[inline(always)]
    pub fn not_active(self) -> &'a mut W {
        self.variant(ERR20_A::NOT_ACTIVE)
    }
    #[doc = "The Error Interrupt is pending for DMA channel."]
    #[inline(always)]
    pub fn pending(self) -> &'a mut W {
        self.variant(ERR20_A::PENDING)
    }
}
#[doc = "Field `ERR21` reader - Error Interrupt flag for DMA channel."]
pub type ERR21_R = crate::BitReader<ERR21_A>;
#[doc = "Error Interrupt flag for DMA channel.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ERR21_A {
    #[doc = "0: The Error Interrupt is not active for DMA channel."]
    NOT_ACTIVE = 0,
    #[doc = "1: The Error Interrupt is pending for DMA channel."]
    PENDING = 1,
}
impl From<ERR21_A> for bool {
    #[inline(always)]
    fn from(variant: ERR21_A) -> Self {
        variant as u8 != 0
    }
}
impl ERR21_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ERR21_A {
        match self.bits {
            false => ERR21_A::NOT_ACTIVE,
            true => ERR21_A::PENDING,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_ACTIVE`"]
    #[inline(always)]
    pub fn is_not_active(&self) -> bool {
        *self == ERR21_A::NOT_ACTIVE
    }
    #[doc = "Checks if the value of the field is `PENDING`"]
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        *self == ERR21_A::PENDING
    }
}
#[doc = "Field `ERR21` writer - Error Interrupt flag for DMA channel."]
pub type ERR21_W<'a, const O: u8> = crate::BitWriter<'a, u32, ERRINT0_SPEC, ERR21_A, O>;
impl<'a, const O: u8> ERR21_W<'a, O> {
    #[doc = "The Error Interrupt is not active for DMA channel."]
    #[inline(always)]
    pub fn not_active(self) -> &'a mut W {
        self.variant(ERR21_A::NOT_ACTIVE)
    }
    #[doc = "The Error Interrupt is pending for DMA channel."]
    #[inline(always)]
    pub fn pending(self) -> &'a mut W {
        self.variant(ERR21_A::PENDING)
    }
}
#[doc = "Field `ERR22` reader - Error Interrupt flag for DMA channel."]
pub type ERR22_R = crate::BitReader<ERR22_A>;
#[doc = "Error Interrupt flag for DMA channel.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ERR22_A {
    #[doc = "0: The Error Interrupt is not active for DMA channel."]
    NOT_ACTIVE = 0,
    #[doc = "1: The Error Interrupt is pending for DMA channel."]
    PENDING = 1,
}
impl From<ERR22_A> for bool {
    #[inline(always)]
    fn from(variant: ERR22_A) -> Self {
        variant as u8 != 0
    }
}
impl ERR22_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ERR22_A {
        match self.bits {
            false => ERR22_A::NOT_ACTIVE,
            true => ERR22_A::PENDING,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_ACTIVE`"]
    #[inline(always)]
    pub fn is_not_active(&self) -> bool {
        *self == ERR22_A::NOT_ACTIVE
    }
    #[doc = "Checks if the value of the field is `PENDING`"]
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        *self == ERR22_A::PENDING
    }
}
#[doc = "Field `ERR22` writer - Error Interrupt flag for DMA channel."]
pub type ERR22_W<'a, const O: u8> = crate::BitWriter<'a, u32, ERRINT0_SPEC, ERR22_A, O>;
impl<'a, const O: u8> ERR22_W<'a, O> {
    #[doc = "The Error Interrupt is not active for DMA channel."]
    #[inline(always)]
    pub fn not_active(self) -> &'a mut W {
        self.variant(ERR22_A::NOT_ACTIVE)
    }
    #[doc = "The Error Interrupt is pending for DMA channel."]
    #[inline(always)]
    pub fn pending(self) -> &'a mut W {
        self.variant(ERR22_A::PENDING)
    }
}
#[doc = "Field `ERR23` reader - Error Interrupt flag for DMA channel."]
pub type ERR23_R = crate::BitReader<ERR23_A>;
#[doc = "Error Interrupt flag for DMA channel.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ERR23_A {
    #[doc = "0: The Error Interrupt is not active for DMA channel."]
    NOT_ACTIVE = 0,
    #[doc = "1: The Error Interrupt is pending for DMA channel."]
    PENDING = 1,
}
impl From<ERR23_A> for bool {
    #[inline(always)]
    fn from(variant: ERR23_A) -> Self {
        variant as u8 != 0
    }
}
impl ERR23_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ERR23_A {
        match self.bits {
            false => ERR23_A::NOT_ACTIVE,
            true => ERR23_A::PENDING,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_ACTIVE`"]
    #[inline(always)]
    pub fn is_not_active(&self) -> bool {
        *self == ERR23_A::NOT_ACTIVE
    }
    #[doc = "Checks if the value of the field is `PENDING`"]
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        *self == ERR23_A::PENDING
    }
}
#[doc = "Field `ERR23` writer - Error Interrupt flag for DMA channel."]
pub type ERR23_W<'a, const O: u8> = crate::BitWriter<'a, u32, ERRINT0_SPEC, ERR23_A, O>;
impl<'a, const O: u8> ERR23_W<'a, O> {
    #[doc = "The Error Interrupt is not active for DMA channel."]
    #[inline(always)]
    pub fn not_active(self) -> &'a mut W {
        self.variant(ERR23_A::NOT_ACTIVE)
    }
    #[doc = "The Error Interrupt is pending for DMA channel."]
    #[inline(always)]
    pub fn pending(self) -> &'a mut W {
        self.variant(ERR23_A::PENDING)
    }
}
#[doc = "Field `ERR24` reader - Error Interrupt flag for DMA channel."]
pub type ERR24_R = crate::BitReader<ERR24_A>;
#[doc = "Error Interrupt flag for DMA channel.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ERR24_A {
    #[doc = "0: The Error Interrupt is not active for DMA channel."]
    NOT_ACTIVE = 0,
    #[doc = "1: The Error Interrupt is pending for DMA channel."]
    PENDING = 1,
}
impl From<ERR24_A> for bool {
    #[inline(always)]
    fn from(variant: ERR24_A) -> Self {
        variant as u8 != 0
    }
}
impl ERR24_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ERR24_A {
        match self.bits {
            false => ERR24_A::NOT_ACTIVE,
            true => ERR24_A::PENDING,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_ACTIVE`"]
    #[inline(always)]
    pub fn is_not_active(&self) -> bool {
        *self == ERR24_A::NOT_ACTIVE
    }
    #[doc = "Checks if the value of the field is `PENDING`"]
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        *self == ERR24_A::PENDING
    }
}
#[doc = "Field `ERR24` writer - Error Interrupt flag for DMA channel."]
pub type ERR24_W<'a, const O: u8> = crate::BitWriter<'a, u32, ERRINT0_SPEC, ERR24_A, O>;
impl<'a, const O: u8> ERR24_W<'a, O> {
    #[doc = "The Error Interrupt is not active for DMA channel."]
    #[inline(always)]
    pub fn not_active(self) -> &'a mut W {
        self.variant(ERR24_A::NOT_ACTIVE)
    }
    #[doc = "The Error Interrupt is pending for DMA channel."]
    #[inline(always)]
    pub fn pending(self) -> &'a mut W {
        self.variant(ERR24_A::PENDING)
    }
}
#[doc = "Field `ERR25` reader - Error Interrupt flag for DMA channel."]
pub type ERR25_R = crate::BitReader<ERR25_A>;
#[doc = "Error Interrupt flag for DMA channel.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ERR25_A {
    #[doc = "0: The Error Interrupt is not active for DMA channel."]
    NOT_ACTIVE = 0,
    #[doc = "1: The Error Interrupt is pending for DMA channel."]
    PENDING = 1,
}
impl From<ERR25_A> for bool {
    #[inline(always)]
    fn from(variant: ERR25_A) -> Self {
        variant as u8 != 0
    }
}
impl ERR25_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ERR25_A {
        match self.bits {
            false => ERR25_A::NOT_ACTIVE,
            true => ERR25_A::PENDING,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_ACTIVE`"]
    #[inline(always)]
    pub fn is_not_active(&self) -> bool {
        *self == ERR25_A::NOT_ACTIVE
    }
    #[doc = "Checks if the value of the field is `PENDING`"]
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        *self == ERR25_A::PENDING
    }
}
#[doc = "Field `ERR25` writer - Error Interrupt flag for DMA channel."]
pub type ERR25_W<'a, const O: u8> = crate::BitWriter<'a, u32, ERRINT0_SPEC, ERR25_A, O>;
impl<'a, const O: u8> ERR25_W<'a, O> {
    #[doc = "The Error Interrupt is not active for DMA channel."]
    #[inline(always)]
    pub fn not_active(self) -> &'a mut W {
        self.variant(ERR25_A::NOT_ACTIVE)
    }
    #[doc = "The Error Interrupt is pending for DMA channel."]
    #[inline(always)]
    pub fn pending(self) -> &'a mut W {
        self.variant(ERR25_A::PENDING)
    }
}
#[doc = "Field `ERR26` reader - Error Interrupt flag for DMA channel."]
pub type ERR26_R = crate::BitReader<ERR26_A>;
#[doc = "Error Interrupt flag for DMA channel.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ERR26_A {
    #[doc = "0: The Error Interrupt is not active for DMA channel."]
    NOT_ACTIVE = 0,
    #[doc = "1: The Error Interrupt is pending for DMA channel."]
    PENDING = 1,
}
impl From<ERR26_A> for bool {
    #[inline(always)]
    fn from(variant: ERR26_A) -> Self {
        variant as u8 != 0
    }
}
impl ERR26_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ERR26_A {
        match self.bits {
            false => ERR26_A::NOT_ACTIVE,
            true => ERR26_A::PENDING,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_ACTIVE`"]
    #[inline(always)]
    pub fn is_not_active(&self) -> bool {
        *self == ERR26_A::NOT_ACTIVE
    }
    #[doc = "Checks if the value of the field is `PENDING`"]
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        *self == ERR26_A::PENDING
    }
}
#[doc = "Field `ERR26` writer - Error Interrupt flag for DMA channel."]
pub type ERR26_W<'a, const O: u8> = crate::BitWriter<'a, u32, ERRINT0_SPEC, ERR26_A, O>;
impl<'a, const O: u8> ERR26_W<'a, O> {
    #[doc = "The Error Interrupt is not active for DMA channel."]
    #[inline(always)]
    pub fn not_active(self) -> &'a mut W {
        self.variant(ERR26_A::NOT_ACTIVE)
    }
    #[doc = "The Error Interrupt is pending for DMA channel."]
    #[inline(always)]
    pub fn pending(self) -> &'a mut W {
        self.variant(ERR26_A::PENDING)
    }
}
#[doc = "Field `ERR27` reader - Error Interrupt flag for DMA channel."]
pub type ERR27_R = crate::BitReader<ERR27_A>;
#[doc = "Error Interrupt flag for DMA channel.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ERR27_A {
    #[doc = "0: The Error Interrupt is not active for DMA channel."]
    NOT_ACTIVE = 0,
    #[doc = "1: The Error Interrupt is pending for DMA channel."]
    PENDING = 1,
}
impl From<ERR27_A> for bool {
    #[inline(always)]
    fn from(variant: ERR27_A) -> Self {
        variant as u8 != 0
    }
}
impl ERR27_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ERR27_A {
        match self.bits {
            false => ERR27_A::NOT_ACTIVE,
            true => ERR27_A::PENDING,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_ACTIVE`"]
    #[inline(always)]
    pub fn is_not_active(&self) -> bool {
        *self == ERR27_A::NOT_ACTIVE
    }
    #[doc = "Checks if the value of the field is `PENDING`"]
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        *self == ERR27_A::PENDING
    }
}
#[doc = "Field `ERR27` writer - Error Interrupt flag for DMA channel."]
pub type ERR27_W<'a, const O: u8> = crate::BitWriter<'a, u32, ERRINT0_SPEC, ERR27_A, O>;
impl<'a, const O: u8> ERR27_W<'a, O> {
    #[doc = "The Error Interrupt is not active for DMA channel."]
    #[inline(always)]
    pub fn not_active(self) -> &'a mut W {
        self.variant(ERR27_A::NOT_ACTIVE)
    }
    #[doc = "The Error Interrupt is pending for DMA channel."]
    #[inline(always)]
    pub fn pending(self) -> &'a mut W {
        self.variant(ERR27_A::PENDING)
    }
}
#[doc = "Field `ERR28` reader - Error Interrupt flag for DMA channel."]
pub type ERR28_R = crate::BitReader<ERR28_A>;
#[doc = "Error Interrupt flag for DMA channel.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ERR28_A {
    #[doc = "0: The Error Interrupt is not active for DMA channel."]
    NOT_ACTIVE = 0,
    #[doc = "1: The Error Interrupt is pending for DMA channel."]
    PENDING = 1,
}
impl From<ERR28_A> for bool {
    #[inline(always)]
    fn from(variant: ERR28_A) -> Self {
        variant as u8 != 0
    }
}
impl ERR28_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ERR28_A {
        match self.bits {
            false => ERR28_A::NOT_ACTIVE,
            true => ERR28_A::PENDING,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_ACTIVE`"]
    #[inline(always)]
    pub fn is_not_active(&self) -> bool {
        *self == ERR28_A::NOT_ACTIVE
    }
    #[doc = "Checks if the value of the field is `PENDING`"]
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        *self == ERR28_A::PENDING
    }
}
#[doc = "Field `ERR28` writer - Error Interrupt flag for DMA channel."]
pub type ERR28_W<'a, const O: u8> = crate::BitWriter<'a, u32, ERRINT0_SPEC, ERR28_A, O>;
impl<'a, const O: u8> ERR28_W<'a, O> {
    #[doc = "The Error Interrupt is not active for DMA channel."]
    #[inline(always)]
    pub fn not_active(self) -> &'a mut W {
        self.variant(ERR28_A::NOT_ACTIVE)
    }
    #[doc = "The Error Interrupt is pending for DMA channel."]
    #[inline(always)]
    pub fn pending(self) -> &'a mut W {
        self.variant(ERR28_A::PENDING)
    }
}
#[doc = "Field `ERR29` reader - Error Interrupt flag for DMA channel."]
pub type ERR29_R = crate::BitReader<ERR29_A>;
#[doc = "Error Interrupt flag for DMA channel.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ERR29_A {
    #[doc = "0: The Error Interrupt is not active for DMA channel."]
    NOT_ACTIVE = 0,
    #[doc = "1: The Error Interrupt is pending for DMA channel."]
    PENDING = 1,
}
impl From<ERR29_A> for bool {
    #[inline(always)]
    fn from(variant: ERR29_A) -> Self {
        variant as u8 != 0
    }
}
impl ERR29_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ERR29_A {
        match self.bits {
            false => ERR29_A::NOT_ACTIVE,
            true => ERR29_A::PENDING,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_ACTIVE`"]
    #[inline(always)]
    pub fn is_not_active(&self) -> bool {
        *self == ERR29_A::NOT_ACTIVE
    }
    #[doc = "Checks if the value of the field is `PENDING`"]
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        *self == ERR29_A::PENDING
    }
}
#[doc = "Field `ERR29` writer - Error Interrupt flag for DMA channel."]
pub type ERR29_W<'a, const O: u8> = crate::BitWriter<'a, u32, ERRINT0_SPEC, ERR29_A, O>;
impl<'a, const O: u8> ERR29_W<'a, O> {
    #[doc = "The Error Interrupt is not active for DMA channel."]
    #[inline(always)]
    pub fn not_active(self) -> &'a mut W {
        self.variant(ERR29_A::NOT_ACTIVE)
    }
    #[doc = "The Error Interrupt is pending for DMA channel."]
    #[inline(always)]
    pub fn pending(self) -> &'a mut W {
        self.variant(ERR29_A::PENDING)
    }
}
#[doc = "Field `ERR30` reader - Error Interrupt flag for DMA channel."]
pub type ERR30_R = crate::BitReader<ERR30_A>;
#[doc = "Error Interrupt flag for DMA channel.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ERR30_A {
    #[doc = "0: The Error Interrupt is not active for DMA channel."]
    NOT_ACTIVE = 0,
    #[doc = "1: The Error Interrupt is pending for DMA channel."]
    PENDING = 1,
}
impl From<ERR30_A> for bool {
    #[inline(always)]
    fn from(variant: ERR30_A) -> Self {
        variant as u8 != 0
    }
}
impl ERR30_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ERR30_A {
        match self.bits {
            false => ERR30_A::NOT_ACTIVE,
            true => ERR30_A::PENDING,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_ACTIVE`"]
    #[inline(always)]
    pub fn is_not_active(&self) -> bool {
        *self == ERR30_A::NOT_ACTIVE
    }
    #[doc = "Checks if the value of the field is `PENDING`"]
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        *self == ERR30_A::PENDING
    }
}
#[doc = "Field `ERR30` writer - Error Interrupt flag for DMA channel."]
pub type ERR30_W<'a, const O: u8> = crate::BitWriter<'a, u32, ERRINT0_SPEC, ERR30_A, O>;
impl<'a, const O: u8> ERR30_W<'a, O> {
    #[doc = "The Error Interrupt is not active for DMA channel."]
    #[inline(always)]
    pub fn not_active(self) -> &'a mut W {
        self.variant(ERR30_A::NOT_ACTIVE)
    }
    #[doc = "The Error Interrupt is pending for DMA channel."]
    #[inline(always)]
    pub fn pending(self) -> &'a mut W {
        self.variant(ERR30_A::PENDING)
    }
}
#[doc = "Field `ERR31` reader - Error Interrupt flag for DMA channel."]
pub type ERR31_R = crate::BitReader<ERR31_A>;
#[doc = "Error Interrupt flag for DMA channel.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ERR31_A {
    #[doc = "0: The Error Interrupt is not active for DMA channel."]
    NOT_ACTIVE = 0,
    #[doc = "1: The Error Interrupt is pending for DMA channel."]
    PENDING = 1,
}
impl From<ERR31_A> for bool {
    #[inline(always)]
    fn from(variant: ERR31_A) -> Self {
        variant as u8 != 0
    }
}
impl ERR31_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ERR31_A {
        match self.bits {
            false => ERR31_A::NOT_ACTIVE,
            true => ERR31_A::PENDING,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_ACTIVE`"]
    #[inline(always)]
    pub fn is_not_active(&self) -> bool {
        *self == ERR31_A::NOT_ACTIVE
    }
    #[doc = "Checks if the value of the field is `PENDING`"]
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        *self == ERR31_A::PENDING
    }
}
#[doc = "Field `ERR31` writer - Error Interrupt flag for DMA channel."]
pub type ERR31_W<'a, const O: u8> = crate::BitWriter<'a, u32, ERRINT0_SPEC, ERR31_A, O>;
impl<'a, const O: u8> ERR31_W<'a, O> {
    #[doc = "The Error Interrupt is not active for DMA channel."]
    #[inline(always)]
    pub fn not_active(self) -> &'a mut W {
        self.variant(ERR31_A::NOT_ACTIVE)
    }
    #[doc = "The Error Interrupt is pending for DMA channel."]
    #[inline(always)]
    pub fn pending(self) -> &'a mut W {
        self.variant(ERR31_A::PENDING)
    }
}
impl R {
    #[doc = "Bit 0 - Error Interrupt flag for DMA channel."]
    #[inline(always)]
    pub fn err0(&self) -> ERR0_R {
        ERR0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Error Interrupt flag for DMA channel."]
    #[inline(always)]
    pub fn err1(&self) -> ERR1_R {
        ERR1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Error Interrupt flag for DMA channel."]
    #[inline(always)]
    pub fn err2(&self) -> ERR2_R {
        ERR2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Error Interrupt flag for DMA channel."]
    #[inline(always)]
    pub fn err3(&self) -> ERR3_R {
        ERR3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Error Interrupt flag for DMA channel."]
    #[inline(always)]
    pub fn err4(&self) -> ERR4_R {
        ERR4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Error Interrupt flag for DMA channel."]
    #[inline(always)]
    pub fn err5(&self) -> ERR5_R {
        ERR5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Error Interrupt flag for DMA channel."]
    #[inline(always)]
    pub fn err6(&self) -> ERR6_R {
        ERR6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Error Interrupt flag for DMA channel."]
    #[inline(always)]
    pub fn err7(&self) -> ERR7_R {
        ERR7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Error Interrupt flag for DMA channel."]
    #[inline(always)]
    pub fn err8(&self) -> ERR8_R {
        ERR8_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Error Interrupt flag for DMA channel."]
    #[inline(always)]
    pub fn err9(&self) -> ERR9_R {
        ERR9_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Error Interrupt flag for DMA channel."]
    #[inline(always)]
    pub fn err10(&self) -> ERR10_R {
        ERR10_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Error Interrupt flag for DMA channel."]
    #[inline(always)]
    pub fn err11(&self) -> ERR11_R {
        ERR11_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Error Interrupt flag for DMA channel."]
    #[inline(always)]
    pub fn err12(&self) -> ERR12_R {
        ERR12_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Error Interrupt flag for DMA channel."]
    #[inline(always)]
    pub fn err13(&self) -> ERR13_R {
        ERR13_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Error Interrupt flag for DMA channel."]
    #[inline(always)]
    pub fn err14(&self) -> ERR14_R {
        ERR14_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Error Interrupt flag for DMA channel."]
    #[inline(always)]
    pub fn err15(&self) -> ERR15_R {
        ERR15_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Error Interrupt flag for DMA channel."]
    #[inline(always)]
    pub fn err16(&self) -> ERR16_R {
        ERR16_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Error Interrupt flag for DMA channel."]
    #[inline(always)]
    pub fn err17(&self) -> ERR17_R {
        ERR17_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Error Interrupt flag for DMA channel."]
    #[inline(always)]
    pub fn err18(&self) -> ERR18_R {
        ERR18_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Error Interrupt flag for DMA channel."]
    #[inline(always)]
    pub fn err19(&self) -> ERR19_R {
        ERR19_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Error Interrupt flag for DMA channel."]
    #[inline(always)]
    pub fn err20(&self) -> ERR20_R {
        ERR20_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Error Interrupt flag for DMA channel."]
    #[inline(always)]
    pub fn err21(&self) -> ERR21_R {
        ERR21_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Error Interrupt flag for DMA channel."]
    #[inline(always)]
    pub fn err22(&self) -> ERR22_R {
        ERR22_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Error Interrupt flag for DMA channel."]
    #[inline(always)]
    pub fn err23(&self) -> ERR23_R {
        ERR23_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Error Interrupt flag for DMA channel."]
    #[inline(always)]
    pub fn err24(&self) -> ERR24_R {
        ERR24_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Error Interrupt flag for DMA channel."]
    #[inline(always)]
    pub fn err25(&self) -> ERR25_R {
        ERR25_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Error Interrupt flag for DMA channel."]
    #[inline(always)]
    pub fn err26(&self) -> ERR26_R {
        ERR26_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Error Interrupt flag for DMA channel."]
    #[inline(always)]
    pub fn err27(&self) -> ERR27_R {
        ERR27_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Error Interrupt flag for DMA channel."]
    #[inline(always)]
    pub fn err28(&self) -> ERR28_R {
        ERR28_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Error Interrupt flag for DMA channel."]
    #[inline(always)]
    pub fn err29(&self) -> ERR29_R {
        ERR29_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Error Interrupt flag for DMA channel."]
    #[inline(always)]
    pub fn err30(&self) -> ERR30_R {
        ERR30_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Error Interrupt flag for DMA channel."]
    #[inline(always)]
    pub fn err31(&self) -> ERR31_R {
        ERR31_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Error Interrupt flag for DMA channel."]
    #[inline(always)]
    #[must_use]
    pub fn err0(&mut self) -> ERR0_W<0> {
        ERR0_W::new(self)
    }
    #[doc = "Bit 1 - Error Interrupt flag for DMA channel."]
    #[inline(always)]
    #[must_use]
    pub fn err1(&mut self) -> ERR1_W<1> {
        ERR1_W::new(self)
    }
    #[doc = "Bit 2 - Error Interrupt flag for DMA channel."]
    #[inline(always)]
    #[must_use]
    pub fn err2(&mut self) -> ERR2_W<2> {
        ERR2_W::new(self)
    }
    #[doc = "Bit 3 - Error Interrupt flag for DMA channel."]
    #[inline(always)]
    #[must_use]
    pub fn err3(&mut self) -> ERR3_W<3> {
        ERR3_W::new(self)
    }
    #[doc = "Bit 4 - Error Interrupt flag for DMA channel."]
    #[inline(always)]
    #[must_use]
    pub fn err4(&mut self) -> ERR4_W<4> {
        ERR4_W::new(self)
    }
    #[doc = "Bit 5 - Error Interrupt flag for DMA channel."]
    #[inline(always)]
    #[must_use]
    pub fn err5(&mut self) -> ERR5_W<5> {
        ERR5_W::new(self)
    }
    #[doc = "Bit 6 - Error Interrupt flag for DMA channel."]
    #[inline(always)]
    #[must_use]
    pub fn err6(&mut self) -> ERR6_W<6> {
        ERR6_W::new(self)
    }
    #[doc = "Bit 7 - Error Interrupt flag for DMA channel."]
    #[inline(always)]
    #[must_use]
    pub fn err7(&mut self) -> ERR7_W<7> {
        ERR7_W::new(self)
    }
    #[doc = "Bit 8 - Error Interrupt flag for DMA channel."]
    #[inline(always)]
    #[must_use]
    pub fn err8(&mut self) -> ERR8_W<8> {
        ERR8_W::new(self)
    }
    #[doc = "Bit 9 - Error Interrupt flag for DMA channel."]
    #[inline(always)]
    #[must_use]
    pub fn err9(&mut self) -> ERR9_W<9> {
        ERR9_W::new(self)
    }
    #[doc = "Bit 10 - Error Interrupt flag for DMA channel."]
    #[inline(always)]
    #[must_use]
    pub fn err10(&mut self) -> ERR10_W<10> {
        ERR10_W::new(self)
    }
    #[doc = "Bit 11 - Error Interrupt flag for DMA channel."]
    #[inline(always)]
    #[must_use]
    pub fn err11(&mut self) -> ERR11_W<11> {
        ERR11_W::new(self)
    }
    #[doc = "Bit 12 - Error Interrupt flag for DMA channel."]
    #[inline(always)]
    #[must_use]
    pub fn err12(&mut self) -> ERR12_W<12> {
        ERR12_W::new(self)
    }
    #[doc = "Bit 13 - Error Interrupt flag for DMA channel."]
    #[inline(always)]
    #[must_use]
    pub fn err13(&mut self) -> ERR13_W<13> {
        ERR13_W::new(self)
    }
    #[doc = "Bit 14 - Error Interrupt flag for DMA channel."]
    #[inline(always)]
    #[must_use]
    pub fn err14(&mut self) -> ERR14_W<14> {
        ERR14_W::new(self)
    }
    #[doc = "Bit 15 - Error Interrupt flag for DMA channel."]
    #[inline(always)]
    #[must_use]
    pub fn err15(&mut self) -> ERR15_W<15> {
        ERR15_W::new(self)
    }
    #[doc = "Bit 16 - Error Interrupt flag for DMA channel."]
    #[inline(always)]
    #[must_use]
    pub fn err16(&mut self) -> ERR16_W<16> {
        ERR16_W::new(self)
    }
    #[doc = "Bit 17 - Error Interrupt flag for DMA channel."]
    #[inline(always)]
    #[must_use]
    pub fn err17(&mut self) -> ERR17_W<17> {
        ERR17_W::new(self)
    }
    #[doc = "Bit 18 - Error Interrupt flag for DMA channel."]
    #[inline(always)]
    #[must_use]
    pub fn err18(&mut self) -> ERR18_W<18> {
        ERR18_W::new(self)
    }
    #[doc = "Bit 19 - Error Interrupt flag for DMA channel."]
    #[inline(always)]
    #[must_use]
    pub fn err19(&mut self) -> ERR19_W<19> {
        ERR19_W::new(self)
    }
    #[doc = "Bit 20 - Error Interrupt flag for DMA channel."]
    #[inline(always)]
    #[must_use]
    pub fn err20(&mut self) -> ERR20_W<20> {
        ERR20_W::new(self)
    }
    #[doc = "Bit 21 - Error Interrupt flag for DMA channel."]
    #[inline(always)]
    #[must_use]
    pub fn err21(&mut self) -> ERR21_W<21> {
        ERR21_W::new(self)
    }
    #[doc = "Bit 22 - Error Interrupt flag for DMA channel."]
    #[inline(always)]
    #[must_use]
    pub fn err22(&mut self) -> ERR22_W<22> {
        ERR22_W::new(self)
    }
    #[doc = "Bit 23 - Error Interrupt flag for DMA channel."]
    #[inline(always)]
    #[must_use]
    pub fn err23(&mut self) -> ERR23_W<23> {
        ERR23_W::new(self)
    }
    #[doc = "Bit 24 - Error Interrupt flag for DMA channel."]
    #[inline(always)]
    #[must_use]
    pub fn err24(&mut self) -> ERR24_W<24> {
        ERR24_W::new(self)
    }
    #[doc = "Bit 25 - Error Interrupt flag for DMA channel."]
    #[inline(always)]
    #[must_use]
    pub fn err25(&mut self) -> ERR25_W<25> {
        ERR25_W::new(self)
    }
    #[doc = "Bit 26 - Error Interrupt flag for DMA channel."]
    #[inline(always)]
    #[must_use]
    pub fn err26(&mut self) -> ERR26_W<26> {
        ERR26_W::new(self)
    }
    #[doc = "Bit 27 - Error Interrupt flag for DMA channel."]
    #[inline(always)]
    #[must_use]
    pub fn err27(&mut self) -> ERR27_W<27> {
        ERR27_W::new(self)
    }
    #[doc = "Bit 28 - Error Interrupt flag for DMA channel."]
    #[inline(always)]
    #[must_use]
    pub fn err28(&mut self) -> ERR28_W<28> {
        ERR28_W::new(self)
    }
    #[doc = "Bit 29 - Error Interrupt flag for DMA channel."]
    #[inline(always)]
    #[must_use]
    pub fn err29(&mut self) -> ERR29_W<29> {
        ERR29_W::new(self)
    }
    #[doc = "Bit 30 - Error Interrupt flag for DMA channel."]
    #[inline(always)]
    #[must_use]
    pub fn err30(&mut self) -> ERR30_W<30> {
        ERR30_W::new(self)
    }
    #[doc = "Bit 31 - Error Interrupt flag for DMA channel."]
    #[inline(always)]
    #[must_use]
    pub fn err31(&mut self) -> ERR31_W<31> {
        ERR31_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Error Interrupt status for all DMA channels\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [errint0](index.html) module"]
pub struct ERRINT0_SPEC;
impl crate::RegisterSpec for ERRINT0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [errint0::R](R) reader structure"]
impl crate::Readable for ERRINT0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [errint0::W](W) writer structure"]
impl crate::Writable for ERRINT0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ERRINT0 to value 0"]
impl crate::Resettable for ERRINT0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
