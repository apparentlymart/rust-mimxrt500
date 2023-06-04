#[doc = "Register `SETVALID1` writer"]
pub struct W(crate::W<SETVALID1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SETVALID1_SPEC>;
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
impl From<crate::W<SETVALID1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SETVALID1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "SetValid control for DMA channel.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SETVALID32_AW {
    #[doc = "0: No effect."]
    NO_EFFECT = 0,
    #[doc = "1: Sets the ValidPending control bit for DMA channel."]
    EFFECT = 1,
}
impl From<SETVALID32_AW> for bool {
    #[inline(always)]
    fn from(variant: SETVALID32_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SETVALID32` writer - SetValid control for DMA channel."]
pub type SETVALID32_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SETVALID1_SPEC, SETVALID32_AW, O>;
impl<'a, const O: u8> SETVALID32_W<'a, O> {
    #[doc = "No effect."]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(SETVALID32_AW::NO_EFFECT)
    }
    #[doc = "Sets the ValidPending control bit for DMA channel."]
    #[inline(always)]
    pub fn effect(self) -> &'a mut W {
        self.variant(SETVALID32_AW::EFFECT)
    }
}
#[doc = "SetValid control for DMA channel.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SETVALID33_AW {
    #[doc = "0: No effect."]
    NO_EFFECT = 0,
    #[doc = "1: Sets the ValidPending control bit for DMA channel."]
    EFFECT = 1,
}
impl From<SETVALID33_AW> for bool {
    #[inline(always)]
    fn from(variant: SETVALID33_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SETVALID33` writer - SetValid control for DMA channel."]
pub type SETVALID33_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SETVALID1_SPEC, SETVALID33_AW, O>;
impl<'a, const O: u8> SETVALID33_W<'a, O> {
    #[doc = "No effect."]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(SETVALID33_AW::NO_EFFECT)
    }
    #[doc = "Sets the ValidPending control bit for DMA channel."]
    #[inline(always)]
    pub fn effect(self) -> &'a mut W {
        self.variant(SETVALID33_AW::EFFECT)
    }
}
#[doc = "SetValid control for DMA channel.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SETVALID34_AW {
    #[doc = "0: No effect."]
    NO_EFFECT = 0,
    #[doc = "1: Sets the ValidPending control bit for DMA channel."]
    EFFECT = 1,
}
impl From<SETVALID34_AW> for bool {
    #[inline(always)]
    fn from(variant: SETVALID34_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SETVALID34` writer - SetValid control for DMA channel."]
pub type SETVALID34_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SETVALID1_SPEC, SETVALID34_AW, O>;
impl<'a, const O: u8> SETVALID34_W<'a, O> {
    #[doc = "No effect."]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(SETVALID34_AW::NO_EFFECT)
    }
    #[doc = "Sets the ValidPending control bit for DMA channel."]
    #[inline(always)]
    pub fn effect(self) -> &'a mut W {
        self.variant(SETVALID34_AW::EFFECT)
    }
}
#[doc = "SetValid control for DMA channel.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SETVALID35_AW {
    #[doc = "0: No effect."]
    NO_EFFECT = 0,
    #[doc = "1: Sets the ValidPending control bit for DMA channel."]
    EFFECT = 1,
}
impl From<SETVALID35_AW> for bool {
    #[inline(always)]
    fn from(variant: SETVALID35_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SETVALID35` writer - SetValid control for DMA channel."]
pub type SETVALID35_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SETVALID1_SPEC, SETVALID35_AW, O>;
impl<'a, const O: u8> SETVALID35_W<'a, O> {
    #[doc = "No effect."]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(SETVALID35_AW::NO_EFFECT)
    }
    #[doc = "Sets the ValidPending control bit for DMA channel."]
    #[inline(always)]
    pub fn effect(self) -> &'a mut W {
        self.variant(SETVALID35_AW::EFFECT)
    }
}
#[doc = "SetValid control for DMA channel.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SETVALID36_AW {
    #[doc = "0: No effect."]
    NO_EFFECT = 0,
    #[doc = "1: Sets the ValidPending control bit for DMA channel."]
    EFFECT = 1,
}
impl From<SETVALID36_AW> for bool {
    #[inline(always)]
    fn from(variant: SETVALID36_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SETVALID36` writer - SetValid control for DMA channel."]
pub type SETVALID36_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SETVALID1_SPEC, SETVALID36_AW, O>;
impl<'a, const O: u8> SETVALID36_W<'a, O> {
    #[doc = "No effect."]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(SETVALID36_AW::NO_EFFECT)
    }
    #[doc = "Sets the ValidPending control bit for DMA channel."]
    #[inline(always)]
    pub fn effect(self) -> &'a mut W {
        self.variant(SETVALID36_AW::EFFECT)
    }
}
impl W {
    #[doc = "Bit 0 - SetValid control for DMA channel."]
    #[inline(always)]
    #[must_use]
    pub fn setvalid32(&mut self) -> SETVALID32_W<0> {
        SETVALID32_W::new(self)
    }
    #[doc = "Bit 1 - SetValid control for DMA channel."]
    #[inline(always)]
    #[must_use]
    pub fn setvalid33(&mut self) -> SETVALID33_W<1> {
        SETVALID33_W::new(self)
    }
    #[doc = "Bit 2 - SetValid control for DMA channel."]
    #[inline(always)]
    #[must_use]
    pub fn setvalid34(&mut self) -> SETVALID34_W<2> {
        SETVALID34_W::new(self)
    }
    #[doc = "Bit 3 - SetValid control for DMA channel."]
    #[inline(always)]
    #[must_use]
    pub fn setvalid35(&mut self) -> SETVALID35_W<3> {
        SETVALID35_W::new(self)
    }
    #[doc = "Bit 4 - SetValid control for DMA channel."]
    #[inline(always)]
    #[must_use]
    pub fn setvalid36(&mut self) -> SETVALID36_W<4> {
        SETVALID36_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Set ValidPending control bits for all DMA channels\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [setvalid1](index.html) module"]
pub struct SETVALID1_SPEC;
impl crate::RegisterSpec for SETVALID1_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [setvalid1::W](W) writer structure"]
impl crate::Writable for SETVALID1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SETVALID1 to value 0"]
impl crate::Resettable for SETVALID1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
