#[doc = "Register `INTENSET1` reader"]
pub struct R(crate::R<INTENSET1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INTENSET1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INTENSET1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INTENSET1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `INTENSET1` writer"]
pub struct W(crate::W<INTENSET1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INTENSET1_SPEC>;
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
impl From<crate::W<INTENSET1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INTENSET1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `INTEN32` reader - Interrupt Enable read and set for DMA channel."]
pub type INTEN32_R = crate::BitReader<INTEN32_A>;
#[doc = "Interrupt Enable read and set for DMA channel.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum INTEN32_A {
    #[doc = "0: The Interrupt for DMA channel is disabled."]
    DISABLED = 0,
    #[doc = "1: The Interrupt for DMA channel is enabled."]
    ENABLED = 1,
}
impl From<INTEN32_A> for bool {
    #[inline(always)]
    fn from(variant: INTEN32_A) -> Self {
        variant as u8 != 0
    }
}
impl INTEN32_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INTEN32_A {
        match self.bits {
            false => INTEN32_A::DISABLED,
            true => INTEN32_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == INTEN32_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == INTEN32_A::ENABLED
    }
}
#[doc = "Field `INTEN32` writer - Interrupt Enable read and set for DMA channel."]
pub type INTEN32_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTENSET1_SPEC, INTEN32_A, O>;
impl<'a, const O: u8> INTEN32_W<'a, O> {
    #[doc = "The Interrupt for DMA channel is disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(INTEN32_A::DISABLED)
    }
    #[doc = "The Interrupt for DMA channel is enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(INTEN32_A::ENABLED)
    }
}
#[doc = "Field `INTEN33` reader - Interrupt Enable read and set for DMA channel."]
pub type INTEN33_R = crate::BitReader<INTEN33_A>;
#[doc = "Interrupt Enable read and set for DMA channel.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum INTEN33_A {
    #[doc = "0: The Interrupt for DMA channel is disabled."]
    DISABLED = 0,
    #[doc = "1: The Interrupt for DMA channel is enabled."]
    ENABLED = 1,
}
impl From<INTEN33_A> for bool {
    #[inline(always)]
    fn from(variant: INTEN33_A) -> Self {
        variant as u8 != 0
    }
}
impl INTEN33_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INTEN33_A {
        match self.bits {
            false => INTEN33_A::DISABLED,
            true => INTEN33_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == INTEN33_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == INTEN33_A::ENABLED
    }
}
#[doc = "Field `INTEN33` writer - Interrupt Enable read and set for DMA channel."]
pub type INTEN33_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTENSET1_SPEC, INTEN33_A, O>;
impl<'a, const O: u8> INTEN33_W<'a, O> {
    #[doc = "The Interrupt for DMA channel is disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(INTEN33_A::DISABLED)
    }
    #[doc = "The Interrupt for DMA channel is enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(INTEN33_A::ENABLED)
    }
}
#[doc = "Field `INTEN34` reader - Interrupt Enable read and set for DMA channel."]
pub type INTEN34_R = crate::BitReader<INTEN34_A>;
#[doc = "Interrupt Enable read and set for DMA channel.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum INTEN34_A {
    #[doc = "0: The Interrupt for DMA channel is disabled."]
    DISABLED = 0,
    #[doc = "1: The Interrupt for DMA channel is enabled."]
    ENABLED = 1,
}
impl From<INTEN34_A> for bool {
    #[inline(always)]
    fn from(variant: INTEN34_A) -> Self {
        variant as u8 != 0
    }
}
impl INTEN34_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INTEN34_A {
        match self.bits {
            false => INTEN34_A::DISABLED,
            true => INTEN34_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == INTEN34_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == INTEN34_A::ENABLED
    }
}
#[doc = "Field `INTEN34` writer - Interrupt Enable read and set for DMA channel."]
pub type INTEN34_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTENSET1_SPEC, INTEN34_A, O>;
impl<'a, const O: u8> INTEN34_W<'a, O> {
    #[doc = "The Interrupt for DMA channel is disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(INTEN34_A::DISABLED)
    }
    #[doc = "The Interrupt for DMA channel is enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(INTEN34_A::ENABLED)
    }
}
#[doc = "Field `INTEN35` reader - Interrupt Enable read and set for DMA channel."]
pub type INTEN35_R = crate::BitReader<INTEN35_A>;
#[doc = "Interrupt Enable read and set for DMA channel.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum INTEN35_A {
    #[doc = "0: The Interrupt for DMA channel is disabled."]
    DISABLED = 0,
    #[doc = "1: The Interrupt for DMA channel is enabled."]
    ENABLED = 1,
}
impl From<INTEN35_A> for bool {
    #[inline(always)]
    fn from(variant: INTEN35_A) -> Self {
        variant as u8 != 0
    }
}
impl INTEN35_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INTEN35_A {
        match self.bits {
            false => INTEN35_A::DISABLED,
            true => INTEN35_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == INTEN35_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == INTEN35_A::ENABLED
    }
}
#[doc = "Field `INTEN35` writer - Interrupt Enable read and set for DMA channel."]
pub type INTEN35_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTENSET1_SPEC, INTEN35_A, O>;
impl<'a, const O: u8> INTEN35_W<'a, O> {
    #[doc = "The Interrupt for DMA channel is disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(INTEN35_A::DISABLED)
    }
    #[doc = "The Interrupt for DMA channel is enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(INTEN35_A::ENABLED)
    }
}
#[doc = "Field `INTEN36` reader - Interrupt Enable read and set for DMA channel."]
pub type INTEN36_R = crate::BitReader<INTEN36_A>;
#[doc = "Interrupt Enable read and set for DMA channel.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum INTEN36_A {
    #[doc = "0: The Interrupt for DMA channel is disabled."]
    DISABLED = 0,
    #[doc = "1: The Interrupt for DMA channel is enabled."]
    ENABLED = 1,
}
impl From<INTEN36_A> for bool {
    #[inline(always)]
    fn from(variant: INTEN36_A) -> Self {
        variant as u8 != 0
    }
}
impl INTEN36_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INTEN36_A {
        match self.bits {
            false => INTEN36_A::DISABLED,
            true => INTEN36_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == INTEN36_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == INTEN36_A::ENABLED
    }
}
#[doc = "Field `INTEN36` writer - Interrupt Enable read and set for DMA channel."]
pub type INTEN36_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTENSET1_SPEC, INTEN36_A, O>;
impl<'a, const O: u8> INTEN36_W<'a, O> {
    #[doc = "The Interrupt for DMA channel is disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(INTEN36_A::DISABLED)
    }
    #[doc = "The Interrupt for DMA channel is enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(INTEN36_A::ENABLED)
    }
}
impl R {
    #[doc = "Bit 0 - Interrupt Enable read and set for DMA channel."]
    #[inline(always)]
    pub fn inten32(&self) -> INTEN32_R {
        INTEN32_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Interrupt Enable read and set for DMA channel."]
    #[inline(always)]
    pub fn inten33(&self) -> INTEN33_R {
        INTEN33_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Interrupt Enable read and set for DMA channel."]
    #[inline(always)]
    pub fn inten34(&self) -> INTEN34_R {
        INTEN34_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Interrupt Enable read and set for DMA channel."]
    #[inline(always)]
    pub fn inten35(&self) -> INTEN35_R {
        INTEN35_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Interrupt Enable read and set for DMA channel."]
    #[inline(always)]
    pub fn inten36(&self) -> INTEN36_R {
        INTEN36_R::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Interrupt Enable read and set for DMA channel."]
    #[inline(always)]
    #[must_use]
    pub fn inten32(&mut self) -> INTEN32_W<0> {
        INTEN32_W::new(self)
    }
    #[doc = "Bit 1 - Interrupt Enable read and set for DMA channel."]
    #[inline(always)]
    #[must_use]
    pub fn inten33(&mut self) -> INTEN33_W<1> {
        INTEN33_W::new(self)
    }
    #[doc = "Bit 2 - Interrupt Enable read and set for DMA channel."]
    #[inline(always)]
    #[must_use]
    pub fn inten34(&mut self) -> INTEN34_W<2> {
        INTEN34_W::new(self)
    }
    #[doc = "Bit 3 - Interrupt Enable read and set for DMA channel."]
    #[inline(always)]
    #[must_use]
    pub fn inten35(&mut self) -> INTEN35_W<3> {
        INTEN35_W::new(self)
    }
    #[doc = "Bit 4 - Interrupt Enable read and set for DMA channel."]
    #[inline(always)]
    #[must_use]
    pub fn inten36(&mut self) -> INTEN36_W<4> {
        INTEN36_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt Enable read and Set for all DMA channels\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intenset1](index.html) module"]
pub struct INTENSET1_SPEC;
impl crate::RegisterSpec for INTENSET1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [intenset1::R](R) reader structure"]
impl crate::Readable for INTENSET1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [intenset1::W](W) writer structure"]
impl crate::Writable for INTENSET1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets INTENSET1 to value 0"]
impl crate::Resettable for INTENSET1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
