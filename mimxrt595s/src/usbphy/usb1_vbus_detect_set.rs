#[doc = "Register `USB1_VBUS_DETECT_SET` reader"]
pub struct R(crate::R<USB1_VBUS_DETECT_SET_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<USB1_VBUS_DETECT_SET_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<USB1_VBUS_DETECT_SET_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<USB1_VBUS_DETECT_SET_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `USB1_VBUS_DETECT_SET` writer"]
pub struct W(crate::W<USB1_VBUS_DETECT_SET_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<USB1_VBUS_DETECT_SET_SPEC>;
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
impl From<crate::W<USB1_VBUS_DETECT_SET_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<USB1_VBUS_DETECT_SET_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `VBUSVALID_THRESH` reader - VBUS comparator threshold"]
pub type VBUSVALID_THRESH_R = crate::FieldReader<u8, VBUSVALID_THRESH_A>;
#[doc = "VBUS comparator threshold\n\nValue on reset: 4"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum VBUSVALID_THRESH_A {
    #[doc = "0: No effect"]
    DISABLE = 0,
    #[doc = "1: Sets the corresponding bit"]
    ENABLE = 1,
}
impl From<VBUSVALID_THRESH_A> for u8 {
    #[inline(always)]
    fn from(variant: VBUSVALID_THRESH_A) -> Self {
        variant as _
    }
}
impl VBUSVALID_THRESH_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<VBUSVALID_THRESH_A> {
        match self.bits {
            0 => Some(VBUSVALID_THRESH_A::DISABLE),
            1 => Some(VBUSVALID_THRESH_A::ENABLE),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == VBUSVALID_THRESH_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == VBUSVALID_THRESH_A::ENABLE
    }
}
#[doc = "Field `VBUSVALID_THRESH` writer - VBUS comparator threshold"]
pub type VBUSVALID_THRESH_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, USB1_VBUS_DETECT_SET_SPEC, u8, VBUSVALID_THRESH_A, 3, O>;
impl<'a, const O: u8> VBUSVALID_THRESH_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(VBUSVALID_THRESH_A::DISABLE)
    }
    #[doc = "Sets the corresponding bit"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(VBUSVALID_THRESH_A::ENABLE)
    }
}
#[doc = "Field `VBUS_OVERRIDE_EN` reader - VBUS detect signal override. This bit is used when EXT_VBUS_OVERRIDE_EN = 1'b0."]
pub type VBUS_OVERRIDE_EN_R = crate::BitReader<VBUS_OVERRIDE_EN_A>;
#[doc = "VBUS detect signal override. This bit is used when EXT_VBUS_OVERRIDE_EN = 1'b0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VBUS_OVERRIDE_EN_A {
    #[doc = "0: No effect"]
    DISABLE = 0,
    #[doc = "1: Sets the corresponding bit"]
    ENABLE = 1,
}
impl From<VBUS_OVERRIDE_EN_A> for bool {
    #[inline(always)]
    fn from(variant: VBUS_OVERRIDE_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl VBUS_OVERRIDE_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> VBUS_OVERRIDE_EN_A {
        match self.bits {
            false => VBUS_OVERRIDE_EN_A::DISABLE,
            true => VBUS_OVERRIDE_EN_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == VBUS_OVERRIDE_EN_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == VBUS_OVERRIDE_EN_A::ENABLE
    }
}
#[doc = "Field `VBUS_OVERRIDE_EN` writer - VBUS detect signal override. This bit is used when EXT_VBUS_OVERRIDE_EN = 1'b0."]
pub type VBUS_OVERRIDE_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, USB1_VBUS_DETECT_SET_SPEC, VBUS_OVERRIDE_EN_A, O>;
impl<'a, const O: u8> VBUS_OVERRIDE_EN_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(VBUS_OVERRIDE_EN_A::DISABLE)
    }
    #[doc = "Sets the corresponding bit"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(VBUS_OVERRIDE_EN_A::ENABLE)
    }
}
#[doc = "Field `SESSEND_OVERRIDE` reader - Override value for SESSEND"]
pub type SESSEND_OVERRIDE_R = crate::BitReader<SESSEND_OVERRIDE_A>;
#[doc = "Override value for SESSEND\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SESSEND_OVERRIDE_A {
    #[doc = "0: No effect"]
    DISABLE = 0,
    #[doc = "1: Sets the corresponding bit"]
    ENABLE = 1,
}
impl From<SESSEND_OVERRIDE_A> for bool {
    #[inline(always)]
    fn from(variant: SESSEND_OVERRIDE_A) -> Self {
        variant as u8 != 0
    }
}
impl SESSEND_OVERRIDE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SESSEND_OVERRIDE_A {
        match self.bits {
            false => SESSEND_OVERRIDE_A::DISABLE,
            true => SESSEND_OVERRIDE_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == SESSEND_OVERRIDE_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == SESSEND_OVERRIDE_A::ENABLE
    }
}
#[doc = "Field `SESSEND_OVERRIDE` writer - Override value for SESSEND"]
pub type SESSEND_OVERRIDE_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, USB1_VBUS_DETECT_SET_SPEC, SESSEND_OVERRIDE_A, O>;
impl<'a, const O: u8> SESSEND_OVERRIDE_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(SESSEND_OVERRIDE_A::DISABLE)
    }
    #[doc = "Sets the corresponding bit"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(SESSEND_OVERRIDE_A::ENABLE)
    }
}
#[doc = "Field `BVALID_OVERRIDE` reader - Override value for B-Device Session Valid"]
pub type BVALID_OVERRIDE_R = crate::BitReader<BVALID_OVERRIDE_A>;
#[doc = "Override value for B-Device Session Valid\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BVALID_OVERRIDE_A {
    #[doc = "0: No effect"]
    DISABLE = 0,
    #[doc = "1: Sets the corresponding bit"]
    ENABLE = 1,
}
impl From<BVALID_OVERRIDE_A> for bool {
    #[inline(always)]
    fn from(variant: BVALID_OVERRIDE_A) -> Self {
        variant as u8 != 0
    }
}
impl BVALID_OVERRIDE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BVALID_OVERRIDE_A {
        match self.bits {
            false => BVALID_OVERRIDE_A::DISABLE,
            true => BVALID_OVERRIDE_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == BVALID_OVERRIDE_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == BVALID_OVERRIDE_A::ENABLE
    }
}
#[doc = "Field `BVALID_OVERRIDE` writer - Override value for B-Device Session Valid"]
pub type BVALID_OVERRIDE_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, USB1_VBUS_DETECT_SET_SPEC, BVALID_OVERRIDE_A, O>;
impl<'a, const O: u8> BVALID_OVERRIDE_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(BVALID_OVERRIDE_A::DISABLE)
    }
    #[doc = "Sets the corresponding bit"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(BVALID_OVERRIDE_A::ENABLE)
    }
}
#[doc = "Field `AVALID_OVERRIDE` reader - Override value for A-Device Session Valid"]
pub type AVALID_OVERRIDE_R = crate::BitReader<AVALID_OVERRIDE_A>;
#[doc = "Override value for A-Device Session Valid\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AVALID_OVERRIDE_A {
    #[doc = "0: No effect"]
    DISABLE = 0,
    #[doc = "1: Sets the corresponding bit"]
    ENABLE = 1,
}
impl From<AVALID_OVERRIDE_A> for bool {
    #[inline(always)]
    fn from(variant: AVALID_OVERRIDE_A) -> Self {
        variant as u8 != 0
    }
}
impl AVALID_OVERRIDE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AVALID_OVERRIDE_A {
        match self.bits {
            false => AVALID_OVERRIDE_A::DISABLE,
            true => AVALID_OVERRIDE_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == AVALID_OVERRIDE_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == AVALID_OVERRIDE_A::ENABLE
    }
}
#[doc = "Field `AVALID_OVERRIDE` writer - Override value for A-Device Session Valid"]
pub type AVALID_OVERRIDE_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, USB1_VBUS_DETECT_SET_SPEC, AVALID_OVERRIDE_A, O>;
impl<'a, const O: u8> AVALID_OVERRIDE_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(AVALID_OVERRIDE_A::DISABLE)
    }
    #[doc = "Sets the corresponding bit"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(AVALID_OVERRIDE_A::ENABLE)
    }
}
#[doc = "Field `VBUSVALID_OVERRIDE` reader - Override value for VBUS_VALID signal sent to USB controller"]
pub type VBUSVALID_OVERRIDE_R = crate::BitReader<VBUSVALID_OVERRIDE_A>;
#[doc = "Override value for VBUS_VALID signal sent to USB controller\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VBUSVALID_OVERRIDE_A {
    #[doc = "0: No effect"]
    DISABLE = 0,
    #[doc = "1: Sets the corresponding bit"]
    ENABLE = 1,
}
impl From<VBUSVALID_OVERRIDE_A> for bool {
    #[inline(always)]
    fn from(variant: VBUSVALID_OVERRIDE_A) -> Self {
        variant as u8 != 0
    }
}
impl VBUSVALID_OVERRIDE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> VBUSVALID_OVERRIDE_A {
        match self.bits {
            false => VBUSVALID_OVERRIDE_A::DISABLE,
            true => VBUSVALID_OVERRIDE_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == VBUSVALID_OVERRIDE_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == VBUSVALID_OVERRIDE_A::ENABLE
    }
}
#[doc = "Field `VBUSVALID_OVERRIDE` writer - Override value for VBUS_VALID signal sent to USB controller"]
pub type VBUSVALID_OVERRIDE_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, USB1_VBUS_DETECT_SET_SPEC, VBUSVALID_OVERRIDE_A, O>;
impl<'a, const O: u8> VBUSVALID_OVERRIDE_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(VBUSVALID_OVERRIDE_A::DISABLE)
    }
    #[doc = "Sets the corresponding bit"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(VBUSVALID_OVERRIDE_A::ENABLE)
    }
}
#[doc = "Field `VBUSVALID_SEL` reader - Selects the source of the VBUS_VALID signal reported to the USB controller"]
pub type VBUSVALID_SEL_R = crate::BitReader<VBUSVALID_SEL_A>;
#[doc = "Selects the source of the VBUS_VALID signal reported to the USB controller\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VBUSVALID_SEL_A {
    #[doc = "0: No effect"]
    DISABLE = 0,
    #[doc = "1: Sets the corresponding bit"]
    ENABLE = 1,
}
impl From<VBUSVALID_SEL_A> for bool {
    #[inline(always)]
    fn from(variant: VBUSVALID_SEL_A) -> Self {
        variant as u8 != 0
    }
}
impl VBUSVALID_SEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> VBUSVALID_SEL_A {
        match self.bits {
            false => VBUSVALID_SEL_A::DISABLE,
            true => VBUSVALID_SEL_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == VBUSVALID_SEL_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == VBUSVALID_SEL_A::ENABLE
    }
}
#[doc = "Field `VBUSVALID_SEL` writer - Selects the source of the VBUS_VALID signal reported to the USB controller"]
pub type VBUSVALID_SEL_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, USB1_VBUS_DETECT_SET_SPEC, VBUSVALID_SEL_A, O>;
impl<'a, const O: u8> VBUSVALID_SEL_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(VBUSVALID_SEL_A::DISABLE)
    }
    #[doc = "Sets the corresponding bit"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(VBUSVALID_SEL_A::ENABLE)
    }
}
#[doc = "Field `VBUS_SOURCE_SEL` reader - Selects the source of the VBUS_VALID signal reported to the USB controller"]
pub type VBUS_SOURCE_SEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `VBUS_SOURCE_SEL` writer - Selects the source of the VBUS_VALID signal reported to the USB controller"]
pub type VBUS_SOURCE_SEL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, USB1_VBUS_DETECT_SET_SPEC, u8, u8, 2, O>;
#[doc = "Field `VBUSVALID_TO_SESSVALID` reader - Selects the comparator used for VBUS_VALID"]
pub type VBUSVALID_TO_SESSVALID_R = crate::BitReader<VBUSVALID_TO_SESSVALID_A>;
#[doc = "Selects the comparator used for VBUS_VALID\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VBUSVALID_TO_SESSVALID_A {
    #[doc = "0: No effect"]
    DISABLE = 0,
    #[doc = "1: Sets the corresponding bit"]
    ENABLE = 1,
}
impl From<VBUSVALID_TO_SESSVALID_A> for bool {
    #[inline(always)]
    fn from(variant: VBUSVALID_TO_SESSVALID_A) -> Self {
        variant as u8 != 0
    }
}
impl VBUSVALID_TO_SESSVALID_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> VBUSVALID_TO_SESSVALID_A {
        match self.bits {
            false => VBUSVALID_TO_SESSVALID_A::DISABLE,
            true => VBUSVALID_TO_SESSVALID_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == VBUSVALID_TO_SESSVALID_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == VBUSVALID_TO_SESSVALID_A::ENABLE
    }
}
#[doc = "Field `VBUSVALID_TO_SESSVALID` writer - Selects the comparator used for VBUS_VALID"]
pub type VBUSVALID_TO_SESSVALID_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, USB1_VBUS_DETECT_SET_SPEC, VBUSVALID_TO_SESSVALID_A, O>;
impl<'a, const O: u8> VBUSVALID_TO_SESSVALID_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(VBUSVALID_TO_SESSVALID_A::DISABLE)
    }
    #[doc = "Sets the corresponding bit"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(VBUSVALID_TO_SESSVALID_A::ENABLE)
    }
}
#[doc = "Field `PWRUP_CMPS` reader - Enables the VBUS_VALID comparator"]
pub type PWRUP_CMPS_R = crate::BitReader<PWRUP_CMPS_A>;
#[doc = "Enables the VBUS_VALID comparator\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PWRUP_CMPS_A {
    #[doc = "0: No effect"]
    DISABLE = 0,
    #[doc = "1: Sets the corresponding bit"]
    ENABLE = 1,
}
impl From<PWRUP_CMPS_A> for bool {
    #[inline(always)]
    fn from(variant: PWRUP_CMPS_A) -> Self {
        variant as u8 != 0
    }
}
impl PWRUP_CMPS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PWRUP_CMPS_A {
        match self.bits {
            false => PWRUP_CMPS_A::DISABLE,
            true => PWRUP_CMPS_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == PWRUP_CMPS_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == PWRUP_CMPS_A::ENABLE
    }
}
#[doc = "Field `PWRUP_CMPS` writer - Enables the VBUS_VALID comparator"]
pub type PWRUP_CMPS_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, USB1_VBUS_DETECT_SET_SPEC, PWRUP_CMPS_A, O>;
impl<'a, const O: u8> PWRUP_CMPS_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(PWRUP_CMPS_A::DISABLE)
    }
    #[doc = "Sets the corresponding bit"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(PWRUP_CMPS_A::ENABLE)
    }
}
#[doc = "Field `DISCHARGE_VBUS` reader - Controls VBUS discharge resistor"]
pub type DISCHARGE_VBUS_R = crate::BitReader<DISCHARGE_VBUS_A>;
#[doc = "Controls VBUS discharge resistor\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DISCHARGE_VBUS_A {
    #[doc = "0: No effect"]
    DISABLE = 0,
    #[doc = "1: Sets the corresponding bit"]
    ENABLE = 1,
}
impl From<DISCHARGE_VBUS_A> for bool {
    #[inline(always)]
    fn from(variant: DISCHARGE_VBUS_A) -> Self {
        variant as u8 != 0
    }
}
impl DISCHARGE_VBUS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DISCHARGE_VBUS_A {
        match self.bits {
            false => DISCHARGE_VBUS_A::DISABLE,
            true => DISCHARGE_VBUS_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == DISCHARGE_VBUS_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == DISCHARGE_VBUS_A::ENABLE
    }
}
#[doc = "Field `DISCHARGE_VBUS` writer - Controls VBUS discharge resistor"]
pub type DISCHARGE_VBUS_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, USB1_VBUS_DETECT_SET_SPEC, DISCHARGE_VBUS_A, O>;
impl<'a, const O: u8> DISCHARGE_VBUS_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(DISCHARGE_VBUS_A::DISABLE)
    }
    #[doc = "Sets the corresponding bit"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(DISCHARGE_VBUS_A::ENABLE)
    }
}
#[doc = "Field `EN_CHARGER_RESISTOR` reader - Enables resistors used for an older method of resistive battery charger detection"]
pub type EN_CHARGER_RESISTOR_R = crate::BitReader<EN_CHARGER_RESISTOR_A>;
#[doc = "Enables resistors used for an older method of resistive battery charger detection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EN_CHARGER_RESISTOR_A {
    #[doc = "0: No effect"]
    DISABLE = 0,
    #[doc = "1: Sets the corresponding bit"]
    ENABLE = 1,
}
impl From<EN_CHARGER_RESISTOR_A> for bool {
    #[inline(always)]
    fn from(variant: EN_CHARGER_RESISTOR_A) -> Self {
        variant as u8 != 0
    }
}
impl EN_CHARGER_RESISTOR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EN_CHARGER_RESISTOR_A {
        match self.bits {
            false => EN_CHARGER_RESISTOR_A::DISABLE,
            true => EN_CHARGER_RESISTOR_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == EN_CHARGER_RESISTOR_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == EN_CHARGER_RESISTOR_A::ENABLE
    }
}
#[doc = "Field `EN_CHARGER_RESISTOR` writer - Enables resistors used for an older method of resistive battery charger detection"]
pub type EN_CHARGER_RESISTOR_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, USB1_VBUS_DETECT_SET_SPEC, EN_CHARGER_RESISTOR_A, O>;
impl<'a, const O: u8> EN_CHARGER_RESISTOR_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(EN_CHARGER_RESISTOR_A::DISABLE)
    }
    #[doc = "Sets the corresponding bit"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(EN_CHARGER_RESISTOR_A::ENABLE)
    }
}
impl R {
    #[doc = "Bits 0:2 - VBUS comparator threshold"]
    #[inline(always)]
    pub fn vbusvalid_thresh(&self) -> VBUSVALID_THRESH_R {
        VBUSVALID_THRESH_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 3 - VBUS detect signal override. This bit is used when EXT_VBUS_OVERRIDE_EN = 1'b0."]
    #[inline(always)]
    pub fn vbus_override_en(&self) -> VBUS_OVERRIDE_EN_R {
        VBUS_OVERRIDE_EN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Override value for SESSEND"]
    #[inline(always)]
    pub fn sessend_override(&self) -> SESSEND_OVERRIDE_R {
        SESSEND_OVERRIDE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Override value for B-Device Session Valid"]
    #[inline(always)]
    pub fn bvalid_override(&self) -> BVALID_OVERRIDE_R {
        BVALID_OVERRIDE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Override value for A-Device Session Valid"]
    #[inline(always)]
    pub fn avalid_override(&self) -> AVALID_OVERRIDE_R {
        AVALID_OVERRIDE_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Override value for VBUS_VALID signal sent to USB controller"]
    #[inline(always)]
    pub fn vbusvalid_override(&self) -> VBUSVALID_OVERRIDE_R {
        VBUSVALID_OVERRIDE_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Selects the source of the VBUS_VALID signal reported to the USB controller"]
    #[inline(always)]
    pub fn vbusvalid_sel(&self) -> VBUSVALID_SEL_R {
        VBUSVALID_SEL_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 9:10 - Selects the source of the VBUS_VALID signal reported to the USB controller"]
    #[inline(always)]
    pub fn vbus_source_sel(&self) -> VBUS_SOURCE_SEL_R {
        VBUS_SOURCE_SEL_R::new(((self.bits >> 9) & 3) as u8)
    }
    #[doc = "Bit 18 - Selects the comparator used for VBUS_VALID"]
    #[inline(always)]
    pub fn vbusvalid_to_sessvalid(&self) -> VBUSVALID_TO_SESSVALID_R {
        VBUSVALID_TO_SESSVALID_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 20 - Enables the VBUS_VALID comparator"]
    #[inline(always)]
    pub fn pwrup_cmps(&self) -> PWRUP_CMPS_R {
        PWRUP_CMPS_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 26 - Controls VBUS discharge resistor"]
    #[inline(always)]
    pub fn discharge_vbus(&self) -> DISCHARGE_VBUS_R {
        DISCHARGE_VBUS_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 31 - Enables resistors used for an older method of resistive battery charger detection"]
    #[inline(always)]
    pub fn en_charger_resistor(&self) -> EN_CHARGER_RESISTOR_R {
        EN_CHARGER_RESISTOR_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - VBUS comparator threshold"]
    #[inline(always)]
    #[must_use]
    pub fn vbusvalid_thresh(&mut self) -> VBUSVALID_THRESH_W<0> {
        VBUSVALID_THRESH_W::new(self)
    }
    #[doc = "Bit 3 - VBUS detect signal override. This bit is used when EXT_VBUS_OVERRIDE_EN = 1'b0."]
    #[inline(always)]
    #[must_use]
    pub fn vbus_override_en(&mut self) -> VBUS_OVERRIDE_EN_W<3> {
        VBUS_OVERRIDE_EN_W::new(self)
    }
    #[doc = "Bit 4 - Override value for SESSEND"]
    #[inline(always)]
    #[must_use]
    pub fn sessend_override(&mut self) -> SESSEND_OVERRIDE_W<4> {
        SESSEND_OVERRIDE_W::new(self)
    }
    #[doc = "Bit 5 - Override value for B-Device Session Valid"]
    #[inline(always)]
    #[must_use]
    pub fn bvalid_override(&mut self) -> BVALID_OVERRIDE_W<5> {
        BVALID_OVERRIDE_W::new(self)
    }
    #[doc = "Bit 6 - Override value for A-Device Session Valid"]
    #[inline(always)]
    #[must_use]
    pub fn avalid_override(&mut self) -> AVALID_OVERRIDE_W<6> {
        AVALID_OVERRIDE_W::new(self)
    }
    #[doc = "Bit 7 - Override value for VBUS_VALID signal sent to USB controller"]
    #[inline(always)]
    #[must_use]
    pub fn vbusvalid_override(&mut self) -> VBUSVALID_OVERRIDE_W<7> {
        VBUSVALID_OVERRIDE_W::new(self)
    }
    #[doc = "Bit 8 - Selects the source of the VBUS_VALID signal reported to the USB controller"]
    #[inline(always)]
    #[must_use]
    pub fn vbusvalid_sel(&mut self) -> VBUSVALID_SEL_W<8> {
        VBUSVALID_SEL_W::new(self)
    }
    #[doc = "Bits 9:10 - Selects the source of the VBUS_VALID signal reported to the USB controller"]
    #[inline(always)]
    #[must_use]
    pub fn vbus_source_sel(&mut self) -> VBUS_SOURCE_SEL_W<9> {
        VBUS_SOURCE_SEL_W::new(self)
    }
    #[doc = "Bit 18 - Selects the comparator used for VBUS_VALID"]
    #[inline(always)]
    #[must_use]
    pub fn vbusvalid_to_sessvalid(&mut self) -> VBUSVALID_TO_SESSVALID_W<18> {
        VBUSVALID_TO_SESSVALID_W::new(self)
    }
    #[doc = "Bit 20 - Enables the VBUS_VALID comparator"]
    #[inline(always)]
    #[must_use]
    pub fn pwrup_cmps(&mut self) -> PWRUP_CMPS_W<20> {
        PWRUP_CMPS_W::new(self)
    }
    #[doc = "Bit 26 - Controls VBUS discharge resistor"]
    #[inline(always)]
    #[must_use]
    pub fn discharge_vbus(&mut self) -> DISCHARGE_VBUS_W<26> {
        DISCHARGE_VBUS_W::new(self)
    }
    #[doc = "Bit 31 - Enables resistors used for an older method of resistive battery charger detection"]
    #[inline(always)]
    #[must_use]
    pub fn en_charger_resistor(&mut self) -> EN_CHARGER_RESISTOR_W<31> {
        EN_CHARGER_RESISTOR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "VBUS detect Set\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usb1_vbus_detect_set](index.html) module"]
pub struct USB1_VBUS_DETECT_SET_SPEC;
impl crate::RegisterSpec for USB1_VBUS_DETECT_SET_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [usb1_vbus_detect_set::R](R) reader structure"]
impl crate::Readable for USB1_VBUS_DETECT_SET_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [usb1_vbus_detect_set::W](W) writer structure"]
impl crate::Writable for USB1_VBUS_DETECT_SET_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets USB1_VBUS_DETECT_SET to value 0x0070_0004"]
impl crate::Resettable for USB1_VBUS_DETECT_SET_SPEC {
    const RESET_VALUE: Self::Ux = 0x0070_0004;
}
