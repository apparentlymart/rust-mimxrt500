#[doc = "Register `CTRL_TOG` reader"]
pub struct R(crate::R<CTRL_TOG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CTRL_TOG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CTRL_TOG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CTRL_TOG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CTRL_TOG` writer"]
pub struct W(crate::W<CTRL_TOG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CTRL_TOG_SPEC>;
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
impl From<crate::W<CTRL_TOG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CTRL_TOG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ENHOSTDISCONDETECT` reader - Disconnect detect."]
pub type ENHOSTDISCONDETECT_R = crate::BitReader<ENHOSTDISCONDETECT_A>;
#[doc = "Disconnect detect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ENHOSTDISCONDETECT_A {
    #[doc = "0: No effect"]
    DISABLE = 0,
    #[doc = "1: Toggles the corresponding CTRL bit"]
    ENABLE = 1,
}
impl From<ENHOSTDISCONDETECT_A> for bool {
    #[inline(always)]
    fn from(variant: ENHOSTDISCONDETECT_A) -> Self {
        variant as u8 != 0
    }
}
impl ENHOSTDISCONDETECT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ENHOSTDISCONDETECT_A {
        match self.bits {
            false => ENHOSTDISCONDETECT_A::DISABLE,
            true => ENHOSTDISCONDETECT_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == ENHOSTDISCONDETECT_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == ENHOSTDISCONDETECT_A::ENABLE
    }
}
#[doc = "Field `ENHOSTDISCONDETECT` writer - Disconnect detect."]
pub type ENHOSTDISCONDETECT_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, CTRL_TOG_SPEC, ENHOSTDISCONDETECT_A, O>;
impl<'a, const O: u8> ENHOSTDISCONDETECT_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(ENHOSTDISCONDETECT_A::DISABLE)
    }
    #[doc = "Toggles the corresponding CTRL bit"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(ENHOSTDISCONDETECT_A::ENABLE)
    }
}
#[doc = "Field `HOSTDISCONDETECT_IRQ` reader - Device disconnect indication."]
pub type HOSTDISCONDETECT_IRQ_R = crate::BitReader<HOSTDISCONDETECT_IRQ_A>;
#[doc = "Device disconnect indication.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HOSTDISCONDETECT_IRQ_A {
    #[doc = "0: No effect"]
    DISABLE = 0,
    #[doc = "1: Toggles the corresponding CTRL bit"]
    ENABLE = 1,
}
impl From<HOSTDISCONDETECT_IRQ_A> for bool {
    #[inline(always)]
    fn from(variant: HOSTDISCONDETECT_IRQ_A) -> Self {
        variant as u8 != 0
    }
}
impl HOSTDISCONDETECT_IRQ_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HOSTDISCONDETECT_IRQ_A {
        match self.bits {
            false => HOSTDISCONDETECT_IRQ_A::DISABLE,
            true => HOSTDISCONDETECT_IRQ_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == HOSTDISCONDETECT_IRQ_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == HOSTDISCONDETECT_IRQ_A::ENABLE
    }
}
#[doc = "Field `HOSTDISCONDETECT_IRQ` writer - Device disconnect indication."]
pub type HOSTDISCONDETECT_IRQ_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, CTRL_TOG_SPEC, HOSTDISCONDETECT_IRQ_A, O>;
impl<'a, const O: u8> HOSTDISCONDETECT_IRQ_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(HOSTDISCONDETECT_IRQ_A::DISABLE)
    }
    #[doc = "Toggles the corresponding CTRL bit"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(HOSTDISCONDETECT_IRQ_A::ENABLE)
    }
}
#[doc = "Field `ENDEVPLUGINDET` reader - Enables non-standard resistive plugged-in detection."]
pub type ENDEVPLUGINDET_R = crate::BitReader<ENDEVPLUGINDET_A>;
#[doc = "Enables non-standard resistive plugged-in detection.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ENDEVPLUGINDET_A {
    #[doc = "0: No effect"]
    DISABLE = 0,
    #[doc = "1: Toggles the corresponding CTRL bit"]
    ENABLE = 1,
}
impl From<ENDEVPLUGINDET_A> for bool {
    #[inline(always)]
    fn from(variant: ENDEVPLUGINDET_A) -> Self {
        variant as u8 != 0
    }
}
impl ENDEVPLUGINDET_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ENDEVPLUGINDET_A {
        match self.bits {
            false => ENDEVPLUGINDET_A::DISABLE,
            true => ENDEVPLUGINDET_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == ENDEVPLUGINDET_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == ENDEVPLUGINDET_A::ENABLE
    }
}
#[doc = "Field `ENDEVPLUGINDET` writer - Enables non-standard resistive plugged-in detection."]
pub type ENDEVPLUGINDET_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, CTRL_TOG_SPEC, ENDEVPLUGINDET_A, O>;
impl<'a, const O: u8> ENDEVPLUGINDET_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(ENDEVPLUGINDET_A::DISABLE)
    }
    #[doc = "Toggles the corresponding CTRL bit"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(ENDEVPLUGINDET_A::ENABLE)
    }
}
#[doc = "Field `DEVPLUGIN_IRQ` reader - Device connected indicator"]
pub type DEVPLUGIN_IRQ_R = crate::BitReader<DEVPLUGIN_IRQ_A>;
#[doc = "Device connected indicator\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DEVPLUGIN_IRQ_A {
    #[doc = "0: No effect"]
    DISABLE = 0,
    #[doc = "1: Toggles the corresponding CTRL bit"]
    ENABLE = 1,
}
impl From<DEVPLUGIN_IRQ_A> for bool {
    #[inline(always)]
    fn from(variant: DEVPLUGIN_IRQ_A) -> Self {
        variant as u8 != 0
    }
}
impl DEVPLUGIN_IRQ_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DEVPLUGIN_IRQ_A {
        match self.bits {
            false => DEVPLUGIN_IRQ_A::DISABLE,
            true => DEVPLUGIN_IRQ_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == DEVPLUGIN_IRQ_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == DEVPLUGIN_IRQ_A::ENABLE
    }
}
#[doc = "Field `DEVPLUGIN_IRQ` writer - Device connected indicator"]
pub type DEVPLUGIN_IRQ_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, CTRL_TOG_SPEC, DEVPLUGIN_IRQ_A, O>;
impl<'a, const O: u8> DEVPLUGIN_IRQ_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(DEVPLUGIN_IRQ_A::DISABLE)
    }
    #[doc = "Toggles the corresponding CTRL bit"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(DEVPLUGIN_IRQ_A::ENABLE)
    }
}
#[doc = "Field `ENUTMILEVEL2` reader - Enable level 2 operation"]
pub type ENUTMILEVEL2_R = crate::BitReader<ENUTMILEVEL2_A>;
#[doc = "Enable level 2 operation\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ENUTMILEVEL2_A {
    #[doc = "0: No effect"]
    DISABLE = 0,
    #[doc = "1: Toggles the corresponding CTRL bit"]
    ENABLE = 1,
}
impl From<ENUTMILEVEL2_A> for bool {
    #[inline(always)]
    fn from(variant: ENUTMILEVEL2_A) -> Self {
        variant as u8 != 0
    }
}
impl ENUTMILEVEL2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ENUTMILEVEL2_A {
        match self.bits {
            false => ENUTMILEVEL2_A::DISABLE,
            true => ENUTMILEVEL2_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == ENUTMILEVEL2_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == ENUTMILEVEL2_A::ENABLE
    }
}
#[doc = "Field `ENUTMILEVEL2` writer - Enable level 2 operation"]
pub type ENUTMILEVEL2_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, CTRL_TOG_SPEC, ENUTMILEVEL2_A, O>;
impl<'a, const O: u8> ENUTMILEVEL2_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(ENUTMILEVEL2_A::DISABLE)
    }
    #[doc = "Toggles the corresponding CTRL bit"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(ENUTMILEVEL2_A::ENABLE)
    }
}
#[doc = "Field `ENUTMILEVEL3` reader - Enable level 2 operation"]
pub type ENUTMILEVEL3_R = crate::BitReader<ENUTMILEVEL3_A>;
#[doc = "Enable level 2 operation\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ENUTMILEVEL3_A {
    #[doc = "0: No effect"]
    DISABLE = 0,
    #[doc = "1: Toggles the corresponding CTRL bit"]
    ENABLE = 1,
}
impl From<ENUTMILEVEL3_A> for bool {
    #[inline(always)]
    fn from(variant: ENUTMILEVEL3_A) -> Self {
        variant as u8 != 0
    }
}
impl ENUTMILEVEL3_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ENUTMILEVEL3_A {
        match self.bits {
            false => ENUTMILEVEL3_A::DISABLE,
            true => ENUTMILEVEL3_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == ENUTMILEVEL3_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == ENUTMILEVEL3_A::ENABLE
    }
}
#[doc = "Field `ENUTMILEVEL3` writer - Enable level 2 operation"]
pub type ENUTMILEVEL3_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, CTRL_TOG_SPEC, ENUTMILEVEL3_A, O>;
impl<'a, const O: u8> ENUTMILEVEL3_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(ENUTMILEVEL3_A::DISABLE)
    }
    #[doc = "Toggles the corresponding CTRL bit"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(ENUTMILEVEL3_A::ENABLE)
    }
}
#[doc = "Field `AUTORESUME_EN` reader - Enable autoresume"]
pub type AUTORESUME_EN_R = crate::BitReader<AUTORESUME_EN_A>;
#[doc = "Enable autoresume\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AUTORESUME_EN_A {
    #[doc = "0: No effect"]
    DISABLE = 0,
    #[doc = "1: Toggles the corresponding CTRL bit"]
    ENABLE = 1,
}
impl From<AUTORESUME_EN_A> for bool {
    #[inline(always)]
    fn from(variant: AUTORESUME_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl AUTORESUME_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AUTORESUME_EN_A {
        match self.bits {
            false => AUTORESUME_EN_A::DISABLE,
            true => AUTORESUME_EN_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == AUTORESUME_EN_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == AUTORESUME_EN_A::ENABLE
    }
}
#[doc = "Field `AUTORESUME_EN` writer - Enable autoresume"]
pub type AUTORESUME_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, CTRL_TOG_SPEC, AUTORESUME_EN_A, O>;
impl<'a, const O: u8> AUTORESUME_EN_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(AUTORESUME_EN_A::DISABLE)
    }
    #[doc = "Toggles the corresponding CTRL bit"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(AUTORESUME_EN_A::ENABLE)
    }
}
#[doc = "Field `ENAUTOCLR_CLKGATE` reader - Autoclear clock gate."]
pub type ENAUTOCLR_CLKGATE_R = crate::BitReader<ENAUTOCLR_CLKGATE_A>;
#[doc = "Autoclear clock gate.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ENAUTOCLR_CLKGATE_A {
    #[doc = "0: No effect"]
    DISABLE = 0,
    #[doc = "1: the corresponding CTRL bit"]
    ENABLE = 1,
}
impl From<ENAUTOCLR_CLKGATE_A> for bool {
    #[inline(always)]
    fn from(variant: ENAUTOCLR_CLKGATE_A) -> Self {
        variant as u8 != 0
    }
}
impl ENAUTOCLR_CLKGATE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ENAUTOCLR_CLKGATE_A {
        match self.bits {
            false => ENAUTOCLR_CLKGATE_A::DISABLE,
            true => ENAUTOCLR_CLKGATE_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == ENAUTOCLR_CLKGATE_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == ENAUTOCLR_CLKGATE_A::ENABLE
    }
}
#[doc = "Field `ENAUTOCLR_CLKGATE` writer - Autoclear clock gate."]
pub type ENAUTOCLR_CLKGATE_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, CTRL_TOG_SPEC, ENAUTOCLR_CLKGATE_A, O>;
impl<'a, const O: u8> ENAUTOCLR_CLKGATE_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(ENAUTOCLR_CLKGATE_A::DISABLE)
    }
    #[doc = "the corresponding CTRL bit"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(ENAUTOCLR_CLKGATE_A::ENABLE)
    }
}
#[doc = "Field `ENAUTOCLR_PHY_PWD` reader - Autoclear PWD register bits."]
pub type ENAUTOCLR_PHY_PWD_R = crate::BitReader<ENAUTOCLR_PHY_PWD_A>;
#[doc = "Autoclear PWD register bits.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ENAUTOCLR_PHY_PWD_A {
    #[doc = "0: No effect"]
    DISABLE = 0,
    #[doc = "1: Toggles the corresponding CTRL bit"]
    ENABLE = 1,
}
impl From<ENAUTOCLR_PHY_PWD_A> for bool {
    #[inline(always)]
    fn from(variant: ENAUTOCLR_PHY_PWD_A) -> Self {
        variant as u8 != 0
    }
}
impl ENAUTOCLR_PHY_PWD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ENAUTOCLR_PHY_PWD_A {
        match self.bits {
            false => ENAUTOCLR_PHY_PWD_A::DISABLE,
            true => ENAUTOCLR_PHY_PWD_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == ENAUTOCLR_PHY_PWD_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == ENAUTOCLR_PHY_PWD_A::ENABLE
    }
}
#[doc = "Field `ENAUTOCLR_PHY_PWD` writer - Autoclear PWD register bits."]
pub type ENAUTOCLR_PHY_PWD_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, CTRL_TOG_SPEC, ENAUTOCLR_PHY_PWD_A, O>;
impl<'a, const O: u8> ENAUTOCLR_PHY_PWD_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(ENAUTOCLR_PHY_PWD_A::DISABLE)
    }
    #[doc = "Toggles the corresponding CTRL bit"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(ENAUTOCLR_PHY_PWD_A::ENABLE)
    }
}
#[doc = "Field `FSDLL_RST_EN` reader - Reset FSDLL lock"]
pub type FSDLL_RST_EN_R = crate::BitReader<FSDLL_RST_EN_A>;
#[doc = "Reset FSDLL lock\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FSDLL_RST_EN_A {
    #[doc = "0: No effect"]
    DISABLE = 0,
    #[doc = "1: Toggles the corresponding CTRL bit"]
    ENABLE = 1,
}
impl From<FSDLL_RST_EN_A> for bool {
    #[inline(always)]
    fn from(variant: FSDLL_RST_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl FSDLL_RST_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FSDLL_RST_EN_A {
        match self.bits {
            false => FSDLL_RST_EN_A::DISABLE,
            true => FSDLL_RST_EN_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == FSDLL_RST_EN_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == FSDLL_RST_EN_A::ENABLE
    }
}
#[doc = "Field `FSDLL_RST_EN` writer - Reset FSDLL lock"]
pub type FSDLL_RST_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, CTRL_TOG_SPEC, FSDLL_RST_EN_A, O>;
impl<'a, const O: u8> FSDLL_RST_EN_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(FSDLL_RST_EN_A::DISABLE)
    }
    #[doc = "Toggles the corresponding CTRL bit"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(FSDLL_RST_EN_A::ENABLE)
    }
}
#[doc = "Field `HOST_FORCE_LS_SE0` reader - FS EOP low-speed timing"]
pub type HOST_FORCE_LS_SE0_R = crate::BitReader<bool>;
#[doc = "Field `HOST_FORCE_LS_SE0` writer - FS EOP low-speed timing"]
pub type HOST_FORCE_LS_SE0_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_TOG_SPEC, bool, O>;
#[doc = "Field `UTMI_SUSPENDM` reader - UTMI suspend"]
pub type UTMI_SUSPENDM_R = crate::BitReader<UTMI_SUSPENDM_A>;
#[doc = "UTMI suspend\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UTMI_SUSPENDM_A {
    #[doc = "0: No effect"]
    DISABLE = 0,
    #[doc = "1: Toggles the corresponding CTRL bit"]
    ENABLE = 1,
}
impl From<UTMI_SUSPENDM_A> for bool {
    #[inline(always)]
    fn from(variant: UTMI_SUSPENDM_A) -> Self {
        variant as u8 != 0
    }
}
impl UTMI_SUSPENDM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> UTMI_SUSPENDM_A {
        match self.bits {
            false => UTMI_SUSPENDM_A::DISABLE,
            true => UTMI_SUSPENDM_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == UTMI_SUSPENDM_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == UTMI_SUSPENDM_A::ENABLE
    }
}
#[doc = "Field `CLKGATE` reader - UTMI clock gate"]
pub type CLKGATE_R = crate::BitReader<CLKGATE_A>;
#[doc = "UTMI clock gate\n\nValue on reset: 1"]
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
#[doc = "Field `CLKGATE` writer - UTMI clock gate"]
pub type CLKGATE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_TOG_SPEC, CLKGATE_A, O>;
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
#[doc = "Field `SFTRST` reader - Software reset"]
pub type SFTRST_R = crate::BitReader<SFTRST_A>;
#[doc = "Software reset\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SFTRST_A {
    #[doc = "0: No effect"]
    DISABLE = 0,
    #[doc = "1: Toggles the corresponding CTRL bit"]
    ENABLE = 1,
}
impl From<SFTRST_A> for bool {
    #[inline(always)]
    fn from(variant: SFTRST_A) -> Self {
        variant as u8 != 0
    }
}
impl SFTRST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SFTRST_A {
        match self.bits {
            false => SFTRST_A::DISABLE,
            true => SFTRST_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == SFTRST_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == SFTRST_A::ENABLE
    }
}
#[doc = "Field `SFTRST` writer - Software reset"]
pub type SFTRST_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_TOG_SPEC, SFTRST_A, O>;
impl<'a, const O: u8> SFTRST_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(SFTRST_A::DISABLE)
    }
    #[doc = "Toggles the corresponding CTRL bit"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(SFTRST_A::ENABLE)
    }
}
impl R {
    #[doc = "Bit 1 - Disconnect detect."]
    #[inline(always)]
    pub fn enhostdiscondetect(&self) -> ENHOSTDISCONDETECT_R {
        ENHOSTDISCONDETECT_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 3 - Device disconnect indication."]
    #[inline(always)]
    pub fn hostdiscondetect_irq(&self) -> HOSTDISCONDETECT_IRQ_R {
        HOSTDISCONDETECT_IRQ_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Enables non-standard resistive plugged-in detection."]
    #[inline(always)]
    pub fn endevplugindet(&self) -> ENDEVPLUGINDET_R {
        ENDEVPLUGINDET_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 12 - Device connected indicator"]
    #[inline(always)]
    pub fn devplugin_irq(&self) -> DEVPLUGIN_IRQ_R {
        DEVPLUGIN_IRQ_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 14 - Enable level 2 operation"]
    #[inline(always)]
    pub fn enutmilevel2(&self) -> ENUTMILEVEL2_R {
        ENUTMILEVEL2_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Enable level 2 operation"]
    #[inline(always)]
    pub fn enutmilevel3(&self) -> ENUTMILEVEL3_R {
        ENUTMILEVEL3_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 18 - Enable autoresume"]
    #[inline(always)]
    pub fn autoresume_en(&self) -> AUTORESUME_EN_R {
        AUTORESUME_EN_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Autoclear clock gate."]
    #[inline(always)]
    pub fn enautoclr_clkgate(&self) -> ENAUTOCLR_CLKGATE_R {
        ENAUTOCLR_CLKGATE_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Autoclear PWD register bits."]
    #[inline(always)]
    pub fn enautoclr_phy_pwd(&self) -> ENAUTOCLR_PHY_PWD_R {
        ENAUTOCLR_PHY_PWD_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 24 - Reset FSDLL lock"]
    #[inline(always)]
    pub fn fsdll_rst_en(&self) -> FSDLL_RST_EN_R {
        FSDLL_RST_EN_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 28 - FS EOP low-speed timing"]
    #[inline(always)]
    pub fn host_force_ls_se0(&self) -> HOST_FORCE_LS_SE0_R {
        HOST_FORCE_LS_SE0_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - UTMI suspend"]
    #[inline(always)]
    pub fn utmi_suspendm(&self) -> UTMI_SUSPENDM_R {
        UTMI_SUSPENDM_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - UTMI clock gate"]
    #[inline(always)]
    pub fn clkgate(&self) -> CLKGATE_R {
        CLKGATE_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Software reset"]
    #[inline(always)]
    pub fn sftrst(&self) -> SFTRST_R {
        SFTRST_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - Disconnect detect."]
    #[inline(always)]
    #[must_use]
    pub fn enhostdiscondetect(&mut self) -> ENHOSTDISCONDETECT_W<1> {
        ENHOSTDISCONDETECT_W::new(self)
    }
    #[doc = "Bit 3 - Device disconnect indication."]
    #[inline(always)]
    #[must_use]
    pub fn hostdiscondetect_irq(&mut self) -> HOSTDISCONDETECT_IRQ_W<3> {
        HOSTDISCONDETECT_IRQ_W::new(self)
    }
    #[doc = "Bit 4 - Enables non-standard resistive plugged-in detection."]
    #[inline(always)]
    #[must_use]
    pub fn endevplugindet(&mut self) -> ENDEVPLUGINDET_W<4> {
        ENDEVPLUGINDET_W::new(self)
    }
    #[doc = "Bit 12 - Device connected indicator"]
    #[inline(always)]
    #[must_use]
    pub fn devplugin_irq(&mut self) -> DEVPLUGIN_IRQ_W<12> {
        DEVPLUGIN_IRQ_W::new(self)
    }
    #[doc = "Bit 14 - Enable level 2 operation"]
    #[inline(always)]
    #[must_use]
    pub fn enutmilevel2(&mut self) -> ENUTMILEVEL2_W<14> {
        ENUTMILEVEL2_W::new(self)
    }
    #[doc = "Bit 15 - Enable level 2 operation"]
    #[inline(always)]
    #[must_use]
    pub fn enutmilevel3(&mut self) -> ENUTMILEVEL3_W<15> {
        ENUTMILEVEL3_W::new(self)
    }
    #[doc = "Bit 18 - Enable autoresume"]
    #[inline(always)]
    #[must_use]
    pub fn autoresume_en(&mut self) -> AUTORESUME_EN_W<18> {
        AUTORESUME_EN_W::new(self)
    }
    #[doc = "Bit 19 - Autoclear clock gate."]
    #[inline(always)]
    #[must_use]
    pub fn enautoclr_clkgate(&mut self) -> ENAUTOCLR_CLKGATE_W<19> {
        ENAUTOCLR_CLKGATE_W::new(self)
    }
    #[doc = "Bit 20 - Autoclear PWD register bits."]
    #[inline(always)]
    #[must_use]
    pub fn enautoclr_phy_pwd(&mut self) -> ENAUTOCLR_PHY_PWD_W<20> {
        ENAUTOCLR_PHY_PWD_W::new(self)
    }
    #[doc = "Bit 24 - Reset FSDLL lock"]
    #[inline(always)]
    #[must_use]
    pub fn fsdll_rst_en(&mut self) -> FSDLL_RST_EN_W<24> {
        FSDLL_RST_EN_W::new(self)
    }
    #[doc = "Bit 28 - FS EOP low-speed timing"]
    #[inline(always)]
    #[must_use]
    pub fn host_force_ls_se0(&mut self) -> HOST_FORCE_LS_SE0_W<28> {
        HOST_FORCE_LS_SE0_W::new(self)
    }
    #[doc = "Bit 30 - UTMI clock gate"]
    #[inline(always)]
    #[must_use]
    pub fn clkgate(&mut self) -> CLKGATE_W<30> {
        CLKGATE_W::new(self)
    }
    #[doc = "Bit 31 - Software reset"]
    #[inline(always)]
    #[must_use]
    pub fn sftrst(&mut self) -> SFTRST_W<31> {
        SFTRST_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "General Purpose Control Toggle\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctrl_tog](index.html) module"]
pub struct CTRL_TOG_SPEC;
impl crate::RegisterSpec for CTRL_TOG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ctrl_tog::R](R) reader structure"]
impl crate::Readable for CTRL_TOG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ctrl_tog::W](W) writer structure"]
impl crate::Writable for CTRL_TOG_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CTRL_TOG to value 0xc000_0000"]
impl crate::Resettable for CTRL_TOG_SPEC {
    const RESET_VALUE: Self::Ux = 0xc000_0000;
}
