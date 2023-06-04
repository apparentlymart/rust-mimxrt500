#[doc = "Register `INTENSET0` reader"]
pub struct R(crate::R<INTENSET0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INTENSET0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INTENSET0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INTENSET0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `INTENSET0` writer"]
pub struct W(crate::W<INTENSET0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INTENSET0_SPEC>;
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
impl From<crate::W<INTENSET0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INTENSET0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `INTEN0` reader - Interrupt Enable read and set for DMA channel."]
pub type INTEN0_R = crate::BitReader<INTEN0_A>;
#[doc = "Interrupt Enable read and set for DMA channel.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum INTEN0_A {
    #[doc = "0: The Interrupt for DMA channel is disabled."]
    DISABLED = 0,
    #[doc = "1: The Interrupt for DMA channel is enabled."]
    ENABLED = 1,
}
impl From<INTEN0_A> for bool {
    #[inline(always)]
    fn from(variant: INTEN0_A) -> Self {
        variant as u8 != 0
    }
}
impl INTEN0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INTEN0_A {
        match self.bits {
            false => INTEN0_A::DISABLED,
            true => INTEN0_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == INTEN0_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == INTEN0_A::ENABLED
    }
}
#[doc = "Field `INTEN0` writer - Interrupt Enable read and set for DMA channel."]
pub type INTEN0_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTENSET0_SPEC, INTEN0_A, O>;
impl<'a, const O: u8> INTEN0_W<'a, O> {
    #[doc = "The Interrupt for DMA channel is disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(INTEN0_A::DISABLED)
    }
    #[doc = "The Interrupt for DMA channel is enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(INTEN0_A::ENABLED)
    }
}
#[doc = "Field `INTEN1` reader - Interrupt Enable read and set for DMA channel."]
pub type INTEN1_R = crate::BitReader<INTEN1_A>;
#[doc = "Interrupt Enable read and set for DMA channel.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum INTEN1_A {
    #[doc = "0: The Interrupt for DMA channel is disabled."]
    DISABLED = 0,
    #[doc = "1: The Interrupt for DMA channel is enabled."]
    ENABLED = 1,
}
impl From<INTEN1_A> for bool {
    #[inline(always)]
    fn from(variant: INTEN1_A) -> Self {
        variant as u8 != 0
    }
}
impl INTEN1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INTEN1_A {
        match self.bits {
            false => INTEN1_A::DISABLED,
            true => INTEN1_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == INTEN1_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == INTEN1_A::ENABLED
    }
}
#[doc = "Field `INTEN1` writer - Interrupt Enable read and set for DMA channel."]
pub type INTEN1_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTENSET0_SPEC, INTEN1_A, O>;
impl<'a, const O: u8> INTEN1_W<'a, O> {
    #[doc = "The Interrupt for DMA channel is disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(INTEN1_A::DISABLED)
    }
    #[doc = "The Interrupt for DMA channel is enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(INTEN1_A::ENABLED)
    }
}
#[doc = "Field `INTEN2` reader - Interrupt Enable read and set for DMA channel."]
pub type INTEN2_R = crate::BitReader<INTEN2_A>;
#[doc = "Interrupt Enable read and set for DMA channel.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum INTEN2_A {
    #[doc = "0: The Interrupt for DMA channel is disabled."]
    DISABLED = 0,
    #[doc = "1: The Interrupt for DMA channel is enabled."]
    ENABLED = 1,
}
impl From<INTEN2_A> for bool {
    #[inline(always)]
    fn from(variant: INTEN2_A) -> Self {
        variant as u8 != 0
    }
}
impl INTEN2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INTEN2_A {
        match self.bits {
            false => INTEN2_A::DISABLED,
            true => INTEN2_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == INTEN2_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == INTEN2_A::ENABLED
    }
}
#[doc = "Field `INTEN2` writer - Interrupt Enable read and set for DMA channel."]
pub type INTEN2_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTENSET0_SPEC, INTEN2_A, O>;
impl<'a, const O: u8> INTEN2_W<'a, O> {
    #[doc = "The Interrupt for DMA channel is disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(INTEN2_A::DISABLED)
    }
    #[doc = "The Interrupt for DMA channel is enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(INTEN2_A::ENABLED)
    }
}
#[doc = "Field `INTEN3` reader - Interrupt Enable read and set for DMA channel."]
pub type INTEN3_R = crate::BitReader<INTEN3_A>;
#[doc = "Interrupt Enable read and set for DMA channel.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum INTEN3_A {
    #[doc = "0: The Interrupt for DMA channel is disabled."]
    DISABLED = 0,
    #[doc = "1: The Interrupt for DMA channel is enabled."]
    ENABLED = 1,
}
impl From<INTEN3_A> for bool {
    #[inline(always)]
    fn from(variant: INTEN3_A) -> Self {
        variant as u8 != 0
    }
}
impl INTEN3_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INTEN3_A {
        match self.bits {
            false => INTEN3_A::DISABLED,
            true => INTEN3_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == INTEN3_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == INTEN3_A::ENABLED
    }
}
#[doc = "Field `INTEN3` writer - Interrupt Enable read and set for DMA channel."]
pub type INTEN3_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTENSET0_SPEC, INTEN3_A, O>;
impl<'a, const O: u8> INTEN3_W<'a, O> {
    #[doc = "The Interrupt for DMA channel is disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(INTEN3_A::DISABLED)
    }
    #[doc = "The Interrupt for DMA channel is enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(INTEN3_A::ENABLED)
    }
}
#[doc = "Field `INTEN4` reader - Interrupt Enable read and set for DMA channel."]
pub type INTEN4_R = crate::BitReader<INTEN4_A>;
#[doc = "Interrupt Enable read and set for DMA channel.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum INTEN4_A {
    #[doc = "0: The Interrupt for DMA channel is disabled."]
    DISABLED = 0,
    #[doc = "1: The Interrupt for DMA channel is enabled."]
    ENABLED = 1,
}
impl From<INTEN4_A> for bool {
    #[inline(always)]
    fn from(variant: INTEN4_A) -> Self {
        variant as u8 != 0
    }
}
impl INTEN4_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INTEN4_A {
        match self.bits {
            false => INTEN4_A::DISABLED,
            true => INTEN4_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == INTEN4_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == INTEN4_A::ENABLED
    }
}
#[doc = "Field `INTEN4` writer - Interrupt Enable read and set for DMA channel."]
pub type INTEN4_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTENSET0_SPEC, INTEN4_A, O>;
impl<'a, const O: u8> INTEN4_W<'a, O> {
    #[doc = "The Interrupt for DMA channel is disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(INTEN4_A::DISABLED)
    }
    #[doc = "The Interrupt for DMA channel is enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(INTEN4_A::ENABLED)
    }
}
#[doc = "Field `INTEN5` reader - Interrupt Enable read and set for DMA channel."]
pub type INTEN5_R = crate::BitReader<INTEN5_A>;
#[doc = "Interrupt Enable read and set for DMA channel.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum INTEN5_A {
    #[doc = "0: The Interrupt for DMA channel is disabled."]
    DISABLED = 0,
    #[doc = "1: The Interrupt for DMA channel is enabled."]
    ENABLED = 1,
}
impl From<INTEN5_A> for bool {
    #[inline(always)]
    fn from(variant: INTEN5_A) -> Self {
        variant as u8 != 0
    }
}
impl INTEN5_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INTEN5_A {
        match self.bits {
            false => INTEN5_A::DISABLED,
            true => INTEN5_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == INTEN5_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == INTEN5_A::ENABLED
    }
}
#[doc = "Field `INTEN5` writer - Interrupt Enable read and set for DMA channel."]
pub type INTEN5_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTENSET0_SPEC, INTEN5_A, O>;
impl<'a, const O: u8> INTEN5_W<'a, O> {
    #[doc = "The Interrupt for DMA channel is disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(INTEN5_A::DISABLED)
    }
    #[doc = "The Interrupt for DMA channel is enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(INTEN5_A::ENABLED)
    }
}
#[doc = "Field `INTEN6` reader - Interrupt Enable read and set for DMA channel."]
pub type INTEN6_R = crate::BitReader<INTEN6_A>;
#[doc = "Interrupt Enable read and set for DMA channel.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum INTEN6_A {
    #[doc = "0: The Interrupt for DMA channel is disabled."]
    DISABLED = 0,
    #[doc = "1: The Interrupt for DMA channel is enabled."]
    ENABLED = 1,
}
impl From<INTEN6_A> for bool {
    #[inline(always)]
    fn from(variant: INTEN6_A) -> Self {
        variant as u8 != 0
    }
}
impl INTEN6_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INTEN6_A {
        match self.bits {
            false => INTEN6_A::DISABLED,
            true => INTEN6_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == INTEN6_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == INTEN6_A::ENABLED
    }
}
#[doc = "Field `INTEN6` writer - Interrupt Enable read and set for DMA channel."]
pub type INTEN6_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTENSET0_SPEC, INTEN6_A, O>;
impl<'a, const O: u8> INTEN6_W<'a, O> {
    #[doc = "The Interrupt for DMA channel is disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(INTEN6_A::DISABLED)
    }
    #[doc = "The Interrupt for DMA channel is enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(INTEN6_A::ENABLED)
    }
}
#[doc = "Field `INTEN7` reader - Interrupt Enable read and set for DMA channel."]
pub type INTEN7_R = crate::BitReader<INTEN7_A>;
#[doc = "Interrupt Enable read and set for DMA channel.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum INTEN7_A {
    #[doc = "0: The Interrupt for DMA channel is disabled."]
    DISABLED = 0,
    #[doc = "1: The Interrupt for DMA channel is enabled."]
    ENABLED = 1,
}
impl From<INTEN7_A> for bool {
    #[inline(always)]
    fn from(variant: INTEN7_A) -> Self {
        variant as u8 != 0
    }
}
impl INTEN7_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INTEN7_A {
        match self.bits {
            false => INTEN7_A::DISABLED,
            true => INTEN7_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == INTEN7_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == INTEN7_A::ENABLED
    }
}
#[doc = "Field `INTEN7` writer - Interrupt Enable read and set for DMA channel."]
pub type INTEN7_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTENSET0_SPEC, INTEN7_A, O>;
impl<'a, const O: u8> INTEN7_W<'a, O> {
    #[doc = "The Interrupt for DMA channel is disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(INTEN7_A::DISABLED)
    }
    #[doc = "The Interrupt for DMA channel is enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(INTEN7_A::ENABLED)
    }
}
#[doc = "Field `INTEN8` reader - Interrupt Enable read and set for DMA channel."]
pub type INTEN8_R = crate::BitReader<INTEN8_A>;
#[doc = "Interrupt Enable read and set for DMA channel.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum INTEN8_A {
    #[doc = "0: The Interrupt for DMA channel is disabled."]
    DISABLED = 0,
    #[doc = "1: The Interrupt for DMA channel is enabled."]
    ENABLED = 1,
}
impl From<INTEN8_A> for bool {
    #[inline(always)]
    fn from(variant: INTEN8_A) -> Self {
        variant as u8 != 0
    }
}
impl INTEN8_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INTEN8_A {
        match self.bits {
            false => INTEN8_A::DISABLED,
            true => INTEN8_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == INTEN8_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == INTEN8_A::ENABLED
    }
}
#[doc = "Field `INTEN8` writer - Interrupt Enable read and set for DMA channel."]
pub type INTEN8_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTENSET0_SPEC, INTEN8_A, O>;
impl<'a, const O: u8> INTEN8_W<'a, O> {
    #[doc = "The Interrupt for DMA channel is disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(INTEN8_A::DISABLED)
    }
    #[doc = "The Interrupt for DMA channel is enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(INTEN8_A::ENABLED)
    }
}
#[doc = "Field `INTEN9` reader - Interrupt Enable read and set for DMA channel."]
pub type INTEN9_R = crate::BitReader<INTEN9_A>;
#[doc = "Interrupt Enable read and set for DMA channel.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum INTEN9_A {
    #[doc = "0: The Interrupt for DMA channel is disabled."]
    DISABLED = 0,
    #[doc = "1: The Interrupt for DMA channel is enabled."]
    ENABLED = 1,
}
impl From<INTEN9_A> for bool {
    #[inline(always)]
    fn from(variant: INTEN9_A) -> Self {
        variant as u8 != 0
    }
}
impl INTEN9_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INTEN9_A {
        match self.bits {
            false => INTEN9_A::DISABLED,
            true => INTEN9_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == INTEN9_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == INTEN9_A::ENABLED
    }
}
#[doc = "Field `INTEN9` writer - Interrupt Enable read and set for DMA channel."]
pub type INTEN9_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTENSET0_SPEC, INTEN9_A, O>;
impl<'a, const O: u8> INTEN9_W<'a, O> {
    #[doc = "The Interrupt for DMA channel is disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(INTEN9_A::DISABLED)
    }
    #[doc = "The Interrupt for DMA channel is enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(INTEN9_A::ENABLED)
    }
}
#[doc = "Field `INTEN10` reader - Interrupt Enable read and set for DMA channel."]
pub type INTEN10_R = crate::BitReader<INTEN10_A>;
#[doc = "Interrupt Enable read and set for DMA channel.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum INTEN10_A {
    #[doc = "0: The Interrupt for DMA channel is disabled."]
    DISABLED = 0,
    #[doc = "1: The Interrupt for DMA channel is enabled."]
    ENABLED = 1,
}
impl From<INTEN10_A> for bool {
    #[inline(always)]
    fn from(variant: INTEN10_A) -> Self {
        variant as u8 != 0
    }
}
impl INTEN10_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INTEN10_A {
        match self.bits {
            false => INTEN10_A::DISABLED,
            true => INTEN10_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == INTEN10_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == INTEN10_A::ENABLED
    }
}
#[doc = "Field `INTEN10` writer - Interrupt Enable read and set for DMA channel."]
pub type INTEN10_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTENSET0_SPEC, INTEN10_A, O>;
impl<'a, const O: u8> INTEN10_W<'a, O> {
    #[doc = "The Interrupt for DMA channel is disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(INTEN10_A::DISABLED)
    }
    #[doc = "The Interrupt for DMA channel is enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(INTEN10_A::ENABLED)
    }
}
#[doc = "Field `INTEN11` reader - Interrupt Enable read and set for DMA channel."]
pub type INTEN11_R = crate::BitReader<INTEN11_A>;
#[doc = "Interrupt Enable read and set for DMA channel.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum INTEN11_A {
    #[doc = "0: The Interrupt for DMA channel is disabled."]
    DISABLED = 0,
    #[doc = "1: The Interrupt for DMA channel is enabled."]
    ENABLED = 1,
}
impl From<INTEN11_A> for bool {
    #[inline(always)]
    fn from(variant: INTEN11_A) -> Self {
        variant as u8 != 0
    }
}
impl INTEN11_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INTEN11_A {
        match self.bits {
            false => INTEN11_A::DISABLED,
            true => INTEN11_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == INTEN11_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == INTEN11_A::ENABLED
    }
}
#[doc = "Field `INTEN11` writer - Interrupt Enable read and set for DMA channel."]
pub type INTEN11_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTENSET0_SPEC, INTEN11_A, O>;
impl<'a, const O: u8> INTEN11_W<'a, O> {
    #[doc = "The Interrupt for DMA channel is disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(INTEN11_A::DISABLED)
    }
    #[doc = "The Interrupt for DMA channel is enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(INTEN11_A::ENABLED)
    }
}
#[doc = "Field `INTEN12` reader - Interrupt Enable read and set for DMA channel."]
pub type INTEN12_R = crate::BitReader<INTEN12_A>;
#[doc = "Interrupt Enable read and set for DMA channel.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum INTEN12_A {
    #[doc = "0: The Interrupt for DMA channel is disabled."]
    DISABLED = 0,
    #[doc = "1: The Interrupt for DMA channel is enabled."]
    ENABLED = 1,
}
impl From<INTEN12_A> for bool {
    #[inline(always)]
    fn from(variant: INTEN12_A) -> Self {
        variant as u8 != 0
    }
}
impl INTEN12_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INTEN12_A {
        match self.bits {
            false => INTEN12_A::DISABLED,
            true => INTEN12_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == INTEN12_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == INTEN12_A::ENABLED
    }
}
#[doc = "Field `INTEN12` writer - Interrupt Enable read and set for DMA channel."]
pub type INTEN12_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTENSET0_SPEC, INTEN12_A, O>;
impl<'a, const O: u8> INTEN12_W<'a, O> {
    #[doc = "The Interrupt for DMA channel is disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(INTEN12_A::DISABLED)
    }
    #[doc = "The Interrupt for DMA channel is enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(INTEN12_A::ENABLED)
    }
}
#[doc = "Field `INTEN13` reader - Interrupt Enable read and set for DMA channel."]
pub type INTEN13_R = crate::BitReader<INTEN13_A>;
#[doc = "Interrupt Enable read and set for DMA channel.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum INTEN13_A {
    #[doc = "0: The Interrupt for DMA channel is disabled."]
    DISABLED = 0,
    #[doc = "1: The Interrupt for DMA channel is enabled."]
    ENABLED = 1,
}
impl From<INTEN13_A> for bool {
    #[inline(always)]
    fn from(variant: INTEN13_A) -> Self {
        variant as u8 != 0
    }
}
impl INTEN13_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INTEN13_A {
        match self.bits {
            false => INTEN13_A::DISABLED,
            true => INTEN13_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == INTEN13_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == INTEN13_A::ENABLED
    }
}
#[doc = "Field `INTEN13` writer - Interrupt Enable read and set for DMA channel."]
pub type INTEN13_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTENSET0_SPEC, INTEN13_A, O>;
impl<'a, const O: u8> INTEN13_W<'a, O> {
    #[doc = "The Interrupt for DMA channel is disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(INTEN13_A::DISABLED)
    }
    #[doc = "The Interrupt for DMA channel is enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(INTEN13_A::ENABLED)
    }
}
#[doc = "Field `INTEN14` reader - Interrupt Enable read and set for DMA channel."]
pub type INTEN14_R = crate::BitReader<INTEN14_A>;
#[doc = "Interrupt Enable read and set for DMA channel.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum INTEN14_A {
    #[doc = "0: The Interrupt for DMA channel is disabled."]
    DISABLED = 0,
    #[doc = "1: The Interrupt for DMA channel is enabled."]
    ENABLED = 1,
}
impl From<INTEN14_A> for bool {
    #[inline(always)]
    fn from(variant: INTEN14_A) -> Self {
        variant as u8 != 0
    }
}
impl INTEN14_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INTEN14_A {
        match self.bits {
            false => INTEN14_A::DISABLED,
            true => INTEN14_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == INTEN14_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == INTEN14_A::ENABLED
    }
}
#[doc = "Field `INTEN14` writer - Interrupt Enable read and set for DMA channel."]
pub type INTEN14_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTENSET0_SPEC, INTEN14_A, O>;
impl<'a, const O: u8> INTEN14_W<'a, O> {
    #[doc = "The Interrupt for DMA channel is disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(INTEN14_A::DISABLED)
    }
    #[doc = "The Interrupt for DMA channel is enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(INTEN14_A::ENABLED)
    }
}
#[doc = "Field `INTEN15` reader - Interrupt Enable read and set for DMA channel."]
pub type INTEN15_R = crate::BitReader<INTEN15_A>;
#[doc = "Interrupt Enable read and set for DMA channel.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum INTEN15_A {
    #[doc = "0: The Interrupt for DMA channel is disabled."]
    DISABLED = 0,
    #[doc = "1: The Interrupt for DMA channel is enabled."]
    ENABLED = 1,
}
impl From<INTEN15_A> for bool {
    #[inline(always)]
    fn from(variant: INTEN15_A) -> Self {
        variant as u8 != 0
    }
}
impl INTEN15_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INTEN15_A {
        match self.bits {
            false => INTEN15_A::DISABLED,
            true => INTEN15_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == INTEN15_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == INTEN15_A::ENABLED
    }
}
#[doc = "Field `INTEN15` writer - Interrupt Enable read and set for DMA channel."]
pub type INTEN15_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTENSET0_SPEC, INTEN15_A, O>;
impl<'a, const O: u8> INTEN15_W<'a, O> {
    #[doc = "The Interrupt for DMA channel is disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(INTEN15_A::DISABLED)
    }
    #[doc = "The Interrupt for DMA channel is enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(INTEN15_A::ENABLED)
    }
}
#[doc = "Field `INTEN16` reader - Interrupt Enable read and set for DMA channel."]
pub type INTEN16_R = crate::BitReader<INTEN16_A>;
#[doc = "Interrupt Enable read and set for DMA channel.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum INTEN16_A {
    #[doc = "0: The Interrupt for DMA channel is disabled."]
    DISABLED = 0,
    #[doc = "1: The Interrupt for DMA channel is enabled."]
    ENABLED = 1,
}
impl From<INTEN16_A> for bool {
    #[inline(always)]
    fn from(variant: INTEN16_A) -> Self {
        variant as u8 != 0
    }
}
impl INTEN16_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INTEN16_A {
        match self.bits {
            false => INTEN16_A::DISABLED,
            true => INTEN16_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == INTEN16_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == INTEN16_A::ENABLED
    }
}
#[doc = "Field `INTEN16` writer - Interrupt Enable read and set for DMA channel."]
pub type INTEN16_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTENSET0_SPEC, INTEN16_A, O>;
impl<'a, const O: u8> INTEN16_W<'a, O> {
    #[doc = "The Interrupt for DMA channel is disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(INTEN16_A::DISABLED)
    }
    #[doc = "The Interrupt for DMA channel is enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(INTEN16_A::ENABLED)
    }
}
#[doc = "Field `INTEN17` reader - Interrupt Enable read and set for DMA channel."]
pub type INTEN17_R = crate::BitReader<INTEN17_A>;
#[doc = "Interrupt Enable read and set for DMA channel.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum INTEN17_A {
    #[doc = "0: The Interrupt for DMA channel is disabled."]
    DISABLED = 0,
    #[doc = "1: The Interrupt for DMA channel is enabled."]
    ENABLED = 1,
}
impl From<INTEN17_A> for bool {
    #[inline(always)]
    fn from(variant: INTEN17_A) -> Self {
        variant as u8 != 0
    }
}
impl INTEN17_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INTEN17_A {
        match self.bits {
            false => INTEN17_A::DISABLED,
            true => INTEN17_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == INTEN17_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == INTEN17_A::ENABLED
    }
}
#[doc = "Field `INTEN17` writer - Interrupt Enable read and set for DMA channel."]
pub type INTEN17_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTENSET0_SPEC, INTEN17_A, O>;
impl<'a, const O: u8> INTEN17_W<'a, O> {
    #[doc = "The Interrupt for DMA channel is disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(INTEN17_A::DISABLED)
    }
    #[doc = "The Interrupt for DMA channel is enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(INTEN17_A::ENABLED)
    }
}
#[doc = "Field `INTEN18` reader - Interrupt Enable read and set for DMA channel."]
pub type INTEN18_R = crate::BitReader<INTEN18_A>;
#[doc = "Interrupt Enable read and set for DMA channel.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum INTEN18_A {
    #[doc = "0: The Interrupt for DMA channel is disabled."]
    DISABLED = 0,
    #[doc = "1: The Interrupt for DMA channel is enabled."]
    ENABLED = 1,
}
impl From<INTEN18_A> for bool {
    #[inline(always)]
    fn from(variant: INTEN18_A) -> Self {
        variant as u8 != 0
    }
}
impl INTEN18_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INTEN18_A {
        match self.bits {
            false => INTEN18_A::DISABLED,
            true => INTEN18_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == INTEN18_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == INTEN18_A::ENABLED
    }
}
#[doc = "Field `INTEN18` writer - Interrupt Enable read and set for DMA channel."]
pub type INTEN18_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTENSET0_SPEC, INTEN18_A, O>;
impl<'a, const O: u8> INTEN18_W<'a, O> {
    #[doc = "The Interrupt for DMA channel is disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(INTEN18_A::DISABLED)
    }
    #[doc = "The Interrupt for DMA channel is enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(INTEN18_A::ENABLED)
    }
}
#[doc = "Field `INTEN19` reader - Interrupt Enable read and set for DMA channel."]
pub type INTEN19_R = crate::BitReader<INTEN19_A>;
#[doc = "Interrupt Enable read and set for DMA channel.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum INTEN19_A {
    #[doc = "0: The Interrupt for DMA channel is disabled."]
    DISABLED = 0,
    #[doc = "1: The Interrupt for DMA channel is enabled."]
    ENABLED = 1,
}
impl From<INTEN19_A> for bool {
    #[inline(always)]
    fn from(variant: INTEN19_A) -> Self {
        variant as u8 != 0
    }
}
impl INTEN19_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INTEN19_A {
        match self.bits {
            false => INTEN19_A::DISABLED,
            true => INTEN19_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == INTEN19_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == INTEN19_A::ENABLED
    }
}
#[doc = "Field `INTEN19` writer - Interrupt Enable read and set for DMA channel."]
pub type INTEN19_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTENSET0_SPEC, INTEN19_A, O>;
impl<'a, const O: u8> INTEN19_W<'a, O> {
    #[doc = "The Interrupt for DMA channel is disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(INTEN19_A::DISABLED)
    }
    #[doc = "The Interrupt for DMA channel is enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(INTEN19_A::ENABLED)
    }
}
#[doc = "Field `INTEN20` reader - Interrupt Enable read and set for DMA channel."]
pub type INTEN20_R = crate::BitReader<INTEN20_A>;
#[doc = "Interrupt Enable read and set for DMA channel.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum INTEN20_A {
    #[doc = "0: The Interrupt for DMA channel is disabled."]
    DISABLED = 0,
    #[doc = "1: The Interrupt for DMA channel is enabled."]
    ENABLED = 1,
}
impl From<INTEN20_A> for bool {
    #[inline(always)]
    fn from(variant: INTEN20_A) -> Self {
        variant as u8 != 0
    }
}
impl INTEN20_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INTEN20_A {
        match self.bits {
            false => INTEN20_A::DISABLED,
            true => INTEN20_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == INTEN20_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == INTEN20_A::ENABLED
    }
}
#[doc = "Field `INTEN20` writer - Interrupt Enable read and set for DMA channel."]
pub type INTEN20_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTENSET0_SPEC, INTEN20_A, O>;
impl<'a, const O: u8> INTEN20_W<'a, O> {
    #[doc = "The Interrupt for DMA channel is disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(INTEN20_A::DISABLED)
    }
    #[doc = "The Interrupt for DMA channel is enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(INTEN20_A::ENABLED)
    }
}
#[doc = "Field `INTEN21` reader - Interrupt Enable read and set for DMA channel."]
pub type INTEN21_R = crate::BitReader<INTEN21_A>;
#[doc = "Interrupt Enable read and set for DMA channel.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum INTEN21_A {
    #[doc = "0: The Interrupt for DMA channel is disabled."]
    DISABLED = 0,
    #[doc = "1: The Interrupt for DMA channel is enabled."]
    ENABLED = 1,
}
impl From<INTEN21_A> for bool {
    #[inline(always)]
    fn from(variant: INTEN21_A) -> Self {
        variant as u8 != 0
    }
}
impl INTEN21_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INTEN21_A {
        match self.bits {
            false => INTEN21_A::DISABLED,
            true => INTEN21_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == INTEN21_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == INTEN21_A::ENABLED
    }
}
#[doc = "Field `INTEN21` writer - Interrupt Enable read and set for DMA channel."]
pub type INTEN21_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTENSET0_SPEC, INTEN21_A, O>;
impl<'a, const O: u8> INTEN21_W<'a, O> {
    #[doc = "The Interrupt for DMA channel is disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(INTEN21_A::DISABLED)
    }
    #[doc = "The Interrupt for DMA channel is enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(INTEN21_A::ENABLED)
    }
}
#[doc = "Field `INTEN22` reader - Interrupt Enable read and set for DMA channel."]
pub type INTEN22_R = crate::BitReader<INTEN22_A>;
#[doc = "Interrupt Enable read and set for DMA channel.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum INTEN22_A {
    #[doc = "0: The Interrupt for DMA channel is disabled."]
    DISABLED = 0,
    #[doc = "1: The Interrupt for DMA channel is enabled."]
    ENABLED = 1,
}
impl From<INTEN22_A> for bool {
    #[inline(always)]
    fn from(variant: INTEN22_A) -> Self {
        variant as u8 != 0
    }
}
impl INTEN22_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INTEN22_A {
        match self.bits {
            false => INTEN22_A::DISABLED,
            true => INTEN22_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == INTEN22_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == INTEN22_A::ENABLED
    }
}
#[doc = "Field `INTEN22` writer - Interrupt Enable read and set for DMA channel."]
pub type INTEN22_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTENSET0_SPEC, INTEN22_A, O>;
impl<'a, const O: u8> INTEN22_W<'a, O> {
    #[doc = "The Interrupt for DMA channel is disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(INTEN22_A::DISABLED)
    }
    #[doc = "The Interrupt for DMA channel is enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(INTEN22_A::ENABLED)
    }
}
#[doc = "Field `INTEN23` reader - Interrupt Enable read and set for DMA channel."]
pub type INTEN23_R = crate::BitReader<INTEN23_A>;
#[doc = "Interrupt Enable read and set for DMA channel.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum INTEN23_A {
    #[doc = "0: The Interrupt for DMA channel is disabled."]
    DISABLED = 0,
    #[doc = "1: The Interrupt for DMA channel is enabled."]
    ENABLED = 1,
}
impl From<INTEN23_A> for bool {
    #[inline(always)]
    fn from(variant: INTEN23_A) -> Self {
        variant as u8 != 0
    }
}
impl INTEN23_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INTEN23_A {
        match self.bits {
            false => INTEN23_A::DISABLED,
            true => INTEN23_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == INTEN23_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == INTEN23_A::ENABLED
    }
}
#[doc = "Field `INTEN23` writer - Interrupt Enable read and set for DMA channel."]
pub type INTEN23_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTENSET0_SPEC, INTEN23_A, O>;
impl<'a, const O: u8> INTEN23_W<'a, O> {
    #[doc = "The Interrupt for DMA channel is disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(INTEN23_A::DISABLED)
    }
    #[doc = "The Interrupt for DMA channel is enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(INTEN23_A::ENABLED)
    }
}
#[doc = "Field `INTEN24` reader - Interrupt Enable read and set for DMA channel."]
pub type INTEN24_R = crate::BitReader<INTEN24_A>;
#[doc = "Interrupt Enable read and set for DMA channel.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum INTEN24_A {
    #[doc = "0: The Interrupt for DMA channel is disabled."]
    DISABLED = 0,
    #[doc = "1: The Interrupt for DMA channel is enabled."]
    ENABLED = 1,
}
impl From<INTEN24_A> for bool {
    #[inline(always)]
    fn from(variant: INTEN24_A) -> Self {
        variant as u8 != 0
    }
}
impl INTEN24_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INTEN24_A {
        match self.bits {
            false => INTEN24_A::DISABLED,
            true => INTEN24_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == INTEN24_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == INTEN24_A::ENABLED
    }
}
#[doc = "Field `INTEN24` writer - Interrupt Enable read and set for DMA channel."]
pub type INTEN24_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTENSET0_SPEC, INTEN24_A, O>;
impl<'a, const O: u8> INTEN24_W<'a, O> {
    #[doc = "The Interrupt for DMA channel is disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(INTEN24_A::DISABLED)
    }
    #[doc = "The Interrupt for DMA channel is enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(INTEN24_A::ENABLED)
    }
}
#[doc = "Field `INTEN25` reader - Interrupt Enable read and set for DMA channel."]
pub type INTEN25_R = crate::BitReader<INTEN25_A>;
#[doc = "Interrupt Enable read and set for DMA channel.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum INTEN25_A {
    #[doc = "0: The Interrupt for DMA channel is disabled."]
    DISABLED = 0,
    #[doc = "1: The Interrupt for DMA channel is enabled."]
    ENABLED = 1,
}
impl From<INTEN25_A> for bool {
    #[inline(always)]
    fn from(variant: INTEN25_A) -> Self {
        variant as u8 != 0
    }
}
impl INTEN25_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INTEN25_A {
        match self.bits {
            false => INTEN25_A::DISABLED,
            true => INTEN25_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == INTEN25_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == INTEN25_A::ENABLED
    }
}
#[doc = "Field `INTEN25` writer - Interrupt Enable read and set for DMA channel."]
pub type INTEN25_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTENSET0_SPEC, INTEN25_A, O>;
impl<'a, const O: u8> INTEN25_W<'a, O> {
    #[doc = "The Interrupt for DMA channel is disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(INTEN25_A::DISABLED)
    }
    #[doc = "The Interrupt for DMA channel is enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(INTEN25_A::ENABLED)
    }
}
#[doc = "Field `INTEN26` reader - Interrupt Enable read and set for DMA channel."]
pub type INTEN26_R = crate::BitReader<INTEN26_A>;
#[doc = "Interrupt Enable read and set for DMA channel.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum INTEN26_A {
    #[doc = "0: The Interrupt for DMA channel is disabled."]
    DISABLED = 0,
    #[doc = "1: The Interrupt for DMA channel is enabled."]
    ENABLED = 1,
}
impl From<INTEN26_A> for bool {
    #[inline(always)]
    fn from(variant: INTEN26_A) -> Self {
        variant as u8 != 0
    }
}
impl INTEN26_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INTEN26_A {
        match self.bits {
            false => INTEN26_A::DISABLED,
            true => INTEN26_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == INTEN26_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == INTEN26_A::ENABLED
    }
}
#[doc = "Field `INTEN26` writer - Interrupt Enable read and set for DMA channel."]
pub type INTEN26_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTENSET0_SPEC, INTEN26_A, O>;
impl<'a, const O: u8> INTEN26_W<'a, O> {
    #[doc = "The Interrupt for DMA channel is disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(INTEN26_A::DISABLED)
    }
    #[doc = "The Interrupt for DMA channel is enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(INTEN26_A::ENABLED)
    }
}
#[doc = "Field `INTEN27` reader - Interrupt Enable read and set for DMA channel."]
pub type INTEN27_R = crate::BitReader<INTEN27_A>;
#[doc = "Interrupt Enable read and set for DMA channel.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum INTEN27_A {
    #[doc = "0: The Interrupt for DMA channel is disabled."]
    DISABLED = 0,
    #[doc = "1: The Interrupt for DMA channel is enabled."]
    ENABLED = 1,
}
impl From<INTEN27_A> for bool {
    #[inline(always)]
    fn from(variant: INTEN27_A) -> Self {
        variant as u8 != 0
    }
}
impl INTEN27_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INTEN27_A {
        match self.bits {
            false => INTEN27_A::DISABLED,
            true => INTEN27_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == INTEN27_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == INTEN27_A::ENABLED
    }
}
#[doc = "Field `INTEN27` writer - Interrupt Enable read and set for DMA channel."]
pub type INTEN27_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTENSET0_SPEC, INTEN27_A, O>;
impl<'a, const O: u8> INTEN27_W<'a, O> {
    #[doc = "The Interrupt for DMA channel is disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(INTEN27_A::DISABLED)
    }
    #[doc = "The Interrupt for DMA channel is enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(INTEN27_A::ENABLED)
    }
}
#[doc = "Field `INTEN28` reader - Interrupt Enable read and set for DMA channel."]
pub type INTEN28_R = crate::BitReader<INTEN28_A>;
#[doc = "Interrupt Enable read and set for DMA channel.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum INTEN28_A {
    #[doc = "0: The Interrupt for DMA channel is disabled."]
    DISABLED = 0,
    #[doc = "1: The Interrupt for DMA channel is enabled."]
    ENABLED = 1,
}
impl From<INTEN28_A> for bool {
    #[inline(always)]
    fn from(variant: INTEN28_A) -> Self {
        variant as u8 != 0
    }
}
impl INTEN28_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INTEN28_A {
        match self.bits {
            false => INTEN28_A::DISABLED,
            true => INTEN28_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == INTEN28_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == INTEN28_A::ENABLED
    }
}
#[doc = "Field `INTEN28` writer - Interrupt Enable read and set for DMA channel."]
pub type INTEN28_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTENSET0_SPEC, INTEN28_A, O>;
impl<'a, const O: u8> INTEN28_W<'a, O> {
    #[doc = "The Interrupt for DMA channel is disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(INTEN28_A::DISABLED)
    }
    #[doc = "The Interrupt for DMA channel is enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(INTEN28_A::ENABLED)
    }
}
#[doc = "Field `INTEN29` reader - Interrupt Enable read and set for DMA channel."]
pub type INTEN29_R = crate::BitReader<INTEN29_A>;
#[doc = "Interrupt Enable read and set for DMA channel.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum INTEN29_A {
    #[doc = "0: The Interrupt for DMA channel is disabled."]
    DISABLED = 0,
    #[doc = "1: The Interrupt for DMA channel is enabled."]
    ENABLED = 1,
}
impl From<INTEN29_A> for bool {
    #[inline(always)]
    fn from(variant: INTEN29_A) -> Self {
        variant as u8 != 0
    }
}
impl INTEN29_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INTEN29_A {
        match self.bits {
            false => INTEN29_A::DISABLED,
            true => INTEN29_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == INTEN29_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == INTEN29_A::ENABLED
    }
}
#[doc = "Field `INTEN29` writer - Interrupt Enable read and set for DMA channel."]
pub type INTEN29_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTENSET0_SPEC, INTEN29_A, O>;
impl<'a, const O: u8> INTEN29_W<'a, O> {
    #[doc = "The Interrupt for DMA channel is disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(INTEN29_A::DISABLED)
    }
    #[doc = "The Interrupt for DMA channel is enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(INTEN29_A::ENABLED)
    }
}
#[doc = "Field `INTEN30` reader - Interrupt Enable read and set for DMA channel."]
pub type INTEN30_R = crate::BitReader<INTEN30_A>;
#[doc = "Interrupt Enable read and set for DMA channel.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum INTEN30_A {
    #[doc = "0: The Interrupt for DMA channel is disabled."]
    DISABLED = 0,
    #[doc = "1: The Interrupt for DMA channel is enabled."]
    ENABLED = 1,
}
impl From<INTEN30_A> for bool {
    #[inline(always)]
    fn from(variant: INTEN30_A) -> Self {
        variant as u8 != 0
    }
}
impl INTEN30_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INTEN30_A {
        match self.bits {
            false => INTEN30_A::DISABLED,
            true => INTEN30_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == INTEN30_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == INTEN30_A::ENABLED
    }
}
#[doc = "Field `INTEN30` writer - Interrupt Enable read and set for DMA channel."]
pub type INTEN30_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTENSET0_SPEC, INTEN30_A, O>;
impl<'a, const O: u8> INTEN30_W<'a, O> {
    #[doc = "The Interrupt for DMA channel is disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(INTEN30_A::DISABLED)
    }
    #[doc = "The Interrupt for DMA channel is enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(INTEN30_A::ENABLED)
    }
}
#[doc = "Field `INTEN31` reader - Interrupt Enable read and set for DMA channel."]
pub type INTEN31_R = crate::BitReader<INTEN31_A>;
#[doc = "Interrupt Enable read and set for DMA channel.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum INTEN31_A {
    #[doc = "0: The Interrupt for DMA channel is disabled."]
    DISABLED = 0,
    #[doc = "1: The Interrupt for DMA channel is enabled."]
    ENABLED = 1,
}
impl From<INTEN31_A> for bool {
    #[inline(always)]
    fn from(variant: INTEN31_A) -> Self {
        variant as u8 != 0
    }
}
impl INTEN31_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INTEN31_A {
        match self.bits {
            false => INTEN31_A::DISABLED,
            true => INTEN31_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == INTEN31_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == INTEN31_A::ENABLED
    }
}
#[doc = "Field `INTEN31` writer - Interrupt Enable read and set for DMA channel."]
pub type INTEN31_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTENSET0_SPEC, INTEN31_A, O>;
impl<'a, const O: u8> INTEN31_W<'a, O> {
    #[doc = "The Interrupt for DMA channel is disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(INTEN31_A::DISABLED)
    }
    #[doc = "The Interrupt for DMA channel is enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(INTEN31_A::ENABLED)
    }
}
impl R {
    #[doc = "Bit 0 - Interrupt Enable read and set for DMA channel."]
    #[inline(always)]
    pub fn inten0(&self) -> INTEN0_R {
        INTEN0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Interrupt Enable read and set for DMA channel."]
    #[inline(always)]
    pub fn inten1(&self) -> INTEN1_R {
        INTEN1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Interrupt Enable read and set for DMA channel."]
    #[inline(always)]
    pub fn inten2(&self) -> INTEN2_R {
        INTEN2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Interrupt Enable read and set for DMA channel."]
    #[inline(always)]
    pub fn inten3(&self) -> INTEN3_R {
        INTEN3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Interrupt Enable read and set for DMA channel."]
    #[inline(always)]
    pub fn inten4(&self) -> INTEN4_R {
        INTEN4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Interrupt Enable read and set for DMA channel."]
    #[inline(always)]
    pub fn inten5(&self) -> INTEN5_R {
        INTEN5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Interrupt Enable read and set for DMA channel."]
    #[inline(always)]
    pub fn inten6(&self) -> INTEN6_R {
        INTEN6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Interrupt Enable read and set for DMA channel."]
    #[inline(always)]
    pub fn inten7(&self) -> INTEN7_R {
        INTEN7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Interrupt Enable read and set for DMA channel."]
    #[inline(always)]
    pub fn inten8(&self) -> INTEN8_R {
        INTEN8_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Interrupt Enable read and set for DMA channel."]
    #[inline(always)]
    pub fn inten9(&self) -> INTEN9_R {
        INTEN9_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Interrupt Enable read and set for DMA channel."]
    #[inline(always)]
    pub fn inten10(&self) -> INTEN10_R {
        INTEN10_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Interrupt Enable read and set for DMA channel."]
    #[inline(always)]
    pub fn inten11(&self) -> INTEN11_R {
        INTEN11_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Interrupt Enable read and set for DMA channel."]
    #[inline(always)]
    pub fn inten12(&self) -> INTEN12_R {
        INTEN12_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Interrupt Enable read and set for DMA channel."]
    #[inline(always)]
    pub fn inten13(&self) -> INTEN13_R {
        INTEN13_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Interrupt Enable read and set for DMA channel."]
    #[inline(always)]
    pub fn inten14(&self) -> INTEN14_R {
        INTEN14_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Interrupt Enable read and set for DMA channel."]
    #[inline(always)]
    pub fn inten15(&self) -> INTEN15_R {
        INTEN15_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Interrupt Enable read and set for DMA channel."]
    #[inline(always)]
    pub fn inten16(&self) -> INTEN16_R {
        INTEN16_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Interrupt Enable read and set for DMA channel."]
    #[inline(always)]
    pub fn inten17(&self) -> INTEN17_R {
        INTEN17_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Interrupt Enable read and set for DMA channel."]
    #[inline(always)]
    pub fn inten18(&self) -> INTEN18_R {
        INTEN18_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Interrupt Enable read and set for DMA channel."]
    #[inline(always)]
    pub fn inten19(&self) -> INTEN19_R {
        INTEN19_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Interrupt Enable read and set for DMA channel."]
    #[inline(always)]
    pub fn inten20(&self) -> INTEN20_R {
        INTEN20_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Interrupt Enable read and set for DMA channel."]
    #[inline(always)]
    pub fn inten21(&self) -> INTEN21_R {
        INTEN21_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Interrupt Enable read and set for DMA channel."]
    #[inline(always)]
    pub fn inten22(&self) -> INTEN22_R {
        INTEN22_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Interrupt Enable read and set for DMA channel."]
    #[inline(always)]
    pub fn inten23(&self) -> INTEN23_R {
        INTEN23_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Interrupt Enable read and set for DMA channel."]
    #[inline(always)]
    pub fn inten24(&self) -> INTEN24_R {
        INTEN24_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Interrupt Enable read and set for DMA channel."]
    #[inline(always)]
    pub fn inten25(&self) -> INTEN25_R {
        INTEN25_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Interrupt Enable read and set for DMA channel."]
    #[inline(always)]
    pub fn inten26(&self) -> INTEN26_R {
        INTEN26_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Interrupt Enable read and set for DMA channel."]
    #[inline(always)]
    pub fn inten27(&self) -> INTEN27_R {
        INTEN27_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Interrupt Enable read and set for DMA channel."]
    #[inline(always)]
    pub fn inten28(&self) -> INTEN28_R {
        INTEN28_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Interrupt Enable read and set for DMA channel."]
    #[inline(always)]
    pub fn inten29(&self) -> INTEN29_R {
        INTEN29_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Interrupt Enable read and set for DMA channel."]
    #[inline(always)]
    pub fn inten30(&self) -> INTEN30_R {
        INTEN30_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Interrupt Enable read and set for DMA channel."]
    #[inline(always)]
    pub fn inten31(&self) -> INTEN31_R {
        INTEN31_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Interrupt Enable read and set for DMA channel."]
    #[inline(always)]
    #[must_use]
    pub fn inten0(&mut self) -> INTEN0_W<0> {
        INTEN0_W::new(self)
    }
    #[doc = "Bit 1 - Interrupt Enable read and set for DMA channel."]
    #[inline(always)]
    #[must_use]
    pub fn inten1(&mut self) -> INTEN1_W<1> {
        INTEN1_W::new(self)
    }
    #[doc = "Bit 2 - Interrupt Enable read and set for DMA channel."]
    #[inline(always)]
    #[must_use]
    pub fn inten2(&mut self) -> INTEN2_W<2> {
        INTEN2_W::new(self)
    }
    #[doc = "Bit 3 - Interrupt Enable read and set for DMA channel."]
    #[inline(always)]
    #[must_use]
    pub fn inten3(&mut self) -> INTEN3_W<3> {
        INTEN3_W::new(self)
    }
    #[doc = "Bit 4 - Interrupt Enable read and set for DMA channel."]
    #[inline(always)]
    #[must_use]
    pub fn inten4(&mut self) -> INTEN4_W<4> {
        INTEN4_W::new(self)
    }
    #[doc = "Bit 5 - Interrupt Enable read and set for DMA channel."]
    #[inline(always)]
    #[must_use]
    pub fn inten5(&mut self) -> INTEN5_W<5> {
        INTEN5_W::new(self)
    }
    #[doc = "Bit 6 - Interrupt Enable read and set for DMA channel."]
    #[inline(always)]
    #[must_use]
    pub fn inten6(&mut self) -> INTEN6_W<6> {
        INTEN6_W::new(self)
    }
    #[doc = "Bit 7 - Interrupt Enable read and set for DMA channel."]
    #[inline(always)]
    #[must_use]
    pub fn inten7(&mut self) -> INTEN7_W<7> {
        INTEN7_W::new(self)
    }
    #[doc = "Bit 8 - Interrupt Enable read and set for DMA channel."]
    #[inline(always)]
    #[must_use]
    pub fn inten8(&mut self) -> INTEN8_W<8> {
        INTEN8_W::new(self)
    }
    #[doc = "Bit 9 - Interrupt Enable read and set for DMA channel."]
    #[inline(always)]
    #[must_use]
    pub fn inten9(&mut self) -> INTEN9_W<9> {
        INTEN9_W::new(self)
    }
    #[doc = "Bit 10 - Interrupt Enable read and set for DMA channel."]
    #[inline(always)]
    #[must_use]
    pub fn inten10(&mut self) -> INTEN10_W<10> {
        INTEN10_W::new(self)
    }
    #[doc = "Bit 11 - Interrupt Enable read and set for DMA channel."]
    #[inline(always)]
    #[must_use]
    pub fn inten11(&mut self) -> INTEN11_W<11> {
        INTEN11_W::new(self)
    }
    #[doc = "Bit 12 - Interrupt Enable read and set for DMA channel."]
    #[inline(always)]
    #[must_use]
    pub fn inten12(&mut self) -> INTEN12_W<12> {
        INTEN12_W::new(self)
    }
    #[doc = "Bit 13 - Interrupt Enable read and set for DMA channel."]
    #[inline(always)]
    #[must_use]
    pub fn inten13(&mut self) -> INTEN13_W<13> {
        INTEN13_W::new(self)
    }
    #[doc = "Bit 14 - Interrupt Enable read and set for DMA channel."]
    #[inline(always)]
    #[must_use]
    pub fn inten14(&mut self) -> INTEN14_W<14> {
        INTEN14_W::new(self)
    }
    #[doc = "Bit 15 - Interrupt Enable read and set for DMA channel."]
    #[inline(always)]
    #[must_use]
    pub fn inten15(&mut self) -> INTEN15_W<15> {
        INTEN15_W::new(self)
    }
    #[doc = "Bit 16 - Interrupt Enable read and set for DMA channel."]
    #[inline(always)]
    #[must_use]
    pub fn inten16(&mut self) -> INTEN16_W<16> {
        INTEN16_W::new(self)
    }
    #[doc = "Bit 17 - Interrupt Enable read and set for DMA channel."]
    #[inline(always)]
    #[must_use]
    pub fn inten17(&mut self) -> INTEN17_W<17> {
        INTEN17_W::new(self)
    }
    #[doc = "Bit 18 - Interrupt Enable read and set for DMA channel."]
    #[inline(always)]
    #[must_use]
    pub fn inten18(&mut self) -> INTEN18_W<18> {
        INTEN18_W::new(self)
    }
    #[doc = "Bit 19 - Interrupt Enable read and set for DMA channel."]
    #[inline(always)]
    #[must_use]
    pub fn inten19(&mut self) -> INTEN19_W<19> {
        INTEN19_W::new(self)
    }
    #[doc = "Bit 20 - Interrupt Enable read and set for DMA channel."]
    #[inline(always)]
    #[must_use]
    pub fn inten20(&mut self) -> INTEN20_W<20> {
        INTEN20_W::new(self)
    }
    #[doc = "Bit 21 - Interrupt Enable read and set for DMA channel."]
    #[inline(always)]
    #[must_use]
    pub fn inten21(&mut self) -> INTEN21_W<21> {
        INTEN21_W::new(self)
    }
    #[doc = "Bit 22 - Interrupt Enable read and set for DMA channel."]
    #[inline(always)]
    #[must_use]
    pub fn inten22(&mut self) -> INTEN22_W<22> {
        INTEN22_W::new(self)
    }
    #[doc = "Bit 23 - Interrupt Enable read and set for DMA channel."]
    #[inline(always)]
    #[must_use]
    pub fn inten23(&mut self) -> INTEN23_W<23> {
        INTEN23_W::new(self)
    }
    #[doc = "Bit 24 - Interrupt Enable read and set for DMA channel."]
    #[inline(always)]
    #[must_use]
    pub fn inten24(&mut self) -> INTEN24_W<24> {
        INTEN24_W::new(self)
    }
    #[doc = "Bit 25 - Interrupt Enable read and set for DMA channel."]
    #[inline(always)]
    #[must_use]
    pub fn inten25(&mut self) -> INTEN25_W<25> {
        INTEN25_W::new(self)
    }
    #[doc = "Bit 26 - Interrupt Enable read and set for DMA channel."]
    #[inline(always)]
    #[must_use]
    pub fn inten26(&mut self) -> INTEN26_W<26> {
        INTEN26_W::new(self)
    }
    #[doc = "Bit 27 - Interrupt Enable read and set for DMA channel."]
    #[inline(always)]
    #[must_use]
    pub fn inten27(&mut self) -> INTEN27_W<27> {
        INTEN27_W::new(self)
    }
    #[doc = "Bit 28 - Interrupt Enable read and set for DMA channel."]
    #[inline(always)]
    #[must_use]
    pub fn inten28(&mut self) -> INTEN28_W<28> {
        INTEN28_W::new(self)
    }
    #[doc = "Bit 29 - Interrupt Enable read and set for DMA channel."]
    #[inline(always)]
    #[must_use]
    pub fn inten29(&mut self) -> INTEN29_W<29> {
        INTEN29_W::new(self)
    }
    #[doc = "Bit 30 - Interrupt Enable read and set for DMA channel."]
    #[inline(always)]
    #[must_use]
    pub fn inten30(&mut self) -> INTEN30_W<30> {
        INTEN30_W::new(self)
    }
    #[doc = "Bit 31 - Interrupt Enable read and set for DMA channel."]
    #[inline(always)]
    #[must_use]
    pub fn inten31(&mut self) -> INTEN31_W<31> {
        INTEN31_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt Enable read and Set for all DMA channels\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intenset0](index.html) module"]
pub struct INTENSET0_SPEC;
impl crate::RegisterSpec for INTENSET0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [intenset0::R](R) reader structure"]
impl crate::Readable for INTENSET0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [intenset0::W](W) writer structure"]
impl crate::Writable for INTENSET0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets INTENSET0 to value 0"]
impl crate::Resettable for INTENSET0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
