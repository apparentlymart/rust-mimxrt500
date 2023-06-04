#[doc = "Register `SYSRSTSTAT` reader"]
pub struct R(crate::R<SYSRSTSTAT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SYSRSTSTAT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SYSRSTSTAT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SYSRSTSTAT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `VDD_POR` reader - VDD Power-On Reset (POR) was detected"]
pub type VDD_POR_R = crate::BitReader<VDD_POR_A>;
#[doc = "VDD Power-On Reset (POR) was detected\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VDD_POR_A {
    #[doc = "0: No VDD POR event is detected"]
    VDD_POR_EVENT_IS_NOT_DETECTED = 0,
    #[doc = "1: VDD POR event was detected"]
    VDD_POR_EVENT_WAS_DETECTED = 1,
}
impl From<VDD_POR_A> for bool {
    #[inline(always)]
    fn from(variant: VDD_POR_A) -> Self {
        variant as u8 != 0
    }
}
impl VDD_POR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> VDD_POR_A {
        match self.bits {
            false => VDD_POR_A::VDD_POR_EVENT_IS_NOT_DETECTED,
            true => VDD_POR_A::VDD_POR_EVENT_WAS_DETECTED,
        }
    }
    #[doc = "Checks if the value of the field is `VDD_POR_EVENT_IS_NOT_DETECTED`"]
    #[inline(always)]
    pub fn is_vdd_por_event_is_not_detected(&self) -> bool {
        *self == VDD_POR_A::VDD_POR_EVENT_IS_NOT_DETECTED
    }
    #[doc = "Checks if the value of the field is `VDD_POR_EVENT_WAS_DETECTED`"]
    #[inline(always)]
    pub fn is_vdd_por_event_was_detected(&self) -> bool {
        *self == VDD_POR_A::VDD_POR_EVENT_WAS_DETECTED
    }
}
#[doc = "Field `PAD_RESET` reader - RESETN pin reset was detected"]
pub type PAD_RESET_R = crate::BitReader<PAD_RESET_A>;
#[doc = "RESETN pin reset was detected\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PAD_RESET_A {
    #[doc = "0: No RESETN pin event is detected"]
    PAD_RESET_IS_NOT_DETECTED = 0,
    #[doc = "1: RESETN pin reset event was detected"]
    PAD_RESET_WAS_DETECTED = 1,
}
impl From<PAD_RESET_A> for bool {
    #[inline(always)]
    fn from(variant: PAD_RESET_A) -> Self {
        variant as u8 != 0
    }
}
impl PAD_RESET_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PAD_RESET_A {
        match self.bits {
            false => PAD_RESET_A::PAD_RESET_IS_NOT_DETECTED,
            true => PAD_RESET_A::PAD_RESET_WAS_DETECTED,
        }
    }
    #[doc = "Checks if the value of the field is `PAD_RESET_IS_NOT_DETECTED`"]
    #[inline(always)]
    pub fn is_pad_reset_is_not_detected(&self) -> bool {
        *self == PAD_RESET_A::PAD_RESET_IS_NOT_DETECTED
    }
    #[doc = "Checks if the value of the field is `PAD_RESET_WAS_DETECTED`"]
    #[inline(always)]
    pub fn is_pad_reset_was_detected(&self) -> bool {
        *self == PAD_RESET_A::PAD_RESET_WAS_DETECTED
    }
}
#[doc = "Field `ARM_RESET` reader - ARM reset was detected"]
pub type ARM_RESET_R = crate::BitReader<ARM_RESET_A>;
#[doc = "ARM reset was detected\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ARM_RESET_A {
    #[doc = "0: No ARM reset event is detected"]
    ARM_RESET_IS_NOT_DETECTED = 0,
    #[doc = "1: ARM reset was detected"]
    ARM_RESET_WAS_DETECTED = 1,
}
impl From<ARM_RESET_A> for bool {
    #[inline(always)]
    fn from(variant: ARM_RESET_A) -> Self {
        variant as u8 != 0
    }
}
impl ARM_RESET_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ARM_RESET_A {
        match self.bits {
            false => ARM_RESET_A::ARM_RESET_IS_NOT_DETECTED,
            true => ARM_RESET_A::ARM_RESET_WAS_DETECTED,
        }
    }
    #[doc = "Checks if the value of the field is `ARM_RESET_IS_NOT_DETECTED`"]
    #[inline(always)]
    pub fn is_arm_reset_is_not_detected(&self) -> bool {
        *self == ARM_RESET_A::ARM_RESET_IS_NOT_DETECTED
    }
    #[doc = "Checks if the value of the field is `ARM_RESET_WAS_DETECTED`"]
    #[inline(always)]
    pub fn is_arm_reset_was_detected(&self) -> bool {
        *self == ARM_RESET_A::ARM_RESET_WAS_DETECTED
    }
}
#[doc = "Field `WDT0_RESET` reader - WDT0 reset was detected"]
pub type WDT0_RESET_R = crate::BitReader<WDT0_RESET_A>;
#[doc = "WDT0 reset was detected\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WDT0_RESET_A {
    #[doc = "0: No WDT0 reset event is detected"]
    WDT0_RESET_IS_NOT_DETECTED = 0,
    #[doc = "1: WDT0 reset was detected"]
    WDT0_RESET_WAS_DETECTED = 1,
}
impl From<WDT0_RESET_A> for bool {
    #[inline(always)]
    fn from(variant: WDT0_RESET_A) -> Self {
        variant as u8 != 0
    }
}
impl WDT0_RESET_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WDT0_RESET_A {
        match self.bits {
            false => WDT0_RESET_A::WDT0_RESET_IS_NOT_DETECTED,
            true => WDT0_RESET_A::WDT0_RESET_WAS_DETECTED,
        }
    }
    #[doc = "Checks if the value of the field is `WDT0_RESET_IS_NOT_DETECTED`"]
    #[inline(always)]
    pub fn is_wdt0_reset_is_not_detected(&self) -> bool {
        *self == WDT0_RESET_A::WDT0_RESET_IS_NOT_DETECTED
    }
    #[doc = "Checks if the value of the field is `WDT0_RESET_WAS_DETECTED`"]
    #[inline(always)]
    pub fn is_wdt0_reset_was_detected(&self) -> bool {
        *self == WDT0_RESET_A::WDT0_RESET_WAS_DETECTED
    }
}
#[doc = "Field `WDT1_RESET` reader - WDT1 reset was detected"]
pub type WDT1_RESET_R = crate::BitReader<WDT1_RESET_A>;
#[doc = "WDT1 reset was detected\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WDT1_RESET_A {
    #[doc = "0: No WDT1 reset event is detected"]
    WDT1_RESET_IS_NOT_DETECTED = 0,
    #[doc = "1: WDT1 reset was detected"]
    WDT1_RESET_WAS_DETECTED = 1,
}
impl From<WDT1_RESET_A> for bool {
    #[inline(always)]
    fn from(variant: WDT1_RESET_A) -> Self {
        variant as u8 != 0
    }
}
impl WDT1_RESET_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WDT1_RESET_A {
        match self.bits {
            false => WDT1_RESET_A::WDT1_RESET_IS_NOT_DETECTED,
            true => WDT1_RESET_A::WDT1_RESET_WAS_DETECTED,
        }
    }
    #[doc = "Checks if the value of the field is `WDT1_RESET_IS_NOT_DETECTED`"]
    #[inline(always)]
    pub fn is_wdt1_reset_is_not_detected(&self) -> bool {
        *self == WDT1_RESET_A::WDT1_RESET_IS_NOT_DETECTED
    }
    #[doc = "Checks if the value of the field is `WDT1_RESET_WAS_DETECTED`"]
    #[inline(always)]
    pub fn is_wdt1_reset_was_detected(&self) -> bool {
        *self == WDT1_RESET_A::WDT1_RESET_WAS_DETECTED
    }
}
impl R {
    #[doc = "Bit 0 - VDD Power-On Reset (POR) was detected"]
    #[inline(always)]
    pub fn vdd_por(&self) -> VDD_POR_R {
        VDD_POR_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 4 - RESETN pin reset was detected"]
    #[inline(always)]
    pub fn pad_reset(&self) -> PAD_RESET_R {
        PAD_RESET_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - ARM reset was detected"]
    #[inline(always)]
    pub fn arm_reset(&self) -> ARM_RESET_R {
        ARM_RESET_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - WDT0 reset was detected"]
    #[inline(always)]
    pub fn wdt0_reset(&self) -> WDT0_RESET_R {
        WDT0_RESET_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - WDT1 reset was detected"]
    #[inline(always)]
    pub fn wdt1_reset(&self) -> WDT1_RESET_R {
        WDT1_RESET_R::new(((self.bits >> 7) & 1) != 0)
    }
}
#[doc = "System Reset Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sysrststat](index.html) module"]
pub struct SYSRSTSTAT_SPEC;
impl crate::RegisterSpec for SYSRSTSTAT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sysrststat::R](R) reader structure"]
impl crate::Readable for SYSRSTSTAT_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets SYSRSTSTAT to value 0x01"]
impl crate::Resettable for SYSRSTSTAT_SPEC {
    const RESET_VALUE: Self::Ux = 0x01;
}
