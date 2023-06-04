#[doc = "Register `INTENCLR` writer"]
pub struct W(crate::W<INTENCLR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INTENCLR_SPEC>;
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
impl From<crate::W<INTENCLR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INTENCLR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Slave Select Assert Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SSAEN_AW {
    #[doc = "0: No effect"]
    NO_EFFECT = 0,
    #[doc = "1: Clear the Slave Select Assert Interrupt Enable bit (INTENSET\\[SSAEN\\])"]
    CLEAR_THE_SSAEN = 1,
}
impl From<SSAEN_AW> for bool {
    #[inline(always)]
    fn from(variant: SSAEN_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SSAEN` writer - Slave Select Assert Interrupt Enable"]
pub type SSAEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTENCLR_SPEC, SSAEN_AW, O>;
impl<'a, const O: u8> SSAEN_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(SSAEN_AW::NO_EFFECT)
    }
    #[doc = "Clear the Slave Select Assert Interrupt Enable bit (INTENSET\\[SSAEN\\])"]
    #[inline(always)]
    pub fn clear_the_ssaen(self) -> &'a mut W {
        self.variant(SSAEN_AW::CLEAR_THE_SSAEN)
    }
}
#[doc = "Slave Select Deassert Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SSDEN_AW {
    #[doc = "0: No effect"]
    NO_EFFECT = 0,
    #[doc = "1: Clear the Slave Select Deassert Interrupt Enable bit (INTENSET\\[SSDEN\\])"]
    CLEAR_THE_SSDEN = 1,
}
impl From<SSDEN_AW> for bool {
    #[inline(always)]
    fn from(variant: SSDEN_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SSDEN` writer - Slave Select Deassert Interrupt Enable"]
pub type SSDEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTENCLR_SPEC, SSDEN_AW, O>;
impl<'a, const O: u8> SSDEN_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(SSDEN_AW::NO_EFFECT)
    }
    #[doc = "Clear the Slave Select Deassert Interrupt Enable bit (INTENSET\\[SSDEN\\])"]
    #[inline(always)]
    pub fn clear_the_ssden(self) -> &'a mut W {
        self.variant(SSDEN_AW::CLEAR_THE_SSDEN)
    }
}
#[doc = "Master Idle Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MSTIDLE_AW {
    #[doc = "0: No effect"]
    NO_EFFECT = 0,
    #[doc = "1: Clear the Master Idle Interrupt Enable bit (INTENSET\\[MSTIDLE\\])"]
    CLEAR_THE_MSTIDLE = 1,
}
impl From<MSTIDLE_AW> for bool {
    #[inline(always)]
    fn from(variant: MSTIDLE_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MSTIDLE` writer - Master Idle Interrupt Enable"]
pub type MSTIDLE_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTENCLR_SPEC, MSTIDLE_AW, O>;
impl<'a, const O: u8> MSTIDLE_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(MSTIDLE_AW::NO_EFFECT)
    }
    #[doc = "Clear the Master Idle Interrupt Enable bit (INTENSET\\[MSTIDLE\\])"]
    #[inline(always)]
    pub fn clear_the_mstidle(self) -> &'a mut W {
        self.variant(MSTIDLE_AW::CLEAR_THE_MSTIDLE)
    }
}
impl W {
    #[doc = "Bit 4 - Slave Select Assert Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ssaen(&mut self) -> SSAEN_W<4> {
        SSAEN_W::new(self)
    }
    #[doc = "Bit 5 - Slave Select Deassert Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ssden(&mut self) -> SSDEN_W<5> {
        SSDEN_W::new(self)
    }
    #[doc = "Bit 8 - Master Idle Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn mstidle(&mut self) -> MSTIDLE_W<8> {
        MSTIDLE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt Enable Clear Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intenclr](index.html) module"]
pub struct INTENCLR_SPEC;
impl crate::RegisterSpec for INTENCLR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [intenclr::W](W) writer structure"]
impl crate::Writable for INTENCLR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets INTENCLR to value 0"]
impl crate::Resettable for INTENCLR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
