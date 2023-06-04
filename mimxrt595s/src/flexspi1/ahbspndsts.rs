#[doc = "Register `AHBSPNDSTS` reader"]
pub struct R(crate::R<AHBSPNDSTS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<AHBSPNDSTS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<AHBSPNDSTS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<AHBSPNDSTS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `ACTIVE` reader - Indicates if an AHB read prefetch command sequence has been suspended."]
pub type ACTIVE_R = crate::BitReader<ACTIVE_A>;
#[doc = "Indicates if an AHB read prefetch command sequence has been suspended.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ACTIVE_A {
    #[doc = "0: No suspended AHB read prefetch command."]
    VAL0 = 0,
    #[doc = "1: An AHB read prefetch command sequence has been suspended."]
    VAL1 = 1,
}
impl From<ACTIVE_A> for bool {
    #[inline(always)]
    fn from(variant: ACTIVE_A) -> Self {
        variant as u8 != 0
    }
}
impl ACTIVE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ACTIVE_A {
        match self.bits {
            false => ACTIVE_A::VAL0,
            true => ACTIVE_A::VAL1,
        }
    }
    #[doc = "Checks if the value of the field is `VAL0`"]
    #[inline(always)]
    pub fn is_val0(&self) -> bool {
        *self == ACTIVE_A::VAL0
    }
    #[doc = "Checks if the value of the field is `VAL1`"]
    #[inline(always)]
    pub fn is_val1(&self) -> bool {
        *self == ACTIVE_A::VAL1
    }
}
#[doc = "Field `BUFID` reader - AHB RX BUF ID for suspended command sequence."]
pub type BUFID_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DATLFT` reader - The Data size left for suspended command sequence (in byte)."]
pub type DATLFT_R = crate::FieldReader<u16, u16>;
impl R {
    #[doc = "Bit 0 - Indicates if an AHB read prefetch command sequence has been suspended."]
    #[inline(always)]
    pub fn active(&self) -> ACTIVE_R {
        ACTIVE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:3 - AHB RX BUF ID for suspended command sequence."]
    #[inline(always)]
    pub fn bufid(&self) -> BUFID_R {
        BUFID_R::new(((self.bits >> 1) & 7) as u8)
    }
    #[doc = "Bits 16:31 - The Data size left for suspended command sequence (in byte)."]
    #[inline(always)]
    pub fn datlft(&self) -> DATLFT_R {
        DATLFT_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
#[doc = "AHB Suspend Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ahbspndsts](index.html) module"]
pub struct AHBSPNDSTS_SPEC;
impl crate::RegisterSpec for AHBSPNDSTS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ahbspndsts::R](R) reader structure"]
impl crate::Readable for AHBSPNDSTS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets AHBSPNDSTS to value 0"]
impl crate::Resettable for AHBSPNDSTS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
