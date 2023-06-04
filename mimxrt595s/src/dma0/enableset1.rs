#[doc = "Register `ENABLESET1` reader"]
pub struct R(crate::R<ENABLESET1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ENABLESET1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ENABLESET1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ENABLESET1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ENABLESET1` writer"]
pub struct W(crate::W<ENABLESET1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ENABLESET1_SPEC>;
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
impl From<crate::W<ENABLESET1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ENABLESET1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ENABLE32` reader - Enable for DMA channel"]
pub type ENABLE32_R = crate::BitReader<ENABLE32_A>;
#[doc = "Enable for DMA channel\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ENABLE32_A {
    #[doc = "0: DMA channel is disabled."]
    DISABLED = 0,
    #[doc = "1: DMA channel is enabled."]
    ENABLED = 1,
}
impl From<ENABLE32_A> for bool {
    #[inline(always)]
    fn from(variant: ENABLE32_A) -> Self {
        variant as u8 != 0
    }
}
impl ENABLE32_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ENABLE32_A {
        match self.bits {
            false => ENABLE32_A::DISABLED,
            true => ENABLE32_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ENABLE32_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ENABLE32_A::ENABLED
    }
}
#[doc = "Field `ENABLE32` writer - Enable for DMA channel"]
pub type ENABLE32_W<'a, const O: u8> = crate::BitWriter<'a, u32, ENABLESET1_SPEC, ENABLE32_A, O>;
impl<'a, const O: u8> ENABLE32_W<'a, O> {
    #[doc = "DMA channel is disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(ENABLE32_A::DISABLED)
    }
    #[doc = "DMA channel is enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(ENABLE32_A::ENABLED)
    }
}
#[doc = "Field `ENABLE33` reader - Enable for DMA channel"]
pub type ENABLE33_R = crate::BitReader<ENABLE33_A>;
#[doc = "Enable for DMA channel\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ENABLE33_A {
    #[doc = "0: DMA channel is disabled."]
    DISABLED = 0,
    #[doc = "1: DMA channel is enabled."]
    ENABLED = 1,
}
impl From<ENABLE33_A> for bool {
    #[inline(always)]
    fn from(variant: ENABLE33_A) -> Self {
        variant as u8 != 0
    }
}
impl ENABLE33_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ENABLE33_A {
        match self.bits {
            false => ENABLE33_A::DISABLED,
            true => ENABLE33_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ENABLE33_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ENABLE33_A::ENABLED
    }
}
#[doc = "Field `ENABLE33` writer - Enable for DMA channel"]
pub type ENABLE33_W<'a, const O: u8> = crate::BitWriter<'a, u32, ENABLESET1_SPEC, ENABLE33_A, O>;
impl<'a, const O: u8> ENABLE33_W<'a, O> {
    #[doc = "DMA channel is disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(ENABLE33_A::DISABLED)
    }
    #[doc = "DMA channel is enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(ENABLE33_A::ENABLED)
    }
}
#[doc = "Field `ENABLE34` reader - Enable for DMA channel"]
pub type ENABLE34_R = crate::BitReader<ENABLE34_A>;
#[doc = "Enable for DMA channel\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ENABLE34_A {
    #[doc = "0: DMA channel is disabled."]
    DISABLED = 0,
    #[doc = "1: DMA channel is enabled."]
    ENABLED = 1,
}
impl From<ENABLE34_A> for bool {
    #[inline(always)]
    fn from(variant: ENABLE34_A) -> Self {
        variant as u8 != 0
    }
}
impl ENABLE34_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ENABLE34_A {
        match self.bits {
            false => ENABLE34_A::DISABLED,
            true => ENABLE34_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ENABLE34_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ENABLE34_A::ENABLED
    }
}
#[doc = "Field `ENABLE34` writer - Enable for DMA channel"]
pub type ENABLE34_W<'a, const O: u8> = crate::BitWriter<'a, u32, ENABLESET1_SPEC, ENABLE34_A, O>;
impl<'a, const O: u8> ENABLE34_W<'a, O> {
    #[doc = "DMA channel is disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(ENABLE34_A::DISABLED)
    }
    #[doc = "DMA channel is enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(ENABLE34_A::ENABLED)
    }
}
#[doc = "Field `ENABLE35` reader - Enable for DMA channel"]
pub type ENABLE35_R = crate::BitReader<ENABLE35_A>;
#[doc = "Enable for DMA channel\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ENABLE35_A {
    #[doc = "0: DMA channel is disabled."]
    DISABLED = 0,
    #[doc = "1: DMA channel is enabled."]
    ENABLED = 1,
}
impl From<ENABLE35_A> for bool {
    #[inline(always)]
    fn from(variant: ENABLE35_A) -> Self {
        variant as u8 != 0
    }
}
impl ENABLE35_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ENABLE35_A {
        match self.bits {
            false => ENABLE35_A::DISABLED,
            true => ENABLE35_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ENABLE35_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ENABLE35_A::ENABLED
    }
}
#[doc = "Field `ENABLE35` writer - Enable for DMA channel"]
pub type ENABLE35_W<'a, const O: u8> = crate::BitWriter<'a, u32, ENABLESET1_SPEC, ENABLE35_A, O>;
impl<'a, const O: u8> ENABLE35_W<'a, O> {
    #[doc = "DMA channel is disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(ENABLE35_A::DISABLED)
    }
    #[doc = "DMA channel is enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(ENABLE35_A::ENABLED)
    }
}
#[doc = "Field `ENABLE36` reader - Enable for DMA channel"]
pub type ENABLE36_R = crate::BitReader<ENABLE36_A>;
#[doc = "Enable for DMA channel\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ENABLE36_A {
    #[doc = "0: DMA channel is disabled."]
    DISABLED = 0,
    #[doc = "1: DMA channel is enabled."]
    ENABLED = 1,
}
impl From<ENABLE36_A> for bool {
    #[inline(always)]
    fn from(variant: ENABLE36_A) -> Self {
        variant as u8 != 0
    }
}
impl ENABLE36_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ENABLE36_A {
        match self.bits {
            false => ENABLE36_A::DISABLED,
            true => ENABLE36_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ENABLE36_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ENABLE36_A::ENABLED
    }
}
#[doc = "Field `ENABLE36` writer - Enable for DMA channel"]
pub type ENABLE36_W<'a, const O: u8> = crate::BitWriter<'a, u32, ENABLESET1_SPEC, ENABLE36_A, O>;
impl<'a, const O: u8> ENABLE36_W<'a, O> {
    #[doc = "DMA channel is disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(ENABLE36_A::DISABLED)
    }
    #[doc = "DMA channel is enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(ENABLE36_A::ENABLED)
    }
}
impl R {
    #[doc = "Bit 0 - Enable for DMA channel"]
    #[inline(always)]
    pub fn enable32(&self) -> ENABLE32_R {
        ENABLE32_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enable for DMA channel"]
    #[inline(always)]
    pub fn enable33(&self) -> ENABLE33_R {
        ENABLE33_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Enable for DMA channel"]
    #[inline(always)]
    pub fn enable34(&self) -> ENABLE34_R {
        ENABLE34_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Enable for DMA channel"]
    #[inline(always)]
    pub fn enable35(&self) -> ENABLE35_R {
        ENABLE35_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Enable for DMA channel"]
    #[inline(always)]
    pub fn enable36(&self) -> ENABLE36_R {
        ENABLE36_R::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable for DMA channel"]
    #[inline(always)]
    #[must_use]
    pub fn enable32(&mut self) -> ENABLE32_W<0> {
        ENABLE32_W::new(self)
    }
    #[doc = "Bit 1 - Enable for DMA channel"]
    #[inline(always)]
    #[must_use]
    pub fn enable33(&mut self) -> ENABLE33_W<1> {
        ENABLE33_W::new(self)
    }
    #[doc = "Bit 2 - Enable for DMA channel"]
    #[inline(always)]
    #[must_use]
    pub fn enable34(&mut self) -> ENABLE34_W<2> {
        ENABLE34_W::new(self)
    }
    #[doc = "Bit 3 - Enable for DMA channel"]
    #[inline(always)]
    #[must_use]
    pub fn enable35(&mut self) -> ENABLE35_W<3> {
        ENABLE35_W::new(self)
    }
    #[doc = "Bit 4 - Enable for DMA channel"]
    #[inline(always)]
    #[must_use]
    pub fn enable36(&mut self) -> ENABLE36_W<4> {
        ENABLE36_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Channel Enable read and set for all DMA channels\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [enableset1](index.html) module"]
pub struct ENABLESET1_SPEC;
impl crate::RegisterSpec for ENABLESET1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [enableset1::R](R) reader structure"]
impl crate::Readable for ENABLESET1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [enableset1::W](W) writer structure"]
impl crate::Writable for ENABLESET1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ENABLESET1 to value 0"]
impl crate::Resettable for ENABLESET1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
