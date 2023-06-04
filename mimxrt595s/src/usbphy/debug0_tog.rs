#[doc = "Register `DEBUG0_TOG` reader"]
pub struct R(crate::R<DEBUG0_TOG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DEBUG0_TOG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DEBUG0_TOG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DEBUG0_TOG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DEBUG0_TOG` writer"]
pub struct W(crate::W<DEBUG0_TOG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DEBUG0_TOG_SPEC>;
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
impl From<crate::W<DEBUG0_TOG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DEBUG0_TOG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DEBUG_INTERFACE_HOLD` reader - Debug interface"]
pub type DEBUG_INTERFACE_HOLD_R = crate::BitReader<DEBUG_INTERFACE_HOLD_A>;
#[doc = "Debug interface\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DEBUG_INTERFACE_HOLD_A {
    #[doc = "0: No effect"]
    DISABLE = 0,
    #[doc = "1: Toggles the corresponding CTRL bit"]
    ENABLE = 1,
}
impl From<DEBUG_INTERFACE_HOLD_A> for bool {
    #[inline(always)]
    fn from(variant: DEBUG_INTERFACE_HOLD_A) -> Self {
        variant as u8 != 0
    }
}
impl DEBUG_INTERFACE_HOLD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DEBUG_INTERFACE_HOLD_A {
        match self.bits {
            false => DEBUG_INTERFACE_HOLD_A::DISABLE,
            true => DEBUG_INTERFACE_HOLD_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == DEBUG_INTERFACE_HOLD_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == DEBUG_INTERFACE_HOLD_A::ENABLE
    }
}
#[doc = "Field `DEBUG_INTERFACE_HOLD` writer - Debug interface"]
pub type DEBUG_INTERFACE_HOLD_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, DEBUG0_TOG_SPEC, DEBUG_INTERFACE_HOLD_A, O>;
impl<'a, const O: u8> DEBUG_INTERFACE_HOLD_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(DEBUG_INTERFACE_HOLD_A::DISABLE)
    }
    #[doc = "Toggles the corresponding CTRL bit"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(DEBUG_INTERFACE_HOLD_A::ENABLE)
    }
}
#[doc = "Field `HSTPULLDOWN` reader - HS DP/DM pulldown resistance select."]
pub type HSTPULLDOWN_R = crate::FieldReader<u8, HSTPULLDOWN_A>;
#[doc = "HS DP/DM pulldown resistance select.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum HSTPULLDOWN_A {
    #[doc = "0: No effect"]
    DISABLE = 0,
    #[doc = "1: Toggles the corresponding CTRL bit"]
    ENABLE = 1,
}
impl From<HSTPULLDOWN_A> for u8 {
    #[inline(always)]
    fn from(variant: HSTPULLDOWN_A) -> Self {
        variant as _
    }
}
impl HSTPULLDOWN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<HSTPULLDOWN_A> {
        match self.bits {
            0 => Some(HSTPULLDOWN_A::DISABLE),
            1 => Some(HSTPULLDOWN_A::ENABLE),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == HSTPULLDOWN_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == HSTPULLDOWN_A::ENABLE
    }
}
#[doc = "Field `HSTPULLDOWN` writer - HS DP/DM pulldown resistance select."]
pub type HSTPULLDOWN_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, DEBUG0_TOG_SPEC, u8, HSTPULLDOWN_A, 2, O>;
impl<'a, const O: u8> HSTPULLDOWN_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(HSTPULLDOWN_A::DISABLE)
    }
    #[doc = "Toggles the corresponding CTRL bit"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(HSTPULLDOWN_A::ENABLE)
    }
}
#[doc = "Field `ENHSTPULLDOWN` reader - Enable Host pulldown"]
pub type ENHSTPULLDOWN_R = crate::FieldReader<u8, ENHSTPULLDOWN_A>;
#[doc = "Enable Host pulldown\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ENHSTPULLDOWN_A {
    #[doc = "0: No effect"]
    DISABLE = 0,
    #[doc = "1: Toggles the corresponding CTRL bit"]
    ENABLE = 1,
}
impl From<ENHSTPULLDOWN_A> for u8 {
    #[inline(always)]
    fn from(variant: ENHSTPULLDOWN_A) -> Self {
        variant as _
    }
}
impl ENHSTPULLDOWN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<ENHSTPULLDOWN_A> {
        match self.bits {
            0 => Some(ENHSTPULLDOWN_A::DISABLE),
            1 => Some(ENHSTPULLDOWN_A::ENABLE),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == ENHSTPULLDOWN_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == ENHSTPULLDOWN_A::ENABLE
    }
}
#[doc = "Field `ENHSTPULLDOWN` writer - Enable Host pulldown"]
pub type ENHSTPULLDOWN_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, DEBUG0_TOG_SPEC, u8, ENHSTPULLDOWN_A, 2, O>;
impl<'a, const O: u8> ENHSTPULLDOWN_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(ENHSTPULLDOWN_A::DISABLE)
    }
    #[doc = "Toggles the corresponding CTRL bit"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(ENHSTPULLDOWN_A::ENABLE)
    }
}
#[doc = "Field `TX2RXCOUNT` reader - TX2RXCOUNT"]
pub type TX2RXCOUNT_R = crate::FieldReader<u8, TX2RXCOUNT_A>;
#[doc = "TX2RXCOUNT\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TX2RXCOUNT_A {
    #[doc = "0: No effect"]
    DISABLE = 0,
    #[doc = "1: Toggles the corresponding CTRL bit"]
    ENABLE = 1,
}
impl From<TX2RXCOUNT_A> for u8 {
    #[inline(always)]
    fn from(variant: TX2RXCOUNT_A) -> Self {
        variant as _
    }
}
impl TX2RXCOUNT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<TX2RXCOUNT_A> {
        match self.bits {
            0 => Some(TX2RXCOUNT_A::DISABLE),
            1 => Some(TX2RXCOUNT_A::ENABLE),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == TX2RXCOUNT_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == TX2RXCOUNT_A::ENABLE
    }
}
#[doc = "Field `TX2RXCOUNT` writer - TX2RXCOUNT"]
pub type TX2RXCOUNT_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, DEBUG0_TOG_SPEC, u8, TX2RXCOUNT_A, 4, O>;
impl<'a, const O: u8> TX2RXCOUNT_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(TX2RXCOUNT_A::DISABLE)
    }
    #[doc = "Toggles the corresponding CTRL bit"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(TX2RXCOUNT_A::ENABLE)
    }
}
#[doc = "Field `ENTX2RXCOUNT` reader - ENTX2RXCOUNT"]
pub type ENTX2RXCOUNT_R = crate::BitReader<ENTX2RXCOUNT_A>;
#[doc = "ENTX2RXCOUNT\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ENTX2RXCOUNT_A {
    #[doc = "0: No effect"]
    DISABLE = 0,
    #[doc = "1: Toggles the corresponding CTRL bit"]
    ENABLE = 1,
}
impl From<ENTX2RXCOUNT_A> for bool {
    #[inline(always)]
    fn from(variant: ENTX2RXCOUNT_A) -> Self {
        variant as u8 != 0
    }
}
impl ENTX2RXCOUNT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ENTX2RXCOUNT_A {
        match self.bits {
            false => ENTX2RXCOUNT_A::DISABLE,
            true => ENTX2RXCOUNT_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == ENTX2RXCOUNT_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == ENTX2RXCOUNT_A::ENABLE
    }
}
#[doc = "Field `ENTX2RXCOUNT` writer - ENTX2RXCOUNT"]
pub type ENTX2RXCOUNT_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, DEBUG0_TOG_SPEC, ENTX2RXCOUNT_A, O>;
impl<'a, const O: u8> ENTX2RXCOUNT_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(ENTX2RXCOUNT_A::DISABLE)
    }
    #[doc = "Toggles the corresponding CTRL bit"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(ENTX2RXCOUNT_A::ENABLE)
    }
}
#[doc = "Field `SQUELCHRESETCOUNT` reader - Squelch reset count"]
pub type SQUELCHRESETCOUNT_R = crate::FieldReader<u8, SQUELCHRESETCOUNT_A>;
#[doc = "Squelch reset count\n\nValue on reset: 24"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SQUELCHRESETCOUNT_A {
    #[doc = "0: No effect"]
    DISABLE = 0,
    #[doc = "1: Toggles the corresponding CTRL bit"]
    ENABLE = 1,
}
impl From<SQUELCHRESETCOUNT_A> for u8 {
    #[inline(always)]
    fn from(variant: SQUELCHRESETCOUNT_A) -> Self {
        variant as _
    }
}
impl SQUELCHRESETCOUNT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<SQUELCHRESETCOUNT_A> {
        match self.bits {
            0 => Some(SQUELCHRESETCOUNT_A::DISABLE),
            1 => Some(SQUELCHRESETCOUNT_A::ENABLE),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == SQUELCHRESETCOUNT_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == SQUELCHRESETCOUNT_A::ENABLE
    }
}
#[doc = "Field `SQUELCHRESETCOUNT` writer - Squelch reset count"]
pub type SQUELCHRESETCOUNT_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, DEBUG0_TOG_SPEC, u8, SQUELCHRESETCOUNT_A, 5, O>;
impl<'a, const O: u8> SQUELCHRESETCOUNT_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(SQUELCHRESETCOUNT_A::DISABLE)
    }
    #[doc = "Toggles the corresponding CTRL bit"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(SQUELCHRESETCOUNT_A::ENABLE)
    }
}
#[doc = "Field `ENSQUELCHRESET` reader - Enable squelch reset"]
pub type ENSQUELCHRESET_R = crate::BitReader<ENSQUELCHRESET_A>;
#[doc = "Enable squelch reset\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ENSQUELCHRESET_A {
    #[doc = "0: No effect"]
    DISABLE = 0,
    #[doc = "1: Toggles the corresponding CTRL bit"]
    ENABLE = 1,
}
impl From<ENSQUELCHRESET_A> for bool {
    #[inline(always)]
    fn from(variant: ENSQUELCHRESET_A) -> Self {
        variant as u8 != 0
    }
}
impl ENSQUELCHRESET_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ENSQUELCHRESET_A {
        match self.bits {
            false => ENSQUELCHRESET_A::DISABLE,
            true => ENSQUELCHRESET_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == ENSQUELCHRESET_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == ENSQUELCHRESET_A::ENABLE
    }
}
#[doc = "Field `ENSQUELCHRESET` writer - Enable squelch reset"]
pub type ENSQUELCHRESET_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, DEBUG0_TOG_SPEC, ENSQUELCHRESET_A, O>;
impl<'a, const O: u8> ENSQUELCHRESET_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(ENSQUELCHRESET_A::DISABLE)
    }
    #[doc = "Toggles the corresponding CTRL bit"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(ENSQUELCHRESET_A::ENABLE)
    }
}
#[doc = "Field `SQUELCHRESETLENGTH` reader - Squelch reset length"]
pub type SQUELCHRESETLENGTH_R = crate::FieldReader<u8, SQUELCHRESETLENGTH_A>;
#[doc = "Squelch reset length\n\nValue on reset: 15"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SQUELCHRESETLENGTH_A {
    #[doc = "0: No effect"]
    DISABLE = 0,
    #[doc = "1: the corresponding CTRL bit"]
    ENABLE = 1,
}
impl From<SQUELCHRESETLENGTH_A> for u8 {
    #[inline(always)]
    fn from(variant: SQUELCHRESETLENGTH_A) -> Self {
        variant as _
    }
}
impl SQUELCHRESETLENGTH_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<SQUELCHRESETLENGTH_A> {
        match self.bits {
            0 => Some(SQUELCHRESETLENGTH_A::DISABLE),
            1 => Some(SQUELCHRESETLENGTH_A::ENABLE),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == SQUELCHRESETLENGTH_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == SQUELCHRESETLENGTH_A::ENABLE
    }
}
#[doc = "Field `SQUELCHRESETLENGTH` writer - Squelch reset length"]
pub type SQUELCHRESETLENGTH_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, DEBUG0_TOG_SPEC, u8, SQUELCHRESETLENGTH_A, 4, O>;
impl<'a, const O: u8> SQUELCHRESETLENGTH_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(SQUELCHRESETLENGTH_A::DISABLE)
    }
    #[doc = "the corresponding CTRL bit"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(SQUELCHRESETLENGTH_A::ENABLE)
    }
}
#[doc = "Field `HOST_RESUME_DEBUG` reader - Host resume"]
pub type HOST_RESUME_DEBUG_R = crate::BitReader<HOST_RESUME_DEBUG_A>;
#[doc = "Host resume\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HOST_RESUME_DEBUG_A {
    #[doc = "0: No effect"]
    DISABLE = 0,
    #[doc = "1: Toggles the corresponding CTRL bit"]
    ENABLE = 1,
}
impl From<HOST_RESUME_DEBUG_A> for bool {
    #[inline(always)]
    fn from(variant: HOST_RESUME_DEBUG_A) -> Self {
        variant as u8 != 0
    }
}
impl HOST_RESUME_DEBUG_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HOST_RESUME_DEBUG_A {
        match self.bits {
            false => HOST_RESUME_DEBUG_A::DISABLE,
            true => HOST_RESUME_DEBUG_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == HOST_RESUME_DEBUG_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == HOST_RESUME_DEBUG_A::ENABLE
    }
}
#[doc = "Field `HOST_RESUME_DEBUG` writer - Host resume"]
pub type HOST_RESUME_DEBUG_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, DEBUG0_TOG_SPEC, HOST_RESUME_DEBUG_A, O>;
impl<'a, const O: u8> HOST_RESUME_DEBUG_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(HOST_RESUME_DEBUG_A::DISABLE)
    }
    #[doc = "Toggles the corresponding CTRL bit"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(HOST_RESUME_DEBUG_A::ENABLE)
    }
}
#[doc = "Field `CLKGATE` reader - Test clock gate"]
pub type CLKGATE_R = crate::BitReader<CLKGATE_A>;
#[doc = "Test clock gate\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CLKGATE_A {
    #[doc = "0: No effect"]
    DISABLE = 0,
    #[doc = "1: Toggles the corresponding CTRL bit"]
    ENABLE = 1,
}
impl From<CLKGATE_A> for bool {
    #[inline(always)]
    fn from(variant: CLKGATE_A) -> Self {
        variant as u8 != 0
    }
}
impl CLKGATE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CLKGATE_A {
        match self.bits {
            false => CLKGATE_A::DISABLE,
            true => CLKGATE_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == CLKGATE_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == CLKGATE_A::ENABLE
    }
}
#[doc = "Field `CLKGATE` writer - Test clock gate"]
pub type CLKGATE_W<'a, const O: u8> = crate::BitWriter<'a, u32, DEBUG0_TOG_SPEC, CLKGATE_A, O>;
impl<'a, const O: u8> CLKGATE_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(CLKGATE_A::DISABLE)
    }
    #[doc = "Toggles the corresponding CTRL bit"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(CLKGATE_A::ENABLE)
    }
}
impl R {
    #[doc = "Bit 1 - Debug interface"]
    #[inline(always)]
    pub fn debug_interface_hold(&self) -> DEBUG_INTERFACE_HOLD_R {
        DEBUG_INTERFACE_HOLD_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:3 - HS DP/DM pulldown resistance select."]
    #[inline(always)]
    pub fn hstpulldown(&self) -> HSTPULLDOWN_R {
        HSTPULLDOWN_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - Enable Host pulldown"]
    #[inline(always)]
    pub fn enhstpulldown(&self) -> ENHSTPULLDOWN_R {
        ENHSTPULLDOWN_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 8:11 - TX2RXCOUNT"]
    #[inline(always)]
    pub fn tx2rxcount(&self) -> TX2RXCOUNT_R {
        TX2RXCOUNT_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bit 12 - ENTX2RXCOUNT"]
    #[inline(always)]
    pub fn entx2rxcount(&self) -> ENTX2RXCOUNT_R {
        ENTX2RXCOUNT_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bits 16:20 - Squelch reset count"]
    #[inline(always)]
    pub fn squelchresetcount(&self) -> SQUELCHRESETCOUNT_R {
        SQUELCHRESETCOUNT_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bit 24 - Enable squelch reset"]
    #[inline(always)]
    pub fn ensquelchreset(&self) -> ENSQUELCHRESET_R {
        ENSQUELCHRESET_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bits 25:28 - Squelch reset length"]
    #[inline(always)]
    pub fn squelchresetlength(&self) -> SQUELCHRESETLENGTH_R {
        SQUELCHRESETLENGTH_R::new(((self.bits >> 25) & 0x0f) as u8)
    }
    #[doc = "Bit 29 - Host resume"]
    #[inline(always)]
    pub fn host_resume_debug(&self) -> HOST_RESUME_DEBUG_R {
        HOST_RESUME_DEBUG_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Test clock gate"]
    #[inline(always)]
    pub fn clkgate(&self) -> CLKGATE_R {
        CLKGATE_R::new(((self.bits >> 30) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - Debug interface"]
    #[inline(always)]
    #[must_use]
    pub fn debug_interface_hold(&mut self) -> DEBUG_INTERFACE_HOLD_W<1> {
        DEBUG_INTERFACE_HOLD_W::new(self)
    }
    #[doc = "Bits 2:3 - HS DP/DM pulldown resistance select."]
    #[inline(always)]
    #[must_use]
    pub fn hstpulldown(&mut self) -> HSTPULLDOWN_W<2> {
        HSTPULLDOWN_W::new(self)
    }
    #[doc = "Bits 4:5 - Enable Host pulldown"]
    #[inline(always)]
    #[must_use]
    pub fn enhstpulldown(&mut self) -> ENHSTPULLDOWN_W<4> {
        ENHSTPULLDOWN_W::new(self)
    }
    #[doc = "Bits 8:11 - TX2RXCOUNT"]
    #[inline(always)]
    #[must_use]
    pub fn tx2rxcount(&mut self) -> TX2RXCOUNT_W<8> {
        TX2RXCOUNT_W::new(self)
    }
    #[doc = "Bit 12 - ENTX2RXCOUNT"]
    #[inline(always)]
    #[must_use]
    pub fn entx2rxcount(&mut self) -> ENTX2RXCOUNT_W<12> {
        ENTX2RXCOUNT_W::new(self)
    }
    #[doc = "Bits 16:20 - Squelch reset count"]
    #[inline(always)]
    #[must_use]
    pub fn squelchresetcount(&mut self) -> SQUELCHRESETCOUNT_W<16> {
        SQUELCHRESETCOUNT_W::new(self)
    }
    #[doc = "Bit 24 - Enable squelch reset"]
    #[inline(always)]
    #[must_use]
    pub fn ensquelchreset(&mut self) -> ENSQUELCHRESET_W<24> {
        ENSQUELCHRESET_W::new(self)
    }
    #[doc = "Bits 25:28 - Squelch reset length"]
    #[inline(always)]
    #[must_use]
    pub fn squelchresetlength(&mut self) -> SQUELCHRESETLENGTH_W<25> {
        SQUELCHRESETLENGTH_W::new(self)
    }
    #[doc = "Bit 29 - Host resume"]
    #[inline(always)]
    #[must_use]
    pub fn host_resume_debug(&mut self) -> HOST_RESUME_DEBUG_W<29> {
        HOST_RESUME_DEBUG_W::new(self)
    }
    #[doc = "Bit 30 - Test clock gate"]
    #[inline(always)]
    #[must_use]
    pub fn clkgate(&mut self) -> CLKGATE_W<30> {
        CLKGATE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Debug Toggle\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [debug0_tog](index.html) module"]
pub struct DEBUG0_TOG_SPEC;
impl crate::RegisterSpec for DEBUG0_TOG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [debug0_tog::R](R) reader structure"]
impl crate::Readable for DEBUG0_TOG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [debug0_tog::W](W) writer structure"]
impl crate::Writable for DEBUG0_TOG_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DEBUG0_TOG to value 0x7f18_0000"]
impl crate::Resettable for DEBUG0_TOG_SPEC {
    const RESET_VALUE: Self::Ux = 0x7f18_0000;
}
