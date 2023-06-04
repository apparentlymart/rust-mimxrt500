#[doc = "Register `FIFOSTAT` reader"]
pub struct R(crate::R<FIFOSTAT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FIFOSTAT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FIFOSTAT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FIFOSTAT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FIFOSTAT` writer"]
pub struct W(crate::W<FIFOSTAT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FIFOSTAT_SPEC>;
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
impl From<crate::W<FIFOSTAT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FIFOSTAT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TXERR` reader - TX FIFO Error"]
pub type TXERR_R = crate::BitReader<TXERR_A>;
#[doc = "TX FIFO Error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TXERR_A {
    #[doc = "0: A transmit FIFO error has not occurred."]
    NO_TXERR = 0,
    #[doc = "1: A transmit FIFO error has occurred. This error could be an overflow caused by pushing data into a full FIFO, or by an underflow if the FIFO is empty when data is needed."]
    TXERR = 1,
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
            false => TXERR_A::NO_TXERR,
            true => TXERR_A::TXERR,
        }
    }
    #[doc = "Checks if the value of the field is `NO_TXERR`"]
    #[inline(always)]
    pub fn is_no_txerr(&self) -> bool {
        *self == TXERR_A::NO_TXERR
    }
    #[doc = "Checks if the value of the field is `TXERR`"]
    #[inline(always)]
    pub fn is_txerr(&self) -> bool {
        *self == TXERR_A::TXERR
    }
}
#[doc = "Field `TXERR` writer - TX FIFO Error"]
pub type TXERR_W<'a, const O: u8> = crate::BitWriter<'a, u32, FIFOSTAT_SPEC, TXERR_A, O>;
impl<'a, const O: u8> TXERR_W<'a, O> {
    #[doc = "A transmit FIFO error has not occurred."]
    #[inline(always)]
    pub fn no_txerr(self) -> &'a mut W {
        self.variant(TXERR_A::NO_TXERR)
    }
    #[doc = "A transmit FIFO error has occurred. This error could be an overflow caused by pushing data into a full FIFO, or by an underflow if the FIFO is empty when data is needed."]
    #[inline(always)]
    pub fn txerr(self) -> &'a mut W {
        self.variant(TXERR_A::TXERR)
    }
}
#[doc = "Field `RXERR` reader - RX FIFO Error"]
pub type RXERR_R = crate::BitReader<RXERR_A>;
#[doc = "RX FIFO Error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RXERR_A {
    #[doc = "0: A receive FIFO overflow has not occurred"]
    NO_RXERR = 0,
    #[doc = "1: A receive FIFO overflow has occurred, caused by software or DMA not emptying the FIFO fast enough"]
    RXERR = 1,
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
            false => RXERR_A::NO_RXERR,
            true => RXERR_A::RXERR,
        }
    }
    #[doc = "Checks if the value of the field is `NO_RXERR`"]
    #[inline(always)]
    pub fn is_no_rxerr(&self) -> bool {
        *self == RXERR_A::NO_RXERR
    }
    #[doc = "Checks if the value of the field is `RXERR`"]
    #[inline(always)]
    pub fn is_rxerr(&self) -> bool {
        *self == RXERR_A::RXERR
    }
}
#[doc = "Field `RXERR` writer - RX FIFO Error"]
pub type RXERR_W<'a, const O: u8> = crate::BitWriter<'a, u32, FIFOSTAT_SPEC, RXERR_A, O>;
impl<'a, const O: u8> RXERR_W<'a, O> {
    #[doc = "A receive FIFO overflow has not occurred"]
    #[inline(always)]
    pub fn no_rxerr(self) -> &'a mut W {
        self.variant(RXERR_A::NO_RXERR)
    }
    #[doc = "A receive FIFO overflow has occurred, caused by software or DMA not emptying the FIFO fast enough"]
    #[inline(always)]
    pub fn rxerr(self) -> &'a mut W {
        self.variant(RXERR_A::RXERR)
    }
}
#[doc = "Field `PERINT` reader - Peripheral Interrupt"]
pub type PERINT_R = crate::BitReader<PERINT_A>;
#[doc = "Peripheral Interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PERINT_A {
    #[doc = "0: The peripheral function has not asserted an interrupt"]
    NO_PERINT = 0,
    #[doc = "1: Indicates that the peripheral function has asserted an interrupt. More information can be found by reading the peripheral's status register (STAT)."]
    PERINT = 1,
}
impl From<PERINT_A> for bool {
    #[inline(always)]
    fn from(variant: PERINT_A) -> Self {
        variant as u8 != 0
    }
}
impl PERINT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PERINT_A {
        match self.bits {
            false => PERINT_A::NO_PERINT,
            true => PERINT_A::PERINT,
        }
    }
    #[doc = "Checks if the value of the field is `NO_PERINT`"]
    #[inline(always)]
    pub fn is_no_perint(&self) -> bool {
        *self == PERINT_A::NO_PERINT
    }
    #[doc = "Checks if the value of the field is `PERINT`"]
    #[inline(always)]
    pub fn is_perint(&self) -> bool {
        *self == PERINT_A::PERINT
    }
}
#[doc = "Field `TXEMPTY` reader - Transmit FIFO Empty"]
pub type TXEMPTY_R = crate::BitReader<TXEMPTY_A>;
#[doc = "Transmit FIFO Empty\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TXEMPTY_A {
    #[doc = "0: The transmit FIFO is not empty"]
    TXFIFO_ISNOTEMPTY = 0,
    #[doc = "1: The transmit FIFO is empty, although the peripheral may still be processing the last piece of data."]
    TXFIFO_ISEMPTY = 1,
}
impl From<TXEMPTY_A> for bool {
    #[inline(always)]
    fn from(variant: TXEMPTY_A) -> Self {
        variant as u8 != 0
    }
}
impl TXEMPTY_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TXEMPTY_A {
        match self.bits {
            false => TXEMPTY_A::TXFIFO_ISNOTEMPTY,
            true => TXEMPTY_A::TXFIFO_ISEMPTY,
        }
    }
    #[doc = "Checks if the value of the field is `TXFIFO_ISNOTEMPTY`"]
    #[inline(always)]
    pub fn is_txfifo_isnotempty(&self) -> bool {
        *self == TXEMPTY_A::TXFIFO_ISNOTEMPTY
    }
    #[doc = "Checks if the value of the field is `TXFIFO_ISEMPTY`"]
    #[inline(always)]
    pub fn is_txfifo_isempty(&self) -> bool {
        *self == TXEMPTY_A::TXFIFO_ISEMPTY
    }
}
#[doc = "Field `TXNOTFULL` reader - Transmit FIFO is Not Full"]
pub type TXNOTFULL_R = crate::BitReader<TXNOTFULL_A>;
#[doc = "Transmit FIFO is Not Full\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TXNOTFULL_A {
    #[doc = "0: The transmit FIFO is full and another write would cause it to overflow"]
    TXFIFO_ISFULL = 0,
    #[doc = "1: The transmit FIFO is not full, so more data can be written"]
    TXFIFO_ISNOTFULL = 1,
}
impl From<TXNOTFULL_A> for bool {
    #[inline(always)]
    fn from(variant: TXNOTFULL_A) -> Self {
        variant as u8 != 0
    }
}
impl TXNOTFULL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TXNOTFULL_A {
        match self.bits {
            false => TXNOTFULL_A::TXFIFO_ISFULL,
            true => TXNOTFULL_A::TXFIFO_ISNOTFULL,
        }
    }
    #[doc = "Checks if the value of the field is `TXFIFO_ISFULL`"]
    #[inline(always)]
    pub fn is_txfifo_isfull(&self) -> bool {
        *self == TXNOTFULL_A::TXFIFO_ISFULL
    }
    #[doc = "Checks if the value of the field is `TXFIFO_ISNOTFULL`"]
    #[inline(always)]
    pub fn is_txfifo_isnotfull(&self) -> bool {
        *self == TXNOTFULL_A::TXFIFO_ISNOTFULL
    }
}
#[doc = "Field `RXNOTEMPTY` reader - Receive FIFO is Not Empty"]
pub type RXNOTEMPTY_R = crate::BitReader<RXNOTEMPTY_A>;
#[doc = "Receive FIFO is Not Empty\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RXNOTEMPTY_A {
    #[doc = "0: When 0, the receive FIFO is empty"]
    RXFIFO_ISEMPTY = 0,
    #[doc = "1: When 1, the receive FIFO is not empty, so data can be read"]
    RXFIFO_ISNOTEMPTY = 1,
}
impl From<RXNOTEMPTY_A> for bool {
    #[inline(always)]
    fn from(variant: RXNOTEMPTY_A) -> Self {
        variant as u8 != 0
    }
}
impl RXNOTEMPTY_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RXNOTEMPTY_A {
        match self.bits {
            false => RXNOTEMPTY_A::RXFIFO_ISEMPTY,
            true => RXNOTEMPTY_A::RXFIFO_ISNOTEMPTY,
        }
    }
    #[doc = "Checks if the value of the field is `RXFIFO_ISEMPTY`"]
    #[inline(always)]
    pub fn is_rxfifo_isempty(&self) -> bool {
        *self == RXNOTEMPTY_A::RXFIFO_ISEMPTY
    }
    #[doc = "Checks if the value of the field is `RXFIFO_ISNOTEMPTY`"]
    #[inline(always)]
    pub fn is_rxfifo_isnotempty(&self) -> bool {
        *self == RXNOTEMPTY_A::RXFIFO_ISNOTEMPTY
    }
}
#[doc = "Field `RXFULL` reader - Receive FIFO is Full"]
pub type RXFULL_R = crate::BitReader<RXFULL_A>;
#[doc = "Receive FIFO is Full\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RXFULL_A {
    #[doc = "0: The receive FIFO is not full"]
    RXFIFO_ISNOTFULL = 0,
    #[doc = "1: The receive FIFO is full. To prevent the peripheral from causing an overflow, data should be read out."]
    RXFIFO_ISFULL = 1,
}
impl From<RXFULL_A> for bool {
    #[inline(always)]
    fn from(variant: RXFULL_A) -> Self {
        variant as u8 != 0
    }
}
impl RXFULL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RXFULL_A {
        match self.bits {
            false => RXFULL_A::RXFIFO_ISNOTFULL,
            true => RXFULL_A::RXFIFO_ISFULL,
        }
    }
    #[doc = "Checks if the value of the field is `RXFIFO_ISNOTFULL`"]
    #[inline(always)]
    pub fn is_rxfifo_isnotfull(&self) -> bool {
        *self == RXFULL_A::RXFIFO_ISNOTFULL
    }
    #[doc = "Checks if the value of the field is `RXFIFO_ISFULL`"]
    #[inline(always)]
    pub fn is_rxfifo_isfull(&self) -> bool {
        *self == RXFULL_A::RXFIFO_ISFULL
    }
}
#[doc = "Field `TXLVL` reader - Transmit FIFO Current Level"]
pub type TXLVL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RXLVL` reader - Receive FIFO Current Level"]
pub type RXLVL_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bit 0 - TX FIFO Error"]
    #[inline(always)]
    pub fn txerr(&self) -> TXERR_R {
        TXERR_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - RX FIFO Error"]
    #[inline(always)]
    pub fn rxerr(&self) -> RXERR_R {
        RXERR_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 3 - Peripheral Interrupt"]
    #[inline(always)]
    pub fn perint(&self) -> PERINT_R {
        PERINT_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Transmit FIFO Empty"]
    #[inline(always)]
    pub fn txempty(&self) -> TXEMPTY_R {
        TXEMPTY_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Transmit FIFO is Not Full"]
    #[inline(always)]
    pub fn txnotfull(&self) -> TXNOTFULL_R {
        TXNOTFULL_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Receive FIFO is Not Empty"]
    #[inline(always)]
    pub fn rxnotempty(&self) -> RXNOTEMPTY_R {
        RXNOTEMPTY_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Receive FIFO is Full"]
    #[inline(always)]
    pub fn rxfull(&self) -> RXFULL_R {
        RXFULL_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:12 - Transmit FIFO Current Level"]
    #[inline(always)]
    pub fn txlvl(&self) -> TXLVL_R {
        TXLVL_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 16:20 - Receive FIFO Current Level"]
    #[inline(always)]
    pub fn rxlvl(&self) -> RXLVL_R {
        RXLVL_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - TX FIFO Error"]
    #[inline(always)]
    #[must_use]
    pub fn txerr(&mut self) -> TXERR_W<0> {
        TXERR_W::new(self)
    }
    #[doc = "Bit 1 - RX FIFO Error"]
    #[inline(always)]
    #[must_use]
    pub fn rxerr(&mut self) -> RXERR_W<1> {
        RXERR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "FIFO Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fifostat](index.html) module"]
pub struct FIFOSTAT_SPEC;
impl crate::RegisterSpec for FIFOSTAT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fifostat::R](R) reader structure"]
impl crate::Readable for FIFOSTAT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fifostat::W](W) writer structure"]
impl crate::Writable for FIFOSTAT_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FIFOSTAT to value 0x30"]
impl crate::Resettable for FIFOSTAT_SPEC {
    const RESET_VALUE: Self::Ux = 0x30;
}
