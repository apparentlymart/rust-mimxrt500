#[doc = "Register `ACTIVE0` reader"]
pub struct R(crate::R<ACTIVE0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ACTIVE0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ACTIVE0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ACTIVE0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `ACTIVE0` reader - Active flag for DMA channel."]
pub type ACTIVE0_R = crate::BitReader<ACTIVE0_A>;
#[doc = "Active flag for DMA channel.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ACTIVE0_A {
    #[doc = "0: DMA channel is not active."]
    NOT_ACTIVE = 0,
    #[doc = "1: DMA channel is active."]
    ACTIVE = 1,
}
impl From<ACTIVE0_A> for bool {
    #[inline(always)]
    fn from(variant: ACTIVE0_A) -> Self {
        variant as u8 != 0
    }
}
impl ACTIVE0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ACTIVE0_A {
        match self.bits {
            false => ACTIVE0_A::NOT_ACTIVE,
            true => ACTIVE0_A::ACTIVE,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_ACTIVE`"]
    #[inline(always)]
    pub fn is_not_active(&self) -> bool {
        *self == ACTIVE0_A::NOT_ACTIVE
    }
    #[doc = "Checks if the value of the field is `ACTIVE`"]
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        *self == ACTIVE0_A::ACTIVE
    }
}
#[doc = "Field `ACTIVE1` reader - Active flag for DMA channel."]
pub type ACTIVE1_R = crate::BitReader<ACTIVE1_A>;
#[doc = "Active flag for DMA channel.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ACTIVE1_A {
    #[doc = "0: DMA channel is not active."]
    NOT_ACTIVE = 0,
    #[doc = "1: DMA channel is active."]
    ACTIVE = 1,
}
impl From<ACTIVE1_A> for bool {
    #[inline(always)]
    fn from(variant: ACTIVE1_A) -> Self {
        variant as u8 != 0
    }
}
impl ACTIVE1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ACTIVE1_A {
        match self.bits {
            false => ACTIVE1_A::NOT_ACTIVE,
            true => ACTIVE1_A::ACTIVE,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_ACTIVE`"]
    #[inline(always)]
    pub fn is_not_active(&self) -> bool {
        *self == ACTIVE1_A::NOT_ACTIVE
    }
    #[doc = "Checks if the value of the field is `ACTIVE`"]
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        *self == ACTIVE1_A::ACTIVE
    }
}
#[doc = "Field `ACTIVE2` reader - Active flag for DMA channel."]
pub type ACTIVE2_R = crate::BitReader<ACTIVE2_A>;
#[doc = "Active flag for DMA channel.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ACTIVE2_A {
    #[doc = "0: DMA channel is not active."]
    NOT_ACTIVE = 0,
    #[doc = "1: DMA channel is active."]
    ACTIVE = 1,
}
impl From<ACTIVE2_A> for bool {
    #[inline(always)]
    fn from(variant: ACTIVE2_A) -> Self {
        variant as u8 != 0
    }
}
impl ACTIVE2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ACTIVE2_A {
        match self.bits {
            false => ACTIVE2_A::NOT_ACTIVE,
            true => ACTIVE2_A::ACTIVE,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_ACTIVE`"]
    #[inline(always)]
    pub fn is_not_active(&self) -> bool {
        *self == ACTIVE2_A::NOT_ACTIVE
    }
    #[doc = "Checks if the value of the field is `ACTIVE`"]
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        *self == ACTIVE2_A::ACTIVE
    }
}
#[doc = "Field `ACTIVE3` reader - Active flag for DMA channel."]
pub type ACTIVE3_R = crate::BitReader<ACTIVE3_A>;
#[doc = "Active flag for DMA channel.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ACTIVE3_A {
    #[doc = "0: DMA channel is not active."]
    NOT_ACTIVE = 0,
    #[doc = "1: DMA channel is active."]
    ACTIVE = 1,
}
impl From<ACTIVE3_A> for bool {
    #[inline(always)]
    fn from(variant: ACTIVE3_A) -> Self {
        variant as u8 != 0
    }
}
impl ACTIVE3_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ACTIVE3_A {
        match self.bits {
            false => ACTIVE3_A::NOT_ACTIVE,
            true => ACTIVE3_A::ACTIVE,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_ACTIVE`"]
    #[inline(always)]
    pub fn is_not_active(&self) -> bool {
        *self == ACTIVE3_A::NOT_ACTIVE
    }
    #[doc = "Checks if the value of the field is `ACTIVE`"]
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        *self == ACTIVE3_A::ACTIVE
    }
}
#[doc = "Field `ACTIVE4` reader - Active flag for DMA channel."]
pub type ACTIVE4_R = crate::BitReader<ACTIVE4_A>;
#[doc = "Active flag for DMA channel.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ACTIVE4_A {
    #[doc = "0: DMA channel is not active."]
    NOT_ACTIVE = 0,
    #[doc = "1: DMA channel is active."]
    ACTIVE = 1,
}
impl From<ACTIVE4_A> for bool {
    #[inline(always)]
    fn from(variant: ACTIVE4_A) -> Self {
        variant as u8 != 0
    }
}
impl ACTIVE4_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ACTIVE4_A {
        match self.bits {
            false => ACTIVE4_A::NOT_ACTIVE,
            true => ACTIVE4_A::ACTIVE,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_ACTIVE`"]
    #[inline(always)]
    pub fn is_not_active(&self) -> bool {
        *self == ACTIVE4_A::NOT_ACTIVE
    }
    #[doc = "Checks if the value of the field is `ACTIVE`"]
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        *self == ACTIVE4_A::ACTIVE
    }
}
#[doc = "Field `ACTIVE5` reader - Active flag for DMA channel."]
pub type ACTIVE5_R = crate::BitReader<ACTIVE5_A>;
#[doc = "Active flag for DMA channel.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ACTIVE5_A {
    #[doc = "0: DMA channel is not active."]
    NOT_ACTIVE = 0,
    #[doc = "1: DMA channel is active."]
    ACTIVE = 1,
}
impl From<ACTIVE5_A> for bool {
    #[inline(always)]
    fn from(variant: ACTIVE5_A) -> Self {
        variant as u8 != 0
    }
}
impl ACTIVE5_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ACTIVE5_A {
        match self.bits {
            false => ACTIVE5_A::NOT_ACTIVE,
            true => ACTIVE5_A::ACTIVE,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_ACTIVE`"]
    #[inline(always)]
    pub fn is_not_active(&self) -> bool {
        *self == ACTIVE5_A::NOT_ACTIVE
    }
    #[doc = "Checks if the value of the field is `ACTIVE`"]
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        *self == ACTIVE5_A::ACTIVE
    }
}
#[doc = "Field `ACTIVE6` reader - Active flag for DMA channel."]
pub type ACTIVE6_R = crate::BitReader<ACTIVE6_A>;
#[doc = "Active flag for DMA channel.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ACTIVE6_A {
    #[doc = "0: DMA channel is not active."]
    NOT_ACTIVE = 0,
    #[doc = "1: DMA channel is active."]
    ACTIVE = 1,
}
impl From<ACTIVE6_A> for bool {
    #[inline(always)]
    fn from(variant: ACTIVE6_A) -> Self {
        variant as u8 != 0
    }
}
impl ACTIVE6_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ACTIVE6_A {
        match self.bits {
            false => ACTIVE6_A::NOT_ACTIVE,
            true => ACTIVE6_A::ACTIVE,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_ACTIVE`"]
    #[inline(always)]
    pub fn is_not_active(&self) -> bool {
        *self == ACTIVE6_A::NOT_ACTIVE
    }
    #[doc = "Checks if the value of the field is `ACTIVE`"]
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        *self == ACTIVE6_A::ACTIVE
    }
}
#[doc = "Field `ACTIVE7` reader - Active flag for DMA channel."]
pub type ACTIVE7_R = crate::BitReader<ACTIVE7_A>;
#[doc = "Active flag for DMA channel.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ACTIVE7_A {
    #[doc = "0: DMA channel is not active."]
    NOT_ACTIVE = 0,
    #[doc = "1: DMA channel is active."]
    ACTIVE = 1,
}
impl From<ACTIVE7_A> for bool {
    #[inline(always)]
    fn from(variant: ACTIVE7_A) -> Self {
        variant as u8 != 0
    }
}
impl ACTIVE7_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ACTIVE7_A {
        match self.bits {
            false => ACTIVE7_A::NOT_ACTIVE,
            true => ACTIVE7_A::ACTIVE,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_ACTIVE`"]
    #[inline(always)]
    pub fn is_not_active(&self) -> bool {
        *self == ACTIVE7_A::NOT_ACTIVE
    }
    #[doc = "Checks if the value of the field is `ACTIVE`"]
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        *self == ACTIVE7_A::ACTIVE
    }
}
#[doc = "Field `ACTIVE8` reader - Active flag for DMA channel."]
pub type ACTIVE8_R = crate::BitReader<ACTIVE8_A>;
#[doc = "Active flag for DMA channel.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ACTIVE8_A {
    #[doc = "0: DMA channel is not active."]
    NOT_ACTIVE = 0,
    #[doc = "1: DMA channel is active."]
    ACTIVE = 1,
}
impl From<ACTIVE8_A> for bool {
    #[inline(always)]
    fn from(variant: ACTIVE8_A) -> Self {
        variant as u8 != 0
    }
}
impl ACTIVE8_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ACTIVE8_A {
        match self.bits {
            false => ACTIVE8_A::NOT_ACTIVE,
            true => ACTIVE8_A::ACTIVE,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_ACTIVE`"]
    #[inline(always)]
    pub fn is_not_active(&self) -> bool {
        *self == ACTIVE8_A::NOT_ACTIVE
    }
    #[doc = "Checks if the value of the field is `ACTIVE`"]
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        *self == ACTIVE8_A::ACTIVE
    }
}
#[doc = "Field `ACTIVE9` reader - Active flag for DMA channel."]
pub type ACTIVE9_R = crate::BitReader<ACTIVE9_A>;
#[doc = "Active flag for DMA channel.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ACTIVE9_A {
    #[doc = "0: DMA channel is not active."]
    NOT_ACTIVE = 0,
    #[doc = "1: DMA channel is active."]
    ACTIVE = 1,
}
impl From<ACTIVE9_A> for bool {
    #[inline(always)]
    fn from(variant: ACTIVE9_A) -> Self {
        variant as u8 != 0
    }
}
impl ACTIVE9_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ACTIVE9_A {
        match self.bits {
            false => ACTIVE9_A::NOT_ACTIVE,
            true => ACTIVE9_A::ACTIVE,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_ACTIVE`"]
    #[inline(always)]
    pub fn is_not_active(&self) -> bool {
        *self == ACTIVE9_A::NOT_ACTIVE
    }
    #[doc = "Checks if the value of the field is `ACTIVE`"]
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        *self == ACTIVE9_A::ACTIVE
    }
}
#[doc = "Field `ACTIVE10` reader - Active flag for DMA channel."]
pub type ACTIVE10_R = crate::BitReader<ACTIVE10_A>;
#[doc = "Active flag for DMA channel.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ACTIVE10_A {
    #[doc = "0: DMA channel is not active."]
    NOT_ACTIVE = 0,
    #[doc = "1: DMA channel is active."]
    ACTIVE = 1,
}
impl From<ACTIVE10_A> for bool {
    #[inline(always)]
    fn from(variant: ACTIVE10_A) -> Self {
        variant as u8 != 0
    }
}
impl ACTIVE10_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ACTIVE10_A {
        match self.bits {
            false => ACTIVE10_A::NOT_ACTIVE,
            true => ACTIVE10_A::ACTIVE,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_ACTIVE`"]
    #[inline(always)]
    pub fn is_not_active(&self) -> bool {
        *self == ACTIVE10_A::NOT_ACTIVE
    }
    #[doc = "Checks if the value of the field is `ACTIVE`"]
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        *self == ACTIVE10_A::ACTIVE
    }
}
#[doc = "Field `ACTIVE11` reader - Active flag for DMA channel."]
pub type ACTIVE11_R = crate::BitReader<ACTIVE11_A>;
#[doc = "Active flag for DMA channel.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ACTIVE11_A {
    #[doc = "0: DMA channel is not active."]
    NOT_ACTIVE = 0,
    #[doc = "1: DMA channel is active."]
    ACTIVE = 1,
}
impl From<ACTIVE11_A> for bool {
    #[inline(always)]
    fn from(variant: ACTIVE11_A) -> Self {
        variant as u8 != 0
    }
}
impl ACTIVE11_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ACTIVE11_A {
        match self.bits {
            false => ACTIVE11_A::NOT_ACTIVE,
            true => ACTIVE11_A::ACTIVE,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_ACTIVE`"]
    #[inline(always)]
    pub fn is_not_active(&self) -> bool {
        *self == ACTIVE11_A::NOT_ACTIVE
    }
    #[doc = "Checks if the value of the field is `ACTIVE`"]
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        *self == ACTIVE11_A::ACTIVE
    }
}
#[doc = "Field `ACTIVE12` reader - Active flag for DMA channel."]
pub type ACTIVE12_R = crate::BitReader<ACTIVE12_A>;
#[doc = "Active flag for DMA channel.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ACTIVE12_A {
    #[doc = "0: DMA channel is not active."]
    NOT_ACTIVE = 0,
    #[doc = "1: DMA channel is active."]
    ACTIVE = 1,
}
impl From<ACTIVE12_A> for bool {
    #[inline(always)]
    fn from(variant: ACTIVE12_A) -> Self {
        variant as u8 != 0
    }
}
impl ACTIVE12_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ACTIVE12_A {
        match self.bits {
            false => ACTIVE12_A::NOT_ACTIVE,
            true => ACTIVE12_A::ACTIVE,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_ACTIVE`"]
    #[inline(always)]
    pub fn is_not_active(&self) -> bool {
        *self == ACTIVE12_A::NOT_ACTIVE
    }
    #[doc = "Checks if the value of the field is `ACTIVE`"]
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        *self == ACTIVE12_A::ACTIVE
    }
}
#[doc = "Field `ACTIVE13` reader - Active flag for DMA channel."]
pub type ACTIVE13_R = crate::BitReader<ACTIVE13_A>;
#[doc = "Active flag for DMA channel.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ACTIVE13_A {
    #[doc = "0: DMA channel is not active."]
    NOT_ACTIVE = 0,
    #[doc = "1: DMA channel is active."]
    ACTIVE = 1,
}
impl From<ACTIVE13_A> for bool {
    #[inline(always)]
    fn from(variant: ACTIVE13_A) -> Self {
        variant as u8 != 0
    }
}
impl ACTIVE13_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ACTIVE13_A {
        match self.bits {
            false => ACTIVE13_A::NOT_ACTIVE,
            true => ACTIVE13_A::ACTIVE,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_ACTIVE`"]
    #[inline(always)]
    pub fn is_not_active(&self) -> bool {
        *self == ACTIVE13_A::NOT_ACTIVE
    }
    #[doc = "Checks if the value of the field is `ACTIVE`"]
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        *self == ACTIVE13_A::ACTIVE
    }
}
#[doc = "Field `ACTIVE14` reader - Active flag for DMA channel."]
pub type ACTIVE14_R = crate::BitReader<ACTIVE14_A>;
#[doc = "Active flag for DMA channel.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ACTIVE14_A {
    #[doc = "0: DMA channel is not active."]
    NOT_ACTIVE = 0,
    #[doc = "1: DMA channel is active."]
    ACTIVE = 1,
}
impl From<ACTIVE14_A> for bool {
    #[inline(always)]
    fn from(variant: ACTIVE14_A) -> Self {
        variant as u8 != 0
    }
}
impl ACTIVE14_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ACTIVE14_A {
        match self.bits {
            false => ACTIVE14_A::NOT_ACTIVE,
            true => ACTIVE14_A::ACTIVE,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_ACTIVE`"]
    #[inline(always)]
    pub fn is_not_active(&self) -> bool {
        *self == ACTIVE14_A::NOT_ACTIVE
    }
    #[doc = "Checks if the value of the field is `ACTIVE`"]
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        *self == ACTIVE14_A::ACTIVE
    }
}
#[doc = "Field `ACTIVE15` reader - Active flag for DMA channel."]
pub type ACTIVE15_R = crate::BitReader<ACTIVE15_A>;
#[doc = "Active flag for DMA channel.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ACTIVE15_A {
    #[doc = "0: DMA channel is not active."]
    NOT_ACTIVE = 0,
    #[doc = "1: DMA channel is active."]
    ACTIVE = 1,
}
impl From<ACTIVE15_A> for bool {
    #[inline(always)]
    fn from(variant: ACTIVE15_A) -> Self {
        variant as u8 != 0
    }
}
impl ACTIVE15_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ACTIVE15_A {
        match self.bits {
            false => ACTIVE15_A::NOT_ACTIVE,
            true => ACTIVE15_A::ACTIVE,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_ACTIVE`"]
    #[inline(always)]
    pub fn is_not_active(&self) -> bool {
        *self == ACTIVE15_A::NOT_ACTIVE
    }
    #[doc = "Checks if the value of the field is `ACTIVE`"]
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        *self == ACTIVE15_A::ACTIVE
    }
}
#[doc = "Field `ACTIVE16` reader - Active flag for DMA channel."]
pub type ACTIVE16_R = crate::BitReader<ACTIVE16_A>;
#[doc = "Active flag for DMA channel.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ACTIVE16_A {
    #[doc = "0: DMA channel is not active."]
    NOT_ACTIVE = 0,
    #[doc = "1: DMA channel is active."]
    ACTIVE = 1,
}
impl From<ACTIVE16_A> for bool {
    #[inline(always)]
    fn from(variant: ACTIVE16_A) -> Self {
        variant as u8 != 0
    }
}
impl ACTIVE16_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ACTIVE16_A {
        match self.bits {
            false => ACTIVE16_A::NOT_ACTIVE,
            true => ACTIVE16_A::ACTIVE,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_ACTIVE`"]
    #[inline(always)]
    pub fn is_not_active(&self) -> bool {
        *self == ACTIVE16_A::NOT_ACTIVE
    }
    #[doc = "Checks if the value of the field is `ACTIVE`"]
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        *self == ACTIVE16_A::ACTIVE
    }
}
#[doc = "Field `ACTIVE17` reader - Active flag for DMA channel."]
pub type ACTIVE17_R = crate::BitReader<ACTIVE17_A>;
#[doc = "Active flag for DMA channel.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ACTIVE17_A {
    #[doc = "0: DMA channel is not active."]
    NOT_ACTIVE = 0,
    #[doc = "1: DMA channel is active."]
    ACTIVE = 1,
}
impl From<ACTIVE17_A> for bool {
    #[inline(always)]
    fn from(variant: ACTIVE17_A) -> Self {
        variant as u8 != 0
    }
}
impl ACTIVE17_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ACTIVE17_A {
        match self.bits {
            false => ACTIVE17_A::NOT_ACTIVE,
            true => ACTIVE17_A::ACTIVE,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_ACTIVE`"]
    #[inline(always)]
    pub fn is_not_active(&self) -> bool {
        *self == ACTIVE17_A::NOT_ACTIVE
    }
    #[doc = "Checks if the value of the field is `ACTIVE`"]
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        *self == ACTIVE17_A::ACTIVE
    }
}
#[doc = "Field `ACTIVE18` reader - Active flag for DMA channel."]
pub type ACTIVE18_R = crate::BitReader<ACTIVE18_A>;
#[doc = "Active flag for DMA channel.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ACTIVE18_A {
    #[doc = "0: DMA channel is not active."]
    NOT_ACTIVE = 0,
    #[doc = "1: DMA channel is active."]
    ACTIVE = 1,
}
impl From<ACTIVE18_A> for bool {
    #[inline(always)]
    fn from(variant: ACTIVE18_A) -> Self {
        variant as u8 != 0
    }
}
impl ACTIVE18_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ACTIVE18_A {
        match self.bits {
            false => ACTIVE18_A::NOT_ACTIVE,
            true => ACTIVE18_A::ACTIVE,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_ACTIVE`"]
    #[inline(always)]
    pub fn is_not_active(&self) -> bool {
        *self == ACTIVE18_A::NOT_ACTIVE
    }
    #[doc = "Checks if the value of the field is `ACTIVE`"]
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        *self == ACTIVE18_A::ACTIVE
    }
}
#[doc = "Field `ACTIVE19` reader - Active flag for DMA channel."]
pub type ACTIVE19_R = crate::BitReader<ACTIVE19_A>;
#[doc = "Active flag for DMA channel.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ACTIVE19_A {
    #[doc = "0: DMA channel is not active."]
    NOT_ACTIVE = 0,
    #[doc = "1: DMA channel is active."]
    ACTIVE = 1,
}
impl From<ACTIVE19_A> for bool {
    #[inline(always)]
    fn from(variant: ACTIVE19_A) -> Self {
        variant as u8 != 0
    }
}
impl ACTIVE19_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ACTIVE19_A {
        match self.bits {
            false => ACTIVE19_A::NOT_ACTIVE,
            true => ACTIVE19_A::ACTIVE,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_ACTIVE`"]
    #[inline(always)]
    pub fn is_not_active(&self) -> bool {
        *self == ACTIVE19_A::NOT_ACTIVE
    }
    #[doc = "Checks if the value of the field is `ACTIVE`"]
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        *self == ACTIVE19_A::ACTIVE
    }
}
#[doc = "Field `ACTIVE20` reader - Active flag for DMA channel."]
pub type ACTIVE20_R = crate::BitReader<ACTIVE20_A>;
#[doc = "Active flag for DMA channel.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ACTIVE20_A {
    #[doc = "0: DMA channel is not active."]
    NOT_ACTIVE = 0,
    #[doc = "1: DMA channel is active."]
    ACTIVE = 1,
}
impl From<ACTIVE20_A> for bool {
    #[inline(always)]
    fn from(variant: ACTIVE20_A) -> Self {
        variant as u8 != 0
    }
}
impl ACTIVE20_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ACTIVE20_A {
        match self.bits {
            false => ACTIVE20_A::NOT_ACTIVE,
            true => ACTIVE20_A::ACTIVE,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_ACTIVE`"]
    #[inline(always)]
    pub fn is_not_active(&self) -> bool {
        *self == ACTIVE20_A::NOT_ACTIVE
    }
    #[doc = "Checks if the value of the field is `ACTIVE`"]
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        *self == ACTIVE20_A::ACTIVE
    }
}
#[doc = "Field `ACTIVE21` reader - Active flag for DMA channel."]
pub type ACTIVE21_R = crate::BitReader<ACTIVE21_A>;
#[doc = "Active flag for DMA channel.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ACTIVE21_A {
    #[doc = "0: DMA channel is not active."]
    NOT_ACTIVE = 0,
    #[doc = "1: DMA channel is active."]
    ACTIVE = 1,
}
impl From<ACTIVE21_A> for bool {
    #[inline(always)]
    fn from(variant: ACTIVE21_A) -> Self {
        variant as u8 != 0
    }
}
impl ACTIVE21_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ACTIVE21_A {
        match self.bits {
            false => ACTIVE21_A::NOT_ACTIVE,
            true => ACTIVE21_A::ACTIVE,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_ACTIVE`"]
    #[inline(always)]
    pub fn is_not_active(&self) -> bool {
        *self == ACTIVE21_A::NOT_ACTIVE
    }
    #[doc = "Checks if the value of the field is `ACTIVE`"]
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        *self == ACTIVE21_A::ACTIVE
    }
}
#[doc = "Field `ACTIVE22` reader - Active flag for DMA channel."]
pub type ACTIVE22_R = crate::BitReader<ACTIVE22_A>;
#[doc = "Active flag for DMA channel.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ACTIVE22_A {
    #[doc = "0: DMA channel is not active."]
    NOT_ACTIVE = 0,
    #[doc = "1: DMA channel is active."]
    ACTIVE = 1,
}
impl From<ACTIVE22_A> for bool {
    #[inline(always)]
    fn from(variant: ACTIVE22_A) -> Self {
        variant as u8 != 0
    }
}
impl ACTIVE22_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ACTIVE22_A {
        match self.bits {
            false => ACTIVE22_A::NOT_ACTIVE,
            true => ACTIVE22_A::ACTIVE,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_ACTIVE`"]
    #[inline(always)]
    pub fn is_not_active(&self) -> bool {
        *self == ACTIVE22_A::NOT_ACTIVE
    }
    #[doc = "Checks if the value of the field is `ACTIVE`"]
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        *self == ACTIVE22_A::ACTIVE
    }
}
#[doc = "Field `ACTIVE23` reader - Active flag for DMA channel."]
pub type ACTIVE23_R = crate::BitReader<ACTIVE23_A>;
#[doc = "Active flag for DMA channel.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ACTIVE23_A {
    #[doc = "0: DMA channel is not active."]
    NOT_ACTIVE = 0,
    #[doc = "1: DMA channel is active."]
    ACTIVE = 1,
}
impl From<ACTIVE23_A> for bool {
    #[inline(always)]
    fn from(variant: ACTIVE23_A) -> Self {
        variant as u8 != 0
    }
}
impl ACTIVE23_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ACTIVE23_A {
        match self.bits {
            false => ACTIVE23_A::NOT_ACTIVE,
            true => ACTIVE23_A::ACTIVE,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_ACTIVE`"]
    #[inline(always)]
    pub fn is_not_active(&self) -> bool {
        *self == ACTIVE23_A::NOT_ACTIVE
    }
    #[doc = "Checks if the value of the field is `ACTIVE`"]
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        *self == ACTIVE23_A::ACTIVE
    }
}
#[doc = "Field `ACTIVE24` reader - Active flag for DMA channel."]
pub type ACTIVE24_R = crate::BitReader<ACTIVE24_A>;
#[doc = "Active flag for DMA channel.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ACTIVE24_A {
    #[doc = "0: DMA channel is not active."]
    NOT_ACTIVE = 0,
    #[doc = "1: DMA channel is active."]
    ACTIVE = 1,
}
impl From<ACTIVE24_A> for bool {
    #[inline(always)]
    fn from(variant: ACTIVE24_A) -> Self {
        variant as u8 != 0
    }
}
impl ACTIVE24_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ACTIVE24_A {
        match self.bits {
            false => ACTIVE24_A::NOT_ACTIVE,
            true => ACTIVE24_A::ACTIVE,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_ACTIVE`"]
    #[inline(always)]
    pub fn is_not_active(&self) -> bool {
        *self == ACTIVE24_A::NOT_ACTIVE
    }
    #[doc = "Checks if the value of the field is `ACTIVE`"]
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        *self == ACTIVE24_A::ACTIVE
    }
}
#[doc = "Field `ACTIVE25` reader - Active flag for DMA channel."]
pub type ACTIVE25_R = crate::BitReader<ACTIVE25_A>;
#[doc = "Active flag for DMA channel.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ACTIVE25_A {
    #[doc = "0: DMA channel is not active."]
    NOT_ACTIVE = 0,
    #[doc = "1: DMA channel is active."]
    ACTIVE = 1,
}
impl From<ACTIVE25_A> for bool {
    #[inline(always)]
    fn from(variant: ACTIVE25_A) -> Self {
        variant as u8 != 0
    }
}
impl ACTIVE25_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ACTIVE25_A {
        match self.bits {
            false => ACTIVE25_A::NOT_ACTIVE,
            true => ACTIVE25_A::ACTIVE,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_ACTIVE`"]
    #[inline(always)]
    pub fn is_not_active(&self) -> bool {
        *self == ACTIVE25_A::NOT_ACTIVE
    }
    #[doc = "Checks if the value of the field is `ACTIVE`"]
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        *self == ACTIVE25_A::ACTIVE
    }
}
#[doc = "Field `ACTIVE26` reader - Active flag for DMA channel."]
pub type ACTIVE26_R = crate::BitReader<ACTIVE26_A>;
#[doc = "Active flag for DMA channel.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ACTIVE26_A {
    #[doc = "0: DMA channel is not active."]
    NOT_ACTIVE = 0,
    #[doc = "1: DMA channel is active."]
    ACTIVE = 1,
}
impl From<ACTIVE26_A> for bool {
    #[inline(always)]
    fn from(variant: ACTIVE26_A) -> Self {
        variant as u8 != 0
    }
}
impl ACTIVE26_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ACTIVE26_A {
        match self.bits {
            false => ACTIVE26_A::NOT_ACTIVE,
            true => ACTIVE26_A::ACTIVE,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_ACTIVE`"]
    #[inline(always)]
    pub fn is_not_active(&self) -> bool {
        *self == ACTIVE26_A::NOT_ACTIVE
    }
    #[doc = "Checks if the value of the field is `ACTIVE`"]
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        *self == ACTIVE26_A::ACTIVE
    }
}
#[doc = "Field `ACTIVE27` reader - Active flag for DMA channel."]
pub type ACTIVE27_R = crate::BitReader<ACTIVE27_A>;
#[doc = "Active flag for DMA channel.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ACTIVE27_A {
    #[doc = "0: DMA channel is not active."]
    NOT_ACTIVE = 0,
    #[doc = "1: DMA channel is active."]
    ACTIVE = 1,
}
impl From<ACTIVE27_A> for bool {
    #[inline(always)]
    fn from(variant: ACTIVE27_A) -> Self {
        variant as u8 != 0
    }
}
impl ACTIVE27_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ACTIVE27_A {
        match self.bits {
            false => ACTIVE27_A::NOT_ACTIVE,
            true => ACTIVE27_A::ACTIVE,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_ACTIVE`"]
    #[inline(always)]
    pub fn is_not_active(&self) -> bool {
        *self == ACTIVE27_A::NOT_ACTIVE
    }
    #[doc = "Checks if the value of the field is `ACTIVE`"]
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        *self == ACTIVE27_A::ACTIVE
    }
}
#[doc = "Field `ACTIVE28` reader - Active flag for DMA channel."]
pub type ACTIVE28_R = crate::BitReader<ACTIVE28_A>;
#[doc = "Active flag for DMA channel.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ACTIVE28_A {
    #[doc = "0: DMA channel is not active."]
    NOT_ACTIVE = 0,
    #[doc = "1: DMA channel is active."]
    ACTIVE = 1,
}
impl From<ACTIVE28_A> for bool {
    #[inline(always)]
    fn from(variant: ACTIVE28_A) -> Self {
        variant as u8 != 0
    }
}
impl ACTIVE28_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ACTIVE28_A {
        match self.bits {
            false => ACTIVE28_A::NOT_ACTIVE,
            true => ACTIVE28_A::ACTIVE,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_ACTIVE`"]
    #[inline(always)]
    pub fn is_not_active(&self) -> bool {
        *self == ACTIVE28_A::NOT_ACTIVE
    }
    #[doc = "Checks if the value of the field is `ACTIVE`"]
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        *self == ACTIVE28_A::ACTIVE
    }
}
#[doc = "Field `ACTIVE29` reader - Active flag for DMA channel."]
pub type ACTIVE29_R = crate::BitReader<ACTIVE29_A>;
#[doc = "Active flag for DMA channel.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ACTIVE29_A {
    #[doc = "0: DMA channel is not active."]
    NOT_ACTIVE = 0,
    #[doc = "1: DMA channel is active."]
    ACTIVE = 1,
}
impl From<ACTIVE29_A> for bool {
    #[inline(always)]
    fn from(variant: ACTIVE29_A) -> Self {
        variant as u8 != 0
    }
}
impl ACTIVE29_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ACTIVE29_A {
        match self.bits {
            false => ACTIVE29_A::NOT_ACTIVE,
            true => ACTIVE29_A::ACTIVE,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_ACTIVE`"]
    #[inline(always)]
    pub fn is_not_active(&self) -> bool {
        *self == ACTIVE29_A::NOT_ACTIVE
    }
    #[doc = "Checks if the value of the field is `ACTIVE`"]
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        *self == ACTIVE29_A::ACTIVE
    }
}
#[doc = "Field `ACTIVE30` reader - Active flag for DMA channel."]
pub type ACTIVE30_R = crate::BitReader<ACTIVE30_A>;
#[doc = "Active flag for DMA channel.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ACTIVE30_A {
    #[doc = "0: DMA channel is not active."]
    NOT_ACTIVE = 0,
    #[doc = "1: DMA channel is active."]
    ACTIVE = 1,
}
impl From<ACTIVE30_A> for bool {
    #[inline(always)]
    fn from(variant: ACTIVE30_A) -> Self {
        variant as u8 != 0
    }
}
impl ACTIVE30_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ACTIVE30_A {
        match self.bits {
            false => ACTIVE30_A::NOT_ACTIVE,
            true => ACTIVE30_A::ACTIVE,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_ACTIVE`"]
    #[inline(always)]
    pub fn is_not_active(&self) -> bool {
        *self == ACTIVE30_A::NOT_ACTIVE
    }
    #[doc = "Checks if the value of the field is `ACTIVE`"]
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        *self == ACTIVE30_A::ACTIVE
    }
}
#[doc = "Field `ACTIVE31` reader - Active flag for DMA channel."]
pub type ACTIVE31_R = crate::BitReader<ACTIVE31_A>;
#[doc = "Active flag for DMA channel.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ACTIVE31_A {
    #[doc = "0: DMA channel is not active."]
    NOT_ACTIVE = 0,
    #[doc = "1: DMA channel is active."]
    ACTIVE = 1,
}
impl From<ACTIVE31_A> for bool {
    #[inline(always)]
    fn from(variant: ACTIVE31_A) -> Self {
        variant as u8 != 0
    }
}
impl ACTIVE31_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ACTIVE31_A {
        match self.bits {
            false => ACTIVE31_A::NOT_ACTIVE,
            true => ACTIVE31_A::ACTIVE,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_ACTIVE`"]
    #[inline(always)]
    pub fn is_not_active(&self) -> bool {
        *self == ACTIVE31_A::NOT_ACTIVE
    }
    #[doc = "Checks if the value of the field is `ACTIVE`"]
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        *self == ACTIVE31_A::ACTIVE
    }
}
impl R {
    #[doc = "Bit 0 - Active flag for DMA channel."]
    #[inline(always)]
    pub fn active0(&self) -> ACTIVE0_R {
        ACTIVE0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Active flag for DMA channel."]
    #[inline(always)]
    pub fn active1(&self) -> ACTIVE1_R {
        ACTIVE1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Active flag for DMA channel."]
    #[inline(always)]
    pub fn active2(&self) -> ACTIVE2_R {
        ACTIVE2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Active flag for DMA channel."]
    #[inline(always)]
    pub fn active3(&self) -> ACTIVE3_R {
        ACTIVE3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Active flag for DMA channel."]
    #[inline(always)]
    pub fn active4(&self) -> ACTIVE4_R {
        ACTIVE4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Active flag for DMA channel."]
    #[inline(always)]
    pub fn active5(&self) -> ACTIVE5_R {
        ACTIVE5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Active flag for DMA channel."]
    #[inline(always)]
    pub fn active6(&self) -> ACTIVE6_R {
        ACTIVE6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Active flag for DMA channel."]
    #[inline(always)]
    pub fn active7(&self) -> ACTIVE7_R {
        ACTIVE7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Active flag for DMA channel."]
    #[inline(always)]
    pub fn active8(&self) -> ACTIVE8_R {
        ACTIVE8_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Active flag for DMA channel."]
    #[inline(always)]
    pub fn active9(&self) -> ACTIVE9_R {
        ACTIVE9_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Active flag for DMA channel."]
    #[inline(always)]
    pub fn active10(&self) -> ACTIVE10_R {
        ACTIVE10_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Active flag for DMA channel."]
    #[inline(always)]
    pub fn active11(&self) -> ACTIVE11_R {
        ACTIVE11_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Active flag for DMA channel."]
    #[inline(always)]
    pub fn active12(&self) -> ACTIVE12_R {
        ACTIVE12_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Active flag for DMA channel."]
    #[inline(always)]
    pub fn active13(&self) -> ACTIVE13_R {
        ACTIVE13_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Active flag for DMA channel."]
    #[inline(always)]
    pub fn active14(&self) -> ACTIVE14_R {
        ACTIVE14_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Active flag for DMA channel."]
    #[inline(always)]
    pub fn active15(&self) -> ACTIVE15_R {
        ACTIVE15_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Active flag for DMA channel."]
    #[inline(always)]
    pub fn active16(&self) -> ACTIVE16_R {
        ACTIVE16_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Active flag for DMA channel."]
    #[inline(always)]
    pub fn active17(&self) -> ACTIVE17_R {
        ACTIVE17_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Active flag for DMA channel."]
    #[inline(always)]
    pub fn active18(&self) -> ACTIVE18_R {
        ACTIVE18_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Active flag for DMA channel."]
    #[inline(always)]
    pub fn active19(&self) -> ACTIVE19_R {
        ACTIVE19_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Active flag for DMA channel."]
    #[inline(always)]
    pub fn active20(&self) -> ACTIVE20_R {
        ACTIVE20_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Active flag for DMA channel."]
    #[inline(always)]
    pub fn active21(&self) -> ACTIVE21_R {
        ACTIVE21_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Active flag for DMA channel."]
    #[inline(always)]
    pub fn active22(&self) -> ACTIVE22_R {
        ACTIVE22_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Active flag for DMA channel."]
    #[inline(always)]
    pub fn active23(&self) -> ACTIVE23_R {
        ACTIVE23_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Active flag for DMA channel."]
    #[inline(always)]
    pub fn active24(&self) -> ACTIVE24_R {
        ACTIVE24_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Active flag for DMA channel."]
    #[inline(always)]
    pub fn active25(&self) -> ACTIVE25_R {
        ACTIVE25_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Active flag for DMA channel."]
    #[inline(always)]
    pub fn active26(&self) -> ACTIVE26_R {
        ACTIVE26_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Active flag for DMA channel."]
    #[inline(always)]
    pub fn active27(&self) -> ACTIVE27_R {
        ACTIVE27_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Active flag for DMA channel."]
    #[inline(always)]
    pub fn active28(&self) -> ACTIVE28_R {
        ACTIVE28_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Active flag for DMA channel."]
    #[inline(always)]
    pub fn active29(&self) -> ACTIVE29_R {
        ACTIVE29_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Active flag for DMA channel."]
    #[inline(always)]
    pub fn active30(&self) -> ACTIVE30_R {
        ACTIVE30_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Active flag for DMA channel."]
    #[inline(always)]
    pub fn active31(&self) -> ACTIVE31_R {
        ACTIVE31_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[doc = "Channel Active status for all DMA channels\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [active0](index.html) module"]
pub struct ACTIVE0_SPEC;
impl crate::RegisterSpec for ACTIVE0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [active0::R](R) reader structure"]
impl crate::Readable for ACTIVE0_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets ACTIVE0 to value 0"]
impl crate::Resettable for ACTIVE0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
