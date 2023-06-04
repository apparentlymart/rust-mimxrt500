#[doc = "Register `STARTEN0_SET` reader"]
pub struct R(crate::R<STARTEN0_SET_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<STARTEN0_SET_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<STARTEN0_SET_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<STARTEN0_SET_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `STARTEN0_SET` writer"]
pub struct W(crate::W<STARTEN0_SET_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<STARTEN0_SET_SPEC>;
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
impl From<crate::W<STARTEN0_SET_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<STARTEN0_SET_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `WDT0` reader - Watchdog timer 0 wake-up."]
pub type WDT0_R = crate::BitReader<WDT0_A>;
#[doc = "Watchdog timer 0 wake-up.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WDT0_A {
    #[doc = "0: No effect"]
    WDT0_0 = 0,
    #[doc = "1: Sets the STARTEN0 Bit"]
    WDT0_1 = 1,
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
            false => WDT0_A::WDT0_0,
            true => WDT0_A::WDT0_1,
        }
    }
    #[doc = "Checks if the value of the field is `WDT0_0`"]
    #[inline(always)]
    pub fn is_wdt0_0(&self) -> bool {
        *self == WDT0_A::WDT0_0
    }
    #[doc = "Checks if the value of the field is `WDT0_1`"]
    #[inline(always)]
    pub fn is_wdt0_1(&self) -> bool {
        *self == WDT0_A::WDT0_1
    }
}
#[doc = "Field `WDT0` writer - Watchdog timer 0 wake-up."]
pub type WDT0_W<'a, const O: u8> = crate::BitWriter1S<'a, u32, STARTEN0_SET_SPEC, WDT0_A, O>;
impl<'a, const O: u8> WDT0_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn wdt0_0(self) -> &'a mut W {
        self.variant(WDT0_A::WDT0_0)
    }
    #[doc = "Sets the STARTEN0 Bit"]
    #[inline(always)]
    pub fn wdt0_1(self) -> &'a mut W {
        self.variant(WDT0_A::WDT0_1)
    }
}
#[doc = "Field `DMAC0` reader - DMA controller 0 wake-up."]
pub type DMAC0_R = crate::BitReader<DMAC0_A>;
#[doc = "DMA controller 0 wake-up.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DMAC0_A {
    #[doc = "0: No effect"]
    DMAC0_0 = 0,
    #[doc = "1: Sets the STARTEN0 Bit"]
    DMAC0_1 = 1,
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
            false => DMAC0_A::DMAC0_0,
            true => DMAC0_A::DMAC0_1,
        }
    }
    #[doc = "Checks if the value of the field is `DMAC0_0`"]
    #[inline(always)]
    pub fn is_dmac0_0(&self) -> bool {
        *self == DMAC0_A::DMAC0_0
    }
    #[doc = "Checks if the value of the field is `DMAC0_1`"]
    #[inline(always)]
    pub fn is_dmac0_1(&self) -> bool {
        *self == DMAC0_A::DMAC0_1
    }
}
#[doc = "Field `DMAC0` writer - DMA controller 0 wake-up."]
pub type DMAC0_W<'a, const O: u8> = crate::BitWriter1S<'a, u32, STARTEN0_SET_SPEC, DMAC0_A, O>;
impl<'a, const O: u8> DMAC0_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn dmac0_0(self) -> &'a mut W {
        self.variant(DMAC0_A::DMAC0_0)
    }
    #[doc = "Sets the STARTEN0 Bit"]
    #[inline(always)]
    pub fn dmac0_1(self) -> &'a mut W {
        self.variant(DMAC0_A::DMAC0_1)
    }
}
#[doc = "Field `GPIO_INTA` reader - Non-secure GPIO interrupt A wake-up."]
pub type GPIO_INTA_R = crate::BitReader<GPIO_INTA_A>;
#[doc = "Non-secure GPIO interrupt A wake-up.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GPIO_INTA_A {
    #[doc = "0: No effect"]
    GPIO_INTA_0 = 0,
    #[doc = "1: Sets the STARTEN0 Bit"]
    GPIO_INTA_1 = 1,
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
            false => GPIO_INTA_A::GPIO_INTA_0,
            true => GPIO_INTA_A::GPIO_INTA_1,
        }
    }
    #[doc = "Checks if the value of the field is `GPIO_INTA_0`"]
    #[inline(always)]
    pub fn is_gpio_inta_0(&self) -> bool {
        *self == GPIO_INTA_A::GPIO_INTA_0
    }
    #[doc = "Checks if the value of the field is `GPIO_INTA_1`"]
    #[inline(always)]
    pub fn is_gpio_inta_1(&self) -> bool {
        *self == GPIO_INTA_A::GPIO_INTA_1
    }
}
#[doc = "Field `GPIO_INTA` writer - Non-secure GPIO interrupt A wake-up."]
pub type GPIO_INTA_W<'a, const O: u8> =
    crate::BitWriter1S<'a, u32, STARTEN0_SET_SPEC, GPIO_INTA_A, O>;
impl<'a, const O: u8> GPIO_INTA_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn gpio_inta_0(self) -> &'a mut W {
        self.variant(GPIO_INTA_A::GPIO_INTA_0)
    }
    #[doc = "Sets the STARTEN0 Bit"]
    #[inline(always)]
    pub fn gpio_inta_1(self) -> &'a mut W {
        self.variant(GPIO_INTA_A::GPIO_INTA_1)
    }
}
#[doc = "Field `GPIO_INTB` reader - Non-secure GPIO interrupt B wake-up."]
pub type GPIO_INTB_R = crate::BitReader<GPIO_INTB_A>;
#[doc = "Non-secure GPIO interrupt B wake-up.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GPIO_INTB_A {
    #[doc = "0: No effect"]
    GPIO_INTB_0 = 0,
    #[doc = "1: Sets the STARTEN0 Bit"]
    GPIO_INTB_1 = 1,
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
            false => GPIO_INTB_A::GPIO_INTB_0,
            true => GPIO_INTB_A::GPIO_INTB_1,
        }
    }
    #[doc = "Checks if the value of the field is `GPIO_INTB_0`"]
    #[inline(always)]
    pub fn is_gpio_intb_0(&self) -> bool {
        *self == GPIO_INTB_A::GPIO_INTB_0
    }
    #[doc = "Checks if the value of the field is `GPIO_INTB_1`"]
    #[inline(always)]
    pub fn is_gpio_intb_1(&self) -> bool {
        *self == GPIO_INTB_A::GPIO_INTB_1
    }
}
#[doc = "Field `GPIO_INTB` writer - Non-secure GPIO interrupt B wake-up."]
pub type GPIO_INTB_W<'a, const O: u8> =
    crate::BitWriter1S<'a, u32, STARTEN0_SET_SPEC, GPIO_INTB_A, O>;
impl<'a, const O: u8> GPIO_INTB_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn gpio_intb_0(self) -> &'a mut W {
        self.variant(GPIO_INTB_A::GPIO_INTB_0)
    }
    #[doc = "Sets the STARTEN0 Bit"]
    #[inline(always)]
    pub fn gpio_intb_1(self) -> &'a mut W {
        self.variant(GPIO_INTB_A::GPIO_INTB_1)
    }
}
#[doc = "Field `GPIO_INT0_IRQ0` reader - GPIO pin interrupt 0 wake-up."]
pub type GPIO_INT0_IRQ0_R = crate::BitReader<GPIO_INT0_IRQ0_A>;
#[doc = "GPIO pin interrupt 0 wake-up.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GPIO_INT0_IRQ0_A {
    #[doc = "0: No effect"]
    GPIO_INT0_IRQ0_0 = 0,
    #[doc = "1: Sets the STARTEN0 Bit"]
    GPIO_INT0_IRQ0_1 = 1,
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
            false => GPIO_INT0_IRQ0_A::GPIO_INT0_IRQ0_0,
            true => GPIO_INT0_IRQ0_A::GPIO_INT0_IRQ0_1,
        }
    }
    #[doc = "Checks if the value of the field is `GPIO_INT0_IRQ0_0`"]
    #[inline(always)]
    pub fn is_gpio_int0_irq0_0(&self) -> bool {
        *self == GPIO_INT0_IRQ0_A::GPIO_INT0_IRQ0_0
    }
    #[doc = "Checks if the value of the field is `GPIO_INT0_IRQ0_1`"]
    #[inline(always)]
    pub fn is_gpio_int0_irq0_1(&self) -> bool {
        *self == GPIO_INT0_IRQ0_A::GPIO_INT0_IRQ0_1
    }
}
#[doc = "Field `GPIO_INT0_IRQ0` writer - GPIO pin interrupt 0 wake-up."]
pub type GPIO_INT0_IRQ0_W<'a, const O: u8> =
    crate::BitWriter1S<'a, u32, STARTEN0_SET_SPEC, GPIO_INT0_IRQ0_A, O>;
impl<'a, const O: u8> GPIO_INT0_IRQ0_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn gpio_int0_irq0_0(self) -> &'a mut W {
        self.variant(GPIO_INT0_IRQ0_A::GPIO_INT0_IRQ0_0)
    }
    #[doc = "Sets the STARTEN0 Bit"]
    #[inline(always)]
    pub fn gpio_int0_irq0_1(self) -> &'a mut W {
        self.variant(GPIO_INT0_IRQ0_A::GPIO_INT0_IRQ0_1)
    }
}
#[doc = "Field `GPIO_INT0_IRQ1` reader - GPIO pin interrupt 1 wake-up."]
pub type GPIO_INT0_IRQ1_R = crate::BitReader<GPIO_INT0_IRQ1_A>;
#[doc = "GPIO pin interrupt 1 wake-up.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GPIO_INT0_IRQ1_A {
    #[doc = "0: No effect"]
    GPIO_INT0_IRQ1_0 = 0,
    #[doc = "1: Sets the STARTEN0 Bit"]
    GPIO_INT0_IRQ1_1 = 1,
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
            false => GPIO_INT0_IRQ1_A::GPIO_INT0_IRQ1_0,
            true => GPIO_INT0_IRQ1_A::GPIO_INT0_IRQ1_1,
        }
    }
    #[doc = "Checks if the value of the field is `GPIO_INT0_IRQ1_0`"]
    #[inline(always)]
    pub fn is_gpio_int0_irq1_0(&self) -> bool {
        *self == GPIO_INT0_IRQ1_A::GPIO_INT0_IRQ1_0
    }
    #[doc = "Checks if the value of the field is `GPIO_INT0_IRQ1_1`"]
    #[inline(always)]
    pub fn is_gpio_int0_irq1_1(&self) -> bool {
        *self == GPIO_INT0_IRQ1_A::GPIO_INT0_IRQ1_1
    }
}
#[doc = "Field `GPIO_INT0_IRQ1` writer - GPIO pin interrupt 1 wake-up."]
pub type GPIO_INT0_IRQ1_W<'a, const O: u8> =
    crate::BitWriter1S<'a, u32, STARTEN0_SET_SPEC, GPIO_INT0_IRQ1_A, O>;
impl<'a, const O: u8> GPIO_INT0_IRQ1_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn gpio_int0_irq1_0(self) -> &'a mut W {
        self.variant(GPIO_INT0_IRQ1_A::GPIO_INT0_IRQ1_0)
    }
    #[doc = "Sets the STARTEN0 Bit"]
    #[inline(always)]
    pub fn gpio_int0_irq1_1(self) -> &'a mut W {
        self.variant(GPIO_INT0_IRQ1_A::GPIO_INT0_IRQ1_1)
    }
}
#[doc = "Field `GPIO_INT0_IRQ2` reader - GPIO pin interrupt 2 wake-up."]
pub type GPIO_INT0_IRQ2_R = crate::BitReader<GPIO_INT0_IRQ2_A>;
#[doc = "GPIO pin interrupt 2 wake-up.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GPIO_INT0_IRQ2_A {
    #[doc = "0: No effect"]
    GPIO_INT0_IRQ2_0 = 0,
    #[doc = "1: Sets the STARTEN0 Bit"]
    GPIO_INT0_IRQ2_1 = 1,
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
            false => GPIO_INT0_IRQ2_A::GPIO_INT0_IRQ2_0,
            true => GPIO_INT0_IRQ2_A::GPIO_INT0_IRQ2_1,
        }
    }
    #[doc = "Checks if the value of the field is `GPIO_INT0_IRQ2_0`"]
    #[inline(always)]
    pub fn is_gpio_int0_irq2_0(&self) -> bool {
        *self == GPIO_INT0_IRQ2_A::GPIO_INT0_IRQ2_0
    }
    #[doc = "Checks if the value of the field is `GPIO_INT0_IRQ2_1`"]
    #[inline(always)]
    pub fn is_gpio_int0_irq2_1(&self) -> bool {
        *self == GPIO_INT0_IRQ2_A::GPIO_INT0_IRQ2_1
    }
}
#[doc = "Field `GPIO_INT0_IRQ2` writer - GPIO pin interrupt 2 wake-up."]
pub type GPIO_INT0_IRQ2_W<'a, const O: u8> =
    crate::BitWriter1S<'a, u32, STARTEN0_SET_SPEC, GPIO_INT0_IRQ2_A, O>;
impl<'a, const O: u8> GPIO_INT0_IRQ2_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn gpio_int0_irq2_0(self) -> &'a mut W {
        self.variant(GPIO_INT0_IRQ2_A::GPIO_INT0_IRQ2_0)
    }
    #[doc = "Sets the STARTEN0 Bit"]
    #[inline(always)]
    pub fn gpio_int0_irq2_1(self) -> &'a mut W {
        self.variant(GPIO_INT0_IRQ2_A::GPIO_INT0_IRQ2_1)
    }
}
#[doc = "Field `GPIO_INT0_IRQ3` reader - GPIO pin interrupt 3 wake-up."]
pub type GPIO_INT0_IRQ3_R = crate::BitReader<GPIO_INT0_IRQ3_A>;
#[doc = "GPIO pin interrupt 3 wake-up.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GPIO_INT0_IRQ3_A {
    #[doc = "0: No effect"]
    GPIO_INT0_IRQ3_0 = 0,
    #[doc = "1: Sets the STARTEN0 Bit"]
    GPIO_INT0_IRQ3_1 = 1,
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
            false => GPIO_INT0_IRQ3_A::GPIO_INT0_IRQ3_0,
            true => GPIO_INT0_IRQ3_A::GPIO_INT0_IRQ3_1,
        }
    }
    #[doc = "Checks if the value of the field is `GPIO_INT0_IRQ3_0`"]
    #[inline(always)]
    pub fn is_gpio_int0_irq3_0(&self) -> bool {
        *self == GPIO_INT0_IRQ3_A::GPIO_INT0_IRQ3_0
    }
    #[doc = "Checks if the value of the field is `GPIO_INT0_IRQ3_1`"]
    #[inline(always)]
    pub fn is_gpio_int0_irq3_1(&self) -> bool {
        *self == GPIO_INT0_IRQ3_A::GPIO_INT0_IRQ3_1
    }
}
#[doc = "Field `GPIO_INT0_IRQ3` writer - GPIO pin interrupt 3 wake-up."]
pub type GPIO_INT0_IRQ3_W<'a, const O: u8> =
    crate::BitWriter1S<'a, u32, STARTEN0_SET_SPEC, GPIO_INT0_IRQ3_A, O>;
impl<'a, const O: u8> GPIO_INT0_IRQ3_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn gpio_int0_irq3_0(self) -> &'a mut W {
        self.variant(GPIO_INT0_IRQ3_A::GPIO_INT0_IRQ3_0)
    }
    #[doc = "Sets the STARTEN0 Bit"]
    #[inline(always)]
    pub fn gpio_int0_irq3_1(self) -> &'a mut W {
        self.variant(GPIO_INT0_IRQ3_A::GPIO_INT0_IRQ3_1)
    }
}
#[doc = "Field `UTICK0` reader - UTICK wake-up."]
pub type UTICK0_R = crate::BitReader<UTICK0_A>;
#[doc = "UTICK wake-up.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UTICK0_A {
    #[doc = "0: No effect"]
    UTICK0_0 = 0,
    #[doc = "1: Sets the STARTEN0 Bit"]
    UTICK0_1 = 1,
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
            false => UTICK0_A::UTICK0_0,
            true => UTICK0_A::UTICK0_1,
        }
    }
    #[doc = "Checks if the value of the field is `UTICK0_0`"]
    #[inline(always)]
    pub fn is_utick0_0(&self) -> bool {
        *self == UTICK0_A::UTICK0_0
    }
    #[doc = "Checks if the value of the field is `UTICK0_1`"]
    #[inline(always)]
    pub fn is_utick0_1(&self) -> bool {
        *self == UTICK0_A::UTICK0_1
    }
}
#[doc = "Field `UTICK0` writer - UTICK wake-up."]
pub type UTICK0_W<'a, const O: u8> = crate::BitWriter1S<'a, u32, STARTEN0_SET_SPEC, UTICK0_A, O>;
impl<'a, const O: u8> UTICK0_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn utick0_0(self) -> &'a mut W {
        self.variant(UTICK0_A::UTICK0_0)
    }
    #[doc = "Sets the STARTEN0 Bit"]
    #[inline(always)]
    pub fn utick0_1(self) -> &'a mut W {
        self.variant(UTICK0_A::UTICK0_1)
    }
}
#[doc = "Field `MRT0` reader - MRT wake-up."]
pub type MRT0_R = crate::BitReader<MRT0_A>;
#[doc = "MRT wake-up.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MRT0_A {
    #[doc = "0: No effect"]
    MRT0_0 = 0,
    #[doc = "1: Sets the STARTEN0 Bit"]
    MRT0_1 = 1,
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
            false => MRT0_A::MRT0_0,
            true => MRT0_A::MRT0_1,
        }
    }
    #[doc = "Checks if the value of the field is `MRT0_0`"]
    #[inline(always)]
    pub fn is_mrt0_0(&self) -> bool {
        *self == MRT0_A::MRT0_0
    }
    #[doc = "Checks if the value of the field is `MRT0_1`"]
    #[inline(always)]
    pub fn is_mrt0_1(&self) -> bool {
        *self == MRT0_A::MRT0_1
    }
}
#[doc = "Field `MRT0` writer - MRT wake-up."]
pub type MRT0_W<'a, const O: u8> = crate::BitWriter1S<'a, u32, STARTEN0_SET_SPEC, MRT0_A, O>;
impl<'a, const O: u8> MRT0_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn mrt0_0(self) -> &'a mut W {
        self.variant(MRT0_A::MRT0_0)
    }
    #[doc = "Sets the STARTEN0 Bit"]
    #[inline(always)]
    pub fn mrt0_1(self) -> &'a mut W {
        self.variant(MRT0_A::MRT0_1)
    }
}
#[doc = "Field `CT32BIT0` reader - CTIMER 0 wake-up."]
pub type CT32BIT0_R = crate::BitReader<CT32BIT0_A>;
#[doc = "CTIMER 0 wake-up.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CT32BIT0_A {
    #[doc = "0: No effect"]
    CT32BIT0_0 = 0,
    #[doc = "1: Sets the STARTEN0 Bit"]
    CT32BIT0_1 = 1,
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
            false => CT32BIT0_A::CT32BIT0_0,
            true => CT32BIT0_A::CT32BIT0_1,
        }
    }
    #[doc = "Checks if the value of the field is `CT32BIT0_0`"]
    #[inline(always)]
    pub fn is_ct32bit0_0(&self) -> bool {
        *self == CT32BIT0_A::CT32BIT0_0
    }
    #[doc = "Checks if the value of the field is `CT32BIT0_1`"]
    #[inline(always)]
    pub fn is_ct32bit0_1(&self) -> bool {
        *self == CT32BIT0_A::CT32BIT0_1
    }
}
#[doc = "Field `CT32BIT0` writer - CTIMER 0 wake-up."]
pub type CT32BIT0_W<'a, const O: u8> =
    crate::BitWriter1S<'a, u32, STARTEN0_SET_SPEC, CT32BIT0_A, O>;
impl<'a, const O: u8> CT32BIT0_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn ct32bit0_0(self) -> &'a mut W {
        self.variant(CT32BIT0_A::CT32BIT0_0)
    }
    #[doc = "Sets the STARTEN0 Bit"]
    #[inline(always)]
    pub fn ct32bit0_1(self) -> &'a mut W {
        self.variant(CT32BIT0_A::CT32BIT0_1)
    }
}
#[doc = "Field `CT32BIT1` reader - CTIMER 1 wake-up."]
pub type CT32BIT1_R = crate::BitReader<CT32BIT1_A>;
#[doc = "CTIMER 1 wake-up.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CT32BIT1_A {
    #[doc = "0: No effect"]
    CT32BIT1_0 = 0,
    #[doc = "1: Sets the STARTEN0 Bit"]
    CT32BIT1_1 = 1,
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
            false => CT32BIT1_A::CT32BIT1_0,
            true => CT32BIT1_A::CT32BIT1_1,
        }
    }
    #[doc = "Checks if the value of the field is `CT32BIT1_0`"]
    #[inline(always)]
    pub fn is_ct32bit1_0(&self) -> bool {
        *self == CT32BIT1_A::CT32BIT1_0
    }
    #[doc = "Checks if the value of the field is `CT32BIT1_1`"]
    #[inline(always)]
    pub fn is_ct32bit1_1(&self) -> bool {
        *self == CT32BIT1_A::CT32BIT1_1
    }
}
#[doc = "Field `CT32BIT1` writer - CTIMER 1 wake-up."]
pub type CT32BIT1_W<'a, const O: u8> =
    crate::BitWriter1S<'a, u32, STARTEN0_SET_SPEC, CT32BIT1_A, O>;
impl<'a, const O: u8> CT32BIT1_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn ct32bit1_0(self) -> &'a mut W {
        self.variant(CT32BIT1_A::CT32BIT1_0)
    }
    #[doc = "Sets the STARTEN0 Bit"]
    #[inline(always)]
    pub fn ct32bit1_1(self) -> &'a mut W {
        self.variant(CT32BIT1_A::CT32BIT1_1)
    }
}
#[doc = "Field `SCT0` reader - SCTimer/PWM wake-up."]
pub type SCT0_R = crate::BitReader<SCT0_A>;
#[doc = "SCTimer/PWM wake-up.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SCT0_A {
    #[doc = "0: No effect"]
    SCT0_0 = 0,
    #[doc = "1: Sets the STARTEN0 Bit"]
    SCT0_1 = 1,
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
            false => SCT0_A::SCT0_0,
            true => SCT0_A::SCT0_1,
        }
    }
    #[doc = "Checks if the value of the field is `SCT0_0`"]
    #[inline(always)]
    pub fn is_sct0_0(&self) -> bool {
        *self == SCT0_A::SCT0_0
    }
    #[doc = "Checks if the value of the field is `SCT0_1`"]
    #[inline(always)]
    pub fn is_sct0_1(&self) -> bool {
        *self == SCT0_A::SCT0_1
    }
}
#[doc = "Field `SCT0` writer - SCTimer/PWM wake-up."]
pub type SCT0_W<'a, const O: u8> = crate::BitWriter1S<'a, u32, STARTEN0_SET_SPEC, SCT0_A, O>;
impl<'a, const O: u8> SCT0_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn sct0_0(self) -> &'a mut W {
        self.variant(SCT0_A::SCT0_0)
    }
    #[doc = "Sets the STARTEN0 Bit"]
    #[inline(always)]
    pub fn sct0_1(self) -> &'a mut W {
        self.variant(SCT0_A::SCT0_1)
    }
}
#[doc = "Field `CT32BIT3` reader - CTIMER 3 wake-up"]
pub type CT32BIT3_R = crate::BitReader<CT32BIT3_A>;
#[doc = "CTIMER 3 wake-up\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CT32BIT3_A {
    #[doc = "0: No effect"]
    CT32BIT3_0 = 0,
    #[doc = "1: Sets the STARTEN0 Bit"]
    CT32BIT3_1 = 1,
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
            false => CT32BIT3_A::CT32BIT3_0,
            true => CT32BIT3_A::CT32BIT3_1,
        }
    }
    #[doc = "Checks if the value of the field is `CT32BIT3_0`"]
    #[inline(always)]
    pub fn is_ct32bit3_0(&self) -> bool {
        *self == CT32BIT3_A::CT32BIT3_0
    }
    #[doc = "Checks if the value of the field is `CT32BIT3_1`"]
    #[inline(always)]
    pub fn is_ct32bit3_1(&self) -> bool {
        *self == CT32BIT3_A::CT32BIT3_1
    }
}
#[doc = "Field `CT32BIT3` writer - CTIMER 3 wake-up"]
pub type CT32BIT3_W<'a, const O: u8> =
    crate::BitWriter1S<'a, u32, STARTEN0_SET_SPEC, CT32BIT3_A, O>;
impl<'a, const O: u8> CT32BIT3_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn ct32bit3_0(self) -> &'a mut W {
        self.variant(CT32BIT3_A::CT32BIT3_0)
    }
    #[doc = "Sets the STARTEN0 Bit"]
    #[inline(always)]
    pub fn ct32bit3_1(self) -> &'a mut W {
        self.variant(CT32BIT3_A::CT32BIT3_1)
    }
}
#[doc = "Field `FLEXCOMM14` reader - FlexComm 14 (High Speed SPI) peripheral interrupt wake-up."]
pub type FLEXCOMM14_R = crate::BitReader<FLEXCOMM14_A>;
#[doc = "FlexComm 14 (High Speed SPI) peripheral interrupt wake-up.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FLEXCOMM14_A {
    #[doc = "0: No effect"]
    FLEXCOMM14_0 = 0,
    #[doc = "1: Sets the STARTEN0 Bit"]
    FLEXCOMM14_1 = 1,
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
            false => FLEXCOMM14_A::FLEXCOMM14_0,
            true => FLEXCOMM14_A::FLEXCOMM14_1,
        }
    }
    #[doc = "Checks if the value of the field is `FLEXCOMM14_0`"]
    #[inline(always)]
    pub fn is_flexcomm14_0(&self) -> bool {
        *self == FLEXCOMM14_A::FLEXCOMM14_0
    }
    #[doc = "Checks if the value of the field is `FLEXCOMM14_1`"]
    #[inline(always)]
    pub fn is_flexcomm14_1(&self) -> bool {
        *self == FLEXCOMM14_A::FLEXCOMM14_1
    }
}
#[doc = "Field `FLEXCOMM14` writer - FlexComm 14 (High Speed SPI) peripheral interrupt wake-up."]
pub type FLEXCOMM14_W<'a, const O: u8> =
    crate::BitWriter1S<'a, u32, STARTEN0_SET_SPEC, FLEXCOMM14_A, O>;
impl<'a, const O: u8> FLEXCOMM14_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn flexcomm14_0(self) -> &'a mut W {
        self.variant(FLEXCOMM14_A::FLEXCOMM14_0)
    }
    #[doc = "Sets the STARTEN0 Bit"]
    #[inline(always)]
    pub fn flexcomm14_1(self) -> &'a mut W {
        self.variant(FLEXCOMM14_A::FLEXCOMM14_1)
    }
}
#[doc = "Field `FLEXCOMM15` reader - FlexComm 15 (PMIC I2C) peripheral interrupt wake-up."]
pub type FLEXCOMM15_R = crate::BitReader<FLEXCOMM15_A>;
#[doc = "FlexComm 15 (PMIC I2C) peripheral interrupt wake-up.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FLEXCOMM15_A {
    #[doc = "0: No effect"]
    FLEXCOMM15_0 = 0,
    #[doc = "1: Sets the STARTEN0 Bit"]
    FLEXCOMM15_1 = 1,
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
            false => FLEXCOMM15_A::FLEXCOMM15_0,
            true => FLEXCOMM15_A::FLEXCOMM15_1,
        }
    }
    #[doc = "Checks if the value of the field is `FLEXCOMM15_0`"]
    #[inline(always)]
    pub fn is_flexcomm15_0(&self) -> bool {
        *self == FLEXCOMM15_A::FLEXCOMM15_0
    }
    #[doc = "Checks if the value of the field is `FLEXCOMM15_1`"]
    #[inline(always)]
    pub fn is_flexcomm15_1(&self) -> bool {
        *self == FLEXCOMM15_A::FLEXCOMM15_1
    }
}
#[doc = "Field `FLEXCOMM15` writer - FlexComm 15 (PMIC I2C) peripheral interrupt wake-up."]
pub type FLEXCOMM15_W<'a, const O: u8> =
    crate::BitWriter1S<'a, u32, STARTEN0_SET_SPEC, FLEXCOMM15_A, O>;
impl<'a, const O: u8> FLEXCOMM15_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn flexcomm15_0(self) -> &'a mut W {
        self.variant(FLEXCOMM15_A::FLEXCOMM15_0)
    }
    #[doc = "Sets the STARTEN0 Bit"]
    #[inline(always)]
    pub fn flexcomm15_1(self) -> &'a mut W {
        self.variant(FLEXCOMM15_A::FLEXCOMM15_1)
    }
}
#[doc = "Field `ADC0` reader - ADC wake-up."]
pub type ADC0_R = crate::BitReader<ADC0_A>;
#[doc = "ADC wake-up.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADC0_A {
    #[doc = "0: No effect"]
    ADC0_0 = 0,
    #[doc = "1: Sets the STARTEN0 Bit"]
    ADC0_1 = 1,
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
            false => ADC0_A::ADC0_0,
            true => ADC0_A::ADC0_1,
        }
    }
    #[doc = "Checks if the value of the field is `ADC0_0`"]
    #[inline(always)]
    pub fn is_adc0_0(&self) -> bool {
        *self == ADC0_A::ADC0_0
    }
    #[doc = "Checks if the value of the field is `ADC0_1`"]
    #[inline(always)]
    pub fn is_adc0_1(&self) -> bool {
        *self == ADC0_A::ADC0_1
    }
}
#[doc = "Field `ADC0` writer - ADC wake-up."]
pub type ADC0_W<'a, const O: u8> = crate::BitWriter1S<'a, u32, STARTEN0_SET_SPEC, ADC0_A, O>;
impl<'a, const O: u8> ADC0_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn adc0_0(self) -> &'a mut W {
        self.variant(ADC0_A::ADC0_0)
    }
    #[doc = "Sets the STARTEN0 Bit"]
    #[inline(always)]
    pub fn adc0_1(self) -> &'a mut W {
        self.variant(ADC0_A::ADC0_1)
    }
}
#[doc = "Field `ACMP` reader - ACMP wake-up."]
pub type ACMP_R = crate::BitReader<ACMP_A>;
#[doc = "ACMP wake-up.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ACMP_A {
    #[doc = "0: No effect"]
    ACMP_0 = 0,
    #[doc = "1: Sets the STARTEN0 Bit"]
    ACMP_1 = 1,
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
            false => ACMP_A::ACMP_0,
            true => ACMP_A::ACMP_1,
        }
    }
    #[doc = "Checks if the value of the field is `ACMP_0`"]
    #[inline(always)]
    pub fn is_acmp_0(&self) -> bool {
        *self == ACMP_A::ACMP_0
    }
    #[doc = "Checks if the value of the field is `ACMP_1`"]
    #[inline(always)]
    pub fn is_acmp_1(&self) -> bool {
        *self == ACMP_A::ACMP_1
    }
}
#[doc = "Field `ACMP` writer - ACMP wake-up."]
pub type ACMP_W<'a, const O: u8> = crate::BitWriter1S<'a, u32, STARTEN0_SET_SPEC, ACMP_A, O>;
impl<'a, const O: u8> ACMP_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn acmp_0(self) -> &'a mut W {
        self.variant(ACMP_A::ACMP_0)
    }
    #[doc = "Sets the STARTEN0 Bit"]
    #[inline(always)]
    pub fn acmp_1(self) -> &'a mut W {
        self.variant(ACMP_A::ACMP_1)
    }
}
#[doc = "Field `DMIC0` reader - DMIC wake-up."]
pub type DMIC0_R = crate::BitReader<DMIC0_A>;
#[doc = "DMIC wake-up.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DMIC0_A {
    #[doc = "0: No effect"]
    DMIC0_0 = 0,
    #[doc = "1: Sets the STARTEN0 Bit"]
    DMIC0_1 = 1,
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
            false => DMIC0_A::DMIC0_0,
            true => DMIC0_A::DMIC0_1,
        }
    }
    #[doc = "Checks if the value of the field is `DMIC0_0`"]
    #[inline(always)]
    pub fn is_dmic0_0(&self) -> bool {
        *self == DMIC0_A::DMIC0_0
    }
    #[doc = "Checks if the value of the field is `DMIC0_1`"]
    #[inline(always)]
    pub fn is_dmic0_1(&self) -> bool {
        *self == DMIC0_A::DMIC0_1
    }
}
#[doc = "Field `DMIC0` writer - DMIC wake-up."]
pub type DMIC0_W<'a, const O: u8> = crate::BitWriter1S<'a, u32, STARTEN0_SET_SPEC, DMIC0_A, O>;
impl<'a, const O: u8> DMIC0_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn dmic0_0(self) -> &'a mut W {
        self.variant(DMIC0_A::DMIC0_0)
    }
    #[doc = "Sets the STARTEN0 Bit"]
    #[inline(always)]
    pub fn dmic0_1(self) -> &'a mut W {
        self.variant(DMIC0_A::DMIC0_1)
    }
}
#[doc = "Field `HYPERVISOR` reader - Hypervisor interrupt wake-up."]
pub type HYPERVISOR_R = crate::BitReader<HYPERVISOR_A>;
#[doc = "Hypervisor interrupt wake-up.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HYPERVISOR_A {
    #[doc = "0: No effect"]
    HYPERVISOR_0 = 0,
    #[doc = "1: Sets the STARTEN0 Bit"]
    HYPERVISOR_1 = 1,
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
            false => HYPERVISOR_A::HYPERVISOR_0,
            true => HYPERVISOR_A::HYPERVISOR_1,
        }
    }
    #[doc = "Checks if the value of the field is `HYPERVISOR_0`"]
    #[inline(always)]
    pub fn is_hypervisor_0(&self) -> bool {
        *self == HYPERVISOR_A::HYPERVISOR_0
    }
    #[doc = "Checks if the value of the field is `HYPERVISOR_1`"]
    #[inline(always)]
    pub fn is_hypervisor_1(&self) -> bool {
        *self == HYPERVISOR_A::HYPERVISOR_1
    }
}
#[doc = "Field `HYPERVISOR` writer - Hypervisor interrupt wake-up."]
pub type HYPERVISOR_W<'a, const O: u8> =
    crate::BitWriter1S<'a, u32, STARTEN0_SET_SPEC, HYPERVISOR_A, O>;
impl<'a, const O: u8> HYPERVISOR_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn hypervisor_0(self) -> &'a mut W {
        self.variant(HYPERVISOR_A::HYPERVISOR_0)
    }
    #[doc = "Sets the STARTEN0 Bit"]
    #[inline(always)]
    pub fn hypervisor_1(self) -> &'a mut W {
        self.variant(HYPERVISOR_A::HYPERVISOR_1)
    }
}
#[doc = "Field `SECUREVIOLATION` reader - Secure Violation wake-up."]
pub type SECUREVIOLATION_R = crate::BitReader<SECUREVIOLATION_A>;
#[doc = "Secure Violation wake-up.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SECUREVIOLATION_A {
    #[doc = "0: No effect"]
    SECUREVIOLATION_0 = 0,
    #[doc = "1: Sets the STARTEN0 Bit"]
    SECUREVIOLATION_1 = 1,
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
            false => SECUREVIOLATION_A::SECUREVIOLATION_0,
            true => SECUREVIOLATION_A::SECUREVIOLATION_1,
        }
    }
    #[doc = "Checks if the value of the field is `SECUREVIOLATION_0`"]
    #[inline(always)]
    pub fn is_secureviolation_0(&self) -> bool {
        *self == SECUREVIOLATION_A::SECUREVIOLATION_0
    }
    #[doc = "Checks if the value of the field is `SECUREVIOLATION_1`"]
    #[inline(always)]
    pub fn is_secureviolation_1(&self) -> bool {
        *self == SECUREVIOLATION_A::SECUREVIOLATION_1
    }
}
#[doc = "Field `SECUREVIOLATION` writer - Secure Violation wake-up."]
pub type SECUREVIOLATION_W<'a, const O: u8> =
    crate::BitWriter1S<'a, u32, STARTEN0_SET_SPEC, SECUREVIOLATION_A, O>;
impl<'a, const O: u8> SECUREVIOLATION_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn secureviolation_0(self) -> &'a mut W {
        self.variant(SECUREVIOLATION_A::SECUREVIOLATION_0)
    }
    #[doc = "Sets the STARTEN0 Bit"]
    #[inline(always)]
    pub fn secureviolation_1(self) -> &'a mut W {
        self.variant(SECUREVIOLATION_A::SECUREVIOLATION_1)
    }
}
#[doc = "Field `HWVAD0` reader - Hardware VAD wake-up."]
pub type HWVAD0_R = crate::BitReader<HWVAD0_A>;
#[doc = "Hardware VAD wake-up.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HWVAD0_A {
    #[doc = "0: No effect"]
    HWVAD0_0 = 0,
    #[doc = "1: Sets the STARTEN0 Bit"]
    HWVAD0_1 = 1,
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
            false => HWVAD0_A::HWVAD0_0,
            true => HWVAD0_A::HWVAD0_1,
        }
    }
    #[doc = "Checks if the value of the field is `HWVAD0_0`"]
    #[inline(always)]
    pub fn is_hwvad0_0(&self) -> bool {
        *self == HWVAD0_A::HWVAD0_0
    }
    #[doc = "Checks if the value of the field is `HWVAD0_1`"]
    #[inline(always)]
    pub fn is_hwvad0_1(&self) -> bool {
        *self == HWVAD0_A::HWVAD0_1
    }
}
#[doc = "Field `HWVAD0` writer - Hardware VAD wake-up."]
pub type HWVAD0_W<'a, const O: u8> = crate::BitWriter1S<'a, u32, STARTEN0_SET_SPEC, HWVAD0_A, O>;
impl<'a, const O: u8> HWVAD0_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn hwvad0_0(self) -> &'a mut W {
        self.variant(HWVAD0_A::HWVAD0_0)
    }
    #[doc = "Sets the STARTEN0 Bit"]
    #[inline(always)]
    pub fn hwvad0_1(self) -> &'a mut W {
        self.variant(HWVAD0_A::HWVAD0_1)
    }
}
#[doc = "Field `PMC` reader - PMC wake-up."]
pub type PMC_R = crate::BitReader<PMC_A>;
#[doc = "PMC wake-up.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PMC_A {
    #[doc = "0: No effect"]
    PMC_0 = 0,
    #[doc = "1: Sets the STARTEN0 Bit"]
    PMC_1 = 1,
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
            false => PMC_A::PMC_0,
            true => PMC_A::PMC_1,
        }
    }
    #[doc = "Checks if the value of the field is `PMC_0`"]
    #[inline(always)]
    pub fn is_pmc_0(&self) -> bool {
        *self == PMC_A::PMC_0
    }
    #[doc = "Checks if the value of the field is `PMC_1`"]
    #[inline(always)]
    pub fn is_pmc_1(&self) -> bool {
        *self == PMC_A::PMC_1
    }
}
#[doc = "Field `PMC` writer - PMC wake-up."]
pub type PMC_W<'a, const O: u8> = crate::BitWriter1S<'a, u32, STARTEN0_SET_SPEC, PMC_A, O>;
impl<'a, const O: u8> PMC_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn pmc_0(self) -> &'a mut W {
        self.variant(PMC_A::PMC_0)
    }
    #[doc = "Sets the STARTEN0 Bit"]
    #[inline(always)]
    pub fn pmc_1(self) -> &'a mut W {
        self.variant(PMC_A::PMC_1)
    }
}
#[doc = "Field `RNG` reader - RNG wake-up."]
pub type RNG_R = crate::BitReader<RNG_A>;
#[doc = "RNG wake-up.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RNG_A {
    #[doc = "0: No effect"]
    RNG_0 = 0,
    #[doc = "1: Sets the STARTEN0 Bit"]
    RNG_1 = 1,
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
            false => RNG_A::RNG_0,
            true => RNG_A::RNG_1,
        }
    }
    #[doc = "Checks if the value of the field is `RNG_0`"]
    #[inline(always)]
    pub fn is_rng_0(&self) -> bool {
        *self == RNG_A::RNG_0
    }
    #[doc = "Checks if the value of the field is `RNG_1`"]
    #[inline(always)]
    pub fn is_rng_1(&self) -> bool {
        *self == RNG_A::RNG_1
    }
}
#[doc = "Field `RNG` writer - RNG wake-up."]
pub type RNG_W<'a, const O: u8> = crate::BitWriter1S<'a, u32, STARTEN0_SET_SPEC, RNG_A, O>;
impl<'a, const O: u8> RNG_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn rng_0(self) -> &'a mut W {
        self.variant(RNG_A::RNG_0)
    }
    #[doc = "Sets the STARTEN0 Bit"]
    #[inline(always)]
    pub fn rng_1(self) -> &'a mut W {
        self.variant(RNG_A::RNG_1)
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
    #[doc = "Bit 10 - CTIMER 0 wake-up."]
    #[inline(always)]
    pub fn ct32bit0(&self) -> CT32BIT0_R {
        CT32BIT0_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - CTIMER 1 wake-up."]
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
    #[doc = "Bit 20 - FlexComm 14 (High Speed SPI) peripheral interrupt wake-up."]
    #[inline(always)]
    pub fn flexcomm14(&self) -> FLEXCOMM14_R {
        FLEXCOMM14_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - FlexComm 15 (PMIC I2C) peripheral interrupt wake-up."]
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
    #[doc = "Bit 10 - CTIMER 0 wake-up."]
    #[inline(always)]
    #[must_use]
    pub fn ct32bit0(&mut self) -> CT32BIT0_W<10> {
        CT32BIT0_W::new(self)
    }
    #[doc = "Bit 11 - CTIMER 1 wake-up."]
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
    #[doc = "Bit 20 - FlexComm 14 (High Speed SPI) peripheral interrupt wake-up."]
    #[inline(always)]
    #[must_use]
    pub fn flexcomm14(&mut self) -> FLEXCOMM14_W<20> {
        FLEXCOMM14_W::new(self)
    }
    #[doc = "Bit 21 - FlexComm 15 (PMIC I2C) peripheral interrupt wake-up."]
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
#[doc = "Start Enable 0 Set\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [starten0_set](index.html) module"]
pub struct STARTEN0_SET_SPEC;
impl crate::RegisterSpec for STARTEN0_SET_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [starten0_set::R](R) reader structure"]
impl crate::Readable for STARTEN0_SET_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [starten0_set::W](W) writer structure"]
impl crate::Writable for STARTEN0_SET_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0xfb70_3fff;
}
#[doc = "`reset()` method sets STARTEN0_SET to value 0"]
impl crate::Resettable for STARTEN0_SET_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
