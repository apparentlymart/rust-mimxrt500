#[doc = "Register `CAPCLR` writer"]
pub struct W(crate::W<CAPCLR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CAPCLR_SPEC>;
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
impl From<crate::W<CAPCLR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CAPCLR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Clear capture 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CAPCLR0_AW {
    #[doc = "0: Does nothing"]
    CAPCLR0NOTHING = 0,
    #[doc = "1: Write 1 to clear the CAP0 register value"]
    CAPCLR0CLEARED = 1,
}
impl From<CAPCLR0_AW> for bool {
    #[inline(always)]
    fn from(variant: CAPCLR0_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CAPCLR0` writer - Clear capture 0"]
pub type CAPCLR0_W<'a, const O: u8> = crate::BitWriter<'a, u32, CAPCLR_SPEC, CAPCLR0_AW, O>;
impl<'a, const O: u8> CAPCLR0_W<'a, O> {
    #[doc = "Does nothing"]
    #[inline(always)]
    pub fn capclr0nothing(self) -> &'a mut W {
        self.variant(CAPCLR0_AW::CAPCLR0NOTHING)
    }
    #[doc = "Write 1 to clear the CAP0 register value"]
    #[inline(always)]
    pub fn capclr0cleared(self) -> &'a mut W {
        self.variant(CAPCLR0_AW::CAPCLR0CLEARED)
    }
}
#[doc = "Clear capture 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CAPCLR1_AW {
    #[doc = "0: Does nothing"]
    CAPCLR1NOTHING = 0,
    #[doc = "1: Write 1 to clear the CAP1 register value"]
    CAPCLR1CLEARED = 1,
}
impl From<CAPCLR1_AW> for bool {
    #[inline(always)]
    fn from(variant: CAPCLR1_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CAPCLR1` writer - Clear capture 1"]
pub type CAPCLR1_W<'a, const O: u8> = crate::BitWriter<'a, u32, CAPCLR_SPEC, CAPCLR1_AW, O>;
impl<'a, const O: u8> CAPCLR1_W<'a, O> {
    #[doc = "Does nothing"]
    #[inline(always)]
    pub fn capclr1nothing(self) -> &'a mut W {
        self.variant(CAPCLR1_AW::CAPCLR1NOTHING)
    }
    #[doc = "Write 1 to clear the CAP1 register value"]
    #[inline(always)]
    pub fn capclr1cleared(self) -> &'a mut W {
        self.variant(CAPCLR1_AW::CAPCLR1CLEARED)
    }
}
#[doc = "Clear capture 2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CAPCLR2_AW {
    #[doc = "0: Does nothing"]
    CAPCLR2NOTHING = 0,
    #[doc = "1: Write 1 to clear the CAP2 register value"]
    CAPCLR2CLEARED = 1,
}
impl From<CAPCLR2_AW> for bool {
    #[inline(always)]
    fn from(variant: CAPCLR2_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CAPCLR2` writer - Clear capture 2"]
pub type CAPCLR2_W<'a, const O: u8> = crate::BitWriter<'a, u32, CAPCLR_SPEC, CAPCLR2_AW, O>;
impl<'a, const O: u8> CAPCLR2_W<'a, O> {
    #[doc = "Does nothing"]
    #[inline(always)]
    pub fn capclr2nothing(self) -> &'a mut W {
        self.variant(CAPCLR2_AW::CAPCLR2NOTHING)
    }
    #[doc = "Write 1 to clear the CAP2 register value"]
    #[inline(always)]
    pub fn capclr2cleared(self) -> &'a mut W {
        self.variant(CAPCLR2_AW::CAPCLR2CLEARED)
    }
}
#[doc = "Clear capture 3\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CAPCLR3_AW {
    #[doc = "0: Does nothing"]
    CAPCLR3NOTHING = 0,
    #[doc = "1: Write 1 to clear the CAP3 register value"]
    CAPCLR3CLEARED = 1,
}
impl From<CAPCLR3_AW> for bool {
    #[inline(always)]
    fn from(variant: CAPCLR3_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CAPCLR3` writer - Clear capture 3"]
pub type CAPCLR3_W<'a, const O: u8> = crate::BitWriter<'a, u32, CAPCLR_SPEC, CAPCLR3_AW, O>;
impl<'a, const O: u8> CAPCLR3_W<'a, O> {
    #[doc = "Does nothing"]
    #[inline(always)]
    pub fn capclr3nothing(self) -> &'a mut W {
        self.variant(CAPCLR3_AW::CAPCLR3NOTHING)
    }
    #[doc = "Write 1 to clear the CAP3 register value"]
    #[inline(always)]
    pub fn capclr3cleared(self) -> &'a mut W {
        self.variant(CAPCLR3_AW::CAPCLR3CLEARED)
    }
}
impl W {
    #[doc = "Bit 0 - Clear capture 0"]
    #[inline(always)]
    #[must_use]
    pub fn capclr0(&mut self) -> CAPCLR0_W<0> {
        CAPCLR0_W::new(self)
    }
    #[doc = "Bit 1 - Clear capture 1"]
    #[inline(always)]
    #[must_use]
    pub fn capclr1(&mut self) -> CAPCLR1_W<1> {
        CAPCLR1_W::new(self)
    }
    #[doc = "Bit 2 - Clear capture 2"]
    #[inline(always)]
    #[must_use]
    pub fn capclr2(&mut self) -> CAPCLR2_W<2> {
        CAPCLR2_W::new(self)
    }
    #[doc = "Bit 3 - Clear capture 3"]
    #[inline(always)]
    #[must_use]
    pub fn capclr3(&mut self) -> CAPCLR3_W<3> {
        CAPCLR3_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Capture Clear\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [capclr](index.html) module"]
pub struct CAPCLR_SPEC;
impl crate::RegisterSpec for CAPCLR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [capclr::W](W) writer structure"]
impl crate::Writable for CAPCLR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CAPCLR to value 0"]
impl crate::Resettable for CAPCLR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
