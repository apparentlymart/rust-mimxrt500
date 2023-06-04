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
#[doc = "Field `TXERR` reader - Transmit Error Interrupt Clear"]
pub type TXERR_R = crate::BitReader<TXERR_A>;
#[doc = "Transmit Error Interrupt Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TXERR_A {
    #[doc = "0: Interrupt is not cleared."]
    NOT_CLEAR = 0,
    #[doc = "1: Interrupt is cleared."]
    CLEAR = 1,
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
            false => TXERR_A::NOT_CLEAR,
            true => TXERR_A::CLEAR,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_CLEAR`"]
    #[inline(always)]
    pub fn is_not_clear(&self) -> bool {
        *self == TXERR_A::NOT_CLEAR
    }
    #[doc = "Checks if the value of the field is `CLEAR`"]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == TXERR_A::CLEAR
    }
}
#[doc = "Field `TXERR` writer - Transmit Error Interrupt Clear"]
pub type TXERR_W<'a, const O: u8> = crate::BitWriter<'a, u32, FIFOINTENCLR_SPEC, TXERR_A, O>;
impl<'a, const O: u8> TXERR_W<'a, O> {
    #[doc = "Interrupt is not cleared."]
    #[inline(always)]
    pub fn not_clear(self) -> &'a mut W {
        self.variant(TXERR_A::NOT_CLEAR)
    }
    #[doc = "Interrupt is cleared."]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(TXERR_A::CLEAR)
    }
}
#[doc = "Field `RXERR` reader - Receive Error Interrupt Clear"]
pub type RXERR_R = crate::BitReader<RXERR_A>;
#[doc = "Receive Error Interrupt Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RXERR_A {
    #[doc = "0: Interrupt is not cleared."]
    NOT_CLEAR = 0,
    #[doc = "1: Interrupt is cleared."]
    CLEAR = 1,
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
            false => RXERR_A::NOT_CLEAR,
            true => RXERR_A::CLEAR,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_CLEAR`"]
    #[inline(always)]
    pub fn is_not_clear(&self) -> bool {
        *self == RXERR_A::NOT_CLEAR
    }
    #[doc = "Checks if the value of the field is `CLEAR`"]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == RXERR_A::CLEAR
    }
}
#[doc = "Field `RXERR` writer - Receive Error Interrupt Clear"]
pub type RXERR_W<'a, const O: u8> = crate::BitWriter<'a, u32, FIFOINTENCLR_SPEC, RXERR_A, O>;
impl<'a, const O: u8> RXERR_W<'a, O> {
    #[doc = "Interrupt is not cleared."]
    #[inline(always)]
    pub fn not_clear(self) -> &'a mut W {
        self.variant(RXERR_A::NOT_CLEAR)
    }
    #[doc = "Interrupt is cleared."]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(RXERR_A::CLEAR)
    }
}
#[doc = "Field `TXLVL` reader - Transmit Level Interrupt Clear"]
pub type TXLVL_R = crate::BitReader<TXLVL_A>;
#[doc = "Transmit Level Interrupt Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TXLVL_A {
    #[doc = "0: Interrupt is not cleared."]
    NOT_CLEAR = 0,
    #[doc = "1: Interrupt is cleared."]
    CLEAR = 1,
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
            false => TXLVL_A::NOT_CLEAR,
            true => TXLVL_A::CLEAR,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_CLEAR`"]
    #[inline(always)]
    pub fn is_not_clear(&self) -> bool {
        *self == TXLVL_A::NOT_CLEAR
    }
    #[doc = "Checks if the value of the field is `CLEAR`"]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == TXLVL_A::CLEAR
    }
}
#[doc = "Field `TXLVL` writer - Transmit Level Interrupt Clear"]
pub type TXLVL_W<'a, const O: u8> = crate::BitWriter<'a, u32, FIFOINTENCLR_SPEC, TXLVL_A, O>;
impl<'a, const O: u8> TXLVL_W<'a, O> {
    #[doc = "Interrupt is not cleared."]
    #[inline(always)]
    pub fn not_clear(self) -> &'a mut W {
        self.variant(TXLVL_A::NOT_CLEAR)
    }
    #[doc = "Interrupt is cleared."]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(TXLVL_A::CLEAR)
    }
}
#[doc = "Field `RXLVL` reader - Receive Level Interrupt Clear"]
pub type RXLVL_R = crate::BitReader<RXLVL_A>;
#[doc = "Receive Level Interrupt Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RXLVL_A {
    #[doc = "0: Interrupt is not cleared."]
    NOT_CLEAR = 0,
    #[doc = "1: Interrupt is cleared."]
    CLEAR = 1,
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
            false => RXLVL_A::NOT_CLEAR,
            true => RXLVL_A::CLEAR,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_CLEAR`"]
    #[inline(always)]
    pub fn is_not_clear(&self) -> bool {
        *self == RXLVL_A::NOT_CLEAR
    }
    #[doc = "Checks if the value of the field is `CLEAR`"]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == RXLVL_A::CLEAR
    }
}
#[doc = "Field `RXLVL` writer - Receive Level Interrupt Clear"]
pub type RXLVL_W<'a, const O: u8> = crate::BitWriter<'a, u32, FIFOINTENCLR_SPEC, RXLVL_A, O>;
impl<'a, const O: u8> RXLVL_W<'a, O> {
    #[doc = "Interrupt is not cleared."]
    #[inline(always)]
    pub fn not_clear(self) -> &'a mut W {
        self.variant(RXLVL_A::NOT_CLEAR)
    }
    #[doc = "Interrupt is cleared."]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(RXLVL_A::CLEAR)
    }
}
impl R {
    #[doc = "Bit 0 - Transmit Error Interrupt Clear"]
    #[inline(always)]
    pub fn txerr(&self) -> TXERR_R {
        TXERR_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Receive Error Interrupt Clear"]
    #[inline(always)]
    pub fn rxerr(&self) -> RXERR_R {
        RXERR_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Transmit Level Interrupt Clear"]
    #[inline(always)]
    pub fn txlvl(&self) -> TXLVL_R {
        TXLVL_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Receive Level Interrupt Clear"]
    #[inline(always)]
    pub fn rxlvl(&self) -> RXLVL_R {
        RXLVL_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Transmit Error Interrupt Clear"]
    #[inline(always)]
    #[must_use]
    pub fn txerr(&mut self) -> TXERR_W<0> {
        TXERR_W::new(self)
    }
    #[doc = "Bit 1 - Receive Error Interrupt Clear"]
    #[inline(always)]
    #[must_use]
    pub fn rxerr(&mut self) -> RXERR_W<1> {
        RXERR_W::new(self)
    }
    #[doc = "Bit 2 - Transmit Level Interrupt Clear"]
    #[inline(always)]
    #[must_use]
    pub fn txlvl(&mut self) -> TXLVL_W<2> {
        TXLVL_W::new(self)
    }
    #[doc = "Bit 3 - Receive Level Interrupt Clear"]
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
#[doc = "FIFO Interrupt Enable Clear and Read\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fifointenclr](index.html) module"]
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
