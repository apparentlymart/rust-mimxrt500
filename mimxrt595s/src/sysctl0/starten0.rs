#[doc = "Register `STARTEN0` reader"]
pub struct R(crate::R<STARTEN0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<STARTEN0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<STARTEN0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<STARTEN0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `STARTEN0` writer"]
pub struct W(crate::W<STARTEN0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<STARTEN0_SPEC>;
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
impl From<crate::W<STARTEN0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<STARTEN0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `WDT0` reader - Watchdog timer 0 wake-up."]
pub type WDT0_R = crate::BitReader<WDT0_A>;
#[doc = "Watchdog timer 0 wake-up.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WDT0_A {
    #[doc = "0: Disabled"]
    DISABLE = 0,
    #[doc = "1: Enabled"]
    ENABLE = 1,
}
impl From<WDT0_A> for bool {
    #[inline(always)]
    fn from(variant: WDT0_A) -> Self {
        variant as u8 != 0
    }
}
impl WDT0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WDT0_A {
        match self.bits {
            false => WDT0_A::DISABLE,
            true => WDT0_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == WDT0_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == WDT0_A::ENABLE
    }
}
#[doc = "Field `WDT0` writer - Watchdog timer 0 wake-up."]
pub type WDT0_W<'a, const O: u8> = crate::BitWriter<'a, u32, STARTEN0_SPEC, WDT0_A, O>;
impl<'a, const O: u8> WDT0_W<'a, O> {
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(WDT0_A::DISABLE)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(WDT0_A::ENABLE)
    }
}
#[doc = "Field `DMAC0` reader - DMA controller 0 wake-up."]
pub type DMAC0_R = crate::BitReader<DMAC0_A>;
#[doc = "DMA controller 0 wake-up.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DMAC0_A {
    #[doc = "0: Disabled"]
    DISABLE = 0,
    #[doc = "1: Enabled"]
    ENABLE = 1,
}
impl From<DMAC0_A> for bool {
    #[inline(always)]
    fn from(variant: DMAC0_A) -> Self {
        variant as u8 != 0
    }
}
impl DMAC0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DMAC0_A {
        match self.bits {
            false => DMAC0_A::DISABLE,
            true => DMAC0_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == DMAC0_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == DMAC0_A::ENABLE
    }
}
#[doc = "Field `DMAC0` writer - DMA controller 0 wake-up."]
pub type DMAC0_W<'a, const O: u8> = crate::BitWriter<'a, u32, STARTEN0_SPEC, DMAC0_A, O>;
impl<'a, const O: u8> DMAC0_W<'a, O> {
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(DMAC0_A::DISABLE)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(DMAC0_A::ENABLE)
    }
}
#[doc = "Field `GPIO_INTA` reader - Non-secure GPIO interrupt A wake-up."]
pub type GPIO_INTA_R = crate::BitReader<GPIO_INTA_A>;
#[doc = "Non-secure GPIO interrupt A wake-up.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GPIO_INTA_A {
    #[doc = "0: Disabled"]
    DISABLE = 0,
    #[doc = "1: Enabled"]
    ENABLE = 1,
}
impl From<GPIO_INTA_A> for bool {
    #[inline(always)]
    fn from(variant: GPIO_INTA_A) -> Self {
        variant as u8 != 0
    }
}
impl GPIO_INTA_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO_INTA_A {
        match self.bits {
            false => GPIO_INTA_A::DISABLE,
            true => GPIO_INTA_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == GPIO_INTA_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == GPIO_INTA_A::ENABLE
    }
}
#[doc = "Field `GPIO_INTA` writer - Non-secure GPIO interrupt A wake-up."]
pub type GPIO_INTA_W<'a, const O: u8> = crate::BitWriter<'a, u32, STARTEN0_SPEC, GPIO_INTA_A, O>;
impl<'a, const O: u8> GPIO_INTA_W<'a, O> {
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(GPIO_INTA_A::DISABLE)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(GPIO_INTA_A::ENABLE)
    }
}
#[doc = "Field `GPIO_INTB` reader - Non-secure GPIO interrupt B wake-up."]
pub type GPIO_INTB_R = crate::BitReader<GPIO_INTB_A>;
#[doc = "Non-secure GPIO interrupt B wake-up.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GPIO_INTB_A {
    #[doc = "0: Disabled"]
    DISABLE = 0,
    #[doc = "1: Enabled"]
    ENABLE = 1,
}
impl From<GPIO_INTB_A> for bool {
    #[inline(always)]
    fn from(variant: GPIO_INTB_A) -> Self {
        variant as u8 != 0
    }
}
impl GPIO_INTB_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO_INTB_A {
        match self.bits {
            false => GPIO_INTB_A::DISABLE,
            true => GPIO_INTB_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == GPIO_INTB_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == GPIO_INTB_A::ENABLE
    }
}
#[doc = "Field `GPIO_INTB` writer - Non-secure GPIO interrupt B wake-up."]
pub type GPIO_INTB_W<'a, const O: u8> = crate::BitWriter<'a, u32, STARTEN0_SPEC, GPIO_INTB_A, O>;
impl<'a, const O: u8> GPIO_INTB_W<'a, O> {
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(GPIO_INTB_A::DISABLE)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(GPIO_INTB_A::ENABLE)
    }
}
#[doc = "Field `GPIO_INT0_IRQ0` reader - GPIO pin interrupt 0 wake-up."]
pub type GPIO_INT0_IRQ0_R = crate::BitReader<GPIO_INT0_IRQ0_A>;
#[doc = "GPIO pin interrupt 0 wake-up.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GPIO_INT0_IRQ0_A {
    #[doc = "0: Disabled"]
    DISABLE = 0,
    #[doc = "1: Enabled"]
    ENABLE = 1,
}
impl From<GPIO_INT0_IRQ0_A> for bool {
    #[inline(always)]
    fn from(variant: GPIO_INT0_IRQ0_A) -> Self {
        variant as u8 != 0
    }
}
impl GPIO_INT0_IRQ0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO_INT0_IRQ0_A {
        match self.bits {
            false => GPIO_INT0_IRQ0_A::DISABLE,
            true => GPIO_INT0_IRQ0_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == GPIO_INT0_IRQ0_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == GPIO_INT0_IRQ0_A::ENABLE
    }
}
#[doc = "Field `GPIO_INT0_IRQ0` writer - GPIO pin interrupt 0 wake-up."]
pub type GPIO_INT0_IRQ0_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, STARTEN0_SPEC, GPIO_INT0_IRQ0_A, O>;
impl<'a, const O: u8> GPIO_INT0_IRQ0_W<'a, O> {
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(GPIO_INT0_IRQ0_A::DISABLE)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(GPIO_INT0_IRQ0_A::ENABLE)
    }
}
#[doc = "Field `GPIO_INT0_IRQ1` reader - GPIO pin interrupt 1 wake-up."]
pub type GPIO_INT0_IRQ1_R = crate::BitReader<GPIO_INT0_IRQ1_A>;
#[doc = "GPIO pin interrupt 1 wake-up.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GPIO_INT0_IRQ1_A {
    #[doc = "0: Disabled"]
    DISABLE = 0,
    #[doc = "1: Enabled"]
    ENABLE = 1,
}
impl From<GPIO_INT0_IRQ1_A> for bool {
    #[inline(always)]
    fn from(variant: GPIO_INT0_IRQ1_A) -> Self {
        variant as u8 != 0
    }
}
impl GPIO_INT0_IRQ1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO_INT0_IRQ1_A {
        match self.bits {
            false => GPIO_INT0_IRQ1_A::DISABLE,
            true => GPIO_INT0_IRQ1_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == GPIO_INT0_IRQ1_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == GPIO_INT0_IRQ1_A::ENABLE
    }
}
#[doc = "Field `GPIO_INT0_IRQ1` writer - GPIO pin interrupt 1 wake-up."]
pub type GPIO_INT0_IRQ1_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, STARTEN0_SPEC, GPIO_INT0_IRQ1_A, O>;
impl<'a, const O: u8> GPIO_INT0_IRQ1_W<'a, O> {
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(GPIO_INT0_IRQ1_A::DISABLE)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(GPIO_INT0_IRQ1_A::ENABLE)
    }
}
#[doc = "Field `GPIO_INT0_IRQ2` reader - GPIO pin interrupt 2 wake-up."]
pub type GPIO_INT0_IRQ2_R = crate::BitReader<GPIO_INT0_IRQ2_A>;
#[doc = "GPIO pin interrupt 2 wake-up.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GPIO_INT0_IRQ2_A {
    #[doc = "0: Disabled"]
    DISABLE = 0,
    #[doc = "1: Enabled"]
    ENABLE = 1,
}
impl From<GPIO_INT0_IRQ2_A> for bool {
    #[inline(always)]
    fn from(variant: GPIO_INT0_IRQ2_A) -> Self {
        variant as u8 != 0
    }
}
impl GPIO_INT0_IRQ2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO_INT0_IRQ2_A {
        match self.bits {
            false => GPIO_INT0_IRQ2_A::DISABLE,
            true => GPIO_INT0_IRQ2_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == GPIO_INT0_IRQ2_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == GPIO_INT0_IRQ2_A::ENABLE
    }
}
#[doc = "Field `GPIO_INT0_IRQ2` writer - GPIO pin interrupt 2 wake-up."]
pub type GPIO_INT0_IRQ2_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, STARTEN0_SPEC, GPIO_INT0_IRQ2_A, O>;
impl<'a, const O: u8> GPIO_INT0_IRQ2_W<'a, O> {
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(GPIO_INT0_IRQ2_A::DISABLE)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(GPIO_INT0_IRQ2_A::ENABLE)
    }
}
#[doc = "Field `GPIO_INT0_IRQ3` reader - GPIO pin interrupt 3 wake-up."]
pub type GPIO_INT0_IRQ3_R = crate::BitReader<GPIO_INT0_IRQ3_A>;
#[doc = "GPIO pin interrupt 3 wake-up.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GPIO_INT0_IRQ3_A {
    #[doc = "0: Disabled"]
    DISABLE = 0,
    #[doc = "1: Enabled"]
    ENABLE = 1,
}
impl From<GPIO_INT0_IRQ3_A> for bool {
    #[inline(always)]
    fn from(variant: GPIO_INT0_IRQ3_A) -> Self {
        variant as u8 != 0
    }
}
impl GPIO_INT0_IRQ3_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO_INT0_IRQ3_A {
        match self.bits {
            false => GPIO_INT0_IRQ3_A::DISABLE,
            true => GPIO_INT0_IRQ3_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == GPIO_INT0_IRQ3_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == GPIO_INT0_IRQ3_A::ENABLE
    }
}
#[doc = "Field `GPIO_INT0_IRQ3` writer - GPIO pin interrupt 3 wake-up."]
pub type GPIO_INT0_IRQ3_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, STARTEN0_SPEC, GPIO_INT0_IRQ3_A, O>;
impl<'a, const O: u8> GPIO_INT0_IRQ3_W<'a, O> {
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(GPIO_INT0_IRQ3_A::DISABLE)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(GPIO_INT0_IRQ3_A::ENABLE)
    }
}
#[doc = "Field `UTICK0` reader - UTICK wake-up."]
pub type UTICK0_R = crate::BitReader<UTICK0_A>;
#[doc = "UTICK wake-up.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UTICK0_A {
    #[doc = "0: Disabled"]
    DISABLE = 0,
    #[doc = "1: Enabled"]
    ENABLE = 1,
}
impl From<UTICK0_A> for bool {
    #[inline(always)]
    fn from(variant: UTICK0_A) -> Self {
        variant as u8 != 0
    }
}
impl UTICK0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> UTICK0_A {
        match self.bits {
            false => UTICK0_A::DISABLE,
            true => UTICK0_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == UTICK0_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == UTICK0_A::ENABLE
    }
}
#[doc = "Field `UTICK0` writer - UTICK wake-up."]
pub type UTICK0_W<'a, const O: u8> = crate::BitWriter<'a, u32, STARTEN0_SPEC, UTICK0_A, O>;
impl<'a, const O: u8> UTICK0_W<'a, O> {
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(UTICK0_A::DISABLE)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(UTICK0_A::ENABLE)
    }
}
#[doc = "Field `MRT0` reader - MRT wake-up."]
pub type MRT0_R = crate::BitReader<MRT0_A>;
#[doc = "MRT wake-up.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MRT0_A {
    #[doc = "0: Disabled"]
    DISABLE = 0,
    #[doc = "1: Enabled"]
    ENABLE = 1,
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
            false => MRT0_A::DISABLE,
            true => MRT0_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == MRT0_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == MRT0_A::ENABLE
    }
}
#[doc = "Field `MRT0` writer - MRT wake-up."]
pub type MRT0_W<'a, const O: u8> = crate::BitWriter<'a, u32, STARTEN0_SPEC, MRT0_A, O>;
impl<'a, const O: u8> MRT0_W<'a, O> {
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(MRT0_A::DISABLE)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(MRT0_A::ENABLE)
    }
}
#[doc = "Field `CT32BIT0` reader - CTIMER 0 wake-up"]
pub type CT32BIT0_R = crate::BitReader<CT32BIT0_A>;
#[doc = "CTIMER 0 wake-up\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CT32BIT0_A {
    #[doc = "0: Disabled"]
    DISABLE = 0,
    #[doc = "1: Enabled"]
    ENABLE = 1,
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
            false => CT32BIT0_A::DISABLE,
            true => CT32BIT0_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == CT32BIT0_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == CT32BIT0_A::ENABLE
    }
}
#[doc = "Field `CT32BIT0` writer - CTIMER 0 wake-up"]
pub type CT32BIT0_W<'a, const O: u8> = crate::BitWriter<'a, u32, STARTEN0_SPEC, CT32BIT0_A, O>;
impl<'a, const O: u8> CT32BIT0_W<'a, O> {
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(CT32BIT0_A::DISABLE)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(CT32BIT0_A::ENABLE)
    }
}
#[doc = "Field `CT32BIT1` reader - CTIMER 1 wake-up"]
pub type CT32BIT1_R = crate::BitReader<CT32BIT1_A>;
#[doc = "CTIMER 1 wake-up\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CT32BIT1_A {
    #[doc = "0: Disabled"]
    DISABLE = 0,
    #[doc = "1: Enabled"]
    ENABLE = 1,
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
            false => CT32BIT1_A::DISABLE,
            true => CT32BIT1_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == CT32BIT1_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == CT32BIT1_A::ENABLE
    }
}
#[doc = "Field `CT32BIT1` writer - CTIMER 1 wake-up"]
pub type CT32BIT1_W<'a, const O: u8> = crate::BitWriter<'a, u32, STARTEN0_SPEC, CT32BIT1_A, O>;
impl<'a, const O: u8> CT32BIT1_W<'a, O> {
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(CT32BIT1_A::DISABLE)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(CT32BIT1_A::ENABLE)
    }
}
#[doc = "Field `SCT0` reader - SCTimer/PWM wake-up."]
pub type SCT0_R = crate::BitReader<SCT0_A>;
#[doc = "SCTimer/PWM wake-up.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SCT0_A {
    #[doc = "0: Disabled"]
    DISABLE = 0,
    #[doc = "1: Enabled"]
    ENABLE = 1,
}
impl From<SCT0_A> for bool {
    #[inline(always)]
    fn from(variant: SCT0_A) -> Self {
        variant as u8 != 0
    }
}
impl SCT0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SCT0_A {
        match self.bits {
            false => SCT0_A::DISABLE,
            true => SCT0_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == SCT0_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == SCT0_A::ENABLE
    }
}
#[doc = "Field `SCT0` writer - SCTimer/PWM wake-up."]
pub type SCT0_W<'a, const O: u8> = crate::BitWriter<'a, u32, STARTEN0_SPEC, SCT0_A, O>;
impl<'a, const O: u8> SCT0_W<'a, O> {
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(SCT0_A::DISABLE)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(SCT0_A::ENABLE)
    }
}
#[doc = "Field `CT32BIT3` reader - CTIMER 3 wake-up"]
pub type CT32BIT3_R = crate::BitReader<CT32BIT3_A>;
#[doc = "CTIMER 3 wake-up\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CT32BIT3_A {
    #[doc = "0: Disabled"]
    DISABLE = 0,
    #[doc = "1: Enabled"]
    ENABLE = 1,
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
            false => CT32BIT3_A::DISABLE,
            true => CT32BIT3_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == CT32BIT3_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == CT32BIT3_A::ENABLE
    }
}
#[doc = "Field `CT32BIT3` writer - CTIMER 3 wake-up"]
pub type CT32BIT3_W<'a, const O: u8> = crate::BitWriter<'a, u32, STARTEN0_SPEC, CT32BIT3_A, O>;
impl<'a, const O: u8> CT32BIT3_W<'a, O> {
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(CT32BIT3_A::DISABLE)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(CT32BIT3_A::ENABLE)
    }
}
#[doc = "Field `FLEXCOMM0` reader - Flexcomm 0 peripheral interrupt wake-up."]
pub type FLEXCOMM0_R = crate::BitReader<FLEXCOMM0_A>;
#[doc = "Flexcomm 0 peripheral interrupt wake-up.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FLEXCOMM0_A {
    #[doc = "0: Disabled"]
    DISABLE = 0,
    #[doc = "1: Enabled"]
    ENABLE = 1,
}
impl From<FLEXCOMM0_A> for bool {
    #[inline(always)]
    fn from(variant: FLEXCOMM0_A) -> Self {
        variant as u8 != 0
    }
}
impl FLEXCOMM0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FLEXCOMM0_A {
        match self.bits {
            false => FLEXCOMM0_A::DISABLE,
            true => FLEXCOMM0_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == FLEXCOMM0_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == FLEXCOMM0_A::ENABLE
    }
}
#[doc = "Field `FLEXCOMM0` writer - Flexcomm 0 peripheral interrupt wake-up."]
pub type FLEXCOMM0_W<'a, const O: u8> = crate::BitWriter<'a, u32, STARTEN0_SPEC, FLEXCOMM0_A, O>;
impl<'a, const O: u8> FLEXCOMM0_W<'a, O> {
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(FLEXCOMM0_A::DISABLE)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(FLEXCOMM0_A::ENABLE)
    }
}
#[doc = "Field `FLEXCOMM1` reader - Flexcomm 1 peripheral interrupt wake-up."]
pub type FLEXCOMM1_R = crate::BitReader<FLEXCOMM1_A>;
#[doc = "Flexcomm 1 peripheral interrupt wake-up.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FLEXCOMM1_A {
    #[doc = "0: Disabled"]
    DISABLE = 0,
    #[doc = "1: Enabled"]
    ENABLE = 1,
}
impl From<FLEXCOMM1_A> for bool {
    #[inline(always)]
    fn from(variant: FLEXCOMM1_A) -> Self {
        variant as u8 != 0
    }
}
impl FLEXCOMM1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FLEXCOMM1_A {
        match self.bits {
            false => FLEXCOMM1_A::DISABLE,
            true => FLEXCOMM1_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == FLEXCOMM1_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == FLEXCOMM1_A::ENABLE
    }
}
#[doc = "Field `FLEXCOMM1` writer - Flexcomm 1 peripheral interrupt wake-up."]
pub type FLEXCOMM1_W<'a, const O: u8> = crate::BitWriter<'a, u32, STARTEN0_SPEC, FLEXCOMM1_A, O>;
impl<'a, const O: u8> FLEXCOMM1_W<'a, O> {
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(FLEXCOMM1_A::DISABLE)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(FLEXCOMM1_A::ENABLE)
    }
}
#[doc = "Field `FLEXCOMM2` reader - Flexcomm 2 peripheral interrupt wake-up."]
pub type FLEXCOMM2_R = crate::BitReader<FLEXCOMM2_A>;
#[doc = "Flexcomm 2 peripheral interrupt wake-up.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FLEXCOMM2_A {
    #[doc = "0: Disabled"]
    DISABLE = 0,
    #[doc = "1: Enabled"]
    ENABLE = 1,
}
impl From<FLEXCOMM2_A> for bool {
    #[inline(always)]
    fn from(variant: FLEXCOMM2_A) -> Self {
        variant as u8 != 0
    }
}
impl FLEXCOMM2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FLEXCOMM2_A {
        match self.bits {
            false => FLEXCOMM2_A::DISABLE,
            true => FLEXCOMM2_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == FLEXCOMM2_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == FLEXCOMM2_A::ENABLE
    }
}
#[doc = "Field `FLEXCOMM2` writer - Flexcomm 2 peripheral interrupt wake-up."]
pub type FLEXCOMM2_W<'a, const O: u8> = crate::BitWriter<'a, u32, STARTEN0_SPEC, FLEXCOMM2_A, O>;
impl<'a, const O: u8> FLEXCOMM2_W<'a, O> {
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(FLEXCOMM2_A::DISABLE)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(FLEXCOMM2_A::ENABLE)
    }
}
#[doc = "Field `FLEXCOMM3` reader - Flexcomm 3 peripheral interrupt wake-up."]
pub type FLEXCOMM3_R = crate::BitReader<FLEXCOMM3_A>;
#[doc = "Flexcomm 3 peripheral interrupt wake-up.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FLEXCOMM3_A {
    #[doc = "0: Disabled"]
    DISABLE = 0,
    #[doc = "1: Enabled"]
    ENABLE = 1,
}
impl From<FLEXCOMM3_A> for bool {
    #[inline(always)]
    fn from(variant: FLEXCOMM3_A) -> Self {
        variant as u8 != 0
    }
}
impl FLEXCOMM3_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FLEXCOMM3_A {
        match self.bits {
            false => FLEXCOMM3_A::DISABLE,
            true => FLEXCOMM3_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == FLEXCOMM3_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == FLEXCOMM3_A::ENABLE
    }
}
#[doc = "Field `FLEXCOMM3` writer - Flexcomm 3 peripheral interrupt wake-up."]
pub type FLEXCOMM3_W<'a, const O: u8> = crate::BitWriter<'a, u32, STARTEN0_SPEC, FLEXCOMM3_A, O>;
impl<'a, const O: u8> FLEXCOMM3_W<'a, O> {
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(FLEXCOMM3_A::DISABLE)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(FLEXCOMM3_A::ENABLE)
    }
}
#[doc = "Field `FLEXCOMM4` reader - Flexcomm 4 peripheral interrupt wake-up."]
pub type FLEXCOMM4_R = crate::BitReader<FLEXCOMM4_A>;
#[doc = "Flexcomm 4 peripheral interrupt wake-up.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FLEXCOMM4_A {
    #[doc = "0: Disabled"]
    DISABLE = 0,
    #[doc = "1: Enabled"]
    ENABLE = 1,
}
impl From<FLEXCOMM4_A> for bool {
    #[inline(always)]
    fn from(variant: FLEXCOMM4_A) -> Self {
        variant as u8 != 0
    }
}
impl FLEXCOMM4_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FLEXCOMM4_A {
        match self.bits {
            false => FLEXCOMM4_A::DISABLE,
            true => FLEXCOMM4_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == FLEXCOMM4_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == FLEXCOMM4_A::ENABLE
    }
}
#[doc = "Field `FLEXCOMM4` writer - Flexcomm 4 peripheral interrupt wake-up."]
pub type FLEXCOMM4_W<'a, const O: u8> = crate::BitWriter<'a, u32, STARTEN0_SPEC, FLEXCOMM4_A, O>;
impl<'a, const O: u8> FLEXCOMM4_W<'a, O> {
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(FLEXCOMM4_A::DISABLE)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(FLEXCOMM4_A::ENABLE)
    }
}
#[doc = "Field `FLEXCOMM5` reader - Flexcomm 5 peripheral interrupt wake-up."]
pub type FLEXCOMM5_R = crate::BitReader<FLEXCOMM5_A>;
#[doc = "Flexcomm 5 peripheral interrupt wake-up.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FLEXCOMM5_A {
    #[doc = "0: Disabled"]
    DISABLE = 0,
    #[doc = "1: Enabled"]
    ENABLE = 1,
}
impl From<FLEXCOMM5_A> for bool {
    #[inline(always)]
    fn from(variant: FLEXCOMM5_A) -> Self {
        variant as u8 != 0
    }
}
impl FLEXCOMM5_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FLEXCOMM5_A {
        match self.bits {
            false => FLEXCOMM5_A::DISABLE,
            true => FLEXCOMM5_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == FLEXCOMM5_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == FLEXCOMM5_A::ENABLE
    }
}
#[doc = "Field `FLEXCOMM5` writer - Flexcomm 5 peripheral interrupt wake-up."]
pub type FLEXCOMM5_W<'a, const O: u8> = crate::BitWriter<'a, u32, STARTEN0_SPEC, FLEXCOMM5_A, O>;
impl<'a, const O: u8> FLEXCOMM5_W<'a, O> {
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(FLEXCOMM5_A::DISABLE)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(FLEXCOMM5_A::ENABLE)
    }
}
#[doc = "Field `FLEXCOMM14` reader - Flexcomm 14 (High Speed SPI) peripheral interrupt wake-up."]
pub type FLEXCOMM14_R = crate::BitReader<FLEXCOMM14_A>;
#[doc = "Flexcomm 14 (High Speed SPI) peripheral interrupt wake-up.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FLEXCOMM14_A {
    #[doc = "0: Disabled"]
    DISABLE = 0,
    #[doc = "1: Enabled"]
    ENABLE = 1,
}
impl From<FLEXCOMM14_A> for bool {
    #[inline(always)]
    fn from(variant: FLEXCOMM14_A) -> Self {
        variant as u8 != 0
    }
}
impl FLEXCOMM14_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FLEXCOMM14_A {
        match self.bits {
            false => FLEXCOMM14_A::DISABLE,
            true => FLEXCOMM14_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == FLEXCOMM14_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == FLEXCOMM14_A::ENABLE
    }
}
#[doc = "Field `FLEXCOMM14` writer - Flexcomm 14 (High Speed SPI) peripheral interrupt wake-up."]
pub type FLEXCOMM14_W<'a, const O: u8> = crate::BitWriter<'a, u32, STARTEN0_SPEC, FLEXCOMM14_A, O>;
impl<'a, const O: u8> FLEXCOMM14_W<'a, O> {
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(FLEXCOMM14_A::DISABLE)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(FLEXCOMM14_A::ENABLE)
    }
}
#[doc = "Field `FLEXCOMM15` reader - Flexcomm 15 (PMIC I2C) peripheral interrupt wake-up."]
pub type FLEXCOMM15_R = crate::BitReader<FLEXCOMM15_A>;
#[doc = "Flexcomm 15 (PMIC I2C) peripheral interrupt wake-up.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FLEXCOMM15_A {
    #[doc = "0: Disabled"]
    DISABLE = 0,
    #[doc = "1: Enabled"]
    ENABLE = 1,
}
impl From<FLEXCOMM15_A> for bool {
    #[inline(always)]
    fn from(variant: FLEXCOMM15_A) -> Self {
        variant as u8 != 0
    }
}
impl FLEXCOMM15_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FLEXCOMM15_A {
        match self.bits {
            false => FLEXCOMM15_A::DISABLE,
            true => FLEXCOMM15_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == FLEXCOMM15_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == FLEXCOMM15_A::ENABLE
    }
}
#[doc = "Field `FLEXCOMM15` writer - Flexcomm 15 (PMIC I2C) peripheral interrupt wake-up."]
pub type FLEXCOMM15_W<'a, const O: u8> = crate::BitWriter<'a, u32, STARTEN0_SPEC, FLEXCOMM15_A, O>;
impl<'a, const O: u8> FLEXCOMM15_W<'a, O> {
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(FLEXCOMM15_A::DISABLE)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(FLEXCOMM15_A::ENABLE)
    }
}
#[doc = "Field `ADC0` reader - ADC wake-up."]
pub type ADC0_R = crate::BitReader<ADC0_A>;
#[doc = "ADC wake-up.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADC0_A {
    #[doc = "0: Disabled"]
    DISABLE = 0,
    #[doc = "1: Enabled"]
    ENABLE = 1,
}
impl From<ADC0_A> for bool {
    #[inline(always)]
    fn from(variant: ADC0_A) -> Self {
        variant as u8 != 0
    }
}
impl ADC0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADC0_A {
        match self.bits {
            false => ADC0_A::DISABLE,
            true => ADC0_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == ADC0_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == ADC0_A::ENABLE
    }
}
#[doc = "Field `ADC0` writer - ADC wake-up."]
pub type ADC0_W<'a, const O: u8> = crate::BitWriter<'a, u32, STARTEN0_SPEC, ADC0_A, O>;
impl<'a, const O: u8> ADC0_W<'a, O> {
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(ADC0_A::DISABLE)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(ADC0_A::ENABLE)
    }
}
#[doc = "Field `ACMP` reader - ACMP wake-up."]
pub type ACMP_R = crate::BitReader<ACMP_A>;
#[doc = "ACMP wake-up.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ACMP_A {
    #[doc = "0: Disabled"]
    DISABLE = 0,
    #[doc = "1: Enabled"]
    ENABLE = 1,
}
impl From<ACMP_A> for bool {
    #[inline(always)]
    fn from(variant: ACMP_A) -> Self {
        variant as u8 != 0
    }
}
impl ACMP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ACMP_A {
        match self.bits {
            false => ACMP_A::DISABLE,
            true => ACMP_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == ACMP_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == ACMP_A::ENABLE
    }
}
#[doc = "Field `ACMP` writer - ACMP wake-up."]
pub type ACMP_W<'a, const O: u8> = crate::BitWriter<'a, u32, STARTEN0_SPEC, ACMP_A, O>;
impl<'a, const O: u8> ACMP_W<'a, O> {
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(ACMP_A::DISABLE)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(ACMP_A::ENABLE)
    }
}
#[doc = "Field `DMIC0` reader - DMIC wake-up."]
pub type DMIC0_R = crate::BitReader<DMIC0_A>;
#[doc = "DMIC wake-up.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DMIC0_A {
    #[doc = "0: Disabled"]
    DISABLE = 0,
    #[doc = "1: Enabled"]
    ENABLE = 1,
}
impl From<DMIC0_A> for bool {
    #[inline(always)]
    fn from(variant: DMIC0_A) -> Self {
        variant as u8 != 0
    }
}
impl DMIC0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DMIC0_A {
        match self.bits {
            false => DMIC0_A::DISABLE,
            true => DMIC0_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == DMIC0_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == DMIC0_A::ENABLE
    }
}
#[doc = "Field `DMIC0` writer - DMIC wake-up."]
pub type DMIC0_W<'a, const O: u8> = crate::BitWriter<'a, u32, STARTEN0_SPEC, DMIC0_A, O>;
impl<'a, const O: u8> DMIC0_W<'a, O> {
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(DMIC0_A::DISABLE)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(DMIC0_A::ENABLE)
    }
}
#[doc = "Field `HYPERVISOR` reader - Hypervisor interrupt wake-up."]
pub type HYPERVISOR_R = crate::BitReader<HYPERVISOR_A>;
#[doc = "Hypervisor interrupt wake-up.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HYPERVISOR_A {
    #[doc = "0: Disabled"]
    DISABLE = 0,
    #[doc = "1: Enabled"]
    ENABLE = 1,
}
impl From<HYPERVISOR_A> for bool {
    #[inline(always)]
    fn from(variant: HYPERVISOR_A) -> Self {
        variant as u8 != 0
    }
}
impl HYPERVISOR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HYPERVISOR_A {
        match self.bits {
            false => HYPERVISOR_A::DISABLE,
            true => HYPERVISOR_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == HYPERVISOR_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == HYPERVISOR_A::ENABLE
    }
}
#[doc = "Field `HYPERVISOR` writer - Hypervisor interrupt wake-up."]
pub type HYPERVISOR_W<'a, const O: u8> = crate::BitWriter<'a, u32, STARTEN0_SPEC, HYPERVISOR_A, O>;
impl<'a, const O: u8> HYPERVISOR_W<'a, O> {
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(HYPERVISOR_A::DISABLE)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(HYPERVISOR_A::ENABLE)
    }
}
#[doc = "Field `SECUREVIOLATION` reader - Secure Violation wake-up."]
pub type SECUREVIOLATION_R = crate::BitReader<SECUREVIOLATION_A>;
#[doc = "Secure Violation wake-up.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SECUREVIOLATION_A {
    #[doc = "0: Disabled"]
    DISABLE = 0,
    #[doc = "1: Enabled"]
    ENABLE = 1,
}
impl From<SECUREVIOLATION_A> for bool {
    #[inline(always)]
    fn from(variant: SECUREVIOLATION_A) -> Self {
        variant as u8 != 0
    }
}
impl SECUREVIOLATION_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SECUREVIOLATION_A {
        match self.bits {
            false => SECUREVIOLATION_A::DISABLE,
            true => SECUREVIOLATION_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == SECUREVIOLATION_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == SECUREVIOLATION_A::ENABLE
    }
}
#[doc = "Field `SECUREVIOLATION` writer - Secure Violation wake-up."]
pub type SECUREVIOLATION_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, STARTEN0_SPEC, SECUREVIOLATION_A, O>;
impl<'a, const O: u8> SECUREVIOLATION_W<'a, O> {
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(SECUREVIOLATION_A::DISABLE)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(SECUREVIOLATION_A::ENABLE)
    }
}
#[doc = "Field `HWVAD0` reader - Hardware VAD wake-up."]
pub type HWVAD0_R = crate::BitReader<HWVAD0_A>;
#[doc = "Hardware VAD wake-up.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HWVAD0_A {
    #[doc = "0: Disabled"]
    DISABLE = 0,
    #[doc = "1: Enabled"]
    ENABLE = 1,
}
impl From<HWVAD0_A> for bool {
    #[inline(always)]
    fn from(variant: HWVAD0_A) -> Self {
        variant as u8 != 0
    }
}
impl HWVAD0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HWVAD0_A {
        match self.bits {
            false => HWVAD0_A::DISABLE,
            true => HWVAD0_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == HWVAD0_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == HWVAD0_A::ENABLE
    }
}
#[doc = "Field `HWVAD0` writer - Hardware VAD wake-up."]
pub type HWVAD0_W<'a, const O: u8> = crate::BitWriter<'a, u32, STARTEN0_SPEC, HWVAD0_A, O>;
impl<'a, const O: u8> HWVAD0_W<'a, O> {
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(HWVAD0_A::DISABLE)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(HWVAD0_A::ENABLE)
    }
}
#[doc = "Field `PMC` reader - PMC wake-up."]
pub type PMC_R = crate::BitReader<PMC_A>;
#[doc = "PMC wake-up.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PMC_A {
    #[doc = "0: Disabled"]
    DISABLE = 0,
    #[doc = "1: Enabled"]
    ENABLE = 1,
}
impl From<PMC_A> for bool {
    #[inline(always)]
    fn from(variant: PMC_A) -> Self {
        variant as u8 != 0
    }
}
impl PMC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PMC_A {
        match self.bits {
            false => PMC_A::DISABLE,
            true => PMC_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == PMC_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == PMC_A::ENABLE
    }
}
#[doc = "Field `PMC` writer - PMC wake-up."]
pub type PMC_W<'a, const O: u8> = crate::BitWriter<'a, u32, STARTEN0_SPEC, PMC_A, O>;
impl<'a, const O: u8> PMC_W<'a, O> {
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(PMC_A::DISABLE)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(PMC_A::ENABLE)
    }
}
#[doc = "Field `RNG` reader - RNG wake-up."]
pub type RNG_R = crate::BitReader<RNG_A>;
#[doc = "RNG wake-up.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RNG_A {
    #[doc = "0: Disabled"]
    DISABLE = 0,
    #[doc = "1: Enabled"]
    ENABLE = 1,
}
impl From<RNG_A> for bool {
    #[inline(always)]
    fn from(variant: RNG_A) -> Self {
        variant as u8 != 0
    }
}
impl RNG_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RNG_A {
        match self.bits {
            false => RNG_A::DISABLE,
            true => RNG_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == RNG_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == RNG_A::ENABLE
    }
}
#[doc = "Field `RNG` writer - RNG wake-up."]
pub type RNG_W<'a, const O: u8> = crate::BitWriter<'a, u32, STARTEN0_SPEC, RNG_A, O>;
impl<'a, const O: u8> RNG_W<'a, O> {
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(RNG_A::DISABLE)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(RNG_A::ENABLE)
    }
}
impl R {
    #[doc = "Bit 0 - Watchdog timer 0 wake-up."]
    #[inline(always)]
    pub fn wdt0(&self) -> WDT0_R {
        WDT0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - DMA controller 0 wake-up."]
    #[inline(always)]
    pub fn dmac0(&self) -> DMAC0_R {
        DMAC0_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Non-secure GPIO interrupt A wake-up."]
    #[inline(always)]
    pub fn gpio_inta(&self) -> GPIO_INTA_R {
        GPIO_INTA_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Non-secure GPIO interrupt B wake-up."]
    #[inline(always)]
    pub fn gpio_intb(&self) -> GPIO_INTB_R {
        GPIO_INTB_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - GPIO pin interrupt 0 wake-up."]
    #[inline(always)]
    pub fn gpio_int0_irq0(&self) -> GPIO_INT0_IRQ0_R {
        GPIO_INT0_IRQ0_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - GPIO pin interrupt 1 wake-up."]
    #[inline(always)]
    pub fn gpio_int0_irq1(&self) -> GPIO_INT0_IRQ1_R {
        GPIO_INT0_IRQ1_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - GPIO pin interrupt 2 wake-up."]
    #[inline(always)]
    pub fn gpio_int0_irq2(&self) -> GPIO_INT0_IRQ2_R {
        GPIO_INT0_IRQ2_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - GPIO pin interrupt 3 wake-up."]
    #[inline(always)]
    pub fn gpio_int0_irq3(&self) -> GPIO_INT0_IRQ3_R {
        GPIO_INT0_IRQ3_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - UTICK wake-up."]
    #[inline(always)]
    pub fn utick0(&self) -> UTICK0_R {
        UTICK0_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - MRT wake-up."]
    #[inline(always)]
    pub fn mrt0(&self) -> MRT0_R {
        MRT0_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - CTIMER 0 wake-up"]
    #[inline(always)]
    pub fn ct32bit0(&self) -> CT32BIT0_R {
        CT32BIT0_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - CTIMER 1 wake-up"]
    #[inline(always)]
    pub fn ct32bit1(&self) -> CT32BIT1_R {
        CT32BIT1_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - SCTimer/PWM wake-up."]
    #[inline(always)]
    pub fn sct0(&self) -> SCT0_R {
        SCT0_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - CTIMER 3 wake-up"]
    #[inline(always)]
    pub fn ct32bit3(&self) -> CT32BIT3_R {
        CT32BIT3_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Flexcomm 0 peripheral interrupt wake-up."]
    #[inline(always)]
    pub fn flexcomm0(&self) -> FLEXCOMM0_R {
        FLEXCOMM0_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Flexcomm 1 peripheral interrupt wake-up."]
    #[inline(always)]
    pub fn flexcomm1(&self) -> FLEXCOMM1_R {
        FLEXCOMM1_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Flexcomm 2 peripheral interrupt wake-up."]
    #[inline(always)]
    pub fn flexcomm2(&self) -> FLEXCOMM2_R {
        FLEXCOMM2_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Flexcomm 3 peripheral interrupt wake-up."]
    #[inline(always)]
    pub fn flexcomm3(&self) -> FLEXCOMM3_R {
        FLEXCOMM3_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Flexcomm 4 peripheral interrupt wake-up."]
    #[inline(always)]
    pub fn flexcomm4(&self) -> FLEXCOMM4_R {
        FLEXCOMM4_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Flexcomm 5 peripheral interrupt wake-up."]
    #[inline(always)]
    pub fn flexcomm5(&self) -> FLEXCOMM5_R {
        FLEXCOMM5_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Flexcomm 14 (High Speed SPI) peripheral interrupt wake-up."]
    #[inline(always)]
    pub fn flexcomm14(&self) -> FLEXCOMM14_R {
        FLEXCOMM14_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Flexcomm 15 (PMIC I2C) peripheral interrupt wake-up."]
    #[inline(always)]
    pub fn flexcomm15(&self) -> FLEXCOMM15_R {
        FLEXCOMM15_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - ADC wake-up."]
    #[inline(always)]
    pub fn adc0(&self) -> ADC0_R {
        ADC0_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 24 - ACMP wake-up."]
    #[inline(always)]
    pub fn acmp(&self) -> ACMP_R {
        ACMP_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - DMIC wake-up."]
    #[inline(always)]
    pub fn dmic0(&self) -> DMIC0_R {
        DMIC0_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 27 - Hypervisor interrupt wake-up."]
    #[inline(always)]
    pub fn hypervisor(&self) -> HYPERVISOR_R {
        HYPERVISOR_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Secure Violation wake-up."]
    #[inline(always)]
    pub fn secureviolation(&self) -> SECUREVIOLATION_R {
        SECUREVIOLATION_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Hardware VAD wake-up."]
    #[inline(always)]
    pub fn hwvad0(&self) -> HWVAD0_R {
        HWVAD0_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - PMC wake-up."]
    #[inline(always)]
    pub fn pmc(&self) -> PMC_R {
        PMC_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - RNG wake-up."]
    #[inline(always)]
    pub fn rng(&self) -> RNG_R {
        RNG_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Watchdog timer 0 wake-up."]
    #[inline(always)]
    #[must_use]
    pub fn wdt0(&mut self) -> WDT0_W<0> {
        WDT0_W::new(self)
    }
    #[doc = "Bit 1 - DMA controller 0 wake-up."]
    #[inline(always)]
    #[must_use]
    pub fn dmac0(&mut self) -> DMAC0_W<1> {
        DMAC0_W::new(self)
    }
    #[doc = "Bit 2 - Non-secure GPIO interrupt A wake-up."]
    #[inline(always)]
    #[must_use]
    pub fn gpio_inta(&mut self) -> GPIO_INTA_W<2> {
        GPIO_INTA_W::new(self)
    }
    #[doc = "Bit 3 - Non-secure GPIO interrupt B wake-up."]
    #[inline(always)]
    #[must_use]
    pub fn gpio_intb(&mut self) -> GPIO_INTB_W<3> {
        GPIO_INTB_W::new(self)
    }
    #[doc = "Bit 4 - GPIO pin interrupt 0 wake-up."]
    #[inline(always)]
    #[must_use]
    pub fn gpio_int0_irq0(&mut self) -> GPIO_INT0_IRQ0_W<4> {
        GPIO_INT0_IRQ0_W::new(self)
    }
    #[doc = "Bit 5 - GPIO pin interrupt 1 wake-up."]
    #[inline(always)]
    #[must_use]
    pub fn gpio_int0_irq1(&mut self) -> GPIO_INT0_IRQ1_W<5> {
        GPIO_INT0_IRQ1_W::new(self)
    }
    #[doc = "Bit 6 - GPIO pin interrupt 2 wake-up."]
    #[inline(always)]
    #[must_use]
    pub fn gpio_int0_irq2(&mut self) -> GPIO_INT0_IRQ2_W<6> {
        GPIO_INT0_IRQ2_W::new(self)
    }
    #[doc = "Bit 7 - GPIO pin interrupt 3 wake-up."]
    #[inline(always)]
    #[must_use]
    pub fn gpio_int0_irq3(&mut self) -> GPIO_INT0_IRQ3_W<7> {
        GPIO_INT0_IRQ3_W::new(self)
    }
    #[doc = "Bit 8 - UTICK wake-up."]
    #[inline(always)]
    #[must_use]
    pub fn utick0(&mut self) -> UTICK0_W<8> {
        UTICK0_W::new(self)
    }
    #[doc = "Bit 9 - MRT wake-up."]
    #[inline(always)]
    #[must_use]
    pub fn mrt0(&mut self) -> MRT0_W<9> {
        MRT0_W::new(self)
    }
    #[doc = "Bit 10 - CTIMER 0 wake-up"]
    #[inline(always)]
    #[must_use]
    pub fn ct32bit0(&mut self) -> CT32BIT0_W<10> {
        CT32BIT0_W::new(self)
    }
    #[doc = "Bit 11 - CTIMER 1 wake-up"]
    #[inline(always)]
    #[must_use]
    pub fn ct32bit1(&mut self) -> CT32BIT1_W<11> {
        CT32BIT1_W::new(self)
    }
    #[doc = "Bit 12 - SCTimer/PWM wake-up."]
    #[inline(always)]
    #[must_use]
    pub fn sct0(&mut self) -> SCT0_W<12> {
        SCT0_W::new(self)
    }
    #[doc = "Bit 13 - CTIMER 3 wake-up"]
    #[inline(always)]
    #[must_use]
    pub fn ct32bit3(&mut self) -> CT32BIT3_W<13> {
        CT32BIT3_W::new(self)
    }
    #[doc = "Bit 14 - Flexcomm 0 peripheral interrupt wake-up."]
    #[inline(always)]
    #[must_use]
    pub fn flexcomm0(&mut self) -> FLEXCOMM0_W<14> {
        FLEXCOMM0_W::new(self)
    }
    #[doc = "Bit 15 - Flexcomm 1 peripheral interrupt wake-up."]
    #[inline(always)]
    #[must_use]
    pub fn flexcomm1(&mut self) -> FLEXCOMM1_W<15> {
        FLEXCOMM1_W::new(self)
    }
    #[doc = "Bit 16 - Flexcomm 2 peripheral interrupt wake-up."]
    #[inline(always)]
    #[must_use]
    pub fn flexcomm2(&mut self) -> FLEXCOMM2_W<16> {
        FLEXCOMM2_W::new(self)
    }
    #[doc = "Bit 17 - Flexcomm 3 peripheral interrupt wake-up."]
    #[inline(always)]
    #[must_use]
    pub fn flexcomm3(&mut self) -> FLEXCOMM3_W<17> {
        FLEXCOMM3_W::new(self)
    }
    #[doc = "Bit 18 - Flexcomm 4 peripheral interrupt wake-up."]
    #[inline(always)]
    #[must_use]
    pub fn flexcomm4(&mut self) -> FLEXCOMM4_W<18> {
        FLEXCOMM4_W::new(self)
    }
    #[doc = "Bit 19 - Flexcomm 5 peripheral interrupt wake-up."]
    #[inline(always)]
    #[must_use]
    pub fn flexcomm5(&mut self) -> FLEXCOMM5_W<19> {
        FLEXCOMM5_W::new(self)
    }
    #[doc = "Bit 20 - Flexcomm 14 (High Speed SPI) peripheral interrupt wake-up."]
    #[inline(always)]
    #[must_use]
    pub fn flexcomm14(&mut self) -> FLEXCOMM14_W<20> {
        FLEXCOMM14_W::new(self)
    }
    #[doc = "Bit 21 - Flexcomm 15 (PMIC I2C) peripheral interrupt wake-up."]
    #[inline(always)]
    #[must_use]
    pub fn flexcomm15(&mut self) -> FLEXCOMM15_W<21> {
        FLEXCOMM15_W::new(self)
    }
    #[doc = "Bit 22 - ADC wake-up."]
    #[inline(always)]
    #[must_use]
    pub fn adc0(&mut self) -> ADC0_W<22> {
        ADC0_W::new(self)
    }
    #[doc = "Bit 24 - ACMP wake-up."]
    #[inline(always)]
    #[must_use]
    pub fn acmp(&mut self) -> ACMP_W<24> {
        ACMP_W::new(self)
    }
    #[doc = "Bit 25 - DMIC wake-up."]
    #[inline(always)]
    #[must_use]
    pub fn dmic0(&mut self) -> DMIC0_W<25> {
        DMIC0_W::new(self)
    }
    #[doc = "Bit 27 - Hypervisor interrupt wake-up."]
    #[inline(always)]
    #[must_use]
    pub fn hypervisor(&mut self) -> HYPERVISOR_W<27> {
        HYPERVISOR_W::new(self)
    }
    #[doc = "Bit 28 - Secure Violation wake-up."]
    #[inline(always)]
    #[must_use]
    pub fn secureviolation(&mut self) -> SECUREVIOLATION_W<28> {
        SECUREVIOLATION_W::new(self)
    }
    #[doc = "Bit 29 - Hardware VAD wake-up."]
    #[inline(always)]
    #[must_use]
    pub fn hwvad0(&mut self) -> HWVAD0_W<29> {
        HWVAD0_W::new(self)
    }
    #[doc = "Bit 30 - PMC wake-up."]
    #[inline(always)]
    #[must_use]
    pub fn pmc(&mut self) -> PMC_W<30> {
        PMC_W::new(self)
    }
    #[doc = "Bit 31 - RNG wake-up."]
    #[inline(always)]
    #[must_use]
    pub fn rng(&mut self) -> RNG_W<31> {
        RNG_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Start Enable 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [starten0](index.html) module"]
pub struct STARTEN0_SPEC;
impl crate::RegisterSpec for STARTEN0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [starten0::R](R) reader structure"]
impl crate::Readable for STARTEN0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [starten0::W](W) writer structure"]
impl crate::Writable for STARTEN0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets STARTEN0 to value 0"]
impl crate::Resettable for STARTEN0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
