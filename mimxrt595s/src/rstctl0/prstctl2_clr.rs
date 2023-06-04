#[doc = "Register `PRSTCTL2_CLR` writer"]
pub struct W(crate::W<PRSTCTL2_CLR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PRSTCTL2_CLR_SPEC>;
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
impl From<crate::W<PRSTCTL2_CLR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PRSTCTL2_CLR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Micro-tick timer 0 reset clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UTICK0_AW {
    #[doc = "0: No effect"]
    UTICK0_CLR = 0,
    #[doc = "1: Clears the PRSTCTL2 Bit"]
    UTICK0_SET = 1,
}
impl From<UTICK0_AW> for bool {
    #[inline(always)]
    fn from(variant: UTICK0_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `UTICK0` writer - Micro-tick timer 0 reset clear"]
pub type UTICK0_W<'a, const O: u8> = crate::BitWriter<'a, u32, PRSTCTL2_CLR_SPEC, UTICK0_AW, O>;
impl<'a, const O: u8> UTICK0_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn utick0_clr(self) -> &'a mut W {
        self.variant(UTICK0_AW::UTICK0_CLR)
    }
    #[doc = "Clears the PRSTCTL2 Bit"]
    #[inline(always)]
    pub fn utick0_set(self) -> &'a mut W {
        self.variant(UTICK0_AW::UTICK0_SET)
    }
}
#[doc = "WWDT0 reset clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WWDT0_AW {
    #[doc = "0: No effect"]
    WWDT0_CLR = 0,
    #[doc = "1: Clears the PRSTCTL2 Bit"]
    WWDT0_SET = 1,
}
impl From<WWDT0_AW> for bool {
    #[inline(always)]
    fn from(variant: WWDT0_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WWDT0` writer - WWDT0 reset clear"]
pub type WWDT0_W<'a, const O: u8> = crate::BitWriter<'a, u32, PRSTCTL2_CLR_SPEC, WWDT0_AW, O>;
impl<'a, const O: u8> WWDT0_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn wwdt0_clr(self) -> &'a mut W {
        self.variant(WWDT0_AW::WWDT0_CLR)
    }
    #[doc = "Clears the PRSTCTL2 Bit"]
    #[inline(always)]
    pub fn wwdt0_set(self) -> &'a mut W {
        self.variant(WWDT0_AW::WWDT0_SET)
    }
}
impl W {
    #[doc = "Bit 0 - Micro-tick timer 0 reset clear"]
    #[inline(always)]
    #[must_use]
    pub fn utick0(&mut self) -> UTICK0_W<0> {
        UTICK0_W::new(self)
    }
    #[doc = "Bit 1 - WWDT0 reset clear"]
    #[inline(always)]
    #[must_use]
    pub fn wwdt0(&mut self) -> WWDT0_W<1> {
        WWDT0_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Peripheral Reset Control Register 2 CLR\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [prstctl2_clr](index.html) module"]
pub struct PRSTCTL2_CLR_SPEC;
impl crate::RegisterSpec for PRSTCTL2_CLR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [prstctl2_clr::W](W) writer structure"]
impl crate::Writable for PRSTCTL2_CLR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PRSTCTL2_CLR to value 0"]
impl crate::Resettable for PRSTCTL2_CLR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
