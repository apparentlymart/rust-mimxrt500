#[doc = "Register `USB1_VBUS_DETECT` reader"]
pub struct R(crate::R<USB1_VBUS_DETECT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<USB1_VBUS_DETECT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<USB1_VBUS_DETECT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<USB1_VBUS_DETECT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `USB1_VBUS_DETECT` writer"]
pub struct W(crate::W<USB1_VBUS_DETECT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<USB1_VBUS_DETECT_SPEC>;
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
impl From<crate::W<USB1_VBUS_DETECT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<USB1_VBUS_DETECT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `VBUSVALID_THRESH` reader - VBUS comparator threshold"]
pub type VBUSVALID_THRESH_R = crate::FieldReader<u8, u8>;
#[doc = "Field `VBUSVALID_THRESH` writer - VBUS comparator threshold"]
pub type VBUSVALID_THRESH_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, USB1_VBUS_DETECT_SPEC, u8, u8, 3, O>;
#[doc = "Field `VBUS_OVERRIDE_EN` reader - VBUS detect signal override enable"]
pub type VBUS_OVERRIDE_EN_R = crate::BitReader<VBUS_OVERRIDE_EN_A>;
#[doc = "VBUS detect signal override enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VBUS_OVERRIDE_EN_A {
    #[doc = "0: Use the results of the internal VBUS_VALID and Session Valid comparators for VBUS_VALID, AVALID, BVALID, and SESSEND (Default)"]
    INTERNAL = 0,
    #[doc = "1: Use the override values for VBUS_VALID, AVALID, BVALID, and SESSEND"]
    OVERRIDE = 1,
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
            false => VBUS_OVERRIDE_EN_A::INTERNAL,
            true => VBUS_OVERRIDE_EN_A::OVERRIDE,
        }
    }
    #[doc = "Checks if the value of the field is `INTERNAL`"]
    #[inline(always)]
    pub fn is_internal(&self) -> bool {
        *self == VBUS_OVERRIDE_EN_A::INTERNAL
    }
    #[doc = "Checks if the value of the field is `OVERRIDE`"]
    #[inline(always)]
    pub fn is_override(&self) -> bool {
        *self == VBUS_OVERRIDE_EN_A::OVERRIDE
    }
}
#[doc = "Field `VBUS_OVERRIDE_EN` writer - VBUS detect signal override enable"]
pub type VBUS_OVERRIDE_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, USB1_VBUS_DETECT_SPEC, VBUS_OVERRIDE_EN_A, O>;
impl<'a, const O: u8> VBUS_OVERRIDE_EN_W<'a, O> {
    #[doc = "Use the results of the internal VBUS_VALID and Session Valid comparators for VBUS_VALID, AVALID, BVALID, and SESSEND (Default)"]
    #[inline(always)]
    pub fn internal(self) -> &'a mut W {
        self.variant(VBUS_OVERRIDE_EN_A::INTERNAL)
    }
    #[doc = "Use the override values for VBUS_VALID, AVALID, BVALID, and SESSEND"]
    #[inline(always)]
    pub fn override_(self) -> &'a mut W {
        self.variant(VBUS_OVERRIDE_EN_A::OVERRIDE)
    }
}
#[doc = "Field `SESSEND_OVERRIDE` reader - Override value for SESSEND"]
pub type SESSEND_OVERRIDE_R = crate::BitReader<bool>;
#[doc = "Field `SESSEND_OVERRIDE` writer - Override value for SESSEND"]
pub type SESSEND_OVERRIDE_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, USB1_VBUS_DETECT_SPEC, bool, O>;
#[doc = "Field `BVALID_OVERRIDE` reader - Override value for B-Device Session Valid"]
pub type BVALID_OVERRIDE_R = crate::BitReader<bool>;
#[doc = "Field `BVALID_OVERRIDE` writer - Override value for B-Device Session Valid"]
pub type BVALID_OVERRIDE_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, USB1_VBUS_DETECT_SPEC, bool, O>;
#[doc = "Field `AVALID_OVERRIDE` reader - Override value for A-Device Session Valid"]
pub type AVALID_OVERRIDE_R = crate::BitReader<bool>;
#[doc = "Field `AVALID_OVERRIDE` writer - Override value for A-Device Session Valid"]
pub type AVALID_OVERRIDE_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, USB1_VBUS_DETECT_SPEC, bool, O>;
#[doc = "Field `VBUSVALID_OVERRIDE` reader - Override value for VBUS_VALID signal sent to USB controller"]
pub type VBUSVALID_OVERRIDE_R = crate::BitReader<bool>;
#[doc = "Field `VBUSVALID_OVERRIDE` writer - Override value for VBUS_VALID signal sent to USB controller"]
pub type VBUSVALID_OVERRIDE_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, USB1_VBUS_DETECT_SPEC, bool, O>;
#[doc = "Field `VBUSVALID_SEL` reader - Selects the source of the VBUS_VALID signal reported to the USB controller"]
pub type VBUSVALID_SEL_R = crate::BitReader<VBUSVALID_SEL_A>;
#[doc = "Selects the source of the VBUS_VALID signal reported to the USB controller\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VBUSVALID_SEL_A {
    #[doc = "0: Use the VBUS_VALID comparator results for signal reported to the USB controller (Default)"]
    COMP = 0,
    #[doc = "1: Use the VBUS_VALID_3V detector results for signal reported to the USB controller"]
    DET_3V = 1,
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
            false => VBUSVALID_SEL_A::COMP,
            true => VBUSVALID_SEL_A::DET_3V,
        }
    }
    #[doc = "Checks if the value of the field is `COMP`"]
    #[inline(always)]
    pub fn is_comp(&self) -> bool {
        *self == VBUSVALID_SEL_A::COMP
    }
    #[doc = "Checks if the value of the field is `DET_3V`"]
    #[inline(always)]
    pub fn is_det_3v(&self) -> bool {
        *self == VBUSVALID_SEL_A::DET_3V
    }
}
#[doc = "Field `VBUSVALID_SEL` writer - Selects the source of the VBUS_VALID signal reported to the USB controller"]
pub type VBUSVALID_SEL_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, USB1_VBUS_DETECT_SPEC, VBUSVALID_SEL_A, O>;
impl<'a, const O: u8> VBUSVALID_SEL_W<'a, O> {
    #[doc = "Use the VBUS_VALID comparator results for signal reported to the USB controller (Default)"]
    #[inline(always)]
    pub fn comp(self) -> &'a mut W {
        self.variant(VBUSVALID_SEL_A::COMP)
    }
    #[doc = "Use the VBUS_VALID_3V detector results for signal reported to the USB controller"]
    #[inline(always)]
    pub fn det_3v(self) -> &'a mut W {
        self.variant(VBUSVALID_SEL_A::DET_3V)
    }
}
#[doc = "Field `VBUS_SOURCE_SEL` reader - Selects the source of the VBUS_VALID signal reported to the USB controller"]
pub type VBUS_SOURCE_SEL_R = crate::FieldReader<u8, VBUS_SOURCE_SEL_A>;
#[doc = "Selects the source of the VBUS_VALID signal reported to the USB controller\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum VBUS_SOURCE_SEL_A {
    #[doc = "0: Use the VBUS_VALID comparator results for signal reported to the USB controller (Default)"]
    VBUS_VALID_COMP = 0,
    #[doc = "1: Use the Session Valid comparator results for signal reported to the USB controller"]
    SESSION_VALID_COMP = 1,
    #[doc = "2: Use the Session Valid comparator results for signal reported to the USB controller"]
    SESSION_VALID_COMP_1 = 2,
}
impl From<VBUS_SOURCE_SEL_A> for u8 {
    #[inline(always)]
    fn from(variant: VBUS_SOURCE_SEL_A) -> Self {
        variant as _
    }
}
impl VBUS_SOURCE_SEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<VBUS_SOURCE_SEL_A> {
        match self.bits {
            0 => Some(VBUS_SOURCE_SEL_A::VBUS_VALID_COMP),
            1 => Some(VBUS_SOURCE_SEL_A::SESSION_VALID_COMP),
            2 => Some(VBUS_SOURCE_SEL_A::SESSION_VALID_COMP_1),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `VBUS_VALID_COMP`"]
    #[inline(always)]
    pub fn is_vbus_valid_comp(&self) -> bool {
        *self == VBUS_SOURCE_SEL_A::VBUS_VALID_COMP
    }
    #[doc = "Checks if the value of the field is `SESSION_VALID_COMP`"]
    #[inline(always)]
    pub fn is_session_valid_comp(&self) -> bool {
        *self == VBUS_SOURCE_SEL_A::SESSION_VALID_COMP
    }
    #[doc = "Checks if the value of the field is `SESSION_VALID_COMP_1`"]
    #[inline(always)]
    pub fn is_session_valid_comp_1(&self) -> bool {
        *self == VBUS_SOURCE_SEL_A::SESSION_VALID_COMP_1
    }
}
#[doc = "Field `VBUS_SOURCE_SEL` writer - Selects the source of the VBUS_VALID signal reported to the USB controller"]
pub type VBUS_SOURCE_SEL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, USB1_VBUS_DETECT_SPEC, u8, VBUS_SOURCE_SEL_A, 2, O>;
impl<'a, const O: u8> VBUS_SOURCE_SEL_W<'a, O> {
    #[doc = "Use the VBUS_VALID comparator results for signal reported to the USB controller (Default)"]
    #[inline(always)]
    pub fn vbus_valid_comp(self) -> &'a mut W {
        self.variant(VBUS_SOURCE_SEL_A::VBUS_VALID_COMP)
    }
    #[doc = "Use the Session Valid comparator results for signal reported to the USB controller"]
    #[inline(always)]
    pub fn session_valid_comp(self) -> &'a mut W {
        self.variant(VBUS_SOURCE_SEL_A::SESSION_VALID_COMP)
    }
    #[doc = "Use the Session Valid comparator results for signal reported to the USB controller"]
    #[inline(always)]
    pub fn session_valid_comp_1(self) -> &'a mut W {
        self.variant(VBUS_SOURCE_SEL_A::SESSION_VALID_COMP_1)
    }
}
#[doc = "Field `VBUSVALID_TO_SESSVALID` reader - Selects the comparator used for VBUS_VALID"]
pub type VBUSVALID_TO_SESSVALID_R = crate::BitReader<VBUSVALID_TO_SESSVALID_A>;
#[doc = "Selects the comparator used for VBUS_VALID\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VBUSVALID_TO_SESSVALID_A {
    #[doc = "0: Use the VBUS_VALID comparator for VBUS_VALID results"]
    VBUS_VALID = 0,
    #[doc = "1: Use the Session End comparator for VBUS_VALID results. The Session End threshold is >0.8V and <4.0V."]
    SESSION_VALID = 1,
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
            false => VBUSVALID_TO_SESSVALID_A::VBUS_VALID,
            true => VBUSVALID_TO_SESSVALID_A::SESSION_VALID,
        }
    }
    #[doc = "Checks if the value of the field is `VBUS_VALID`"]
    #[inline(always)]
    pub fn is_vbus_valid(&self) -> bool {
        *self == VBUSVALID_TO_SESSVALID_A::VBUS_VALID
    }
    #[doc = "Checks if the value of the field is `SESSION_VALID`"]
    #[inline(always)]
    pub fn is_session_valid(&self) -> bool {
        *self == VBUSVALID_TO_SESSVALID_A::SESSION_VALID
    }
}
#[doc = "Field `VBUSVALID_TO_SESSVALID` writer - Selects the comparator used for VBUS_VALID"]
pub type VBUSVALID_TO_SESSVALID_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, USB1_VBUS_DETECT_SPEC, VBUSVALID_TO_SESSVALID_A, O>;
impl<'a, const O: u8> VBUSVALID_TO_SESSVALID_W<'a, O> {
    #[doc = "Use the VBUS_VALID comparator for VBUS_VALID results"]
    #[inline(always)]
    pub fn vbus_valid(self) -> &'a mut W {
        self.variant(VBUSVALID_TO_SESSVALID_A::VBUS_VALID)
    }
    #[doc = "Use the Session End comparator for VBUS_VALID results. The Session End threshold is >0.8V and <4.0V."]
    #[inline(always)]
    pub fn session_valid(self) -> &'a mut W {
        self.variant(VBUSVALID_TO_SESSVALID_A::SESSION_VALID)
    }
}
#[doc = "Field `PWRUP_CMPS` reader - Enables the VBUS_VALID comparator"]
pub type PWRUP_CMPS_R = crate::BitReader<PWRUP_CMPS_A>;
#[doc = "Enables the VBUS_VALID comparator\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PWRUP_CMPS_A {
    #[doc = "0: Powers down the VBUS_VALID comparator"]
    DISABLE = 0,
    #[doc = "1: Enables the VBUS_VALID comparator (default)"]
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
    crate::BitWriter<'a, u32, USB1_VBUS_DETECT_SPEC, PWRUP_CMPS_A, O>;
impl<'a, const O: u8> PWRUP_CMPS_W<'a, O> {
    #[doc = "Powers down the VBUS_VALID comparator"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(PWRUP_CMPS_A::DISABLE)
    }
    #[doc = "Enables the VBUS_VALID comparator (default)"]
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
    #[doc = "0: VBUS discharge resistor is disabled (Default)"]
    DISABLE = 0,
    #[doc = "1: VBUS discharge resistor is enabled"]
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
    crate::BitWriter<'a, u32, USB1_VBUS_DETECT_SPEC, DISCHARGE_VBUS_A, O>;
impl<'a, const O: u8> DISCHARGE_VBUS_W<'a, O> {
    #[doc = "VBUS discharge resistor is disabled (Default)"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(DISCHARGE_VBUS_A::DISABLE)
    }
    #[doc = "VBUS discharge resistor is enabled"]
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
    #[doc = "0: Disable resistive charger detection resistors on DP and DP"]
    DISABLE = 0,
    #[doc = "1: Enable resistive charger detection resistors on DP and DP"]
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
    crate::BitWriter<'a, u32, USB1_VBUS_DETECT_SPEC, EN_CHARGER_RESISTOR_A, O>;
impl<'a, const O: u8> EN_CHARGER_RESISTOR_W<'a, O> {
    #[doc = "Disable resistive charger detection resistors on DP and DP"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(EN_CHARGER_RESISTOR_A::DISABLE)
    }
    #[doc = "Enable resistive charger detection resistors on DP and DP"]
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
    #[doc = "Bit 3 - VBUS detect signal override enable"]
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
    #[doc = "Bit 3 - VBUS detect signal override enable"]
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
#[doc = "VBUS detect\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usb1_vbus_detect](index.html) module"]
pub struct USB1_VBUS_DETECT_SPEC;
impl crate::RegisterSpec for USB1_VBUS_DETECT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [usb1_vbus_detect::R](R) reader structure"]
impl crate::Readable for USB1_VBUS_DETECT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [usb1_vbus_detect::W](W) writer structure"]
impl crate::Writable for USB1_VBUS_DETECT_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets USB1_VBUS_DETECT to value 0x0070_0004"]
impl crate::Resettable for USB1_VBUS_DETECT_SPEC {
    const RESET_VALUE: Self::Ux = 0x0070_0004;
}
