#[doc = "Register `ABORT1` writer"]
pub struct W(crate::W<ABORT1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ABORT1_SPEC>;
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
impl From<crate::W<ABORT1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ABORT1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Abort control for DMA channel.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ABORT32_AW {
    #[doc = "0: No effect."]
    NO_EFFECT = 0,
    #[doc = "1: Aborts DMA operations on channel."]
    EFFECT = 1,
}
impl From<ABORT32_AW> for bool {
    #[inline(always)]
    fn from(variant: ABORT32_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ABORT32` writer - Abort control for DMA channel."]
pub type ABORT32_W<'a, const O: u8> = crate::BitWriter<'a, u32, ABORT1_SPEC, ABORT32_AW, O>;
impl<'a, const O: u8> ABORT32_W<'a, O> {
    #[doc = "No effect."]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(ABORT32_AW::NO_EFFECT)
    }
    #[doc = "Aborts DMA operations on channel."]
    #[inline(always)]
    pub fn effect(self) -> &'a mut W {
        self.variant(ABORT32_AW::EFFECT)
    }
}
#[doc = "Abort control for DMA channel.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ABORT33_AW {
    #[doc = "0: No effect."]
    NO_EFFECT = 0,
    #[doc = "1: Aborts DMA operations on channel."]
    EFFECT = 1,
}
impl From<ABORT33_AW> for bool {
    #[inline(always)]
    fn from(variant: ABORT33_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ABORT33` writer - Abort control for DMA channel."]
pub type ABORT33_W<'a, const O: u8> = crate::BitWriter<'a, u32, ABORT1_SPEC, ABORT33_AW, O>;
impl<'a, const O: u8> ABORT33_W<'a, O> {
    #[doc = "No effect."]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(ABORT33_AW::NO_EFFECT)
    }
    #[doc = "Aborts DMA operations on channel."]
    #[inline(always)]
    pub fn effect(self) -> &'a mut W {
        self.variant(ABORT33_AW::EFFECT)
    }
}
#[doc = "Abort control for DMA channel.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ABORT34_AW {
    #[doc = "0: No effect."]
    NO_EFFECT = 0,
    #[doc = "1: Aborts DMA operations on channel."]
    EFFECT = 1,
}
impl From<ABORT34_AW> for bool {
    #[inline(always)]
    fn from(variant: ABORT34_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ABORT34` writer - Abort control for DMA channel."]
pub type ABORT34_W<'a, const O: u8> = crate::BitWriter<'a, u32, ABORT1_SPEC, ABORT34_AW, O>;
impl<'a, const O: u8> ABORT34_W<'a, O> {
    #[doc = "No effect."]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(ABORT34_AW::NO_EFFECT)
    }
    #[doc = "Aborts DMA operations on channel."]
    #[inline(always)]
    pub fn effect(self) -> &'a mut W {
        self.variant(ABORT34_AW::EFFECT)
    }
}
#[doc = "Abort control for DMA channel.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ABORT35_AW {
    #[doc = "0: No effect."]
    NO_EFFECT = 0,
    #[doc = "1: Aborts DMA operations on channel."]
    EFFECT = 1,
}
impl From<ABORT35_AW> for bool {
    #[inline(always)]
    fn from(variant: ABORT35_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ABORT35` writer - Abort control for DMA channel."]
pub type ABORT35_W<'a, const O: u8> = crate::BitWriter<'a, u32, ABORT1_SPEC, ABORT35_AW, O>;
impl<'a, const O: u8> ABORT35_W<'a, O> {
    #[doc = "No effect."]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(ABORT35_AW::NO_EFFECT)
    }
    #[doc = "Aborts DMA operations on channel."]
    #[inline(always)]
    pub fn effect(self) -> &'a mut W {
        self.variant(ABORT35_AW::EFFECT)
    }
}
#[doc = "Abort control for DMA channel.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ABORT36_AW {
    #[doc = "0: No effect."]
    NO_EFFECT = 0,
    #[doc = "1: Aborts DMA operations on channel."]
    EFFECT = 1,
}
impl From<ABORT36_AW> for bool {
    #[inline(always)]
    fn from(variant: ABORT36_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ABORT36` writer - Abort control for DMA channel."]
pub type ABORT36_W<'a, const O: u8> = crate::BitWriter<'a, u32, ABORT1_SPEC, ABORT36_AW, O>;
impl<'a, const O: u8> ABORT36_W<'a, O> {
    #[doc = "No effect."]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(ABORT36_AW::NO_EFFECT)
    }
    #[doc = "Aborts DMA operations on channel."]
    #[inline(always)]
    pub fn effect(self) -> &'a mut W {
        self.variant(ABORT36_AW::EFFECT)
    }
}
impl W {
    #[doc = "Bit 0 - Abort control for DMA channel."]
    #[inline(always)]
    #[must_use]
    pub fn abort32(&mut self) -> ABORT32_W<0> {
        ABORT32_W::new(self)
    }
    #[doc = "Bit 1 - Abort control for DMA channel."]
    #[inline(always)]
    #[must_use]
    pub fn abort33(&mut self) -> ABORT33_W<1> {
        ABORT33_W::new(self)
    }
    #[doc = "Bit 2 - Abort control for DMA channel."]
    #[inline(always)]
    #[must_use]
    pub fn abort34(&mut self) -> ABORT34_W<2> {
        ABORT34_W::new(self)
    }
    #[doc = "Bit 3 - Abort control for DMA channel."]
    #[inline(always)]
    #[must_use]
    pub fn abort35(&mut self) -> ABORT35_W<3> {
        ABORT35_W::new(self)
    }
    #[doc = "Bit 4 - Abort control for DMA channel."]
    #[inline(always)]
    #[must_use]
    pub fn abort36(&mut self) -> ABORT36_W<4> {
        ABORT36_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Channel Abort control for all DMA channels\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [abort1](index.html) module"]
pub struct ABORT1_SPEC;
impl crate::RegisterSpec for ABORT1_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [abort1::W](W) writer structure"]
impl crate::Writable for ABORT1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ABORT1 to value 0"]
impl crate::Resettable for ABORT1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
