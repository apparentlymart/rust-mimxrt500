#[doc = "Register `PSCCTL2_CLR` reader"]
pub struct R(crate::R<PSCCTL2_CLR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PSCCTL2_CLR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PSCCTL2_CLR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PSCCTL2_CLR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PSCCTL2_CLR` writer"]
pub struct W(crate::W<PSCCTL2_CLR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PSCCTL2_CLR_SPEC>;
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
impl From<crate::W<PSCCTL2_CLR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PSCCTL2_CLR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "CT32BIT bit timer 0 clock clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CT32BIT0_CLK_AW {
    #[doc = "0: No Effect"]
    NO_EFFECT = 0,
    #[doc = "1: Set Bit"]
    SET_BIT = 1,
}
impl From<CT32BIT0_CLK_AW> for bool {
    #[inline(always)]
    fn from(variant: CT32BIT0_CLK_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CT32BIT0_CLK` writer - CT32BIT bit timer 0 clock clear"]
pub type CT32BIT0_CLK_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, PSCCTL2_CLR_SPEC, CT32BIT0_CLK_AW, O>;
impl<'a, const O: u8> CT32BIT0_CLK_W<'a, O> {
    #[doc = "No Effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(CT32BIT0_CLK_AW::NO_EFFECT)
    }
    #[doc = "Set Bit"]
    #[inline(always)]
    pub fn set_bit_(self) -> &'a mut W {
        self.variant(CT32BIT0_CLK_AW::SET_BIT)
    }
}
#[doc = "CT32BIT bit timer 1 clock clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CT32BIT1_CLK_AW {
    #[doc = "0: No Effect"]
    NO_EFFECT = 0,
    #[doc = "1: Set Bit"]
    SET_BIT = 1,
}
impl From<CT32BIT1_CLK_AW> for bool {
    #[inline(always)]
    fn from(variant: CT32BIT1_CLK_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CT32BIT1_CLK` writer - CT32BIT bit timer 1 clock clear"]
pub type CT32BIT1_CLK_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, PSCCTL2_CLR_SPEC, CT32BIT1_CLK_AW, O>;
impl<'a, const O: u8> CT32BIT1_CLK_W<'a, O> {
    #[doc = "No Effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(CT32BIT1_CLK_AW::NO_EFFECT)
    }
    #[doc = "Set Bit"]
    #[inline(always)]
    pub fn set_bit_(self) -> &'a mut W {
        self.variant(CT32BIT1_CLK_AW::SET_BIT)
    }
}
#[doc = "CT32BIT bit timer 2 clock clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CT32BIT2_CLK_AW {
    #[doc = "0: No Effect"]
    NO_EFFECT = 0,
    #[doc = "1: Set Bit"]
    SET_BIT = 1,
}
impl From<CT32BIT2_CLK_AW> for bool {
    #[inline(always)]
    fn from(variant: CT32BIT2_CLK_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CT32BIT2_CLK` writer - CT32BIT bit timer 2 clock clear"]
pub type CT32BIT2_CLK_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, PSCCTL2_CLR_SPEC, CT32BIT2_CLK_AW, O>;
impl<'a, const O: u8> CT32BIT2_CLK_W<'a, O> {
    #[doc = "No Effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(CT32BIT2_CLK_AW::NO_EFFECT)
    }
    #[doc = "Set Bit"]
    #[inline(always)]
    pub fn set_bit_(self) -> &'a mut W {
        self.variant(CT32BIT2_CLK_AW::SET_BIT)
    }
}
#[doc = "CT32BIT bit timer 3 clock clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CT32BIT3_CLK_AW {
    #[doc = "0: No Effect"]
    NO_EFFECT = 0,
    #[doc = "1: Set Bit"]
    SET_BIT = 1,
}
impl From<CT32BIT3_CLK_AW> for bool {
    #[inline(always)]
    fn from(variant: CT32BIT3_CLK_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CT32BIT3_CLK` writer - CT32BIT bit timer 3 clock clear"]
pub type CT32BIT3_CLK_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, PSCCTL2_CLR_SPEC, CT32BIT3_CLK_AW, O>;
impl<'a, const O: u8> CT32BIT3_CLK_W<'a, O> {
    #[doc = "No Effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(CT32BIT3_CLK_AW::NO_EFFECT)
    }
    #[doc = "Set Bit"]
    #[inline(always)]
    pub fn set_bit_(self) -> &'a mut W {
        self.variant(CT32BIT3_CLK_AW::SET_BIT)
    }
}
#[doc = "CT32BIT bit timer 4 clock clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CT32BIT4_CLK_AW {
    #[doc = "0: No Effect"]
    NO_EFFECT = 0,
    #[doc = "1: Set Bit"]
    SET_BIT = 1,
}
impl From<CT32BIT4_CLK_AW> for bool {
    #[inline(always)]
    fn from(variant: CT32BIT4_CLK_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CT32BIT4_CLK` writer - CT32BIT bit timer 4 clock clear"]
pub type CT32BIT4_CLK_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, PSCCTL2_CLR_SPEC, CT32BIT4_CLK_AW, O>;
impl<'a, const O: u8> CT32BIT4_CLK_W<'a, O> {
    #[doc = "No Effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(CT32BIT4_CLK_AW::NO_EFFECT)
    }
    #[doc = "Set Bit"]
    #[inline(always)]
    pub fn set_bit_(self) -> &'a mut W {
        self.variant(CT32BIT4_CLK_AW::SET_BIT)
    }
}
#[doc = "Field `RTCLITE_CLK` reader - RTC clock control clear"]
pub type RTCLITE_CLK_R = crate::BitReader<RTCLITE_CLK_A>;
#[doc = "RTC clock control clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RTCLITE_CLK_A {
    #[doc = "0: Disable Clock"]
    DISABLE = 0,
    #[doc = "1: Enable Clock"]
    ENABLE = 1,
}
impl From<RTCLITE_CLK_A> for bool {
    #[inline(always)]
    fn from(variant: RTCLITE_CLK_A) -> Self {
        variant as u8 != 0
    }
}
impl RTCLITE_CLK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RTCLITE_CLK_A {
        match self.bits {
            false => RTCLITE_CLK_A::DISABLE,
            true => RTCLITE_CLK_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == RTCLITE_CLK_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == RTCLITE_CLK_A::ENABLE
    }
}
#[doc = "Field `RTCLITE_CLK` writer - RTC clock control clear"]
pub type RTCLITE_CLK_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, PSCCTL2_CLR_SPEC, RTCLITE_CLK_A, O>;
impl<'a, const O: u8> RTCLITE_CLK_W<'a, O> {
    #[doc = "Disable Clock"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(RTCLITE_CLK_A::DISABLE)
    }
    #[doc = "Enable Clock"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(RTCLITE_CLK_A::ENABLE)
    }
}
#[doc = "Field `MRT0_CLK` reader - Multi-Rate Timer 0 clock control clear"]
pub type MRT0_CLK_R = crate::BitReader<MRT0_CLK_A>;
#[doc = "Multi-Rate Timer 0 clock control clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MRT0_CLK_A {
    #[doc = "0: Disable Clock"]
    DISABLE = 0,
    #[doc = "1: Enable Clock"]
    ENABLE = 1,
}
impl From<MRT0_CLK_A> for bool {
    #[inline(always)]
    fn from(variant: MRT0_CLK_A) -> Self {
        variant as u8 != 0
    }
}
impl MRT0_CLK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MRT0_CLK_A {
        match self.bits {
            false => MRT0_CLK_A::DISABLE,
            true => MRT0_CLK_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == MRT0_CLK_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == MRT0_CLK_A::ENABLE
    }
}
#[doc = "Field `MRT0_CLK` writer - Multi-Rate Timer 0 clock control clear"]
pub type MRT0_CLK_W<'a, const O: u8> = crate::BitWriter<'a, u32, PSCCTL2_CLR_SPEC, MRT0_CLK_A, O>;
impl<'a, const O: u8> MRT0_CLK_W<'a, O> {
    #[doc = "Disable Clock"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(MRT0_CLK_A::DISABLE)
    }
    #[doc = "Enable Clock"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(MRT0_CLK_A::ENABLE)
    }
}
#[doc = "Field `WWDT1_CLK` reader - Watchdog Timer 1 clock control clear"]
pub type WWDT1_CLK_R = crate::BitReader<WWDT1_CLK_A>;
#[doc = "Watchdog Timer 1 clock control clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WWDT1_CLK_A {
    #[doc = "0: Disable Clock"]
    DISABLE = 0,
    #[doc = "1: Enable Clock"]
    ENABLE = 1,
}
impl From<WWDT1_CLK_A> for bool {
    #[inline(always)]
    fn from(variant: WWDT1_CLK_A) -> Self {
        variant as u8 != 0
    }
}
impl WWDT1_CLK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WWDT1_CLK_A {
        match self.bits {
            false => WWDT1_CLK_A::DISABLE,
            true => WWDT1_CLK_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == WWDT1_CLK_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == WWDT1_CLK_A::ENABLE
    }
}
#[doc = "Field `WWDT1_CLK` writer - Watchdog Timer 1 clock control clear"]
pub type WWDT1_CLK_W<'a, const O: u8> = crate::BitWriter<'a, u32, PSCCTL2_CLR_SPEC, WWDT1_CLK_A, O>;
impl<'a, const O: u8> WWDT1_CLK_W<'a, O> {
    #[doc = "Disable Clock"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(WWDT1_CLK_A::DISABLE)
    }
    #[doc = "Enable Clock"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(WWDT1_CLK_A::ENABLE)
    }
}
#[doc = "Field `I3C0_CLK` reader - I3C0 clock control clear"]
pub type I3C0_CLK_R = crate::BitReader<I3C0_CLK_A>;
#[doc = "I3C0 clock control clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum I3C0_CLK_A {
    #[doc = "0: Disable Clock"]
    DISABLE = 0,
    #[doc = "1: Enable Clock"]
    ENABLE = 1,
}
impl From<I3C0_CLK_A> for bool {
    #[inline(always)]
    fn from(variant: I3C0_CLK_A) -> Self {
        variant as u8 != 0
    }
}
impl I3C0_CLK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> I3C0_CLK_A {
        match self.bits {
            false => I3C0_CLK_A::DISABLE,
            true => I3C0_CLK_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == I3C0_CLK_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == I3C0_CLK_A::ENABLE
    }
}
#[doc = "Field `I3C0_CLK` writer - I3C0 clock control clear"]
pub type I3C0_CLK_W<'a, const O: u8> = crate::BitWriter<'a, u32, PSCCTL2_CLR_SPEC, I3C0_CLK_A, O>;
impl<'a, const O: u8> I3C0_CLK_W<'a, O> {
    #[doc = "Disable Clock"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(I3C0_CLK_A::DISABLE)
    }
    #[doc = "Enable Clock"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(I3C0_CLK_A::ENABLE)
    }
}
#[doc = "Field `I3C1_CLK` reader - I3C1 clock control clear"]
pub type I3C1_CLK_R = crate::BitReader<I3C1_CLK_A>;
#[doc = "I3C1 clock control clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum I3C1_CLK_A {
    #[doc = "0: Disable Clock"]
    DISABLE = 0,
    #[doc = "1: Enable Clock"]
    ENABLE = 1,
}
impl From<I3C1_CLK_A> for bool {
    #[inline(always)]
    fn from(variant: I3C1_CLK_A) -> Self {
        variant as u8 != 0
    }
}
impl I3C1_CLK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> I3C1_CLK_A {
        match self.bits {
            false => I3C1_CLK_A::DISABLE,
            true => I3C1_CLK_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == I3C1_CLK_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == I3C1_CLK_A::ENABLE
    }
}
#[doc = "Field `I3C1_CLK` writer - I3C1 clock control clear"]
pub type I3C1_CLK_W<'a, const O: u8> = crate::BitWriter<'a, u32, PSCCTL2_CLR_SPEC, I3C1_CLK_A, O>;
impl<'a, const O: u8> I3C1_CLK_W<'a, O> {
    #[doc = "Disable Clock"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(I3C1_CLK_A::DISABLE)
    }
    #[doc = "Enable Clock"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(I3C1_CLK_A::ENABLE)
    }
}
#[doc = "Field `GPIOINTCTL_CLK` reader - PINT clock control clear"]
pub type GPIOINTCTL_CLK_R = crate::BitReader<GPIOINTCTL_CLK_A>;
#[doc = "PINT clock control clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GPIOINTCTL_CLK_A {
    #[doc = "0: Disable Clock"]
    DISABLE = 0,
    #[doc = "1: Enable Clock"]
    ENABLE = 1,
}
impl From<GPIOINTCTL_CLK_A> for bool {
    #[inline(always)]
    fn from(variant: GPIOINTCTL_CLK_A) -> Self {
        variant as u8 != 0
    }
}
impl GPIOINTCTL_CLK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIOINTCTL_CLK_A {
        match self.bits {
            false => GPIOINTCTL_CLK_A::DISABLE,
            true => GPIOINTCTL_CLK_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == GPIOINTCTL_CLK_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == GPIOINTCTL_CLK_A::ENABLE
    }
}
#[doc = "Field `GPIOINTCTL_CLK` writer - PINT clock control clear"]
pub type GPIOINTCTL_CLK_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, PSCCTL2_CLR_SPEC, GPIOINTCTL_CLK_A, O>;
impl<'a, const O: u8> GPIOINTCTL_CLK_W<'a, O> {
    #[doc = "Disable Clock"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(GPIOINTCTL_CLK_A::DISABLE)
    }
    #[doc = "Enable Clock"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(GPIOINTCTL_CLK_A::ENABLE)
    }
}
#[doc = "Field `PIMCTL_CLK` reader - INPUTMUX clock control clear"]
pub type PIMCTL_CLK_R = crate::BitReader<PIMCTL_CLK_A>;
#[doc = "INPUTMUX clock control clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PIMCTL_CLK_A {
    #[doc = "0: Disable Clock"]
    DISABLE = 0,
    #[doc = "1: Enable Clock"]
    ENABLE = 1,
}
impl From<PIMCTL_CLK_A> for bool {
    #[inline(always)]
    fn from(variant: PIMCTL_CLK_A) -> Self {
        variant as u8 != 0
    }
}
impl PIMCTL_CLK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PIMCTL_CLK_A {
        match self.bits {
            false => PIMCTL_CLK_A::DISABLE,
            true => PIMCTL_CLK_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == PIMCTL_CLK_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == PIMCTL_CLK_A::ENABLE
    }
}
#[doc = "Field `PIMCTL_CLK` writer - INPUTMUX clock control clear"]
pub type PIMCTL_CLK_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, PSCCTL2_CLR_SPEC, PIMCTL_CLK_A, O>;
impl<'a, const O: u8> PIMCTL_CLK_W<'a, O> {
    #[doc = "Disable Clock"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(PIMCTL_CLK_A::DISABLE)
    }
    #[doc = "Enable Clock"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(PIMCTL_CLK_A::ENABLE)
    }
}
impl R {
    #[doc = "Bit 7 - RTC clock control clear"]
    #[inline(always)]
    pub fn rtclite_clk(&self) -> RTCLITE_CLK_R {
        RTCLITE_CLK_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Multi-Rate Timer 0 clock control clear"]
    #[inline(always)]
    pub fn mrt0_clk(&self) -> MRT0_CLK_R {
        MRT0_CLK_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 10 - Watchdog Timer 1 clock control clear"]
    #[inline(always)]
    pub fn wwdt1_clk(&self) -> WWDT1_CLK_R {
        WWDT1_CLK_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 16 - I3C0 clock control clear"]
    #[inline(always)]
    pub fn i3c0_clk(&self) -> I3C0_CLK_R {
        I3C0_CLK_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - I3C1 clock control clear"]
    #[inline(always)]
    pub fn i3c1_clk(&self) -> I3C1_CLK_R {
        I3C1_CLK_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 30 - PINT clock control clear"]
    #[inline(always)]
    pub fn gpiointctl_clk(&self) -> GPIOINTCTL_CLK_R {
        GPIOINTCTL_CLK_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - INPUTMUX clock control clear"]
    #[inline(always)]
    pub fn pimctl_clk(&self) -> PIMCTL_CLK_R {
        PIMCTL_CLK_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - CT32BIT bit timer 0 clock clear"]
    #[inline(always)]
    #[must_use]
    pub fn ct32bit0_clk(&mut self) -> CT32BIT0_CLK_W<0> {
        CT32BIT0_CLK_W::new(self)
    }
    #[doc = "Bit 1 - CT32BIT bit timer 1 clock clear"]
    #[inline(always)]
    #[must_use]
    pub fn ct32bit1_clk(&mut self) -> CT32BIT1_CLK_W<1> {
        CT32BIT1_CLK_W::new(self)
    }
    #[doc = "Bit 2 - CT32BIT bit timer 2 clock clear"]
    #[inline(always)]
    #[must_use]
    pub fn ct32bit2_clk(&mut self) -> CT32BIT2_CLK_W<2> {
        CT32BIT2_CLK_W::new(self)
    }
    #[doc = "Bit 3 - CT32BIT bit timer 3 clock clear"]
    #[inline(always)]
    #[must_use]
    pub fn ct32bit3_clk(&mut self) -> CT32BIT3_CLK_W<3> {
        CT32BIT3_CLK_W::new(self)
    }
    #[doc = "Bit 4 - CT32BIT bit timer 4 clock clear"]
    #[inline(always)]
    #[must_use]
    pub fn ct32bit4_clk(&mut self) -> CT32BIT4_CLK_W<4> {
        CT32BIT4_CLK_W::new(self)
    }
    #[doc = "Bit 7 - RTC clock control clear"]
    #[inline(always)]
    #[must_use]
    pub fn rtclite_clk(&mut self) -> RTCLITE_CLK_W<7> {
        RTCLITE_CLK_W::new(self)
    }
    #[doc = "Bit 8 - Multi-Rate Timer 0 clock control clear"]
    #[inline(always)]
    #[must_use]
    pub fn mrt0_clk(&mut self) -> MRT0_CLK_W<8> {
        MRT0_CLK_W::new(self)
    }
    #[doc = "Bit 10 - Watchdog Timer 1 clock control clear"]
    #[inline(always)]
    #[must_use]
    pub fn wwdt1_clk(&mut self) -> WWDT1_CLK_W<10> {
        WWDT1_CLK_W::new(self)
    }
    #[doc = "Bit 16 - I3C0 clock control clear"]
    #[inline(always)]
    #[must_use]
    pub fn i3c0_clk(&mut self) -> I3C0_CLK_W<16> {
        I3C0_CLK_W::new(self)
    }
    #[doc = "Bit 17 - I3C1 clock control clear"]
    #[inline(always)]
    #[must_use]
    pub fn i3c1_clk(&mut self) -> I3C1_CLK_W<17> {
        I3C1_CLK_W::new(self)
    }
    #[doc = "Bit 30 - PINT clock control clear"]
    #[inline(always)]
    #[must_use]
    pub fn gpiointctl_clk(&mut self) -> GPIOINTCTL_CLK_W<30> {
        GPIOINTCTL_CLK_W::new(self)
    }
    #[doc = "Bit 31 - INPUTMUX clock control clear"]
    #[inline(always)]
    #[must_use]
    pub fn pimctl_clk(&mut self) -> PIMCTL_CLK_W<31> {
        PIMCTL_CLK_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Clock Clear 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pscctl2_clr](index.html) module"]
pub struct PSCCTL2_CLR_SPEC;
impl crate::RegisterSpec for PSCCTL2_CLR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pscctl2_clr::R](R) reader structure"]
impl crate::Readable for PSCCTL2_CLR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pscctl2_clr::W](W) writer structure"]
impl crate::Writable for PSCCTL2_CLR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PSCCTL2_CLR to value 0"]
impl crate::Resettable for PSCCTL2_CLR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
