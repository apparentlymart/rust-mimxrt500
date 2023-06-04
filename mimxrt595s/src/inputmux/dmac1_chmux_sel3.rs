#[doc = "Register `DMAC1_CHMUX_SEL3` reader"]
pub struct R(crate::R<DMAC1_CHMUX_SEL3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DMAC1_CHMUX_SEL3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DMAC1_CHMUX_SEL3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DMAC1_CHMUX_SEL3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DMAC1_CHMUX_SEL3` writer"]
pub struct W(crate::W<DMAC1_CHMUX_SEL3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DMAC1_CHMUX_SEL3_SPEC>;
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
impl From<crate::W<DMAC1_CHMUX_SEL3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DMAC1_CHMUX_SEL3_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DMAC1_CHMUX_SEL` reader - DMAC1 Channel mux select"]
pub type DMAC1_CHMUX_SEL_R = crate::BitReader<DMAC1_CHMUX_SEL_A>;
#[doc = "DMAC1 Channel mux select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DMAC1_CHMUX_SEL_A {
    #[doc = "0: DMIC_CH3_DMA"]
    DMAC1_CHMUX_SEL_0 = 0,
    #[doc = "1: FLEXCOM9_TX_DMA"]
    DMAC1_CHMUX_SEL_1 = 1,
}
impl From<DMAC1_CHMUX_SEL_A> for bool {
    #[inline(always)]
    fn from(variant: DMAC1_CHMUX_SEL_A) -> Self {
        variant as u8 != 0
    }
}
impl DMAC1_CHMUX_SEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DMAC1_CHMUX_SEL_A {
        match self.bits {
            false => DMAC1_CHMUX_SEL_A::DMAC1_CHMUX_SEL_0,
            true => DMAC1_CHMUX_SEL_A::DMAC1_CHMUX_SEL_1,
        }
    }
    #[doc = "Checks if the value of the field is `DMAC1_CHMUX_SEL_0`"]
    #[inline(always)]
    pub fn is_dmac1_chmux_sel_0(&self) -> bool {
        *self == DMAC1_CHMUX_SEL_A::DMAC1_CHMUX_SEL_0
    }
    #[doc = "Checks if the value of the field is `DMAC1_CHMUX_SEL_1`"]
    #[inline(always)]
    pub fn is_dmac1_chmux_sel_1(&self) -> bool {
        *self == DMAC1_CHMUX_SEL_A::DMAC1_CHMUX_SEL_1
    }
}
#[doc = "Field `DMAC1_CHMUX_SEL` writer - DMAC1 Channel mux select"]
pub type DMAC1_CHMUX_SEL_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, DMAC1_CHMUX_SEL3_SPEC, DMAC1_CHMUX_SEL_A, O>;
impl<'a, const O: u8> DMAC1_CHMUX_SEL_W<'a, O> {
    #[doc = "DMIC_CH3_DMA"]
    #[inline(always)]
    pub fn dmac1_chmux_sel_0(self) -> &'a mut W {
        self.variant(DMAC1_CHMUX_SEL_A::DMAC1_CHMUX_SEL_0)
    }
    #[doc = "FLEXCOM9_TX_DMA"]
    #[inline(always)]
    pub fn dmac1_chmux_sel_1(self) -> &'a mut W {
        self.variant(DMAC1_CHMUX_SEL_A::DMAC1_CHMUX_SEL_1)
    }
}
impl R {
    #[doc = "Bit 0 - DMAC1 Channel mux select"]
    #[inline(always)]
    pub fn dmac1_chmux_sel(&self) -> DMAC1_CHMUX_SEL_R {
        DMAC1_CHMUX_SEL_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - DMAC1 Channel mux select"]
    #[inline(always)]
    #[must_use]
    pub fn dmac1_chmux_sel(&mut self) -> DMAC1_CHMUX_SEL_W<0> {
        DMAC1_CHMUX_SEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DMAC1 Channel mux select 3\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dmac1_chmux_sel3](index.html) module"]
pub struct DMAC1_CHMUX_SEL3_SPEC;
impl crate::RegisterSpec for DMAC1_CHMUX_SEL3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dmac1_chmux_sel3::R](R) reader structure"]
impl crate::Readable for DMAC1_CHMUX_SEL3_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dmac1_chmux_sel3::W](W) writer structure"]
impl crate::Writable for DMAC1_CHMUX_SEL3_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DMAC1_CHMUX_SEL3 to value 0"]
impl crate::Resettable for DMAC1_CHMUX_SEL3_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
