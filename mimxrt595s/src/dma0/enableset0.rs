#[doc = "Register `ENABLESET0` reader"]
pub struct R(crate::R<ENABLESET0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ENABLESET0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ENABLESET0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ENABLESET0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ENABLESET0` writer"]
pub struct W(crate::W<ENABLESET0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ENABLESET0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<ENABLESET0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ENABLESET0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ENABLE0` reader - Enable for DMA channel"]
pub type ENABLE0_R = crate::BitReader<ENABLE0_A>;
#[doc = "Enable for DMA channel\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ENABLE0_A {
    #[doc = "0: DMA channel is disabled."]
    DISABLED = 0,
    #[doc = "1: DMA channel is enabled."]
    ENABLED = 1,
}
impl From<ENABLE0_A> for bool {
    #[inline(always)]
    fn from(variant: ENABLE0_A) -> Self {
        variant as u8 != 0
    }
}
impl ENABLE0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ENABLE0_A {
        match self.bits {
            false => ENABLE0_A::DISABLED,
            true => ENABLE0_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ENABLE0_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ENABLE0_A::ENABLED
    }
}
#[doc = "Field `ENABLE0` writer - Enable for DMA channel"]
pub type ENABLE0_W<'a, const O: u8> = crate::BitWriter<'a, u32, ENABLESET0_SPEC, ENABLE0_A, O>;
impl<'a, const O: u8> ENABLE0_W<'a, O> {
    #[doc = "DMA channel is disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(ENABLE0_A::DISABLED)
    }
    #[doc = "DMA channel is enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(ENABLE0_A::ENABLED)
    }
}
#[doc = "Field `ENABLE1` reader - Enable for DMA channel"]
pub type ENABLE1_R = crate::BitReader<ENABLE1_A>;
#[doc = "Enable for DMA channel\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ENABLE1_A {
    #[doc = "0: DMA channel is disabled."]
    DISABLED = 0,
    #[doc = "1: DMA channel is enabled."]
    ENABLED = 1,
}
impl From<ENABLE1_A> for bool {
    #[inline(always)]
    fn from(variant: ENABLE1_A) -> Self {
        variant as u8 != 0
    }
}
impl ENABLE1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ENABLE1_A {
        match self.bits {
            false => ENABLE1_A::DISABLED,
            true => ENABLE1_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ENABLE1_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ENABLE1_A::ENABLED
    }
}
#[doc = "Field `ENABLE1` writer - Enable for DMA channel"]
pub type ENABLE1_W<'a, const O: u8> = crate::BitWriter<'a, u32, ENABLESET0_SPEC, ENABLE1_A, O>;
impl<'a, const O: u8> ENABLE1_W<'a, O> {
    #[doc = "DMA channel is disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(ENABLE1_A::DISABLED)
    }
    #[doc = "DMA channel is enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(ENABLE1_A::ENABLED)
    }
}
#[doc = "Field `ENABLE2` reader - Enable for DMA channel"]
pub type ENABLE2_R = crate::BitReader<ENABLE2_A>;
#[doc = "Enable for DMA channel\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ENABLE2_A {
    #[doc = "0: DMA channel is disabled."]
    DISABLED = 0,
    #[doc = "1: DMA channel is enabled."]
    ENABLED = 1,
}
impl From<ENABLE2_A> for bool {
    #[inline(always)]
    fn from(variant: ENABLE2_A) -> Self {
        variant as u8 != 0
    }
}
impl ENABLE2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ENABLE2_A {
        match self.bits {
            false => ENABLE2_A::DISABLED,
            true => ENABLE2_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ENABLE2_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ENABLE2_A::ENABLED
    }
}
#[doc = "Field `ENABLE2` writer - Enable for DMA channel"]
pub type ENABLE2_W<'a, const O: u8> = crate::BitWriter<'a, u32, ENABLESET0_SPEC, ENABLE2_A, O>;
impl<'a, const O: u8> ENABLE2_W<'a, O> {
    #[doc = "DMA channel is disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(ENABLE2_A::DISABLED)
    }
    #[doc = "DMA channel is enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(ENABLE2_A::ENABLED)
    }
}
#[doc = "Field `ENABLE3` reader - Enable for DMA channel"]
pub type ENABLE3_R = crate::BitReader<ENABLE3_A>;
#[doc = "Enable for DMA channel\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ENABLE3_A {
    #[doc = "0: DMA channel is disabled."]
    DISABLED = 0,
    #[doc = "1: DMA channel is enabled."]
    ENABLED = 1,
}
impl From<ENABLE3_A> for bool {
    #[inline(always)]
    fn from(variant: ENABLE3_A) -> Self {
        variant as u8 != 0
    }
}
impl ENABLE3_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ENABLE3_A {
        match self.bits {
            false => ENABLE3_A::DISABLED,
            true => ENABLE3_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ENABLE3_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ENABLE3_A::ENABLED
    }
}
#[doc = "Field `ENABLE3` writer - Enable for DMA channel"]
pub type ENABLE3_W<'a, const O: u8> = crate::BitWriter<'a, u32, ENABLESET0_SPEC, ENABLE3_A, O>;
impl<'a, const O: u8> ENABLE3_W<'a, O> {
    #[doc = "DMA channel is disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(ENABLE3_A::DISABLED)
    }
    #[doc = "DMA channel is enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(ENABLE3_A::ENABLED)
    }
}
#[doc = "Field `ENABLE4` reader - Enable for DMA channel"]
pub type ENABLE4_R = crate::BitReader<ENABLE4_A>;
#[doc = "Enable for DMA channel\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ENABLE4_A {
    #[doc = "0: DMA channel is disabled."]
    DISABLED = 0,
    #[doc = "1: DMA channel is enabled."]
    ENABLED = 1,
}
impl From<ENABLE4_A> for bool {
    #[inline(always)]
    fn from(variant: ENABLE4_A) -> Self {
        variant as u8 != 0
    }
}
impl ENABLE4_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ENABLE4_A {
        match self.bits {
            false => ENABLE4_A::DISABLED,
            true => ENABLE4_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ENABLE4_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ENABLE4_A::ENABLED
    }
}
#[doc = "Field `ENABLE4` writer - Enable for DMA channel"]
pub type ENABLE4_W<'a, const O: u8> = crate::BitWriter<'a, u32, ENABLESET0_SPEC, ENABLE4_A, O>;
impl<'a, const O: u8> ENABLE4_W<'a, O> {
    #[doc = "DMA channel is disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(ENABLE4_A::DISABLED)
    }
    #[doc = "DMA channel is enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(ENABLE4_A::ENABLED)
    }
}
#[doc = "Field `ENABLE5` reader - Enable for DMA channel"]
pub type ENABLE5_R = crate::BitReader<ENABLE5_A>;
#[doc = "Enable for DMA channel\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ENABLE5_A {
    #[doc = "0: DMA channel is disabled."]
    DISABLED = 0,
    #[doc = "1: DMA channel is enabled."]
    ENABLED = 1,
}
impl From<ENABLE5_A> for bool {
    #[inline(always)]
    fn from(variant: ENABLE5_A) -> Self {
        variant as u8 != 0
    }
}
impl ENABLE5_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ENABLE5_A {
        match self.bits {
            false => ENABLE5_A::DISABLED,
            true => ENABLE5_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ENABLE5_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ENABLE5_A::ENABLED
    }
}
#[doc = "Field `ENABLE5` writer - Enable for DMA channel"]
pub type ENABLE5_W<'a, const O: u8> = crate::BitWriter<'a, u32, ENABLESET0_SPEC, ENABLE5_A, O>;
impl<'a, const O: u8> ENABLE5_W<'a, O> {
    #[doc = "DMA channel is disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(ENABLE5_A::DISABLED)
    }
    #[doc = "DMA channel is enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(ENABLE5_A::ENABLED)
    }
}
#[doc = "Field `ENABLE6` reader - Enable for DMA channel"]
pub type ENABLE6_R = crate::BitReader<ENABLE6_A>;
#[doc = "Enable for DMA channel\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ENABLE6_A {
    #[doc = "0: DMA channel is disabled."]
    DISABLED = 0,
    #[doc = "1: DMA channel is enabled."]
    ENABLED = 1,
}
impl From<ENABLE6_A> for bool {
    #[inline(always)]
    fn from(variant: ENABLE6_A) -> Self {
        variant as u8 != 0
    }
}
impl ENABLE6_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ENABLE6_A {
        match self.bits {
            false => ENABLE6_A::DISABLED,
            true => ENABLE6_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ENABLE6_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ENABLE6_A::ENABLED
    }
}
#[doc = "Field `ENABLE6` writer - Enable for DMA channel"]
pub type ENABLE6_W<'a, const O: u8> = crate::BitWriter<'a, u32, ENABLESET0_SPEC, ENABLE6_A, O>;
impl<'a, const O: u8> ENABLE6_W<'a, O> {
    #[doc = "DMA channel is disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(ENABLE6_A::DISABLED)
    }
    #[doc = "DMA channel is enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(ENABLE6_A::ENABLED)
    }
}
#[doc = "Field `ENABLE7` reader - Enable for DMA channel"]
pub type ENABLE7_R = crate::BitReader<ENABLE7_A>;
#[doc = "Enable for DMA channel\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ENABLE7_A {
    #[doc = "0: DMA channel is disabled."]
    DISABLED = 0,
    #[doc = "1: DMA channel is enabled."]
    ENABLED = 1,
}
impl From<ENABLE7_A> for bool {
    #[inline(always)]
    fn from(variant: ENABLE7_A) -> Self {
        variant as u8 != 0
    }
}
impl ENABLE7_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ENABLE7_A {
        match self.bits {
            false => ENABLE7_A::DISABLED,
            true => ENABLE7_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ENABLE7_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ENABLE7_A::ENABLED
    }
}
#[doc = "Field `ENABLE7` writer - Enable for DMA channel"]
pub type ENABLE7_W<'a, const O: u8> = crate::BitWriter<'a, u32, ENABLESET0_SPEC, ENABLE7_A, O>;
impl<'a, const O: u8> ENABLE7_W<'a, O> {
    #[doc = "DMA channel is disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(ENABLE7_A::DISABLED)
    }
    #[doc = "DMA channel is enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(ENABLE7_A::ENABLED)
    }
}
#[doc = "Field `ENABLE8` reader - Enable for DMA channel"]
pub type ENABLE8_R = crate::BitReader<ENABLE8_A>;
#[doc = "Enable for DMA channel\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ENABLE8_A {
    #[doc = "0: DMA channel is disabled."]
    DISABLED = 0,
    #[doc = "1: DMA channel is enabled."]
    ENABLED = 1,
}
impl From<ENABLE8_A> for bool {
    #[inline(always)]
    fn from(variant: ENABLE8_A) -> Self {
        variant as u8 != 0
    }
}
impl ENABLE8_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ENABLE8_A {
        match self.bits {
            false => ENABLE8_A::DISABLED,
            true => ENABLE8_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ENABLE8_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ENABLE8_A::ENABLED
    }
}
#[doc = "Field `ENABLE8` writer - Enable for DMA channel"]
pub type ENABLE8_W<'a, const O: u8> = crate::BitWriter<'a, u32, ENABLESET0_SPEC, ENABLE8_A, O>;
impl<'a, const O: u8> ENABLE8_W<'a, O> {
    #[doc = "DMA channel is disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(ENABLE8_A::DISABLED)
    }
    #[doc = "DMA channel is enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(ENABLE8_A::ENABLED)
    }
}
#[doc = "Field `ENABLE9` reader - Enable for DMA channel"]
pub type ENABLE9_R = crate::BitReader<ENABLE9_A>;
#[doc = "Enable for DMA channel\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ENABLE9_A {
    #[doc = "0: DMA channel is disabled."]
    DISABLED = 0,
    #[doc = "1: DMA channel is enabled."]
    ENABLED = 1,
}
impl From<ENABLE9_A> for bool {
    #[inline(always)]
    fn from(variant: ENABLE9_A) -> Self {
        variant as u8 != 0
    }
}
impl ENABLE9_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ENABLE9_A {
        match self.bits {
            false => ENABLE9_A::DISABLED,
            true => ENABLE9_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ENABLE9_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ENABLE9_A::ENABLED
    }
}
#[doc = "Field `ENABLE9` writer - Enable for DMA channel"]
pub type ENABLE9_W<'a, const O: u8> = crate::BitWriter<'a, u32, ENABLESET0_SPEC, ENABLE9_A, O>;
impl<'a, const O: u8> ENABLE9_W<'a, O> {
    #[doc = "DMA channel is disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(ENABLE9_A::DISABLED)
    }
    #[doc = "DMA channel is enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(ENABLE9_A::ENABLED)
    }
}
#[doc = "Field `ENABLE10` reader - Enable for DMA channel"]
pub type ENABLE10_R = crate::BitReader<ENABLE10_A>;
#[doc = "Enable for DMA channel\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ENABLE10_A {
    #[doc = "0: DMA channel is disabled."]
    DISABLED = 0,
    #[doc = "1: DMA channel is enabled."]
    ENABLED = 1,
}
impl From<ENABLE10_A> for bool {
    #[inline(always)]
    fn from(variant: ENABLE10_A) -> Self {
        variant as u8 != 0
    }
}
impl ENABLE10_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ENABLE10_A {
        match self.bits {
            false => ENABLE10_A::DISABLED,
            true => ENABLE10_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ENABLE10_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ENABLE10_A::ENABLED
    }
}
#[doc = "Field `ENABLE10` writer - Enable for DMA channel"]
pub type ENABLE10_W<'a, const O: u8> = crate::BitWriter<'a, u32, ENABLESET0_SPEC, ENABLE10_A, O>;
impl<'a, const O: u8> ENABLE10_W<'a, O> {
    #[doc = "DMA channel is disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(ENABLE10_A::DISABLED)
    }
    #[doc = "DMA channel is enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(ENABLE10_A::ENABLED)
    }
}
#[doc = "Field `ENABLE11` reader - Enable for DMA channel"]
pub type ENABLE11_R = crate::BitReader<ENABLE11_A>;
#[doc = "Enable for DMA channel\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ENABLE11_A {
    #[doc = "0: DMA channel is disabled."]
    DISABLED = 0,
    #[doc = "1: DMA channel is enabled."]
    ENABLED = 1,
}
impl From<ENABLE11_A> for bool {
    #[inline(always)]
    fn from(variant: ENABLE11_A) -> Self {
        variant as u8 != 0
    }
}
impl ENABLE11_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ENABLE11_A {
        match self.bits {
            false => ENABLE11_A::DISABLED,
            true => ENABLE11_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ENABLE11_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ENABLE11_A::ENABLED
    }
}
#[doc = "Field `ENABLE11` writer - Enable for DMA channel"]
pub type ENABLE11_W<'a, const O: u8> = crate::BitWriter<'a, u32, ENABLESET0_SPEC, ENABLE11_A, O>;
impl<'a, const O: u8> ENABLE11_W<'a, O> {
    #[doc = "DMA channel is disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(ENABLE11_A::DISABLED)
    }
    #[doc = "DMA channel is enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(ENABLE11_A::ENABLED)
    }
}
#[doc = "Field `ENABLE12` reader - Enable for DMA channel"]
pub type ENABLE12_R = crate::BitReader<ENABLE12_A>;
#[doc = "Enable for DMA channel\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ENABLE12_A {
    #[doc = "0: DMA channel is disabled."]
    DISABLED = 0,
    #[doc = "1: DMA channel is enabled."]
    ENABLED = 1,
}
impl From<ENABLE12_A> for bool {
    #[inline(always)]
    fn from(variant: ENABLE12_A) -> Self {
        variant as u8 != 0
    }
}
impl ENABLE12_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ENABLE12_A {
        match self.bits {
            false => ENABLE12_A::DISABLED,
            true => ENABLE12_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ENABLE12_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ENABLE12_A::ENABLED
    }
}
#[doc = "Field `ENABLE12` writer - Enable for DMA channel"]
pub type ENABLE12_W<'a, const O: u8> = crate::BitWriter<'a, u32, ENABLESET0_SPEC, ENABLE12_A, O>;
impl<'a, const O: u8> ENABLE12_W<'a, O> {
    #[doc = "DMA channel is disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(ENABLE12_A::DISABLED)
    }
    #[doc = "DMA channel is enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(ENABLE12_A::ENABLED)
    }
}
#[doc = "Field `ENABLE13` reader - Enable for DMA channel"]
pub type ENABLE13_R = crate::BitReader<ENABLE13_A>;
#[doc = "Enable for DMA channel\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ENABLE13_A {
    #[doc = "0: DMA channel is disabled."]
    DISABLED = 0,
    #[doc = "1: DMA channel is enabled."]
    ENABLED = 1,
}
impl From<ENABLE13_A> for bool {
    #[inline(always)]
    fn from(variant: ENABLE13_A) -> Self {
        variant as u8 != 0
    }
}
impl ENABLE13_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ENABLE13_A {
        match self.bits {
            false => ENABLE13_A::DISABLED,
            true => ENABLE13_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ENABLE13_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ENABLE13_A::ENABLED
    }
}
#[doc = "Field `ENABLE13` writer - Enable for DMA channel"]
pub type ENABLE13_W<'a, const O: u8> = crate::BitWriter<'a, u32, ENABLESET0_SPEC, ENABLE13_A, O>;
impl<'a, const O: u8> ENABLE13_W<'a, O> {
    #[doc = "DMA channel is disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(ENABLE13_A::DISABLED)
    }
    #[doc = "DMA channel is enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(ENABLE13_A::ENABLED)
    }
}
#[doc = "Field `ENABLE14` reader - Enable for DMA channel"]
pub type ENABLE14_R = crate::BitReader<ENABLE14_A>;
#[doc = "Enable for DMA channel\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ENABLE14_A {
    #[doc = "0: DMA channel is disabled."]
    DISABLED = 0,
    #[doc = "1: DMA channel is enabled."]
    ENABLED = 1,
}
impl From<ENABLE14_A> for bool {
    #[inline(always)]
    fn from(variant: ENABLE14_A) -> Self {
        variant as u8 != 0
    }
}
impl ENABLE14_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ENABLE14_A {
        match self.bits {
            false => ENABLE14_A::DISABLED,
            true => ENABLE14_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ENABLE14_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ENABLE14_A::ENABLED
    }
}
#[doc = "Field `ENABLE14` writer - Enable for DMA channel"]
pub type ENABLE14_W<'a, const O: u8> = crate::BitWriter<'a, u32, ENABLESET0_SPEC, ENABLE14_A, O>;
impl<'a, const O: u8> ENABLE14_W<'a, O> {
    #[doc = "DMA channel is disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(ENABLE14_A::DISABLED)
    }
    #[doc = "DMA channel is enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(ENABLE14_A::ENABLED)
    }
}
#[doc = "Field `ENABLE15` reader - Enable for DMA channel"]
pub type ENABLE15_R = crate::BitReader<ENABLE15_A>;
#[doc = "Enable for DMA channel\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ENABLE15_A {
    #[doc = "0: DMA channel is disabled."]
    DISABLED = 0,
    #[doc = "1: DMA channel is enabled."]
    ENABLED = 1,
}
impl From<ENABLE15_A> for bool {
    #[inline(always)]
    fn from(variant: ENABLE15_A) -> Self {
        variant as u8 != 0
    }
}
impl ENABLE15_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ENABLE15_A {
        match self.bits {
            false => ENABLE15_A::DISABLED,
            true => ENABLE15_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ENABLE15_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ENABLE15_A::ENABLED
    }
}
#[doc = "Field `ENABLE15` writer - Enable for DMA channel"]
pub type ENABLE15_W<'a, const O: u8> = crate::BitWriter<'a, u32, ENABLESET0_SPEC, ENABLE15_A, O>;
impl<'a, const O: u8> ENABLE15_W<'a, O> {
    #[doc = "DMA channel is disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(ENABLE15_A::DISABLED)
    }
    #[doc = "DMA channel is enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(ENABLE15_A::ENABLED)
    }
}
#[doc = "Field `ENABLE16` reader - Enable for DMA channel"]
pub type ENABLE16_R = crate::BitReader<ENABLE16_A>;
#[doc = "Enable for DMA channel\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ENABLE16_A {
    #[doc = "0: DMA channel is disabled."]
    DISABLED = 0,
    #[doc = "1: DMA channel is enabled."]
    ENABLED = 1,
}
impl From<ENABLE16_A> for bool {
    #[inline(always)]
    fn from(variant: ENABLE16_A) -> Self {
        variant as u8 != 0
    }
}
impl ENABLE16_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ENABLE16_A {
        match self.bits {
            false => ENABLE16_A::DISABLED,
            true => ENABLE16_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ENABLE16_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ENABLE16_A::ENABLED
    }
}
#[doc = "Field `ENABLE16` writer - Enable for DMA channel"]
pub type ENABLE16_W<'a, const O: u8> = crate::BitWriter<'a, u32, ENABLESET0_SPEC, ENABLE16_A, O>;
impl<'a, const O: u8> ENABLE16_W<'a, O> {
    #[doc = "DMA channel is disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(ENABLE16_A::DISABLED)
    }
    #[doc = "DMA channel is enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(ENABLE16_A::ENABLED)
    }
}
#[doc = "Field `ENABLE17` reader - Enable for DMA channel"]
pub type ENABLE17_R = crate::BitReader<ENABLE17_A>;
#[doc = "Enable for DMA channel\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ENABLE17_A {
    #[doc = "0: DMA channel is disabled."]
    DISABLED = 0,
    #[doc = "1: DMA channel is enabled."]
    ENABLED = 1,
}
impl From<ENABLE17_A> for bool {
    #[inline(always)]
    fn from(variant: ENABLE17_A) -> Self {
        variant as u8 != 0
    }
}
impl ENABLE17_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ENABLE17_A {
        match self.bits {
            false => ENABLE17_A::DISABLED,
            true => ENABLE17_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ENABLE17_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ENABLE17_A::ENABLED
    }
}
#[doc = "Field `ENABLE17` writer - Enable for DMA channel"]
pub type ENABLE17_W<'a, const O: u8> = crate::BitWriter<'a, u32, ENABLESET0_SPEC, ENABLE17_A, O>;
impl<'a, const O: u8> ENABLE17_W<'a, O> {
    #[doc = "DMA channel is disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(ENABLE17_A::DISABLED)
    }
    #[doc = "DMA channel is enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(ENABLE17_A::ENABLED)
    }
}
#[doc = "Field `ENABLE18` reader - Enable for DMA channel"]
pub type ENABLE18_R = crate::BitReader<ENABLE18_A>;
#[doc = "Enable for DMA channel\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ENABLE18_A {
    #[doc = "0: DMA channel is disabled."]
    DISABLED = 0,
    #[doc = "1: DMA channel is enabled."]
    ENABLED = 1,
}
impl From<ENABLE18_A> for bool {
    #[inline(always)]
    fn from(variant: ENABLE18_A) -> Self {
        variant as u8 != 0
    }
}
impl ENABLE18_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ENABLE18_A {
        match self.bits {
            false => ENABLE18_A::DISABLED,
            true => ENABLE18_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ENABLE18_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ENABLE18_A::ENABLED
    }
}
#[doc = "Field `ENABLE18` writer - Enable for DMA channel"]
pub type ENABLE18_W<'a, const O: u8> = crate::BitWriter<'a, u32, ENABLESET0_SPEC, ENABLE18_A, O>;
impl<'a, const O: u8> ENABLE18_W<'a, O> {
    #[doc = "DMA channel is disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(ENABLE18_A::DISABLED)
    }
    #[doc = "DMA channel is enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(ENABLE18_A::ENABLED)
    }
}
#[doc = "Field `ENABLE19` reader - Enable for DMA channel"]
pub type ENABLE19_R = crate::BitReader<ENABLE19_A>;
#[doc = "Enable for DMA channel\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ENABLE19_A {
    #[doc = "0: DMA channel is disabled."]
    DISABLED = 0,
    #[doc = "1: DMA channel is enabled."]
    ENABLED = 1,
}
impl From<ENABLE19_A> for bool {
    #[inline(always)]
    fn from(variant: ENABLE19_A) -> Self {
        variant as u8 != 0
    }
}
impl ENABLE19_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ENABLE19_A {
        match self.bits {
            false => ENABLE19_A::DISABLED,
            true => ENABLE19_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ENABLE19_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ENABLE19_A::ENABLED
    }
}
#[doc = "Field `ENABLE19` writer - Enable for DMA channel"]
pub type ENABLE19_W<'a, const O: u8> = crate::BitWriter<'a, u32, ENABLESET0_SPEC, ENABLE19_A, O>;
impl<'a, const O: u8> ENABLE19_W<'a, O> {
    #[doc = "DMA channel is disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(ENABLE19_A::DISABLED)
    }
    #[doc = "DMA channel is enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(ENABLE19_A::ENABLED)
    }
}
#[doc = "Field `ENABLE20` reader - Enable for DMA channel"]
pub type ENABLE20_R = crate::BitReader<ENABLE20_A>;
#[doc = "Enable for DMA channel\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ENABLE20_A {
    #[doc = "0: DMA channel is disabled."]
    DISABLED = 0,
    #[doc = "1: DMA channel is enabled."]
    ENABLED = 1,
}
impl From<ENABLE20_A> for bool {
    #[inline(always)]
    fn from(variant: ENABLE20_A) -> Self {
        variant as u8 != 0
    }
}
impl ENABLE20_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ENABLE20_A {
        match self.bits {
            false => ENABLE20_A::DISABLED,
            true => ENABLE20_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ENABLE20_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ENABLE20_A::ENABLED
    }
}
#[doc = "Field `ENABLE20` writer - Enable for DMA channel"]
pub type ENABLE20_W<'a, const O: u8> = crate::BitWriter<'a, u32, ENABLESET0_SPEC, ENABLE20_A, O>;
impl<'a, const O: u8> ENABLE20_W<'a, O> {
    #[doc = "DMA channel is disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(ENABLE20_A::DISABLED)
    }
    #[doc = "DMA channel is enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(ENABLE20_A::ENABLED)
    }
}
#[doc = "Field `ENABLE21` reader - Enable for DMA channel"]
pub type ENABLE21_R = crate::BitReader<ENABLE21_A>;
#[doc = "Enable for DMA channel\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ENABLE21_A {
    #[doc = "0: DMA channel is disabled."]
    DISABLED = 0,
    #[doc = "1: DMA channel is enabled."]
    ENABLED = 1,
}
impl From<ENABLE21_A> for bool {
    #[inline(always)]
    fn from(variant: ENABLE21_A) -> Self {
        variant as u8 != 0
    }
}
impl ENABLE21_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ENABLE21_A {
        match self.bits {
            false => ENABLE21_A::DISABLED,
            true => ENABLE21_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ENABLE21_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ENABLE21_A::ENABLED
    }
}
#[doc = "Field `ENABLE21` writer - Enable for DMA channel"]
pub type ENABLE21_W<'a, const O: u8> = crate::BitWriter<'a, u32, ENABLESET0_SPEC, ENABLE21_A, O>;
impl<'a, const O: u8> ENABLE21_W<'a, O> {
    #[doc = "DMA channel is disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(ENABLE21_A::DISABLED)
    }
    #[doc = "DMA channel is enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(ENABLE21_A::ENABLED)
    }
}
#[doc = "Field `ENABLE22` reader - Enable for DMA channel"]
pub type ENABLE22_R = crate::BitReader<ENABLE22_A>;
#[doc = "Enable for DMA channel\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ENABLE22_A {
    #[doc = "0: DMA channel is disabled."]
    DISABLED = 0,
    #[doc = "1: DMA channel is enabled."]
    ENABLED = 1,
}
impl From<ENABLE22_A> for bool {
    #[inline(always)]
    fn from(variant: ENABLE22_A) -> Self {
        variant as u8 != 0
    }
}
impl ENABLE22_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ENABLE22_A {
        match self.bits {
            false => ENABLE22_A::DISABLED,
            true => ENABLE22_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ENABLE22_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ENABLE22_A::ENABLED
    }
}
#[doc = "Field `ENABLE22` writer - Enable for DMA channel"]
pub type ENABLE22_W<'a, const O: u8> = crate::BitWriter<'a, u32, ENABLESET0_SPEC, ENABLE22_A, O>;
impl<'a, const O: u8> ENABLE22_W<'a, O> {
    #[doc = "DMA channel is disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(ENABLE22_A::DISABLED)
    }
    #[doc = "DMA channel is enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(ENABLE22_A::ENABLED)
    }
}
#[doc = "Field `ENABLE23` reader - Enable for DMA channel"]
pub type ENABLE23_R = crate::BitReader<ENABLE23_A>;
#[doc = "Enable for DMA channel\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ENABLE23_A {
    #[doc = "0: DMA channel is disabled."]
    DISABLED = 0,
    #[doc = "1: DMA channel is enabled."]
    ENABLED = 1,
}
impl From<ENABLE23_A> for bool {
    #[inline(always)]
    fn from(variant: ENABLE23_A) -> Self {
        variant as u8 != 0
    }
}
impl ENABLE23_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ENABLE23_A {
        match self.bits {
            false => ENABLE23_A::DISABLED,
            true => ENABLE23_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ENABLE23_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ENABLE23_A::ENABLED
    }
}
#[doc = "Field `ENABLE23` writer - Enable for DMA channel"]
pub type ENABLE23_W<'a, const O: u8> = crate::BitWriter<'a, u32, ENABLESET0_SPEC, ENABLE23_A, O>;
impl<'a, const O: u8> ENABLE23_W<'a, O> {
    #[doc = "DMA channel is disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(ENABLE23_A::DISABLED)
    }
    #[doc = "DMA channel is enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(ENABLE23_A::ENABLED)
    }
}
#[doc = "Field `ENABLE24` reader - Enable for DMA channel"]
pub type ENABLE24_R = crate::BitReader<ENABLE24_A>;
#[doc = "Enable for DMA channel\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ENABLE24_A {
    #[doc = "0: DMA channel is disabled."]
    DISABLED = 0,
    #[doc = "1: DMA channel is enabled."]
    ENABLED = 1,
}
impl From<ENABLE24_A> for bool {
    #[inline(always)]
    fn from(variant: ENABLE24_A) -> Self {
        variant as u8 != 0
    }
}
impl ENABLE24_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ENABLE24_A {
        match self.bits {
            false => ENABLE24_A::DISABLED,
            true => ENABLE24_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ENABLE24_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ENABLE24_A::ENABLED
    }
}
#[doc = "Field `ENABLE24` writer - Enable for DMA channel"]
pub type ENABLE24_W<'a, const O: u8> = crate::BitWriter<'a, u32, ENABLESET0_SPEC, ENABLE24_A, O>;
impl<'a, const O: u8> ENABLE24_W<'a, O> {
    #[doc = "DMA channel is disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(ENABLE24_A::DISABLED)
    }
    #[doc = "DMA channel is enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(ENABLE24_A::ENABLED)
    }
}
#[doc = "Field `ENABLE25` reader - Enable for DMA channel"]
pub type ENABLE25_R = crate::BitReader<ENABLE25_A>;
#[doc = "Enable for DMA channel\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ENABLE25_A {
    #[doc = "0: DMA channel is disabled."]
    DISABLED = 0,
    #[doc = "1: DMA channel is enabled."]
    ENABLED = 1,
}
impl From<ENABLE25_A> for bool {
    #[inline(always)]
    fn from(variant: ENABLE25_A) -> Self {
        variant as u8 != 0
    }
}
impl ENABLE25_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ENABLE25_A {
        match self.bits {
            false => ENABLE25_A::DISABLED,
            true => ENABLE25_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ENABLE25_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ENABLE25_A::ENABLED
    }
}
#[doc = "Field `ENABLE25` writer - Enable for DMA channel"]
pub type ENABLE25_W<'a, const O: u8> = crate::BitWriter<'a, u32, ENABLESET0_SPEC, ENABLE25_A, O>;
impl<'a, const O: u8> ENABLE25_W<'a, O> {
    #[doc = "DMA channel is disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(ENABLE25_A::DISABLED)
    }
    #[doc = "DMA channel is enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(ENABLE25_A::ENABLED)
    }
}
#[doc = "Field `ENABLE26` reader - Enable for DMA channel"]
pub type ENABLE26_R = crate::BitReader<ENABLE26_A>;
#[doc = "Enable for DMA channel\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ENABLE26_A {
    #[doc = "0: DMA channel is disabled."]
    DISABLED = 0,
    #[doc = "1: DMA channel is enabled."]
    ENABLED = 1,
}
impl From<ENABLE26_A> for bool {
    #[inline(always)]
    fn from(variant: ENABLE26_A) -> Self {
        variant as u8 != 0
    }
}
impl ENABLE26_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ENABLE26_A {
        match self.bits {
            false => ENABLE26_A::DISABLED,
            true => ENABLE26_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ENABLE26_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ENABLE26_A::ENABLED
    }
}
#[doc = "Field `ENABLE26` writer - Enable for DMA channel"]
pub type ENABLE26_W<'a, const O: u8> = crate::BitWriter<'a, u32, ENABLESET0_SPEC, ENABLE26_A, O>;
impl<'a, const O: u8> ENABLE26_W<'a, O> {
    #[doc = "DMA channel is disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(ENABLE26_A::DISABLED)
    }
    #[doc = "DMA channel is enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(ENABLE26_A::ENABLED)
    }
}
#[doc = "Field `ENABLE27` reader - Enable for DMA channel"]
pub type ENABLE27_R = crate::BitReader<ENABLE27_A>;
#[doc = "Enable for DMA channel\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ENABLE27_A {
    #[doc = "0: DMA channel is disabled."]
    DISABLED = 0,
    #[doc = "1: DMA channel is enabled."]
    ENABLED = 1,
}
impl From<ENABLE27_A> for bool {
    #[inline(always)]
    fn from(variant: ENABLE27_A) -> Self {
        variant as u8 != 0
    }
}
impl ENABLE27_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ENABLE27_A {
        match self.bits {
            false => ENABLE27_A::DISABLED,
            true => ENABLE27_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ENABLE27_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ENABLE27_A::ENABLED
    }
}
#[doc = "Field `ENABLE27` writer - Enable for DMA channel"]
pub type ENABLE27_W<'a, const O: u8> = crate::BitWriter<'a, u32, ENABLESET0_SPEC, ENABLE27_A, O>;
impl<'a, const O: u8> ENABLE27_W<'a, O> {
    #[doc = "DMA channel is disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(ENABLE27_A::DISABLED)
    }
    #[doc = "DMA channel is enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(ENABLE27_A::ENABLED)
    }
}
#[doc = "Field `ENABLE28` reader - Enable for DMA channel"]
pub type ENABLE28_R = crate::BitReader<ENABLE28_A>;
#[doc = "Enable for DMA channel\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ENABLE28_A {
    #[doc = "0: DMA channel is disabled."]
    DISABLED = 0,
    #[doc = "1: DMA channel is enabled."]
    ENABLED = 1,
}
impl From<ENABLE28_A> for bool {
    #[inline(always)]
    fn from(variant: ENABLE28_A) -> Self {
        variant as u8 != 0
    }
}
impl ENABLE28_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ENABLE28_A {
        match self.bits {
            false => ENABLE28_A::DISABLED,
            true => ENABLE28_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ENABLE28_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ENABLE28_A::ENABLED
    }
}
#[doc = "Field `ENABLE28` writer - Enable for DMA channel"]
pub type ENABLE28_W<'a, const O: u8> = crate::BitWriter<'a, u32, ENABLESET0_SPEC, ENABLE28_A, O>;
impl<'a, const O: u8> ENABLE28_W<'a, O> {
    #[doc = "DMA channel is disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(ENABLE28_A::DISABLED)
    }
    #[doc = "DMA channel is enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(ENABLE28_A::ENABLED)
    }
}
#[doc = "Field `ENABLE29` reader - Enable for DMA channel"]
pub type ENABLE29_R = crate::BitReader<ENABLE29_A>;
#[doc = "Enable for DMA channel\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ENABLE29_A {
    #[doc = "0: DMA channel is disabled."]
    DISABLED = 0,
    #[doc = "1: DMA channel is enabled."]
    ENABLED = 1,
}
impl From<ENABLE29_A> for bool {
    #[inline(always)]
    fn from(variant: ENABLE29_A) -> Self {
        variant as u8 != 0
    }
}
impl ENABLE29_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ENABLE29_A {
        match self.bits {
            false => ENABLE29_A::DISABLED,
            true => ENABLE29_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ENABLE29_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ENABLE29_A::ENABLED
    }
}
#[doc = "Field `ENABLE29` writer - Enable for DMA channel"]
pub type ENABLE29_W<'a, const O: u8> = crate::BitWriter<'a, u32, ENABLESET0_SPEC, ENABLE29_A, O>;
impl<'a, const O: u8> ENABLE29_W<'a, O> {
    #[doc = "DMA channel is disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(ENABLE29_A::DISABLED)
    }
    #[doc = "DMA channel is enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(ENABLE29_A::ENABLED)
    }
}
#[doc = "Field `ENABLE30` reader - Enable for DMA channel"]
pub type ENABLE30_R = crate::BitReader<ENABLE30_A>;
#[doc = "Enable for DMA channel\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ENABLE30_A {
    #[doc = "0: DMA channel is disabled."]
    DISABLED = 0,
    #[doc = "1: DMA channel is enabled."]
    ENABLED = 1,
}
impl From<ENABLE30_A> for bool {
    #[inline(always)]
    fn from(variant: ENABLE30_A) -> Self {
        variant as u8 != 0
    }
}
impl ENABLE30_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ENABLE30_A {
        match self.bits {
            false => ENABLE30_A::DISABLED,
            true => ENABLE30_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ENABLE30_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ENABLE30_A::ENABLED
    }
}
#[doc = "Field `ENABLE30` writer - Enable for DMA channel"]
pub type ENABLE30_W<'a, const O: u8> = crate::BitWriter<'a, u32, ENABLESET0_SPEC, ENABLE30_A, O>;
impl<'a, const O: u8> ENABLE30_W<'a, O> {
    #[doc = "DMA channel is disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(ENABLE30_A::DISABLED)
    }
    #[doc = "DMA channel is enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(ENABLE30_A::ENABLED)
    }
}
#[doc = "Field `ENABLE31` reader - Enable for DMA channel"]
pub type ENABLE31_R = crate::BitReader<ENABLE31_A>;
#[doc = "Enable for DMA channel\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ENABLE31_A {
    #[doc = "0: DMA channel is disabled."]
    DISABLED = 0,
    #[doc = "1: DMA channel is enabled."]
    ENABLED = 1,
}
impl From<ENABLE31_A> for bool {
    #[inline(always)]
    fn from(variant: ENABLE31_A) -> Self {
        variant as u8 != 0
    }
}
impl ENABLE31_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ENABLE31_A {
        match self.bits {
            false => ENABLE31_A::DISABLED,
            true => ENABLE31_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ENABLE31_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ENABLE31_A::ENABLED
    }
}
#[doc = "Field `ENABLE31` writer - Enable for DMA channel"]
pub type ENABLE31_W<'a, const O: u8> = crate::BitWriter<'a, u32, ENABLESET0_SPEC, ENABLE31_A, O>;
impl<'a, const O: u8> ENABLE31_W<'a, O> {
    #[doc = "DMA channel is disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(ENABLE31_A::DISABLED)
    }
    #[doc = "DMA channel is enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(ENABLE31_A::ENABLED)
    }
}
impl R {
    #[doc = "Bit 0 - Enable for DMA channel"]
    #[inline(always)]
    pub fn enable0(&self) -> ENABLE0_R {
        ENABLE0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enable for DMA channel"]
    #[inline(always)]
    pub fn enable1(&self) -> ENABLE1_R {
        ENABLE1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Enable for DMA channel"]
    #[inline(always)]
    pub fn enable2(&self) -> ENABLE2_R {
        ENABLE2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Enable for DMA channel"]
    #[inline(always)]
    pub fn enable3(&self) -> ENABLE3_R {
        ENABLE3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Enable for DMA channel"]
    #[inline(always)]
    pub fn enable4(&self) -> ENABLE4_R {
        ENABLE4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Enable for DMA channel"]
    #[inline(always)]
    pub fn enable5(&self) -> ENABLE5_R {
        ENABLE5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Enable for DMA channel"]
    #[inline(always)]
    pub fn enable6(&self) -> ENABLE6_R {
        ENABLE6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Enable for DMA channel"]
    #[inline(always)]
    pub fn enable7(&self) -> ENABLE7_R {
        ENABLE7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Enable for DMA channel"]
    #[inline(always)]
    pub fn enable8(&self) -> ENABLE8_R {
        ENABLE8_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Enable for DMA channel"]
    #[inline(always)]
    pub fn enable9(&self) -> ENABLE9_R {
        ENABLE9_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Enable for DMA channel"]
    #[inline(always)]
    pub fn enable10(&self) -> ENABLE10_R {
        ENABLE10_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Enable for DMA channel"]
    #[inline(always)]
    pub fn enable11(&self) -> ENABLE11_R {
        ENABLE11_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Enable for DMA channel"]
    #[inline(always)]
    pub fn enable12(&self) -> ENABLE12_R {
        ENABLE12_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Enable for DMA channel"]
    #[inline(always)]
    pub fn enable13(&self) -> ENABLE13_R {
        ENABLE13_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Enable for DMA channel"]
    #[inline(always)]
    pub fn enable14(&self) -> ENABLE14_R {
        ENABLE14_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Enable for DMA channel"]
    #[inline(always)]
    pub fn enable15(&self) -> ENABLE15_R {
        ENABLE15_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Enable for DMA channel"]
    #[inline(always)]
    pub fn enable16(&self) -> ENABLE16_R {
        ENABLE16_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Enable for DMA channel"]
    #[inline(always)]
    pub fn enable17(&self) -> ENABLE17_R {
        ENABLE17_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Enable for DMA channel"]
    #[inline(always)]
    pub fn enable18(&self) -> ENABLE18_R {
        ENABLE18_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Enable for DMA channel"]
    #[inline(always)]
    pub fn enable19(&self) -> ENABLE19_R {
        ENABLE19_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Enable for DMA channel"]
    #[inline(always)]
    pub fn enable20(&self) -> ENABLE20_R {
        ENABLE20_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Enable for DMA channel"]
    #[inline(always)]
    pub fn enable21(&self) -> ENABLE21_R {
        ENABLE21_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Enable for DMA channel"]
    #[inline(always)]
    pub fn enable22(&self) -> ENABLE22_R {
        ENABLE22_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Enable for DMA channel"]
    #[inline(always)]
    pub fn enable23(&self) -> ENABLE23_R {
        ENABLE23_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Enable for DMA channel"]
    #[inline(always)]
    pub fn enable24(&self) -> ENABLE24_R {
        ENABLE24_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Enable for DMA channel"]
    #[inline(always)]
    pub fn enable25(&self) -> ENABLE25_R {
        ENABLE25_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Enable for DMA channel"]
    #[inline(always)]
    pub fn enable26(&self) -> ENABLE26_R {
        ENABLE26_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Enable for DMA channel"]
    #[inline(always)]
    pub fn enable27(&self) -> ENABLE27_R {
        ENABLE27_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Enable for DMA channel"]
    #[inline(always)]
    pub fn enable28(&self) -> ENABLE28_R {
        ENABLE28_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Enable for DMA channel"]
    #[inline(always)]
    pub fn enable29(&self) -> ENABLE29_R {
        ENABLE29_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Enable for DMA channel"]
    #[inline(always)]
    pub fn enable30(&self) -> ENABLE30_R {
        ENABLE30_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Enable for DMA channel"]
    #[inline(always)]
    pub fn enable31(&self) -> ENABLE31_R {
        ENABLE31_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable for DMA channel"]
    #[inline(always)]
    #[must_use]
    pub fn enable0(&mut self) -> ENABLE0_W<0> {
        ENABLE0_W::new(self)
    }
    #[doc = "Bit 1 - Enable for DMA channel"]
    #[inline(always)]
    #[must_use]
    pub fn enable1(&mut self) -> ENABLE1_W<1> {
        ENABLE1_W::new(self)
    }
    #[doc = "Bit 2 - Enable for DMA channel"]
    #[inline(always)]
    #[must_use]
    pub fn enable2(&mut self) -> ENABLE2_W<2> {
        ENABLE2_W::new(self)
    }
    #[doc = "Bit 3 - Enable for DMA channel"]
    #[inline(always)]
    #[must_use]
    pub fn enable3(&mut self) -> ENABLE3_W<3> {
        ENABLE3_W::new(self)
    }
    #[doc = "Bit 4 - Enable for DMA channel"]
    #[inline(always)]
    #[must_use]
    pub fn enable4(&mut self) -> ENABLE4_W<4> {
        ENABLE4_W::new(self)
    }
    #[doc = "Bit 5 - Enable for DMA channel"]
    #[inline(always)]
    #[must_use]
    pub fn enable5(&mut self) -> ENABLE5_W<5> {
        ENABLE5_W::new(self)
    }
    #[doc = "Bit 6 - Enable for DMA channel"]
    #[inline(always)]
    #[must_use]
    pub fn enable6(&mut self) -> ENABLE6_W<6> {
        ENABLE6_W::new(self)
    }
    #[doc = "Bit 7 - Enable for DMA channel"]
    #[inline(always)]
    #[must_use]
    pub fn enable7(&mut self) -> ENABLE7_W<7> {
        ENABLE7_W::new(self)
    }
    #[doc = "Bit 8 - Enable for DMA channel"]
    #[inline(always)]
    #[must_use]
    pub fn enable8(&mut self) -> ENABLE8_W<8> {
        ENABLE8_W::new(self)
    }
    #[doc = "Bit 9 - Enable for DMA channel"]
    #[inline(always)]
    #[must_use]
    pub fn enable9(&mut self) -> ENABLE9_W<9> {
        ENABLE9_W::new(self)
    }
    #[doc = "Bit 10 - Enable for DMA channel"]
    #[inline(always)]
    #[must_use]
    pub fn enable10(&mut self) -> ENABLE10_W<10> {
        ENABLE10_W::new(self)
    }
    #[doc = "Bit 11 - Enable for DMA channel"]
    #[inline(always)]
    #[must_use]
    pub fn enable11(&mut self) -> ENABLE11_W<11> {
        ENABLE11_W::new(self)
    }
    #[doc = "Bit 12 - Enable for DMA channel"]
    #[inline(always)]
    #[must_use]
    pub fn enable12(&mut self) -> ENABLE12_W<12> {
        ENABLE12_W::new(self)
    }
    #[doc = "Bit 13 - Enable for DMA channel"]
    #[inline(always)]
    #[must_use]
    pub fn enable13(&mut self) -> ENABLE13_W<13> {
        ENABLE13_W::new(self)
    }
    #[doc = "Bit 14 - Enable for DMA channel"]
    #[inline(always)]
    #[must_use]
    pub fn enable14(&mut self) -> ENABLE14_W<14> {
        ENABLE14_W::new(self)
    }
    #[doc = "Bit 15 - Enable for DMA channel"]
    #[inline(always)]
    #[must_use]
    pub fn enable15(&mut self) -> ENABLE15_W<15> {
        ENABLE15_W::new(self)
    }
    #[doc = "Bit 16 - Enable for DMA channel"]
    #[inline(always)]
    #[must_use]
    pub fn enable16(&mut self) -> ENABLE16_W<16> {
        ENABLE16_W::new(self)
    }
    #[doc = "Bit 17 - Enable for DMA channel"]
    #[inline(always)]
    #[must_use]
    pub fn enable17(&mut self) -> ENABLE17_W<17> {
        ENABLE17_W::new(self)
    }
    #[doc = "Bit 18 - Enable for DMA channel"]
    #[inline(always)]
    #[must_use]
    pub fn enable18(&mut self) -> ENABLE18_W<18> {
        ENABLE18_W::new(self)
    }
    #[doc = "Bit 19 - Enable for DMA channel"]
    #[inline(always)]
    #[must_use]
    pub fn enable19(&mut self) -> ENABLE19_W<19> {
        ENABLE19_W::new(self)
    }
    #[doc = "Bit 20 - Enable for DMA channel"]
    #[inline(always)]
    #[must_use]
    pub fn enable20(&mut self) -> ENABLE20_W<20> {
        ENABLE20_W::new(self)
    }
    #[doc = "Bit 21 - Enable for DMA channel"]
    #[inline(always)]
    #[must_use]
    pub fn enable21(&mut self) -> ENABLE21_W<21> {
        ENABLE21_W::new(self)
    }
    #[doc = "Bit 22 - Enable for DMA channel"]
    #[inline(always)]
    #[must_use]
    pub fn enable22(&mut self) -> ENABLE22_W<22> {
        ENABLE22_W::new(self)
    }
    #[doc = "Bit 23 - Enable for DMA channel"]
    #[inline(always)]
    #[must_use]
    pub fn enable23(&mut self) -> ENABLE23_W<23> {
        ENABLE23_W::new(self)
    }
    #[doc = "Bit 24 - Enable for DMA channel"]
    #[inline(always)]
    #[must_use]
    pub fn enable24(&mut self) -> ENABLE24_W<24> {
        ENABLE24_W::new(self)
    }
    #[doc = "Bit 25 - Enable for DMA channel"]
    #[inline(always)]
    #[must_use]
    pub fn enable25(&mut self) -> ENABLE25_W<25> {
        ENABLE25_W::new(self)
    }
    #[doc = "Bit 26 - Enable for DMA channel"]
    #[inline(always)]
    #[must_use]
    pub fn enable26(&mut self) -> ENABLE26_W<26> {
        ENABLE26_W::new(self)
    }
    #[doc = "Bit 27 - Enable for DMA channel"]
    #[inline(always)]
    #[must_use]
    pub fn enable27(&mut self) -> ENABLE27_W<27> {
        ENABLE27_W::new(self)
    }
    #[doc = "Bit 28 - Enable for DMA channel"]
    #[inline(always)]
    #[must_use]
    pub fn enable28(&mut self) -> ENABLE28_W<28> {
        ENABLE28_W::new(self)
    }
    #[doc = "Bit 29 - Enable for DMA channel"]
    #[inline(always)]
    #[must_use]
    pub fn enable29(&mut self) -> ENABLE29_W<29> {
        ENABLE29_W::new(self)
    }
    #[doc = "Bit 30 - Enable for DMA channel"]
    #[inline(always)]
    #[must_use]
    pub fn enable30(&mut self) -> ENABLE30_W<30> {
        ENABLE30_W::new(self)
    }
    #[doc = "Bit 31 - Enable for DMA channel"]
    #[inline(always)]
    #[must_use]
    pub fn enable31(&mut self) -> ENABLE31_W<31> {
        ENABLE31_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Channel Enable read and set for all DMA channels\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [enableset0](index.html) module"]
pub struct ENABLESET0_SPEC;
impl crate::RegisterSpec for ENABLESET0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [enableset0::R](R) reader structure"]
impl crate::Readable for ENABLESET0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [enableset0::W](W) writer structure"]
impl crate::Writable for ENABLESET0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ENABLESET0 to value 0"]
impl crate::Resettable for ENABLESET0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
