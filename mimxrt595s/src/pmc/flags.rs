#[doc = "Register `FLAGS` reader"]
pub struct R(crate::R<FLAGS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FLAGS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FLAGS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FLAGS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FLAGS` writer"]
pub struct W(crate::W<FLAGS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FLAGS_SPEC>;
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
impl From<crate::W<FLAGS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FLAGS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PORCOREF` reader - vddcore POR Flag"]
pub type PORCOREF_R = crate::BitReader<PORCOREF_A>;
#[doc = "vddcore POR Flag\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PORCOREF_A {
    #[doc = "0: vddcore POR was not tripped since the last cleared."]
    DISABLE = 0,
    #[doc = "1: POR triggered by the vddcore POR monitor. Write 1 to clear"]
    ENABLE = 1,
}
impl From<PORCOREF_A> for bool {
    #[inline(always)]
    fn from(variant: PORCOREF_A) -> Self {
        variant as u8 != 0
    }
}
impl PORCOREF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PORCOREF_A {
        match self.bits {
            false => PORCOREF_A::DISABLE,
            true => PORCOREF_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == PORCOREF_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == PORCOREF_A::ENABLE
    }
}
#[doc = "Field `PORCOREF` writer - vddcore POR Flag"]
pub type PORCOREF_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, FLAGS_SPEC, PORCOREF_A, O>;
impl<'a, const O: u8> PORCOREF_W<'a, O> {
    #[doc = "vddcore POR was not tripped since the last cleared."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(PORCOREF_A::DISABLE)
    }
    #[doc = "POR triggered by the vddcore POR monitor. Write 1 to clear"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(PORCOREF_A::ENABLE)
    }
}
#[doc = "Field `POR1V8F` reader - vdd1v8 power on reset flag"]
pub type POR1V8F_R = crate::BitReader<POR1V8F_A>;
#[doc = "vdd1v8 power on reset flag\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum POR1V8F_A {
    #[doc = "0: No vdd1v8 power on event detected since last cleared."]
    DISABLE = 0,
    #[doc = "1: vdd1v8 power on detect caused a reset or deep power down wakeup. Write 1 to clear."]
    ENABLE = 1,
}
impl From<POR1V8F_A> for bool {
    #[inline(always)]
    fn from(variant: POR1V8F_A) -> Self {
        variant as u8 != 0
    }
}
impl POR1V8F_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> POR1V8F_A {
        match self.bits {
            false => POR1V8F_A::DISABLE,
            true => POR1V8F_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == POR1V8F_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == POR1V8F_A::ENABLE
    }
}
#[doc = "Field `POR1V8F` writer - vdd1v8 power on reset flag"]
pub type POR1V8F_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, FLAGS_SPEC, POR1V8F_A, O>;
impl<'a, const O: u8> POR1V8F_W<'a, O> {
    #[doc = "No vdd1v8 power on event detected since last cleared."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(POR1V8F_A::DISABLE)
    }
    #[doc = "vdd1v8 power on detect caused a reset or deep power down wakeup. Write 1 to clear."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(POR1V8F_A::ENABLE)
    }
}
#[doc = "Field `PORAO18F` reader - VDD_AO1V8 power on reset flag"]
pub type PORAO18F_R = crate::BitReader<PORAO18F_A>;
#[doc = "VDD_AO1V8 power on reset flag\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PORAO18F_A {
    #[doc = "0: No VDD_AO1V8 power on event detected since last cleared."]
    DISABLE = 0,
    #[doc = "1: VDD_AO1V8 power on detect caused a reset. Write 1 to clear."]
    ENABLE = 1,
}
impl From<PORAO18F_A> for bool {
    #[inline(always)]
    fn from(variant: PORAO18F_A) -> Self {
        variant as u8 != 0
    }
}
impl PORAO18F_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PORAO18F_A {
        match self.bits {
            false => PORAO18F_A::DISABLE,
            true => PORAO18F_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == PORAO18F_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == PORAO18F_A::ENABLE
    }
}
#[doc = "Field `PORAO18F` writer - VDD_AO1V8 power on reset flag"]
pub type PORAO18F_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, FLAGS_SPEC, PORAO18F_A, O>;
impl<'a, const O: u8> PORAO18F_W<'a, O> {
    #[doc = "No VDD_AO1V8 power on event detected since last cleared."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(PORAO18F_A::DISABLE)
    }
    #[doc = "VDD_AO1V8 power on detect caused a reset. Write 1 to clear."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(PORAO18F_A::ENABLE)
    }
}
#[doc = "Field `LVDCOREF` reader - vddcore Low-Voltage Detector Flag"]
pub type LVDCOREF_R = crate::BitReader<LVDCOREF_A>;
#[doc = "vddcore Low-Voltage Detector Flag\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LVDCOREF_A {
    #[doc = "0: vddcore LVD has not tripped since last clear"]
    DISABLE = 0,
    #[doc = "1: vddcore LVD tripped since last time this bit was cleared. Write 1 to clear"]
    ENABLE = 1,
}
impl From<LVDCOREF_A> for bool {
    #[inline(always)]
    fn from(variant: LVDCOREF_A) -> Self {
        variant as u8 != 0
    }
}
impl LVDCOREF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LVDCOREF_A {
        match self.bits {
            false => LVDCOREF_A::DISABLE,
            true => LVDCOREF_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == LVDCOREF_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == LVDCOREF_A::ENABLE
    }
}
#[doc = "Field `LVDCOREF` writer - vddcore Low-Voltage Detector Flag"]
pub type LVDCOREF_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, FLAGS_SPEC, LVDCOREF_A, O>;
impl<'a, const O: u8> LVDCOREF_W<'a, O> {
    #[doc = "vddcore LVD has not tripped since last clear"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(LVDCOREF_A::DISABLE)
    }
    #[doc = "vddcore LVD tripped since last time this bit was cleared. Write 1 to clear"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(LVDCOREF_A::ENABLE)
    }
}
#[doc = "Field `HVDCOREF` reader - vddcore High-Voltage Detector Flag"]
pub type HVDCOREF_R = crate::BitReader<HVDCOREF_A>;
#[doc = "vddcore High-Voltage Detector Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HVDCOREF_A {
    #[doc = "0: vddcore HVD has not tripped since last clear"]
    DISABLE = 0,
    #[doc = "1: vddcore HVD tripped since last time this bit was cleared. Write 1 to clear"]
    ENABLE = 1,
}
impl From<HVDCOREF_A> for bool {
    #[inline(always)]
    fn from(variant: HVDCOREF_A) -> Self {
        variant as u8 != 0
    }
}
impl HVDCOREF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HVDCOREF_A {
        match self.bits {
            false => HVDCOREF_A::DISABLE,
            true => HVDCOREF_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == HVDCOREF_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == HVDCOREF_A::ENABLE
    }
}
#[doc = "Field `HVDCOREF` writer - vddcore High-Voltage Detector Flag"]
pub type HVDCOREF_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, FLAGS_SPEC, HVDCOREF_A, O>;
impl<'a, const O: u8> HVDCOREF_W<'a, O> {
    #[doc = "vddcore HVD has not tripped since last clear"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(HVDCOREF_A::DISABLE)
    }
    #[doc = "vddcore HVD tripped since last time this bit was cleared. Write 1 to clear"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(HVDCOREF_A::ENABLE)
    }
}
#[doc = "Field `HVD1V8F` reader - vdd1v8 High-Voltage Detector Flag"]
pub type HVD1V8F_R = crate::BitReader<HVD1V8F_A>;
#[doc = "vdd1v8 High-Voltage Detector Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HVD1V8F_A {
    #[doc = "0: vdd1v8 HVD has not tripped since last clear"]
    DISABLE = 0,
    #[doc = "1: vdd1v8 HVD tripped since last time this bit was cleared. Write 1 to clear"]
    ENABLE = 1,
}
impl From<HVD1V8F_A> for bool {
    #[inline(always)]
    fn from(variant: HVD1V8F_A) -> Self {
        variant as u8 != 0
    }
}
impl HVD1V8F_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HVD1V8F_A {
        match self.bits {
            false => HVD1V8F_A::DISABLE,
            true => HVD1V8F_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == HVD1V8F_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == HVD1V8F_A::ENABLE
    }
}
#[doc = "Field `HVD1V8F` writer - vdd1v8 High-Voltage Detector Flag"]
pub type HVD1V8F_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, FLAGS_SPEC, HVD1V8F_A, O>;
impl<'a, const O: u8> HVD1V8F_W<'a, O> {
    #[doc = "vdd1v8 HVD has not tripped since last clear"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(HVD1V8F_A::DISABLE)
    }
    #[doc = "vdd1v8 HVD tripped since last time this bit was cleared. Write 1 to clear"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(HVD1V8F_A::ENABLE)
    }
}
#[doc = "Field `RTCF` reader - RTC Wakeup from deep powerdown mode flag"]
pub type RTCF_R = crate::BitReader<RTCF_A>;
#[doc = "RTC Wakeup from deep powerdown mode flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RTCF_A {
    #[doc = "0: No RTC wakeup detected since last time flag was cleared."]
    DISABLE = 0,
    #[doc = "1: RTC wakeup caused a deep powerdown wakeup. Write 1 to clear."]
    ENABLE = 1,
}
impl From<RTCF_A> for bool {
    #[inline(always)]
    fn from(variant: RTCF_A) -> Self {
        variant as u8 != 0
    }
}
impl RTCF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RTCF_A {
        match self.bits {
            false => RTCF_A::DISABLE,
            true => RTCF_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == RTCF_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == RTCF_A::ENABLE
    }
}
#[doc = "Field `RTCF` writer - RTC Wakeup from deep powerdown mode flag"]
pub type RTCF_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, FLAGS_SPEC, RTCF_A, O>;
impl<'a, const O: u8> RTCF_W<'a, O> {
    #[doc = "No RTC wakeup detected since last time flag was cleared."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(RTCF_A::DISABLE)
    }
    #[doc = "RTC wakeup caused a deep powerdown wakeup. Write 1 to clear."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(RTCF_A::ENABLE)
    }
}
#[doc = "Field `AUTOWKF` reader - PMC Auto Wakeup Interrupt flag"]
pub type AUTOWKF_R = crate::BitReader<AUTOWKF_A>;
#[doc = "PMC Auto Wakeup Interrupt flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AUTOWKF_A {
    #[doc = "0: No PMC Auto Wakeup Interrupt detected since last time cleared."]
    DISABLE = 0,
    #[doc = "1: PMC Auto wakeup caused a deep sleep wakeup and interrupt. Write 1 to clear."]
    ENABLE = 1,
}
impl From<AUTOWKF_A> for bool {
    #[inline(always)]
    fn from(variant: AUTOWKF_A) -> Self {
        variant as u8 != 0
    }
}
impl AUTOWKF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AUTOWKF_A {
        match self.bits {
            false => AUTOWKF_A::DISABLE,
            true => AUTOWKF_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == AUTOWKF_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == AUTOWKF_A::ENABLE
    }
}
#[doc = "Field `AUTOWKF` writer - PMC Auto Wakeup Interrupt flag"]
pub type AUTOWKF_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, FLAGS_SPEC, AUTOWKF_A, O>;
impl<'a, const O: u8> AUTOWKF_W<'a, O> {
    #[doc = "No PMC Auto Wakeup Interrupt detected since last time cleared."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(AUTOWKF_A::DISABLE)
    }
    #[doc = "PMC Auto wakeup caused a deep sleep wakeup and interrupt. Write 1 to clear."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(AUTOWKF_A::ENABLE)
    }
}
#[doc = "Field `INTNPADF` reader - PMIC_IRQ_N Interrupt pin flag"]
pub type INTNPADF_R = crate::BitReader<INTNPADF_A>;
#[doc = "PMIC_IRQ_N Interrupt pin flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum INTNPADF_A {
    #[doc = "0: No interrupt detected since flag last cleared."]
    DISABLE = 0,
    #[doc = "1: Pad interrupt caused a wakeup or interrupt event since the last time this flag was cleared. Write 1 to clear."]
    ENABLE = 1,
}
impl From<INTNPADF_A> for bool {
    #[inline(always)]
    fn from(variant: INTNPADF_A) -> Self {
        variant as u8 != 0
    }
}
impl INTNPADF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INTNPADF_A {
        match self.bits {
            false => INTNPADF_A::DISABLE,
            true => INTNPADF_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == INTNPADF_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == INTNPADF_A::ENABLE
    }
}
#[doc = "Field `INTNPADF` writer - PMIC_IRQ_N Interrupt pin flag"]
pub type INTNPADF_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, FLAGS_SPEC, INTNPADF_A, O>;
impl<'a, const O: u8> INTNPADF_W<'a, O> {
    #[doc = "No interrupt detected since flag last cleared."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(INTNPADF_A::DISABLE)
    }
    #[doc = "Pad interrupt caused a wakeup or interrupt event since the last time this flag was cleared. Write 1 to clear."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(INTNPADF_A::ENABLE)
    }
}
#[doc = "Field `RESETNPADF` reader - Reset pad flag"]
pub type RESETNPADF_R = crate::BitReader<RESETNPADF_A>;
#[doc = "Reset pad flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RESETNPADF_A {
    #[doc = "0: No reset detected since last time this flag was cleared."]
    DISABLE = 0,
    #[doc = "1: Reset pad wakeup caused a wakeup or reset event since the last time this bit was cleared. Write 1 to clear."]
    ENABLE = 1,
}
impl From<RESETNPADF_A> for bool {
    #[inline(always)]
    fn from(variant: RESETNPADF_A) -> Self {
        variant as u8 != 0
    }
}
impl RESETNPADF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RESETNPADF_A {
        match self.bits {
            false => RESETNPADF_A::DISABLE,
            true => RESETNPADF_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == RESETNPADF_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == RESETNPADF_A::ENABLE
    }
}
#[doc = "Field `RESETNPADF` writer - Reset pad flag"]
pub type RESETNPADF_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, FLAGS_SPEC, RESETNPADF_A, O>;
impl<'a, const O: u8> RESETNPADF_W<'a, O> {
    #[doc = "No reset detected since last time this flag was cleared."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(RESETNPADF_A::DISABLE)
    }
    #[doc = "Reset pad wakeup caused a wakeup or reset event since the last time this bit was cleared. Write 1 to clear."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(RESETNPADF_A::ENABLE)
    }
}
#[doc = "Field `DEEPPDF` reader - Deep powerdown wakeup flag"]
pub type DEEPPDF_R = crate::BitReader<DEEPPDF_A>;
#[doc = "Deep powerdown wakeup flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DEEPPDF_A {
    #[doc = "0: No deep powerdown wakeup since last time flag was cleared."]
    DISABLE = 0,
    #[doc = "1: Deep powerdown was entered since the last time this flag was cleared. Write 1 to clear"]
    ENABLE = 1,
}
impl From<DEEPPDF_A> for bool {
    #[inline(always)]
    fn from(variant: DEEPPDF_A) -> Self {
        variant as u8 != 0
    }
}
impl DEEPPDF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DEEPPDF_A {
        match self.bits {
            false => DEEPPDF_A::DISABLE,
            true => DEEPPDF_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == DEEPPDF_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == DEEPPDF_A::ENABLE
    }
}
#[doc = "Field `DEEPPDF` writer - Deep powerdown wakeup flag"]
pub type DEEPPDF_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, FLAGS_SPEC, DEEPPDF_A, O>;
impl<'a, const O: u8> DEEPPDF_W<'a, O> {
    #[doc = "No deep powerdown wakeup since last time flag was cleared."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(DEEPPDF_A::DISABLE)
    }
    #[doc = "Deep powerdown was entered since the last time this flag was cleared. Write 1 to clear"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(DEEPPDF_A::ENABLE)
    }
}
impl R {
    #[doc = "Bit 16 - vddcore POR Flag"]
    #[inline(always)]
    pub fn porcoref(&self) -> PORCOREF_R {
        PORCOREF_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - vdd1v8 power on reset flag"]
    #[inline(always)]
    pub fn por1v8f(&self) -> POR1V8F_R {
        POR1V8F_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - VDD_AO1V8 power on reset flag"]
    #[inline(always)]
    pub fn porao18f(&self) -> PORAO18F_R {
        PORAO18F_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 20 - vddcore Low-Voltage Detector Flag"]
    #[inline(always)]
    pub fn lvdcoref(&self) -> LVDCOREF_R {
        LVDCOREF_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 22 - vddcore High-Voltage Detector Flag"]
    #[inline(always)]
    pub fn hvdcoref(&self) -> HVDCOREF_R {
        HVDCOREF_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 24 - vdd1v8 High-Voltage Detector Flag"]
    #[inline(always)]
    pub fn hvd1v8f(&self) -> HVD1V8F_R {
        HVD1V8F_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 27 - RTC Wakeup from deep powerdown mode flag"]
    #[inline(always)]
    pub fn rtcf(&self) -> RTCF_R {
        RTCF_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - PMC Auto Wakeup Interrupt flag"]
    #[inline(always)]
    pub fn autowkf(&self) -> AUTOWKF_R {
        AUTOWKF_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - PMIC_IRQ_N Interrupt pin flag"]
    #[inline(always)]
    pub fn intnpadf(&self) -> INTNPADF_R {
        INTNPADF_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Reset pad flag"]
    #[inline(always)]
    pub fn resetnpadf(&self) -> RESETNPADF_R {
        RESETNPADF_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Deep powerdown wakeup flag"]
    #[inline(always)]
    pub fn deeppdf(&self) -> DEEPPDF_R {
        DEEPPDF_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 16 - vddcore POR Flag"]
    #[inline(always)]
    #[must_use]
    pub fn porcoref(&mut self) -> PORCOREF_W<16> {
        PORCOREF_W::new(self)
    }
    #[doc = "Bit 17 - vdd1v8 power on reset flag"]
    #[inline(always)]
    #[must_use]
    pub fn por1v8f(&mut self) -> POR1V8F_W<17> {
        POR1V8F_W::new(self)
    }
    #[doc = "Bit 18 - VDD_AO1V8 power on reset flag"]
    #[inline(always)]
    #[must_use]
    pub fn porao18f(&mut self) -> PORAO18F_W<18> {
        PORAO18F_W::new(self)
    }
    #[doc = "Bit 20 - vddcore Low-Voltage Detector Flag"]
    #[inline(always)]
    #[must_use]
    pub fn lvdcoref(&mut self) -> LVDCOREF_W<20> {
        LVDCOREF_W::new(self)
    }
    #[doc = "Bit 22 - vddcore High-Voltage Detector Flag"]
    #[inline(always)]
    #[must_use]
    pub fn hvdcoref(&mut self) -> HVDCOREF_W<22> {
        HVDCOREF_W::new(self)
    }
    #[doc = "Bit 24 - vdd1v8 High-Voltage Detector Flag"]
    #[inline(always)]
    #[must_use]
    pub fn hvd1v8f(&mut self) -> HVD1V8F_W<24> {
        HVD1V8F_W::new(self)
    }
    #[doc = "Bit 27 - RTC Wakeup from deep powerdown mode flag"]
    #[inline(always)]
    #[must_use]
    pub fn rtcf(&mut self) -> RTCF_W<27> {
        RTCF_W::new(self)
    }
    #[doc = "Bit 28 - PMC Auto Wakeup Interrupt flag"]
    #[inline(always)]
    #[must_use]
    pub fn autowkf(&mut self) -> AUTOWKF_W<28> {
        AUTOWKF_W::new(self)
    }
    #[doc = "Bit 29 - PMIC_IRQ_N Interrupt pin flag"]
    #[inline(always)]
    #[must_use]
    pub fn intnpadf(&mut self) -> INTNPADF_W<29> {
        INTNPADF_W::new(self)
    }
    #[doc = "Bit 30 - Reset pad flag"]
    #[inline(always)]
    #[must_use]
    pub fn resetnpadf(&mut self) -> RESETNPADF_W<30> {
        RESETNPADF_W::new(self)
    }
    #[doc = "Bit 31 - Deep powerdown wakeup flag"]
    #[inline(always)]
    #[must_use]
    pub fn deeppdf(&mut self) -> DEEPPDF_W<31> {
        DEEPPDF_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Wakeup, Interrupt, Reset Flags\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [flags](index.html) module"]
pub struct FLAGS_SPEC;
impl crate::RegisterSpec for FLAGS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [flags::R](R) reader structure"]
impl crate::Readable for FLAGS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [flags::W](W) writer structure"]
impl crate::Writable for FLAGS_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0xf957_0000;
}
#[doc = "`reset()` method sets FLAGS to value 0x0017_0000"]
impl crate::Resettable for FLAGS_SPEC {
    const RESET_VALUE: Self::Ux = 0x0017_0000;
}
