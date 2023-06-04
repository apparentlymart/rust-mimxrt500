#[doc = "Register `INTSTAT` reader"]
pub struct R(crate::R<INTSTAT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INTSTAT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INTSTAT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INTSTAT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `MSTPENDING` reader - Master Pending"]
pub type MSTPENDING_R = crate::BitReader<MSTPENDING_A>;
#[doc = "Master Pending\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MSTPENDING_A {
    #[doc = "0: Not active"]
    MSTPENDING_ISNOTACTIVE = 0,
    #[doc = "1: Active"]
    MSTPENDING_ISACTIVE = 1,
}
impl From<MSTPENDING_A> for bool {
    #[inline(always)]
    fn from(variant: MSTPENDING_A) -> Self {
        variant as u8 != 0
    }
}
impl MSTPENDING_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MSTPENDING_A {
        match self.bits {
            false => MSTPENDING_A::MSTPENDING_ISNOTACTIVE,
            true => MSTPENDING_A::MSTPENDING_ISACTIVE,
        }
    }
    #[doc = "Checks if the value of the field is `MSTPENDING_ISNOTACTIVE`"]
    #[inline(always)]
    pub fn is_mstpending_isnotactive(&self) -> bool {
        *self == MSTPENDING_A::MSTPENDING_ISNOTACTIVE
    }
    #[doc = "Checks if the value of the field is `MSTPENDING_ISACTIVE`"]
    #[inline(always)]
    pub fn is_mstpending_isactive(&self) -> bool {
        *self == MSTPENDING_A::MSTPENDING_ISACTIVE
    }
}
#[doc = "Field `MSTARBLOSS` reader - Master Arbitration Loss flag"]
pub type MSTARBLOSS_R = crate::BitReader<MSTARBLOSS_A>;
#[doc = "Master Arbitration Loss flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MSTARBLOSS_A {
    #[doc = "0: Not active"]
    MSTARBLOSS_ISNOTACTIVE = 0,
    #[doc = "1: Active"]
    MSTARBLOSS_ISACTIVE = 1,
}
impl From<MSTARBLOSS_A> for bool {
    #[inline(always)]
    fn from(variant: MSTARBLOSS_A) -> Self {
        variant as u8 != 0
    }
}
impl MSTARBLOSS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MSTARBLOSS_A {
        match self.bits {
            false => MSTARBLOSS_A::MSTARBLOSS_ISNOTACTIVE,
            true => MSTARBLOSS_A::MSTARBLOSS_ISACTIVE,
        }
    }
    #[doc = "Checks if the value of the field is `MSTARBLOSS_ISNOTACTIVE`"]
    #[inline(always)]
    pub fn is_mstarbloss_isnotactive(&self) -> bool {
        *self == MSTARBLOSS_A::MSTARBLOSS_ISNOTACTIVE
    }
    #[doc = "Checks if the value of the field is `MSTARBLOSS_ISACTIVE`"]
    #[inline(always)]
    pub fn is_mstarbloss_isactive(&self) -> bool {
        *self == MSTARBLOSS_A::MSTARBLOSS_ISACTIVE
    }
}
#[doc = "Field `MSTSTSTPERR` reader - Master Start/Stop Error flag"]
pub type MSTSTSTPERR_R = crate::BitReader<MSTSTSTPERR_A>;
#[doc = "Master Start/Stop Error flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MSTSTSTPERR_A {
    #[doc = "0: Not active"]
    MSTSTSTPERR_ISNOTACTIVE = 0,
    #[doc = "1: Active"]
    MSTSTSTPERR_ISACTIVE = 1,
}
impl From<MSTSTSTPERR_A> for bool {
    #[inline(always)]
    fn from(variant: MSTSTSTPERR_A) -> Self {
        variant as u8 != 0
    }
}
impl MSTSTSTPERR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MSTSTSTPERR_A {
        match self.bits {
            false => MSTSTSTPERR_A::MSTSTSTPERR_ISNOTACTIVE,
            true => MSTSTSTPERR_A::MSTSTSTPERR_ISACTIVE,
        }
    }
    #[doc = "Checks if the value of the field is `MSTSTSTPERR_ISNOTACTIVE`"]
    #[inline(always)]
    pub fn is_mstststperr_isnotactive(&self) -> bool {
        *self == MSTSTSTPERR_A::MSTSTSTPERR_ISNOTACTIVE
    }
    #[doc = "Checks if the value of the field is `MSTSTSTPERR_ISACTIVE`"]
    #[inline(always)]
    pub fn is_mstststperr_isactive(&self) -> bool {
        *self == MSTSTSTPERR_A::MSTSTSTPERR_ISACTIVE
    }
}
#[doc = "Field `SLVPENDING` reader - Slave Pending"]
pub type SLVPENDING_R = crate::BitReader<SLVPENDING_A>;
#[doc = "Slave Pending\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SLVPENDING_A {
    #[doc = "0: Not active"]
    SLVPENDING_ISNOTACTIVE = 0,
    #[doc = "1: Active"]
    SLVPENDING_ISACTIVE = 1,
}
impl From<SLVPENDING_A> for bool {
    #[inline(always)]
    fn from(variant: SLVPENDING_A) -> Self {
        variant as u8 != 0
    }
}
impl SLVPENDING_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SLVPENDING_A {
        match self.bits {
            false => SLVPENDING_A::SLVPENDING_ISNOTACTIVE,
            true => SLVPENDING_A::SLVPENDING_ISACTIVE,
        }
    }
    #[doc = "Checks if the value of the field is `SLVPENDING_ISNOTACTIVE`"]
    #[inline(always)]
    pub fn is_slvpending_isnotactive(&self) -> bool {
        *self == SLVPENDING_A::SLVPENDING_ISNOTACTIVE
    }
    #[doc = "Checks if the value of the field is `SLVPENDING_ISACTIVE`"]
    #[inline(always)]
    pub fn is_slvpending_isactive(&self) -> bool {
        *self == SLVPENDING_A::SLVPENDING_ISACTIVE
    }
}
#[doc = "Field `SLVNOTSTR` reader - Slave Not Stretching status"]
pub type SLVNOTSTR_R = crate::BitReader<SLVNOTSTR_A>;
#[doc = "Slave Not Stretching status\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SLVNOTSTR_A {
    #[doc = "0: Not active"]
    SLVNOTSTR_ISNOTACTIVE = 0,
    #[doc = "1: Active"]
    SLVNOTSTR_ISACTIVE = 1,
}
impl From<SLVNOTSTR_A> for bool {
    #[inline(always)]
    fn from(variant: SLVNOTSTR_A) -> Self {
        variant as u8 != 0
    }
}
impl SLVNOTSTR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SLVNOTSTR_A {
        match self.bits {
            false => SLVNOTSTR_A::SLVNOTSTR_ISNOTACTIVE,
            true => SLVNOTSTR_A::SLVNOTSTR_ISACTIVE,
        }
    }
    #[doc = "Checks if the value of the field is `SLVNOTSTR_ISNOTACTIVE`"]
    #[inline(always)]
    pub fn is_slvnotstr_isnotactive(&self) -> bool {
        *self == SLVNOTSTR_A::SLVNOTSTR_ISNOTACTIVE
    }
    #[doc = "Checks if the value of the field is `SLVNOTSTR_ISACTIVE`"]
    #[inline(always)]
    pub fn is_slvnotstr_isactive(&self) -> bool {
        *self == SLVNOTSTR_A::SLVNOTSTR_ISACTIVE
    }
}
#[doc = "Field `SLVDESEL` reader - Slave Deselected flag"]
pub type SLVDESEL_R = crate::BitReader<SLVDESEL_A>;
#[doc = "Slave Deselected flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SLVDESEL_A {
    #[doc = "0: Not active"]
    SLVDESEL_ISNOTACTIVE = 0,
    #[doc = "1: Active"]
    SLVDESEL_ISACTIVE = 1,
}
impl From<SLVDESEL_A> for bool {
    #[inline(always)]
    fn from(variant: SLVDESEL_A) -> Self {
        variant as u8 != 0
    }
}
impl SLVDESEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SLVDESEL_A {
        match self.bits {
            false => SLVDESEL_A::SLVDESEL_ISNOTACTIVE,
            true => SLVDESEL_A::SLVDESEL_ISACTIVE,
        }
    }
    #[doc = "Checks if the value of the field is `SLVDESEL_ISNOTACTIVE`"]
    #[inline(always)]
    pub fn is_slvdesel_isnotactive(&self) -> bool {
        *self == SLVDESEL_A::SLVDESEL_ISNOTACTIVE
    }
    #[doc = "Checks if the value of the field is `SLVDESEL_ISACTIVE`"]
    #[inline(always)]
    pub fn is_slvdesel_isactive(&self) -> bool {
        *self == SLVDESEL_A::SLVDESEL_ISACTIVE
    }
}
#[doc = "Field `MONRDY` reader - Monitor Ready"]
pub type MONRDY_R = crate::BitReader<MONRDY_A>;
#[doc = "Monitor Ready\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MONRDY_A {
    #[doc = "0: Not active"]
    MONRDY_ISNOTACTIVE = 0,
    #[doc = "1: Active"]
    MONRDY_ISACTIVE = 1,
}
impl From<MONRDY_A> for bool {
    #[inline(always)]
    fn from(variant: MONRDY_A) -> Self {
        variant as u8 != 0
    }
}
impl MONRDY_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MONRDY_A {
        match self.bits {
            false => MONRDY_A::MONRDY_ISNOTACTIVE,
            true => MONRDY_A::MONRDY_ISACTIVE,
        }
    }
    #[doc = "Checks if the value of the field is `MONRDY_ISNOTACTIVE`"]
    #[inline(always)]
    pub fn is_monrdy_isnotactive(&self) -> bool {
        *self == MONRDY_A::MONRDY_ISNOTACTIVE
    }
    #[doc = "Checks if the value of the field is `MONRDY_ISACTIVE`"]
    #[inline(always)]
    pub fn is_monrdy_isactive(&self) -> bool {
        *self == MONRDY_A::MONRDY_ISACTIVE
    }
}
#[doc = "Field `MONOV` reader - Monitor Overflow flag"]
pub type MONOV_R = crate::BitReader<MONOV_A>;
#[doc = "Monitor Overflow flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MONOV_A {
    #[doc = "0: Not active"]
    MONOV_ISNOTACTIVE = 0,
    #[doc = "1: Active"]
    MONOV_ISACTIVE = 1,
}
impl From<MONOV_A> for bool {
    #[inline(always)]
    fn from(variant: MONOV_A) -> Self {
        variant as u8 != 0
    }
}
impl MONOV_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MONOV_A {
        match self.bits {
            false => MONOV_A::MONOV_ISNOTACTIVE,
            true => MONOV_A::MONOV_ISACTIVE,
        }
    }
    #[doc = "Checks if the value of the field is `MONOV_ISNOTACTIVE`"]
    #[inline(always)]
    pub fn is_monov_isnotactive(&self) -> bool {
        *self == MONOV_A::MONOV_ISNOTACTIVE
    }
    #[doc = "Checks if the value of the field is `MONOV_ISACTIVE`"]
    #[inline(always)]
    pub fn is_monov_isactive(&self) -> bool {
        *self == MONOV_A::MONOV_ISACTIVE
    }
}
#[doc = "Field `MONIDLE` reader - Monitor Idle flag"]
pub type MONIDLE_R = crate::BitReader<MONIDLE_A>;
#[doc = "Monitor Idle flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MONIDLE_A {
    #[doc = "0: Not active"]
    MONIDLE_ISNOTACTIVE = 0,
    #[doc = "1: Active"]
    MONIDLE_ISACTIVE = 1,
}
impl From<MONIDLE_A> for bool {
    #[inline(always)]
    fn from(variant: MONIDLE_A) -> Self {
        variant as u8 != 0
    }
}
impl MONIDLE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MONIDLE_A {
        match self.bits {
            false => MONIDLE_A::MONIDLE_ISNOTACTIVE,
            true => MONIDLE_A::MONIDLE_ISACTIVE,
        }
    }
    #[doc = "Checks if the value of the field is `MONIDLE_ISNOTACTIVE`"]
    #[inline(always)]
    pub fn is_monidle_isnotactive(&self) -> bool {
        *self == MONIDLE_A::MONIDLE_ISNOTACTIVE
    }
    #[doc = "Checks if the value of the field is `MONIDLE_ISACTIVE`"]
    #[inline(always)]
    pub fn is_monidle_isactive(&self) -> bool {
        *self == MONIDLE_A::MONIDLE_ISACTIVE
    }
}
#[doc = "Field `EVENTTIMEOUT` reader - Event Time-out Interrupt flag"]
pub type EVENTTIMEOUT_R = crate::BitReader<EVENTTIMEOUT_A>;
#[doc = "Event Time-out Interrupt flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EVENTTIMEOUT_A {
    #[doc = "0: Not active"]
    EVENTTIMEOUT_ISNOTACTIVE = 0,
    #[doc = "1: Active"]
    EVENTTIMEOUT_ISACTIVE = 1,
}
impl From<EVENTTIMEOUT_A> for bool {
    #[inline(always)]
    fn from(variant: EVENTTIMEOUT_A) -> Self {
        variant as u8 != 0
    }
}
impl EVENTTIMEOUT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EVENTTIMEOUT_A {
        match self.bits {
            false => EVENTTIMEOUT_A::EVENTTIMEOUT_ISNOTACTIVE,
            true => EVENTTIMEOUT_A::EVENTTIMEOUT_ISACTIVE,
        }
    }
    #[doc = "Checks if the value of the field is `EVENTTIMEOUT_ISNOTACTIVE`"]
    #[inline(always)]
    pub fn is_eventtimeout_isnotactive(&self) -> bool {
        *self == EVENTTIMEOUT_A::EVENTTIMEOUT_ISNOTACTIVE
    }
    #[doc = "Checks if the value of the field is `EVENTTIMEOUT_ISACTIVE`"]
    #[inline(always)]
    pub fn is_eventtimeout_isactive(&self) -> bool {
        *self == EVENTTIMEOUT_A::EVENTTIMEOUT_ISACTIVE
    }
}
#[doc = "Field `SCLTIMEOUT` reader - SCL Time-out Interrupt flag"]
pub type SCLTIMEOUT_R = crate::BitReader<SCLTIMEOUT_A>;
#[doc = "SCL Time-out Interrupt flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SCLTIMEOUT_A {
    #[doc = "0: Not active"]
    SCLTIMEOUT_ISNOTACTIVE = 0,
    #[doc = "1: Active"]
    SCLTIMEOUT_ISACTIVE = 1,
}
impl From<SCLTIMEOUT_A> for bool {
    #[inline(always)]
    fn from(variant: SCLTIMEOUT_A) -> Self {
        variant as u8 != 0
    }
}
impl SCLTIMEOUT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SCLTIMEOUT_A {
        match self.bits {
            false => SCLTIMEOUT_A::SCLTIMEOUT_ISNOTACTIVE,
            true => SCLTIMEOUT_A::SCLTIMEOUT_ISACTIVE,
        }
    }
    #[doc = "Checks if the value of the field is `SCLTIMEOUT_ISNOTACTIVE`"]
    #[inline(always)]
    pub fn is_scltimeout_isnotactive(&self) -> bool {
        *self == SCLTIMEOUT_A::SCLTIMEOUT_ISNOTACTIVE
    }
    #[doc = "Checks if the value of the field is `SCLTIMEOUT_ISACTIVE`"]
    #[inline(always)]
    pub fn is_scltimeout_isactive(&self) -> bool {
        *self == SCLTIMEOUT_A::SCLTIMEOUT_ISACTIVE
    }
}
impl R {
    #[doc = "Bit 0 - Master Pending"]
    #[inline(always)]
    pub fn mstpending(&self) -> MSTPENDING_R {
        MSTPENDING_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 4 - Master Arbitration Loss flag"]
    #[inline(always)]
    pub fn mstarbloss(&self) -> MSTARBLOSS_R {
        MSTARBLOSS_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 6 - Master Start/Stop Error flag"]
    #[inline(always)]
    pub fn mstststperr(&self) -> MSTSTSTPERR_R {
        MSTSTSTPERR_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 8 - Slave Pending"]
    #[inline(always)]
    pub fn slvpending(&self) -> SLVPENDING_R {
        SLVPENDING_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 11 - Slave Not Stretching status"]
    #[inline(always)]
    pub fn slvnotstr(&self) -> SLVNOTSTR_R {
        SLVNOTSTR_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 15 - Slave Deselected flag"]
    #[inline(always)]
    pub fn slvdesel(&self) -> SLVDESEL_R {
        SLVDESEL_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Monitor Ready"]
    #[inline(always)]
    pub fn monrdy(&self) -> MONRDY_R {
        MONRDY_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Monitor Overflow flag"]
    #[inline(always)]
    pub fn monov(&self) -> MONOV_R {
        MONOV_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 19 - Monitor Idle flag"]
    #[inline(always)]
    pub fn monidle(&self) -> MONIDLE_R {
        MONIDLE_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 24 - Event Time-out Interrupt flag"]
    #[inline(always)]
    pub fn eventtimeout(&self) -> EVENTTIMEOUT_R {
        EVENTTIMEOUT_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - SCL Time-out Interrupt flag"]
    #[inline(always)]
    pub fn scltimeout(&self) -> SCLTIMEOUT_R {
        SCLTIMEOUT_R::new(((self.bits >> 25) & 1) != 0)
    }
}
#[doc = "Interrupt Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intstat](index.html) module"]
pub struct INTSTAT_SPEC;
impl crate::RegisterSpec for INTSTAT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [intstat::R](R) reader structure"]
impl crate::Readable for INTSTAT_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets INTSTAT to value 0x0801"]
impl crate::Resettable for INTSTAT_SPEC {
    const RESET_VALUE: Self::Ux = 0x0801;
}
