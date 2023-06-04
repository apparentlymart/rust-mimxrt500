#[doc = "Register `STS0` reader"]
pub struct R(crate::R<STS0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<STS0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<STS0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<STS0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `SEQIDLE` reader - This status bit indicates the state machine in SEQ_CTL is idle and there is command sequence executing on FlexSPI interface."]
pub type SEQIDLE_R = crate::BitReader<SEQIDLE_A>;
#[doc = "This status bit indicates the state machine in SEQ_CTL is idle and there is command sequence executing on FlexSPI interface.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SEQIDLE_A {
    #[doc = "0: State machine in SEQ_CTL is not idle."]
    VALUE0 = 0,
    #[doc = "1: State machine in SEQ_CTL is idle."]
    VALUE1 = 1,
}
impl From<SEQIDLE_A> for bool {
    #[inline(always)]
    fn from(variant: SEQIDLE_A) -> Self {
        variant as u8 != 0
    }
}
impl SEQIDLE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SEQIDLE_A {
        match self.bits {
            false => SEQIDLE_A::VALUE0,
            true => SEQIDLE_A::VALUE1,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE0`"]
    #[inline(always)]
    pub fn is_value0(&self) -> bool {
        *self == SEQIDLE_A::VALUE0
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == SEQIDLE_A::VALUE1
    }
}
#[doc = "Field `ARBIDLE` reader - This status bit indicates the state machine in ARB_CTL is busy and there is command sequence granted by arbitrator and not finished yet on FlexSPI interface. When ARB_CTL state (ARBIDLE=0x1) is idle, there will be no transaction on FlexSPI interface also (SEQIDLE=0x1). So this bit should be polled to wait for FlexSPI controller become idle instead of SEQIDLE."]
pub type ARBIDLE_R = crate::BitReader<bool>;
#[doc = "Field `ARBCMDSRC` reader - This status field indicates the trigger source of current command sequence granted by arbitrator. This field value is meaningless when ARB_CTL is not busy (STS0\\[ARBIDLE\\]=0x1)."]
pub type ARBCMDSRC_R = crate::FieldReader<u8, ARBCMDSRC_A>;
#[doc = "This status field indicates the trigger source of current command sequence granted by arbitrator. This field value is meaningless when ARB_CTL is not busy (STS0\\[ARBIDLE\\]=0x1).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ARBCMDSRC_A {
    #[doc = "0: Triggered by AHB read command."]
    VAL0 = 0,
    #[doc = "1: Triggered by AHB write command."]
    VAL1 = 1,
    #[doc = "2: Triggered by IP command (triggered by setting register bit IPCMD\\[TRG\\])."]
    VAL2 = 2,
    #[doc = "3: Triggered by suspended command (resumed)."]
    VAL3 = 3,
}
impl From<ARBCMDSRC_A> for u8 {
    #[inline(always)]
    fn from(variant: ARBCMDSRC_A) -> Self {
        variant as _
    }
}
impl ARBCMDSRC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ARBCMDSRC_A {
        match self.bits {
            0 => ARBCMDSRC_A::VAL0,
            1 => ARBCMDSRC_A::VAL1,
            2 => ARBCMDSRC_A::VAL2,
            3 => ARBCMDSRC_A::VAL3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `VAL0`"]
    #[inline(always)]
    pub fn is_val0(&self) -> bool {
        *self == ARBCMDSRC_A::VAL0
    }
    #[doc = "Checks if the value of the field is `VAL1`"]
    #[inline(always)]
    pub fn is_val1(&self) -> bool {
        *self == ARBCMDSRC_A::VAL1
    }
    #[doc = "Checks if the value of the field is `VAL2`"]
    #[inline(always)]
    pub fn is_val2(&self) -> bool {
        *self == ARBCMDSRC_A::VAL2
    }
    #[doc = "Checks if the value of the field is `VAL3`"]
    #[inline(always)]
    pub fn is_val3(&self) -> bool {
        *self == ARBCMDSRC_A::VAL3
    }
}
#[doc = "Field `DATALEARNPHASEA` reader - Indicate the sampling clock phase selection on Port A after Data Learning."]
pub type DATALEARNPHASEA_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bit 0 - This status bit indicates the state machine in SEQ_CTL is idle and there is command sequence executing on FlexSPI interface."]
    #[inline(always)]
    pub fn seqidle(&self) -> SEQIDLE_R {
        SEQIDLE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - This status bit indicates the state machine in ARB_CTL is busy and there is command sequence granted by arbitrator and not finished yet on FlexSPI interface. When ARB_CTL state (ARBIDLE=0x1) is idle, there will be no transaction on FlexSPI interface also (SEQIDLE=0x1). So this bit should be polled to wait for FlexSPI controller become idle instead of SEQIDLE."]
    #[inline(always)]
    pub fn arbidle(&self) -> ARBIDLE_R {
        ARBIDLE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:3 - This status field indicates the trigger source of current command sequence granted by arbitrator. This field value is meaningless when ARB_CTL is not busy (STS0\\[ARBIDLE\\]=0x1)."]
    #[inline(always)]
    pub fn arbcmdsrc(&self) -> ARBCMDSRC_R {
        ARBCMDSRC_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:7 - Indicate the sampling clock phase selection on Port A after Data Learning."]
    #[inline(always)]
    pub fn datalearnphasea(&self) -> DATALEARNPHASEA_R {
        DATALEARNPHASEA_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
#[doc = "Status Register 0\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sts0](index.html) module"]
pub struct STS0_SPEC;
impl crate::RegisterSpec for STS0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sts0::R](R) reader structure"]
impl crate::Readable for STS0_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets STS0 to value 0x02"]
impl crate::Resettable for STS0_SPEC {
    const RESET_VALUE: Self::Ux = 0x02;
}
