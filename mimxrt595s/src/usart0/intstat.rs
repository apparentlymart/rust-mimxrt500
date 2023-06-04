#[doc = "Register `INTSTAT` reader"]
pub struct R(crate::R<INTSTAT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INTSTAT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INTSTAT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INTSTAT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `TXIDLE` reader - Transmitter Idle Flag"]
pub type TXIDLE_R = crate::BitReader<bool>;
#[doc = "Field `DELTACTS` reader - Delta CTS Change Flag"]
pub type DELTACTS_R = crate::BitReader<bool>;
#[doc = "Field `TXDISINT` reader - Transmitter Disabled Interrupt Flag"]
pub type TXDISINT_R = crate::BitReader<bool>;
#[doc = "Field `DELTARXBRK` reader - Delta Receiver Break Change Flag"]
pub type DELTARXBRK_R = crate::BitReader<bool>;
#[doc = "Field `START` reader - Start Detected on Receiver Flag"]
pub type START_R = crate::BitReader<bool>;
#[doc = "Field `FRAMERRINT` reader - Framing Error Interrupt Flag"]
pub type FRAMERRINT_R = crate::BitReader<bool>;
#[doc = "Field `PARITYERRINT` reader - Parity Error Interrupt Flag"]
pub type PARITYERRINT_R = crate::BitReader<bool>;
#[doc = "Field `RXNOISEINT` reader - Received Noise Interrupt Flag"]
pub type RXNOISEINT_R = crate::BitReader<bool>;
#[doc = "Field `ABERRINT` reader - Auto Baud Error Interrupt Flag"]
pub type ABERRINT_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 3 - Transmitter Idle Flag"]
    #[inline(always)]
    pub fn txidle(&self) -> TXIDLE_R {
        TXIDLE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 5 - Delta CTS Change Flag"]
    #[inline(always)]
    pub fn deltacts(&self) -> DELTACTS_R {
        DELTACTS_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Transmitter Disabled Interrupt Flag"]
    #[inline(always)]
    pub fn txdisint(&self) -> TXDISINT_R {
        TXDISINT_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 11 - Delta Receiver Break Change Flag"]
    #[inline(always)]
    pub fn deltarxbrk(&self) -> DELTARXBRK_R {
        DELTARXBRK_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Start Detected on Receiver Flag"]
    #[inline(always)]
    pub fn start(&self) -> START_R {
        START_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Framing Error Interrupt Flag"]
    #[inline(always)]
    pub fn framerrint(&self) -> FRAMERRINT_R {
        FRAMERRINT_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Parity Error Interrupt Flag"]
    #[inline(always)]
    pub fn parityerrint(&self) -> PARITYERRINT_R {
        PARITYERRINT_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Received Noise Interrupt Flag"]
    #[inline(always)]
    pub fn rxnoiseint(&self) -> RXNOISEINT_R {
        RXNOISEINT_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Auto Baud Error Interrupt Flag"]
    #[inline(always)]
    pub fn aberrint(&self) -> ABERRINT_R {
        ABERRINT_R::new(((self.bits >> 16) & 1) != 0)
    }
}
#[doc = "Interrupt Status\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intstat](index.html) module"]
pub struct INTSTAT_SPEC;
impl crate::RegisterSpec for INTSTAT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [intstat::R](R) reader structure"]
impl crate::Readable for INTSTAT_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets INTSTAT to value 0"]
impl crate::Resettable for INTSTAT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
