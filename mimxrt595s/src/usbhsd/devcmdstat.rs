#[doc = "Register `DEVCMDSTAT` reader"]
pub struct R(crate::R<DEVCMDSTAT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DEVCMDSTAT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DEVCMDSTAT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DEVCMDSTAT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DEVCMDSTAT` writer"]
pub struct W(crate::W<DEVCMDSTAT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DEVCMDSTAT_SPEC>;
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
impl From<crate::W<DEVCMDSTAT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DEVCMDSTAT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DEV_ADDR` reader - USB Device Address"]
pub type DEV_ADDR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DEV_ADDR` writer - USB Device Address"]
pub type DEV_ADDR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DEVCMDSTAT_SPEC, u8, u8, 7, O>;
#[doc = "Field `DEV_EN` reader - USB device enable"]
pub type DEV_EN_R = crate::BitReader<bool>;
#[doc = "Field `DEV_EN` writer - USB device enable"]
pub type DEV_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, DEVCMDSTAT_SPEC, bool, O>;
#[doc = "Field `SETUP` reader - SETUP token received."]
pub type SETUP_R = crate::BitReader<bool>;
#[doc = "Field `SETUP` writer - SETUP token received."]
pub type SETUP_W<'a, const O: u8> = crate::BitWriter<'a, u32, DEVCMDSTAT_SPEC, bool, O>;
#[doc = "Field `FORCE_NEEDCLK` reader - Force the NEEDCLK output to always be on."]
pub type FORCE_NEEDCLK_R = crate::BitReader<FORCE_NEEDCLK_A>;
#[doc = "Force the NEEDCLK output to always be on.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FORCE_NEEDCLK_A {
    #[doc = "0: USB_NEEDCLK has normal function."]
    FORCE_NEEDCLK_0 = 0,
    #[doc = "1: USB_NEEDCLK always 1. Clock will not be stopped in case of suspend."]
    FORCE_NEEDCLK_1 = 1,
}
impl From<FORCE_NEEDCLK_A> for bool {
    #[inline(always)]
    fn from(variant: FORCE_NEEDCLK_A) -> Self {
        variant as u8 != 0
    }
}
impl FORCE_NEEDCLK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FORCE_NEEDCLK_A {
        match self.bits {
            false => FORCE_NEEDCLK_A::FORCE_NEEDCLK_0,
            true => FORCE_NEEDCLK_A::FORCE_NEEDCLK_1,
        }
    }
    #[doc = "Checks if the value of the field is `FORCE_NEEDCLK_0`"]
    #[inline(always)]
    pub fn is_force_needclk_0(&self) -> bool {
        *self == FORCE_NEEDCLK_A::FORCE_NEEDCLK_0
    }
    #[doc = "Checks if the value of the field is `FORCE_NEEDCLK_1`"]
    #[inline(always)]
    pub fn is_force_needclk_1(&self) -> bool {
        *self == FORCE_NEEDCLK_A::FORCE_NEEDCLK_1
    }
}
#[doc = "Field `FORCE_NEEDCLK` writer - Force the NEEDCLK output to always be on."]
pub type FORCE_NEEDCLK_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, DEVCMDSTAT_SPEC, FORCE_NEEDCLK_A, O>;
impl<'a, const O: u8> FORCE_NEEDCLK_W<'a, O> {
    #[doc = "USB_NEEDCLK has normal function."]
    #[inline(always)]
    pub fn force_needclk_0(self) -> &'a mut W {
        self.variant(FORCE_NEEDCLK_A::FORCE_NEEDCLK_0)
    }
    #[doc = "USB_NEEDCLK always 1. Clock will not be stopped in case of suspend."]
    #[inline(always)]
    pub fn force_needclk_1(self) -> &'a mut W {
        self.variant(FORCE_NEEDCLK_A::FORCE_NEEDCLK_1)
    }
}
#[doc = "Field `FORCE_VBUS` reader - Force VBUS"]
pub type FORCE_VBUS_R = crate::BitReader<bool>;
#[doc = "Field `FORCE_VBUS` writer - Force VBUS"]
pub type FORCE_VBUS_W<'a, const O: u8> = crate::BitWriter<'a, u32, DEVCMDSTAT_SPEC, bool, O>;
#[doc = "Field `LPM_SUP` reader - LPM Support"]
pub type LPM_SUP_R = crate::BitReader<LPM_SUP_A>;
#[doc = "LPM Support\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LPM_SUP_A {
    #[doc = "0: LPM not supported."]
    LPM_SUPP_0 = 0,
    #[doc = "1: LPM supported."]
    LPM_SUPP_1 = 1,
}
impl From<LPM_SUP_A> for bool {
    #[inline(always)]
    fn from(variant: LPM_SUP_A) -> Self {
        variant as u8 != 0
    }
}
impl LPM_SUP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LPM_SUP_A {
        match self.bits {
            false => LPM_SUP_A::LPM_SUPP_0,
            true => LPM_SUP_A::LPM_SUPP_1,
        }
    }
    #[doc = "Checks if the value of the field is `LPM_SUPP_0`"]
    #[inline(always)]
    pub fn is_lpm_supp_0(&self) -> bool {
        *self == LPM_SUP_A::LPM_SUPP_0
    }
    #[doc = "Checks if the value of the field is `LPM_SUPP_1`"]
    #[inline(always)]
    pub fn is_lpm_supp_1(&self) -> bool {
        *self == LPM_SUP_A::LPM_SUPP_1
    }
}
#[doc = "Field `LPM_SUP` writer - LPM Support"]
pub type LPM_SUP_W<'a, const O: u8> = crate::BitWriter<'a, u32, DEVCMDSTAT_SPEC, LPM_SUP_A, O>;
impl<'a, const O: u8> LPM_SUP_W<'a, O> {
    #[doc = "LPM not supported."]
    #[inline(always)]
    pub fn lpm_supp_0(self) -> &'a mut W {
        self.variant(LPM_SUP_A::LPM_SUPP_0)
    }
    #[doc = "LPM supported."]
    #[inline(always)]
    pub fn lpm_supp_1(self) -> &'a mut W {
        self.variant(LPM_SUP_A::LPM_SUPP_1)
    }
}
#[doc = "Field `INTONNAK_AO` reader - Interrupt on NAK for interrupt and bulk OUT EP"]
pub type INTONNAK_AO_R = crate::BitReader<INTONNAK_AO_A>;
#[doc = "Interrupt on NAK for interrupt and bulk OUT EP\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum INTONNAK_AO_A {
    #[doc = "0: Only acknowledged packets generate an interrupt"]
    INTONNAK_AO_0 = 0,
    #[doc = "1: Both acknowledged and NAKed packets generate interrupts."]
    INTONNAK_AO_1 = 1,
}
impl From<INTONNAK_AO_A> for bool {
    #[inline(always)]
    fn from(variant: INTONNAK_AO_A) -> Self {
        variant as u8 != 0
    }
}
impl INTONNAK_AO_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INTONNAK_AO_A {
        match self.bits {
            false => INTONNAK_AO_A::INTONNAK_AO_0,
            true => INTONNAK_AO_A::INTONNAK_AO_1,
        }
    }
    #[doc = "Checks if the value of the field is `INTONNAK_AO_0`"]
    #[inline(always)]
    pub fn is_intonnak_ao_0(&self) -> bool {
        *self == INTONNAK_AO_A::INTONNAK_AO_0
    }
    #[doc = "Checks if the value of the field is `INTONNAK_AO_1`"]
    #[inline(always)]
    pub fn is_intonnak_ao_1(&self) -> bool {
        *self == INTONNAK_AO_A::INTONNAK_AO_1
    }
}
#[doc = "Field `INTONNAK_AO` writer - Interrupt on NAK for interrupt and bulk OUT EP"]
pub type INTONNAK_AO_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, DEVCMDSTAT_SPEC, INTONNAK_AO_A, O>;
impl<'a, const O: u8> INTONNAK_AO_W<'a, O> {
    #[doc = "Only acknowledged packets generate an interrupt"]
    #[inline(always)]
    pub fn intonnak_ao_0(self) -> &'a mut W {
        self.variant(INTONNAK_AO_A::INTONNAK_AO_0)
    }
    #[doc = "Both acknowledged and NAKed packets generate interrupts."]
    #[inline(always)]
    pub fn intonnak_ao_1(self) -> &'a mut W {
        self.variant(INTONNAK_AO_A::INTONNAK_AO_1)
    }
}
#[doc = "Field `INTONNAK_AI` reader - Interrupt on NAK for interrupt and bulk IN EP"]
pub type INTONNAK_AI_R = crate::BitReader<INTONNAK_AI_A>;
#[doc = "Interrupt on NAK for interrupt and bulk IN EP\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum INTONNAK_AI_A {
    #[doc = "0: Only acknowledged packets generate an interrupt"]
    INTONNAK_AI_0 = 0,
    #[doc = "1: Both acknowledged and NAKed packets generate interrupts."]
    INTONNAK_AI_1 = 1,
}
impl From<INTONNAK_AI_A> for bool {
    #[inline(always)]
    fn from(variant: INTONNAK_AI_A) -> Self {
        variant as u8 != 0
    }
}
impl INTONNAK_AI_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INTONNAK_AI_A {
        match self.bits {
            false => INTONNAK_AI_A::INTONNAK_AI_0,
            true => INTONNAK_AI_A::INTONNAK_AI_1,
        }
    }
    #[doc = "Checks if the value of the field is `INTONNAK_AI_0`"]
    #[inline(always)]
    pub fn is_intonnak_ai_0(&self) -> bool {
        *self == INTONNAK_AI_A::INTONNAK_AI_0
    }
    #[doc = "Checks if the value of the field is `INTONNAK_AI_1`"]
    #[inline(always)]
    pub fn is_intonnak_ai_1(&self) -> bool {
        *self == INTONNAK_AI_A::INTONNAK_AI_1
    }
}
#[doc = "Field `INTONNAK_AI` writer - Interrupt on NAK for interrupt and bulk IN EP"]
pub type INTONNAK_AI_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, DEVCMDSTAT_SPEC, INTONNAK_AI_A, O>;
impl<'a, const O: u8> INTONNAK_AI_W<'a, O> {
    #[doc = "Only acknowledged packets generate an interrupt"]
    #[inline(always)]
    pub fn intonnak_ai_0(self) -> &'a mut W {
        self.variant(INTONNAK_AI_A::INTONNAK_AI_0)
    }
    #[doc = "Both acknowledged and NAKed packets generate interrupts."]
    #[inline(always)]
    pub fn intonnak_ai_1(self) -> &'a mut W {
        self.variant(INTONNAK_AI_A::INTONNAK_AI_1)
    }
}
#[doc = "Field `INTONNAK_CO` reader - Interrupt on NAK for interrupt and bulk OUT EP"]
pub type INTONNAK_CO_R = crate::BitReader<INTONNAK_CO_A>;
#[doc = "Interrupt on NAK for interrupt and bulk OUT EP\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum INTONNAK_CO_A {
    #[doc = "0: Only acknowledged packets generate an interrupt"]
    INTONNAK_CO_0 = 0,
    #[doc = "1: Both acknowledged and NAKed packets generate interrupts."]
    INTONNAK_CO_1 = 1,
}
impl From<INTONNAK_CO_A> for bool {
    #[inline(always)]
    fn from(variant: INTONNAK_CO_A) -> Self {
        variant as u8 != 0
    }
}
impl INTONNAK_CO_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INTONNAK_CO_A {
        match self.bits {
            false => INTONNAK_CO_A::INTONNAK_CO_0,
            true => INTONNAK_CO_A::INTONNAK_CO_1,
        }
    }
    #[doc = "Checks if the value of the field is `INTONNAK_CO_0`"]
    #[inline(always)]
    pub fn is_intonnak_co_0(&self) -> bool {
        *self == INTONNAK_CO_A::INTONNAK_CO_0
    }
    #[doc = "Checks if the value of the field is `INTONNAK_CO_1`"]
    #[inline(always)]
    pub fn is_intonnak_co_1(&self) -> bool {
        *self == INTONNAK_CO_A::INTONNAK_CO_1
    }
}
#[doc = "Field `INTONNAK_CO` writer - Interrupt on NAK for interrupt and bulk OUT EP"]
pub type INTONNAK_CO_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, DEVCMDSTAT_SPEC, INTONNAK_CO_A, O>;
impl<'a, const O: u8> INTONNAK_CO_W<'a, O> {
    #[doc = "Only acknowledged packets generate an interrupt"]
    #[inline(always)]
    pub fn intonnak_co_0(self) -> &'a mut W {
        self.variant(INTONNAK_CO_A::INTONNAK_CO_0)
    }
    #[doc = "Both acknowledged and NAKed packets generate interrupts."]
    #[inline(always)]
    pub fn intonnak_co_1(self) -> &'a mut W {
        self.variant(INTONNAK_CO_A::INTONNAK_CO_1)
    }
}
#[doc = "Field `INTONNAK_CI` reader - Interrupt on NAK for interrupt and bulk OUT EP"]
pub type INTONNAK_CI_R = crate::BitReader<INTONNAK_CI_A>;
#[doc = "Interrupt on NAK for interrupt and bulk OUT EP\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum INTONNAK_CI_A {
    #[doc = "0: Only acknowledged packets generate an interrupt"]
    INTONNAK_CI_0 = 0,
    #[doc = "1: Both acknowledged and NAKed packets generate interrupts."]
    INTONNAK_CI_1 = 1,
}
impl From<INTONNAK_CI_A> for bool {
    #[inline(always)]
    fn from(variant: INTONNAK_CI_A) -> Self {
        variant as u8 != 0
    }
}
impl INTONNAK_CI_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INTONNAK_CI_A {
        match self.bits {
            false => INTONNAK_CI_A::INTONNAK_CI_0,
            true => INTONNAK_CI_A::INTONNAK_CI_1,
        }
    }
    #[doc = "Checks if the value of the field is `INTONNAK_CI_0`"]
    #[inline(always)]
    pub fn is_intonnak_ci_0(&self) -> bool {
        *self == INTONNAK_CI_A::INTONNAK_CI_0
    }
    #[doc = "Checks if the value of the field is `INTONNAK_CI_1`"]
    #[inline(always)]
    pub fn is_intonnak_ci_1(&self) -> bool {
        *self == INTONNAK_CI_A::INTONNAK_CI_1
    }
}
#[doc = "Field `INTONNAK_CI` writer - Interrupt on NAK for interrupt and bulk OUT EP"]
pub type INTONNAK_CI_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, DEVCMDSTAT_SPEC, INTONNAK_CI_A, O>;
impl<'a, const O: u8> INTONNAK_CI_W<'a, O> {
    #[doc = "Only acknowledged packets generate an interrupt"]
    #[inline(always)]
    pub fn intonnak_ci_0(self) -> &'a mut W {
        self.variant(INTONNAK_CI_A::INTONNAK_CI_0)
    }
    #[doc = "Both acknowledged and NAKed packets generate interrupts."]
    #[inline(always)]
    pub fn intonnak_ci_1(self) -> &'a mut W {
        self.variant(INTONNAK_CI_A::INTONNAK_CI_1)
    }
}
#[doc = "Field `DCON` reader - Device status - connect."]
pub type DCON_R = crate::BitReader<bool>;
#[doc = "Field `DCON` writer - Device status - connect."]
pub type DCON_W<'a, const O: u8> = crate::BitWriter<'a, u32, DEVCMDSTAT_SPEC, bool, O>;
#[doc = "Field `DSUS` reader - Device status suspend."]
pub type DSUS_R = crate::BitReader<DSUS_A>;
#[doc = "Device status suspend.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DSUS_A {
    #[doc = "0: When the device is not connected or not suspended, a writing a 0 has no effect. Writing a 1 never has an effect."]
    DSUS_0 = 0,
    #[doc = "1: It is set to 1 when the device has not seen any activity on its upstream port for more than 3 ms. It is reset to 0 on any activity. When the device is suspended (Suspend bit DSUS = 1) and the software writes a 0 to it, the device will generate a remote wake-up. This will only happen when the device is connected (Connect bit = 1)."]
    DSUS_1 = 1,
}
impl From<DSUS_A> for bool {
    #[inline(always)]
    fn from(variant: DSUS_A) -> Self {
        variant as u8 != 0
    }
}
impl DSUS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DSUS_A {
        match self.bits {
            false => DSUS_A::DSUS_0,
            true => DSUS_A::DSUS_1,
        }
    }
    #[doc = "Checks if the value of the field is `DSUS_0`"]
    #[inline(always)]
    pub fn is_dsus_0(&self) -> bool {
        *self == DSUS_A::DSUS_0
    }
    #[doc = "Checks if the value of the field is `DSUS_1`"]
    #[inline(always)]
    pub fn is_dsus_1(&self) -> bool {
        *self == DSUS_A::DSUS_1
    }
}
#[doc = "Field `DSUS` writer - Device status suspend."]
pub type DSUS_W<'a, const O: u8> = crate::BitWriter<'a, u32, DEVCMDSTAT_SPEC, DSUS_A, O>;
impl<'a, const O: u8> DSUS_W<'a, O> {
    #[doc = "When the device is not connected or not suspended, a writing a 0 has no effect. Writing a 1 never has an effect."]
    #[inline(always)]
    pub fn dsus_0(self) -> &'a mut W {
        self.variant(DSUS_A::DSUS_0)
    }
    #[doc = "It is set to 1 when the device has not seen any activity on its upstream port for more than 3 ms. It is reset to 0 on any activity. When the device is suspended (Suspend bit DSUS = 1) and the software writes a 0 to it, the device will generate a remote wake-up. This will only happen when the device is connected (Connect bit = 1)."]
    #[inline(always)]
    pub fn dsus_1(self) -> &'a mut W {
        self.variant(DSUS_A::DSUS_1)
    }
}
#[doc = "Field `LPM_SUS` reader - Device status - LPM Suspend."]
pub type LPM_SUS_R = crate::BitReader<LPM_SUS_A>;
#[doc = "Device status - LPM Suspend.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LPM_SUS_A {
    #[doc = "0: Software can only write a 0 to this bit when the LPM_REWP bit is set to 1. Hardware resets this bit when it receives a host initiated resume. Hardware only updates the LPM_SUS bit when the LPM_SUPP bit is equal to 1."]
    LPM_SUS_0 = 0,
    #[doc = "1: When the device is in the LPM suspended state (LPM suspend bit = 1) and the software writes a 0 to this bit, the device will generate a remote walk-up."]
    LPM_SUS_1 = 1,
}
impl From<LPM_SUS_A> for bool {
    #[inline(always)]
    fn from(variant: LPM_SUS_A) -> Self {
        variant as u8 != 0
    }
}
impl LPM_SUS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LPM_SUS_A {
        match self.bits {
            false => LPM_SUS_A::LPM_SUS_0,
            true => LPM_SUS_A::LPM_SUS_1,
        }
    }
    #[doc = "Checks if the value of the field is `LPM_SUS_0`"]
    #[inline(always)]
    pub fn is_lpm_sus_0(&self) -> bool {
        *self == LPM_SUS_A::LPM_SUS_0
    }
    #[doc = "Checks if the value of the field is `LPM_SUS_1`"]
    #[inline(always)]
    pub fn is_lpm_sus_1(&self) -> bool {
        *self == LPM_SUS_A::LPM_SUS_1
    }
}
#[doc = "Field `LPM_SUS` writer - Device status - LPM Suspend."]
pub type LPM_SUS_W<'a, const O: u8> = crate::BitWriter<'a, u32, DEVCMDSTAT_SPEC, LPM_SUS_A, O>;
impl<'a, const O: u8> LPM_SUS_W<'a, O> {
    #[doc = "Software can only write a 0 to this bit when the LPM_REWP bit is set to 1. Hardware resets this bit when it receives a host initiated resume. Hardware only updates the LPM_SUS bit when the LPM_SUPP bit is equal to 1."]
    #[inline(always)]
    pub fn lpm_sus_0(self) -> &'a mut W {
        self.variant(LPM_SUS_A::LPM_SUS_0)
    }
    #[doc = "When the device is in the LPM suspended state (LPM suspend bit = 1) and the software writes a 0 to this bit, the device will generate a remote walk-up."]
    #[inline(always)]
    pub fn lpm_sus_1(self) -> &'a mut W {
        self.variant(LPM_SUS_A::LPM_SUS_1)
    }
}
#[doc = "Field `LPM_REWP` reader - LPM Remote Wake-up Enabled by USB host."]
pub type LPM_REWP_R = crate::BitReader<bool>;
#[doc = "Field `LPM_REWP` writer - LPM Remote Wake-up Enabled by USB host."]
pub type LPM_REWP_W<'a, const O: u8> = crate::BitWriter<'a, u32, DEVCMDSTAT_SPEC, bool, O>;
#[doc = "Field `SPEED` reader - This field indicates the speed at which the device operates."]
pub type SPEED_R = crate::FieldReader<u8, SPEED_A>;
#[doc = "This field indicates the speed at which the device operates.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SPEED_A {
    #[doc = "1: Full-speed"]
    SPEED_1 = 1,
    #[doc = "2: High-speed"]
    SPEED_2 = 2,
}
impl From<SPEED_A> for u8 {
    #[inline(always)]
    fn from(variant: SPEED_A) -> Self {
        variant as _
    }
}
impl SPEED_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<SPEED_A> {
        match self.bits {
            1 => Some(SPEED_A::SPEED_1),
            2 => Some(SPEED_A::SPEED_2),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `SPEED_1`"]
    #[inline(always)]
    pub fn is_speed_1(&self) -> bool {
        *self == SPEED_A::SPEED_1
    }
    #[doc = "Checks if the value of the field is `SPEED_2`"]
    #[inline(always)]
    pub fn is_speed_2(&self) -> bool {
        *self == SPEED_A::SPEED_2
    }
}
#[doc = "Field `DCON_C` reader - Device status - connect change."]
pub type DCON_C_R = crate::BitReader<bool>;
#[doc = "Field `DCON_C` writer - Device status - connect change."]
pub type DCON_C_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, DEVCMDSTAT_SPEC, bool, O>;
#[doc = "Field `DSUS_C` reader - Device status - suspend change."]
pub type DSUS_C_R = crate::BitReader<bool>;
#[doc = "Field `DSUS_C` writer - Device status - suspend change."]
pub type DSUS_C_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, DEVCMDSTAT_SPEC, bool, O>;
#[doc = "Field `DRES_C` reader - Device status - reset change."]
pub type DRES_C_R = crate::BitReader<bool>;
#[doc = "Field `DRES_C` writer - Device status - reset change."]
pub type DRES_C_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, DEVCMDSTAT_SPEC, bool, O>;
#[doc = "Field `VBUS_DEBOUNCED` reader - VBUS detect."]
pub type VBUS_DEBOUNCED_R = crate::BitReader<bool>;
#[doc = "Field `PHY_TEST_MODE` reader - PHY test mode"]
pub type PHY_TEST_MODE_R = crate::FieldReader<u8, PHY_TEST_MODE_A>;
#[doc = "PHY test mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PHY_TEST_MODE_A {
    #[doc = "0: Test mode disabled"]
    PHY_TEST_MODE_0 = 0,
    #[doc = "1: Test_J"]
    PHY_TEST_MODE_1 = 1,
    #[doc = "2: Test_K"]
    PHY_TEST_MODE_2 = 2,
    #[doc = "3: Test_SE0_NAK"]
    PHY_TEST_MODE_3 = 3,
    #[doc = "4: Test_Packet"]
    PHY_TEST_MODE_4 = 4,
    #[doc = "5: Test_Force_Enable"]
    PHY_TEST_MODE_5 = 5,
}
impl From<PHY_TEST_MODE_A> for u8 {
    #[inline(always)]
    fn from(variant: PHY_TEST_MODE_A) -> Self {
        variant as _
    }
}
impl PHY_TEST_MODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<PHY_TEST_MODE_A> {
        match self.bits {
            0 => Some(PHY_TEST_MODE_A::PHY_TEST_MODE_0),
            1 => Some(PHY_TEST_MODE_A::PHY_TEST_MODE_1),
            2 => Some(PHY_TEST_MODE_A::PHY_TEST_MODE_2),
            3 => Some(PHY_TEST_MODE_A::PHY_TEST_MODE_3),
            4 => Some(PHY_TEST_MODE_A::PHY_TEST_MODE_4),
            5 => Some(PHY_TEST_MODE_A::PHY_TEST_MODE_5),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `PHY_TEST_MODE_0`"]
    #[inline(always)]
    pub fn is_phy_test_mode_0(&self) -> bool {
        *self == PHY_TEST_MODE_A::PHY_TEST_MODE_0
    }
    #[doc = "Checks if the value of the field is `PHY_TEST_MODE_1`"]
    #[inline(always)]
    pub fn is_phy_test_mode_1(&self) -> bool {
        *self == PHY_TEST_MODE_A::PHY_TEST_MODE_1
    }
    #[doc = "Checks if the value of the field is `PHY_TEST_MODE_2`"]
    #[inline(always)]
    pub fn is_phy_test_mode_2(&self) -> bool {
        *self == PHY_TEST_MODE_A::PHY_TEST_MODE_2
    }
    #[doc = "Checks if the value of the field is `PHY_TEST_MODE_3`"]
    #[inline(always)]
    pub fn is_phy_test_mode_3(&self) -> bool {
        *self == PHY_TEST_MODE_A::PHY_TEST_MODE_3
    }
    #[doc = "Checks if the value of the field is `PHY_TEST_MODE_4`"]
    #[inline(always)]
    pub fn is_phy_test_mode_4(&self) -> bool {
        *self == PHY_TEST_MODE_A::PHY_TEST_MODE_4
    }
    #[doc = "Checks if the value of the field is `PHY_TEST_MODE_5`"]
    #[inline(always)]
    pub fn is_phy_test_mode_5(&self) -> bool {
        *self == PHY_TEST_MODE_A::PHY_TEST_MODE_5
    }
}
#[doc = "Field `PHY_TEST_MODE` writer - PHY test mode"]
pub type PHY_TEST_MODE_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, DEVCMDSTAT_SPEC, u8, PHY_TEST_MODE_A, 3, O>;
impl<'a, const O: u8> PHY_TEST_MODE_W<'a, O> {
    #[doc = "Test mode disabled"]
    #[inline(always)]
    pub fn phy_test_mode_0(self) -> &'a mut W {
        self.variant(PHY_TEST_MODE_A::PHY_TEST_MODE_0)
    }
    #[doc = "Test_J"]
    #[inline(always)]
    pub fn phy_test_mode_1(self) -> &'a mut W {
        self.variant(PHY_TEST_MODE_A::PHY_TEST_MODE_1)
    }
    #[doc = "Test_K"]
    #[inline(always)]
    pub fn phy_test_mode_2(self) -> &'a mut W {
        self.variant(PHY_TEST_MODE_A::PHY_TEST_MODE_2)
    }
    #[doc = "Test_SE0_NAK"]
    #[inline(always)]
    pub fn phy_test_mode_3(self) -> &'a mut W {
        self.variant(PHY_TEST_MODE_A::PHY_TEST_MODE_3)
    }
    #[doc = "Test_Packet"]
    #[inline(always)]
    pub fn phy_test_mode_4(self) -> &'a mut W {
        self.variant(PHY_TEST_MODE_A::PHY_TEST_MODE_4)
    }
    #[doc = "Test_Force_Enable"]
    #[inline(always)]
    pub fn phy_test_mode_5(self) -> &'a mut W {
        self.variant(PHY_TEST_MODE_A::PHY_TEST_MODE_5)
    }
}
impl R {
    #[doc = "Bits 0:6 - USB Device Address"]
    #[inline(always)]
    pub fn dev_addr(&self) -> DEV_ADDR_R {
        DEV_ADDR_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bit 7 - USB device enable"]
    #[inline(always)]
    pub fn dev_en(&self) -> DEV_EN_R {
        DEV_EN_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - SETUP token received."]
    #[inline(always)]
    pub fn setup(&self) -> SETUP_R {
        SETUP_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Force the NEEDCLK output to always be on."]
    #[inline(always)]
    pub fn force_needclk(&self) -> FORCE_NEEDCLK_R {
        FORCE_NEEDCLK_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Force VBUS"]
    #[inline(always)]
    pub fn force_vbus(&self) -> FORCE_VBUS_R {
        FORCE_VBUS_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - LPM Support"]
    #[inline(always)]
    pub fn lpm_sup(&self) -> LPM_SUP_R {
        LPM_SUP_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Interrupt on NAK for interrupt and bulk OUT EP"]
    #[inline(always)]
    pub fn intonnak_ao(&self) -> INTONNAK_AO_R {
        INTONNAK_AO_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Interrupt on NAK for interrupt and bulk IN EP"]
    #[inline(always)]
    pub fn intonnak_ai(&self) -> INTONNAK_AI_R {
        INTONNAK_AI_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Interrupt on NAK for interrupt and bulk OUT EP"]
    #[inline(always)]
    pub fn intonnak_co(&self) -> INTONNAK_CO_R {
        INTONNAK_CO_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Interrupt on NAK for interrupt and bulk OUT EP"]
    #[inline(always)]
    pub fn intonnak_ci(&self) -> INTONNAK_CI_R {
        INTONNAK_CI_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Device status - connect."]
    #[inline(always)]
    pub fn dcon(&self) -> DCON_R {
        DCON_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Device status suspend."]
    #[inline(always)]
    pub fn dsus(&self) -> DSUS_R {
        DSUS_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 19 - Device status - LPM Suspend."]
    #[inline(always)]
    pub fn lpm_sus(&self) -> LPM_SUS_R {
        LPM_SUS_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - LPM Remote Wake-up Enabled by USB host."]
    #[inline(always)]
    pub fn lpm_rewp(&self) -> LPM_REWP_R {
        LPM_REWP_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bits 22:23 - This field indicates the speed at which the device operates."]
    #[inline(always)]
    pub fn speed(&self) -> SPEED_R {
        SPEED_R::new(((self.bits >> 22) & 3) as u8)
    }
    #[doc = "Bit 24 - Device status - connect change."]
    #[inline(always)]
    pub fn dcon_c(&self) -> DCON_C_R {
        DCON_C_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Device status - suspend change."]
    #[inline(always)]
    pub fn dsus_c(&self) -> DSUS_C_R {
        DSUS_C_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Device status - reset change."]
    #[inline(always)]
    pub fn dres_c(&self) -> DRES_C_R {
        DRES_C_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 28 - VBUS detect."]
    #[inline(always)]
    pub fn vbus_debounced(&self) -> VBUS_DEBOUNCED_R {
        VBUS_DEBOUNCED_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bits 29:31 - PHY test mode"]
    #[inline(always)]
    pub fn phy_test_mode(&self) -> PHY_TEST_MODE_R {
        PHY_TEST_MODE_R::new(((self.bits >> 29) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - USB Device Address"]
    #[inline(always)]
    #[must_use]
    pub fn dev_addr(&mut self) -> DEV_ADDR_W<0> {
        DEV_ADDR_W::new(self)
    }
    #[doc = "Bit 7 - USB device enable"]
    #[inline(always)]
    #[must_use]
    pub fn dev_en(&mut self) -> DEV_EN_W<7> {
        DEV_EN_W::new(self)
    }
    #[doc = "Bit 8 - SETUP token received."]
    #[inline(always)]
    #[must_use]
    pub fn setup(&mut self) -> SETUP_W<8> {
        SETUP_W::new(self)
    }
    #[doc = "Bit 9 - Force the NEEDCLK output to always be on."]
    #[inline(always)]
    #[must_use]
    pub fn force_needclk(&mut self) -> FORCE_NEEDCLK_W<9> {
        FORCE_NEEDCLK_W::new(self)
    }
    #[doc = "Bit 10 - Force VBUS"]
    #[inline(always)]
    #[must_use]
    pub fn force_vbus(&mut self) -> FORCE_VBUS_W<10> {
        FORCE_VBUS_W::new(self)
    }
    #[doc = "Bit 11 - LPM Support"]
    #[inline(always)]
    #[must_use]
    pub fn lpm_sup(&mut self) -> LPM_SUP_W<11> {
        LPM_SUP_W::new(self)
    }
    #[doc = "Bit 12 - Interrupt on NAK for interrupt and bulk OUT EP"]
    #[inline(always)]
    #[must_use]
    pub fn intonnak_ao(&mut self) -> INTONNAK_AO_W<12> {
        INTONNAK_AO_W::new(self)
    }
    #[doc = "Bit 13 - Interrupt on NAK for interrupt and bulk IN EP"]
    #[inline(always)]
    #[must_use]
    pub fn intonnak_ai(&mut self) -> INTONNAK_AI_W<13> {
        INTONNAK_AI_W::new(self)
    }
    #[doc = "Bit 14 - Interrupt on NAK for interrupt and bulk OUT EP"]
    #[inline(always)]
    #[must_use]
    pub fn intonnak_co(&mut self) -> INTONNAK_CO_W<14> {
        INTONNAK_CO_W::new(self)
    }
    #[doc = "Bit 15 - Interrupt on NAK for interrupt and bulk OUT EP"]
    #[inline(always)]
    #[must_use]
    pub fn intonnak_ci(&mut self) -> INTONNAK_CI_W<15> {
        INTONNAK_CI_W::new(self)
    }
    #[doc = "Bit 16 - Device status - connect."]
    #[inline(always)]
    #[must_use]
    pub fn dcon(&mut self) -> DCON_W<16> {
        DCON_W::new(self)
    }
    #[doc = "Bit 17 - Device status suspend."]
    #[inline(always)]
    #[must_use]
    pub fn dsus(&mut self) -> DSUS_W<17> {
        DSUS_W::new(self)
    }
    #[doc = "Bit 19 - Device status - LPM Suspend."]
    #[inline(always)]
    #[must_use]
    pub fn lpm_sus(&mut self) -> LPM_SUS_W<19> {
        LPM_SUS_W::new(self)
    }
    #[doc = "Bit 20 - LPM Remote Wake-up Enabled by USB host."]
    #[inline(always)]
    #[must_use]
    pub fn lpm_rewp(&mut self) -> LPM_REWP_W<20> {
        LPM_REWP_W::new(self)
    }
    #[doc = "Bit 24 - Device status - connect change."]
    #[inline(always)]
    #[must_use]
    pub fn dcon_c(&mut self) -> DCON_C_W<24> {
        DCON_C_W::new(self)
    }
    #[doc = "Bit 25 - Device status - suspend change."]
    #[inline(always)]
    #[must_use]
    pub fn dsus_c(&mut self) -> DSUS_C_W<25> {
        DSUS_C_W::new(self)
    }
    #[doc = "Bit 26 - Device status - reset change."]
    #[inline(always)]
    #[must_use]
    pub fn dres_c(&mut self) -> DRES_C_W<26> {
        DRES_C_W::new(self)
    }
    #[doc = "Bits 29:31 - PHY test mode"]
    #[inline(always)]
    #[must_use]
    pub fn phy_test_mode(&mut self) -> PHY_TEST_MODE_W<29> {
        PHY_TEST_MODE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "USB Device Command/Status\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [devcmdstat](index.html) module"]
pub struct DEVCMDSTAT_SPEC;
impl crate::RegisterSpec for DEVCMDSTAT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [devcmdstat::R](R) reader structure"]
impl crate::Readable for DEVCMDSTAT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [devcmdstat::W](W) writer structure"]
impl crate::Writable for DEVCMDSTAT_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0x0700_0000;
}
#[doc = "`reset()` method sets DEVCMDSTAT to value 0x0800"]
impl crate::Resettable for DEVCMDSTAT_SPEC {
    const RESET_VALUE: Self::Ux = 0x0800;
}
