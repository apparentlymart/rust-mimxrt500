#[doc = "Register `STARTEN1_SET` reader"]
pub struct R(crate::R<STARTEN1_SET_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<STARTEN1_SET_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<STARTEN1_SET_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<STARTEN1_SET_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `STARTEN1_SET` writer"]
pub struct W(crate::W<STARTEN1_SET_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<STARTEN1_SET_SPEC>;
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
impl From<crate::W<STARTEN1_SET_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<STARTEN1_SET_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RTC_LITE0_WAKEUP` reader - RTC wake-up."]
pub type RTC_LITE0_WAKEUP_R = crate::BitReader<RTC_LITE0_WAKEUP_A>;
#[doc = "RTC wake-up.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RTC_LITE0_WAKEUP_A {
    #[doc = "0: No Effect"]
    DISABLE = 0,
    #[doc = "1: Sets the STARTEN1 Bit"]
    ENABLE = 1,
}
impl From<RTC_LITE0_WAKEUP_A> for bool {
    #[inline(always)]
    fn from(variant: RTC_LITE0_WAKEUP_A) -> Self {
        variant as u8 != 0
    }
}
impl RTC_LITE0_WAKEUP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RTC_LITE0_WAKEUP_A {
        match self.bits {
            false => RTC_LITE0_WAKEUP_A::DISABLE,
            true => RTC_LITE0_WAKEUP_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == RTC_LITE0_WAKEUP_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == RTC_LITE0_WAKEUP_A::ENABLE
    }
}
#[doc = "Field `RTC_LITE0_WAKEUP` writer - RTC wake-up."]
pub type RTC_LITE0_WAKEUP_W<'a, const O: u8> =
    crate::BitWriter1S<'a, u32, STARTEN1_SET_SPEC, RTC_LITE0_WAKEUP_A, O>;
impl<'a, const O: u8> RTC_LITE0_WAKEUP_W<'a, O> {
    #[doc = "No Effect"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(RTC_LITE0_WAKEUP_A::DISABLE)
    }
    #[doc = "Sets the STARTEN1 Bit"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(RTC_LITE0_WAKEUP_A::ENABLE)
    }
}
#[doc = "Field `DSP_TIE_EXPSTATE1` reader - DSP wake-up"]
pub type DSP_TIE_EXPSTATE1_R = crate::BitReader<DSP_TIE_EXPSTATE1_A>;
#[doc = "DSP wake-up\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DSP_TIE_EXPSTATE1_A {
    #[doc = "0: Disabled"]
    DISABLE = 0,
    #[doc = "1: Enabled"]
    ENABLE = 1,
}
impl From<DSP_TIE_EXPSTATE1_A> for bool {
    #[inline(always)]
    fn from(variant: DSP_TIE_EXPSTATE1_A) -> Self {
        variant as u8 != 0
    }
}
impl DSP_TIE_EXPSTATE1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DSP_TIE_EXPSTATE1_A {
        match self.bits {
            false => DSP_TIE_EXPSTATE1_A::DISABLE,
            true => DSP_TIE_EXPSTATE1_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == DSP_TIE_EXPSTATE1_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == DSP_TIE_EXPSTATE1_A::ENABLE
    }
}
#[doc = "Field `DSP_TIE_EXPSTATE1` writer - DSP wake-up"]
pub type DSP_TIE_EXPSTATE1_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, STARTEN1_SET_SPEC, DSP_TIE_EXPSTATE1_A, O>;
impl<'a, const O: u8> DSP_TIE_EXPSTATE1_W<'a, O> {
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(DSP_TIE_EXPSTATE1_A::DISABLE)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(DSP_TIE_EXPSTATE1_A::ENABLE)
    }
}
#[doc = "Field `MU` reader - Message Unit wake-up."]
pub type MU_R = crate::BitReader<MU_A>;
#[doc = "Message Unit wake-up.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MU_A {
    #[doc = "0: No Effect"]
    DISABLE = 0,
    #[doc = "1: Sets the STARTEN1 Bit"]
    ENABLE = 1,
}
impl From<MU_A> for bool {
    #[inline(always)]
    fn from(variant: MU_A) -> Self {
        variant as u8 != 0
    }
}
impl MU_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MU_A {
        match self.bits {
            false => MU_A::DISABLE,
            true => MU_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == MU_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == MU_A::ENABLE
    }
}
#[doc = "Field `MU` writer - Message Unit wake-up."]
pub type MU_W<'a, const O: u8> = crate::BitWriter1S<'a, u32, STARTEN1_SET_SPEC, MU_A, O>;
impl<'a, const O: u8> MU_W<'a, O> {
    #[doc = "No Effect"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(MU_A::DISABLE)
    }
    #[doc = "Sets the STARTEN1 Bit"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(MU_A::ENABLE)
    }
}
#[doc = "Field `GPIO_INT0_IRQ4` reader - Message Unit wake-up."]
pub type GPIO_INT0_IRQ4_R = crate::BitReader<GPIO_INT0_IRQ4_A>;
#[doc = "Message Unit wake-up.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GPIO_INT0_IRQ4_A {
    #[doc = "0: No Effect"]
    DISABLE = 0,
    #[doc = "1: Sets the STARTEN1 Bit"]
    ENABLE = 1,
}
impl From<GPIO_INT0_IRQ4_A> for bool {
    #[inline(always)]
    fn from(variant: GPIO_INT0_IRQ4_A) -> Self {
        variant as u8 != 0
    }
}
impl GPIO_INT0_IRQ4_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO_INT0_IRQ4_A {
        match self.bits {
            false => GPIO_INT0_IRQ4_A::DISABLE,
            true => GPIO_INT0_IRQ4_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == GPIO_INT0_IRQ4_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == GPIO_INT0_IRQ4_A::ENABLE
    }
}
#[doc = "Field `GPIO_INT0_IRQ4` writer - Message Unit wake-up."]
pub type GPIO_INT0_IRQ4_W<'a, const O: u8> =
    crate::BitWriter1S<'a, u32, STARTEN1_SET_SPEC, GPIO_INT0_IRQ4_A, O>;
impl<'a, const O: u8> GPIO_INT0_IRQ4_W<'a, O> {
    #[doc = "No Effect"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(GPIO_INT0_IRQ4_A::DISABLE)
    }
    #[doc = "Sets the STARTEN1 Bit"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(GPIO_INT0_IRQ4_A::ENABLE)
    }
}
#[doc = "Field `GPIO_INT0_IRQ5` reader - GPIO pin interrupt 5 wake-up."]
pub type GPIO_INT0_IRQ5_R = crate::BitReader<GPIO_INT0_IRQ5_A>;
#[doc = "GPIO pin interrupt 5 wake-up.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GPIO_INT0_IRQ5_A {
    #[doc = "0: No Effect"]
    DISABLE = 0,
    #[doc = "1: Sets the STARTEN1 Bit"]
    ENABLE = 1,
}
impl From<GPIO_INT0_IRQ5_A> for bool {
    #[inline(always)]
    fn from(variant: GPIO_INT0_IRQ5_A) -> Self {
        variant as u8 != 0
    }
}
impl GPIO_INT0_IRQ5_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO_INT0_IRQ5_A {
        match self.bits {
            false => GPIO_INT0_IRQ5_A::DISABLE,
            true => GPIO_INT0_IRQ5_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == GPIO_INT0_IRQ5_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == GPIO_INT0_IRQ5_A::ENABLE
    }
}
#[doc = "Field `GPIO_INT0_IRQ5` writer - GPIO pin interrupt 5 wake-up."]
pub type GPIO_INT0_IRQ5_W<'a, const O: u8> =
    crate::BitWriter1S<'a, u32, STARTEN1_SET_SPEC, GPIO_INT0_IRQ5_A, O>;
impl<'a, const O: u8> GPIO_INT0_IRQ5_W<'a, O> {
    #[doc = "No Effect"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(GPIO_INT0_IRQ5_A::DISABLE)
    }
    #[doc = "Sets the STARTEN1 Bit"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(GPIO_INT0_IRQ5_A::ENABLE)
    }
}
#[doc = "Field `GPIO_INT0_IRQ6` reader - GPIO pin interrupt 6 wake-up."]
pub type GPIO_INT0_IRQ6_R = crate::BitReader<GPIO_INT0_IRQ6_A>;
#[doc = "GPIO pin interrupt 6 wake-up.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GPIO_INT0_IRQ6_A {
    #[doc = "0: No Effect"]
    DISABLE = 0,
    #[doc = "1: Sets the STARTEN1 Bit"]
    ENABLE = 1,
}
impl From<GPIO_INT0_IRQ6_A> for bool {
    #[inline(always)]
    fn from(variant: GPIO_INT0_IRQ6_A) -> Self {
        variant as u8 != 0
    }
}
impl GPIO_INT0_IRQ6_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO_INT0_IRQ6_A {
        match self.bits {
            false => GPIO_INT0_IRQ6_A::DISABLE,
            true => GPIO_INT0_IRQ6_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == GPIO_INT0_IRQ6_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == GPIO_INT0_IRQ6_A::ENABLE
    }
}
#[doc = "Field `GPIO_INT0_IRQ6` writer - GPIO pin interrupt 6 wake-up."]
pub type GPIO_INT0_IRQ6_W<'a, const O: u8> =
    crate::BitWriter1S<'a, u32, STARTEN1_SET_SPEC, GPIO_INT0_IRQ6_A, O>;
impl<'a, const O: u8> GPIO_INT0_IRQ6_W<'a, O> {
    #[doc = "No Effect"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(GPIO_INT0_IRQ6_A::DISABLE)
    }
    #[doc = "Sets the STARTEN1 Bit"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(GPIO_INT0_IRQ6_A::ENABLE)
    }
}
#[doc = "Field `GPIO_INT0_IRQ7` reader - GPIO pin interrupt 7 wake-up."]
pub type GPIO_INT0_IRQ7_R = crate::BitReader<GPIO_INT0_IRQ7_A>;
#[doc = "GPIO pin interrupt 7 wake-up.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GPIO_INT0_IRQ7_A {
    #[doc = "0: No Effect"]
    DISABLE = 0,
    #[doc = "1: Sets the STARTEN1 Bit"]
    ENABLE = 1,
}
impl From<GPIO_INT0_IRQ7_A> for bool {
    #[inline(always)]
    fn from(variant: GPIO_INT0_IRQ7_A) -> Self {
        variant as u8 != 0
    }
}
impl GPIO_INT0_IRQ7_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO_INT0_IRQ7_A {
        match self.bits {
            false => GPIO_INT0_IRQ7_A::DISABLE,
            true => GPIO_INT0_IRQ7_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == GPIO_INT0_IRQ7_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == GPIO_INT0_IRQ7_A::ENABLE
    }
}
#[doc = "Field `GPIO_INT0_IRQ7` writer - GPIO pin interrupt 7 wake-up."]
pub type GPIO_INT0_IRQ7_W<'a, const O: u8> =
    crate::BitWriter1S<'a, u32, STARTEN1_SET_SPEC, GPIO_INT0_IRQ7_A, O>;
impl<'a, const O: u8> GPIO_INT0_IRQ7_W<'a, O> {
    #[doc = "No Effect"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(GPIO_INT0_IRQ7_A::DISABLE)
    }
    #[doc = "Sets the STARTEN1 Bit"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(GPIO_INT0_IRQ7_A::ENABLE)
    }
}
#[doc = "Field `CT32BIT2` reader - CTIMER 2 wake-up"]
pub type CT32BIT2_R = crate::BitReader<CT32BIT2_A>;
#[doc = "CTIMER 2 wake-up\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CT32BIT2_A {
    #[doc = "0: No Effect"]
    DISABLE = 0,
    #[doc = "1: Sets the STARTEN1 Bit"]
    ENABLE = 1,
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
            false => CT32BIT2_A::DISABLE,
            true => CT32BIT2_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == CT32BIT2_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == CT32BIT2_A::ENABLE
    }
}
#[doc = "Field `CT32BIT2` writer - CTIMER 2 wake-up"]
pub type CT32BIT2_W<'a, const O: u8> =
    crate::BitWriter1S<'a, u32, STARTEN1_SET_SPEC, CT32BIT2_A, O>;
impl<'a, const O: u8> CT32BIT2_W<'a, O> {
    #[doc = "No Effect"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(CT32BIT2_A::DISABLE)
    }
    #[doc = "Sets the STARTEN1 Bit"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(CT32BIT2_A::ENABLE)
    }
}
#[doc = "Field `CT32BIT4` reader - CTIMER 4 wake-up"]
pub type CT32BIT4_R = crate::BitReader<CT32BIT4_A>;
#[doc = "CTIMER 4 wake-up\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CT32BIT4_A {
    #[doc = "0: No Effect"]
    DISABLE = 0,
    #[doc = "1: Sets the STARTEN1 Bit"]
    ENABLE = 1,
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
            false => CT32BIT4_A::DISABLE,
            true => CT32BIT4_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == CT32BIT4_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == CT32BIT4_A::ENABLE
    }
}
#[doc = "Field `CT32BIT4` writer - CTIMER 4 wake-up"]
pub type CT32BIT4_W<'a, const O: u8> =
    crate::BitWriter1S<'a, u32, STARTEN1_SET_SPEC, CT32BIT4_A, O>;
impl<'a, const O: u8> CT32BIT4_W<'a, O> {
    #[doc = "No Effect"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(CT32BIT4_A::DISABLE)
    }
    #[doc = "Sets the STARTEN1 Bit"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(CT32BIT4_A::ENABLE)
    }
}
#[doc = "Field `OS_EVENT_TIMER_WU` reader - OS Event Timer wake-up."]
pub type OS_EVENT_TIMER_WU_R = crate::BitReader<OS_EVENT_TIMER_WU_A>;
#[doc = "OS Event Timer wake-up.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OS_EVENT_TIMER_WU_A {
    #[doc = "0: No Effect"]
    DISABLE = 0,
    #[doc = "1: Sets the STARTEN1 Bit"]
    ENABLE = 1,
}
impl From<OS_EVENT_TIMER_WU_A> for bool {
    #[inline(always)]
    fn from(variant: OS_EVENT_TIMER_WU_A) -> Self {
        variant as u8 != 0
    }
}
impl OS_EVENT_TIMER_WU_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OS_EVENT_TIMER_WU_A {
        match self.bits {
            false => OS_EVENT_TIMER_WU_A::DISABLE,
            true => OS_EVENT_TIMER_WU_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == OS_EVENT_TIMER_WU_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == OS_EVENT_TIMER_WU_A::ENABLE
    }
}
#[doc = "Field `OS_EVENT_TIMER_WU` writer - OS Event Timer wake-up."]
pub type OS_EVENT_TIMER_WU_W<'a, const O: u8> =
    crate::BitWriter1S<'a, u32, STARTEN1_SET_SPEC, OS_EVENT_TIMER_WU_A, O>;
impl<'a, const O: u8> OS_EVENT_TIMER_WU_W<'a, O> {
    #[doc = "No Effect"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(OS_EVENT_TIMER_WU_A::DISABLE)
    }
    #[doc = "Sets the STARTEN1 Bit"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(OS_EVENT_TIMER_WU_A::ENABLE)
    }
}
#[doc = "Field `FLEXSPI` reader - Quad/octal SPI wake-up."]
pub type FLEXSPI_R = crate::BitReader<FLEXSPI_A>;
#[doc = "Quad/octal SPI wake-up.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FLEXSPI_A {
    #[doc = "0: No Effect"]
    DISABLE = 0,
    #[doc = "1: Sets the STARTEN1 Bit"]
    ENABLE = 1,
}
impl From<FLEXSPI_A> for bool {
    #[inline(always)]
    fn from(variant: FLEXSPI_A) -> Self {
        variant as u8 != 0
    }
}
impl FLEXSPI_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FLEXSPI_A {
        match self.bits {
            false => FLEXSPI_A::DISABLE,
            true => FLEXSPI_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == FLEXSPI_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == FLEXSPI_A::ENABLE
    }
}
#[doc = "Field `FLEXSPI` writer - Quad/octal SPI wake-up."]
pub type FLEXSPI_W<'a, const O: u8> = crate::BitWriter1S<'a, u32, STARTEN1_SET_SPEC, FLEXSPI_A, O>;
impl<'a, const O: u8> FLEXSPI_W<'a, O> {
    #[doc = "No Effect"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(FLEXSPI_A::DISABLE)
    }
    #[doc = "Sets the STARTEN1 Bit"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(FLEXSPI_A::ENABLE)
    }
}
#[doc = "Field `FLEXCOMM6` reader - FLEXCOMM6 wake-up."]
pub type FLEXCOMM6_R = crate::BitReader<FLEXCOMM6_A>;
#[doc = "FLEXCOMM6 wake-up.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FLEXCOMM6_A {
    #[doc = "0: No Effect"]
    DISABLE = 0,
    #[doc = "1: Sets the STARTEN1 Bit"]
    ENABLE = 1,
}
impl From<FLEXCOMM6_A> for bool {
    #[inline(always)]
    fn from(variant: FLEXCOMM6_A) -> Self {
        variant as u8 != 0
    }
}
impl FLEXCOMM6_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FLEXCOMM6_A {
        match self.bits {
            false => FLEXCOMM6_A::DISABLE,
            true => FLEXCOMM6_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == FLEXCOMM6_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == FLEXCOMM6_A::ENABLE
    }
}
#[doc = "Field `FLEXCOMM6` writer - FLEXCOMM6 wake-up."]
pub type FLEXCOMM6_W<'a, const O: u8> =
    crate::BitWriter1S<'a, u32, STARTEN1_SET_SPEC, FLEXCOMM6_A, O>;
impl<'a, const O: u8> FLEXCOMM6_W<'a, O> {
    #[doc = "No Effect"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(FLEXCOMM6_A::DISABLE)
    }
    #[doc = "Sets the STARTEN1 Bit"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(FLEXCOMM6_A::ENABLE)
    }
}
#[doc = "Field `FLEXCOMM7` reader - FLEXCOMM7 wake-up."]
pub type FLEXCOMM7_R = crate::BitReader<FLEXCOMM7_A>;
#[doc = "FLEXCOMM7 wake-up.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FLEXCOMM7_A {
    #[doc = "0: No Effect"]
    DISABLE = 0,
    #[doc = "1: Sets the STARTEN1 Bit"]
    ENABLE = 1,
}
impl From<FLEXCOMM7_A> for bool {
    #[inline(always)]
    fn from(variant: FLEXCOMM7_A) -> Self {
        variant as u8 != 0
    }
}
impl FLEXCOMM7_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FLEXCOMM7_A {
        match self.bits {
            false => FLEXCOMM7_A::DISABLE,
            true => FLEXCOMM7_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == FLEXCOMM7_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == FLEXCOMM7_A::ENABLE
    }
}
#[doc = "Field `FLEXCOMM7` writer - FLEXCOMM7 wake-up."]
pub type FLEXCOMM7_W<'a, const O: u8> =
    crate::BitWriter1S<'a, u32, STARTEN1_SET_SPEC, FLEXCOMM7_A, O>;
impl<'a, const O: u8> FLEXCOMM7_W<'a, O> {
    #[doc = "No Effect"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(FLEXCOMM7_A::DISABLE)
    }
    #[doc = "Sets the STARTEN1 Bit"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(FLEXCOMM7_A::ENABLE)
    }
}
#[doc = "Field `SDIO0` reader - SDIO0 wake-up."]
pub type SDIO0_R = crate::BitReader<SDIO0_A>;
#[doc = "SDIO0 wake-up.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SDIO0_A {
    #[doc = "0: No Effect"]
    DISABLE = 0,
    #[doc = "1: Sets the STARTEN1 Bit"]
    ENABLE = 1,
}
impl From<SDIO0_A> for bool {
    #[inline(always)]
    fn from(variant: SDIO0_A) -> Self {
        variant as u8 != 0
    }
}
impl SDIO0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SDIO0_A {
        match self.bits {
            false => SDIO0_A::DISABLE,
            true => SDIO0_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == SDIO0_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == SDIO0_A::ENABLE
    }
}
#[doc = "Field `SDIO0` writer - SDIO0 wake-up."]
pub type SDIO0_W<'a, const O: u8> = crate::BitWriter1S<'a, u32, STARTEN1_SET_SPEC, SDIO0_A, O>;
impl<'a, const O: u8> SDIO0_W<'a, O> {
    #[doc = "No Effect"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(SDIO0_A::DISABLE)
    }
    #[doc = "Sets the STARTEN1 Bit"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(SDIO0_A::ENABLE)
    }
}
#[doc = "Field `SDIO1` reader - SDIO01 wake-up."]
pub type SDIO1_R = crate::BitReader<SDIO1_A>;
#[doc = "SDIO01 wake-up.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SDIO1_A {
    #[doc = "0: No Effect"]
    DISABLE = 0,
    #[doc = "1: Sets the STARTEN1 Bit"]
    ENABLE = 1,
}
impl From<SDIO1_A> for bool {
    #[inline(always)]
    fn from(variant: SDIO1_A) -> Self {
        variant as u8 != 0
    }
}
impl SDIO1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SDIO1_A {
        match self.bits {
            false => SDIO1_A::DISABLE,
            true => SDIO1_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == SDIO1_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == SDIO1_A::ENABLE
    }
}
#[doc = "Field `SDIO1` writer - SDIO01 wake-up."]
pub type SDIO1_W<'a, const O: u8> = crate::BitWriter1S<'a, u32, STARTEN1_SET_SPEC, SDIO1_A, O>;
impl<'a, const O: u8> SDIO1_W<'a, O> {
    #[doc = "No Effect"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(SDIO1_A::DISABLE)
    }
    #[doc = "Sets the STARTEN1 Bit"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(SDIO1_A::ENABLE)
    }
}
#[doc = "Field `SGPIO_INTA` reader - Secure GPIO interrupt A wake-up."]
pub type SGPIO_INTA_R = crate::BitReader<SGPIO_INTA_A>;
#[doc = "Secure GPIO interrupt A wake-up.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SGPIO_INTA_A {
    #[doc = "0: No Effect"]
    DISABLE = 0,
    #[doc = "1: Sets the STARTEN1 Bit"]
    ENABLE = 1,
}
impl From<SGPIO_INTA_A> for bool {
    #[inline(always)]
    fn from(variant: SGPIO_INTA_A) -> Self {
        variant as u8 != 0
    }
}
impl SGPIO_INTA_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SGPIO_INTA_A {
        match self.bits {
            false => SGPIO_INTA_A::DISABLE,
            true => SGPIO_INTA_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == SGPIO_INTA_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == SGPIO_INTA_A::ENABLE
    }
}
#[doc = "Field `SGPIO_INTA` writer - Secure GPIO interrupt A wake-up."]
pub type SGPIO_INTA_W<'a, const O: u8> =
    crate::BitWriter1S<'a, u32, STARTEN1_SET_SPEC, SGPIO_INTA_A, O>;
impl<'a, const O: u8> SGPIO_INTA_W<'a, O> {
    #[doc = "No Effect"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(SGPIO_INTA_A::DISABLE)
    }
    #[doc = "Sets the STARTEN1 Bit"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(SGPIO_INTA_A::ENABLE)
    }
}
#[doc = "Field `SGPIO_INTB` reader - Secure GPIO interrupt B wake-up."]
pub type SGPIO_INTB_R = crate::BitReader<SGPIO_INTB_A>;
#[doc = "Secure GPIO interrupt B wake-up.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SGPIO_INTB_A {
    #[doc = "0: No Effect"]
    DISABLE = 0,
    #[doc = "1: Sets the STARTEN1 Bit"]
    ENABLE = 1,
}
impl From<SGPIO_INTB_A> for bool {
    #[inline(always)]
    fn from(variant: SGPIO_INTB_A) -> Self {
        variant as u8 != 0
    }
}
impl SGPIO_INTB_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SGPIO_INTB_A {
        match self.bits {
            false => SGPIO_INTB_A::DISABLE,
            true => SGPIO_INTB_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == SGPIO_INTB_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == SGPIO_INTB_A::ENABLE
    }
}
#[doc = "Field `SGPIO_INTB` writer - Secure GPIO interrupt B wake-up."]
pub type SGPIO_INTB_W<'a, const O: u8> =
    crate::BitWriter1S<'a, u32, STARTEN1_SET_SPEC, SGPIO_INTB_A, O>;
impl<'a, const O: u8> SGPIO_INTB_W<'a, O> {
    #[doc = "No Effect"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(SGPIO_INTB_A::DISABLE)
    }
    #[doc = "Sets the STARTEN1 Bit"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(SGPIO_INTB_A::ENABLE)
    }
}
#[doc = "Field `USB0_NEEDCLK` reader - USB activity wake-up."]
pub type USB0_NEEDCLK_R = crate::BitReader<USB0_NEEDCLK_A>;
#[doc = "USB activity wake-up.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum USB0_NEEDCLK_A {
    #[doc = "0: No Effect"]
    DISABLE = 0,
    #[doc = "1: Sets the STARTEN1 Bit"]
    ENABLE = 1,
}
impl From<USB0_NEEDCLK_A> for bool {
    #[inline(always)]
    fn from(variant: USB0_NEEDCLK_A) -> Self {
        variant as u8 != 0
    }
}
impl USB0_NEEDCLK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> USB0_NEEDCLK_A {
        match self.bits {
            false => USB0_NEEDCLK_A::DISABLE,
            true => USB0_NEEDCLK_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == USB0_NEEDCLK_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == USB0_NEEDCLK_A::ENABLE
    }
}
#[doc = "Field `USB0_NEEDCLK` writer - USB activity wake-up."]
pub type USB0_NEEDCLK_W<'a, const O: u8> =
    crate::BitWriter1S<'a, u32, STARTEN1_SET_SPEC, USB0_NEEDCLK_A, O>;
impl<'a, const O: u8> USB0_NEEDCLK_W<'a, O> {
    #[doc = "No Effect"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(USB0_NEEDCLK_A::DISABLE)
    }
    #[doc = "Sets the STARTEN1 Bit"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(USB0_NEEDCLK_A::ENABLE)
    }
}
#[doc = "Field `USB_PHYDCD` reader - USB PHY DCD interrupt wake-up"]
pub type USB_PHYDCD_R = crate::BitReader<USB_PHYDCD_A>;
#[doc = "USB PHY DCD interrupt wake-up\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum USB_PHYDCD_A {
    #[doc = "0: No Effect"]
    DISABLE = 0,
    #[doc = "1: Sets the STARTEN1 Bit"]
    ENABLE = 1,
}
impl From<USB_PHYDCD_A> for bool {
    #[inline(always)]
    fn from(variant: USB_PHYDCD_A) -> Self {
        variant as u8 != 0
    }
}
impl USB_PHYDCD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> USB_PHYDCD_A {
        match self.bits {
            false => USB_PHYDCD_A::DISABLE,
            true => USB_PHYDCD_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == USB_PHYDCD_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == USB_PHYDCD_A::ENABLE
    }
}
#[doc = "Field `USB_PHYDCD` writer - USB PHY DCD interrupt wake-up"]
pub type USB_PHYDCD_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, STARTEN1_SET_SPEC, USB_PHYDCD_A, O>;
impl<'a, const O: u8> USB_PHYDCD_W<'a, O> {
    #[doc = "No Effect"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(USB_PHYDCD_A::DISABLE)
    }
    #[doc = "Sets the STARTEN1 Bit"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(USB_PHYDCD_A::ENABLE)
    }
}
#[doc = "Field `DMAC1` reader - DMA controller 1 wake-up."]
pub type DMAC1_R = crate::BitReader<DMAC1_A>;
#[doc = "DMA controller 1 wake-up.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DMAC1_A {
    #[doc = "0: No Effect"]
    DISABLE = 0,
    #[doc = "1: Sets the STARTEN1 Bit"]
    ENABLE = 1,
}
impl From<DMAC1_A> for bool {
    #[inline(always)]
    fn from(variant: DMAC1_A) -> Self {
        variant as u8 != 0
    }
}
impl DMAC1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DMAC1_A {
        match self.bits {
            false => DMAC1_A::DISABLE,
            true => DMAC1_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == DMAC1_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == DMAC1_A::ENABLE
    }
}
#[doc = "Field `DMAC1` writer - DMA controller 1 wake-up."]
pub type DMAC1_W<'a, const O: u8> = crate::BitWriter1S<'a, u32, STARTEN1_SET_SPEC, DMAC1_A, O>;
impl<'a, const O: u8> DMAC1_W<'a, O> {
    #[doc = "No Effect"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(DMAC1_A::DISABLE)
    }
    #[doc = "Sets the STARTEN1 Bit"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(DMAC1_A::ENABLE)
    }
}
#[doc = "Field `PUF` reader - PUF wake-up."]
pub type PUF_R = crate::BitReader<PUF_A>;
#[doc = "PUF wake-up.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PUF_A {
    #[doc = "0: No Effect"]
    DISABLE = 0,
    #[doc = "1: Sets the STARTEN1 Bit"]
    ENABLE = 1,
}
impl From<PUF_A> for bool {
    #[inline(always)]
    fn from(variant: PUF_A) -> Self {
        variant as u8 != 0
    }
}
impl PUF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PUF_A {
        match self.bits {
            false => PUF_A::DISABLE,
            true => PUF_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == PUF_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == PUF_A::ENABLE
    }
}
#[doc = "Field `PUF` writer - PUF wake-up."]
pub type PUF_W<'a, const O: u8> = crate::BitWriter1S<'a, u32, STARTEN1_SET_SPEC, PUF_A, O>;
impl<'a, const O: u8> PUF_W<'a, O> {
    #[doc = "No Effect"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(PUF_A::DISABLE)
    }
    #[doc = "Sets the STARTEN1 Bit"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(PUF_A::ENABLE)
    }
}
#[doc = "Field `POWERQUAD` reader - POWERQUAD co-processor wake-up."]
pub type POWERQUAD_R = crate::BitReader<POWERQUAD_A>;
#[doc = "POWERQUAD co-processor wake-up.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum POWERQUAD_A {
    #[doc = "0: No Effect"]
    DISABLE = 0,
    #[doc = "1: Sets the STARTEN1 Bit"]
    ENABLE = 1,
}
impl From<POWERQUAD_A> for bool {
    #[inline(always)]
    fn from(variant: POWERQUAD_A) -> Self {
        variant as u8 != 0
    }
}
impl POWERQUAD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> POWERQUAD_A {
        match self.bits {
            false => POWERQUAD_A::DISABLE,
            true => POWERQUAD_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == POWERQUAD_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == POWERQUAD_A::ENABLE
    }
}
#[doc = "Field `POWERQUAD` writer - POWERQUAD co-processor wake-up."]
pub type POWERQUAD_W<'a, const O: u8> =
    crate::BitWriter1S<'a, u32, STARTEN1_SET_SPEC, POWERQUAD_A, O>;
impl<'a, const O: u8> POWERQUAD_W<'a, O> {
    #[doc = "No Effect"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(POWERQUAD_A::DISABLE)
    }
    #[doc = "Sets the STARTEN1 Bit"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(POWERQUAD_A::ENABLE)
    }
}
#[doc = "Field `CASPER` reader - CASPER co-processor wake-up."]
pub type CASPER_R = crate::BitReader<CASPER_A>;
#[doc = "CASPER co-processor wake-up.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CASPER_A {
    #[doc = "0: No Effect"]
    DISABLE = 0,
    #[doc = "1: Sets the STARTEN1 Bit"]
    ENABLE = 1,
}
impl From<CASPER_A> for bool {
    #[inline(always)]
    fn from(variant: CASPER_A) -> Self {
        variant as u8 != 0
    }
}
impl CASPER_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CASPER_A {
        match self.bits {
            false => CASPER_A::DISABLE,
            true => CASPER_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == CASPER_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == CASPER_A::ENABLE
    }
}
#[doc = "Field `CASPER` writer - CASPER co-processor wake-up."]
pub type CASPER_W<'a, const O: u8> = crate::BitWriter1S<'a, u32, STARTEN1_SET_SPEC, CASPER_A, O>;
impl<'a, const O: u8> CASPER_W<'a, O> {
    #[doc = "No Effect"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(CASPER_A::DISABLE)
    }
    #[doc = "Sets the STARTEN1 Bit"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(CASPER_A::ENABLE)
    }
}
#[doc = "Field `PMIC` reader - Wake-up from on-chip PMC or off-chip PMIC."]
pub type PMIC_R = crate::BitReader<PMIC_A>;
#[doc = "Wake-up from on-chip PMC or off-chip PMIC.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PMIC_A {
    #[doc = "0: No Effect"]
    DISABLE = 0,
    #[doc = "1: Sets the STARTEN1 Bit"]
    ENABLE = 1,
}
impl From<PMIC_A> for bool {
    #[inline(always)]
    fn from(variant: PMIC_A) -> Self {
        variant as u8 != 0
    }
}
impl PMIC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PMIC_A {
        match self.bits {
            false => PMIC_A::DISABLE,
            true => PMIC_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == PMIC_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == PMIC_A::ENABLE
    }
}
#[doc = "Field `PMIC` writer - Wake-up from on-chip PMC or off-chip PMIC."]
pub type PMIC_W<'a, const O: u8> = crate::BitWriter1S<'a, u32, STARTEN1_SET_SPEC, PMIC_A, O>;
impl<'a, const O: u8> PMIC_W<'a, O> {
    #[doc = "No Effect"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(PMIC_A::DISABLE)
    }
    #[doc = "Sets the STARTEN1 Bit"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(PMIC_A::ENABLE)
    }
}
#[doc = "Field `SHA` reader - Hash-AES wake-up."]
pub type SHA_R = crate::BitReader<SHA_A>;
#[doc = "Hash-AES wake-up.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SHA_A {
    #[doc = "0: No Effect"]
    DISABLE = 0,
    #[doc = "1: Sets the STARTEN1 Bit"]
    ENABLE = 1,
}
impl From<SHA_A> for bool {
    #[inline(always)]
    fn from(variant: SHA_A) -> Self {
        variant as u8 != 0
    }
}
impl SHA_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SHA_A {
        match self.bits {
            false => SHA_A::DISABLE,
            true => SHA_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == SHA_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == SHA_A::ENABLE
    }
}
#[doc = "Field `SHA` writer - Hash-AES wake-up."]
pub type SHA_W<'a, const O: u8> = crate::BitWriter1S<'a, u32, STARTEN1_SET_SPEC, SHA_A, O>;
impl<'a, const O: u8> SHA_W<'a, O> {
    #[doc = "No Effect"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(SHA_A::DISABLE)
    }
    #[doc = "Sets the STARTEN1 Bit"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(SHA_A::ENABLE)
    }
}
#[doc = "Field `FLEXCOMM8` reader - FLEXCOMM 8 peripheral interrupt wake-up."]
pub type FLEXCOMM8_R = crate::BitReader<FLEXCOMM8_A>;
#[doc = "FLEXCOMM 8 peripheral interrupt wake-up.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FLEXCOMM8_A {
    #[doc = "0: No Effect"]
    DISABLE = 0,
    #[doc = "1: Sets the STARTEN1 Bit"]
    ENABLE = 1,
}
impl From<FLEXCOMM8_A> for bool {
    #[inline(always)]
    fn from(variant: FLEXCOMM8_A) -> Self {
        variant as u8 != 0
    }
}
impl FLEXCOMM8_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FLEXCOMM8_A {
        match self.bits {
            false => FLEXCOMM8_A::DISABLE,
            true => FLEXCOMM8_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == FLEXCOMM8_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == FLEXCOMM8_A::ENABLE
    }
}
#[doc = "Field `FLEXCOMM8` writer - FLEXCOMM 8 peripheral interrupt wake-up."]
pub type FLEXCOMM8_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, STARTEN1_SET_SPEC, FLEXCOMM8_A, O>;
impl<'a, const O: u8> FLEXCOMM8_W<'a, O> {
    #[doc = "No Effect"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(FLEXCOMM8_A::DISABLE)
    }
    #[doc = "Sets the STARTEN1 Bit"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(FLEXCOMM8_A::ENABLE)
    }
}
#[doc = "Field `FLEXCOMM9` reader - FLEXCOMM 9 peripheral interrupt wake-up."]
pub type FLEXCOMM9_R = crate::BitReader<FLEXCOMM9_A>;
#[doc = "FLEXCOMM 9 peripheral interrupt wake-up.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FLEXCOMM9_A {
    #[doc = "0: No Effect"]
    DISABLE = 0,
    #[doc = "1: Sets the STARTEN1 Bit"]
    ENABLE = 1,
}
impl From<FLEXCOMM9_A> for bool {
    #[inline(always)]
    fn from(variant: FLEXCOMM9_A) -> Self {
        variant as u8 != 0
    }
}
impl FLEXCOMM9_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FLEXCOMM9_A {
        match self.bits {
            false => FLEXCOMM9_A::DISABLE,
            true => FLEXCOMM9_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == FLEXCOMM9_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == FLEXCOMM9_A::ENABLE
    }
}
#[doc = "Field `FLEXCOMM9` writer - FLEXCOMM 9 peripheral interrupt wake-up."]
pub type FLEXCOMM9_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, STARTEN1_SET_SPEC, FLEXCOMM9_A, O>;
impl<'a, const O: u8> FLEXCOMM9_W<'a, O> {
    #[doc = "No Effect"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(FLEXCOMM9_A::DISABLE)
    }
    #[doc = "Sets the STARTEN1 Bit"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(FLEXCOMM9_A::ENABLE)
    }
}
#[doc = "Field `FLEXCOMM10` reader - FLEXCOMM 10 peripheral interrupt wake-up."]
pub type FLEXCOMM10_R = crate::BitReader<FLEXCOMM10_A>;
#[doc = "FLEXCOMM 10 peripheral interrupt wake-up.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FLEXCOMM10_A {
    #[doc = "0: No Effect"]
    DISABLE = 0,
    #[doc = "1: Sets the STARTEN1 Bit"]
    ENABLE = 1,
}
impl From<FLEXCOMM10_A> for bool {
    #[inline(always)]
    fn from(variant: FLEXCOMM10_A) -> Self {
        variant as u8 != 0
    }
}
impl FLEXCOMM10_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FLEXCOMM10_A {
        match self.bits {
            false => FLEXCOMM10_A::DISABLE,
            true => FLEXCOMM10_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == FLEXCOMM10_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == FLEXCOMM10_A::ENABLE
    }
}
#[doc = "Field `FLEXCOMM10` writer - FLEXCOMM 10 peripheral interrupt wake-up."]
pub type FLEXCOMM10_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, STARTEN1_SET_SPEC, FLEXCOMM10_A, O>;
impl<'a, const O: u8> FLEXCOMM10_W<'a, O> {
    #[doc = "No Effect"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(FLEXCOMM10_A::DISABLE)
    }
    #[doc = "Sets the STARTEN1 Bit"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(FLEXCOMM10_A::ENABLE)
    }
}
#[doc = "Field `FLEXCOMM11` reader - FLEXCOMM 11 peripheral interrupt wake-up."]
pub type FLEXCOMM11_R = crate::BitReader<FLEXCOMM11_A>;
#[doc = "FLEXCOMM 11 peripheral interrupt wake-up.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FLEXCOMM11_A {
    #[doc = "0: No Effect"]
    DISABLE = 0,
    #[doc = "1: Sets the STARTEN1 Bit"]
    ENABLE = 1,
}
impl From<FLEXCOMM11_A> for bool {
    #[inline(always)]
    fn from(variant: FLEXCOMM11_A) -> Self {
        variant as u8 != 0
    }
}
impl FLEXCOMM11_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FLEXCOMM11_A {
        match self.bits {
            false => FLEXCOMM11_A::DISABLE,
            true => FLEXCOMM11_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == FLEXCOMM11_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == FLEXCOMM11_A::ENABLE
    }
}
#[doc = "Field `FLEXCOMM11` writer - FLEXCOMM 11 peripheral interrupt wake-up."]
pub type FLEXCOMM11_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, STARTEN1_SET_SPEC, FLEXCOMM11_A, O>;
impl<'a, const O: u8> FLEXCOMM11_W<'a, O> {
    #[doc = "No Effect"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(FLEXCOMM11_A::DISABLE)
    }
    #[doc = "Sets the STARTEN1 Bit"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(FLEXCOMM11_A::ENABLE)
    }
}
impl R {
    #[doc = "Bit 0 - RTC wake-up."]
    #[inline(always)]
    pub fn rtc_lite0_wakeup(&self) -> RTC_LITE0_WAKEUP_R {
        RTC_LITE0_WAKEUP_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - DSP wake-up"]
    #[inline(always)]
    pub fn dsp_tie_expstate1(&self) -> DSP_TIE_EXPSTATE1_R {
        DSP_TIE_EXPSTATE1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Message Unit wake-up."]
    #[inline(always)]
    pub fn mu(&self) -> MU_R {
        MU_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Message Unit wake-up."]
    #[inline(always)]
    pub fn gpio_int0_irq4(&self) -> GPIO_INT0_IRQ4_R {
        GPIO_INT0_IRQ4_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - GPIO pin interrupt 5 wake-up."]
    #[inline(always)]
    pub fn gpio_int0_irq5(&self) -> GPIO_INT0_IRQ5_R {
        GPIO_INT0_IRQ5_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - GPIO pin interrupt 6 wake-up."]
    #[inline(always)]
    pub fn gpio_int0_irq6(&self) -> GPIO_INT0_IRQ6_R {
        GPIO_INT0_IRQ6_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - GPIO pin interrupt 7 wake-up."]
    #[inline(always)]
    pub fn gpio_int0_irq7(&self) -> GPIO_INT0_IRQ7_R {
        GPIO_INT0_IRQ7_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - CTIMER 2 wake-up"]
    #[inline(always)]
    pub fn ct32bit2(&self) -> CT32BIT2_R {
        CT32BIT2_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - CTIMER 4 wake-up"]
    #[inline(always)]
    pub fn ct32bit4(&self) -> CT32BIT4_R {
        CT32BIT4_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - OS Event Timer wake-up."]
    #[inline(always)]
    pub fn os_event_timer_wu(&self) -> OS_EVENT_TIMER_WU_R {
        OS_EVENT_TIMER_WU_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Quad/octal SPI wake-up."]
    #[inline(always)]
    pub fn flexspi(&self) -> FLEXSPI_R {
        FLEXSPI_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - FLEXCOMM6 wake-up."]
    #[inline(always)]
    pub fn flexcomm6(&self) -> FLEXCOMM6_R {
        FLEXCOMM6_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - FLEXCOMM7 wake-up."]
    #[inline(always)]
    pub fn flexcomm7(&self) -> FLEXCOMM7_R {
        FLEXCOMM7_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - SDIO0 wake-up."]
    #[inline(always)]
    pub fn sdio0(&self) -> SDIO0_R {
        SDIO0_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - SDIO01 wake-up."]
    #[inline(always)]
    pub fn sdio1(&self) -> SDIO1_R {
        SDIO1_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Secure GPIO interrupt A wake-up."]
    #[inline(always)]
    pub fn sgpio_inta(&self) -> SGPIO_INTA_R {
        SGPIO_INTA_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Secure GPIO interrupt B wake-up."]
    #[inline(always)]
    pub fn sgpio_intb(&self) -> SGPIO_INTB_R {
        SGPIO_INTB_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 19 - USB activity wake-up."]
    #[inline(always)]
    pub fn usb0_needclk(&self) -> USB0_NEEDCLK_R {
        USB0_NEEDCLK_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 21 - USB PHY DCD interrupt wake-up"]
    #[inline(always)]
    pub fn usb_phydcd(&self) -> USB_PHYDCD_R {
        USB_PHYDCD_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - DMA controller 1 wake-up."]
    #[inline(always)]
    pub fn dmac1(&self) -> DMAC1_R {
        DMAC1_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - PUF wake-up."]
    #[inline(always)]
    pub fn puf(&self) -> PUF_R {
        PUF_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - POWERQUAD co-processor wake-up."]
    #[inline(always)]
    pub fn powerquad(&self) -> POWERQUAD_R {
        POWERQUAD_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - CASPER co-processor wake-up."]
    #[inline(always)]
    pub fn casper(&self) -> CASPER_R {
        CASPER_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Wake-up from on-chip PMC or off-chip PMIC."]
    #[inline(always)]
    pub fn pmic(&self) -> PMIC_R {
        PMIC_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Hash-AES wake-up."]
    #[inline(always)]
    pub fn sha(&self) -> SHA_R {
        SHA_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - FLEXCOMM 8 peripheral interrupt wake-up."]
    #[inline(always)]
    pub fn flexcomm8(&self) -> FLEXCOMM8_R {
        FLEXCOMM8_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - FLEXCOMM 9 peripheral interrupt wake-up."]
    #[inline(always)]
    pub fn flexcomm9(&self) -> FLEXCOMM9_R {
        FLEXCOMM9_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - FLEXCOMM 10 peripheral interrupt wake-up."]
    #[inline(always)]
    pub fn flexcomm10(&self) -> FLEXCOMM10_R {
        FLEXCOMM10_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - FLEXCOMM 11 peripheral interrupt wake-up."]
    #[inline(always)]
    pub fn flexcomm11(&self) -> FLEXCOMM11_R {
        FLEXCOMM11_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - RTC wake-up."]
    #[inline(always)]
    #[must_use]
    pub fn rtc_lite0_wakeup(&mut self) -> RTC_LITE0_WAKEUP_W<0> {
        RTC_LITE0_WAKEUP_W::new(self)
    }
    #[doc = "Bit 1 - DSP wake-up"]
    #[inline(always)]
    #[must_use]
    pub fn dsp_tie_expstate1(&mut self) -> DSP_TIE_EXPSTATE1_W<1> {
        DSP_TIE_EXPSTATE1_W::new(self)
    }
    #[doc = "Bit 2 - Message Unit wake-up."]
    #[inline(always)]
    #[must_use]
    pub fn mu(&mut self) -> MU_W<2> {
        MU_W::new(self)
    }
    #[doc = "Bit 3 - Message Unit wake-up."]
    #[inline(always)]
    #[must_use]
    pub fn gpio_int0_irq4(&mut self) -> GPIO_INT0_IRQ4_W<3> {
        GPIO_INT0_IRQ4_W::new(self)
    }
    #[doc = "Bit 4 - GPIO pin interrupt 5 wake-up."]
    #[inline(always)]
    #[must_use]
    pub fn gpio_int0_irq5(&mut self) -> GPIO_INT0_IRQ5_W<4> {
        GPIO_INT0_IRQ5_W::new(self)
    }
    #[doc = "Bit 5 - GPIO pin interrupt 6 wake-up."]
    #[inline(always)]
    #[must_use]
    pub fn gpio_int0_irq6(&mut self) -> GPIO_INT0_IRQ6_W<5> {
        GPIO_INT0_IRQ6_W::new(self)
    }
    #[doc = "Bit 6 - GPIO pin interrupt 7 wake-up."]
    #[inline(always)]
    #[must_use]
    pub fn gpio_int0_irq7(&mut self) -> GPIO_INT0_IRQ7_W<6> {
        GPIO_INT0_IRQ7_W::new(self)
    }
    #[doc = "Bit 7 - CTIMER 2 wake-up"]
    #[inline(always)]
    #[must_use]
    pub fn ct32bit2(&mut self) -> CT32BIT2_W<7> {
        CT32BIT2_W::new(self)
    }
    #[doc = "Bit 8 - CTIMER 4 wake-up"]
    #[inline(always)]
    #[must_use]
    pub fn ct32bit4(&mut self) -> CT32BIT4_W<8> {
        CT32BIT4_W::new(self)
    }
    #[doc = "Bit 9 - OS Event Timer wake-up."]
    #[inline(always)]
    #[must_use]
    pub fn os_event_timer_wu(&mut self) -> OS_EVENT_TIMER_WU_W<9> {
        OS_EVENT_TIMER_WU_W::new(self)
    }
    #[doc = "Bit 10 - Quad/octal SPI wake-up."]
    #[inline(always)]
    #[must_use]
    pub fn flexspi(&mut self) -> FLEXSPI_W<10> {
        FLEXSPI_W::new(self)
    }
    #[doc = "Bit 11 - FLEXCOMM6 wake-up."]
    #[inline(always)]
    #[must_use]
    pub fn flexcomm6(&mut self) -> FLEXCOMM6_W<11> {
        FLEXCOMM6_W::new(self)
    }
    #[doc = "Bit 12 - FLEXCOMM7 wake-up."]
    #[inline(always)]
    #[must_use]
    pub fn flexcomm7(&mut self) -> FLEXCOMM7_W<12> {
        FLEXCOMM7_W::new(self)
    }
    #[doc = "Bit 13 - SDIO0 wake-up."]
    #[inline(always)]
    #[must_use]
    pub fn sdio0(&mut self) -> SDIO0_W<13> {
        SDIO0_W::new(self)
    }
    #[doc = "Bit 14 - SDIO01 wake-up."]
    #[inline(always)]
    #[must_use]
    pub fn sdio1(&mut self) -> SDIO1_W<14> {
        SDIO1_W::new(self)
    }
    #[doc = "Bit 15 - Secure GPIO interrupt A wake-up."]
    #[inline(always)]
    #[must_use]
    pub fn sgpio_inta(&mut self) -> SGPIO_INTA_W<15> {
        SGPIO_INTA_W::new(self)
    }
    #[doc = "Bit 16 - Secure GPIO interrupt B wake-up."]
    #[inline(always)]
    #[must_use]
    pub fn sgpio_intb(&mut self) -> SGPIO_INTB_W<16> {
        SGPIO_INTB_W::new(self)
    }
    #[doc = "Bit 19 - USB activity wake-up."]
    #[inline(always)]
    #[must_use]
    pub fn usb0_needclk(&mut self) -> USB0_NEEDCLK_W<19> {
        USB0_NEEDCLK_W::new(self)
    }
    #[doc = "Bit 21 - USB PHY DCD interrupt wake-up"]
    #[inline(always)]
    #[must_use]
    pub fn usb_phydcd(&mut self) -> USB_PHYDCD_W<21> {
        USB_PHYDCD_W::new(self)
    }
    #[doc = "Bit 22 - DMA controller 1 wake-up."]
    #[inline(always)]
    #[must_use]
    pub fn dmac1(&mut self) -> DMAC1_W<22> {
        DMAC1_W::new(self)
    }
    #[doc = "Bit 23 - PUF wake-up."]
    #[inline(always)]
    #[must_use]
    pub fn puf(&mut self) -> PUF_W<23> {
        PUF_W::new(self)
    }
    #[doc = "Bit 24 - POWERQUAD co-processor wake-up."]
    #[inline(always)]
    #[must_use]
    pub fn powerquad(&mut self) -> POWERQUAD_W<24> {
        POWERQUAD_W::new(self)
    }
    #[doc = "Bit 25 - CASPER co-processor wake-up."]
    #[inline(always)]
    #[must_use]
    pub fn casper(&mut self) -> CASPER_W<25> {
        CASPER_W::new(self)
    }
    #[doc = "Bit 26 - Wake-up from on-chip PMC or off-chip PMIC."]
    #[inline(always)]
    #[must_use]
    pub fn pmic(&mut self) -> PMIC_W<26> {
        PMIC_W::new(self)
    }
    #[doc = "Bit 27 - Hash-AES wake-up."]
    #[inline(always)]
    #[must_use]
    pub fn sha(&mut self) -> SHA_W<27> {
        SHA_W::new(self)
    }
    #[doc = "Bit 28 - FLEXCOMM 8 peripheral interrupt wake-up."]
    #[inline(always)]
    #[must_use]
    pub fn flexcomm8(&mut self) -> FLEXCOMM8_W<28> {
        FLEXCOMM8_W::new(self)
    }
    #[doc = "Bit 29 - FLEXCOMM 9 peripheral interrupt wake-up."]
    #[inline(always)]
    #[must_use]
    pub fn flexcomm9(&mut self) -> FLEXCOMM9_W<29> {
        FLEXCOMM9_W::new(self)
    }
    #[doc = "Bit 30 - FLEXCOMM 10 peripheral interrupt wake-up."]
    #[inline(always)]
    #[must_use]
    pub fn flexcomm10(&mut self) -> FLEXCOMM10_W<30> {
        FLEXCOMM10_W::new(self)
    }
    #[doc = "Bit 31 - FLEXCOMM 11 peripheral interrupt wake-up."]
    #[inline(always)]
    #[must_use]
    pub fn flexcomm11(&mut self) -> FLEXCOMM11_W<31> {
        FLEXCOMM11_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Start Enable 1 Set\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [starten1_set](index.html) module"]
pub struct STARTEN1_SET_SPEC;
impl crate::RegisterSpec for STARTEN1_SET_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [starten1_set::R](R) reader structure"]
impl crate::Readable for STARTEN1_SET_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [starten1_set::W](W) writer structure"]
impl crate::Writable for STARTEN1_SET_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0x0fc9_fffd;
}
#[doc = "`reset()` method sets STARTEN1_SET to value 0"]
impl crate::Resettable for STARTEN1_SET_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
