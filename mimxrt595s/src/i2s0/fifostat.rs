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
    #[doc = "0: No transmit FIFO error occured"]
    NO_ERROR = 0,
    #[doc = "1: Transmit FIFO error occured"]
    ERROR = 1,
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
            false => TXERR_A::NO_ERROR,
            true => TXERR_A::ERROR,
        }
    }
    #[doc = "Checks if the value of the field is `NO_ERROR`"]
    #[inline(always)]
    pub fn is_no_error(&self) -> bool {
        *self == TXERR_A::NO_ERROR
    }
    #[doc = "Checks if the value of the field is `ERROR`"]
    #[inline(always)]
    pub fn is_error(&self) -> bool {
        *self == TXERR_A::ERROR
    }
}
#[doc = "Field `TXERR` writer - TX FIFO Error"]
pub type TXERR_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, FIFOSTAT_SPEC, TXERR_A, O>;
impl<'a, const O: u8> TXERR_W<'a, O> {
    #[doc = "No transmit FIFO error occured"]
    #[inline(always)]
    pub fn no_error(self) -> &'a mut W {
        self.variant(TXERR_A::NO_ERROR)
    }
    #[doc = "Transmit FIFO error occured"]
    #[inline(always)]
    pub fn error(self) -> &'a mut W {
        self.variant(TXERR_A::ERROR)
    }
}
#[doc = "Field `RXERR` reader - RX FIFO Error"]
pub type RXERR_R = crate::BitReader<RXERR_A>;
#[doc = "RX FIFO Error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RXERR_A {
    #[doc = "0: No receive FIFO error occured"]
    NO_ERROR = 0,
    #[doc = "1: Receive FIFO error occured"]
    ERROR = 1,
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
            false => RXERR_A::NO_ERROR,
            true => RXERR_A::ERROR,
        }
    }
    #[doc = "Checks if the value of the field is `NO_ERROR`"]
    #[inline(always)]
    pub fn is_no_error(&self) -> bool {
        *self == RXERR_A::NO_ERROR
    }
    #[doc = "Checks if the value of the field is `ERROR`"]
    #[inline(always)]
    pub fn is_error(&self) -> bool {
        *self == RXERR_A::ERROR
    }
}
#[doc = "Field `RXERR` writer - RX FIFO Error"]
pub type RXERR_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, FIFOSTAT_SPEC, RXERR_A, O>;
impl<'a, const O: u8> RXERR_W<'a, O> {
    #[doc = "No receive FIFO error occured"]
    #[inline(always)]
    pub fn no_error(self) -> &'a mut W {
        self.variant(RXERR_A::NO_ERROR)
    }
    #[doc = "Receive FIFO error occured"]
    #[inline(always)]
    pub fn error(self) -> &'a mut W {
        self.variant(RXERR_A::ERROR)
    }
}
#[doc = "Field `PERINT` reader - Peripheral Interrupt"]
pub type PERINT_R = crate::BitReader<PERINT_A>;
#[doc = "Peripheral Interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PERINT_A {
    #[doc = "0: No interrupt"]
    NOT_ASSERTED = 0,
    #[doc = "1: Interrupt"]
    ASSERTED = 1,
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
            false => PERINT_A::NOT_ASSERTED,
            true => PERINT_A::ASSERTED,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_ASSERTED`"]
    #[inline(always)]
    pub fn is_not_asserted(&self) -> bool {
        *self == PERINT_A::NOT_ASSERTED
    }
    #[doc = "Checks if the value of the field is `ASSERTED`"]
    #[inline(always)]
    pub fn is_asserted(&self) -> bool {
        *self == PERINT_A::ASSERTED
    }
}
#[doc = "Field `TXEMPTY` reader - Transmit FIFO Empty"]
pub type TXEMPTY_R = crate::BitReader<TXEMPTY_A>;
#[doc = "Transmit FIFO Empty\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TXEMPTY_A {
    #[doc = "0: Transmit FIFO is not empty"]
    NOT_EMPTY = 0,
    #[doc = "1: Transmit FIFO is empty; however, the peripheral may still be processing the last piece of data."]
    EMPTY = 1,
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
            false => TXEMPTY_A::NOT_EMPTY,
            true => TXEMPTY_A::EMPTY,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_EMPTY`"]
    #[inline(always)]
    pub fn is_not_empty(&self) -> bool {
        *self == TXEMPTY_A::NOT_EMPTY
    }
    #[doc = "Checks if the value of the field is `EMPTY`"]
    #[inline(always)]
    pub fn is_empty(&self) -> bool {
        *self == TXEMPTY_A::EMPTY
    }
}
#[doc = "Field `TXNOTFULL` reader - Transmit FIFO Not Full"]
pub type TXNOTFULL_R = crate::BitReader<TXNOTFULL_A>;
#[doc = "Transmit FIFO Not Full\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TXNOTFULL_A {
    #[doc = "0: Transmit FIFO is full, and another write would cause an overflow"]
    FULL = 0,
    #[doc = "1: Transmit FIFO is not full, so more data can be written"]
    NOT_FULL = 1,
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
            false => TXNOTFULL_A::FULL,
            true => TXNOTFULL_A::NOT_FULL,
        }
    }
    #[doc = "Checks if the value of the field is `FULL`"]
    #[inline(always)]
    pub fn is_full(&self) -> bool {
        *self == TXNOTFULL_A::FULL
    }
    #[doc = "Checks if the value of the field is `NOT_FULL`"]
    #[inline(always)]
    pub fn is_not_full(&self) -> bool {
        *self == TXNOTFULL_A::NOT_FULL
    }
}
#[doc = "Field `RXNOTEMPTY` reader - Receive FIFO Not Empty"]
pub type RXNOTEMPTY_R = crate::BitReader<RXNOTEMPTY_A>;
#[doc = "Receive FIFO Not Empty\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RXNOTEMPTY_A {
    #[doc = "0: Receive FIFO is empty"]
    EMPTY = 0,
    #[doc = "1: Receive FIFO is not empty, so data can be read."]
    NOT_EMPTY = 1,
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
            false => RXNOTEMPTY_A::EMPTY,
            true => RXNOTEMPTY_A::NOT_EMPTY,
        }
    }
    #[doc = "Checks if the value of the field is `EMPTY`"]
    #[inline(always)]
    pub fn is_empty(&self) -> bool {
        *self == RXNOTEMPTY_A::EMPTY
    }
    #[doc = "Checks if the value of the field is `NOT_EMPTY`"]
    #[inline(always)]
    pub fn is_not_empty(&self) -> bool {
        *self == RXNOTEMPTY_A::NOT_EMPTY
    }
}
#[doc = "Field `RXFULL` reader - Receive FIFO Full"]
pub type RXFULL_R = crate::BitReader<RXFULL_A>;
#[doc = "Receive FIFO Full\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RXFULL_A {
    #[doc = "0: Receive FIFO is not full"]
    NOT_FULL = 0,
    #[doc = "1: Receive FIFO is full"]
    FULL = 1,
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
            false => RXFULL_A::NOT_FULL,
            true => RXFULL_A::FULL,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_FULL`"]
    #[inline(always)]
    pub fn is_not_full(&self) -> bool {
        *self == RXFULL_A::NOT_FULL
    }
    #[doc = "Checks if the value of the field is `FULL`"]
    #[inline(always)]
    pub fn is_full(&self) -> bool {
        *self == RXFULL_A::FULL
    }
}
#[doc = "Field `TXLVL` reader - Transmit FIFO Current Level"]
pub type TXLVL_R = crate::FieldReader<u8, TXLVL_A>;
#[doc = "Transmit FIFO Current Level\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TXLVL_A {
    #[doc = "0: TX FIFO is empty"]
    EMPTY = 0,
}
impl From<TXLVL_A> for u8 {
    #[inline(always)]
    fn from(variant: TXLVL_A) -> Self {
        variant as _
    }
}
impl TXLVL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<TXLVL_A> {
        match self.bits {
            0 => Some(TXLVL_A::EMPTY),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `EMPTY`"]
    #[inline(always)]
    pub fn is_empty(&self) -> bool {
        *self == TXLVL_A::EMPTY
    }
}
#[doc = "Field `RXLVL` reader - Receive FIFO Current Level"]
pub type RXLVL_R = crate::FieldReader<u8, RXLVL_A>;
#[doc = "Receive FIFO Current Level\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum RXLVL_A {
    #[doc = "0: RX FIFO is empty"]
    EMPTY = 0,
}
impl From<RXLVL_A> for u8 {
    #[inline(always)]
    fn from(variant: RXLVL_A) -> Self {
        variant as _
    }
}
impl RXLVL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<RXLVL_A> {
        match self.bits {
            0 => Some(RXLVL_A::EMPTY),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `EMPTY`"]
    #[inline(always)]
    pub fn is_empty(&self) -> bool {
        *self == RXLVL_A::EMPTY
    }
}
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
    #[doc = "Bit 5 - Transmit FIFO Not Full"]
    #[inline(always)]
    pub fn txnotfull(&self) -> TXNOTFULL_R {
        TXNOTFULL_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Receive FIFO Not Empty"]
    #[inline(always)]
    pub fn rxnotempty(&self) -> RXNOTEMPTY_R {
        RXNOTEMPTY_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Receive FIFO Full"]
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
#[doc = "FIFO Status\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fifostat](index.html) module"]
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
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0x03;
}
#[doc = "`reset()` method sets FIFOSTAT to value 0x30"]
impl crate::Resettable for FIFOSTAT_SPEC {
    const RESET_VALUE: Self::Ux = 0x30;
}
