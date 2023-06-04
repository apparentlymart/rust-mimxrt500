#[doc = "Register `IPRXFCR` reader"]
pub struct R(crate::R<IPRXFCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IPRXFCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IPRXFCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IPRXFCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IPRXFCR` writer"]
pub struct W(crate::W<IPRXFCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IPRXFCR_SPEC>;
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
impl From<crate::W<IPRXFCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IPRXFCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CLRIPRXF` reader - Clear all valid data entries in IP RX FIFO."]
pub type CLRIPRXF_R = crate::BitReader<CLRIPRXF_A>;
#[doc = "Clear all valid data entries in IP RX FIFO.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CLRIPRXF_A {
    #[doc = "0: No function."]
    VALUE0 = 0,
    #[doc = "1: A clock cycle pulse to clear all valid data entries in IP RX FIFO."]
    VALUE1 = 1,
}
impl From<CLRIPRXF_A> for bool {
    #[inline(always)]
    fn from(variant: CLRIPRXF_A) -> Self {
        variant as u8 != 0
    }
}
impl CLRIPRXF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CLRIPRXF_A {
        match self.bits {
            false => CLRIPRXF_A::VALUE0,
            true => CLRIPRXF_A::VALUE1,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE0`"]
    #[inline(always)]
    pub fn is_value0(&self) -> bool {
        *self == CLRIPRXF_A::VALUE0
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == CLRIPRXF_A::VALUE1
    }
}
#[doc = "Field `CLRIPRXF` writer - Clear all valid data entries in IP RX FIFO."]
pub type CLRIPRXF_W<'a, const O: u8> = crate::BitWriter<'a, u32, IPRXFCR_SPEC, CLRIPRXF_A, O>;
impl<'a, const O: u8> CLRIPRXF_W<'a, O> {
    #[doc = "No function."]
    #[inline(always)]
    pub fn value0(self) -> &'a mut W {
        self.variant(CLRIPRXF_A::VALUE0)
    }
    #[doc = "A clock cycle pulse to clear all valid data entries in IP RX FIFO."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(CLRIPRXF_A::VALUE1)
    }
}
#[doc = "Field `RXDMAEN` reader - IP RX FIFO reading by DMA enabled."]
pub type RXDMAEN_R = crate::BitReader<RXDMAEN_A>;
#[doc = "IP RX FIFO reading by DMA enabled.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RXDMAEN_A {
    #[doc = "0: IP RX FIFO would be read by processor."]
    VAL0 = 0,
    #[doc = "1: IP RX FIFO would be read by DMA."]
    VAL1 = 1,
}
impl From<RXDMAEN_A> for bool {
    #[inline(always)]
    fn from(variant: RXDMAEN_A) -> Self {
        variant as u8 != 0
    }
}
impl RXDMAEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RXDMAEN_A {
        match self.bits {
            false => RXDMAEN_A::VAL0,
            true => RXDMAEN_A::VAL1,
        }
    }
    #[doc = "Checks if the value of the field is `VAL0`"]
    #[inline(always)]
    pub fn is_val0(&self) -> bool {
        *self == RXDMAEN_A::VAL0
    }
    #[doc = "Checks if the value of the field is `VAL1`"]
    #[inline(always)]
    pub fn is_val1(&self) -> bool {
        *self == RXDMAEN_A::VAL1
    }
}
#[doc = "Field `RXDMAEN` writer - IP RX FIFO reading by DMA enabled."]
pub type RXDMAEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, IPRXFCR_SPEC, RXDMAEN_A, O>;
impl<'a, const O: u8> RXDMAEN_W<'a, O> {
    #[doc = "IP RX FIFO would be read by processor."]
    #[inline(always)]
    pub fn val0(self) -> &'a mut W {
        self.variant(RXDMAEN_A::VAL0)
    }
    #[doc = "IP RX FIFO would be read by DMA."]
    #[inline(always)]
    pub fn val1(self) -> &'a mut W {
        self.variant(RXDMAEN_A::VAL1)
    }
}
#[doc = "Field `RXWMRK` reader - Watermark level is (RXWMRK+1)*64 bits."]
pub type RXWMRK_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RXWMRK` writer - Watermark level is (RXWMRK+1)*64 bits."]
pub type RXWMRK_W<'a, const O: u8> = crate::FieldWriter<'a, u32, IPRXFCR_SPEC, u8, u8, 7, O>;
impl R {
    #[doc = "Bit 0 - Clear all valid data entries in IP RX FIFO."]
    #[inline(always)]
    pub fn clriprxf(&self) -> CLRIPRXF_R {
        CLRIPRXF_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - IP RX FIFO reading by DMA enabled."]
    #[inline(always)]
    pub fn rxdmaen(&self) -> RXDMAEN_R {
        RXDMAEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:8 - Watermark level is (RXWMRK+1)*64 bits."]
    #[inline(always)]
    pub fn rxwmrk(&self) -> RXWMRK_R {
        RXWMRK_R::new(((self.bits >> 2) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Clear all valid data entries in IP RX FIFO."]
    #[inline(always)]
    #[must_use]
    pub fn clriprxf(&mut self) -> CLRIPRXF_W<0> {
        CLRIPRXF_W::new(self)
    }
    #[doc = "Bit 1 - IP RX FIFO reading by DMA enabled."]
    #[inline(always)]
    #[must_use]
    pub fn rxdmaen(&mut self) -> RXDMAEN_W<1> {
        RXDMAEN_W::new(self)
    }
    #[doc = "Bits 2:8 - Watermark level is (RXWMRK+1)*64 bits."]
    #[inline(always)]
    #[must_use]
    pub fn rxwmrk(&mut self) -> RXWMRK_W<2> {
        RXWMRK_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "IP RX FIFO Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [iprxfcr](index.html) module"]
pub struct IPRXFCR_SPEC;
impl crate::RegisterSpec for IPRXFCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [iprxfcr::R](R) reader structure"]
impl crate::Readable for IPRXFCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [iprxfcr::W](W) writer structure"]
impl crate::Writable for IPRXFCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets IPRXFCR to value 0"]
impl crate::Resettable for IPRXFCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
