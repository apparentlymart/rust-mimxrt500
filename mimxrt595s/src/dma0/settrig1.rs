#[doc = "Register `SETTRIG1` writer"]
pub struct W(crate::W<SETTRIG1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SETTRIG1_SPEC>;
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
impl From<crate::W<SETTRIG1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SETTRIG1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Set Trigger control bit for DMA channel.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SETTRIG32_AW {
    #[doc = "0: No effect."]
    NO_EFFECT = 0,
    #[doc = "1: Sets the Trig bit for DMA channel."]
    EFFECT = 1,
}
impl From<SETTRIG32_AW> for bool {
    #[inline(always)]
    fn from(variant: SETTRIG32_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SETTRIG32` writer - Set Trigger control bit for DMA channel."]
pub type SETTRIG32_W<'a, const O: u8> = crate::BitWriter<'a, u32, SETTRIG1_SPEC, SETTRIG32_AW, O>;
impl<'a, const O: u8> SETTRIG32_W<'a, O> {
    #[doc = "No effect."]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(SETTRIG32_AW::NO_EFFECT)
    }
    #[doc = "Sets the Trig bit for DMA channel."]
    #[inline(always)]
    pub fn effect(self) -> &'a mut W {
        self.variant(SETTRIG32_AW::EFFECT)
    }
}
#[doc = "Set Trigger control bit for DMA channel.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SETTRIG33_AW {
    #[doc = "0: No effect."]
    NO_EFFECT = 0,
    #[doc = "1: Sets the Trig bit for DMA channel."]
    EFFECT = 1,
}
impl From<SETTRIG33_AW> for bool {
    #[inline(always)]
    fn from(variant: SETTRIG33_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SETTRIG33` writer - Set Trigger control bit for DMA channel."]
pub type SETTRIG33_W<'a, const O: u8> = crate::BitWriter<'a, u32, SETTRIG1_SPEC, SETTRIG33_AW, O>;
impl<'a, const O: u8> SETTRIG33_W<'a, O> {
    #[doc = "No effect."]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(SETTRIG33_AW::NO_EFFECT)
    }
    #[doc = "Sets the Trig bit for DMA channel."]
    #[inline(always)]
    pub fn effect(self) -> &'a mut W {
        self.variant(SETTRIG33_AW::EFFECT)
    }
}
#[doc = "Set Trigger control bit for DMA channel.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SETTRIG34_AW {
    #[doc = "0: No effect."]
    NO_EFFECT = 0,
    #[doc = "1: Sets the Trig bit for DMA channel."]
    EFFECT = 1,
}
impl From<SETTRIG34_AW> for bool {
    #[inline(always)]
    fn from(variant: SETTRIG34_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SETTRIG34` writer - Set Trigger control bit for DMA channel."]
pub type SETTRIG34_W<'a, const O: u8> = crate::BitWriter<'a, u32, SETTRIG1_SPEC, SETTRIG34_AW, O>;
impl<'a, const O: u8> SETTRIG34_W<'a, O> {
    #[doc = "No effect."]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(SETTRIG34_AW::NO_EFFECT)
    }
    #[doc = "Sets the Trig bit for DMA channel."]
    #[inline(always)]
    pub fn effect(self) -> &'a mut W {
        self.variant(SETTRIG34_AW::EFFECT)
    }
}
#[doc = "Set Trigger control bit for DMA channel.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SETTRIG35_AW {
    #[doc = "0: No effect."]
    NO_EFFECT = 0,
    #[doc = "1: Sets the Trig bit for DMA channel."]
    EFFECT = 1,
}
impl From<SETTRIG35_AW> for bool {
    #[inline(always)]
    fn from(variant: SETTRIG35_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SETTRIG35` writer - Set Trigger control bit for DMA channel."]
pub type SETTRIG35_W<'a, const O: u8> = crate::BitWriter<'a, u32, SETTRIG1_SPEC, SETTRIG35_AW, O>;
impl<'a, const O: u8> SETTRIG35_W<'a, O> {
    #[doc = "No effect."]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(SETTRIG35_AW::NO_EFFECT)
    }
    #[doc = "Sets the Trig bit for DMA channel."]
    #[inline(always)]
    pub fn effect(self) -> &'a mut W {
        self.variant(SETTRIG35_AW::EFFECT)
    }
}
#[doc = "Set Trigger control bit for DMA channel.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SETTRIG36_AW {
    #[doc = "0: No effect."]
    NO_EFFECT = 0,
    #[doc = "1: Sets the Trig bit for DMA channel."]
    EFFECT = 1,
}
impl From<SETTRIG36_AW> for bool {
    #[inline(always)]
    fn from(variant: SETTRIG36_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SETTRIG36` writer - Set Trigger control bit for DMA channel."]
pub type SETTRIG36_W<'a, const O: u8> = crate::BitWriter<'a, u32, SETTRIG1_SPEC, SETTRIG36_AW, O>;
impl<'a, const O: u8> SETTRIG36_W<'a, O> {
    #[doc = "No effect."]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(SETTRIG36_AW::NO_EFFECT)
    }
    #[doc = "Sets the Trig bit for DMA channel."]
    #[inline(always)]
    pub fn effect(self) -> &'a mut W {
        self.variant(SETTRIG36_AW::EFFECT)
    }
}
impl W {
    #[doc = "Bit 0 - Set Trigger control bit for DMA channel."]
    #[inline(always)]
    #[must_use]
    pub fn settrig32(&mut self) -> SETTRIG32_W<0> {
        SETTRIG32_W::new(self)
    }
    #[doc = "Bit 1 - Set Trigger control bit for DMA channel."]
    #[inline(always)]
    #[must_use]
    pub fn settrig33(&mut self) -> SETTRIG33_W<1> {
        SETTRIG33_W::new(self)
    }
    #[doc = "Bit 2 - Set Trigger control bit for DMA channel."]
    #[inline(always)]
    #[must_use]
    pub fn settrig34(&mut self) -> SETTRIG34_W<2> {
        SETTRIG34_W::new(self)
    }
    #[doc = "Bit 3 - Set Trigger control bit for DMA channel."]
    #[inline(always)]
    #[must_use]
    pub fn settrig35(&mut self) -> SETTRIG35_W<3> {
        SETTRIG35_W::new(self)
    }
    #[doc = "Bit 4 - Set Trigger control bit for DMA channel."]
    #[inline(always)]
    #[must_use]
    pub fn settrig36(&mut self) -> SETTRIG36_W<4> {
        SETTRIG36_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Set Trigger control bits for all DMA channels\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [settrig1](index.html) module"]
pub struct SETTRIG1_SPEC;
impl crate::RegisterSpec for SETTRIG1_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [settrig1::W](W) writer structure"]
impl crate::Writable for SETTRIG1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SETTRIG1 to value 0"]
impl crate::Resettable for SETTRIG1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
