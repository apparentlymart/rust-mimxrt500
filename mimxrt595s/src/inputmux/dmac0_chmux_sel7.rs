#[doc = "Register `DMAC0_CHMUX_SEL7` reader"]
pub struct R(crate::R<DMAC0_CHMUX_SEL7_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DMAC0_CHMUX_SEL7_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DMAC0_CHMUX_SEL7_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DMAC0_CHMUX_SEL7_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DMAC0_CHMUX_SEL7` writer"]
pub struct W(crate::W<DMAC0_CHMUX_SEL7_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DMAC0_CHMUX_SEL7_SPEC>;
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
impl From<crate::W<DMAC0_CHMUX_SEL7_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DMAC0_CHMUX_SEL7_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DMAC0_CHMUX_SEL` reader - DMAC0 Channel mux select"]
pub type DMAC0_CHMUX_SEL_R = crate::BitReader<DMAC0_CHMUX_SEL_A>;
#[doc = "DMAC0 Channel mux select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DMAC0_CHMUX_SEL_A {
    #[doc = "0: DMIC_CH7_DMA"]
    DMAC0_CHMUX_SEL_0 = 0,
    #[doc = "1: FLEXCOM13_TX_DMA"]
    DMAC0_CHMUX_SEL_1 = 1,
}
impl From<DMAC0_CHMUX_SEL_A> for bool {
    #[inline(always)]
    fn from(variant: DMAC0_CHMUX_SEL_A) -> Self {
        variant as u8 != 0
    }
}
impl DMAC0_CHMUX_SEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DMAC0_CHMUX_SEL_A {
        match self.bits {
            false => DMAC0_CHMUX_SEL_A::DMAC0_CHMUX_SEL_0,
            true => DMAC0_CHMUX_SEL_A::DMAC0_CHMUX_SEL_1,
        }
    }
    #[doc = "Checks if the value of the field is `DMAC0_CHMUX_SEL_0`"]
    #[inline(always)]
    pub fn is_dmac0_chmux_sel_0(&self) -> bool {
        *self == DMAC0_CHMUX_SEL_A::DMAC0_CHMUX_SEL_0
    }
    #[doc = "Checks if the value of the field is `DMAC0_CHMUX_SEL_1`"]
    #[inline(always)]
    pub fn is_dmac0_chmux_sel_1(&self) -> bool {
        *self == DMAC0_CHMUX_SEL_A::DMAC0_CHMUX_SEL_1
    }
}
#[doc = "Field `DMAC0_CHMUX_SEL` writer - DMAC0 Channel mux select"]
pub type DMAC0_CHMUX_SEL_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, DMAC0_CHMUX_SEL7_SPEC, DMAC0_CHMUX_SEL_A, O>;
impl<'a, const O: u8> DMAC0_CHMUX_SEL_W<'a, O> {
    #[doc = "DMIC_CH7_DMA"]
    #[inline(always)]
    pub fn dmac0_chmux_sel_0(self) -> &'a mut W {
        self.variant(DMAC0_CHMUX_SEL_A::DMAC0_CHMUX_SEL_0)
    }
    #[doc = "FLEXCOM13_TX_DMA"]
    #[inline(always)]
    pub fn dmac0_chmux_sel_1(self) -> &'a mut W {
        self.variant(DMAC0_CHMUX_SEL_A::DMAC0_CHMUX_SEL_1)
    }
}
impl R {
    #[doc = "Bit 0 - DMAC0 Channel mux select"]
    #[inline(always)]
    pub fn dmac0_chmux_sel(&self) -> DMAC0_CHMUX_SEL_R {
        DMAC0_CHMUX_SEL_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - DMAC0 Channel mux select"]
    #[inline(always)]
    #[must_use]
    pub fn dmac0_chmux_sel(&mut self) -> DMAC0_CHMUX_SEL_W<0> {
        DMAC0_CHMUX_SEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DMAC0 Channel mux select 7\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dmac0_chmux_sel7](index.html) module"]
pub struct DMAC0_CHMUX_SEL7_SPEC;
impl crate::RegisterSpec for DMAC0_CHMUX_SEL7_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dmac0_chmux_sel7::R](R) reader structure"]
impl crate::Readable for DMAC0_CHMUX_SEL7_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dmac0_chmux_sel7::W](W) writer structure"]
impl crate::Writable for DMAC0_CHMUX_SEL7_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DMAC0_CHMUX_SEL7 to value 0"]
impl crate::Resettable for DMAC0_CHMUX_SEL7_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
