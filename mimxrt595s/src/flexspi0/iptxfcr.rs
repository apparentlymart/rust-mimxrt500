#[doc = "Register `IPTXFCR` reader"]
pub struct R(crate::R<IPTXFCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IPTXFCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IPTXFCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IPTXFCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IPTXFCR` writer"]
pub struct W(crate::W<IPTXFCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IPTXFCR_SPEC>;
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
impl From<crate::W<IPTXFCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IPTXFCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CLRIPTXF` reader - Clear all valid data entries in IP TX FIFO."]
pub type CLRIPTXF_R = crate::BitReader<CLRIPTXF_A>;
#[doc = "Clear all valid data entries in IP TX FIFO.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CLRIPTXF_A {
    #[doc = "0: No function."]
    VALUE0 = 0,
    #[doc = "1: A clock cycle pulse to clear all valid data entries in IP TX FIFO."]
    VALUE1 = 1,
}
impl From<CLRIPTXF_A> for bool {
    #[inline(always)]
    fn from(variant: CLRIPTXF_A) -> Self {
        variant as u8 != 0
    }
}
impl CLRIPTXF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CLRIPTXF_A {
        match self.bits {
            false => CLRIPTXF_A::VALUE0,
            true => CLRIPTXF_A::VALUE1,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE0`"]
    #[inline(always)]
    pub fn is_value0(&self) -> bool {
        *self == CLRIPTXF_A::VALUE0
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == CLRIPTXF_A::VALUE1
    }
}
#[doc = "Field `CLRIPTXF` writer - Clear all valid data entries in IP TX FIFO."]
pub type CLRIPTXF_W<'a, const O: u8> = crate::BitWriter<'a, u32, IPTXFCR_SPEC, CLRIPTXF_A, O>;
impl<'a, const O: u8> CLRIPTXF_W<'a, O> {
    #[doc = "No function."]
    #[inline(always)]
    pub fn value0(self) -> &'a mut W {
        self.variant(CLRIPTXF_A::VALUE0)
    }
    #[doc = "A clock cycle pulse to clear all valid data entries in IP TX FIFO."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(CLRIPTXF_A::VALUE1)
    }
}
#[doc = "Field `TXDMAEN` reader - IP TX FIFO filling by DMA enabled."]
pub type TXDMAEN_R = crate::BitReader<TXDMAEN_A>;
#[doc = "IP TX FIFO filling by DMA enabled.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TXDMAEN_A {
    #[doc = "0: IP TX FIFO would be filled by processor."]
    VAL0 = 0,
    #[doc = "1: IP TX FIFO would be filled by DMA."]
    VAL1 = 1,
}
impl From<TXDMAEN_A> for bool {
    #[inline(always)]
    fn from(variant: TXDMAEN_A) -> Self {
        variant as u8 != 0
    }
}
impl TXDMAEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TXDMAEN_A {
        match self.bits {
            false => TXDMAEN_A::VAL0,
            true => TXDMAEN_A::VAL1,
        }
    }
    #[doc = "Checks if the value of the field is `VAL0`"]
    #[inline(always)]
    pub fn is_val0(&self) -> bool {
        *self == TXDMAEN_A::VAL0
    }
    #[doc = "Checks if the value of the field is `VAL1`"]
    #[inline(always)]
    pub fn is_val1(&self) -> bool {
        *self == TXDMAEN_A::VAL1
    }
}
#[doc = "Field `TXDMAEN` writer - IP TX FIFO filling by DMA enabled."]
pub type TXDMAEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, IPTXFCR_SPEC, TXDMAEN_A, O>;
impl<'a, const O: u8> TXDMAEN_W<'a, O> {
    #[doc = "IP TX FIFO would be filled by processor."]
    #[inline(always)]
    pub fn val0(self) -> &'a mut W {
        self.variant(TXDMAEN_A::VAL0)
    }
    #[doc = "IP TX FIFO would be filled by DMA."]
    #[inline(always)]
    pub fn val1(self) -> &'a mut W {
        self.variant(TXDMAEN_A::VAL1)
    }
}
#[doc = "Field `TXWMRK` reader - Watermark level is (TXWMRK+1)*64 Bits."]
pub type TXWMRK_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TXWMRK` writer - Watermark level is (TXWMRK+1)*64 Bits."]
pub type TXWMRK_W<'a, const O: u8> = crate::FieldWriter<'a, u32, IPTXFCR_SPEC, u8, u8, 7, O>;
impl R {
    #[doc = "Bit 0 - Clear all valid data entries in IP TX FIFO."]
    #[inline(always)]
    pub fn clriptxf(&self) -> CLRIPTXF_R {
        CLRIPTXF_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - IP TX FIFO filling by DMA enabled."]
    #[inline(always)]
    pub fn txdmaen(&self) -> TXDMAEN_R {
        TXDMAEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:8 - Watermark level is (TXWMRK+1)*64 Bits."]
    #[inline(always)]
    pub fn txwmrk(&self) -> TXWMRK_R {
        TXWMRK_R::new(((self.bits >> 2) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Clear all valid data entries in IP TX FIFO."]
    #[inline(always)]
    #[must_use]
    pub fn clriptxf(&mut self) -> CLRIPTXF_W<0> {
        CLRIPTXF_W::new(self)
    }
    #[doc = "Bit 1 - IP TX FIFO filling by DMA enabled."]
    #[inline(always)]
    #[must_use]
    pub fn txdmaen(&mut self) -> TXDMAEN_W<1> {
        TXDMAEN_W::new(self)
    }
    #[doc = "Bits 2:8 - Watermark level is (TXWMRK+1)*64 Bits."]
    #[inline(always)]
    #[must_use]
    pub fn txwmrk(&mut self) -> TXWMRK_W<2> {
        TXWMRK_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "IP TX FIFO Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [iptxfcr](index.html) module"]
pub struct IPTXFCR_SPEC;
impl crate::RegisterSpec for IPTXFCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [iptxfcr::R](R) reader structure"]
impl crate::Readable for IPTXFCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [iptxfcr::W](W) writer structure"]
impl crate::Writable for IPTXFCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets IPTXFCR to value 0"]
impl crate::Resettable for IPTXFCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
