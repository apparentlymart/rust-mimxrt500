#[doc = "Register `ACTIVE1` reader"]
pub struct R(crate::R<ACTIVE1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ACTIVE1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ACTIVE1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ACTIVE1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `ACTIVE32` reader - Active flag for DMA channel."]
pub type ACTIVE32_R = crate::BitReader<ACTIVE32_A>;
#[doc = "Active flag for DMA channel.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ACTIVE32_A {
    #[doc = "0: DMA channel is not active."]
    NOT_ACTIVE = 0,
    #[doc = "1: DMA channel is active."]
    ACTIVE = 1,
}
impl From<ACTIVE32_A> for bool {
    #[inline(always)]
    fn from(variant: ACTIVE32_A) -> Self {
        variant as u8 != 0
    }
}
impl ACTIVE32_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ACTIVE32_A {
        match self.bits {
            false => ACTIVE32_A::NOT_ACTIVE,
            true => ACTIVE32_A::ACTIVE,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_ACTIVE`"]
    #[inline(always)]
    pub fn is_not_active(&self) -> bool {
        *self == ACTIVE32_A::NOT_ACTIVE
    }
    #[doc = "Checks if the value of the field is `ACTIVE`"]
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        *self == ACTIVE32_A::ACTIVE
    }
}
#[doc = "Field `ACTIVE33` reader - Active flag for DMA channel."]
pub type ACTIVE33_R = crate::BitReader<ACTIVE33_A>;
#[doc = "Active flag for DMA channel.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ACTIVE33_A {
    #[doc = "0: DMA channel is not active."]
    NOT_ACTIVE = 0,
    #[doc = "1: DMA channel is active."]
    ACTIVE = 1,
}
impl From<ACTIVE33_A> for bool {
    #[inline(always)]
    fn from(variant: ACTIVE33_A) -> Self {
        variant as u8 != 0
    }
}
impl ACTIVE33_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ACTIVE33_A {
        match self.bits {
            false => ACTIVE33_A::NOT_ACTIVE,
            true => ACTIVE33_A::ACTIVE,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_ACTIVE`"]
    #[inline(always)]
    pub fn is_not_active(&self) -> bool {
        *self == ACTIVE33_A::NOT_ACTIVE
    }
    #[doc = "Checks if the value of the field is `ACTIVE`"]
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        *self == ACTIVE33_A::ACTIVE
    }
}
#[doc = "Field `ACTIVE34` reader - Active flag for DMA channel."]
pub type ACTIVE34_R = crate::BitReader<ACTIVE34_A>;
#[doc = "Active flag for DMA channel.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ACTIVE34_A {
    #[doc = "0: DMA channel is not active."]
    NOT_ACTIVE = 0,
    #[doc = "1: DMA channel is active."]
    ACTIVE = 1,
}
impl From<ACTIVE34_A> for bool {
    #[inline(always)]
    fn from(variant: ACTIVE34_A) -> Self {
        variant as u8 != 0
    }
}
impl ACTIVE34_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ACTIVE34_A {
        match self.bits {
            false => ACTIVE34_A::NOT_ACTIVE,
            true => ACTIVE34_A::ACTIVE,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_ACTIVE`"]
    #[inline(always)]
    pub fn is_not_active(&self) -> bool {
        *self == ACTIVE34_A::NOT_ACTIVE
    }
    #[doc = "Checks if the value of the field is `ACTIVE`"]
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        *self == ACTIVE34_A::ACTIVE
    }
}
#[doc = "Field `ACTIVE35` reader - Active flag for DMA channel."]
pub type ACTIVE35_R = crate::BitReader<ACTIVE35_A>;
#[doc = "Active flag for DMA channel.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ACTIVE35_A {
    #[doc = "0: DMA channel is not active."]
    NOT_ACTIVE = 0,
    #[doc = "1: DMA channel is active."]
    ACTIVE = 1,
}
impl From<ACTIVE35_A> for bool {
    #[inline(always)]
    fn from(variant: ACTIVE35_A) -> Self {
        variant as u8 != 0
    }
}
impl ACTIVE35_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ACTIVE35_A {
        match self.bits {
            false => ACTIVE35_A::NOT_ACTIVE,
            true => ACTIVE35_A::ACTIVE,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_ACTIVE`"]
    #[inline(always)]
    pub fn is_not_active(&self) -> bool {
        *self == ACTIVE35_A::NOT_ACTIVE
    }
    #[doc = "Checks if the value of the field is `ACTIVE`"]
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        *self == ACTIVE35_A::ACTIVE
    }
}
#[doc = "Field `ACTIVE36` reader - Active flag for DMA channel."]
pub type ACTIVE36_R = crate::BitReader<ACTIVE36_A>;
#[doc = "Active flag for DMA channel.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ACTIVE36_A {
    #[doc = "0: DMA channel is not active."]
    NOT_ACTIVE = 0,
    #[doc = "1: DMA channel is active."]
    ACTIVE = 1,
}
impl From<ACTIVE36_A> for bool {
    #[inline(always)]
    fn from(variant: ACTIVE36_A) -> Self {
        variant as u8 != 0
    }
}
impl ACTIVE36_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ACTIVE36_A {
        match self.bits {
            false => ACTIVE36_A::NOT_ACTIVE,
            true => ACTIVE36_A::ACTIVE,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_ACTIVE`"]
    #[inline(always)]
    pub fn is_not_active(&self) -> bool {
        *self == ACTIVE36_A::NOT_ACTIVE
    }
    #[doc = "Checks if the value of the field is `ACTIVE`"]
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        *self == ACTIVE36_A::ACTIVE
    }
}
impl R {
    #[doc = "Bit 0 - Active flag for DMA channel."]
    #[inline(always)]
    pub fn active32(&self) -> ACTIVE32_R {
        ACTIVE32_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Active flag for DMA channel."]
    #[inline(always)]
    pub fn active33(&self) -> ACTIVE33_R {
        ACTIVE33_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Active flag for DMA channel."]
    #[inline(always)]
    pub fn active34(&self) -> ACTIVE34_R {
        ACTIVE34_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Active flag for DMA channel."]
    #[inline(always)]
    pub fn active35(&self) -> ACTIVE35_R {
        ACTIVE35_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Active flag for DMA channel."]
    #[inline(always)]
    pub fn active36(&self) -> ACTIVE36_R {
        ACTIVE36_R::new(((self.bits >> 4) & 1) != 0)
    }
}
#[doc = "Channel Active status for all DMA channels\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [active1](index.html) module"]
pub struct ACTIVE1_SPEC;
impl crate::RegisterSpec for ACTIVE1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [active1::R](R) reader structure"]
impl crate::Readable for ACTIVE1_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets ACTIVE1 to value 0"]
impl crate::Resettable for ACTIVE1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
