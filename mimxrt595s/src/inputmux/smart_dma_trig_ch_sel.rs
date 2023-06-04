#[doc = "Register `SMART_DMA_TRIG_CH_SEL[%s]` reader"]
pub struct R(crate::R<SMART_DMA_TRIG_CH_SEL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SMART_DMA_TRIG_CH_SEL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SMART_DMA_TRIG_CH_SEL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SMART_DMA_TRIG_CH_SEL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SMART_DMA_TRIG_CH_SEL[%s]` writer"]
pub struct W(crate::W<SMART_DMA_TRIG_CH_SEL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SMART_DMA_TRIG_CH_SEL_SPEC>;
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
impl From<crate::W<SMART_DMA_TRIG_CH_SEL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SMART_DMA_TRIG_CH_SEL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SMART_DMA_IN_SEL` reader - SMART_DMA Input(n) Selection"]
pub type SMART_DMA_IN_SEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SMART_DMA_IN_SEL` writer - SMART_DMA Input(n) Selection"]
pub type SMART_DMA_IN_SEL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, SMART_DMA_TRIG_CH_SEL_SPEC, u8, u8, 7, O>;
impl R {
    #[doc = "Bits 0:6 - SMART_DMA Input(n) Selection"]
    #[inline(always)]
    pub fn smart_dma_in_sel(&self) -> SMART_DMA_IN_SEL_R {
        SMART_DMA_IN_SEL_R::new((self.bits & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - SMART_DMA Input(n) Selection"]
    #[inline(always)]
    #[must_use]
    pub fn smart_dma_in_sel(&mut self) -> SMART_DMA_IN_SEL_W<0> {
        SMART_DMA_IN_SEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SMART_DMA trigger channel select\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [smart_dma_trig_ch_sel](index.html) module"]
pub struct SMART_DMA_TRIG_CH_SEL_SPEC;
impl crate::RegisterSpec for SMART_DMA_TRIG_CH_SEL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [smart_dma_trig_ch_sel::R](R) reader structure"]
impl crate::Readable for SMART_DMA_TRIG_CH_SEL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [smart_dma_trig_ch_sel::W](W) writer structure"]
impl crate::Writable for SMART_DMA_TRIG_CH_SEL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SMART_DMA_TRIG_CH_SEL[%s]
to value 0x7f"]
impl crate::Resettable for SMART_DMA_TRIG_CH_SEL_SPEC {
    const RESET_VALUE: Self::Ux = 0x7f;
}
