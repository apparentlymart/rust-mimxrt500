#[doc = "Register `BUSY1` reader"]
pub struct R(crate::R<BUSY1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BUSY1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BUSY1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BUSY1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `BUSY32` reader - Busy flag for DMA channel."]
pub type BUSY32_R = crate::BitReader<BUSY32_A>;
#[doc = "Busy flag for DMA channel.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BUSY32_A {
    #[doc = "0: DMA channel is not busy."]
    NOT_BUSY = 0,
    #[doc = "1: DMA channel is busy."]
    BUSY = 1,
}
impl From<BUSY32_A> for bool {
    #[inline(always)]
    fn from(variant: BUSY32_A) -> Self {
        variant as u8 != 0
    }
}
impl BUSY32_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BUSY32_A {
        match self.bits {
            false => BUSY32_A::NOT_BUSY,
            true => BUSY32_A::BUSY,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_BUSY`"]
    #[inline(always)]
    pub fn is_not_busy(&self) -> bool {
        *self == BUSY32_A::NOT_BUSY
    }
    #[doc = "Checks if the value of the field is `BUSY`"]
    #[inline(always)]
    pub fn is_busy(&self) -> bool {
        *self == BUSY32_A::BUSY
    }
}
#[doc = "Field `BUSY33` reader - Busy flag for DMA channel."]
pub type BUSY33_R = crate::BitReader<BUSY33_A>;
#[doc = "Busy flag for DMA channel.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BUSY33_A {
    #[doc = "0: DMA channel is not busy."]
    NOT_BUSY = 0,
    #[doc = "1: DMA channel is busy."]
    BUSY = 1,
}
impl From<BUSY33_A> for bool {
    #[inline(always)]
    fn from(variant: BUSY33_A) -> Self {
        variant as u8 != 0
    }
}
impl BUSY33_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BUSY33_A {
        match self.bits {
            false => BUSY33_A::NOT_BUSY,
            true => BUSY33_A::BUSY,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_BUSY`"]
    #[inline(always)]
    pub fn is_not_busy(&self) -> bool {
        *self == BUSY33_A::NOT_BUSY
    }
    #[doc = "Checks if the value of the field is `BUSY`"]
    #[inline(always)]
    pub fn is_busy(&self) -> bool {
        *self == BUSY33_A::BUSY
    }
}
#[doc = "Field `BUSY34` reader - Busy flag for DMA channel."]
pub type BUSY34_R = crate::BitReader<BUSY34_A>;
#[doc = "Busy flag for DMA channel.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BUSY34_A {
    #[doc = "0: DMA channel is not busy."]
    NOT_BUSY = 0,
    #[doc = "1: DMA channel is busy."]
    BUSY = 1,
}
impl From<BUSY34_A> for bool {
    #[inline(always)]
    fn from(variant: BUSY34_A) -> Self {
        variant as u8 != 0
    }
}
impl BUSY34_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BUSY34_A {
        match self.bits {
            false => BUSY34_A::NOT_BUSY,
            true => BUSY34_A::BUSY,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_BUSY`"]
    #[inline(always)]
    pub fn is_not_busy(&self) -> bool {
        *self == BUSY34_A::NOT_BUSY
    }
    #[doc = "Checks if the value of the field is `BUSY`"]
    #[inline(always)]
    pub fn is_busy(&self) -> bool {
        *self == BUSY34_A::BUSY
    }
}
#[doc = "Field `BUSY35` reader - Busy flag for DMA channel."]
pub type BUSY35_R = crate::BitReader<BUSY35_A>;
#[doc = "Busy flag for DMA channel.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BUSY35_A {
    #[doc = "0: DMA channel is not busy."]
    NOT_BUSY = 0,
    #[doc = "1: DMA channel is busy."]
    BUSY = 1,
}
impl From<BUSY35_A> for bool {
    #[inline(always)]
    fn from(variant: BUSY35_A) -> Self {
        variant as u8 != 0
    }
}
impl BUSY35_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BUSY35_A {
        match self.bits {
            false => BUSY35_A::NOT_BUSY,
            true => BUSY35_A::BUSY,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_BUSY`"]
    #[inline(always)]
    pub fn is_not_busy(&self) -> bool {
        *self == BUSY35_A::NOT_BUSY
    }
    #[doc = "Checks if the value of the field is `BUSY`"]
    #[inline(always)]
    pub fn is_busy(&self) -> bool {
        *self == BUSY35_A::BUSY
    }
}
#[doc = "Field `BUSY36` reader - Busy flag for DMA channel."]
pub type BUSY36_R = crate::BitReader<BUSY36_A>;
#[doc = "Busy flag for DMA channel.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BUSY36_A {
    #[doc = "0: DMA channel is not busy."]
    NOT_BUSY = 0,
    #[doc = "1: DMA channel is busy."]
    BUSY = 1,
}
impl From<BUSY36_A> for bool {
    #[inline(always)]
    fn from(variant: BUSY36_A) -> Self {
        variant as u8 != 0
    }
}
impl BUSY36_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BUSY36_A {
        match self.bits {
            false => BUSY36_A::NOT_BUSY,
            true => BUSY36_A::BUSY,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_BUSY`"]
    #[inline(always)]
    pub fn is_not_busy(&self) -> bool {
        *self == BUSY36_A::NOT_BUSY
    }
    #[doc = "Checks if the value of the field is `BUSY`"]
    #[inline(always)]
    pub fn is_busy(&self) -> bool {
        *self == BUSY36_A::BUSY
    }
}
impl R {
    #[doc = "Bit 0 - Busy flag for DMA channel."]
    #[inline(always)]
    pub fn busy32(&self) -> BUSY32_R {
        BUSY32_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Busy flag for DMA channel."]
    #[inline(always)]
    pub fn busy33(&self) -> BUSY33_R {
        BUSY33_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Busy flag for DMA channel."]
    #[inline(always)]
    pub fn busy34(&self) -> BUSY34_R {
        BUSY34_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Busy flag for DMA channel."]
    #[inline(always)]
    pub fn busy35(&self) -> BUSY35_R {
        BUSY35_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Busy flag for DMA channel."]
    #[inline(always)]
    pub fn busy36(&self) -> BUSY36_R {
        BUSY36_R::new(((self.bits >> 4) & 1) != 0)
    }
}
#[doc = "Channel Busy status for all DMA channels\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [busy1](index.html) module"]
pub struct BUSY1_SPEC;
impl crate::RegisterSpec for BUSY1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [busy1::R](R) reader structure"]
impl crate::Readable for BUSY1_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets BUSY1 to value 0"]
impl crate::Resettable for BUSY1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
