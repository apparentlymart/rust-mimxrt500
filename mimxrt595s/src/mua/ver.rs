#[doc = "Register `VER` reader"]
pub struct R(crate::R<VER_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<VER_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<VER_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<VER_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `FEATURE` reader - Feature Specification Number"]
pub type FEATURE_R = crate::FieldReader<u16, FEATURE_A>;
#[doc = "Feature Specification Number\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u16)]
pub enum FEATURE_A {
    #[doc = "0: Standard features implemented"]
    STANDARD = 0,
    #[doc = "32768: Core Control and Status Registers are implemented in both MUA and MUB."]
    CTRL_STAT_AB = 32768,
}
impl From<FEATURE_A> for u16 {
    #[inline(always)]
    fn from(variant: FEATURE_A) -> Self {
        variant as _
    }
}
impl FEATURE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<FEATURE_A> {
        match self.bits {
            0 => Some(FEATURE_A::STANDARD),
            32768 => Some(FEATURE_A::CTRL_STAT_AB),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `STANDARD`"]
    #[inline(always)]
    pub fn is_standard(&self) -> bool {
        *self == FEATURE_A::STANDARD
    }
    #[doc = "Checks if the value of the field is `CTRL_STAT_AB`"]
    #[inline(always)]
    pub fn is_ctrl_stat_ab(&self) -> bool {
        *self == FEATURE_A::CTRL_STAT_AB
    }
}
#[doc = "Field `MINOR` reader - Minor Version Number"]
pub type MINOR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `MAJOR` reader - Major Version Number"]
pub type MAJOR_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 0:15 - Feature Specification Number"]
    #[inline(always)]
    pub fn feature(&self) -> FEATURE_R {
        FEATURE_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:23 - Minor Version Number"]
    #[inline(always)]
    pub fn minor(&self) -> MINOR_R {
        MINOR_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Major Version Number"]
    #[inline(always)]
    pub fn major(&self) -> MAJOR_R {
        MAJOR_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
#[doc = "Version ID Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ver](index.html) module"]
pub struct VER_SPEC;
impl crate::RegisterSpec for VER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ver::R](R) reader structure"]
impl crate::Readable for VER_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets VER to value 0x0100_0001"]
impl crate::Resettable for VER_SPEC {
    const RESET_VALUE: Self::Ux = 0x0100_0001;
}
