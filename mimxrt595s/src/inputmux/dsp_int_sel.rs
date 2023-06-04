#[doc = "Register `DSP_INT_SEL[%s]` reader"]
pub struct R(crate::R<DSP_INT_SEL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DSP_INT_SEL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DSP_INT_SEL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DSP_INT_SEL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DSP_INT_SEL[%s]` writer"]
pub struct W(crate::W<DSP_INT_SEL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DSP_INT_SEL_SPEC>;
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
impl From<crate::W<DSP_INT_SEL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DSP_INT_SEL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DSP_INT_SEL` reader - Fusion DSP Input(n) Selection."]
pub type DSP_INT_SEL_R = crate::FieldReader<u8, DSP_INT_SEL_A>;
#[doc = "Fusion DSP Input(n) Selection.\n\nValue on reset: 63"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DSP_INT_SEL_A {
    #[doc = "0: FLEXCOMM0_IRQ"]
    FLEXCOMM0 = 0,
    #[doc = "1: FLEXCOMM1_IRQ"]
    FLEXCOMM1 = 1,
    #[doc = "2: FLEXCOMM2_IRQ"]
    FLEXCOMM2 = 2,
    #[doc = "3: FLEXCOMM3_IRQ"]
    FLEXCOMM3 = 3,
    #[doc = "4: FLEXCOMM4_IRQ"]
    FLEXCOMM4 = 4,
    #[doc = "5: FLEXCOMM5_IRQ"]
    FLEXCOMM5 = 5,
    #[doc = "6: FLEXCOMM6_IRQ"]
    FLEXCOMM6 = 6,
    #[doc = "7: FLEXCOMM7_IRQ"]
    FLEXCOMM7 = 7,
    #[doc = "8: FLEXCOMM14_IRQ"]
    FLEXCOMM14 = 8,
    #[doc = "9: FLEXCOMM16_IRQ"]
    FLEXCOMM16 = 9,
    #[doc = "10: GPIO_INT0_IRQ0"]
    GPIO_INT0_IRQ0 = 10,
    #[doc = "11: GPIO_INT0_IRQ1"]
    GPIO_INT0_IRQ1 = 11,
    #[doc = "12: GPIO_INT0_IRQ2"]
    GPIO_INT0_IRQ2 = 12,
    #[doc = "13: GPIO_INT0_IRQ3"]
    GPIO_INT0_IRQ3 = 13,
    #[doc = "14: GPIO_INT0_IRQ4"]
    GPIO_INT0_IRQ4 = 14,
    #[doc = "15: GPIO_INT0_IRQ5"]
    GPIO_INT0_IRQ5 = 15,
    #[doc = "16: GPIO_INT0_IRQ6"]
    GPIO_INT0_IRQ6 = 16,
    #[doc = "17: GPIO_INT0_IRQ7"]
    GPIO_INT0_IRQ7 = 17,
    #[doc = "18: NSHSGPIO_INT0_IRQ0"]
    NSHSGPIO_INT0 = 18,
    #[doc = "19: NSHSGPIO_INT1_IRQ1"]
    NSHSGPIO_INT1 = 19,
    #[doc = "20: WDT1"]
    WDT1 = 20,
    #[doc = "21: DMAC0_IRQ"]
    DMAC0 = 21,
    #[doc = "22: DMAC1_IRQ"]
    DMAC1 = 22,
    #[doc = "23: MU_B_IRQ"]
    MU = 23,
    #[doc = "24: UTICK0_IRQ"]
    UTICK0 = 24,
    #[doc = "25: MRT0_IRQ"]
    MRT0 = 25,
    #[doc = "26: OS_EVENT_TIMER or OS_EVENT_WAKEUP"]
    OS_EVENT_TIMER = 26,
    #[doc = "27: CTIMER0"]
    CTIMER0 = 27,
    #[doc = "28: CTIMER1"]
    CTIMER1 = 28,
    #[doc = "29: CTIMER2"]
    CTIMER2 = 29,
    #[doc = "30: CTIMER3"]
    CTIMER3 = 30,
    #[doc = "31: CTIMER4"]
    CTIMER4 = 31,
    #[doc = "32: RTC_LITE0_ALARM or RTC_LITE0_WAKEUP"]
    RTC_LITE0_ALARM = 32,
    #[doc = "33: I3C0"]
    I3C0 = 33,
    #[doc = "34: I3C1"]
    I3C1 = 34,
    #[doc = "35: DMIC0"]
    DMIC0 = 35,
    #[doc = "36: HWVAD"]
    HWVAD = 36,
    #[doc = "37: LCDIF_IRQ"]
    LCDIF_IRQ = 37,
    #[doc = "38: GPU_IRQ"]
    GPU_IRQ = 38,
    #[doc = "39: SMARTDMA_IRQ"]
    SMARTDMA_IRQ = 39,
    #[doc = "40: FLEXIO_IRQ"]
    FLEXIO_IRQ = 40,
}
impl From<DSP_INT_SEL_A> for u8 {
    #[inline(always)]
    fn from(variant: DSP_INT_SEL_A) -> Self {
        variant as _
    }
}
impl DSP_INT_SEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<DSP_INT_SEL_A> {
        match self.bits {
            0 => Some(DSP_INT_SEL_A::FLEXCOMM0),
            1 => Some(DSP_INT_SEL_A::FLEXCOMM1),
            2 => Some(DSP_INT_SEL_A::FLEXCOMM2),
            3 => Some(DSP_INT_SEL_A::FLEXCOMM3),
            4 => Some(DSP_INT_SEL_A::FLEXCOMM4),
            5 => Some(DSP_INT_SEL_A::FLEXCOMM5),
            6 => Some(DSP_INT_SEL_A::FLEXCOMM6),
            7 => Some(DSP_INT_SEL_A::FLEXCOMM7),
            8 => Some(DSP_INT_SEL_A::FLEXCOMM14),
            9 => Some(DSP_INT_SEL_A::FLEXCOMM16),
            10 => Some(DSP_INT_SEL_A::GPIO_INT0_IRQ0),
            11 => Some(DSP_INT_SEL_A::GPIO_INT0_IRQ1),
            12 => Some(DSP_INT_SEL_A::GPIO_INT0_IRQ2),
            13 => Some(DSP_INT_SEL_A::GPIO_INT0_IRQ3),
            14 => Some(DSP_INT_SEL_A::GPIO_INT0_IRQ4),
            15 => Some(DSP_INT_SEL_A::GPIO_INT0_IRQ5),
            16 => Some(DSP_INT_SEL_A::GPIO_INT0_IRQ6),
            17 => Some(DSP_INT_SEL_A::GPIO_INT0_IRQ7),
            18 => Some(DSP_INT_SEL_A::NSHSGPIO_INT0),
            19 => Some(DSP_INT_SEL_A::NSHSGPIO_INT1),
            20 => Some(DSP_INT_SEL_A::WDT1),
            21 => Some(DSP_INT_SEL_A::DMAC0),
            22 => Some(DSP_INT_SEL_A::DMAC1),
            23 => Some(DSP_INT_SEL_A::MU),
            24 => Some(DSP_INT_SEL_A::UTICK0),
            25 => Some(DSP_INT_SEL_A::MRT0),
            26 => Some(DSP_INT_SEL_A::OS_EVENT_TIMER),
            27 => Some(DSP_INT_SEL_A::CTIMER0),
            28 => Some(DSP_INT_SEL_A::CTIMER1),
            29 => Some(DSP_INT_SEL_A::CTIMER2),
            30 => Some(DSP_INT_SEL_A::CTIMER3),
            31 => Some(DSP_INT_SEL_A::CTIMER4),
            32 => Some(DSP_INT_SEL_A::RTC_LITE0_ALARM),
            33 => Some(DSP_INT_SEL_A::I3C0),
            34 => Some(DSP_INT_SEL_A::I3C1),
            35 => Some(DSP_INT_SEL_A::DMIC0),
            36 => Some(DSP_INT_SEL_A::HWVAD),
            37 => Some(DSP_INT_SEL_A::LCDIF_IRQ),
            38 => Some(DSP_INT_SEL_A::GPU_IRQ),
            39 => Some(DSP_INT_SEL_A::SMARTDMA_IRQ),
            40 => Some(DSP_INT_SEL_A::FLEXIO_IRQ),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `FLEXCOMM0`"]
    #[inline(always)]
    pub fn is_flexcomm0(&self) -> bool {
        *self == DSP_INT_SEL_A::FLEXCOMM0
    }
    #[doc = "Checks if the value of the field is `FLEXCOMM1`"]
    #[inline(always)]
    pub fn is_flexcomm1(&self) -> bool {
        *self == DSP_INT_SEL_A::FLEXCOMM1
    }
    #[doc = "Checks if the value of the field is `FLEXCOMM2`"]
    #[inline(always)]
    pub fn is_flexcomm2(&self) -> bool {
        *self == DSP_INT_SEL_A::FLEXCOMM2
    }
    #[doc = "Checks if the value of the field is `FLEXCOMM3`"]
    #[inline(always)]
    pub fn is_flexcomm3(&self) -> bool {
        *self == DSP_INT_SEL_A::FLEXCOMM3
    }
    #[doc = "Checks if the value of the field is `FLEXCOMM4`"]
    #[inline(always)]
    pub fn is_flexcomm4(&self) -> bool {
        *self == DSP_INT_SEL_A::FLEXCOMM4
    }
    #[doc = "Checks if the value of the field is `FLEXCOMM5`"]
    #[inline(always)]
    pub fn is_flexcomm5(&self) -> bool {
        *self == DSP_INT_SEL_A::FLEXCOMM5
    }
    #[doc = "Checks if the value of the field is `FLEXCOMM6`"]
    #[inline(always)]
    pub fn is_flexcomm6(&self) -> bool {
        *self == DSP_INT_SEL_A::FLEXCOMM6
    }
    #[doc = "Checks if the value of the field is `FLEXCOMM7`"]
    #[inline(always)]
    pub fn is_flexcomm7(&self) -> bool {
        *self == DSP_INT_SEL_A::FLEXCOMM7
    }
    #[doc = "Checks if the value of the field is `FLEXCOMM14`"]
    #[inline(always)]
    pub fn is_flexcomm14(&self) -> bool {
        *self == DSP_INT_SEL_A::FLEXCOMM14
    }
    #[doc = "Checks if the value of the field is `FLEXCOMM16`"]
    #[inline(always)]
    pub fn is_flexcomm16(&self) -> bool {
        *self == DSP_INT_SEL_A::FLEXCOMM16
    }
    #[doc = "Checks if the value of the field is `GPIO_INT0_IRQ0`"]
    #[inline(always)]
    pub fn is_gpio_int0_irq0(&self) -> bool {
        *self == DSP_INT_SEL_A::GPIO_INT0_IRQ0
    }
    #[doc = "Checks if the value of the field is `GPIO_INT0_IRQ1`"]
    #[inline(always)]
    pub fn is_gpio_int0_irq1(&self) -> bool {
        *self == DSP_INT_SEL_A::GPIO_INT0_IRQ1
    }
    #[doc = "Checks if the value of the field is `GPIO_INT0_IRQ2`"]
    #[inline(always)]
    pub fn is_gpio_int0_irq2(&self) -> bool {
        *self == DSP_INT_SEL_A::GPIO_INT0_IRQ2
    }
    #[doc = "Checks if the value of the field is `GPIO_INT0_IRQ3`"]
    #[inline(always)]
    pub fn is_gpio_int0_irq3(&self) -> bool {
        *self == DSP_INT_SEL_A::GPIO_INT0_IRQ3
    }
    #[doc = "Checks if the value of the field is `GPIO_INT0_IRQ4`"]
    #[inline(always)]
    pub fn is_gpio_int0_irq4(&self) -> bool {
        *self == DSP_INT_SEL_A::GPIO_INT0_IRQ4
    }
    #[doc = "Checks if the value of the field is `GPIO_INT0_IRQ5`"]
    #[inline(always)]
    pub fn is_gpio_int0_irq5(&self) -> bool {
        *self == DSP_INT_SEL_A::GPIO_INT0_IRQ5
    }
    #[doc = "Checks if the value of the field is `GPIO_INT0_IRQ6`"]
    #[inline(always)]
    pub fn is_gpio_int0_irq6(&self) -> bool {
        *self == DSP_INT_SEL_A::GPIO_INT0_IRQ6
    }
    #[doc = "Checks if the value of the field is `GPIO_INT0_IRQ7`"]
    #[inline(always)]
    pub fn is_gpio_int0_irq7(&self) -> bool {
        *self == DSP_INT_SEL_A::GPIO_INT0_IRQ7
    }
    #[doc = "Checks if the value of the field is `NSHSGPIO_INT0`"]
    #[inline(always)]
    pub fn is_nshsgpio_int0(&self) -> bool {
        *self == DSP_INT_SEL_A::NSHSGPIO_INT0
    }
    #[doc = "Checks if the value of the field is `NSHSGPIO_INT1`"]
    #[inline(always)]
    pub fn is_nshsgpio_int1(&self) -> bool {
        *self == DSP_INT_SEL_A::NSHSGPIO_INT1
    }
    #[doc = "Checks if the value of the field is `WDT1`"]
    #[inline(always)]
    pub fn is_wdt1(&self) -> bool {
        *self == DSP_INT_SEL_A::WDT1
    }
    #[doc = "Checks if the value of the field is `DMAC0`"]
    #[inline(always)]
    pub fn is_dmac0(&self) -> bool {
        *self == DSP_INT_SEL_A::DMAC0
    }
    #[doc = "Checks if the value of the field is `DMAC1`"]
    #[inline(always)]
    pub fn is_dmac1(&self) -> bool {
        *self == DSP_INT_SEL_A::DMAC1
    }
    #[doc = "Checks if the value of the field is `MU`"]
    #[inline(always)]
    pub fn is_mu(&self) -> bool {
        *self == DSP_INT_SEL_A::MU
    }
    #[doc = "Checks if the value of the field is `UTICK0`"]
    #[inline(always)]
    pub fn is_utick0(&self) -> bool {
        *self == DSP_INT_SEL_A::UTICK0
    }
    #[doc = "Checks if the value of the field is `MRT0`"]
    #[inline(always)]
    pub fn is_mrt0(&self) -> bool {
        *self == DSP_INT_SEL_A::MRT0
    }
    #[doc = "Checks if the value of the field is `OS_EVENT_TIMER`"]
    #[inline(always)]
    pub fn is_os_event_timer(&self) -> bool {
        *self == DSP_INT_SEL_A::OS_EVENT_TIMER
    }
    #[doc = "Checks if the value of the field is `CTIMER0`"]
    #[inline(always)]
    pub fn is_ctimer0(&self) -> bool {
        *self == DSP_INT_SEL_A::CTIMER0
    }
    #[doc = "Checks if the value of the field is `CTIMER1`"]
    #[inline(always)]
    pub fn is_ctimer1(&self) -> bool {
        *self == DSP_INT_SEL_A::CTIMER1
    }
    #[doc = "Checks if the value of the field is `CTIMER2`"]
    #[inline(always)]
    pub fn is_ctimer2(&self) -> bool {
        *self == DSP_INT_SEL_A::CTIMER2
    }
    #[doc = "Checks if the value of the field is `CTIMER3`"]
    #[inline(always)]
    pub fn is_ctimer3(&self) -> bool {
        *self == DSP_INT_SEL_A::CTIMER3
    }
    #[doc = "Checks if the value of the field is `CTIMER4`"]
    #[inline(always)]
    pub fn is_ctimer4(&self) -> bool {
        *self == DSP_INT_SEL_A::CTIMER4
    }
    #[doc = "Checks if the value of the field is `RTC_LITE0_ALARM`"]
    #[inline(always)]
    pub fn is_rtc_lite0_alarm(&self) -> bool {
        *self == DSP_INT_SEL_A::RTC_LITE0_ALARM
    }
    #[doc = "Checks if the value of the field is `I3C0`"]
    #[inline(always)]
    pub fn is_i3c0(&self) -> bool {
        *self == DSP_INT_SEL_A::I3C0
    }
    #[doc = "Checks if the value of the field is `I3C1`"]
    #[inline(always)]
    pub fn is_i3c1(&self) -> bool {
        *self == DSP_INT_SEL_A::I3C1
    }
    #[doc = "Checks if the value of the field is `DMIC0`"]
    #[inline(always)]
    pub fn is_dmic0(&self) -> bool {
        *self == DSP_INT_SEL_A::DMIC0
    }
    #[doc = "Checks if the value of the field is `HWVAD`"]
    #[inline(always)]
    pub fn is_hwvad(&self) -> bool {
        *self == DSP_INT_SEL_A::HWVAD
    }
    #[doc = "Checks if the value of the field is `LCDIF_IRQ`"]
    #[inline(always)]
    pub fn is_lcdif_irq(&self) -> bool {
        *self == DSP_INT_SEL_A::LCDIF_IRQ
    }
    #[doc = "Checks if the value of the field is `GPU_IRQ`"]
    #[inline(always)]
    pub fn is_gpu_irq(&self) -> bool {
        *self == DSP_INT_SEL_A::GPU_IRQ
    }
    #[doc = "Checks if the value of the field is `SMARTDMA_IRQ`"]
    #[inline(always)]
    pub fn is_smartdma_irq(&self) -> bool {
        *self == DSP_INT_SEL_A::SMARTDMA_IRQ
    }
    #[doc = "Checks if the value of the field is `FLEXIO_IRQ`"]
    #[inline(always)]
    pub fn is_flexio_irq(&self) -> bool {
        *self == DSP_INT_SEL_A::FLEXIO_IRQ
    }
}
#[doc = "Field `DSP_INT_SEL` writer - Fusion DSP Input(n) Selection."]
pub type DSP_INT_SEL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, DSP_INT_SEL_SPEC, u8, DSP_INT_SEL_A, 6, O>;
impl<'a, const O: u8> DSP_INT_SEL_W<'a, O> {
    #[doc = "FLEXCOMM0_IRQ"]
    #[inline(always)]
    pub fn flexcomm0(self) -> &'a mut W {
        self.variant(DSP_INT_SEL_A::FLEXCOMM0)
    }
    #[doc = "FLEXCOMM1_IRQ"]
    #[inline(always)]
    pub fn flexcomm1(self) -> &'a mut W {
        self.variant(DSP_INT_SEL_A::FLEXCOMM1)
    }
    #[doc = "FLEXCOMM2_IRQ"]
    #[inline(always)]
    pub fn flexcomm2(self) -> &'a mut W {
        self.variant(DSP_INT_SEL_A::FLEXCOMM2)
    }
    #[doc = "FLEXCOMM3_IRQ"]
    #[inline(always)]
    pub fn flexcomm3(self) -> &'a mut W {
        self.variant(DSP_INT_SEL_A::FLEXCOMM3)
    }
    #[doc = "FLEXCOMM4_IRQ"]
    #[inline(always)]
    pub fn flexcomm4(self) -> &'a mut W {
        self.variant(DSP_INT_SEL_A::FLEXCOMM4)
    }
    #[doc = "FLEXCOMM5_IRQ"]
    #[inline(always)]
    pub fn flexcomm5(self) -> &'a mut W {
        self.variant(DSP_INT_SEL_A::FLEXCOMM5)
    }
    #[doc = "FLEXCOMM6_IRQ"]
    #[inline(always)]
    pub fn flexcomm6(self) -> &'a mut W {
        self.variant(DSP_INT_SEL_A::FLEXCOMM6)
    }
    #[doc = "FLEXCOMM7_IRQ"]
    #[inline(always)]
    pub fn flexcomm7(self) -> &'a mut W {
        self.variant(DSP_INT_SEL_A::FLEXCOMM7)
    }
    #[doc = "FLEXCOMM14_IRQ"]
    #[inline(always)]
    pub fn flexcomm14(self) -> &'a mut W {
        self.variant(DSP_INT_SEL_A::FLEXCOMM14)
    }
    #[doc = "FLEXCOMM16_IRQ"]
    #[inline(always)]
    pub fn flexcomm16(self) -> &'a mut W {
        self.variant(DSP_INT_SEL_A::FLEXCOMM16)
    }
    #[doc = "GPIO_INT0_IRQ0"]
    #[inline(always)]
    pub fn gpio_int0_irq0(self) -> &'a mut W {
        self.variant(DSP_INT_SEL_A::GPIO_INT0_IRQ0)
    }
    #[doc = "GPIO_INT0_IRQ1"]
    #[inline(always)]
    pub fn gpio_int0_irq1(self) -> &'a mut W {
        self.variant(DSP_INT_SEL_A::GPIO_INT0_IRQ1)
    }
    #[doc = "GPIO_INT0_IRQ2"]
    #[inline(always)]
    pub fn gpio_int0_irq2(self) -> &'a mut W {
        self.variant(DSP_INT_SEL_A::GPIO_INT0_IRQ2)
    }
    #[doc = "GPIO_INT0_IRQ3"]
    #[inline(always)]
    pub fn gpio_int0_irq3(self) -> &'a mut W {
        self.variant(DSP_INT_SEL_A::GPIO_INT0_IRQ3)
    }
    #[doc = "GPIO_INT0_IRQ4"]
    #[inline(always)]
    pub fn gpio_int0_irq4(self) -> &'a mut W {
        self.variant(DSP_INT_SEL_A::GPIO_INT0_IRQ4)
    }
    #[doc = "GPIO_INT0_IRQ5"]
    #[inline(always)]
    pub fn gpio_int0_irq5(self) -> &'a mut W {
        self.variant(DSP_INT_SEL_A::GPIO_INT0_IRQ5)
    }
    #[doc = "GPIO_INT0_IRQ6"]
    #[inline(always)]
    pub fn gpio_int0_irq6(self) -> &'a mut W {
        self.variant(DSP_INT_SEL_A::GPIO_INT0_IRQ6)
    }
    #[doc = "GPIO_INT0_IRQ7"]
    #[inline(always)]
    pub fn gpio_int0_irq7(self) -> &'a mut W {
        self.variant(DSP_INT_SEL_A::GPIO_INT0_IRQ7)
    }
    #[doc = "NSHSGPIO_INT0_IRQ0"]
    #[inline(always)]
    pub fn nshsgpio_int0(self) -> &'a mut W {
        self.variant(DSP_INT_SEL_A::NSHSGPIO_INT0)
    }
    #[doc = "NSHSGPIO_INT1_IRQ1"]
    #[inline(always)]
    pub fn nshsgpio_int1(self) -> &'a mut W {
        self.variant(DSP_INT_SEL_A::NSHSGPIO_INT1)
    }
    #[doc = "WDT1"]
    #[inline(always)]
    pub fn wdt1(self) -> &'a mut W {
        self.variant(DSP_INT_SEL_A::WDT1)
    }
    #[doc = "DMAC0_IRQ"]
    #[inline(always)]
    pub fn dmac0(self) -> &'a mut W {
        self.variant(DSP_INT_SEL_A::DMAC0)
    }
    #[doc = "DMAC1_IRQ"]
    #[inline(always)]
    pub fn dmac1(self) -> &'a mut W {
        self.variant(DSP_INT_SEL_A::DMAC1)
    }
    #[doc = "MU_B_IRQ"]
    #[inline(always)]
    pub fn mu(self) -> &'a mut W {
        self.variant(DSP_INT_SEL_A::MU)
    }
    #[doc = "UTICK0_IRQ"]
    #[inline(always)]
    pub fn utick0(self) -> &'a mut W {
        self.variant(DSP_INT_SEL_A::UTICK0)
    }
    #[doc = "MRT0_IRQ"]
    #[inline(always)]
    pub fn mrt0(self) -> &'a mut W {
        self.variant(DSP_INT_SEL_A::MRT0)
    }
    #[doc = "OS_EVENT_TIMER or OS_EVENT_WAKEUP"]
    #[inline(always)]
    pub fn os_event_timer(self) -> &'a mut W {
        self.variant(DSP_INT_SEL_A::OS_EVENT_TIMER)
    }
    #[doc = "CTIMER0"]
    #[inline(always)]
    pub fn ctimer0(self) -> &'a mut W {
        self.variant(DSP_INT_SEL_A::CTIMER0)
    }
    #[doc = "CTIMER1"]
    #[inline(always)]
    pub fn ctimer1(self) -> &'a mut W {
        self.variant(DSP_INT_SEL_A::CTIMER1)
    }
    #[doc = "CTIMER2"]
    #[inline(always)]
    pub fn ctimer2(self) -> &'a mut W {
        self.variant(DSP_INT_SEL_A::CTIMER2)
    }
    #[doc = "CTIMER3"]
    #[inline(always)]
    pub fn ctimer3(self) -> &'a mut W {
        self.variant(DSP_INT_SEL_A::CTIMER3)
    }
    #[doc = "CTIMER4"]
    #[inline(always)]
    pub fn ctimer4(self) -> &'a mut W {
        self.variant(DSP_INT_SEL_A::CTIMER4)
    }
    #[doc = "RTC_LITE0_ALARM or RTC_LITE0_WAKEUP"]
    #[inline(always)]
    pub fn rtc_lite0_alarm(self) -> &'a mut W {
        self.variant(DSP_INT_SEL_A::RTC_LITE0_ALARM)
    }
    #[doc = "I3C0"]
    #[inline(always)]
    pub fn i3c0(self) -> &'a mut W {
        self.variant(DSP_INT_SEL_A::I3C0)
    }
    #[doc = "I3C1"]
    #[inline(always)]
    pub fn i3c1(self) -> &'a mut W {
        self.variant(DSP_INT_SEL_A::I3C1)
    }
    #[doc = "DMIC0"]
    #[inline(always)]
    pub fn dmic0(self) -> &'a mut W {
        self.variant(DSP_INT_SEL_A::DMIC0)
    }
    #[doc = "HWVAD"]
    #[inline(always)]
    pub fn hwvad(self) -> &'a mut W {
        self.variant(DSP_INT_SEL_A::HWVAD)
    }
    #[doc = "LCDIF_IRQ"]
    #[inline(always)]
    pub fn lcdif_irq(self) -> &'a mut W {
        self.variant(DSP_INT_SEL_A::LCDIF_IRQ)
    }
    #[doc = "GPU_IRQ"]
    #[inline(always)]
    pub fn gpu_irq(self) -> &'a mut W {
        self.variant(DSP_INT_SEL_A::GPU_IRQ)
    }
    #[doc = "SMARTDMA_IRQ"]
    #[inline(always)]
    pub fn smartdma_irq(self) -> &'a mut W {
        self.variant(DSP_INT_SEL_A::SMARTDMA_IRQ)
    }
    #[doc = "FLEXIO_IRQ"]
    #[inline(always)]
    pub fn flexio_irq(self) -> &'a mut W {
        self.variant(DSP_INT_SEL_A::FLEXIO_IRQ)
    }
}
impl R {
    #[doc = "Bits 0:5 - Fusion DSP Input(n) Selection."]
    #[inline(always)]
    pub fn dsp_int_sel(&self) -> DSP_INT_SEL_R {
        DSP_INT_SEL_R::new((self.bits & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - Fusion DSP Input(n) Selection."]
    #[inline(always)]
    #[must_use]
    pub fn dsp_int_sel(&mut self) -> DSP_INT_SEL_W<0> {
        DSP_INT_SEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Fusion DSP Interrupt Input Multiplexer\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dsp_int_sel](index.html) module"]
pub struct DSP_INT_SEL_SPEC;
impl crate::RegisterSpec for DSP_INT_SEL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dsp_int_sel::R](R) reader structure"]
impl crate::Readable for DSP_INT_SEL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dsp_int_sel::W](W) writer structure"]
impl crate::Writable for DSP_INT_SEL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DSP_INT_SEL[%s]
to value 0x3f"]
impl crate::Resettable for DSP_INT_SEL_SPEC {
    const RESET_VALUE: Self::Ux = 0x3f;
}
