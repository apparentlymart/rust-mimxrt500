#[doc = "Register `INTENSET` reader"]
pub struct R(crate::R<INTENSET_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INTENSET_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INTENSET_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INTENSET_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `INTENSET` writer"]
pub struct W(crate::W<INTENSET_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INTENSET_SPEC>;
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
impl From<crate::W<INTENSET_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INTENSET_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TXIDLEEN` reader - Transmit Idle Flag"]
pub type TXIDLEEN_R = crate::BitReader<TXIDLEEN_A>;
#[doc = "Transmit Idle Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TXIDLEEN_A {
    #[doc = "1: Enables an interrupt when the transmitter becomes idle (STAT\\[TXIDLE\\]
= 1)."]
    ENABLE = 1,
}
impl From<TXIDLEEN_A> for bool {
    #[inline(always)]
    fn from(variant: TXIDLEEN_A) -> Self {
        variant as u8 != 0
    }
}
impl TXIDLEEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<TXIDLEEN_A> {
        match self.bits {
            true => Some(TXIDLEEN_A::ENABLE),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == TXIDLEEN_A::ENABLE
    }
}
#[doc = "Field `TXIDLEEN` writer - Transmit Idle Flag"]
pub type TXIDLEEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTENSET_SPEC, TXIDLEEN_A, O>;
impl<'a, const O: u8> TXIDLEEN_W<'a, O> {
    #[doc = "Enables an interrupt when the transmitter becomes idle (STAT\\[TXIDLE\\]
= 1)."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(TXIDLEEN_A::ENABLE)
    }
}
#[doc = "Field `DELTACTSEN` reader - Delta CTS Input Flag"]
pub type DELTACTSEN_R = crate::BitReader<DELTACTSEN_A>;
#[doc = "Delta CTS Input Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DELTACTSEN_A {
    #[doc = "1: Enables an interrupt when there is a change in the state of the CTS input."]
    ENABLE = 1,
}
impl From<DELTACTSEN_A> for bool {
    #[inline(always)]
    fn from(variant: DELTACTSEN_A) -> Self {
        variant as u8 != 0
    }
}
impl DELTACTSEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<DELTACTSEN_A> {
        match self.bits {
            true => Some(DELTACTSEN_A::ENABLE),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == DELTACTSEN_A::ENABLE
    }
}
#[doc = "Field `DELTACTSEN` writer - Delta CTS Input Flag"]
pub type DELTACTSEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTENSET_SPEC, DELTACTSEN_A, O>;
impl<'a, const O: u8> DELTACTSEN_W<'a, O> {
    #[doc = "Enables an interrupt when there is a change in the state of the CTS input."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(DELTACTSEN_A::ENABLE)
    }
}
#[doc = "Field `TXDISEN` reader - Transmit Disabled Flag"]
pub type TXDISEN_R = crate::BitReader<TXDISEN_A>;
#[doc = "Transmit Disabled Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TXDISEN_A {
    #[doc = "1: Enables an interrupt when the transmitter is fully disabled as indicated by the STAT\\[TXDISINT\\]
flag. See the description of the STAT\\[TXDISINT\\]
flag."]
    ENABLE = 1,
}
impl From<TXDISEN_A> for bool {
    #[inline(always)]
    fn from(variant: TXDISEN_A) -> Self {
        variant as u8 != 0
    }
}
impl TXDISEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<TXDISEN_A> {
        match self.bits {
            true => Some(TXDISEN_A::ENABLE),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == TXDISEN_A::ENABLE
    }
}
#[doc = "Field `TXDISEN` writer - Transmit Disabled Flag"]
pub type TXDISEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTENSET_SPEC, TXDISEN_A, O>;
impl<'a, const O: u8> TXDISEN_W<'a, O> {
    #[doc = "Enables an interrupt when the transmitter is fully disabled as indicated by the STAT\\[TXDISINT\\]
flag. See the description of the STAT\\[TXDISINT\\]
flag."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(TXDISEN_A::ENABLE)
    }
}
#[doc = "Field `DELTARXBRKEN` reader - Delta Receive Break Enable"]
pub type DELTARXBRKEN_R = crate::BitReader<DELTARXBRKEN_A>;
#[doc = "Delta Receive Break Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DELTARXBRKEN_A {
    #[doc = "1: Enable"]
    ENABLE = 1,
}
impl From<DELTARXBRKEN_A> for bool {
    #[inline(always)]
    fn from(variant: DELTARXBRKEN_A) -> Self {
        variant as u8 != 0
    }
}
impl DELTARXBRKEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<DELTARXBRKEN_A> {
        match self.bits {
            true => Some(DELTARXBRKEN_A::ENABLE),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == DELTARXBRKEN_A::ENABLE
    }
}
#[doc = "Field `DELTARXBRKEN` writer - Delta Receive Break Enable"]
pub type DELTARXBRKEN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, INTENSET_SPEC, DELTARXBRKEN_A, O>;
impl<'a, const O: u8> DELTARXBRKEN_W<'a, O> {
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(DELTARXBRKEN_A::ENABLE)
    }
}
#[doc = "Field `STARTEN` reader - Start Enable"]
pub type STARTEN_R = crate::BitReader<STARTEN_A>;
#[doc = "Start Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum STARTEN_A {
    #[doc = "1: Enables an interrupt when a received start bit has been detected."]
    ENABLE = 1,
}
impl From<STARTEN_A> for bool {
    #[inline(always)]
    fn from(variant: STARTEN_A) -> Self {
        variant as u8 != 0
    }
}
impl STARTEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<STARTEN_A> {
        match self.bits {
            true => Some(STARTEN_A::ENABLE),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == STARTEN_A::ENABLE
    }
}
#[doc = "Field `STARTEN` writer - Start Enable"]
pub type STARTEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTENSET_SPEC, STARTEN_A, O>;
impl<'a, const O: u8> STARTEN_W<'a, O> {
    #[doc = "Enables an interrupt when a received start bit has been detected."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(STARTEN_A::ENABLE)
    }
}
#[doc = "Field `FRAMERREN` reader - Frame Error Enable"]
pub type FRAMERREN_R = crate::BitReader<FRAMERREN_A>;
#[doc = "Frame Error Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FRAMERREN_A {
    #[doc = "1: Enables an interrupt when a framing error has been detected."]
    ENABLE = 1,
}
impl From<FRAMERREN_A> for bool {
    #[inline(always)]
    fn from(variant: FRAMERREN_A) -> Self {
        variant as u8 != 0
    }
}
impl FRAMERREN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<FRAMERREN_A> {
        match self.bits {
            true => Some(FRAMERREN_A::ENABLE),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == FRAMERREN_A::ENABLE
    }
}
#[doc = "Field `FRAMERREN` writer - Frame Error Enable"]
pub type FRAMERREN_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTENSET_SPEC, FRAMERREN_A, O>;
impl<'a, const O: u8> FRAMERREN_W<'a, O> {
    #[doc = "Enables an interrupt when a framing error has been detected."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(FRAMERREN_A::ENABLE)
    }
}
#[doc = "Field `PARITYERREN` reader - Parity Error Enble"]
pub type PARITYERREN_R = crate::BitReader<PARITYERREN_A>;
#[doc = "Parity Error Enble\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PARITYERREN_A {
    #[doc = "1: Enables an interrupt when a parity error has been detected."]
    ENABLE = 1,
}
impl From<PARITYERREN_A> for bool {
    #[inline(always)]
    fn from(variant: PARITYERREN_A) -> Self {
        variant as u8 != 0
    }
}
impl PARITYERREN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<PARITYERREN_A> {
        match self.bits {
            true => Some(PARITYERREN_A::ENABLE),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == PARITYERREN_A::ENABLE
    }
}
#[doc = "Field `PARITYERREN` writer - Parity Error Enble"]
pub type PARITYERREN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, INTENSET_SPEC, PARITYERREN_A, O>;
impl<'a, const O: u8> PARITYERREN_W<'a, O> {
    #[doc = "Enables an interrupt when a parity error has been detected."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(PARITYERREN_A::ENABLE)
    }
}
#[doc = "Field `RXNOISEEN` reader - Receive Noise Enable"]
pub type RXNOISEEN_R = crate::BitReader<RXNOISEEN_A>;
#[doc = "Receive Noise Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RXNOISEEN_A {
    #[doc = "1: Enables an interrupt when noise is detected. See the description of the CTL\\[RXNOISEINT\\]
bit."]
    ENABLE = 1,
}
impl From<RXNOISEEN_A> for bool {
    #[inline(always)]
    fn from(variant: RXNOISEEN_A) -> Self {
        variant as u8 != 0
    }
}
impl RXNOISEEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<RXNOISEEN_A> {
        match self.bits {
            true => Some(RXNOISEEN_A::ENABLE),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == RXNOISEEN_A::ENABLE
    }
}
#[doc = "Field `RXNOISEEN` writer - Receive Noise Enable"]
pub type RXNOISEEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTENSET_SPEC, RXNOISEEN_A, O>;
impl<'a, const O: u8> RXNOISEEN_W<'a, O> {
    #[doc = "Enables an interrupt when noise is detected. See the description of the CTL\\[RXNOISEINT\\]
bit."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(RXNOISEEN_A::ENABLE)
    }
}
#[doc = "Field `ABERREN` reader - Auto Baud Error Enable"]
pub type ABERREN_R = crate::BitReader<ABERREN_A>;
#[doc = "Auto Baud Error Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ABERREN_A {
    #[doc = "1: Enables an interrupt when an auto baud error occurs."]
    ENABLE = 1,
}
impl From<ABERREN_A> for bool {
    #[inline(always)]
    fn from(variant: ABERREN_A) -> Self {
        variant as u8 != 0
    }
}
impl ABERREN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<ABERREN_A> {
        match self.bits {
            true => Some(ABERREN_A::ENABLE),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == ABERREN_A::ENABLE
    }
}
#[doc = "Field `ABERREN` writer - Auto Baud Error Enable"]
pub type ABERREN_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTENSET_SPEC, ABERREN_A, O>;
impl<'a, const O: u8> ABERREN_W<'a, O> {
    #[doc = "Enables an interrupt when an auto baud error occurs."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(ABERREN_A::ENABLE)
    }
}
impl R {
    #[doc = "Bit 3 - Transmit Idle Flag"]
    #[inline(always)]
    pub fn txidleen(&self) -> TXIDLEEN_R {
        TXIDLEEN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 5 - Delta CTS Input Flag"]
    #[inline(always)]
    pub fn deltactsen(&self) -> DELTACTSEN_R {
        DELTACTSEN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Transmit Disabled Flag"]
    #[inline(always)]
    pub fn txdisen(&self) -> TXDISEN_R {
        TXDISEN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 11 - Delta Receive Break Enable"]
    #[inline(always)]
    pub fn deltarxbrken(&self) -> DELTARXBRKEN_R {
        DELTARXBRKEN_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Start Enable"]
    #[inline(always)]
    pub fn starten(&self) -> STARTEN_R {
        STARTEN_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Frame Error Enable"]
    #[inline(always)]
    pub fn framerren(&self) -> FRAMERREN_R {
        FRAMERREN_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Parity Error Enble"]
    #[inline(always)]
    pub fn parityerren(&self) -> PARITYERREN_R {
        PARITYERREN_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Receive Noise Enable"]
    #[inline(always)]
    pub fn rxnoiseen(&self) -> RXNOISEEN_R {
        RXNOISEEN_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Auto Baud Error Enable"]
    #[inline(always)]
    pub fn aberren(&self) -> ABERREN_R {
        ABERREN_R::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 3 - Transmit Idle Flag"]
    #[inline(always)]
    #[must_use]
    pub fn txidleen(&mut self) -> TXIDLEEN_W<3> {
        TXIDLEEN_W::new(self)
    }
    #[doc = "Bit 5 - Delta CTS Input Flag"]
    #[inline(always)]
    #[must_use]
    pub fn deltactsen(&mut self) -> DELTACTSEN_W<5> {
        DELTACTSEN_W::new(self)
    }
    #[doc = "Bit 6 - Transmit Disabled Flag"]
    #[inline(always)]
    #[must_use]
    pub fn txdisen(&mut self) -> TXDISEN_W<6> {
        TXDISEN_W::new(self)
    }
    #[doc = "Bit 11 - Delta Receive Break Enable"]
    #[inline(always)]
    #[must_use]
    pub fn deltarxbrken(&mut self) -> DELTARXBRKEN_W<11> {
        DELTARXBRKEN_W::new(self)
    }
    #[doc = "Bit 12 - Start Enable"]
    #[inline(always)]
    #[must_use]
    pub fn starten(&mut self) -> STARTEN_W<12> {
        STARTEN_W::new(self)
    }
    #[doc = "Bit 13 - Frame Error Enable"]
    #[inline(always)]
    #[must_use]
    pub fn framerren(&mut self) -> FRAMERREN_W<13> {
        FRAMERREN_W::new(self)
    }
    #[doc = "Bit 14 - Parity Error Enble"]
    #[inline(always)]
    #[must_use]
    pub fn parityerren(&mut self) -> PARITYERREN_W<14> {
        PARITYERREN_W::new(self)
    }
    #[doc = "Bit 15 - Receive Noise Enable"]
    #[inline(always)]
    #[must_use]
    pub fn rxnoiseen(&mut self) -> RXNOISEEN_W<15> {
        RXNOISEEN_W::new(self)
    }
    #[doc = "Bit 16 - Auto Baud Error Enable"]
    #[inline(always)]
    #[must_use]
    pub fn aberren(&mut self) -> ABERREN_W<16> {
        ABERREN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt Enable Read and Set for USART (not FIFO) Status\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intenset](index.html) module"]
pub struct INTENSET_SPEC;
impl crate::RegisterSpec for INTENSET_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [intenset::R](R) reader structure"]
impl crate::Readable for INTENSET_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [intenset::W](W) writer structure"]
impl crate::Writable for INTENSET_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets INTENSET to value 0"]
impl crate::Resettable for INTENSET_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
