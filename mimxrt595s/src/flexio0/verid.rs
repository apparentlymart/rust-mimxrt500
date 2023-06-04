#[doc = "Register `VERID` reader"]
pub struct R(crate::R<VERID_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<VERID_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<VERID_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<VERID_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `FEATURE` reader - Feature Specification Number"]
pub type FEATURE_R = crate::FieldReader<u16, FEATURE_A>;
#[doc = "Feature Specification Number\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u16)]
pub enum FEATURE_A {
    #[doc = "0: Standard features implemented."]
    STANDARD = 0,
    #[doc = "1: Supports state, logic and parallel modes."]
    STATE_LOGIC_PARALLEL = 1,
    #[doc = "2: Supports pin control registers."]
    PINCTRL = 2,
    #[doc = "3: Supports state, logic and parallel modes; plus pin control registers."]
    STATE_LOGIC_PARALLEL_PINCTRL = 3,
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
            1 => Some(FEATURE_A::STATE_LOGIC_PARALLEL),
            2 => Some(FEATURE_A::PINCTRL),
            3 => Some(FEATURE_A::STATE_LOGIC_PARALLEL_PINCTRL),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `STANDARD`"]
    #[inline(always)]
    pub fn is_standard(&self) -> bool {
        *self == FEATURE_A::STANDARD
    }
    #[doc = "Checks if the value of the field is `STATE_LOGIC_PARALLEL`"]
    #[inline(always)]
    pub fn is_state_logic_parallel(&self) -> bool {
        *self == FEATURE_A::STATE_LOGIC_PARALLEL
    }
    #[doc = "Checks if the value of the field is `PINCTRL`"]
    #[inline(always)]
    pub fn is_pinctrl(&self) -> bool {
        *self == FEATURE_A::PINCTRL
    }
    #[doc = "Checks if the value of the field is `STATE_LOGIC_PARALLEL_PINCTRL`"]
    #[inline(always)]
    pub fn is_state_logic_parallel_pinctrl(&self) -> bool {
        *self == FEATURE_A::STATE_LOGIC_PARALLEL_PINCTRL
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
#[doc = "Version ID Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [verid](index.html) module"]
pub struct VERID_SPEC;
impl crate::RegisterSpec for VERID_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [verid::R](R) reader structure"]
impl crate::Readable for VERID_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets VERID to value 0x0201_0003"]
impl crate::Resettable for VERID_SPEC {
    const RESET_VALUE: Self::Ux = 0x0201_0003;
}
