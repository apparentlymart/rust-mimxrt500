#[doc = "Register `ENABLECLR0` reader"]
pub struct R(crate::R<ENABLECLR0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ENABLECLR0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ENABLECLR0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ENABLECLR0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ENABLECLR0` writer"]
pub struct W(crate::W<ENABLECLR0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ENABLECLR0_SPEC>;
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
impl From<crate::W<ENABLECLR0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ENABLECLR0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CLR0` reader - Writing ones to this register clears the corresponding bits in ENABLESET0."]
pub type CLR0_R = crate::BitReader<CLR0_A>;
#[doc = "Writing ones to this register clears the corresponding bits in ENABLESET0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CLR0_A {
    #[doc = "0: No effect."]
    NO_EFFECT = 0,
    #[doc = "1: DMA channel is cleared."]
    CLEARED = 1,
}
impl From<CLR0_A> for bool {
    #[inline(always)]
    fn from(variant: CLR0_A) -> Self {
        variant as u8 != 0
    }
}
impl CLR0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CLR0_A {
        match self.bits {
            false => CLR0_A::NO_EFFECT,
            true => CLR0_A::CLEARED,
        }
    }
    #[doc = "Checks if the value of the field is `NO_EFFECT`"]
    #[inline(always)]
    pub fn is_no_effect(&self) -> bool {
        *self == CLR0_A::NO_EFFECT
    }
    #[doc = "Checks if the value of the field is `CLEARED`"]
    #[inline(always)]
    pub fn is_cleared(&self) -> bool {
        *self == CLR0_A::CLEARED
    }
}
#[doc = "Field `CLR0` writer - Writing ones to this register clears the corresponding bits in ENABLESET0."]
pub type CLR0_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, ENABLECLR0_SPEC, CLR0_A, O>;
impl<'a, const O: u8> CLR0_W<'a, O> {
    #[doc = "No effect."]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(CLR0_A::NO_EFFECT)
    }
    #[doc = "DMA channel is cleared."]
    #[inline(always)]
    pub fn cleared(self) -> &'a mut W {
        self.variant(CLR0_A::CLEARED)
    }
}
#[doc = "Field `CLR1` reader - Writing ones to this register clears the corresponding bits in ENABLESET0."]
pub type CLR1_R = crate::BitReader<CLR1_A>;
#[doc = "Writing ones to this register clears the corresponding bits in ENABLESET0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CLR1_A {
    #[doc = "0: No effect."]
    NO_EFFECT = 0,
    #[doc = "1: DMA channel is cleared."]
    CLEARED = 1,
}
impl From<CLR1_A> for bool {
    #[inline(always)]
    fn from(variant: CLR1_A) -> Self {
        variant as u8 != 0
    }
}
impl CLR1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CLR1_A {
        match self.bits {
            false => CLR1_A::NO_EFFECT,
            true => CLR1_A::CLEARED,
        }
    }
    #[doc = "Checks if the value of the field is `NO_EFFECT`"]
    #[inline(always)]
    pub fn is_no_effect(&self) -> bool {
        *self == CLR1_A::NO_EFFECT
    }
    #[doc = "Checks if the value of the field is `CLEARED`"]
    #[inline(always)]
    pub fn is_cleared(&self) -> bool {
        *self == CLR1_A::CLEARED
    }
}
#[doc = "Field `CLR1` writer - Writing ones to this register clears the corresponding bits in ENABLESET0."]
pub type CLR1_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, ENABLECLR0_SPEC, CLR1_A, O>;
impl<'a, const O: u8> CLR1_W<'a, O> {
    #[doc = "No effect."]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(CLR1_A::NO_EFFECT)
    }
    #[doc = "DMA channel is cleared."]
    #[inline(always)]
    pub fn cleared(self) -> &'a mut W {
        self.variant(CLR1_A::CLEARED)
    }
}
#[doc = "Field `CLR2` reader - Writing ones to this register clears the corresponding bits in ENABLESET0."]
pub type CLR2_R = crate::BitReader<CLR2_A>;
#[doc = "Writing ones to this register clears the corresponding bits in ENABLESET0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CLR2_A {
    #[doc = "0: No effect."]
    NO_EFFECT = 0,
    #[doc = "1: DMA channel is cleared."]
    CLEARED = 1,
}
impl From<CLR2_A> for bool {
    #[inline(always)]
    fn from(variant: CLR2_A) -> Self {
        variant as u8 != 0
    }
}
impl CLR2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CLR2_A {
        match self.bits {
            false => CLR2_A::NO_EFFECT,
            true => CLR2_A::CLEARED,
        }
    }
    #[doc = "Checks if the value of the field is `NO_EFFECT`"]
    #[inline(always)]
    pub fn is_no_effect(&self) -> bool {
        *self == CLR2_A::NO_EFFECT
    }
    #[doc = "Checks if the value of the field is `CLEARED`"]
    #[inline(always)]
    pub fn is_cleared(&self) -> bool {
        *self == CLR2_A::CLEARED
    }
}
#[doc = "Field `CLR2` writer - Writing ones to this register clears the corresponding bits in ENABLESET0."]
pub type CLR2_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, ENABLECLR0_SPEC, CLR2_A, O>;
impl<'a, const O: u8> CLR2_W<'a, O> {
    #[doc = "No effect."]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(CLR2_A::NO_EFFECT)
    }
    #[doc = "DMA channel is cleared."]
    #[inline(always)]
    pub fn cleared(self) -> &'a mut W {
        self.variant(CLR2_A::CLEARED)
    }
}
#[doc = "Field `CLR3` reader - Writing ones to this register clears the corresponding bits in ENABLESET0."]
pub type CLR3_R = crate::BitReader<CLR3_A>;
#[doc = "Writing ones to this register clears the corresponding bits in ENABLESET0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CLR3_A {
    #[doc = "0: No effect."]
    NO_EFFECT = 0,
    #[doc = "1: DMA channel is cleared."]
    CLEARED = 1,
}
impl From<CLR3_A> for bool {
    #[inline(always)]
    fn from(variant: CLR3_A) -> Self {
        variant as u8 != 0
    }
}
impl CLR3_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CLR3_A {
        match self.bits {
            false => CLR3_A::NO_EFFECT,
            true => CLR3_A::CLEARED,
        }
    }
    #[doc = "Checks if the value of the field is `NO_EFFECT`"]
    #[inline(always)]
    pub fn is_no_effect(&self) -> bool {
        *self == CLR3_A::NO_EFFECT
    }
    #[doc = "Checks if the value of the field is `CLEARED`"]
    #[inline(always)]
    pub fn is_cleared(&self) -> bool {
        *self == CLR3_A::CLEARED
    }
}
#[doc = "Field `CLR3` writer - Writing ones to this register clears the corresponding bits in ENABLESET0."]
pub type CLR3_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, ENABLECLR0_SPEC, CLR3_A, O>;
impl<'a, const O: u8> CLR3_W<'a, O> {
    #[doc = "No effect."]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(CLR3_A::NO_EFFECT)
    }
    #[doc = "DMA channel is cleared."]
    #[inline(always)]
    pub fn cleared(self) -> &'a mut W {
        self.variant(CLR3_A::CLEARED)
    }
}
#[doc = "Field `CLR4` reader - Writing ones to this register clears the corresponding bits in ENABLESET0."]
pub type CLR4_R = crate::BitReader<CLR4_A>;
#[doc = "Writing ones to this register clears the corresponding bits in ENABLESET0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CLR4_A {
    #[doc = "0: No effect."]
    NO_EFFECT = 0,
    #[doc = "1: DMA channel is cleared."]
    CLEARED = 1,
}
impl From<CLR4_A> for bool {
    #[inline(always)]
    fn from(variant: CLR4_A) -> Self {
        variant as u8 != 0
    }
}
impl CLR4_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CLR4_A {
        match self.bits {
            false => CLR4_A::NO_EFFECT,
            true => CLR4_A::CLEARED,
        }
    }
    #[doc = "Checks if the value of the field is `NO_EFFECT`"]
    #[inline(always)]
    pub fn is_no_effect(&self) -> bool {
        *self == CLR4_A::NO_EFFECT
    }
    #[doc = "Checks if the value of the field is `CLEARED`"]
    #[inline(always)]
    pub fn is_cleared(&self) -> bool {
        *self == CLR4_A::CLEARED
    }
}
#[doc = "Field `CLR4` writer - Writing ones to this register clears the corresponding bits in ENABLESET0."]
pub type CLR4_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, ENABLECLR0_SPEC, CLR4_A, O>;
impl<'a, const O: u8> CLR4_W<'a, O> {
    #[doc = "No effect."]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(CLR4_A::NO_EFFECT)
    }
    #[doc = "DMA channel is cleared."]
    #[inline(always)]
    pub fn cleared(self) -> &'a mut W {
        self.variant(CLR4_A::CLEARED)
    }
}
#[doc = "Field `CLR5` reader - Writing ones to this register clears the corresponding bits in ENABLESET0."]
pub type CLR5_R = crate::BitReader<CLR5_A>;
#[doc = "Writing ones to this register clears the corresponding bits in ENABLESET0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CLR5_A {
    #[doc = "0: No effect."]
    NO_EFFECT = 0,
    #[doc = "1: DMA channel is cleared."]
    CLEARED = 1,
}
impl From<CLR5_A> for bool {
    #[inline(always)]
    fn from(variant: CLR5_A) -> Self {
        variant as u8 != 0
    }
}
impl CLR5_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CLR5_A {
        match self.bits {
            false => CLR5_A::NO_EFFECT,
            true => CLR5_A::CLEARED,
        }
    }
    #[doc = "Checks if the value of the field is `NO_EFFECT`"]
    #[inline(always)]
    pub fn is_no_effect(&self) -> bool {
        *self == CLR5_A::NO_EFFECT
    }
    #[doc = "Checks if the value of the field is `CLEARED`"]
    #[inline(always)]
    pub fn is_cleared(&self) -> bool {
        *self == CLR5_A::CLEARED
    }
}
#[doc = "Field `CLR5` writer - Writing ones to this register clears the corresponding bits in ENABLESET0."]
pub type CLR5_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, ENABLECLR0_SPEC, CLR5_A, O>;
impl<'a, const O: u8> CLR5_W<'a, O> {
    #[doc = "No effect."]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(CLR5_A::NO_EFFECT)
    }
    #[doc = "DMA channel is cleared."]
    #[inline(always)]
    pub fn cleared(self) -> &'a mut W {
        self.variant(CLR5_A::CLEARED)
    }
}
#[doc = "Field `CLR6` reader - Writing ones to this register clears the corresponding bits in ENABLESET0."]
pub type CLR6_R = crate::BitReader<CLR6_A>;
#[doc = "Writing ones to this register clears the corresponding bits in ENABLESET0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CLR6_A {
    #[doc = "0: No effect."]
    NO_EFFECT = 0,
    #[doc = "1: DMA channel is cleared."]
    CLEARED = 1,
}
impl From<CLR6_A> for bool {
    #[inline(always)]
    fn from(variant: CLR6_A) -> Self {
        variant as u8 != 0
    }
}
impl CLR6_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CLR6_A {
        match self.bits {
            false => CLR6_A::NO_EFFECT,
            true => CLR6_A::CLEARED,
        }
    }
    #[doc = "Checks if the value of the field is `NO_EFFECT`"]
    #[inline(always)]
    pub fn is_no_effect(&self) -> bool {
        *self == CLR6_A::NO_EFFECT
    }
    #[doc = "Checks if the value of the field is `CLEARED`"]
    #[inline(always)]
    pub fn is_cleared(&self) -> bool {
        *self == CLR6_A::CLEARED
    }
}
#[doc = "Field `CLR6` writer - Writing ones to this register clears the corresponding bits in ENABLESET0."]
pub type CLR6_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, ENABLECLR0_SPEC, CLR6_A, O>;
impl<'a, const O: u8> CLR6_W<'a, O> {
    #[doc = "No effect."]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(CLR6_A::NO_EFFECT)
    }
    #[doc = "DMA channel is cleared."]
    #[inline(always)]
    pub fn cleared(self) -> &'a mut W {
        self.variant(CLR6_A::CLEARED)
    }
}
#[doc = "Field `CLR7` reader - Writing ones to this register clears the corresponding bits in ENABLESET0."]
pub type CLR7_R = crate::BitReader<CLR7_A>;
#[doc = "Writing ones to this register clears the corresponding bits in ENABLESET0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CLR7_A {
    #[doc = "0: No effect."]
    NO_EFFECT = 0,
    #[doc = "1: DMA channel is cleared."]
    CLEARED = 1,
}
impl From<CLR7_A> for bool {
    #[inline(always)]
    fn from(variant: CLR7_A) -> Self {
        variant as u8 != 0
    }
}
impl CLR7_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CLR7_A {
        match self.bits {
            false => CLR7_A::NO_EFFECT,
            true => CLR7_A::CLEARED,
        }
    }
    #[doc = "Checks if the value of the field is `NO_EFFECT`"]
    #[inline(always)]
    pub fn is_no_effect(&self) -> bool {
        *self == CLR7_A::NO_EFFECT
    }
    #[doc = "Checks if the value of the field is `CLEARED`"]
    #[inline(always)]
    pub fn is_cleared(&self) -> bool {
        *self == CLR7_A::CLEARED
    }
}
#[doc = "Field `CLR7` writer - Writing ones to this register clears the corresponding bits in ENABLESET0."]
pub type CLR7_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, ENABLECLR0_SPEC, CLR7_A, O>;
impl<'a, const O: u8> CLR7_W<'a, O> {
    #[doc = "No effect."]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(CLR7_A::NO_EFFECT)
    }
    #[doc = "DMA channel is cleared."]
    #[inline(always)]
    pub fn cleared(self) -> &'a mut W {
        self.variant(CLR7_A::CLEARED)
    }
}
#[doc = "Field `CLR8` reader - Writing ones to this register clears the corresponding bits in ENABLESET0."]
pub type CLR8_R = crate::BitReader<CLR8_A>;
#[doc = "Writing ones to this register clears the corresponding bits in ENABLESET0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CLR8_A {
    #[doc = "0: No effect."]
    NO_EFFECT = 0,
    #[doc = "1: DMA channel is cleared."]
    CLEARED = 1,
}
impl From<CLR8_A> for bool {
    #[inline(always)]
    fn from(variant: CLR8_A) -> Self {
        variant as u8 != 0
    }
}
impl CLR8_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CLR8_A {
        match self.bits {
            false => CLR8_A::NO_EFFECT,
            true => CLR8_A::CLEARED,
        }
    }
    #[doc = "Checks if the value of the field is `NO_EFFECT`"]
    #[inline(always)]
    pub fn is_no_effect(&self) -> bool {
        *self == CLR8_A::NO_EFFECT
    }
    #[doc = "Checks if the value of the field is `CLEARED`"]
    #[inline(always)]
    pub fn is_cleared(&self) -> bool {
        *self == CLR8_A::CLEARED
    }
}
#[doc = "Field `CLR8` writer - Writing ones to this register clears the corresponding bits in ENABLESET0."]
pub type CLR8_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, ENABLECLR0_SPEC, CLR8_A, O>;
impl<'a, const O: u8> CLR8_W<'a, O> {
    #[doc = "No effect."]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(CLR8_A::NO_EFFECT)
    }
    #[doc = "DMA channel is cleared."]
    #[inline(always)]
    pub fn cleared(self) -> &'a mut W {
        self.variant(CLR8_A::CLEARED)
    }
}
#[doc = "Field `CLR9` reader - Writing ones to this register clears the corresponding bits in ENABLESET0."]
pub type CLR9_R = crate::BitReader<CLR9_A>;
#[doc = "Writing ones to this register clears the corresponding bits in ENABLESET0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CLR9_A {
    #[doc = "0: No effect."]
    NO_EFFECT = 0,
    #[doc = "1: DMA channel is cleared."]
    CLEARED = 1,
}
impl From<CLR9_A> for bool {
    #[inline(always)]
    fn from(variant: CLR9_A) -> Self {
        variant as u8 != 0
    }
}
impl CLR9_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CLR9_A {
        match self.bits {
            false => CLR9_A::NO_EFFECT,
            true => CLR9_A::CLEARED,
        }
    }
    #[doc = "Checks if the value of the field is `NO_EFFECT`"]
    #[inline(always)]
    pub fn is_no_effect(&self) -> bool {
        *self == CLR9_A::NO_EFFECT
    }
    #[doc = "Checks if the value of the field is `CLEARED`"]
    #[inline(always)]
    pub fn is_cleared(&self) -> bool {
        *self == CLR9_A::CLEARED
    }
}
#[doc = "Field `CLR9` writer - Writing ones to this register clears the corresponding bits in ENABLESET0."]
pub type CLR9_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, ENABLECLR0_SPEC, CLR9_A, O>;
impl<'a, const O: u8> CLR9_W<'a, O> {
    #[doc = "No effect."]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(CLR9_A::NO_EFFECT)
    }
    #[doc = "DMA channel is cleared."]
    #[inline(always)]
    pub fn cleared(self) -> &'a mut W {
        self.variant(CLR9_A::CLEARED)
    }
}
#[doc = "Field `CLR10` reader - Writing ones to this register clears the corresponding bits in ENABLESET0."]
pub type CLR10_R = crate::BitReader<CLR10_A>;
#[doc = "Writing ones to this register clears the corresponding bits in ENABLESET0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CLR10_A {
    #[doc = "0: No effect."]
    NO_EFFECT = 0,
    #[doc = "1: DMA channel is cleared."]
    CLEARED = 1,
}
impl From<CLR10_A> for bool {
    #[inline(always)]
    fn from(variant: CLR10_A) -> Self {
        variant as u8 != 0
    }
}
impl CLR10_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CLR10_A {
        match self.bits {
            false => CLR10_A::NO_EFFECT,
            true => CLR10_A::CLEARED,
        }
    }
    #[doc = "Checks if the value of the field is `NO_EFFECT`"]
    #[inline(always)]
    pub fn is_no_effect(&self) -> bool {
        *self == CLR10_A::NO_EFFECT
    }
    #[doc = "Checks if the value of the field is `CLEARED`"]
    #[inline(always)]
    pub fn is_cleared(&self) -> bool {
        *self == CLR10_A::CLEARED
    }
}
#[doc = "Field `CLR10` writer - Writing ones to this register clears the corresponding bits in ENABLESET0."]
pub type CLR10_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, ENABLECLR0_SPEC, CLR10_A, O>;
impl<'a, const O: u8> CLR10_W<'a, O> {
    #[doc = "No effect."]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(CLR10_A::NO_EFFECT)
    }
    #[doc = "DMA channel is cleared."]
    #[inline(always)]
    pub fn cleared(self) -> &'a mut W {
        self.variant(CLR10_A::CLEARED)
    }
}
#[doc = "Field `CLR11` reader - Writing ones to this register clears the corresponding bits in ENABLESET0."]
pub type CLR11_R = crate::BitReader<CLR11_A>;
#[doc = "Writing ones to this register clears the corresponding bits in ENABLESET0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CLR11_A {
    #[doc = "0: No effect."]
    NO_EFFECT = 0,
    #[doc = "1: DMA channel is cleared."]
    CLEARED = 1,
}
impl From<CLR11_A> for bool {
    #[inline(always)]
    fn from(variant: CLR11_A) -> Self {
        variant as u8 != 0
    }
}
impl CLR11_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CLR11_A {
        match self.bits {
            false => CLR11_A::NO_EFFECT,
            true => CLR11_A::CLEARED,
        }
    }
    #[doc = "Checks if the value of the field is `NO_EFFECT`"]
    #[inline(always)]
    pub fn is_no_effect(&self) -> bool {
        *self == CLR11_A::NO_EFFECT
    }
    #[doc = "Checks if the value of the field is `CLEARED`"]
    #[inline(always)]
    pub fn is_cleared(&self) -> bool {
        *self == CLR11_A::CLEARED
    }
}
#[doc = "Field `CLR11` writer - Writing ones to this register clears the corresponding bits in ENABLESET0."]
pub type CLR11_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, ENABLECLR0_SPEC, CLR11_A, O>;
impl<'a, const O: u8> CLR11_W<'a, O> {
    #[doc = "No effect."]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(CLR11_A::NO_EFFECT)
    }
    #[doc = "DMA channel is cleared."]
    #[inline(always)]
    pub fn cleared(self) -> &'a mut W {
        self.variant(CLR11_A::CLEARED)
    }
}
#[doc = "Field `CLR12` reader - Writing ones to this register clears the corresponding bits in ENABLESET0."]
pub type CLR12_R = crate::BitReader<CLR12_A>;
#[doc = "Writing ones to this register clears the corresponding bits in ENABLESET0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CLR12_A {
    #[doc = "0: No effect."]
    NO_EFFECT = 0,
    #[doc = "1: DMA channel is cleared."]
    CLEARED = 1,
}
impl From<CLR12_A> for bool {
    #[inline(always)]
    fn from(variant: CLR12_A) -> Self {
        variant as u8 != 0
    }
}
impl CLR12_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CLR12_A {
        match self.bits {
            false => CLR12_A::NO_EFFECT,
            true => CLR12_A::CLEARED,
        }
    }
    #[doc = "Checks if the value of the field is `NO_EFFECT`"]
    #[inline(always)]
    pub fn is_no_effect(&self) -> bool {
        *self == CLR12_A::NO_EFFECT
    }
    #[doc = "Checks if the value of the field is `CLEARED`"]
    #[inline(always)]
    pub fn is_cleared(&self) -> bool {
        *self == CLR12_A::CLEARED
    }
}
#[doc = "Field `CLR12` writer - Writing ones to this register clears the corresponding bits in ENABLESET0."]
pub type CLR12_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, ENABLECLR0_SPEC, CLR12_A, O>;
impl<'a, const O: u8> CLR12_W<'a, O> {
    #[doc = "No effect."]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(CLR12_A::NO_EFFECT)
    }
    #[doc = "DMA channel is cleared."]
    #[inline(always)]
    pub fn cleared(self) -> &'a mut W {
        self.variant(CLR12_A::CLEARED)
    }
}
#[doc = "Field `CLR13` reader - Writing ones to this register clears the corresponding bits in ENABLESET0."]
pub type CLR13_R = crate::BitReader<CLR13_A>;
#[doc = "Writing ones to this register clears the corresponding bits in ENABLESET0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CLR13_A {
    #[doc = "0: No effect."]
    NO_EFFECT = 0,
    #[doc = "1: DMA channel is cleared."]
    CLEARED = 1,
}
impl From<CLR13_A> for bool {
    #[inline(always)]
    fn from(variant: CLR13_A) -> Self {
        variant as u8 != 0
    }
}
impl CLR13_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CLR13_A {
        match self.bits {
            false => CLR13_A::NO_EFFECT,
            true => CLR13_A::CLEARED,
        }
    }
    #[doc = "Checks if the value of the field is `NO_EFFECT`"]
    #[inline(always)]
    pub fn is_no_effect(&self) -> bool {
        *self == CLR13_A::NO_EFFECT
    }
    #[doc = "Checks if the value of the field is `CLEARED`"]
    #[inline(always)]
    pub fn is_cleared(&self) -> bool {
        *self == CLR13_A::CLEARED
    }
}
#[doc = "Field `CLR13` writer - Writing ones to this register clears the corresponding bits in ENABLESET0."]
pub type CLR13_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, ENABLECLR0_SPEC, CLR13_A, O>;
impl<'a, const O: u8> CLR13_W<'a, O> {
    #[doc = "No effect."]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(CLR13_A::NO_EFFECT)
    }
    #[doc = "DMA channel is cleared."]
    #[inline(always)]
    pub fn cleared(self) -> &'a mut W {
        self.variant(CLR13_A::CLEARED)
    }
}
#[doc = "Field `CLR14` reader - Writing ones to this register clears the corresponding bits in ENABLESET0."]
pub type CLR14_R = crate::BitReader<CLR14_A>;
#[doc = "Writing ones to this register clears the corresponding bits in ENABLESET0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CLR14_A {
    #[doc = "0: No effect."]
    NO_EFFECT = 0,
    #[doc = "1: DMA channel is cleared."]
    CLEARED = 1,
}
impl From<CLR14_A> for bool {
    #[inline(always)]
    fn from(variant: CLR14_A) -> Self {
        variant as u8 != 0
    }
}
impl CLR14_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CLR14_A {
        match self.bits {
            false => CLR14_A::NO_EFFECT,
            true => CLR14_A::CLEARED,
        }
    }
    #[doc = "Checks if the value of the field is `NO_EFFECT`"]
    #[inline(always)]
    pub fn is_no_effect(&self) -> bool {
        *self == CLR14_A::NO_EFFECT
    }
    #[doc = "Checks if the value of the field is `CLEARED`"]
    #[inline(always)]
    pub fn is_cleared(&self) -> bool {
        *self == CLR14_A::CLEARED
    }
}
#[doc = "Field `CLR14` writer - Writing ones to this register clears the corresponding bits in ENABLESET0."]
pub type CLR14_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, ENABLECLR0_SPEC, CLR14_A, O>;
impl<'a, const O: u8> CLR14_W<'a, O> {
    #[doc = "No effect."]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(CLR14_A::NO_EFFECT)
    }
    #[doc = "DMA channel is cleared."]
    #[inline(always)]
    pub fn cleared(self) -> &'a mut W {
        self.variant(CLR14_A::CLEARED)
    }
}
#[doc = "Field `CLR15` reader - Writing ones to this register clears the corresponding bits in ENABLESET0."]
pub type CLR15_R = crate::BitReader<CLR15_A>;
#[doc = "Writing ones to this register clears the corresponding bits in ENABLESET0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CLR15_A {
    #[doc = "0: No effect."]
    NO_EFFECT = 0,
    #[doc = "1: DMA channel is cleared."]
    CLEARED = 1,
}
impl From<CLR15_A> for bool {
    #[inline(always)]
    fn from(variant: CLR15_A) -> Self {
        variant as u8 != 0
    }
}
impl CLR15_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CLR15_A {
        match self.bits {
            false => CLR15_A::NO_EFFECT,
            true => CLR15_A::CLEARED,
        }
    }
    #[doc = "Checks if the value of the field is `NO_EFFECT`"]
    #[inline(always)]
    pub fn is_no_effect(&self) -> bool {
        *self == CLR15_A::NO_EFFECT
    }
    #[doc = "Checks if the value of the field is `CLEARED`"]
    #[inline(always)]
    pub fn is_cleared(&self) -> bool {
        *self == CLR15_A::CLEARED
    }
}
#[doc = "Field `CLR15` writer - Writing ones to this register clears the corresponding bits in ENABLESET0."]
pub type CLR15_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, ENABLECLR0_SPEC, CLR15_A, O>;
impl<'a, const O: u8> CLR15_W<'a, O> {
    #[doc = "No effect."]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(CLR15_A::NO_EFFECT)
    }
    #[doc = "DMA channel is cleared."]
    #[inline(always)]
    pub fn cleared(self) -> &'a mut W {
        self.variant(CLR15_A::CLEARED)
    }
}
#[doc = "Field `CLR16` reader - Writing ones to this register clears the corresponding bits in ENABLESET0."]
pub type CLR16_R = crate::BitReader<CLR16_A>;
#[doc = "Writing ones to this register clears the corresponding bits in ENABLESET0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CLR16_A {
    #[doc = "0: No effect."]
    NO_EFFECT = 0,
    #[doc = "1: DMA channel is cleared."]
    CLEARED = 1,
}
impl From<CLR16_A> for bool {
    #[inline(always)]
    fn from(variant: CLR16_A) -> Self {
        variant as u8 != 0
    }
}
impl CLR16_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CLR16_A {
        match self.bits {
            false => CLR16_A::NO_EFFECT,
            true => CLR16_A::CLEARED,
        }
    }
    #[doc = "Checks if the value of the field is `NO_EFFECT`"]
    #[inline(always)]
    pub fn is_no_effect(&self) -> bool {
        *self == CLR16_A::NO_EFFECT
    }
    #[doc = "Checks if the value of the field is `CLEARED`"]
    #[inline(always)]
    pub fn is_cleared(&self) -> bool {
        *self == CLR16_A::CLEARED
    }
}
#[doc = "Field `CLR16` writer - Writing ones to this register clears the corresponding bits in ENABLESET0."]
pub type CLR16_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, ENABLECLR0_SPEC, CLR16_A, O>;
impl<'a, const O: u8> CLR16_W<'a, O> {
    #[doc = "No effect."]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(CLR16_A::NO_EFFECT)
    }
    #[doc = "DMA channel is cleared."]
    #[inline(always)]
    pub fn cleared(self) -> &'a mut W {
        self.variant(CLR16_A::CLEARED)
    }
}
#[doc = "Field `CLR17` reader - Writing ones to this register clears the corresponding bits in ENABLESET0."]
pub type CLR17_R = crate::BitReader<CLR17_A>;
#[doc = "Writing ones to this register clears the corresponding bits in ENABLESET0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CLR17_A {
    #[doc = "0: No effect."]
    NO_EFFECT = 0,
    #[doc = "1: DMA channel is cleared."]
    CLEARED = 1,
}
impl From<CLR17_A> for bool {
    #[inline(always)]
    fn from(variant: CLR17_A) -> Self {
        variant as u8 != 0
    }
}
impl CLR17_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CLR17_A {
        match self.bits {
            false => CLR17_A::NO_EFFECT,
            true => CLR17_A::CLEARED,
        }
    }
    #[doc = "Checks if the value of the field is `NO_EFFECT`"]
    #[inline(always)]
    pub fn is_no_effect(&self) -> bool {
        *self == CLR17_A::NO_EFFECT
    }
    #[doc = "Checks if the value of the field is `CLEARED`"]
    #[inline(always)]
    pub fn is_cleared(&self) -> bool {
        *self == CLR17_A::CLEARED
    }
}
#[doc = "Field `CLR17` writer - Writing ones to this register clears the corresponding bits in ENABLESET0."]
pub type CLR17_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, ENABLECLR0_SPEC, CLR17_A, O>;
impl<'a, const O: u8> CLR17_W<'a, O> {
    #[doc = "No effect."]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(CLR17_A::NO_EFFECT)
    }
    #[doc = "DMA channel is cleared."]
    #[inline(always)]
    pub fn cleared(self) -> &'a mut W {
        self.variant(CLR17_A::CLEARED)
    }
}
#[doc = "Field `CLR18` reader - Writing ones to this register clears the corresponding bits in ENABLESET0."]
pub type CLR18_R = crate::BitReader<CLR18_A>;
#[doc = "Writing ones to this register clears the corresponding bits in ENABLESET0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CLR18_A {
    #[doc = "0: No effect."]
    NO_EFFECT = 0,
    #[doc = "1: DMA channel is cleared."]
    CLEARED = 1,
}
impl From<CLR18_A> for bool {
    #[inline(always)]
    fn from(variant: CLR18_A) -> Self {
        variant as u8 != 0
    }
}
impl CLR18_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CLR18_A {
        match self.bits {
            false => CLR18_A::NO_EFFECT,
            true => CLR18_A::CLEARED,
        }
    }
    #[doc = "Checks if the value of the field is `NO_EFFECT`"]
    #[inline(always)]
    pub fn is_no_effect(&self) -> bool {
        *self == CLR18_A::NO_EFFECT
    }
    #[doc = "Checks if the value of the field is `CLEARED`"]
    #[inline(always)]
    pub fn is_cleared(&self) -> bool {
        *self == CLR18_A::CLEARED
    }
}
#[doc = "Field `CLR18` writer - Writing ones to this register clears the corresponding bits in ENABLESET0."]
pub type CLR18_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, ENABLECLR0_SPEC, CLR18_A, O>;
impl<'a, const O: u8> CLR18_W<'a, O> {
    #[doc = "No effect."]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(CLR18_A::NO_EFFECT)
    }
    #[doc = "DMA channel is cleared."]
    #[inline(always)]
    pub fn cleared(self) -> &'a mut W {
        self.variant(CLR18_A::CLEARED)
    }
}
#[doc = "Field `CLR19` reader - Writing ones to this register clears the corresponding bits in ENABLESET0."]
pub type CLR19_R = crate::BitReader<CLR19_A>;
#[doc = "Writing ones to this register clears the corresponding bits in ENABLESET0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CLR19_A {
    #[doc = "0: No effect."]
    NO_EFFECT = 0,
    #[doc = "1: DMA channel is cleared."]
    CLEARED = 1,
}
impl From<CLR19_A> for bool {
    #[inline(always)]
    fn from(variant: CLR19_A) -> Self {
        variant as u8 != 0
    }
}
impl CLR19_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CLR19_A {
        match self.bits {
            false => CLR19_A::NO_EFFECT,
            true => CLR19_A::CLEARED,
        }
    }
    #[doc = "Checks if the value of the field is `NO_EFFECT`"]
    #[inline(always)]
    pub fn is_no_effect(&self) -> bool {
        *self == CLR19_A::NO_EFFECT
    }
    #[doc = "Checks if the value of the field is `CLEARED`"]
    #[inline(always)]
    pub fn is_cleared(&self) -> bool {
        *self == CLR19_A::CLEARED
    }
}
#[doc = "Field `CLR19` writer - Writing ones to this register clears the corresponding bits in ENABLESET0."]
pub type CLR19_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, ENABLECLR0_SPEC, CLR19_A, O>;
impl<'a, const O: u8> CLR19_W<'a, O> {
    #[doc = "No effect."]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(CLR19_A::NO_EFFECT)
    }
    #[doc = "DMA channel is cleared."]
    #[inline(always)]
    pub fn cleared(self) -> &'a mut W {
        self.variant(CLR19_A::CLEARED)
    }
}
#[doc = "Field `CLR20` reader - Writing ones to this register clears the corresponding bits in ENABLESET0."]
pub type CLR20_R = crate::BitReader<CLR20_A>;
#[doc = "Writing ones to this register clears the corresponding bits in ENABLESET0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CLR20_A {
    #[doc = "0: No effect."]
    NO_EFFECT = 0,
    #[doc = "1: DMA channel is cleared."]
    CLEARED = 1,
}
impl From<CLR20_A> for bool {
    #[inline(always)]
    fn from(variant: CLR20_A) -> Self {
        variant as u8 != 0
    }
}
impl CLR20_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CLR20_A {
        match self.bits {
            false => CLR20_A::NO_EFFECT,
            true => CLR20_A::CLEARED,
        }
    }
    #[doc = "Checks if the value of the field is `NO_EFFECT`"]
    #[inline(always)]
    pub fn is_no_effect(&self) -> bool {
        *self == CLR20_A::NO_EFFECT
    }
    #[doc = "Checks if the value of the field is `CLEARED`"]
    #[inline(always)]
    pub fn is_cleared(&self) -> bool {
        *self == CLR20_A::CLEARED
    }
}
#[doc = "Field `CLR20` writer - Writing ones to this register clears the corresponding bits in ENABLESET0."]
pub type CLR20_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, ENABLECLR0_SPEC, CLR20_A, O>;
impl<'a, const O: u8> CLR20_W<'a, O> {
    #[doc = "No effect."]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(CLR20_A::NO_EFFECT)
    }
    #[doc = "DMA channel is cleared."]
    #[inline(always)]
    pub fn cleared(self) -> &'a mut W {
        self.variant(CLR20_A::CLEARED)
    }
}
#[doc = "Field `CLR21` reader - Writing ones to this register clears the corresponding bits in ENABLESET0."]
pub type CLR21_R = crate::BitReader<CLR21_A>;
#[doc = "Writing ones to this register clears the corresponding bits in ENABLESET0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CLR21_A {
    #[doc = "0: No effect."]
    NO_EFFECT = 0,
    #[doc = "1: DMA channel is cleared."]
    CLEARED = 1,
}
impl From<CLR21_A> for bool {
    #[inline(always)]
    fn from(variant: CLR21_A) -> Self {
        variant as u8 != 0
    }
}
impl CLR21_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CLR21_A {
        match self.bits {
            false => CLR21_A::NO_EFFECT,
            true => CLR21_A::CLEARED,
        }
    }
    #[doc = "Checks if the value of the field is `NO_EFFECT`"]
    #[inline(always)]
    pub fn is_no_effect(&self) -> bool {
        *self == CLR21_A::NO_EFFECT
    }
    #[doc = "Checks if the value of the field is `CLEARED`"]
    #[inline(always)]
    pub fn is_cleared(&self) -> bool {
        *self == CLR21_A::CLEARED
    }
}
#[doc = "Field `CLR21` writer - Writing ones to this register clears the corresponding bits in ENABLESET0."]
pub type CLR21_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, ENABLECLR0_SPEC, CLR21_A, O>;
impl<'a, const O: u8> CLR21_W<'a, O> {
    #[doc = "No effect."]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(CLR21_A::NO_EFFECT)
    }
    #[doc = "DMA channel is cleared."]
    #[inline(always)]
    pub fn cleared(self) -> &'a mut W {
        self.variant(CLR21_A::CLEARED)
    }
}
#[doc = "Field `CLR22` reader - Writing ones to this register clears the corresponding bits in ENABLESET0."]
pub type CLR22_R = crate::BitReader<CLR22_A>;
#[doc = "Writing ones to this register clears the corresponding bits in ENABLESET0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CLR22_A {
    #[doc = "0: No effect."]
    NO_EFFECT = 0,
    #[doc = "1: DMA channel is cleared."]
    CLEARED = 1,
}
impl From<CLR22_A> for bool {
    #[inline(always)]
    fn from(variant: CLR22_A) -> Self {
        variant as u8 != 0
    }
}
impl CLR22_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CLR22_A {
        match self.bits {
            false => CLR22_A::NO_EFFECT,
            true => CLR22_A::CLEARED,
        }
    }
    #[doc = "Checks if the value of the field is `NO_EFFECT`"]
    #[inline(always)]
    pub fn is_no_effect(&self) -> bool {
        *self == CLR22_A::NO_EFFECT
    }
    #[doc = "Checks if the value of the field is `CLEARED`"]
    #[inline(always)]
    pub fn is_cleared(&self) -> bool {
        *self == CLR22_A::CLEARED
    }
}
#[doc = "Field `CLR22` writer - Writing ones to this register clears the corresponding bits in ENABLESET0."]
pub type CLR22_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, ENABLECLR0_SPEC, CLR22_A, O>;
impl<'a, const O: u8> CLR22_W<'a, O> {
    #[doc = "No effect."]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(CLR22_A::NO_EFFECT)
    }
    #[doc = "DMA channel is cleared."]
    #[inline(always)]
    pub fn cleared(self) -> &'a mut W {
        self.variant(CLR22_A::CLEARED)
    }
}
#[doc = "Field `CLR23` reader - Writing ones to this register clears the corresponding bits in ENABLESET0."]
pub type CLR23_R = crate::BitReader<CLR23_A>;
#[doc = "Writing ones to this register clears the corresponding bits in ENABLESET0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CLR23_A {
    #[doc = "0: No effect."]
    NO_EFFECT = 0,
    #[doc = "1: DMA channel is cleared."]
    CLEARED = 1,
}
impl From<CLR23_A> for bool {
    #[inline(always)]
    fn from(variant: CLR23_A) -> Self {
        variant as u8 != 0
    }
}
impl CLR23_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CLR23_A {
        match self.bits {
            false => CLR23_A::NO_EFFECT,
            true => CLR23_A::CLEARED,
        }
    }
    #[doc = "Checks if the value of the field is `NO_EFFECT`"]
    #[inline(always)]
    pub fn is_no_effect(&self) -> bool {
        *self == CLR23_A::NO_EFFECT
    }
    #[doc = "Checks if the value of the field is `CLEARED`"]
    #[inline(always)]
    pub fn is_cleared(&self) -> bool {
        *self == CLR23_A::CLEARED
    }
}
#[doc = "Field `CLR23` writer - Writing ones to this register clears the corresponding bits in ENABLESET0."]
pub type CLR23_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, ENABLECLR0_SPEC, CLR23_A, O>;
impl<'a, const O: u8> CLR23_W<'a, O> {
    #[doc = "No effect."]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(CLR23_A::NO_EFFECT)
    }
    #[doc = "DMA channel is cleared."]
    #[inline(always)]
    pub fn cleared(self) -> &'a mut W {
        self.variant(CLR23_A::CLEARED)
    }
}
#[doc = "Field `CLR24` reader - Writing ones to this register clears the corresponding bits in ENABLESET0."]
pub type CLR24_R = crate::BitReader<CLR24_A>;
#[doc = "Writing ones to this register clears the corresponding bits in ENABLESET0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CLR24_A {
    #[doc = "0: No effect."]
    NO_EFFECT = 0,
    #[doc = "1: DMA channel is cleared."]
    CLEARED = 1,
}
impl From<CLR24_A> for bool {
    #[inline(always)]
    fn from(variant: CLR24_A) -> Self {
        variant as u8 != 0
    }
}
impl CLR24_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CLR24_A {
        match self.bits {
            false => CLR24_A::NO_EFFECT,
            true => CLR24_A::CLEARED,
        }
    }
    #[doc = "Checks if the value of the field is `NO_EFFECT`"]
    #[inline(always)]
    pub fn is_no_effect(&self) -> bool {
        *self == CLR24_A::NO_EFFECT
    }
    #[doc = "Checks if the value of the field is `CLEARED`"]
    #[inline(always)]
    pub fn is_cleared(&self) -> bool {
        *self == CLR24_A::CLEARED
    }
}
#[doc = "Field `CLR24` writer - Writing ones to this register clears the corresponding bits in ENABLESET0."]
pub type CLR24_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, ENABLECLR0_SPEC, CLR24_A, O>;
impl<'a, const O: u8> CLR24_W<'a, O> {
    #[doc = "No effect."]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(CLR24_A::NO_EFFECT)
    }
    #[doc = "DMA channel is cleared."]
    #[inline(always)]
    pub fn cleared(self) -> &'a mut W {
        self.variant(CLR24_A::CLEARED)
    }
}
#[doc = "Field `CLR25` reader - Writing ones to this register clears the corresponding bits in ENABLESET0."]
pub type CLR25_R = crate::BitReader<CLR25_A>;
#[doc = "Writing ones to this register clears the corresponding bits in ENABLESET0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CLR25_A {
    #[doc = "0: No effect."]
    NO_EFFECT = 0,
    #[doc = "1: DMA channel is cleared."]
    CLEARED = 1,
}
impl From<CLR25_A> for bool {
    #[inline(always)]
    fn from(variant: CLR25_A) -> Self {
        variant as u8 != 0
    }
}
impl CLR25_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CLR25_A {
        match self.bits {
            false => CLR25_A::NO_EFFECT,
            true => CLR25_A::CLEARED,
        }
    }
    #[doc = "Checks if the value of the field is `NO_EFFECT`"]
    #[inline(always)]
    pub fn is_no_effect(&self) -> bool {
        *self == CLR25_A::NO_EFFECT
    }
    #[doc = "Checks if the value of the field is `CLEARED`"]
    #[inline(always)]
    pub fn is_cleared(&self) -> bool {
        *self == CLR25_A::CLEARED
    }
}
#[doc = "Field `CLR25` writer - Writing ones to this register clears the corresponding bits in ENABLESET0."]
pub type CLR25_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, ENABLECLR0_SPEC, CLR25_A, O>;
impl<'a, const O: u8> CLR25_W<'a, O> {
    #[doc = "No effect."]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(CLR25_A::NO_EFFECT)
    }
    #[doc = "DMA channel is cleared."]
    #[inline(always)]
    pub fn cleared(self) -> &'a mut W {
        self.variant(CLR25_A::CLEARED)
    }
}
#[doc = "Field `CLR26` reader - Writing ones to this register clears the corresponding bits in ENABLESET0."]
pub type CLR26_R = crate::BitReader<CLR26_A>;
#[doc = "Writing ones to this register clears the corresponding bits in ENABLESET0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CLR26_A {
    #[doc = "0: No effect."]
    NO_EFFECT = 0,
    #[doc = "1: DMA channel is cleared."]
    CLEARED = 1,
}
impl From<CLR26_A> for bool {
    #[inline(always)]
    fn from(variant: CLR26_A) -> Self {
        variant as u8 != 0
    }
}
impl CLR26_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CLR26_A {
        match self.bits {
            false => CLR26_A::NO_EFFECT,
            true => CLR26_A::CLEARED,
        }
    }
    #[doc = "Checks if the value of the field is `NO_EFFECT`"]
    #[inline(always)]
    pub fn is_no_effect(&self) -> bool {
        *self == CLR26_A::NO_EFFECT
    }
    #[doc = "Checks if the value of the field is `CLEARED`"]
    #[inline(always)]
    pub fn is_cleared(&self) -> bool {
        *self == CLR26_A::CLEARED
    }
}
#[doc = "Field `CLR26` writer - Writing ones to this register clears the corresponding bits in ENABLESET0."]
pub type CLR26_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, ENABLECLR0_SPEC, CLR26_A, O>;
impl<'a, const O: u8> CLR26_W<'a, O> {
    #[doc = "No effect."]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(CLR26_A::NO_EFFECT)
    }
    #[doc = "DMA channel is cleared."]
    #[inline(always)]
    pub fn cleared(self) -> &'a mut W {
        self.variant(CLR26_A::CLEARED)
    }
}
#[doc = "Field `CLR27` reader - Writing ones to this register clears the corresponding bits in ENABLESET0."]
pub type CLR27_R = crate::BitReader<CLR27_A>;
#[doc = "Writing ones to this register clears the corresponding bits in ENABLESET0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CLR27_A {
    #[doc = "0: No effect."]
    NO_EFFECT = 0,
    #[doc = "1: DMA channel is cleared."]
    CLEARED = 1,
}
impl From<CLR27_A> for bool {
    #[inline(always)]
    fn from(variant: CLR27_A) -> Self {
        variant as u8 != 0
    }
}
impl CLR27_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CLR27_A {
        match self.bits {
            false => CLR27_A::NO_EFFECT,
            true => CLR27_A::CLEARED,
        }
    }
    #[doc = "Checks if the value of the field is `NO_EFFECT`"]
    #[inline(always)]
    pub fn is_no_effect(&self) -> bool {
        *self == CLR27_A::NO_EFFECT
    }
    #[doc = "Checks if the value of the field is `CLEARED`"]
    #[inline(always)]
    pub fn is_cleared(&self) -> bool {
        *self == CLR27_A::CLEARED
    }
}
#[doc = "Field `CLR27` writer - Writing ones to this register clears the corresponding bits in ENABLESET0."]
pub type CLR27_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, ENABLECLR0_SPEC, CLR27_A, O>;
impl<'a, const O: u8> CLR27_W<'a, O> {
    #[doc = "No effect."]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(CLR27_A::NO_EFFECT)
    }
    #[doc = "DMA channel is cleared."]
    #[inline(always)]
    pub fn cleared(self) -> &'a mut W {
        self.variant(CLR27_A::CLEARED)
    }
}
#[doc = "Field `CLR28` reader - Writing ones to this register clears the corresponding bits in ENABLESET0."]
pub type CLR28_R = crate::BitReader<CLR28_A>;
#[doc = "Writing ones to this register clears the corresponding bits in ENABLESET0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CLR28_A {
    #[doc = "0: No effect."]
    NO_EFFECT = 0,
    #[doc = "1: DMA channel is cleared."]
    CLEARED = 1,
}
impl From<CLR28_A> for bool {
    #[inline(always)]
    fn from(variant: CLR28_A) -> Self {
        variant as u8 != 0
    }
}
impl CLR28_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CLR28_A {
        match self.bits {
            false => CLR28_A::NO_EFFECT,
            true => CLR28_A::CLEARED,
        }
    }
    #[doc = "Checks if the value of the field is `NO_EFFECT`"]
    #[inline(always)]
    pub fn is_no_effect(&self) -> bool {
        *self == CLR28_A::NO_EFFECT
    }
    #[doc = "Checks if the value of the field is `CLEARED`"]
    #[inline(always)]
    pub fn is_cleared(&self) -> bool {
        *self == CLR28_A::CLEARED
    }
}
#[doc = "Field `CLR28` writer - Writing ones to this register clears the corresponding bits in ENABLESET0."]
pub type CLR28_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, ENABLECLR0_SPEC, CLR28_A, O>;
impl<'a, const O: u8> CLR28_W<'a, O> {
    #[doc = "No effect."]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(CLR28_A::NO_EFFECT)
    }
    #[doc = "DMA channel is cleared."]
    #[inline(always)]
    pub fn cleared(self) -> &'a mut W {
        self.variant(CLR28_A::CLEARED)
    }
}
#[doc = "Field `CLR29` reader - Writing ones to this register clears the corresponding bits in ENABLESET0."]
pub type CLR29_R = crate::BitReader<CLR29_A>;
#[doc = "Writing ones to this register clears the corresponding bits in ENABLESET0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CLR29_A {
    #[doc = "0: No effect."]
    NO_EFFECT = 0,
    #[doc = "1: DMA channel is cleared."]
    CLEARED = 1,
}
impl From<CLR29_A> for bool {
    #[inline(always)]
    fn from(variant: CLR29_A) -> Self {
        variant as u8 != 0
    }
}
impl CLR29_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CLR29_A {
        match self.bits {
            false => CLR29_A::NO_EFFECT,
            true => CLR29_A::CLEARED,
        }
    }
    #[doc = "Checks if the value of the field is `NO_EFFECT`"]
    #[inline(always)]
    pub fn is_no_effect(&self) -> bool {
        *self == CLR29_A::NO_EFFECT
    }
    #[doc = "Checks if the value of the field is `CLEARED`"]
    #[inline(always)]
    pub fn is_cleared(&self) -> bool {
        *self == CLR29_A::CLEARED
    }
}
#[doc = "Field `CLR29` writer - Writing ones to this register clears the corresponding bits in ENABLESET0."]
pub type CLR29_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, ENABLECLR0_SPEC, CLR29_A, O>;
impl<'a, const O: u8> CLR29_W<'a, O> {
    #[doc = "No effect."]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(CLR29_A::NO_EFFECT)
    }
    #[doc = "DMA channel is cleared."]
    #[inline(always)]
    pub fn cleared(self) -> &'a mut W {
        self.variant(CLR29_A::CLEARED)
    }
}
#[doc = "Field `CLR30` reader - Writing ones to this register clears the corresponding bits in ENABLESET0."]
pub type CLR30_R = crate::BitReader<CLR30_A>;
#[doc = "Writing ones to this register clears the corresponding bits in ENABLESET0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CLR30_A {
    #[doc = "0: No effect."]
    NO_EFFECT = 0,
    #[doc = "1: DMA channel is cleared."]
    CLEARED = 1,
}
impl From<CLR30_A> for bool {
    #[inline(always)]
    fn from(variant: CLR30_A) -> Self {
        variant as u8 != 0
    }
}
impl CLR30_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CLR30_A {
        match self.bits {
            false => CLR30_A::NO_EFFECT,
            true => CLR30_A::CLEARED,
        }
    }
    #[doc = "Checks if the value of the field is `NO_EFFECT`"]
    #[inline(always)]
    pub fn is_no_effect(&self) -> bool {
        *self == CLR30_A::NO_EFFECT
    }
    #[doc = "Checks if the value of the field is `CLEARED`"]
    #[inline(always)]
    pub fn is_cleared(&self) -> bool {
        *self == CLR30_A::CLEARED
    }
}
#[doc = "Field `CLR30` writer - Writing ones to this register clears the corresponding bits in ENABLESET0."]
pub type CLR30_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, ENABLECLR0_SPEC, CLR30_A, O>;
impl<'a, const O: u8> CLR30_W<'a, O> {
    #[doc = "No effect."]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(CLR30_A::NO_EFFECT)
    }
    #[doc = "DMA channel is cleared."]
    #[inline(always)]
    pub fn cleared(self) -> &'a mut W {
        self.variant(CLR30_A::CLEARED)
    }
}
#[doc = "Field `CLR31` reader - Writing ones to this register clears the corresponding bits in ENABLESET0."]
pub type CLR31_R = crate::BitReader<CLR31_A>;
#[doc = "Writing ones to this register clears the corresponding bits in ENABLESET0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CLR31_A {
    #[doc = "0: No effect."]
    NO_EFFECT = 0,
    #[doc = "1: DMA channel is cleared."]
    CLEARED = 1,
}
impl From<CLR31_A> for bool {
    #[inline(always)]
    fn from(variant: CLR31_A) -> Self {
        variant as u8 != 0
    }
}
impl CLR31_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CLR31_A {
        match self.bits {
            false => CLR31_A::NO_EFFECT,
            true => CLR31_A::CLEARED,
        }
    }
    #[doc = "Checks if the value of the field is `NO_EFFECT`"]
    #[inline(always)]
    pub fn is_no_effect(&self) -> bool {
        *self == CLR31_A::NO_EFFECT
    }
    #[doc = "Checks if the value of the field is `CLEARED`"]
    #[inline(always)]
    pub fn is_cleared(&self) -> bool {
        *self == CLR31_A::CLEARED
    }
}
#[doc = "Field `CLR31` writer - Writing ones to this register clears the corresponding bits in ENABLESET0."]
pub type CLR31_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, ENABLECLR0_SPEC, CLR31_A, O>;
impl<'a, const O: u8> CLR31_W<'a, O> {
    #[doc = "No effect."]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(CLR31_A::NO_EFFECT)
    }
    #[doc = "DMA channel is cleared."]
    #[inline(always)]
    pub fn cleared(self) -> &'a mut W {
        self.variant(CLR31_A::CLEARED)
    }
}
impl R {
    #[doc = "Bit 0 - Writing ones to this register clears the corresponding bits in ENABLESET0."]
    #[inline(always)]
    pub fn clr0(&self) -> CLR0_R {
        CLR0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Writing ones to this register clears the corresponding bits in ENABLESET0."]
    #[inline(always)]
    pub fn clr1(&self) -> CLR1_R {
        CLR1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Writing ones to this register clears the corresponding bits in ENABLESET0."]
    #[inline(always)]
    pub fn clr2(&self) -> CLR2_R {
        CLR2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Writing ones to this register clears the corresponding bits in ENABLESET0."]
    #[inline(always)]
    pub fn clr3(&self) -> CLR3_R {
        CLR3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Writing ones to this register clears the corresponding bits in ENABLESET0."]
    #[inline(always)]
    pub fn clr4(&self) -> CLR4_R {
        CLR4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Writing ones to this register clears the corresponding bits in ENABLESET0."]
    #[inline(always)]
    pub fn clr5(&self) -> CLR5_R {
        CLR5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Writing ones to this register clears the corresponding bits in ENABLESET0."]
    #[inline(always)]
    pub fn clr6(&self) -> CLR6_R {
        CLR6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Writing ones to this register clears the corresponding bits in ENABLESET0."]
    #[inline(always)]
    pub fn clr7(&self) -> CLR7_R {
        CLR7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Writing ones to this register clears the corresponding bits in ENABLESET0."]
    #[inline(always)]
    pub fn clr8(&self) -> CLR8_R {
        CLR8_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Writing ones to this register clears the corresponding bits in ENABLESET0."]
    #[inline(always)]
    pub fn clr9(&self) -> CLR9_R {
        CLR9_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Writing ones to this register clears the corresponding bits in ENABLESET0."]
    #[inline(always)]
    pub fn clr10(&self) -> CLR10_R {
        CLR10_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Writing ones to this register clears the corresponding bits in ENABLESET0."]
    #[inline(always)]
    pub fn clr11(&self) -> CLR11_R {
        CLR11_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Writing ones to this register clears the corresponding bits in ENABLESET0."]
    #[inline(always)]
    pub fn clr12(&self) -> CLR12_R {
        CLR12_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Writing ones to this register clears the corresponding bits in ENABLESET0."]
    #[inline(always)]
    pub fn clr13(&self) -> CLR13_R {
        CLR13_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Writing ones to this register clears the corresponding bits in ENABLESET0."]
    #[inline(always)]
    pub fn clr14(&self) -> CLR14_R {
        CLR14_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Writing ones to this register clears the corresponding bits in ENABLESET0."]
    #[inline(always)]
    pub fn clr15(&self) -> CLR15_R {
        CLR15_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Writing ones to this register clears the corresponding bits in ENABLESET0."]
    #[inline(always)]
    pub fn clr16(&self) -> CLR16_R {
        CLR16_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Writing ones to this register clears the corresponding bits in ENABLESET0."]
    #[inline(always)]
    pub fn clr17(&self) -> CLR17_R {
        CLR17_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Writing ones to this register clears the corresponding bits in ENABLESET0."]
    #[inline(always)]
    pub fn clr18(&self) -> CLR18_R {
        CLR18_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Writing ones to this register clears the corresponding bits in ENABLESET0."]
    #[inline(always)]
    pub fn clr19(&self) -> CLR19_R {
        CLR19_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Writing ones to this register clears the corresponding bits in ENABLESET0."]
    #[inline(always)]
    pub fn clr20(&self) -> CLR20_R {
        CLR20_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Writing ones to this register clears the corresponding bits in ENABLESET0."]
    #[inline(always)]
    pub fn clr21(&self) -> CLR21_R {
        CLR21_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Writing ones to this register clears the corresponding bits in ENABLESET0."]
    #[inline(always)]
    pub fn clr22(&self) -> CLR22_R {
        CLR22_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Writing ones to this register clears the corresponding bits in ENABLESET0."]
    #[inline(always)]
    pub fn clr23(&self) -> CLR23_R {
        CLR23_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Writing ones to this register clears the corresponding bits in ENABLESET0."]
    #[inline(always)]
    pub fn clr24(&self) -> CLR24_R {
        CLR24_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Writing ones to this register clears the corresponding bits in ENABLESET0."]
    #[inline(always)]
    pub fn clr25(&self) -> CLR25_R {
        CLR25_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Writing ones to this register clears the corresponding bits in ENABLESET0."]
    #[inline(always)]
    pub fn clr26(&self) -> CLR26_R {
        CLR26_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Writing ones to this register clears the corresponding bits in ENABLESET0."]
    #[inline(always)]
    pub fn clr27(&self) -> CLR27_R {
        CLR27_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Writing ones to this register clears the corresponding bits in ENABLESET0."]
    #[inline(always)]
    pub fn clr28(&self) -> CLR28_R {
        CLR28_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Writing ones to this register clears the corresponding bits in ENABLESET0."]
    #[inline(always)]
    pub fn clr29(&self) -> CLR29_R {
        CLR29_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Writing ones to this register clears the corresponding bits in ENABLESET0."]
    #[inline(always)]
    pub fn clr30(&self) -> CLR30_R {
        CLR30_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Writing ones to this register clears the corresponding bits in ENABLESET0."]
    #[inline(always)]
    pub fn clr31(&self) -> CLR31_R {
        CLR31_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Writing ones to this register clears the corresponding bits in ENABLESET0."]
    #[inline(always)]
    #[must_use]
    pub fn clr0(&mut self) -> CLR0_W<0> {
        CLR0_W::new(self)
    }
    #[doc = "Bit 1 - Writing ones to this register clears the corresponding bits in ENABLESET0."]
    #[inline(always)]
    #[must_use]
    pub fn clr1(&mut self) -> CLR1_W<1> {
        CLR1_W::new(self)
    }
    #[doc = "Bit 2 - Writing ones to this register clears the corresponding bits in ENABLESET0."]
    #[inline(always)]
    #[must_use]
    pub fn clr2(&mut self) -> CLR2_W<2> {
        CLR2_W::new(self)
    }
    #[doc = "Bit 3 - Writing ones to this register clears the corresponding bits in ENABLESET0."]
    #[inline(always)]
    #[must_use]
    pub fn clr3(&mut self) -> CLR3_W<3> {
        CLR3_W::new(self)
    }
    #[doc = "Bit 4 - Writing ones to this register clears the corresponding bits in ENABLESET0."]
    #[inline(always)]
    #[must_use]
    pub fn clr4(&mut self) -> CLR4_W<4> {
        CLR4_W::new(self)
    }
    #[doc = "Bit 5 - Writing ones to this register clears the corresponding bits in ENABLESET0."]
    #[inline(always)]
    #[must_use]
    pub fn clr5(&mut self) -> CLR5_W<5> {
        CLR5_W::new(self)
    }
    #[doc = "Bit 6 - Writing ones to this register clears the corresponding bits in ENABLESET0."]
    #[inline(always)]
    #[must_use]
    pub fn clr6(&mut self) -> CLR6_W<6> {
        CLR6_W::new(self)
    }
    #[doc = "Bit 7 - Writing ones to this register clears the corresponding bits in ENABLESET0."]
    #[inline(always)]
    #[must_use]
    pub fn clr7(&mut self) -> CLR7_W<7> {
        CLR7_W::new(self)
    }
    #[doc = "Bit 8 - Writing ones to this register clears the corresponding bits in ENABLESET0."]
    #[inline(always)]
    #[must_use]
    pub fn clr8(&mut self) -> CLR8_W<8> {
        CLR8_W::new(self)
    }
    #[doc = "Bit 9 - Writing ones to this register clears the corresponding bits in ENABLESET0."]
    #[inline(always)]
    #[must_use]
    pub fn clr9(&mut self) -> CLR9_W<9> {
        CLR9_W::new(self)
    }
    #[doc = "Bit 10 - Writing ones to this register clears the corresponding bits in ENABLESET0."]
    #[inline(always)]
    #[must_use]
    pub fn clr10(&mut self) -> CLR10_W<10> {
        CLR10_W::new(self)
    }
    #[doc = "Bit 11 - Writing ones to this register clears the corresponding bits in ENABLESET0."]
    #[inline(always)]
    #[must_use]
    pub fn clr11(&mut self) -> CLR11_W<11> {
        CLR11_W::new(self)
    }
    #[doc = "Bit 12 - Writing ones to this register clears the corresponding bits in ENABLESET0."]
    #[inline(always)]
    #[must_use]
    pub fn clr12(&mut self) -> CLR12_W<12> {
        CLR12_W::new(self)
    }
    #[doc = "Bit 13 - Writing ones to this register clears the corresponding bits in ENABLESET0."]
    #[inline(always)]
    #[must_use]
    pub fn clr13(&mut self) -> CLR13_W<13> {
        CLR13_W::new(self)
    }
    #[doc = "Bit 14 - Writing ones to this register clears the corresponding bits in ENABLESET0."]
    #[inline(always)]
    #[must_use]
    pub fn clr14(&mut self) -> CLR14_W<14> {
        CLR14_W::new(self)
    }
    #[doc = "Bit 15 - Writing ones to this register clears the corresponding bits in ENABLESET0."]
    #[inline(always)]
    #[must_use]
    pub fn clr15(&mut self) -> CLR15_W<15> {
        CLR15_W::new(self)
    }
    #[doc = "Bit 16 - Writing ones to this register clears the corresponding bits in ENABLESET0."]
    #[inline(always)]
    #[must_use]
    pub fn clr16(&mut self) -> CLR16_W<16> {
        CLR16_W::new(self)
    }
    #[doc = "Bit 17 - Writing ones to this register clears the corresponding bits in ENABLESET0."]
    #[inline(always)]
    #[must_use]
    pub fn clr17(&mut self) -> CLR17_W<17> {
        CLR17_W::new(self)
    }
    #[doc = "Bit 18 - Writing ones to this register clears the corresponding bits in ENABLESET0."]
    #[inline(always)]
    #[must_use]
    pub fn clr18(&mut self) -> CLR18_W<18> {
        CLR18_W::new(self)
    }
    #[doc = "Bit 19 - Writing ones to this register clears the corresponding bits in ENABLESET0."]
    #[inline(always)]
    #[must_use]
    pub fn clr19(&mut self) -> CLR19_W<19> {
        CLR19_W::new(self)
    }
    #[doc = "Bit 20 - Writing ones to this register clears the corresponding bits in ENABLESET0."]
    #[inline(always)]
    #[must_use]
    pub fn clr20(&mut self) -> CLR20_W<20> {
        CLR20_W::new(self)
    }
    #[doc = "Bit 21 - Writing ones to this register clears the corresponding bits in ENABLESET0."]
    #[inline(always)]
    #[must_use]
    pub fn clr21(&mut self) -> CLR21_W<21> {
        CLR21_W::new(self)
    }
    #[doc = "Bit 22 - Writing ones to this register clears the corresponding bits in ENABLESET0."]
    #[inline(always)]
    #[must_use]
    pub fn clr22(&mut self) -> CLR22_W<22> {
        CLR22_W::new(self)
    }
    #[doc = "Bit 23 - Writing ones to this register clears the corresponding bits in ENABLESET0."]
    #[inline(always)]
    #[must_use]
    pub fn clr23(&mut self) -> CLR23_W<23> {
        CLR23_W::new(self)
    }
    #[doc = "Bit 24 - Writing ones to this register clears the corresponding bits in ENABLESET0."]
    #[inline(always)]
    #[must_use]
    pub fn clr24(&mut self) -> CLR24_W<24> {
        CLR24_W::new(self)
    }
    #[doc = "Bit 25 - Writing ones to this register clears the corresponding bits in ENABLESET0."]
    #[inline(always)]
    #[must_use]
    pub fn clr25(&mut self) -> CLR25_W<25> {
        CLR25_W::new(self)
    }
    #[doc = "Bit 26 - Writing ones to this register clears the corresponding bits in ENABLESET0."]
    #[inline(always)]
    #[must_use]
    pub fn clr26(&mut self) -> CLR26_W<26> {
        CLR26_W::new(self)
    }
    #[doc = "Bit 27 - Writing ones to this register clears the corresponding bits in ENABLESET0."]
    #[inline(always)]
    #[must_use]
    pub fn clr27(&mut self) -> CLR27_W<27> {
        CLR27_W::new(self)
    }
    #[doc = "Bit 28 - Writing ones to this register clears the corresponding bits in ENABLESET0."]
    #[inline(always)]
    #[must_use]
    pub fn clr28(&mut self) -> CLR28_W<28> {
        CLR28_W::new(self)
    }
    #[doc = "Bit 29 - Writing ones to this register clears the corresponding bits in ENABLESET0."]
    #[inline(always)]
    #[must_use]
    pub fn clr29(&mut self) -> CLR29_W<29> {
        CLR29_W::new(self)
    }
    #[doc = "Bit 30 - Writing ones to this register clears the corresponding bits in ENABLESET0."]
    #[inline(always)]
    #[must_use]
    pub fn clr30(&mut self) -> CLR30_W<30> {
        CLR30_W::new(self)
    }
    #[doc = "Bit 31 - Writing ones to this register clears the corresponding bits in ENABLESET0."]
    #[inline(always)]
    #[must_use]
    pub fn clr31(&mut self) -> CLR31_W<31> {
        CLR31_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Channel Enable Clear for all DMA channels\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [enableclr0](index.html) module"]
pub struct ENABLECLR0_SPEC;
impl crate::RegisterSpec for ENABLECLR0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [enableclr0::R](R) reader structure"]
impl crate::Readable for ENABLECLR0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [enableclr0::W](W) writer structure"]
impl crate::Writable for ENABLECLR0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0xffff_ffff;
}
#[doc = "`reset()` method sets ENABLECLR0 to value 0"]
impl crate::Resettable for ENABLECLR0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
