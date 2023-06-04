#[doc = "Register `BUSY0` reader"]
pub struct R(crate::R<BUSY0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BUSY0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BUSY0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BUSY0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `BUSY0` reader - Busy flag for DMA channel."]
pub type BUSY0_R = crate::BitReader<BUSY0_A>;
#[doc = "Busy flag for DMA channel.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BUSY0_A {
    #[doc = "0: DMA channel is not busy."]
    NOT_BUSY = 0,
    #[doc = "1: DMA channel is busy."]
    BUSY = 1,
}
impl From<BUSY0_A> for bool {
    #[inline(always)]
    fn from(variant: BUSY0_A) -> Self {
        variant as u8 != 0
    }
}
impl BUSY0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BUSY0_A {
        match self.bits {
            false => BUSY0_A::NOT_BUSY,
            true => BUSY0_A::BUSY,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_BUSY`"]
    #[inline(always)]
    pub fn is_not_busy(&self) -> bool {
        *self == BUSY0_A::NOT_BUSY
    }
    #[doc = "Checks if the value of the field is `BUSY`"]
    #[inline(always)]
    pub fn is_busy(&self) -> bool {
        *self == BUSY0_A::BUSY
    }
}
#[doc = "Field `BUSY1` reader - Busy flag for DMA channel."]
pub type BUSY1_R = crate::BitReader<BUSY1_A>;
#[doc = "Busy flag for DMA channel.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BUSY1_A {
    #[doc = "0: DMA channel is not busy."]
    NOT_BUSY = 0,
    #[doc = "1: DMA channel is busy."]
    BUSY = 1,
}
impl From<BUSY1_A> for bool {
    #[inline(always)]
    fn from(variant: BUSY1_A) -> Self {
        variant as u8 != 0
    }
}
impl BUSY1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BUSY1_A {
        match self.bits {
            false => BUSY1_A::NOT_BUSY,
            true => BUSY1_A::BUSY,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_BUSY`"]
    #[inline(always)]
    pub fn is_not_busy(&self) -> bool {
        *self == BUSY1_A::NOT_BUSY
    }
    #[doc = "Checks if the value of the field is `BUSY`"]
    #[inline(always)]
    pub fn is_busy(&self) -> bool {
        *self == BUSY1_A::BUSY
    }
}
#[doc = "Field `BUSY2` reader - Busy flag for DMA channel."]
pub type BUSY2_R = crate::BitReader<BUSY2_A>;
#[doc = "Busy flag for DMA channel.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BUSY2_A {
    #[doc = "0: DMA channel is not busy."]
    NOT_BUSY = 0,
    #[doc = "1: DMA channel is busy."]
    BUSY = 1,
}
impl From<BUSY2_A> for bool {
    #[inline(always)]
    fn from(variant: BUSY2_A) -> Self {
        variant as u8 != 0
    }
}
impl BUSY2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BUSY2_A {
        match self.bits {
            false => BUSY2_A::NOT_BUSY,
            true => BUSY2_A::BUSY,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_BUSY`"]
    #[inline(always)]
    pub fn is_not_busy(&self) -> bool {
        *self == BUSY2_A::NOT_BUSY
    }
    #[doc = "Checks if the value of the field is `BUSY`"]
    #[inline(always)]
    pub fn is_busy(&self) -> bool {
        *self == BUSY2_A::BUSY
    }
}
#[doc = "Field `BUSY3` reader - Busy flag for DMA channel."]
pub type BUSY3_R = crate::BitReader<BUSY3_A>;
#[doc = "Busy flag for DMA channel.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BUSY3_A {
    #[doc = "0: DMA channel is not busy."]
    NOT_BUSY = 0,
    #[doc = "1: DMA channel is busy."]
    BUSY = 1,
}
impl From<BUSY3_A> for bool {
    #[inline(always)]
    fn from(variant: BUSY3_A) -> Self {
        variant as u8 != 0
    }
}
impl BUSY3_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BUSY3_A {
        match self.bits {
            false => BUSY3_A::NOT_BUSY,
            true => BUSY3_A::BUSY,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_BUSY`"]
    #[inline(always)]
    pub fn is_not_busy(&self) -> bool {
        *self == BUSY3_A::NOT_BUSY
    }
    #[doc = "Checks if the value of the field is `BUSY`"]
    #[inline(always)]
    pub fn is_busy(&self) -> bool {
        *self == BUSY3_A::BUSY
    }
}
#[doc = "Field `BUSY4` reader - Busy flag for DMA channel."]
pub type BUSY4_R = crate::BitReader<BUSY4_A>;
#[doc = "Busy flag for DMA channel.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BUSY4_A {
    #[doc = "0: DMA channel is not busy."]
    NOT_BUSY = 0,
    #[doc = "1: DMA channel is busy."]
    BUSY = 1,
}
impl From<BUSY4_A> for bool {
    #[inline(always)]
    fn from(variant: BUSY4_A) -> Self {
        variant as u8 != 0
    }
}
impl BUSY4_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BUSY4_A {
        match self.bits {
            false => BUSY4_A::NOT_BUSY,
            true => BUSY4_A::BUSY,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_BUSY`"]
    #[inline(always)]
    pub fn is_not_busy(&self) -> bool {
        *self == BUSY4_A::NOT_BUSY
    }
    #[doc = "Checks if the value of the field is `BUSY`"]
    #[inline(always)]
    pub fn is_busy(&self) -> bool {
        *self == BUSY4_A::BUSY
    }
}
#[doc = "Field `BUSY5` reader - Busy flag for DMA channel."]
pub type BUSY5_R = crate::BitReader<BUSY5_A>;
#[doc = "Busy flag for DMA channel.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BUSY5_A {
    #[doc = "0: DMA channel is not busy."]
    NOT_BUSY = 0,
    #[doc = "1: DMA channel is busy."]
    BUSY = 1,
}
impl From<BUSY5_A> for bool {
    #[inline(always)]
    fn from(variant: BUSY5_A) -> Self {
        variant as u8 != 0
    }
}
impl BUSY5_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BUSY5_A {
        match self.bits {
            false => BUSY5_A::NOT_BUSY,
            true => BUSY5_A::BUSY,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_BUSY`"]
    #[inline(always)]
    pub fn is_not_busy(&self) -> bool {
        *self == BUSY5_A::NOT_BUSY
    }
    #[doc = "Checks if the value of the field is `BUSY`"]
    #[inline(always)]
    pub fn is_busy(&self) -> bool {
        *self == BUSY5_A::BUSY
    }
}
#[doc = "Field `BUSY6` reader - Busy flag for DMA channel."]
pub type BUSY6_R = crate::BitReader<BUSY6_A>;
#[doc = "Busy flag for DMA channel.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BUSY6_A {
    #[doc = "0: DMA channel is not busy."]
    NOT_BUSY = 0,
    #[doc = "1: DMA channel is busy."]
    BUSY = 1,
}
impl From<BUSY6_A> for bool {
    #[inline(always)]
    fn from(variant: BUSY6_A) -> Self {
        variant as u8 != 0
    }
}
impl BUSY6_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BUSY6_A {
        match self.bits {
            false => BUSY6_A::NOT_BUSY,
            true => BUSY6_A::BUSY,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_BUSY`"]
    #[inline(always)]
    pub fn is_not_busy(&self) -> bool {
        *self == BUSY6_A::NOT_BUSY
    }
    #[doc = "Checks if the value of the field is `BUSY`"]
    #[inline(always)]
    pub fn is_busy(&self) -> bool {
        *self == BUSY6_A::BUSY
    }
}
#[doc = "Field `BUSY7` reader - Busy flag for DMA channel."]
pub type BUSY7_R = crate::BitReader<BUSY7_A>;
#[doc = "Busy flag for DMA channel.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BUSY7_A {
    #[doc = "0: DMA channel is not busy."]
    NOT_BUSY = 0,
    #[doc = "1: DMA channel is busy."]
    BUSY = 1,
}
impl From<BUSY7_A> for bool {
    #[inline(always)]
    fn from(variant: BUSY7_A) -> Self {
        variant as u8 != 0
    }
}
impl BUSY7_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BUSY7_A {
        match self.bits {
            false => BUSY7_A::NOT_BUSY,
            true => BUSY7_A::BUSY,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_BUSY`"]
    #[inline(always)]
    pub fn is_not_busy(&self) -> bool {
        *self == BUSY7_A::NOT_BUSY
    }
    #[doc = "Checks if the value of the field is `BUSY`"]
    #[inline(always)]
    pub fn is_busy(&self) -> bool {
        *self == BUSY7_A::BUSY
    }
}
#[doc = "Field `BUSY8` reader - Busy flag for DMA channel."]
pub type BUSY8_R = crate::BitReader<BUSY8_A>;
#[doc = "Busy flag for DMA channel.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BUSY8_A {
    #[doc = "0: DMA channel is not busy."]
    NOT_BUSY = 0,
    #[doc = "1: DMA channel is busy."]
    BUSY = 1,
}
impl From<BUSY8_A> for bool {
    #[inline(always)]
    fn from(variant: BUSY8_A) -> Self {
        variant as u8 != 0
    }
}
impl BUSY8_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BUSY8_A {
        match self.bits {
            false => BUSY8_A::NOT_BUSY,
            true => BUSY8_A::BUSY,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_BUSY`"]
    #[inline(always)]
    pub fn is_not_busy(&self) -> bool {
        *self == BUSY8_A::NOT_BUSY
    }
    #[doc = "Checks if the value of the field is `BUSY`"]
    #[inline(always)]
    pub fn is_busy(&self) -> bool {
        *self == BUSY8_A::BUSY
    }
}
#[doc = "Field `BUSY9` reader - Busy flag for DMA channel."]
pub type BUSY9_R = crate::BitReader<BUSY9_A>;
#[doc = "Busy flag for DMA channel.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BUSY9_A {
    #[doc = "0: DMA channel is not busy."]
    NOT_BUSY = 0,
    #[doc = "1: DMA channel is busy."]
    BUSY = 1,
}
impl From<BUSY9_A> for bool {
    #[inline(always)]
    fn from(variant: BUSY9_A) -> Self {
        variant as u8 != 0
    }
}
impl BUSY9_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BUSY9_A {
        match self.bits {
            false => BUSY9_A::NOT_BUSY,
            true => BUSY9_A::BUSY,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_BUSY`"]
    #[inline(always)]
    pub fn is_not_busy(&self) -> bool {
        *self == BUSY9_A::NOT_BUSY
    }
    #[doc = "Checks if the value of the field is `BUSY`"]
    #[inline(always)]
    pub fn is_busy(&self) -> bool {
        *self == BUSY9_A::BUSY
    }
}
#[doc = "Field `BUSY10` reader - Busy flag for DMA channel."]
pub type BUSY10_R = crate::BitReader<BUSY10_A>;
#[doc = "Busy flag for DMA channel.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BUSY10_A {
    #[doc = "0: DMA channel is not busy."]
    NOT_BUSY = 0,
    #[doc = "1: DMA channel is busy."]
    BUSY = 1,
}
impl From<BUSY10_A> for bool {
    #[inline(always)]
    fn from(variant: BUSY10_A) -> Self {
        variant as u8 != 0
    }
}
impl BUSY10_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BUSY10_A {
        match self.bits {
            false => BUSY10_A::NOT_BUSY,
            true => BUSY10_A::BUSY,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_BUSY`"]
    #[inline(always)]
    pub fn is_not_busy(&self) -> bool {
        *self == BUSY10_A::NOT_BUSY
    }
    #[doc = "Checks if the value of the field is `BUSY`"]
    #[inline(always)]
    pub fn is_busy(&self) -> bool {
        *self == BUSY10_A::BUSY
    }
}
#[doc = "Field `BUSY11` reader - Busy flag for DMA channel."]
pub type BUSY11_R = crate::BitReader<BUSY11_A>;
#[doc = "Busy flag for DMA channel.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BUSY11_A {
    #[doc = "0: DMA channel is not busy."]
    NOT_BUSY = 0,
    #[doc = "1: DMA channel is busy."]
    BUSY = 1,
}
impl From<BUSY11_A> for bool {
    #[inline(always)]
    fn from(variant: BUSY11_A) -> Self {
        variant as u8 != 0
    }
}
impl BUSY11_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BUSY11_A {
        match self.bits {
            false => BUSY11_A::NOT_BUSY,
            true => BUSY11_A::BUSY,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_BUSY`"]
    #[inline(always)]
    pub fn is_not_busy(&self) -> bool {
        *self == BUSY11_A::NOT_BUSY
    }
    #[doc = "Checks if the value of the field is `BUSY`"]
    #[inline(always)]
    pub fn is_busy(&self) -> bool {
        *self == BUSY11_A::BUSY
    }
}
#[doc = "Field `BUSY12` reader - Busy flag for DMA channel."]
pub type BUSY12_R = crate::BitReader<BUSY12_A>;
#[doc = "Busy flag for DMA channel.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BUSY12_A {
    #[doc = "0: DMA channel is not busy."]
    NOT_BUSY = 0,
    #[doc = "1: DMA channel is busy."]
    BUSY = 1,
}
impl From<BUSY12_A> for bool {
    #[inline(always)]
    fn from(variant: BUSY12_A) -> Self {
        variant as u8 != 0
    }
}
impl BUSY12_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BUSY12_A {
        match self.bits {
            false => BUSY12_A::NOT_BUSY,
            true => BUSY12_A::BUSY,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_BUSY`"]
    #[inline(always)]
    pub fn is_not_busy(&self) -> bool {
        *self == BUSY12_A::NOT_BUSY
    }
    #[doc = "Checks if the value of the field is `BUSY`"]
    #[inline(always)]
    pub fn is_busy(&self) -> bool {
        *self == BUSY12_A::BUSY
    }
}
#[doc = "Field `BUSY13` reader - Busy flag for DMA channel."]
pub type BUSY13_R = crate::BitReader<BUSY13_A>;
#[doc = "Busy flag for DMA channel.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BUSY13_A {
    #[doc = "0: DMA channel is not busy."]
    NOT_BUSY = 0,
    #[doc = "1: DMA channel is busy."]
    BUSY = 1,
}
impl From<BUSY13_A> for bool {
    #[inline(always)]
    fn from(variant: BUSY13_A) -> Self {
        variant as u8 != 0
    }
}
impl BUSY13_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BUSY13_A {
        match self.bits {
            false => BUSY13_A::NOT_BUSY,
            true => BUSY13_A::BUSY,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_BUSY`"]
    #[inline(always)]
    pub fn is_not_busy(&self) -> bool {
        *self == BUSY13_A::NOT_BUSY
    }
    #[doc = "Checks if the value of the field is `BUSY`"]
    #[inline(always)]
    pub fn is_busy(&self) -> bool {
        *self == BUSY13_A::BUSY
    }
}
#[doc = "Field `BUSY14` reader - Busy flag for DMA channel."]
pub type BUSY14_R = crate::BitReader<BUSY14_A>;
#[doc = "Busy flag for DMA channel.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BUSY14_A {
    #[doc = "0: DMA channel is not busy."]
    NOT_BUSY = 0,
    #[doc = "1: DMA channel is busy."]
    BUSY = 1,
}
impl From<BUSY14_A> for bool {
    #[inline(always)]
    fn from(variant: BUSY14_A) -> Self {
        variant as u8 != 0
    }
}
impl BUSY14_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BUSY14_A {
        match self.bits {
            false => BUSY14_A::NOT_BUSY,
            true => BUSY14_A::BUSY,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_BUSY`"]
    #[inline(always)]
    pub fn is_not_busy(&self) -> bool {
        *self == BUSY14_A::NOT_BUSY
    }
    #[doc = "Checks if the value of the field is `BUSY`"]
    #[inline(always)]
    pub fn is_busy(&self) -> bool {
        *self == BUSY14_A::BUSY
    }
}
#[doc = "Field `BUSY15` reader - Busy flag for DMA channel."]
pub type BUSY15_R = crate::BitReader<BUSY15_A>;
#[doc = "Busy flag for DMA channel.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BUSY15_A {
    #[doc = "0: DMA channel is not busy."]
    NOT_BUSY = 0,
    #[doc = "1: DMA channel is busy."]
    BUSY = 1,
}
impl From<BUSY15_A> for bool {
    #[inline(always)]
    fn from(variant: BUSY15_A) -> Self {
        variant as u8 != 0
    }
}
impl BUSY15_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BUSY15_A {
        match self.bits {
            false => BUSY15_A::NOT_BUSY,
            true => BUSY15_A::BUSY,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_BUSY`"]
    #[inline(always)]
    pub fn is_not_busy(&self) -> bool {
        *self == BUSY15_A::NOT_BUSY
    }
    #[doc = "Checks if the value of the field is `BUSY`"]
    #[inline(always)]
    pub fn is_busy(&self) -> bool {
        *self == BUSY15_A::BUSY
    }
}
#[doc = "Field `BUSY16` reader - Busy flag for DMA channel."]
pub type BUSY16_R = crate::BitReader<BUSY16_A>;
#[doc = "Busy flag for DMA channel.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BUSY16_A {
    #[doc = "0: DMA channel is not busy."]
    NOT_BUSY = 0,
    #[doc = "1: DMA channel is busy."]
    BUSY = 1,
}
impl From<BUSY16_A> for bool {
    #[inline(always)]
    fn from(variant: BUSY16_A) -> Self {
        variant as u8 != 0
    }
}
impl BUSY16_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BUSY16_A {
        match self.bits {
            false => BUSY16_A::NOT_BUSY,
            true => BUSY16_A::BUSY,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_BUSY`"]
    #[inline(always)]
    pub fn is_not_busy(&self) -> bool {
        *self == BUSY16_A::NOT_BUSY
    }
    #[doc = "Checks if the value of the field is `BUSY`"]
    #[inline(always)]
    pub fn is_busy(&self) -> bool {
        *self == BUSY16_A::BUSY
    }
}
#[doc = "Field `BUSY17` reader - Busy flag for DMA channel."]
pub type BUSY17_R = crate::BitReader<BUSY17_A>;
#[doc = "Busy flag for DMA channel.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BUSY17_A {
    #[doc = "0: DMA channel is not busy."]
    NOT_BUSY = 0,
    #[doc = "1: DMA channel is busy."]
    BUSY = 1,
}
impl From<BUSY17_A> for bool {
    #[inline(always)]
    fn from(variant: BUSY17_A) -> Self {
        variant as u8 != 0
    }
}
impl BUSY17_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BUSY17_A {
        match self.bits {
            false => BUSY17_A::NOT_BUSY,
            true => BUSY17_A::BUSY,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_BUSY`"]
    #[inline(always)]
    pub fn is_not_busy(&self) -> bool {
        *self == BUSY17_A::NOT_BUSY
    }
    #[doc = "Checks if the value of the field is `BUSY`"]
    #[inline(always)]
    pub fn is_busy(&self) -> bool {
        *self == BUSY17_A::BUSY
    }
}
#[doc = "Field `BUSY18` reader - Busy flag for DMA channel."]
pub type BUSY18_R = crate::BitReader<BUSY18_A>;
#[doc = "Busy flag for DMA channel.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BUSY18_A {
    #[doc = "0: DMA channel is not busy."]
    NOT_BUSY = 0,
    #[doc = "1: DMA channel is busy."]
    BUSY = 1,
}
impl From<BUSY18_A> for bool {
    #[inline(always)]
    fn from(variant: BUSY18_A) -> Self {
        variant as u8 != 0
    }
}
impl BUSY18_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BUSY18_A {
        match self.bits {
            false => BUSY18_A::NOT_BUSY,
            true => BUSY18_A::BUSY,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_BUSY`"]
    #[inline(always)]
    pub fn is_not_busy(&self) -> bool {
        *self == BUSY18_A::NOT_BUSY
    }
    #[doc = "Checks if the value of the field is `BUSY`"]
    #[inline(always)]
    pub fn is_busy(&self) -> bool {
        *self == BUSY18_A::BUSY
    }
}
#[doc = "Field `BUSY19` reader - Busy flag for DMA channel."]
pub type BUSY19_R = crate::BitReader<BUSY19_A>;
#[doc = "Busy flag for DMA channel.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BUSY19_A {
    #[doc = "0: DMA channel is not busy."]
    NOT_BUSY = 0,
    #[doc = "1: DMA channel is busy."]
    BUSY = 1,
}
impl From<BUSY19_A> for bool {
    #[inline(always)]
    fn from(variant: BUSY19_A) -> Self {
        variant as u8 != 0
    }
}
impl BUSY19_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BUSY19_A {
        match self.bits {
            false => BUSY19_A::NOT_BUSY,
            true => BUSY19_A::BUSY,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_BUSY`"]
    #[inline(always)]
    pub fn is_not_busy(&self) -> bool {
        *self == BUSY19_A::NOT_BUSY
    }
    #[doc = "Checks if the value of the field is `BUSY`"]
    #[inline(always)]
    pub fn is_busy(&self) -> bool {
        *self == BUSY19_A::BUSY
    }
}
#[doc = "Field `BUSY20` reader - Busy flag for DMA channel."]
pub type BUSY20_R = crate::BitReader<BUSY20_A>;
#[doc = "Busy flag for DMA channel.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BUSY20_A {
    #[doc = "0: DMA channel is not busy."]
    NOT_BUSY = 0,
    #[doc = "1: DMA channel is busy."]
    BUSY = 1,
}
impl From<BUSY20_A> for bool {
    #[inline(always)]
    fn from(variant: BUSY20_A) -> Self {
        variant as u8 != 0
    }
}
impl BUSY20_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BUSY20_A {
        match self.bits {
            false => BUSY20_A::NOT_BUSY,
            true => BUSY20_A::BUSY,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_BUSY`"]
    #[inline(always)]
    pub fn is_not_busy(&self) -> bool {
        *self == BUSY20_A::NOT_BUSY
    }
    #[doc = "Checks if the value of the field is `BUSY`"]
    #[inline(always)]
    pub fn is_busy(&self) -> bool {
        *self == BUSY20_A::BUSY
    }
}
#[doc = "Field `BUSY21` reader - Busy flag for DMA channel."]
pub type BUSY21_R = crate::BitReader<BUSY21_A>;
#[doc = "Busy flag for DMA channel.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BUSY21_A {
    #[doc = "0: DMA channel is not busy."]
    NOT_BUSY = 0,
    #[doc = "1: DMA channel is busy."]
    BUSY = 1,
}
impl From<BUSY21_A> for bool {
    #[inline(always)]
    fn from(variant: BUSY21_A) -> Self {
        variant as u8 != 0
    }
}
impl BUSY21_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BUSY21_A {
        match self.bits {
            false => BUSY21_A::NOT_BUSY,
            true => BUSY21_A::BUSY,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_BUSY`"]
    #[inline(always)]
    pub fn is_not_busy(&self) -> bool {
        *self == BUSY21_A::NOT_BUSY
    }
    #[doc = "Checks if the value of the field is `BUSY`"]
    #[inline(always)]
    pub fn is_busy(&self) -> bool {
        *self == BUSY21_A::BUSY
    }
}
#[doc = "Field `BUSY22` reader - Busy flag for DMA channel."]
pub type BUSY22_R = crate::BitReader<BUSY22_A>;
#[doc = "Busy flag for DMA channel.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BUSY22_A {
    #[doc = "0: DMA channel is not busy."]
    NOT_BUSY = 0,
    #[doc = "1: DMA channel is busy."]
    BUSY = 1,
}
impl From<BUSY22_A> for bool {
    #[inline(always)]
    fn from(variant: BUSY22_A) -> Self {
        variant as u8 != 0
    }
}
impl BUSY22_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BUSY22_A {
        match self.bits {
            false => BUSY22_A::NOT_BUSY,
            true => BUSY22_A::BUSY,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_BUSY`"]
    #[inline(always)]
    pub fn is_not_busy(&self) -> bool {
        *self == BUSY22_A::NOT_BUSY
    }
    #[doc = "Checks if the value of the field is `BUSY`"]
    #[inline(always)]
    pub fn is_busy(&self) -> bool {
        *self == BUSY22_A::BUSY
    }
}
#[doc = "Field `BUSY23` reader - Busy flag for DMA channel."]
pub type BUSY23_R = crate::BitReader<BUSY23_A>;
#[doc = "Busy flag for DMA channel.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BUSY23_A {
    #[doc = "0: DMA channel is not busy."]
    NOT_BUSY = 0,
    #[doc = "1: DMA channel is busy."]
    BUSY = 1,
}
impl From<BUSY23_A> for bool {
    #[inline(always)]
    fn from(variant: BUSY23_A) -> Self {
        variant as u8 != 0
    }
}
impl BUSY23_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BUSY23_A {
        match self.bits {
            false => BUSY23_A::NOT_BUSY,
            true => BUSY23_A::BUSY,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_BUSY`"]
    #[inline(always)]
    pub fn is_not_busy(&self) -> bool {
        *self == BUSY23_A::NOT_BUSY
    }
    #[doc = "Checks if the value of the field is `BUSY`"]
    #[inline(always)]
    pub fn is_busy(&self) -> bool {
        *self == BUSY23_A::BUSY
    }
}
#[doc = "Field `BUSY24` reader - Busy flag for DMA channel."]
pub type BUSY24_R = crate::BitReader<BUSY24_A>;
#[doc = "Busy flag for DMA channel.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BUSY24_A {
    #[doc = "0: DMA channel is not busy."]
    NOT_BUSY = 0,
    #[doc = "1: DMA channel is busy."]
    BUSY = 1,
}
impl From<BUSY24_A> for bool {
    #[inline(always)]
    fn from(variant: BUSY24_A) -> Self {
        variant as u8 != 0
    }
}
impl BUSY24_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BUSY24_A {
        match self.bits {
            false => BUSY24_A::NOT_BUSY,
            true => BUSY24_A::BUSY,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_BUSY`"]
    #[inline(always)]
    pub fn is_not_busy(&self) -> bool {
        *self == BUSY24_A::NOT_BUSY
    }
    #[doc = "Checks if the value of the field is `BUSY`"]
    #[inline(always)]
    pub fn is_busy(&self) -> bool {
        *self == BUSY24_A::BUSY
    }
}
#[doc = "Field `BUSY25` reader - Busy flag for DMA channel."]
pub type BUSY25_R = crate::BitReader<BUSY25_A>;
#[doc = "Busy flag for DMA channel.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BUSY25_A {
    #[doc = "0: DMA channel is not busy."]
    NOT_BUSY = 0,
    #[doc = "1: DMA channel is busy."]
    BUSY = 1,
}
impl From<BUSY25_A> for bool {
    #[inline(always)]
    fn from(variant: BUSY25_A) -> Self {
        variant as u8 != 0
    }
}
impl BUSY25_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BUSY25_A {
        match self.bits {
            false => BUSY25_A::NOT_BUSY,
            true => BUSY25_A::BUSY,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_BUSY`"]
    #[inline(always)]
    pub fn is_not_busy(&self) -> bool {
        *self == BUSY25_A::NOT_BUSY
    }
    #[doc = "Checks if the value of the field is `BUSY`"]
    #[inline(always)]
    pub fn is_busy(&self) -> bool {
        *self == BUSY25_A::BUSY
    }
}
#[doc = "Field `BUSY26` reader - Busy flag for DMA channel."]
pub type BUSY26_R = crate::BitReader<BUSY26_A>;
#[doc = "Busy flag for DMA channel.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BUSY26_A {
    #[doc = "0: DMA channel is not busy."]
    NOT_BUSY = 0,
    #[doc = "1: DMA channel is busy."]
    BUSY = 1,
}
impl From<BUSY26_A> for bool {
    #[inline(always)]
    fn from(variant: BUSY26_A) -> Self {
        variant as u8 != 0
    }
}
impl BUSY26_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BUSY26_A {
        match self.bits {
            false => BUSY26_A::NOT_BUSY,
            true => BUSY26_A::BUSY,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_BUSY`"]
    #[inline(always)]
    pub fn is_not_busy(&self) -> bool {
        *self == BUSY26_A::NOT_BUSY
    }
    #[doc = "Checks if the value of the field is `BUSY`"]
    #[inline(always)]
    pub fn is_busy(&self) -> bool {
        *self == BUSY26_A::BUSY
    }
}
#[doc = "Field `BUSY27` reader - Busy flag for DMA channel."]
pub type BUSY27_R = crate::BitReader<BUSY27_A>;
#[doc = "Busy flag for DMA channel.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BUSY27_A {
    #[doc = "0: DMA channel is not busy."]
    NOT_BUSY = 0,
    #[doc = "1: DMA channel is busy."]
    BUSY = 1,
}
impl From<BUSY27_A> for bool {
    #[inline(always)]
    fn from(variant: BUSY27_A) -> Self {
        variant as u8 != 0
    }
}
impl BUSY27_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BUSY27_A {
        match self.bits {
            false => BUSY27_A::NOT_BUSY,
            true => BUSY27_A::BUSY,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_BUSY`"]
    #[inline(always)]
    pub fn is_not_busy(&self) -> bool {
        *self == BUSY27_A::NOT_BUSY
    }
    #[doc = "Checks if the value of the field is `BUSY`"]
    #[inline(always)]
    pub fn is_busy(&self) -> bool {
        *self == BUSY27_A::BUSY
    }
}
#[doc = "Field `BUSY28` reader - Busy flag for DMA channel."]
pub type BUSY28_R = crate::BitReader<BUSY28_A>;
#[doc = "Busy flag for DMA channel.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BUSY28_A {
    #[doc = "0: DMA channel is not busy."]
    NOT_BUSY = 0,
    #[doc = "1: DMA channel is busy."]
    BUSY = 1,
}
impl From<BUSY28_A> for bool {
    #[inline(always)]
    fn from(variant: BUSY28_A) -> Self {
        variant as u8 != 0
    }
}
impl BUSY28_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BUSY28_A {
        match self.bits {
            false => BUSY28_A::NOT_BUSY,
            true => BUSY28_A::BUSY,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_BUSY`"]
    #[inline(always)]
    pub fn is_not_busy(&self) -> bool {
        *self == BUSY28_A::NOT_BUSY
    }
    #[doc = "Checks if the value of the field is `BUSY`"]
    #[inline(always)]
    pub fn is_busy(&self) -> bool {
        *self == BUSY28_A::BUSY
    }
}
#[doc = "Field `BUSY29` reader - Busy flag for DMA channel."]
pub type BUSY29_R = crate::BitReader<BUSY29_A>;
#[doc = "Busy flag for DMA channel.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BUSY29_A {
    #[doc = "0: DMA channel is not busy."]
    NOT_BUSY = 0,
    #[doc = "1: DMA channel is busy."]
    BUSY = 1,
}
impl From<BUSY29_A> for bool {
    #[inline(always)]
    fn from(variant: BUSY29_A) -> Self {
        variant as u8 != 0
    }
}
impl BUSY29_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BUSY29_A {
        match self.bits {
            false => BUSY29_A::NOT_BUSY,
            true => BUSY29_A::BUSY,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_BUSY`"]
    #[inline(always)]
    pub fn is_not_busy(&self) -> bool {
        *self == BUSY29_A::NOT_BUSY
    }
    #[doc = "Checks if the value of the field is `BUSY`"]
    #[inline(always)]
    pub fn is_busy(&self) -> bool {
        *self == BUSY29_A::BUSY
    }
}
#[doc = "Field `BUSY30` reader - Busy flag for DMA channel."]
pub type BUSY30_R = crate::BitReader<BUSY30_A>;
#[doc = "Busy flag for DMA channel.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BUSY30_A {
    #[doc = "0: DMA channel is not busy."]
    NOT_BUSY = 0,
    #[doc = "1: DMA channel is busy."]
    BUSY = 1,
}
impl From<BUSY30_A> for bool {
    #[inline(always)]
    fn from(variant: BUSY30_A) -> Self {
        variant as u8 != 0
    }
}
impl BUSY30_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BUSY30_A {
        match self.bits {
            false => BUSY30_A::NOT_BUSY,
            true => BUSY30_A::BUSY,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_BUSY`"]
    #[inline(always)]
    pub fn is_not_busy(&self) -> bool {
        *self == BUSY30_A::NOT_BUSY
    }
    #[doc = "Checks if the value of the field is `BUSY`"]
    #[inline(always)]
    pub fn is_busy(&self) -> bool {
        *self == BUSY30_A::BUSY
    }
}
#[doc = "Field `BUSY31` reader - Busy flag for DMA channel."]
pub type BUSY31_R = crate::BitReader<BUSY31_A>;
#[doc = "Busy flag for DMA channel.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BUSY31_A {
    #[doc = "0: DMA channel is not busy."]
    NOT_BUSY = 0,
    #[doc = "1: DMA channel is busy."]
    BUSY = 1,
}
impl From<BUSY31_A> for bool {
    #[inline(always)]
    fn from(variant: BUSY31_A) -> Self {
        variant as u8 != 0
    }
}
impl BUSY31_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BUSY31_A {
        match self.bits {
            false => BUSY31_A::NOT_BUSY,
            true => BUSY31_A::BUSY,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_BUSY`"]
    #[inline(always)]
    pub fn is_not_busy(&self) -> bool {
        *self == BUSY31_A::NOT_BUSY
    }
    #[doc = "Checks if the value of the field is `BUSY`"]
    #[inline(always)]
    pub fn is_busy(&self) -> bool {
        *self == BUSY31_A::BUSY
    }
}
impl R {
    #[doc = "Bit 0 - Busy flag for DMA channel."]
    #[inline(always)]
    pub fn busy0(&self) -> BUSY0_R {
        BUSY0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Busy flag for DMA channel."]
    #[inline(always)]
    pub fn busy1(&self) -> BUSY1_R {
        BUSY1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Busy flag for DMA channel."]
    #[inline(always)]
    pub fn busy2(&self) -> BUSY2_R {
        BUSY2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Busy flag for DMA channel."]
    #[inline(always)]
    pub fn busy3(&self) -> BUSY3_R {
        BUSY3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Busy flag for DMA channel."]
    #[inline(always)]
    pub fn busy4(&self) -> BUSY4_R {
        BUSY4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Busy flag for DMA channel."]
    #[inline(always)]
    pub fn busy5(&self) -> BUSY5_R {
        BUSY5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Busy flag for DMA channel."]
    #[inline(always)]
    pub fn busy6(&self) -> BUSY6_R {
        BUSY6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Busy flag for DMA channel."]
    #[inline(always)]
    pub fn busy7(&self) -> BUSY7_R {
        BUSY7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Busy flag for DMA channel."]
    #[inline(always)]
    pub fn busy8(&self) -> BUSY8_R {
        BUSY8_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Busy flag for DMA channel."]
    #[inline(always)]
    pub fn busy9(&self) -> BUSY9_R {
        BUSY9_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Busy flag for DMA channel."]
    #[inline(always)]
    pub fn busy10(&self) -> BUSY10_R {
        BUSY10_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Busy flag for DMA channel."]
    #[inline(always)]
    pub fn busy11(&self) -> BUSY11_R {
        BUSY11_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Busy flag for DMA channel."]
    #[inline(always)]
    pub fn busy12(&self) -> BUSY12_R {
        BUSY12_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Busy flag for DMA channel."]
    #[inline(always)]
    pub fn busy13(&self) -> BUSY13_R {
        BUSY13_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Busy flag for DMA channel."]
    #[inline(always)]
    pub fn busy14(&self) -> BUSY14_R {
        BUSY14_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Busy flag for DMA channel."]
    #[inline(always)]
    pub fn busy15(&self) -> BUSY15_R {
        BUSY15_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Busy flag for DMA channel."]
    #[inline(always)]
    pub fn busy16(&self) -> BUSY16_R {
        BUSY16_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Busy flag for DMA channel."]
    #[inline(always)]
    pub fn busy17(&self) -> BUSY17_R {
        BUSY17_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Busy flag for DMA channel."]
    #[inline(always)]
    pub fn busy18(&self) -> BUSY18_R {
        BUSY18_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Busy flag for DMA channel."]
    #[inline(always)]
    pub fn busy19(&self) -> BUSY19_R {
        BUSY19_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Busy flag for DMA channel."]
    #[inline(always)]
    pub fn busy20(&self) -> BUSY20_R {
        BUSY20_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Busy flag for DMA channel."]
    #[inline(always)]
    pub fn busy21(&self) -> BUSY21_R {
        BUSY21_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Busy flag for DMA channel."]
    #[inline(always)]
    pub fn busy22(&self) -> BUSY22_R {
        BUSY22_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Busy flag for DMA channel."]
    #[inline(always)]
    pub fn busy23(&self) -> BUSY23_R {
        BUSY23_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Busy flag for DMA channel."]
    #[inline(always)]
    pub fn busy24(&self) -> BUSY24_R {
        BUSY24_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Busy flag for DMA channel."]
    #[inline(always)]
    pub fn busy25(&self) -> BUSY25_R {
        BUSY25_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Busy flag for DMA channel."]
    #[inline(always)]
    pub fn busy26(&self) -> BUSY26_R {
        BUSY26_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Busy flag for DMA channel."]
    #[inline(always)]
    pub fn busy27(&self) -> BUSY27_R {
        BUSY27_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Busy flag for DMA channel."]
    #[inline(always)]
    pub fn busy28(&self) -> BUSY28_R {
        BUSY28_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Busy flag for DMA channel."]
    #[inline(always)]
    pub fn busy29(&self) -> BUSY29_R {
        BUSY29_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Busy flag for DMA channel."]
    #[inline(always)]
    pub fn busy30(&self) -> BUSY30_R {
        BUSY30_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Busy flag for DMA channel."]
    #[inline(always)]
    pub fn busy31(&self) -> BUSY31_R {
        BUSY31_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[doc = "Channel Busy status for all DMA channels\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [busy0](index.html) module"]
pub struct BUSY0_SPEC;
impl crate::RegisterSpec for BUSY0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [busy0::R](R) reader structure"]
impl crate::Readable for BUSY0_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets BUSY0 to value 0"]
impl crate::Resettable for BUSY0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
