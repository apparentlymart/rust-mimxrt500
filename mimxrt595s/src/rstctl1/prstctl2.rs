#[doc = "Register `PRSTCTL2` reader"]
pub struct R(crate::R<PRSTCTL2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PRSTCTL2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PRSTCTL2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PRSTCTL2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PRSTCTL2` writer"]
pub struct W(crate::W<PRSTCTL2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PRSTCTL2_SPEC>;
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
impl From<crate::W<PRSTCTL2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PRSTCTL2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CT32BIT0` reader - CT32BIT\\[4:0\\]
reset"]
pub type CT32BIT0_R = crate::BitReader<CT32BIT0_A>;
#[doc = "CT32BIT\\[4:0\\]
reset\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CT32BIT0_A {
    #[doc = "0: Clear Reset"]
    CT32BIT_CLR = 0,
    #[doc = "1: Set Reset"]
    CT32BIT_SET = 1,
}
impl From<CT32BIT0_A> for bool {
    #[inline(always)]
    fn from(variant: CT32BIT0_A) -> Self {
        variant as u8 != 0
    }
}
impl CT32BIT0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CT32BIT0_A {
        match self.bits {
            false => CT32BIT0_A::CT32BIT_CLR,
            true => CT32BIT0_A::CT32BIT_SET,
        }
    }
    #[doc = "Checks if the value of the field is `CT32BIT_CLR`"]
    #[inline(always)]
    pub fn is_ct32bit_clr(&self) -> bool {
        *self == CT32BIT0_A::CT32BIT_CLR
    }
    #[doc = "Checks if the value of the field is `CT32BIT_SET`"]
    #[inline(always)]
    pub fn is_ct32bit_set(&self) -> bool {
        *self == CT32BIT0_A::CT32BIT_SET
    }
}
#[doc = "Field `CT32BIT0` writer - CT32BIT\\[4:0\\]
reset"]
pub type CT32BIT0_W<'a, const O: u8> = crate::BitWriter<'a, u32, PRSTCTL2_SPEC, CT32BIT0_A, O>;
impl<'a, const O: u8> CT32BIT0_W<'a, O> {
    #[doc = "Clear Reset"]
    #[inline(always)]
    pub fn ct32bit_clr(self) -> &'a mut W {
        self.variant(CT32BIT0_A::CT32BIT_CLR)
    }
    #[doc = "Set Reset"]
    #[inline(always)]
    pub fn ct32bit_set(self) -> &'a mut W {
        self.variant(CT32BIT0_A::CT32BIT_SET)
    }
}
#[doc = "Field `CT32BIT1` reader - CT32BIT\\[4:0\\]
reset"]
pub type CT32BIT1_R = crate::BitReader<CT32BIT1_A>;
#[doc = "CT32BIT\\[4:0\\]
reset\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CT32BIT1_A {
    #[doc = "0: Clear Reset"]
    CT32BIT_CLR = 0,
    #[doc = "1: Set Reset"]
    CT32BIT_SET = 1,
}
impl From<CT32BIT1_A> for bool {
    #[inline(always)]
    fn from(variant: CT32BIT1_A) -> Self {
        variant as u8 != 0
    }
}
impl CT32BIT1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CT32BIT1_A {
        match self.bits {
            false => CT32BIT1_A::CT32BIT_CLR,
            true => CT32BIT1_A::CT32BIT_SET,
        }
    }
    #[doc = "Checks if the value of the field is `CT32BIT_CLR`"]
    #[inline(always)]
    pub fn is_ct32bit_clr(&self) -> bool {
        *self == CT32BIT1_A::CT32BIT_CLR
    }
    #[doc = "Checks if the value of the field is `CT32BIT_SET`"]
    #[inline(always)]
    pub fn is_ct32bit_set(&self) -> bool {
        *self == CT32BIT1_A::CT32BIT_SET
    }
}
#[doc = "Field `CT32BIT1` writer - CT32BIT\\[4:0\\]
reset"]
pub type CT32BIT1_W<'a, const O: u8> = crate::BitWriter<'a, u32, PRSTCTL2_SPEC, CT32BIT1_A, O>;
impl<'a, const O: u8> CT32BIT1_W<'a, O> {
    #[doc = "Clear Reset"]
    #[inline(always)]
    pub fn ct32bit_clr(self) -> &'a mut W {
        self.variant(CT32BIT1_A::CT32BIT_CLR)
    }
    #[doc = "Set Reset"]
    #[inline(always)]
    pub fn ct32bit_set(self) -> &'a mut W {
        self.variant(CT32BIT1_A::CT32BIT_SET)
    }
}
#[doc = "Field `CT32BIT2` reader - CT32BIT\\[4:0\\]
reset"]
pub type CT32BIT2_R = crate::BitReader<CT32BIT2_A>;
#[doc = "CT32BIT\\[4:0\\]
reset\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CT32BIT2_A {
    #[doc = "0: Clear Reset"]
    CT32BIT_CLR = 0,
    #[doc = "1: Set Reset"]
    CT32BIT_SET = 1,
}
impl From<CT32BIT2_A> for bool {
    #[inline(always)]
    fn from(variant: CT32BIT2_A) -> Self {
        variant as u8 != 0
    }
}
impl CT32BIT2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CT32BIT2_A {
        match self.bits {
            false => CT32BIT2_A::CT32BIT_CLR,
            true => CT32BIT2_A::CT32BIT_SET,
        }
    }
    #[doc = "Checks if the value of the field is `CT32BIT_CLR`"]
    #[inline(always)]
    pub fn is_ct32bit_clr(&self) -> bool {
        *self == CT32BIT2_A::CT32BIT_CLR
    }
    #[doc = "Checks if the value of the field is `CT32BIT_SET`"]
    #[inline(always)]
    pub fn is_ct32bit_set(&self) -> bool {
        *self == CT32BIT2_A::CT32BIT_SET
    }
}
#[doc = "Field `CT32BIT2` writer - CT32BIT\\[4:0\\]
reset"]
pub type CT32BIT2_W<'a, const O: u8> = crate::BitWriter<'a, u32, PRSTCTL2_SPEC, CT32BIT2_A, O>;
impl<'a, const O: u8> CT32BIT2_W<'a, O> {
    #[doc = "Clear Reset"]
    #[inline(always)]
    pub fn ct32bit_clr(self) -> &'a mut W {
        self.variant(CT32BIT2_A::CT32BIT_CLR)
    }
    #[doc = "Set Reset"]
    #[inline(always)]
    pub fn ct32bit_set(self) -> &'a mut W {
        self.variant(CT32BIT2_A::CT32BIT_SET)
    }
}
#[doc = "Field `CT32BIT3` reader - CT32BIT\\[4:0\\]
reset"]
pub type CT32BIT3_R = crate::BitReader<CT32BIT3_A>;
#[doc = "CT32BIT\\[4:0\\]
reset\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CT32BIT3_A {
    #[doc = "0: Clear Reset"]
    CT32BIT_CLR = 0,
    #[doc = "1: Set Reset"]
    CT32BIT_SET = 1,
}
impl From<CT32BIT3_A> for bool {
    #[inline(always)]
    fn from(variant: CT32BIT3_A) -> Self {
        variant as u8 != 0
    }
}
impl CT32BIT3_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CT32BIT3_A {
        match self.bits {
            false => CT32BIT3_A::CT32BIT_CLR,
            true => CT32BIT3_A::CT32BIT_SET,
        }
    }
    #[doc = "Checks if the value of the field is `CT32BIT_CLR`"]
    #[inline(always)]
    pub fn is_ct32bit_clr(&self) -> bool {
        *self == CT32BIT3_A::CT32BIT_CLR
    }
    #[doc = "Checks if the value of the field is `CT32BIT_SET`"]
    #[inline(always)]
    pub fn is_ct32bit_set(&self) -> bool {
        *self == CT32BIT3_A::CT32BIT_SET
    }
}
#[doc = "Field `CT32BIT3` writer - CT32BIT\\[4:0\\]
reset"]
pub type CT32BIT3_W<'a, const O: u8> = crate::BitWriter<'a, u32, PRSTCTL2_SPEC, CT32BIT3_A, O>;
impl<'a, const O: u8> CT32BIT3_W<'a, O> {
    #[doc = "Clear Reset"]
    #[inline(always)]
    pub fn ct32bit_clr(self) -> &'a mut W {
        self.variant(CT32BIT3_A::CT32BIT_CLR)
    }
    #[doc = "Set Reset"]
    #[inline(always)]
    pub fn ct32bit_set(self) -> &'a mut W {
        self.variant(CT32BIT3_A::CT32BIT_SET)
    }
}
#[doc = "Field `CT32BIT4` reader - CT32BIT\\[4:0\\]
reset"]
pub type CT32BIT4_R = crate::BitReader<CT32BIT4_A>;
#[doc = "CT32BIT\\[4:0\\]
reset\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CT32BIT4_A {
    #[doc = "0: Clear Reset"]
    CT32BIT_CLR = 0,
    #[doc = "1: Set Reset"]
    CT32BIT_SET = 1,
}
impl From<CT32BIT4_A> for bool {
    #[inline(always)]
    fn from(variant: CT32BIT4_A) -> Self {
        variant as u8 != 0
    }
}
impl CT32BIT4_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CT32BIT4_A {
        match self.bits {
            false => CT32BIT4_A::CT32BIT_CLR,
            true => CT32BIT4_A::CT32BIT_SET,
        }
    }
    #[doc = "Checks if the value of the field is `CT32BIT_CLR`"]
    #[inline(always)]
    pub fn is_ct32bit_clr(&self) -> bool {
        *self == CT32BIT4_A::CT32BIT_CLR
    }
    #[doc = "Checks if the value of the field is `CT32BIT_SET`"]
    #[inline(always)]
    pub fn is_ct32bit_set(&self) -> bool {
        *self == CT32BIT4_A::CT32BIT_SET
    }
}
#[doc = "Field `CT32BIT4` writer - CT32BIT\\[4:0\\]
reset"]
pub type CT32BIT4_W<'a, const O: u8> = crate::BitWriter<'a, u32, PRSTCTL2_SPEC, CT32BIT4_A, O>;
impl<'a, const O: u8> CT32BIT4_W<'a, O> {
    #[doc = "Clear Reset"]
    #[inline(always)]
    pub fn ct32bit_clr(self) -> &'a mut W {
        self.variant(CT32BIT4_A::CT32BIT_CLR)
    }
    #[doc = "Set Reset"]
    #[inline(always)]
    pub fn ct32bit_set(self) -> &'a mut W {
        self.variant(CT32BIT4_A::CT32BIT_SET)
    }
}
#[doc = "Field `MRT0` reader - MRT0 reset control"]
pub type MRT0_R = crate::BitReader<MRT0_A>;
#[doc = "MRT0 reset control\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MRT0_A {
    #[doc = "0: Clear Reset"]
    MRT0_CLR = 0,
    #[doc = "1: Set Reset"]
    MRT0_SET = 1,
}
impl From<MRT0_A> for bool {
    #[inline(always)]
    fn from(variant: MRT0_A) -> Self {
        variant as u8 != 0
    }
}
impl MRT0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MRT0_A {
        match self.bits {
            false => MRT0_A::MRT0_CLR,
            true => MRT0_A::MRT0_SET,
        }
    }
    #[doc = "Checks if the value of the field is `MRT0_CLR`"]
    #[inline(always)]
    pub fn is_mrt0_clr(&self) -> bool {
        *self == MRT0_A::MRT0_CLR
    }
    #[doc = "Checks if the value of the field is `MRT0_SET`"]
    #[inline(always)]
    pub fn is_mrt0_set(&self) -> bool {
        *self == MRT0_A::MRT0_SET
    }
}
#[doc = "Field `MRT0` writer - MRT0 reset control"]
pub type MRT0_W<'a, const O: u8> = crate::BitWriter<'a, u32, PRSTCTL2_SPEC, MRT0_A, O>;
impl<'a, const O: u8> MRT0_W<'a, O> {
    #[doc = "Clear Reset"]
    #[inline(always)]
    pub fn mrt0_clr(self) -> &'a mut W {
        self.variant(MRT0_A::MRT0_CLR)
    }
    #[doc = "Set Reset"]
    #[inline(always)]
    pub fn mrt0_set(self) -> &'a mut W {
        self.variant(MRT0_A::MRT0_SET)
    }
}
#[doc = "Field `WWDT1` reader - WWDT1 reset control"]
pub type WWDT1_R = crate::BitReader<WWDT1_A>;
#[doc = "WWDT1 reset control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WWDT1_A {
    #[doc = "0: Clear Reset"]
    WWDT1_CLR = 0,
    #[doc = "1: Set Reset"]
    WWDT1_SET = 1,
}
impl From<WWDT1_A> for bool {
    #[inline(always)]
    fn from(variant: WWDT1_A) -> Self {
        variant as u8 != 0
    }
}
impl WWDT1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WWDT1_A {
        match self.bits {
            false => WWDT1_A::WWDT1_CLR,
            true => WWDT1_A::WWDT1_SET,
        }
    }
    #[doc = "Checks if the value of the field is `WWDT1_CLR`"]
    #[inline(always)]
    pub fn is_wwdt1_clr(&self) -> bool {
        *self == WWDT1_A::WWDT1_CLR
    }
    #[doc = "Checks if the value of the field is `WWDT1_SET`"]
    #[inline(always)]
    pub fn is_wwdt1_set(&self) -> bool {
        *self == WWDT1_A::WWDT1_SET
    }
}
#[doc = "Field `WWDT1` writer - WWDT1 reset control"]
pub type WWDT1_W<'a, const O: u8> = crate::BitWriter<'a, u32, PRSTCTL2_SPEC, WWDT1_A, O>;
impl<'a, const O: u8> WWDT1_W<'a, O> {
    #[doc = "Clear Reset"]
    #[inline(always)]
    pub fn wwdt1_clr(self) -> &'a mut W {
        self.variant(WWDT1_A::WWDT1_CLR)
    }
    #[doc = "Set Reset"]
    #[inline(always)]
    pub fn wwdt1_set(self) -> &'a mut W {
        self.variant(WWDT1_A::WWDT1_SET)
    }
}
#[doc = "Field `I3C0` reader - I3C0 reset control"]
pub type I3C0_R = crate::BitReader<I3C0_A>;
#[doc = "I3C0 reset control\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum I3C0_A {
    #[doc = "0: Clear Reset"]
    I3C_CLR = 0,
    #[doc = "1: Set Reset"]
    I3C_SET = 1,
}
impl From<I3C0_A> for bool {
    #[inline(always)]
    fn from(variant: I3C0_A) -> Self {
        variant as u8 != 0
    }
}
impl I3C0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> I3C0_A {
        match self.bits {
            false => I3C0_A::I3C_CLR,
            true => I3C0_A::I3C_SET,
        }
    }
    #[doc = "Checks if the value of the field is `I3C_CLR`"]
    #[inline(always)]
    pub fn is_i3c_clr(&self) -> bool {
        *self == I3C0_A::I3C_CLR
    }
    #[doc = "Checks if the value of the field is `I3C_SET`"]
    #[inline(always)]
    pub fn is_i3c_set(&self) -> bool {
        *self == I3C0_A::I3C_SET
    }
}
#[doc = "Field `I3C0` writer - I3C0 reset control"]
pub type I3C0_W<'a, const O: u8> = crate::BitWriter<'a, u32, PRSTCTL2_SPEC, I3C0_A, O>;
impl<'a, const O: u8> I3C0_W<'a, O> {
    #[doc = "Clear Reset"]
    #[inline(always)]
    pub fn i3c_clr(self) -> &'a mut W {
        self.variant(I3C0_A::I3C_CLR)
    }
    #[doc = "Set Reset"]
    #[inline(always)]
    pub fn i3c_set(self) -> &'a mut W {
        self.variant(I3C0_A::I3C_SET)
    }
}
#[doc = "Field `I3C1` reader - I3C1 reset control"]
pub type I3C1_R = crate::BitReader<I3C1_A>;
#[doc = "I3C1 reset control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum I3C1_A {
    #[doc = "0: Clear Reset"]
    I3C_CLR = 0,
    #[doc = "1: Set Reset"]
    I3C_SET = 1,
}
impl From<I3C1_A> for bool {
    #[inline(always)]
    fn from(variant: I3C1_A) -> Self {
        variant as u8 != 0
    }
}
impl I3C1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> I3C1_A {
        match self.bits {
            false => I3C1_A::I3C_CLR,
            true => I3C1_A::I3C_SET,
        }
    }
    #[doc = "Checks if the value of the field is `I3C_CLR`"]
    #[inline(always)]
    pub fn is_i3c_clr(&self) -> bool {
        *self == I3C1_A::I3C_CLR
    }
    #[doc = "Checks if the value of the field is `I3C_SET`"]
    #[inline(always)]
    pub fn is_i3c_set(&self) -> bool {
        *self == I3C1_A::I3C_SET
    }
}
#[doc = "Field `I3C1` writer - I3C1 reset control"]
pub type I3C1_W<'a, const O: u8> = crate::BitWriter<'a, u32, PRSTCTL2_SPEC, I3C1_A, O>;
impl<'a, const O: u8> I3C1_W<'a, O> {
    #[doc = "Clear Reset"]
    #[inline(always)]
    pub fn i3c_clr(self) -> &'a mut W {
        self.variant(I3C1_A::I3C_CLR)
    }
    #[doc = "Set Reset"]
    #[inline(always)]
    pub fn i3c_set(self) -> &'a mut W {
        self.variant(I3C1_A::I3C_SET)
    }
}
#[doc = "Field `GPIOINTCTL` reader - GPIOINTCTL reset control"]
pub type GPIOINTCTL_R = crate::BitReader<GPIOINTCTL_A>;
#[doc = "GPIOINTCTL reset control\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GPIOINTCTL_A {
    #[doc = "0: Clear Reset"]
    GPIOINTCTL_CLR = 0,
    #[doc = "1: Set Reset"]
    GPIOINTCTL_SET = 1,
}
impl From<GPIOINTCTL_A> for bool {
    #[inline(always)]
    fn from(variant: GPIOINTCTL_A) -> Self {
        variant as u8 != 0
    }
}
impl GPIOINTCTL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIOINTCTL_A {
        match self.bits {
            false => GPIOINTCTL_A::GPIOINTCTL_CLR,
            true => GPIOINTCTL_A::GPIOINTCTL_SET,
        }
    }
    #[doc = "Checks if the value of the field is `GPIOINTCTL_CLR`"]
    #[inline(always)]
    pub fn is_gpiointctl_clr(&self) -> bool {
        *self == GPIOINTCTL_A::GPIOINTCTL_CLR
    }
    #[doc = "Checks if the value of the field is `GPIOINTCTL_SET`"]
    #[inline(always)]
    pub fn is_gpiointctl_set(&self) -> bool {
        *self == GPIOINTCTL_A::GPIOINTCTL_SET
    }
}
#[doc = "Field `GPIOINTCTL` writer - GPIOINTCTL reset control"]
pub type GPIOINTCTL_W<'a, const O: u8> = crate::BitWriter<'a, u32, PRSTCTL2_SPEC, GPIOINTCTL_A, O>;
impl<'a, const O: u8> GPIOINTCTL_W<'a, O> {
    #[doc = "Clear Reset"]
    #[inline(always)]
    pub fn gpiointctl_clr(self) -> &'a mut W {
        self.variant(GPIOINTCTL_A::GPIOINTCTL_CLR)
    }
    #[doc = "Set Reset"]
    #[inline(always)]
    pub fn gpiointctl_set(self) -> &'a mut W {
        self.variant(GPIOINTCTL_A::GPIOINTCTL_SET)
    }
}
#[doc = "Field `PIMCTL` reader - INPUTMUX reset control"]
pub type PIMCTL_R = crate::BitReader<PIMCTL_A>;
#[doc = "INPUTMUX reset control\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PIMCTL_A {
    #[doc = "0: Clear Reset"]
    PIMCTL_CLR = 0,
    #[doc = "1: Set Reset"]
    PIMCTL_SET = 1,
}
impl From<PIMCTL_A> for bool {
    #[inline(always)]
    fn from(variant: PIMCTL_A) -> Self {
        variant as u8 != 0
    }
}
impl PIMCTL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PIMCTL_A {
        match self.bits {
            false => PIMCTL_A::PIMCTL_CLR,
            true => PIMCTL_A::PIMCTL_SET,
        }
    }
    #[doc = "Checks if the value of the field is `PIMCTL_CLR`"]
    #[inline(always)]
    pub fn is_pimctl_clr(&self) -> bool {
        *self == PIMCTL_A::PIMCTL_CLR
    }
    #[doc = "Checks if the value of the field is `PIMCTL_SET`"]
    #[inline(always)]
    pub fn is_pimctl_set(&self) -> bool {
        *self == PIMCTL_A::PIMCTL_SET
    }
}
#[doc = "Field `PIMCTL` writer - INPUTMUX reset control"]
pub type PIMCTL_W<'a, const O: u8> = crate::BitWriter<'a, u32, PRSTCTL2_SPEC, PIMCTL_A, O>;
impl<'a, const O: u8> PIMCTL_W<'a, O> {
    #[doc = "Clear Reset"]
    #[inline(always)]
    pub fn pimctl_clr(self) -> &'a mut W {
        self.variant(PIMCTL_A::PIMCTL_CLR)
    }
    #[doc = "Set Reset"]
    #[inline(always)]
    pub fn pimctl_set(self) -> &'a mut W {
        self.variant(PIMCTL_A::PIMCTL_SET)
    }
}
impl R {
    #[doc = "Bit 0 - CT32BIT\\[4:0\\]
reset"]
    #[inline(always)]
    pub fn ct32bit0(&self) -> CT32BIT0_R {
        CT32BIT0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - CT32BIT\\[4:0\\]
reset"]
    #[inline(always)]
    pub fn ct32bit1(&self) -> CT32BIT1_R {
        CT32BIT1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - CT32BIT\\[4:0\\]
reset"]
    #[inline(always)]
    pub fn ct32bit2(&self) -> CT32BIT2_R {
        CT32BIT2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - CT32BIT\\[4:0\\]
reset"]
    #[inline(always)]
    pub fn ct32bit3(&self) -> CT32BIT3_R {
        CT32BIT3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - CT32BIT\\[4:0\\]
reset"]
    #[inline(always)]
    pub fn ct32bit4(&self) -> CT32BIT4_R {
        CT32BIT4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 8 - MRT0 reset control"]
    #[inline(always)]
    pub fn mrt0(&self) -> MRT0_R {
        MRT0_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 10 - WWDT1 reset control"]
    #[inline(always)]
    pub fn wwdt1(&self) -> WWDT1_R {
        WWDT1_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 16 - I3C0 reset control"]
    #[inline(always)]
    pub fn i3c0(&self) -> I3C0_R {
        I3C0_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - I3C1 reset control"]
    #[inline(always)]
    pub fn i3c1(&self) -> I3C1_R {
        I3C1_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 30 - GPIOINTCTL reset control"]
    #[inline(always)]
    pub fn gpiointctl(&self) -> GPIOINTCTL_R {
        GPIOINTCTL_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - INPUTMUX reset control"]
    #[inline(always)]
    pub fn pimctl(&self) -> PIMCTL_R {
        PIMCTL_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - CT32BIT\\[4:0\\]
reset"]
    #[inline(always)]
    #[must_use]
    pub fn ct32bit0(&mut self) -> CT32BIT0_W<0> {
        CT32BIT0_W::new(self)
    }
    #[doc = "Bit 1 - CT32BIT\\[4:0\\]
reset"]
    #[inline(always)]
    #[must_use]
    pub fn ct32bit1(&mut self) -> CT32BIT1_W<1> {
        CT32BIT1_W::new(self)
    }
    #[doc = "Bit 2 - CT32BIT\\[4:0\\]
reset"]
    #[inline(always)]
    #[must_use]
    pub fn ct32bit2(&mut self) -> CT32BIT2_W<2> {
        CT32BIT2_W::new(self)
    }
    #[doc = "Bit 3 - CT32BIT\\[4:0\\]
reset"]
    #[inline(always)]
    #[must_use]
    pub fn ct32bit3(&mut self) -> CT32BIT3_W<3> {
        CT32BIT3_W::new(self)
    }
    #[doc = "Bit 4 - CT32BIT\\[4:0\\]
reset"]
    #[inline(always)]
    #[must_use]
    pub fn ct32bit4(&mut self) -> CT32BIT4_W<4> {
        CT32BIT4_W::new(self)
    }
    #[doc = "Bit 8 - MRT0 reset control"]
    #[inline(always)]
    #[must_use]
    pub fn mrt0(&mut self) -> MRT0_W<8> {
        MRT0_W::new(self)
    }
    #[doc = "Bit 10 - WWDT1 reset control"]
    #[inline(always)]
    #[must_use]
    pub fn wwdt1(&mut self) -> WWDT1_W<10> {
        WWDT1_W::new(self)
    }
    #[doc = "Bit 16 - I3C0 reset control"]
    #[inline(always)]
    #[must_use]
    pub fn i3c0(&mut self) -> I3C0_W<16> {
        I3C0_W::new(self)
    }
    #[doc = "Bit 17 - I3C1 reset control"]
    #[inline(always)]
    #[must_use]
    pub fn i3c1(&mut self) -> I3C1_W<17> {
        I3C1_W::new(self)
    }
    #[doc = "Bit 30 - GPIOINTCTL reset control"]
    #[inline(always)]
    #[must_use]
    pub fn gpiointctl(&mut self) -> GPIOINTCTL_W<30> {
        GPIOINTCTL_W::new(self)
    }
    #[doc = "Bit 31 - INPUTMUX reset control"]
    #[inline(always)]
    #[must_use]
    pub fn pimctl(&mut self) -> PIMCTL_W<31> {
        PIMCTL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Peripheral Reset Control Register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [prstctl2](index.html) module"]
pub struct PRSTCTL2_SPEC;
impl crate::RegisterSpec for PRSTCTL2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [prstctl2::R](R) reader structure"]
impl crate::Readable for PRSTCTL2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [prstctl2::W](W) writer structure"]
impl crate::Writable for PRSTCTL2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PRSTCTL2 to value 0xc001_011f"]
impl crate::Resettable for PRSTCTL2_SPEC {
    const RESET_VALUE: Self::Ux = 0xc001_011f;
}
