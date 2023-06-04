#[doc = "Register `FIFOINTSTAT` reader"]
pub struct R(crate::R<FIFOINTSTAT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FIFOINTSTAT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FIFOINTSTAT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FIFOINTSTAT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `TXERR` reader - TX FIFO Error Interrupt Status"]
pub type TXERR_R = crate::BitReader<TXERR_A>;
#[doc = "TX FIFO Error Interrupt Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TXERR_A {
    #[doc = "0: Not pending"]
    TXERR_ISNOTPENDING = 0,
    #[doc = "1: Pending"]
    TXERR_ISPENDING = 1,
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
            false => TXERR_A::TXERR_ISNOTPENDING,
            true => TXERR_A::TXERR_ISPENDING,
        }
    }
    #[doc = "Checks if the value of the field is `TXERR_ISNOTPENDING`"]
    #[inline(always)]
    pub fn is_txerr_isnotpending(&self) -> bool {
        *self == TXERR_A::TXERR_ISNOTPENDING
    }
    #[doc = "Checks if the value of the field is `TXERR_ISPENDING`"]
    #[inline(always)]
    pub fn is_txerr_ispending(&self) -> bool {
        *self == TXERR_A::TXERR_ISPENDING
    }
}
#[doc = "Field `RXERR` reader - RX FIFO Error Interrupt Status"]
pub type RXERR_R = crate::BitReader<RXERR_A>;
#[doc = "RX FIFO Error Interrupt Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RXERR_A {
    #[doc = "0: Not pending"]
    RXERR_ISNOTPENDING = 0,
    #[doc = "1: Pending"]
    RXERR_ISPENDING = 1,
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
            false => RXERR_A::RXERR_ISNOTPENDING,
            true => RXERR_A::RXERR_ISPENDING,
        }
    }
    #[doc = "Checks if the value of the field is `RXERR_ISNOTPENDING`"]
    #[inline(always)]
    pub fn is_rxerr_isnotpending(&self) -> bool {
        *self == RXERR_A::RXERR_ISNOTPENDING
    }
    #[doc = "Checks if the value of the field is `RXERR_ISPENDING`"]
    #[inline(always)]
    pub fn is_rxerr_ispending(&self) -> bool {
        *self == RXERR_A::RXERR_ISPENDING
    }
}
#[doc = "Field `TXLVL` reader - Transmit FIFO Level Interrupt Status"]
pub type TXLVL_R = crate::BitReader<TXLVL_A>;
#[doc = "Transmit FIFO Level Interrupt Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TXLVL_A {
    #[doc = "0: Not pending"]
    TXLVL_ISNOTPENDING = 0,
    #[doc = "1: Pending"]
    TXLVL_ISPENDING = 1,
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
            false => TXLVL_A::TXLVL_ISNOTPENDING,
            true => TXLVL_A::TXLVL_ISPENDING,
        }
    }
    #[doc = "Checks if the value of the field is `TXLVL_ISNOTPENDING`"]
    #[inline(always)]
    pub fn is_txlvl_isnotpending(&self) -> bool {
        *self == TXLVL_A::TXLVL_ISNOTPENDING
    }
    #[doc = "Checks if the value of the field is `TXLVL_ISPENDING`"]
    #[inline(always)]
    pub fn is_txlvl_ispending(&self) -> bool {
        *self == TXLVL_A::TXLVL_ISPENDING
    }
}
#[doc = "Field `RXLVL` reader - Receive FIFO Level Interrupt Status"]
pub type RXLVL_R = crate::BitReader<RXLVL_A>;
#[doc = "Receive FIFO Level Interrupt Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RXLVL_A {
    #[doc = "0: Not pending"]
    RXLVL_ISNOTPENDING = 0,
    #[doc = "1: Pending"]
    RXLVL_ISPENDING = 1,
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
            false => RXLVL_A::RXLVL_ISNOTPENDING,
            true => RXLVL_A::RXLVL_ISPENDING,
        }
    }
    #[doc = "Checks if the value of the field is `RXLVL_ISNOTPENDING`"]
    #[inline(always)]
    pub fn is_rxlvl_isnotpending(&self) -> bool {
        *self == RXLVL_A::RXLVL_ISNOTPENDING
    }
    #[doc = "Checks if the value of the field is `RXLVL_ISPENDING`"]
    #[inline(always)]
    pub fn is_rxlvl_ispending(&self) -> bool {
        *self == RXLVL_A::RXLVL_ISPENDING
    }
}
#[doc = "Field `PERINT` reader - Peripheral Interrupt Status"]
pub type PERINT_R = crate::BitReader<PERINT_A>;
#[doc = "Peripheral Interrupt Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PERINT_A {
    #[doc = "0: Not pending"]
    PERINT_ISNOTPENDING = 0,
    #[doc = "1: Pending"]
    PERINT_ISPENDING = 1,
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
            false => PERINT_A::PERINT_ISNOTPENDING,
            true => PERINT_A::PERINT_ISPENDING,
        }
    }
    #[doc = "Checks if the value of the field is `PERINT_ISNOTPENDING`"]
    #[inline(always)]
    pub fn is_perint_isnotpending(&self) -> bool {
        *self == PERINT_A::PERINT_ISNOTPENDING
    }
    #[doc = "Checks if the value of the field is `PERINT_ISPENDING`"]
    #[inline(always)]
    pub fn is_perint_ispending(&self) -> bool {
        *self == PERINT_A::PERINT_ISPENDING
    }
}
impl R {
    #[doc = "Bit 0 - TX FIFO Error Interrupt Status"]
    #[inline(always)]
    pub fn txerr(&self) -> TXERR_R {
        TXERR_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - RX FIFO Error Interrupt Status"]
    #[inline(always)]
    pub fn rxerr(&self) -> RXERR_R {
        RXERR_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Transmit FIFO Level Interrupt Status"]
    #[inline(always)]
    pub fn txlvl(&self) -> TXLVL_R {
        TXLVL_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Receive FIFO Level Interrupt Status"]
    #[inline(always)]
    pub fn rxlvl(&self) -> RXLVL_R {
        RXLVL_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Peripheral Interrupt Status"]
    #[inline(always)]
    pub fn perint(&self) -> PERINT_R {
        PERINT_R::new(((self.bits >> 4) & 1) != 0)
    }
}
#[doc = "FIFO Interrupt Status\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fifointstat](index.html) module"]
pub struct FIFOINTSTAT_SPEC;
impl crate::RegisterSpec for FIFOINTSTAT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fifointstat::R](R) reader structure"]
impl crate::Readable for FIFOINTSTAT_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets FIFOINTSTAT to value 0"]
impl crate::Resettable for FIFOINTSTAT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
