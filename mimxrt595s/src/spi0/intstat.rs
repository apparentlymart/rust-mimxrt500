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
#[doc = "Field `SSA` reader - Slave Select Assert Interrupt"]
pub type SSA_R = crate::BitReader<SSA_A>;
#[doc = "Slave Select Assert Interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SSA_A {
    #[doc = "0: Disabled"]
    SSA_INTERRUPT_DISABLED = 0,
    #[doc = "1: Enabled"]
    SSA_INTERRUPT_ENABLED = 1,
}
impl From<SSA_A> for bool {
    #[inline(always)]
    fn from(variant: SSA_A) -> Self {
        variant as u8 != 0
    }
}
impl SSA_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SSA_A {
        match self.bits {
            false => SSA_A::SSA_INTERRUPT_DISABLED,
            true => SSA_A::SSA_INTERRUPT_ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `SSA_INTERRUPT_DISABLED`"]
    #[inline(always)]
    pub fn is_ssa_interrupt_disabled(&self) -> bool {
        *self == SSA_A::SSA_INTERRUPT_DISABLED
    }
    #[doc = "Checks if the value of the field is `SSA_INTERRUPT_ENABLED`"]
    #[inline(always)]
    pub fn is_ssa_interrupt_enabled(&self) -> bool {
        *self == SSA_A::SSA_INTERRUPT_ENABLED
    }
}
#[doc = "Field `SSD` reader - Slave Select Deassert Interrupt"]
pub type SSD_R = crate::BitReader<SSD_A>;
#[doc = "Slave Select Deassert Interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SSD_A {
    #[doc = "0: Disabled"]
    SSD_INTERRUPT_DISABLED = 0,
    #[doc = "1: Enabled"]
    SSD_INTERRUPT_ENABLED = 1,
}
impl From<SSD_A> for bool {
    #[inline(always)]
    fn from(variant: SSD_A) -> Self {
        variant as u8 != 0
    }
}
impl SSD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SSD_A {
        match self.bits {
            false => SSD_A::SSD_INTERRUPT_DISABLED,
            true => SSD_A::SSD_INTERRUPT_ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `SSD_INTERRUPT_DISABLED`"]
    #[inline(always)]
    pub fn is_ssd_interrupt_disabled(&self) -> bool {
        *self == SSD_A::SSD_INTERRUPT_DISABLED
    }
    #[doc = "Checks if the value of the field is `SSD_INTERRUPT_ENABLED`"]
    #[inline(always)]
    pub fn is_ssd_interrupt_enabled(&self) -> bool {
        *self == SSD_A::SSD_INTERRUPT_ENABLED
    }
}
#[doc = "Field `MSTIDLE` reader - Master Idle Status Flag Interrupt"]
pub type MSTIDLE_R = crate::BitReader<MSTIDLE_A>;
#[doc = "Master Idle Status Flag Interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MSTIDLE_A {
    #[doc = "0: Disabled"]
    MSTIDLE_INTERRUPT_DISABLED = 0,
    #[doc = "1: Enabled"]
    MSTIDLE_INTERRUPT_ENABLED = 1,
}
impl From<MSTIDLE_A> for bool {
    #[inline(always)]
    fn from(variant: MSTIDLE_A) -> Self {
        variant as u8 != 0
    }
}
impl MSTIDLE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MSTIDLE_A {
        match self.bits {
            false => MSTIDLE_A::MSTIDLE_INTERRUPT_DISABLED,
            true => MSTIDLE_A::MSTIDLE_INTERRUPT_ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `MSTIDLE_INTERRUPT_DISABLED`"]
    #[inline(always)]
    pub fn is_mstidle_interrupt_disabled(&self) -> bool {
        *self == MSTIDLE_A::MSTIDLE_INTERRUPT_DISABLED
    }
    #[doc = "Checks if the value of the field is `MSTIDLE_INTERRUPT_ENABLED`"]
    #[inline(always)]
    pub fn is_mstidle_interrupt_enabled(&self) -> bool {
        *self == MSTIDLE_A::MSTIDLE_INTERRUPT_ENABLED
    }
}
impl R {
    #[doc = "Bit 4 - Slave Select Assert Interrupt"]
    #[inline(always)]
    pub fn ssa(&self) -> SSA_R {
        SSA_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Slave Select Deassert Interrupt"]
    #[inline(always)]
    pub fn ssd(&self) -> SSD_R {
        SSD_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 8 - Master Idle Status Flag Interrupt"]
    #[inline(always)]
    pub fn mstidle(&self) -> MSTIDLE_R {
        MSTIDLE_R::new(((self.bits >> 8) & 1) != 0)
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
#[doc = "`reset()` method sets INTSTAT to value 0"]
impl crate::Resettable for INTSTAT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
