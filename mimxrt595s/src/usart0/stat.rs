#[doc = "Register `STAT` reader"]
pub struct R(crate::R<STAT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<STAT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<STAT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<STAT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `STAT` writer"]
pub struct W(crate::W<STAT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<STAT_SPEC>;
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
impl From<crate::W<STAT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<STAT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RXIDLE` reader - Receiver Idle"]
pub type RXIDLE_R = crate::BitReader<RXIDLE_A>;
#[doc = "Receiver Idle\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RXIDLE_A {
    #[doc = "0: The receiver is currently receiving data."]
    RX_ACTIVE = 0,
    #[doc = "1: The receiver is not currently receiving data."]
    RX_IDLE = 1,
}
impl From<RXIDLE_A> for bool {
    #[inline(always)]
    fn from(variant: RXIDLE_A) -> Self {
        variant as u8 != 0
    }
}
impl RXIDLE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RXIDLE_A {
        match self.bits {
            false => RXIDLE_A::RX_ACTIVE,
            true => RXIDLE_A::RX_IDLE,
        }
    }
    #[doc = "Checks if the value of the field is `RX_ACTIVE`"]
    #[inline(always)]
    pub fn is_rx_active(&self) -> bool {
        *self == RXIDLE_A::RX_ACTIVE
    }
    #[doc = "Checks if the value of the field is `RX_IDLE`"]
    #[inline(always)]
    pub fn is_rx_idle(&self) -> bool {
        *self == RXIDLE_A::RX_IDLE
    }
}
#[doc = "Field `TXIDLE` reader - Transmitter Idle"]
pub type TXIDLE_R = crate::BitReader<TXIDLE_A>;
#[doc = "Transmitter Idle\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TXIDLE_A {
    #[doc = "0: The transmitter is currently sending data."]
    TX_ACTIVE = 0,
    #[doc = "1: The transmitter is not currently sending data."]
    TX_IDLE = 1,
}
impl From<TXIDLE_A> for bool {
    #[inline(always)]
    fn from(variant: TXIDLE_A) -> Self {
        variant as u8 != 0
    }
}
impl TXIDLE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TXIDLE_A {
        match self.bits {
            false => TXIDLE_A::TX_ACTIVE,
            true => TXIDLE_A::TX_IDLE,
        }
    }
    #[doc = "Checks if the value of the field is `TX_ACTIVE`"]
    #[inline(always)]
    pub fn is_tx_active(&self) -> bool {
        *self == TXIDLE_A::TX_ACTIVE
    }
    #[doc = "Checks if the value of the field is `TX_IDLE`"]
    #[inline(always)]
    pub fn is_tx_idle(&self) -> bool {
        *self == TXIDLE_A::TX_IDLE
    }
}
#[doc = "Field `CTS` reader - CTS value"]
pub type CTS_R = crate::BitReader<bool>;
#[doc = "Field `DELTACTS` reader - Delta CTS"]
pub type DELTACTS_R = crate::BitReader<bool>;
#[doc = "Field `DELTACTS` writer - Delta CTS"]
pub type DELTACTS_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, STAT_SPEC, bool, O>;
#[doc = "Field `TXDISSTAT` reader - Transmitter Disabled Status Flag"]
pub type TXDISSTAT_R = crate::BitReader<TXDISSTAT_A>;
#[doc = "Transmitter Disabled Status Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TXDISSTAT_A {
    #[doc = "0: Not Idle. Indicates that the USART transmitter is NOT fully idle after being disabled."]
    TX_NOT_IDLE = 0,
    #[doc = "1: Idle. Indicates that the USART transmitter is fully idle after being disabled (CTL\\[TXDIS\\]
= 1)."]
    TX_IDLE = 1,
}
impl From<TXDISSTAT_A> for bool {
    #[inline(always)]
    fn from(variant: TXDISSTAT_A) -> Self {
        variant as u8 != 0
    }
}
impl TXDISSTAT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TXDISSTAT_A {
        match self.bits {
            false => TXDISSTAT_A::TX_NOT_IDLE,
            true => TXDISSTAT_A::TX_IDLE,
        }
    }
    #[doc = "Checks if the value of the field is `TX_NOT_IDLE`"]
    #[inline(always)]
    pub fn is_tx_not_idle(&self) -> bool {
        *self == TXDISSTAT_A::TX_NOT_IDLE
    }
    #[doc = "Checks if the value of the field is `TX_IDLE`"]
    #[inline(always)]
    pub fn is_tx_idle(&self) -> bool {
        *self == TXDISSTAT_A::TX_IDLE
    }
}
#[doc = "Field `RXBRK` reader - Received Break"]
pub type RXBRK_R = crate::BitReader<bool>;
#[doc = "Field `DELTARXBRK` reader - Delta Received Break"]
pub type DELTARXBRK_R = crate::BitReader<bool>;
#[doc = "Field `DELTARXBRK` writer - Delta Received Break"]
pub type DELTARXBRK_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, STAT_SPEC, bool, O>;
#[doc = "Field `START` reader - Start"]
pub type START_R = crate::BitReader<bool>;
#[doc = "Field `START` writer - Start"]
pub type START_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, STAT_SPEC, bool, O>;
#[doc = "Field `FRAMERRINT` reader - Framing Error Interrupt Flag"]
pub type FRAMERRINT_R = crate::BitReader<bool>;
#[doc = "Field `FRAMERRINT` writer - Framing Error Interrupt Flag"]
pub type FRAMERRINT_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, STAT_SPEC, bool, O>;
#[doc = "Field `PARITYERRINT` reader - Parity Error Interrupt Flag"]
pub type PARITYERRINT_R = crate::BitReader<bool>;
#[doc = "Field `PARITYERRINT` writer - Parity Error Interrupt Flag"]
pub type PARITYERRINT_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, STAT_SPEC, bool, O>;
#[doc = "Field `RXNOISEINT` reader - Received Noise Interrupt Flag"]
pub type RXNOISEINT_R = crate::BitReader<bool>;
#[doc = "Field `RXNOISEINT` writer - Received Noise Interrupt Flag"]
pub type RXNOISEINT_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, STAT_SPEC, bool, O>;
#[doc = "Field `ABERR` reader - Auto Baud Error"]
pub type ABERR_R = crate::BitReader<bool>;
#[doc = "Field `ABERR` writer - Auto Baud Error"]
pub type ABERR_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, STAT_SPEC, bool, O>;
impl R {
    #[doc = "Bit 1 - Receiver Idle"]
    #[inline(always)]
    pub fn rxidle(&self) -> RXIDLE_R {
        RXIDLE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 3 - Transmitter Idle"]
    #[inline(always)]
    pub fn txidle(&self) -> TXIDLE_R {
        TXIDLE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - CTS value"]
    #[inline(always)]
    pub fn cts(&self) -> CTS_R {
        CTS_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Delta CTS"]
    #[inline(always)]
    pub fn deltacts(&self) -> DELTACTS_R {
        DELTACTS_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Transmitter Disabled Status Flag"]
    #[inline(always)]
    pub fn txdisstat(&self) -> TXDISSTAT_R {
        TXDISSTAT_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 10 - Received Break"]
    #[inline(always)]
    pub fn rxbrk(&self) -> RXBRK_R {
        RXBRK_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Delta Received Break"]
    #[inline(always)]
    pub fn deltarxbrk(&self) -> DELTARXBRK_R {
        DELTARXBRK_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Start"]
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
    #[doc = "Bit 16 - Auto Baud Error"]
    #[inline(always)]
    pub fn aberr(&self) -> ABERR_R {
        ABERR_R::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 5 - Delta CTS"]
    #[inline(always)]
    #[must_use]
    pub fn deltacts(&mut self) -> DELTACTS_W<5> {
        DELTACTS_W::new(self)
    }
    #[doc = "Bit 11 - Delta Received Break"]
    #[inline(always)]
    #[must_use]
    pub fn deltarxbrk(&mut self) -> DELTARXBRK_W<11> {
        DELTARXBRK_W::new(self)
    }
    #[doc = "Bit 12 - Start"]
    #[inline(always)]
    #[must_use]
    pub fn start(&mut self) -> START_W<12> {
        START_W::new(self)
    }
    #[doc = "Bit 13 - Framing Error Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn framerrint(&mut self) -> FRAMERRINT_W<13> {
        FRAMERRINT_W::new(self)
    }
    #[doc = "Bit 14 - Parity Error Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn parityerrint(&mut self) -> PARITYERRINT_W<14> {
        PARITYERRINT_W::new(self)
    }
    #[doc = "Bit 15 - Received Noise Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn rxnoiseint(&mut self) -> RXNOISEINT_W<15> {
        RXNOISEINT_W::new(self)
    }
    #[doc = "Bit 16 - Auto Baud Error"]
    #[inline(always)]
    #[must_use]
    pub fn aberr(&mut self) -> ABERR_W<16> {
        ABERR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "USART Status\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [stat](index.html) module"]
pub struct STAT_SPEC;
impl crate::RegisterSpec for STAT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [stat::R](R) reader structure"]
impl crate::Readable for STAT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [stat::W](W) writer structure"]
impl crate::Writable for STAT_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0x0001_f820;
}
#[doc = "`reset()` method sets STAT to value 0x0a"]
impl crate::Resettable for STAT_SPEC {
    const RESET_VALUE: Self::Ux = 0x0a;
}
