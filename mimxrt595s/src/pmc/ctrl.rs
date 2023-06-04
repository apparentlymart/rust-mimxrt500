#[doc = "Register `CTRL` reader"]
pub struct R(crate::R<CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CTRL` writer"]
pub struct W(crate::W<CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CTRL_SPEC>;
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
impl From<crate::W<CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Apply updated PMC PDRUNCFG bits\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum APPLYCFG_AW {
    #[doc = "0: Always reads 0. Write 0 has no effect"]
    DISABLE = 0,
    #[doc = "1: Write 1 = initiate update sequencing of PMC state machines"]
    ENABLE = 1,
}
impl From<APPLYCFG_AW> for bool {
    #[inline(always)]
    fn from(variant: APPLYCFG_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `APPLYCFG` writer - Apply updated PMC PDRUNCFG bits"]
pub type APPLYCFG_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, APPLYCFG_AW, O>;
impl<'a, const O: u8> APPLYCFG_W<'a, O> {
    #[doc = "Always reads 0. Write 0 has no effect"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(APPLYCFG_AW::DISABLE)
    }
    #[doc = "Write 1 = initiate update sequencing of PMC state machines"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(APPLYCFG_AW::ENABLE)
    }
}
#[doc = "Field `CLKDIVEN` reader - Internal clock divider enable"]
pub type CLKDIVEN_R = crate::BitReader<CLKDIVEN_A>;
#[doc = "Internal clock divider enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CLKDIVEN_A {
    #[doc = "0: 16MHz clock selected"]
    DISABLE = 0,
    #[doc = "1: 4MHz clock selected"]
    ENABLE = 1,
}
impl From<CLKDIVEN_A> for bool {
    #[inline(always)]
    fn from(variant: CLKDIVEN_A) -> Self {
        variant as u8 != 0
    }
}
impl CLKDIVEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CLKDIVEN_A {
        match self.bits {
            false => CLKDIVEN_A::DISABLE,
            true => CLKDIVEN_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == CLKDIVEN_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == CLKDIVEN_A::ENABLE
    }
}
#[doc = "Field `CLKDIVEN` writer - Internal clock divider enable"]
pub type CLKDIVEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, CLKDIVEN_A, O>;
impl<'a, const O: u8> CLKDIVEN_W<'a, O> {
    #[doc = "16MHz clock selected"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(CLKDIVEN_A::DISABLE)
    }
    #[doc = "4MHz clock selected"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(CLKDIVEN_A::ENABLE)
    }
}
#[doc = "Field `BUFEN` reader - Enable analog buffer for references or ATX2"]
pub type BUFEN_R = crate::BitReader<BUFEN_A>;
#[doc = "Enable analog buffer for references or ATX2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BUFEN_A {
    #[doc = "0: Disabled"]
    DISABLE = 0,
    #[doc = "1: Enabled"]
    ENABLE = 1,
}
impl From<BUFEN_A> for bool {
    #[inline(always)]
    fn from(variant: BUFEN_A) -> Self {
        variant as u8 != 0
    }
}
impl BUFEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BUFEN_A {
        match self.bits {
            false => BUFEN_A::DISABLE,
            true => BUFEN_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == BUFEN_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == BUFEN_A::ENABLE
    }
}
#[doc = "Field `BUFEN` writer - Enable analog buffer for references or ATX2"]
pub type BUFEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, BUFEN_A, O>;
impl<'a, const O: u8> BUFEN_W<'a, O> {
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(BUFEN_A::DISABLE)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(BUFEN_A::ENABLE)
    }
}
#[doc = "Field `OTPSWREN` reader - OTP Switch RBB enable"]
pub type OTPSWREN_R = crate::BitReader<bool>;
#[doc = "Field `OTPSWREN` writer - OTP Switch RBB enable"]
pub type OTPSWREN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, O>;
#[doc = "Field `LVDCOREIE` reader - vddcore Low-Voltage Detector Interrupt Enable"]
pub type LVDCOREIE_R = crate::BitReader<LVDCOREIE_A>;
#[doc = "vddcore Low-Voltage Detector Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LVDCOREIE_A {
    #[doc = "0: vddcore LVD interrupt disabled"]
    DISABLE = 0,
    #[doc = "1: vddcore LVD causes interrupt and wakeup from deep sleep."]
    ENABLE = 1,
}
impl From<LVDCOREIE_A> for bool {
    #[inline(always)]
    fn from(variant: LVDCOREIE_A) -> Self {
        variant as u8 != 0
    }
}
impl LVDCOREIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LVDCOREIE_A {
        match self.bits {
            false => LVDCOREIE_A::DISABLE,
            true => LVDCOREIE_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == LVDCOREIE_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == LVDCOREIE_A::ENABLE
    }
}
#[doc = "Field `LVDCOREIE` writer - vddcore Low-Voltage Detector Interrupt Enable"]
pub type LVDCOREIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, LVDCOREIE_A, O>;
impl<'a, const O: u8> LVDCOREIE_W<'a, O> {
    #[doc = "vddcore LVD interrupt disabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(LVDCOREIE_A::DISABLE)
    }
    #[doc = "vddcore LVD causes interrupt and wakeup from deep sleep."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(LVDCOREIE_A::ENABLE)
    }
}
#[doc = "Field `LVDCORERE` reader - vddcore Low-Voltage Detector Reset Enable"]
pub type LVDCORERE_R = crate::BitReader<LVDCORERE_A>;
#[doc = "vddcore Low-Voltage Detector Reset Enable\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LVDCORERE_A {
    #[doc = "0: vddcore LVD reset disabled"]
    DISABLE = 0,
    #[doc = "1: vddcore LVD causes reset"]
    ENABLE = 1,
}
impl From<LVDCORERE_A> for bool {
    #[inline(always)]
    fn from(variant: LVDCORERE_A) -> Self {
        variant as u8 != 0
    }
}
impl LVDCORERE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LVDCORERE_A {
        match self.bits {
            false => LVDCORERE_A::DISABLE,
            true => LVDCORERE_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == LVDCORERE_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == LVDCORERE_A::ENABLE
    }
}
#[doc = "Field `LVDCORERE` writer - vddcore Low-Voltage Detector Reset Enable"]
pub type LVDCORERE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, LVDCORERE_A, O>;
impl<'a, const O: u8> LVDCORERE_W<'a, O> {
    #[doc = "vddcore LVD reset disabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(LVDCORERE_A::DISABLE)
    }
    #[doc = "vddcore LVD causes reset"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(LVDCORERE_A::ENABLE)
    }
}
#[doc = "Field `HVDCOREIE` reader - vddcore High-Voltage Detector Interrupt Enable"]
pub type HVDCOREIE_R = crate::BitReader<HVDCOREIE_A>;
#[doc = "vddcore High-Voltage Detector Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HVDCOREIE_A {
    #[doc = "0: vddcore HVD interrupt disabled"]
    DISABLE = 0,
    #[doc = "1: vddcore HVD causes interrupt and wakeup from deep sleep."]
    ENABLE = 1,
}
impl From<HVDCOREIE_A> for bool {
    #[inline(always)]
    fn from(variant: HVDCOREIE_A) -> Self {
        variant as u8 != 0
    }
}
impl HVDCOREIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HVDCOREIE_A {
        match self.bits {
            false => HVDCOREIE_A::DISABLE,
            true => HVDCOREIE_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == HVDCOREIE_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == HVDCOREIE_A::ENABLE
    }
}
#[doc = "Field `HVDCOREIE` writer - vddcore High-Voltage Detector Interrupt Enable"]
pub type HVDCOREIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, HVDCOREIE_A, O>;
impl<'a, const O: u8> HVDCOREIE_W<'a, O> {
    #[doc = "vddcore HVD interrupt disabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(HVDCOREIE_A::DISABLE)
    }
    #[doc = "vddcore HVD causes interrupt and wakeup from deep sleep."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(HVDCOREIE_A::ENABLE)
    }
}
#[doc = "Field `HVDCORERE` reader - vddcore High-Voltage Detector Reset Enable"]
pub type HVDCORERE_R = crate::BitReader<HVDCORERE_A>;
#[doc = "vddcore High-Voltage Detector Reset Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HVDCORERE_A {
    #[doc = "0: vddcore HVD reset disabled"]
    DISABLE = 0,
    #[doc = "1: vddcore HVD causes reset"]
    ENABLE = 1,
}
impl From<HVDCORERE_A> for bool {
    #[inline(always)]
    fn from(variant: HVDCORERE_A) -> Self {
        variant as u8 != 0
    }
}
impl HVDCORERE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HVDCORERE_A {
        match self.bits {
            false => HVDCORERE_A::DISABLE,
            true => HVDCORERE_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == HVDCORERE_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == HVDCORERE_A::ENABLE
    }
}
#[doc = "Field `HVDCORERE` writer - vddcore High-Voltage Detector Reset Enable"]
pub type HVDCORERE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, HVDCORERE_A, O>;
impl<'a, const O: u8> HVDCORERE_W<'a, O> {
    #[doc = "vddcore HVD reset disabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(HVDCORERE_A::DISABLE)
    }
    #[doc = "vddcore HVD causes reset"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(HVDCORERE_A::ENABLE)
    }
}
#[doc = "Field `HVD1V8IE` reader - vdd1v8 High-Voltage Detector Interrupt Enable"]
pub type HVD1V8IE_R = crate::BitReader<HVD1V8IE_A>;
#[doc = "vdd1v8 High-Voltage Detector Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HVD1V8IE_A {
    #[doc = "0: vdd1v8 HVD interrupt disabled"]
    DISABLE = 0,
    #[doc = "1: vdd1v8 HVD causes interrupt and wakeup from deep sleep or deep power down mode."]
    ENABLE = 1,
}
impl From<HVD1V8IE_A> for bool {
    #[inline(always)]
    fn from(variant: HVD1V8IE_A) -> Self {
        variant as u8 != 0
    }
}
impl HVD1V8IE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HVD1V8IE_A {
        match self.bits {
            false => HVD1V8IE_A::DISABLE,
            true => HVD1V8IE_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == HVD1V8IE_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == HVD1V8IE_A::ENABLE
    }
}
#[doc = "Field `HVD1V8IE` writer - vdd1v8 High-Voltage Detector Interrupt Enable"]
pub type HVD1V8IE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, HVD1V8IE_A, O>;
impl<'a, const O: u8> HVD1V8IE_W<'a, O> {
    #[doc = "vdd1v8 HVD interrupt disabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(HVD1V8IE_A::DISABLE)
    }
    #[doc = "vdd1v8 HVD causes interrupt and wakeup from deep sleep or deep power down mode."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(HVD1V8IE_A::ENABLE)
    }
}
#[doc = "Field `HVD1V8RE` reader - vdd1v8 High-Voltage Detector Reset Enable"]
pub type HVD1V8RE_R = crate::BitReader<HVD1V8RE_A>;
#[doc = "vdd1v8 High-Voltage Detector Reset Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HVD1V8RE_A {
    #[doc = "0: vdd1v8 HVD reset disabled"]
    DISABLE = 0,
    #[doc = "1: vdd1v8 HVD causes reset"]
    ENABLE = 1,
}
impl From<HVD1V8RE_A> for bool {
    #[inline(always)]
    fn from(variant: HVD1V8RE_A) -> Self {
        variant as u8 != 0
    }
}
impl HVD1V8RE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HVD1V8RE_A {
        match self.bits {
            false => HVD1V8RE_A::DISABLE,
            true => HVD1V8RE_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == HVD1V8RE_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == HVD1V8RE_A::ENABLE
    }
}
#[doc = "Field `HVD1V8RE` writer - vdd1v8 High-Voltage Detector Reset Enable"]
pub type HVD1V8RE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, HVD1V8RE_A, O>;
impl<'a, const O: u8> HVD1V8RE_W<'a, O> {
    #[doc = "vdd1v8 HVD reset disabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(HVD1V8RE_A::DISABLE)
    }
    #[doc = "vdd1v8 HVD causes reset"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(HVD1V8RE_A::ENABLE)
    }
}
#[doc = "Field `AUTOWKEN` reader - PMC automatic wakeup enable and interrupt enable"]
pub type AUTOWKEN_R = crate::BitReader<AUTOWKEN_A>;
#[doc = "PMC automatic wakeup enable and interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AUTOWKEN_A {
    #[doc = "0: Auto wakeup interrupt and counter disabled"]
    DISABLE = 0,
    #[doc = "1: Auto wakeup interrupt generated when PMC sequencer finishes and AUTOWAKE counter = 0 after entering deep sleep mode (but not deep powerdown mode). Interrupt will wake up the M33."]
    ENABLE = 1,
}
impl From<AUTOWKEN_A> for bool {
    #[inline(always)]
    fn from(variant: AUTOWKEN_A) -> Self {
        variant as u8 != 0
    }
}
impl AUTOWKEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AUTOWKEN_A {
        match self.bits {
            false => AUTOWKEN_A::DISABLE,
            true => AUTOWKEN_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == AUTOWKEN_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == AUTOWKEN_A::ENABLE
    }
}
#[doc = "Field `AUTOWKEN` writer - PMC automatic wakeup enable and interrupt enable"]
pub type AUTOWKEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, AUTOWKEN_A, O>;
impl<'a, const O: u8> AUTOWKEN_W<'a, O> {
    #[doc = "Auto wakeup interrupt and counter disabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(AUTOWKEN_A::DISABLE)
    }
    #[doc = "Auto wakeup interrupt generated when PMC sequencer finishes and AUTOWAKE counter = 0 after entering deep sleep mode (but not deep powerdown mode). Interrupt will wake up the M33."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(AUTOWKEN_A::ENABLE)
    }
}
#[doc = "Field `INTRPADEN` reader - PMIC_IRQ_N enable"]
pub type INTRPADEN_R = crate::BitReader<INTRPADEN_A>;
#[doc = "PMIC_IRQ_N enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum INTRPADEN_A {
    #[doc = "0: Interrupt pad low has no effect"]
    DISABLE = 0,
    #[doc = "1: Interrupt pad low triggers an interrupt and deep sleep wakeup or deep powerdown wakeup event."]
    ENABLE = 1,
}
impl From<INTRPADEN_A> for bool {
    #[inline(always)]
    fn from(variant: INTRPADEN_A) -> Self {
        variant as u8 != 0
    }
}
impl INTRPADEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INTRPADEN_A {
        match self.bits {
            false => INTRPADEN_A::DISABLE,
            true => INTRPADEN_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == INTRPADEN_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == INTRPADEN_A::ENABLE
    }
}
#[doc = "Field `INTRPADEN` writer - PMIC_IRQ_N enable"]
pub type INTRPADEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, INTRPADEN_A, O>;
impl<'a, const O: u8> INTRPADEN_W<'a, O> {
    #[doc = "Interrupt pad low has no effect"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(INTRPADEN_A::DISABLE)
    }
    #[doc = "Interrupt pad low triggers an interrupt and deep sleep wakeup or deep powerdown wakeup event."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(INTRPADEN_A::ENABLE)
    }
}
impl R {
    #[doc = "Bit 1 - Internal clock divider enable"]
    #[inline(always)]
    pub fn clkdiven(&self) -> CLKDIVEN_R {
        CLKDIVEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 4 - Enable analog buffer for references or ATX2"]
    #[inline(always)]
    pub fn bufen(&self) -> BUFEN_R {
        BUFEN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 18 - OTP Switch RBB enable"]
    #[inline(always)]
    pub fn otpswren(&self) -> OTPSWREN_R {
        OTPSWREN_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 20 - vddcore Low-Voltage Detector Interrupt Enable"]
    #[inline(always)]
    pub fn lvdcoreie(&self) -> LVDCOREIE_R {
        LVDCOREIE_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - vddcore Low-Voltage Detector Reset Enable"]
    #[inline(always)]
    pub fn lvdcorere(&self) -> LVDCORERE_R {
        LVDCORERE_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - vddcore High-Voltage Detector Interrupt Enable"]
    #[inline(always)]
    pub fn hvdcoreie(&self) -> HVDCOREIE_R {
        HVDCOREIE_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - vddcore High-Voltage Detector Reset Enable"]
    #[inline(always)]
    pub fn hvdcorere(&self) -> HVDCORERE_R {
        HVDCORERE_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - vdd1v8 High-Voltage Detector Interrupt Enable"]
    #[inline(always)]
    pub fn hvd1v8ie(&self) -> HVD1V8IE_R {
        HVD1V8IE_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - vdd1v8 High-Voltage Detector Reset Enable"]
    #[inline(always)]
    pub fn hvd1v8re(&self) -> HVD1V8RE_R {
        HVD1V8RE_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 28 - PMC automatic wakeup enable and interrupt enable"]
    #[inline(always)]
    pub fn autowken(&self) -> AUTOWKEN_R {
        AUTOWKEN_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - PMIC_IRQ_N enable"]
    #[inline(always)]
    pub fn intrpaden(&self) -> INTRPADEN_R {
        INTRPADEN_R::new(((self.bits >> 29) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Apply updated PMC PDRUNCFG bits"]
    #[inline(always)]
    #[must_use]
    pub fn applycfg(&mut self) -> APPLYCFG_W<0> {
        APPLYCFG_W::new(self)
    }
    #[doc = "Bit 1 - Internal clock divider enable"]
    #[inline(always)]
    #[must_use]
    pub fn clkdiven(&mut self) -> CLKDIVEN_W<1> {
        CLKDIVEN_W::new(self)
    }
    #[doc = "Bit 4 - Enable analog buffer for references or ATX2"]
    #[inline(always)]
    #[must_use]
    pub fn bufen(&mut self) -> BUFEN_W<4> {
        BUFEN_W::new(self)
    }
    #[doc = "Bit 18 - OTP Switch RBB enable"]
    #[inline(always)]
    #[must_use]
    pub fn otpswren(&mut self) -> OTPSWREN_W<18> {
        OTPSWREN_W::new(self)
    }
    #[doc = "Bit 20 - vddcore Low-Voltage Detector Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn lvdcoreie(&mut self) -> LVDCOREIE_W<20> {
        LVDCOREIE_W::new(self)
    }
    #[doc = "Bit 21 - vddcore Low-Voltage Detector Reset Enable"]
    #[inline(always)]
    #[must_use]
    pub fn lvdcorere(&mut self) -> LVDCORERE_W<21> {
        LVDCORERE_W::new(self)
    }
    #[doc = "Bit 22 - vddcore High-Voltage Detector Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn hvdcoreie(&mut self) -> HVDCOREIE_W<22> {
        HVDCOREIE_W::new(self)
    }
    #[doc = "Bit 23 - vddcore High-Voltage Detector Reset Enable"]
    #[inline(always)]
    #[must_use]
    pub fn hvdcorere(&mut self) -> HVDCORERE_W<23> {
        HVDCORERE_W::new(self)
    }
    #[doc = "Bit 24 - vdd1v8 High-Voltage Detector Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn hvd1v8ie(&mut self) -> HVD1V8IE_W<24> {
        HVD1V8IE_W::new(self)
    }
    #[doc = "Bit 25 - vdd1v8 High-Voltage Detector Reset Enable"]
    #[inline(always)]
    #[must_use]
    pub fn hvd1v8re(&mut self) -> HVD1V8RE_W<25> {
        HVD1V8RE_W::new(self)
    }
    #[doc = "Bit 28 - PMC automatic wakeup enable and interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn autowken(&mut self) -> AUTOWKEN_W<28> {
        AUTOWKEN_W::new(self)
    }
    #[doc = "Bit 29 - PMIC_IRQ_N enable"]
    #[inline(always)]
    #[must_use]
    pub fn intrpaden(&mut self) -> INTRPADEN_W<29> {
        INTRPADEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PMC control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctrl](index.html) module"]
pub struct CTRL_SPEC;
impl crate::RegisterSpec for CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ctrl::R](R) reader structure"]
impl crate::Readable for CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ctrl::W](W) writer structure"]
impl crate::Writable for CTRL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CTRL to value 0x0020_0000"]
impl crate::Resettable for CTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0x0020_0000;
}
