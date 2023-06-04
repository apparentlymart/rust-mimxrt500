#[doc = "Register `FIFOINTENCLR` reader"]
pub struct R(crate::R<FIFOINTENCLR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FIFOINTENCLR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FIFOINTENCLR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FIFOINTENCLR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FIFOINTENCLR` writer"]
pub struct W(crate::W<FIFOINTENCLR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FIFOINTENCLR_SPEC>;
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
impl From<crate::W<FIFOINTENCLR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FIFOINTENCLR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TXERR` reader - TX Error Interrupt Enable"]
pub type TXERR_R = crate::BitReader<TXERR_A>;
#[doc = "TX Error Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TXERR_A {
    #[doc = "0: No effect"]
    NO_EFFECT = 0,
    #[doc = "1: Clear the TX Error Interrupt Enable bit FIFOINTENSET\\[TXERR\\]"]
    CLEAR_THE_TXERR = 1,
}
impl From<TXERR_A> for bool {
    #[inline(always)]
    fn from(variant: TXERR_A) -> Self {
        variant as u8 != 0
    }
}
impl TXERR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TXERR_A {
        match self.bits {
            false => TXERR_A::NO_EFFECT,
            true => TXERR_A::CLEAR_THE_TXERR,
        }
    }
    #[doc = "Checks if the value of the field is `NO_EFFECT`"]
    #[inline(always)]
    pub fn is_no_effect(&self) -> bool {
        *self == TXERR_A::NO_EFFECT
    }
    #[doc = "Checks if the value of the field is `CLEAR_THE_TXERR`"]
    #[inline(always)]
    pub fn is_clear_the_txerr(&self) -> bool {
        *self == TXERR_A::CLEAR_THE_TXERR
    }
}
#[doc = "Field `TXERR` writer - TX Error Interrupt Enable"]
pub type TXERR_W<'a, const O: u8> = crate::BitWriter<'a, u32, FIFOINTENCLR_SPEC, TXERR_A, O>;
impl<'a, const O: u8> TXERR_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(TXERR_A::NO_EFFECT)
    }
    #[doc = "Clear the TX Error Interrupt Enable bit FIFOINTENSET\\[TXERR\\]"]
    #[inline(always)]
    pub fn clear_the_txerr(self) -> &'a mut W {
        self.variant(TXERR_A::CLEAR_THE_TXERR)
    }
}
#[doc = "Field `RXERR` reader - Receive Error Interrupt Enable"]
pub type RXERR_R = crate::BitReader<RXERR_A>;
#[doc = "Receive Error Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RXERR_A {
    #[doc = "0: No effect"]
    NO_EFFECT = 0,
    #[doc = "1: Clear the Receive Error Interrupt Enable bit FIFOINTENSET\\[RXERR\\]"]
    CLEAR_THE_RXERR = 1,
}
impl From<RXERR_A> for bool {
    #[inline(always)]
    fn from(variant: RXERR_A) -> Self {
        variant as u8 != 0
    }
}
impl RXERR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RXERR_A {
        match self.bits {
            false => RXERR_A::NO_EFFECT,
            true => RXERR_A::CLEAR_THE_RXERR,
        }
    }
    #[doc = "Checks if the value of the field is `NO_EFFECT`"]
    #[inline(always)]
    pub fn is_no_effect(&self) -> bool {
        *self == RXERR_A::NO_EFFECT
    }
    #[doc = "Checks if the value of the field is `CLEAR_THE_RXERR`"]
    #[inline(always)]
    pub fn is_clear_the_rxerr(&self) -> bool {
        *self == RXERR_A::CLEAR_THE_RXERR
    }
}
#[doc = "Field `RXERR` writer - Receive Error Interrupt Enable"]
pub type RXERR_W<'a, const O: u8> = crate::BitWriter<'a, u32, FIFOINTENCLR_SPEC, RXERR_A, O>;
impl<'a, const O: u8> RXERR_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(RXERR_A::NO_EFFECT)
    }
    #[doc = "Clear the Receive Error Interrupt Enable bit FIFOINTENSET\\[RXERR\\]"]
    #[inline(always)]
    pub fn clear_the_rxerr(self) -> &'a mut W {
        self.variant(RXERR_A::CLEAR_THE_RXERR)
    }
}
#[doc = "Field `TXLVL` reader - Transmit FIFO Level Interrupt Enable"]
pub type TXLVL_R = crate::BitReader<TXLVL_A>;
#[doc = "Transmit FIFO Level Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TXLVL_A {
    #[doc = "0: No effect"]
    NO_EFFECT = 0,
    #[doc = "1: Clear the Transmit FIFO Level Interrupt Enable bit FIFOINTENSET\\[TXLVL\\]"]
    CLEAR_THE_TXLVL = 1,
}
impl From<TXLVL_A> for bool {
    #[inline(always)]
    fn from(variant: TXLVL_A) -> Self {
        variant as u8 != 0
    }
}
impl TXLVL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TXLVL_A {
        match self.bits {
            false => TXLVL_A::NO_EFFECT,
            true => TXLVL_A::CLEAR_THE_TXLVL,
        }
    }
    #[doc = "Checks if the value of the field is `NO_EFFECT`"]
    #[inline(always)]
    pub fn is_no_effect(&self) -> bool {
        *self == TXLVL_A::NO_EFFECT
    }
    #[doc = "Checks if the value of the field is `CLEAR_THE_TXLVL`"]
    #[inline(always)]
    pub fn is_clear_the_txlvl(&self) -> bool {
        *self == TXLVL_A::CLEAR_THE_TXLVL
    }
}
#[doc = "Field `TXLVL` writer - Transmit FIFO Level Interrupt Enable"]
pub type TXLVL_W<'a, const O: u8> = crate::BitWriter<'a, u32, FIFOINTENCLR_SPEC, TXLVL_A, O>;
impl<'a, const O: u8> TXLVL_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(TXLVL_A::NO_EFFECT)
    }
    #[doc = "Clear the Transmit FIFO Level Interrupt Enable bit FIFOINTENSET\\[TXLVL\\]"]
    #[inline(always)]
    pub fn clear_the_txlvl(self) -> &'a mut W {
        self.variant(TXLVL_A::CLEAR_THE_TXLVL)
    }
}
#[doc = "Field `RXLVL` reader - Receive FIFO Level Interrupt Enable"]
pub type RXLVL_R = crate::BitReader<RXLVL_A>;
#[doc = "Receive FIFO Level Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RXLVL_A {
    #[doc = "0: No effect"]
    NO_EFFECT = 0,
    #[doc = "1: Clear the Receive FIFO Level Interrupt Enable bit FIFOINTENSET\\[RXLVL\\]"]
    CLEAR_THE_RXLVL = 1,
}
impl From<RXLVL_A> for bool {
    #[inline(always)]
    fn from(variant: RXLVL_A) -> Self {
        variant as u8 != 0
    }
}
impl RXLVL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RXLVL_A {
        match self.bits {
            false => RXLVL_A::NO_EFFECT,
            true => RXLVL_A::CLEAR_THE_RXLVL,
        }
    }
    #[doc = "Checks if the value of the field is `NO_EFFECT`"]
    #[inline(always)]
    pub fn is_no_effect(&self) -> bool {
        *self == RXLVL_A::NO_EFFECT
    }
    #[doc = "Checks if the value of the field is `CLEAR_THE_RXLVL`"]
    #[inline(always)]
    pub fn is_clear_the_rxlvl(&self) -> bool {
        *self == RXLVL_A::CLEAR_THE_RXLVL
    }
}
#[doc = "Field `RXLVL` writer - Receive FIFO Level Interrupt Enable"]
pub type RXLVL_W<'a, const O: u8> = crate::BitWriter<'a, u32, FIFOINTENCLR_SPEC, RXLVL_A, O>;
impl<'a, const O: u8> RXLVL_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(RXLVL_A::NO_EFFECT)
    }
    #[doc = "Clear the Receive FIFO Level Interrupt Enable bit FIFOINTENSET\\[RXLVL\\]"]
    #[inline(always)]
    pub fn clear_the_rxlvl(self) -> &'a mut W {
        self.variant(RXLVL_A::CLEAR_THE_RXLVL)
    }
}
impl R {
    #[doc = "Bit 0 - TX Error Interrupt Enable"]
    #[inline(always)]
    pub fn txerr(&self) -> TXERR_R {
        TXERR_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Receive Error Interrupt Enable"]
    #[inline(always)]
    pub fn rxerr(&self) -> RXERR_R {
        RXERR_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Transmit FIFO Level Interrupt Enable"]
    #[inline(always)]
    pub fn txlvl(&self) -> TXLVL_R {
        TXLVL_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Receive FIFO Level Interrupt Enable"]
    #[inline(always)]
    pub fn rxlvl(&self) -> RXLVL_R {
        RXLVL_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - TX Error Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn txerr(&mut self) -> TXERR_W<0> {
        TXERR_W::new(self)
    }
    #[doc = "Bit 1 - Receive Error Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn rxerr(&mut self) -> RXERR_W<1> {
        RXERR_W::new(self)
    }
    #[doc = "Bit 2 - Transmit FIFO Level Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn txlvl(&mut self) -> TXLVL_W<2> {
        TXLVL_W::new(self)
    }
    #[doc = "Bit 3 - Receive FIFO Level Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn rxlvl(&mut self) -> RXLVL_W<3> {
        RXLVL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "FIFO Interrupt Enable Clear Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fifointenclr](index.html) module"]
pub struct FIFOINTENCLR_SPEC;
impl crate::RegisterSpec for FIFOINTENCLR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fifointenclr::R](R) reader structure"]
impl crate::Readable for FIFOINTENCLR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fifointenclr::W](W) writer structure"]
impl crate::Writable for FIFOINTENCLR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FIFOINTENCLR to value 0"]
impl crate::Resettable for FIFOINTENCLR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
