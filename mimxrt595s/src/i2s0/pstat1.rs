#[doc = "Register `PSTAT1` reader"]
pub struct R(crate::R<PSTAT1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PSTAT1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PSTAT1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PSTAT1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `BUSY` reader - Busy Status for Channel Pair"]
pub type BUSY_R = crate::BitReader<BUSY_A>;
#[doc = "Busy Status for Channel Pair\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BUSY_A {
    #[doc = "0: Idle. The transmitter/receiver for this channel pair is currently idle."]
    IDLE = 0,
    #[doc = "1: Busy. The transmitter/receiver for this channel pair is currently processing data."]
    BUSY = 1,
}
impl From<BUSY_A> for bool {
    #[inline(always)]
    fn from(variant: BUSY_A) -> Self {
        variant as u8 != 0
    }
}
impl BUSY_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BUSY_A {
        match self.bits {
            false => BUSY_A::IDLE,
            true => BUSY_A::BUSY,
        }
    }
    #[doc = "Checks if the value of the field is `IDLE`"]
    #[inline(always)]
    pub fn is_idle(&self) -> bool {
        *self == BUSY_A::IDLE
    }
    #[doc = "Checks if the value of the field is `BUSY`"]
    #[inline(always)]
    pub fn is_busy(&self) -> bool {
        *self == BUSY_A::BUSY
    }
}
#[doc = "Field `SLVFRMERR` reader - Save Frame Error Flag"]
pub type SLVFRMERR_R = crate::BitReader<SLVFRMERR_A>;
#[doc = "Save Frame Error Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SLVFRMERR_A {
    #[doc = "0: No Error"]
    NO_ERROR = 0,
    #[doc = "1: Error"]
    ERROR = 1,
}
impl From<SLVFRMERR_A> for bool {
    #[inline(always)]
    fn from(variant: SLVFRMERR_A) -> Self {
        variant as u8 != 0
    }
}
impl SLVFRMERR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SLVFRMERR_A {
        match self.bits {
            false => SLVFRMERR_A::NO_ERROR,
            true => SLVFRMERR_A::ERROR,
        }
    }
    #[doc = "Checks if the value of the field is `NO_ERROR`"]
    #[inline(always)]
    pub fn is_no_error(&self) -> bool {
        *self == SLVFRMERR_A::NO_ERROR
    }
    #[doc = "Checks if the value of the field is `ERROR`"]
    #[inline(always)]
    pub fn is_error(&self) -> bool {
        *self == SLVFRMERR_A::ERROR
    }
}
#[doc = "Field `LR` reader - Left/Right Indication"]
pub type LR_R = crate::BitReader<LR_A>;
#[doc = "Left/Right Indication\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LR_A {
    #[doc = "0: Left channel"]
    LEFT_CHANNEL = 0,
    #[doc = "1: Right channel"]
    RIGHT_CHANNEL = 1,
}
impl From<LR_A> for bool {
    #[inline(always)]
    fn from(variant: LR_A) -> Self {
        variant as u8 != 0
    }
}
impl LR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LR_A {
        match self.bits {
            false => LR_A::LEFT_CHANNEL,
            true => LR_A::RIGHT_CHANNEL,
        }
    }
    #[doc = "Checks if the value of the field is `LEFT_CHANNEL`"]
    #[inline(always)]
    pub fn is_left_channel(&self) -> bool {
        *self == LR_A::LEFT_CHANNEL
    }
    #[doc = "Checks if the value of the field is `RIGHT_CHANNEL`"]
    #[inline(always)]
    pub fn is_right_channel(&self) -> bool {
        *self == LR_A::RIGHT_CHANNEL
    }
}
#[doc = "Field `DATAPAUSED` reader - Data Paused Status Flag"]
pub type DATAPAUSED_R = crate::BitReader<DATAPAUSED_A>;
#[doc = "Data Paused Status Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DATAPAUSED_A {
    #[doc = "0: Data Not Paused. Data is not currently paused. A data pause may have been requested but is not yet in force, waiting for an allowed pause point. Refer to the description in CFG1\\[DATAPAUSE\\]."]
    NOT_PAUSED = 0,
    #[doc = "1: Data Paused. A data pause has been requested and is now in force."]
    PAUSED = 1,
}
impl From<DATAPAUSED_A> for bool {
    #[inline(always)]
    fn from(variant: DATAPAUSED_A) -> Self {
        variant as u8 != 0
    }
}
impl DATAPAUSED_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DATAPAUSED_A {
        match self.bits {
            false => DATAPAUSED_A::NOT_PAUSED,
            true => DATAPAUSED_A::PAUSED,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_PAUSED`"]
    #[inline(always)]
    pub fn is_not_paused(&self) -> bool {
        *self == DATAPAUSED_A::NOT_PAUSED
    }
    #[doc = "Checks if the value of the field is `PAUSED`"]
    #[inline(always)]
    pub fn is_paused(&self) -> bool {
        *self == DATAPAUSED_A::PAUSED
    }
}
impl R {
    #[doc = "Bit 0 - Busy Status for Channel Pair"]
    #[inline(always)]
    pub fn busy(&self) -> BUSY_R {
        BUSY_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Save Frame Error Flag"]
    #[inline(always)]
    pub fn slvfrmerr(&self) -> SLVFRMERR_R {
        SLVFRMERR_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Left/Right Indication"]
    #[inline(always)]
    pub fn lr(&self) -> LR_R {
        LR_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Data Paused Status Flag"]
    #[inline(always)]
    pub fn datapaused(&self) -> DATAPAUSED_R {
        DATAPAUSED_R::new(((self.bits >> 3) & 1) != 0)
    }
}
#[doc = "Status Register for Channel Pair 1\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pstat1](index.html) module"]
pub struct PSTAT1_SPEC;
impl crate::RegisterSpec for PSTAT1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pstat1::R](R) reader structure"]
impl crate::Readable for PSTAT1_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets PSTAT1 to value 0"]
impl crate::Resettable for PSTAT1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
