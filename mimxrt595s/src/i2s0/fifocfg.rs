#[doc = "Register `FIFOCFG` reader"]
pub struct R(crate::R<FIFOCFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FIFOCFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FIFOCFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FIFOCFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FIFOCFG` writer"]
pub struct W(crate::W<FIFOCFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FIFOCFG_SPEC>;
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
impl From<crate::W<FIFOCFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FIFOCFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ENABLETX` reader - Enable Transmit FIFO"]
pub type ENABLETX_R = crate::BitReader<ENABLETX_A>;
#[doc = "Enable Transmit FIFO\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ENABLETX_A {
    #[doc = "0: Disabled Transmit. The transmit FIFO is not enabled."]
    DISABLED = 0,
    #[doc = "1: Enabled transmit. The transmit FIFO is enabled."]
    ENABLED = 1,
}
impl From<ENABLETX_A> for bool {
    #[inline(always)]
    fn from(variant: ENABLETX_A) -> Self {
        variant as u8 != 0
    }
}
impl ENABLETX_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ENABLETX_A {
        match self.bits {
            false => ENABLETX_A::DISABLED,
            true => ENABLETX_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ENABLETX_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ENABLETX_A::ENABLED
    }
}
#[doc = "Field `ENABLETX` writer - Enable Transmit FIFO"]
pub type ENABLETX_W<'a, const O: u8> = crate::BitWriter<'a, u32, FIFOCFG_SPEC, ENABLETX_A, O>;
impl<'a, const O: u8> ENABLETX_W<'a, O> {
    #[doc = "Disabled Transmit. The transmit FIFO is not enabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(ENABLETX_A::DISABLED)
    }
    #[doc = "Enabled transmit. The transmit FIFO is enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(ENABLETX_A::ENABLED)
    }
}
#[doc = "Field `ENABLERX` reader - Enable Receive FIFO"]
pub type ENABLERX_R = crate::BitReader<ENABLERX_A>;
#[doc = "Enable Receive FIFO\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ENABLERX_A {
    #[doc = "0: Disabled. The receive FIFO is not enabled."]
    DISABLED = 0,
    #[doc = "1: Enabled. The receive FIFO is enabled."]
    ENABLED = 1,
}
impl From<ENABLERX_A> for bool {
    #[inline(always)]
    fn from(variant: ENABLERX_A) -> Self {
        variant as u8 != 0
    }
}
impl ENABLERX_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ENABLERX_A {
        match self.bits {
            false => ENABLERX_A::DISABLED,
            true => ENABLERX_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ENABLERX_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ENABLERX_A::ENABLED
    }
}
#[doc = "Field `ENABLERX` writer - Enable Receive FIFO"]
pub type ENABLERX_W<'a, const O: u8> = crate::BitWriter<'a, u32, FIFOCFG_SPEC, ENABLERX_A, O>;
impl<'a, const O: u8> ENABLERX_W<'a, O> {
    #[doc = "Disabled. The receive FIFO is not enabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(ENABLERX_A::DISABLED)
    }
    #[doc = "Enabled. The receive FIFO is enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(ENABLERX_A::ENABLED)
    }
}
#[doc = "Field `TXI2SE0` reader - Transmit I2S Empty 0"]
pub type TXI2SE0_R = crate::BitReader<TXI2SE0_A>;
#[doc = "Transmit I2S Empty 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TXI2SE0_A {
    #[doc = "0: Last value"]
    LAST_VALUE = 0,
    #[doc = "1: Zero"]
    ZERO = 1,
}
impl From<TXI2SE0_A> for bool {
    #[inline(always)]
    fn from(variant: TXI2SE0_A) -> Self {
        variant as u8 != 0
    }
}
impl TXI2SE0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TXI2SE0_A {
        match self.bits {
            false => TXI2SE0_A::LAST_VALUE,
            true => TXI2SE0_A::ZERO,
        }
    }
    #[doc = "Checks if the value of the field is `LAST_VALUE`"]
    #[inline(always)]
    pub fn is_last_value(&self) -> bool {
        *self == TXI2SE0_A::LAST_VALUE
    }
    #[doc = "Checks if the value of the field is `ZERO`"]
    #[inline(always)]
    pub fn is_zero(&self) -> bool {
        *self == TXI2SE0_A::ZERO
    }
}
#[doc = "Field `TXI2SE0` writer - Transmit I2S Empty 0"]
pub type TXI2SE0_W<'a, const O: u8> = crate::BitWriter<'a, u32, FIFOCFG_SPEC, TXI2SE0_A, O>;
impl<'a, const O: u8> TXI2SE0_W<'a, O> {
    #[doc = "Last value"]
    #[inline(always)]
    pub fn last_value(self) -> &'a mut W {
        self.variant(TXI2SE0_A::LAST_VALUE)
    }
    #[doc = "Zero"]
    #[inline(always)]
    pub fn zero(self) -> &'a mut W {
        self.variant(TXI2SE0_A::ZERO)
    }
}
#[doc = "Field `PACK48` reader - Packing Format 48-bit data"]
pub type PACK48_R = crate::BitReader<PACK48_A>;
#[doc = "Packing Format 48-bit data\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PACK48_A {
    #[doc = "0: Bits_24"]
    BIT_24 = 0,
    #[doc = "1: Bits_32_16"]
    BIT_32_16 = 1,
}
impl From<PACK48_A> for bool {
    #[inline(always)]
    fn from(variant: PACK48_A) -> Self {
        variant as u8 != 0
    }
}
impl PACK48_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PACK48_A {
        match self.bits {
            false => PACK48_A::BIT_24,
            true => PACK48_A::BIT_32_16,
        }
    }
    #[doc = "Checks if the value of the field is `BIT_24`"]
    #[inline(always)]
    pub fn is_bit_24(&self) -> bool {
        *self == PACK48_A::BIT_24
    }
    #[doc = "Checks if the value of the field is `BIT_32_16`"]
    #[inline(always)]
    pub fn is_bit_32_16(&self) -> bool {
        *self == PACK48_A::BIT_32_16
    }
}
#[doc = "Field `PACK48` writer - Packing Format 48-bit data"]
pub type PACK48_W<'a, const O: u8> = crate::BitWriter<'a, u32, FIFOCFG_SPEC, PACK48_A, O>;
impl<'a, const O: u8> PACK48_W<'a, O> {
    #[doc = "Bits_24"]
    #[inline(always)]
    pub fn bit_24(self) -> &'a mut W {
        self.variant(PACK48_A::BIT_24)
    }
    #[doc = "Bits_32_16"]
    #[inline(always)]
    pub fn bit_32_16(self) -> &'a mut W {
        self.variant(PACK48_A::BIT_32_16)
    }
}
#[doc = "Field `SIZE` reader - FIFO Size Configuration"]
pub type SIZE_R = crate::FieldReader<u8, SIZE_A>;
#[doc = "FIFO Size Configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SIZE_A {
    #[doc = "2: Size 32 Bits"]
    BITS32 = 2,
    #[doc = "3: Size 48 Bits"]
    BITS48 = 3,
}
impl From<SIZE_A> for u8 {
    #[inline(always)]
    fn from(variant: SIZE_A) -> Self {
        variant as _
    }
}
impl SIZE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<SIZE_A> {
        match self.bits {
            2 => Some(SIZE_A::BITS32),
            3 => Some(SIZE_A::BITS48),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `BITS32`"]
    #[inline(always)]
    pub fn is_bits32(&self) -> bool {
        *self == SIZE_A::BITS32
    }
    #[doc = "Checks if the value of the field is `BITS48`"]
    #[inline(always)]
    pub fn is_bits48(&self) -> bool {
        *self == SIZE_A::BITS48
    }
}
#[doc = "Field `DMATX` reader - DMA Transmit"]
pub type DMATX_R = crate::BitReader<DMATX_A>;
#[doc = "DMA Transmit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DMATX_A {
    #[doc = "0: Disabled"]
    DISABLED = 0,
    #[doc = "1: Enabled"]
    ENABLED = 1,
}
impl From<DMATX_A> for bool {
    #[inline(always)]
    fn from(variant: DMATX_A) -> Self {
        variant as u8 != 0
    }
}
impl DMATX_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DMATX_A {
        match self.bits {
            false => DMATX_A::DISABLED,
            true => DMATX_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == DMATX_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == DMATX_A::ENABLED
    }
}
#[doc = "Field `DMATX` writer - DMA Transmit"]
pub type DMATX_W<'a, const O: u8> = crate::BitWriter<'a, u32, FIFOCFG_SPEC, DMATX_A, O>;
impl<'a, const O: u8> DMATX_W<'a, O> {
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(DMATX_A::DISABLED)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(DMATX_A::ENABLED)
    }
}
#[doc = "Field `DMARX` reader - DMA Receive"]
pub type DMARX_R = crate::BitReader<DMARX_A>;
#[doc = "DMA Receive\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DMARX_A {
    #[doc = "0: Disabled"]
    DISABLED = 0,
    #[doc = "1: Enabled"]
    ENABLED = 1,
}
impl From<DMARX_A> for bool {
    #[inline(always)]
    fn from(variant: DMARX_A) -> Self {
        variant as u8 != 0
    }
}
impl DMARX_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DMARX_A {
        match self.bits {
            false => DMARX_A::DISABLED,
            true => DMARX_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == DMARX_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == DMARX_A::ENABLED
    }
}
#[doc = "Field `DMARX` writer - DMA Receive"]
pub type DMARX_W<'a, const O: u8> = crate::BitWriter<'a, u32, FIFOCFG_SPEC, DMARX_A, O>;
impl<'a, const O: u8> DMARX_W<'a, O> {
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(DMARX_A::DISABLED)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(DMARX_A::ENABLED)
    }
}
#[doc = "Field `WAKETX` reader - Wake-up for Transmit FIFO Level"]
pub type WAKETX_R = crate::BitReader<WAKETX_A>;
#[doc = "Wake-up for Transmit FIFO Level\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WAKETX_A {
    #[doc = "0: Disabled"]
    DISABLED = 0,
    #[doc = "1: Enabled"]
    ENABLED = 1,
}
impl From<WAKETX_A> for bool {
    #[inline(always)]
    fn from(variant: WAKETX_A) -> Self {
        variant as u8 != 0
    }
}
impl WAKETX_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WAKETX_A {
        match self.bits {
            false => WAKETX_A::DISABLED,
            true => WAKETX_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == WAKETX_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == WAKETX_A::ENABLED
    }
}
#[doc = "Field `WAKETX` writer - Wake-up for Transmit FIFO Level"]
pub type WAKETX_W<'a, const O: u8> = crate::BitWriter<'a, u32, FIFOCFG_SPEC, WAKETX_A, O>;
impl<'a, const O: u8> WAKETX_W<'a, O> {
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(WAKETX_A::DISABLED)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(WAKETX_A::ENABLED)
    }
}
#[doc = "Field `WAKERX` reader - Wake-up for Receive FIFO Level"]
pub type WAKERX_R = crate::BitReader<WAKERX_A>;
#[doc = "Wake-up for Receive FIFO Level\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WAKERX_A {
    #[doc = "0: Only enabled interrupts wake up the device from reduced power modes."]
    DISABLED = 0,
    #[doc = "1: A device wake-up for DMA occurs if the receive FIFO level reaches the value specified by FIFOTRIG\\[RXLVL\\], even when the RXLVL interrupt is not enabled."]
    ENABLED = 1,
}
impl From<WAKERX_A> for bool {
    #[inline(always)]
    fn from(variant: WAKERX_A) -> Self {
        variant as u8 != 0
    }
}
impl WAKERX_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WAKERX_A {
        match self.bits {
            false => WAKERX_A::DISABLED,
            true => WAKERX_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == WAKERX_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == WAKERX_A::ENABLED
    }
}
#[doc = "Field `WAKERX` writer - Wake-up for Receive FIFO Level"]
pub type WAKERX_W<'a, const O: u8> = crate::BitWriter<'a, u32, FIFOCFG_SPEC, WAKERX_A, O>;
impl<'a, const O: u8> WAKERX_W<'a, O> {
    #[doc = "Only enabled interrupts wake up the device from reduced power modes."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(WAKERX_A::DISABLED)
    }
    #[doc = "A device wake-up for DMA occurs if the receive FIFO level reaches the value specified by FIFOTRIG\\[RXLVL\\], even when the RXLVL interrupt is not enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(WAKERX_A::ENABLED)
    }
}
#[doc = "Field `EMPTYTX` writer - Empty command for the transmit FIFO. When a 1 is written to this bit, the TX FIFO is emptied."]
pub type EMPTYTX_W<'a, const O: u8> = crate::BitWriter<'a, u32, FIFOCFG_SPEC, bool, O>;
#[doc = "Field `EMPTYRX` writer - Empty command for the receive FIFO. When a 1 is written to this bit, the RX FIFO is emptied."]
pub type EMPTYRX_W<'a, const O: u8> = crate::BitWriter<'a, u32, FIFOCFG_SPEC, bool, O>;
#[doc = "Field `POPDBG` reader - Pop FIFO for Debug Reads"]
pub type POPDBG_R = crate::BitReader<POPDBG_A>;
#[doc = "Pop FIFO for Debug Reads\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum POPDBG_A {
    #[doc = "0: Debug reads of the FIFO do not pop the FIFO."]
    DO_NOT_POP = 0,
    #[doc = "1: A debug read causes the FIFO to pop."]
    POP = 1,
}
impl From<POPDBG_A> for bool {
    #[inline(always)]
    fn from(variant: POPDBG_A) -> Self {
        variant as u8 != 0
    }
}
impl POPDBG_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> POPDBG_A {
        match self.bits {
            false => POPDBG_A::DO_NOT_POP,
            true => POPDBG_A::POP,
        }
    }
    #[doc = "Checks if the value of the field is `DO_NOT_POP`"]
    #[inline(always)]
    pub fn is_do_not_pop(&self) -> bool {
        *self == POPDBG_A::DO_NOT_POP
    }
    #[doc = "Checks if the value of the field is `POP`"]
    #[inline(always)]
    pub fn is_pop(&self) -> bool {
        *self == POPDBG_A::POP
    }
}
#[doc = "Field `POPDBG` writer - Pop FIFO for Debug Reads"]
pub type POPDBG_W<'a, const O: u8> = crate::BitWriter<'a, u32, FIFOCFG_SPEC, POPDBG_A, O>;
impl<'a, const O: u8> POPDBG_W<'a, O> {
    #[doc = "Debug reads of the FIFO do not pop the FIFO."]
    #[inline(always)]
    pub fn do_not_pop(self) -> &'a mut W {
        self.variant(POPDBG_A::DO_NOT_POP)
    }
    #[doc = "A debug read causes the FIFO to pop."]
    #[inline(always)]
    pub fn pop(self) -> &'a mut W {
        self.variant(POPDBG_A::POP)
    }
}
impl R {
    #[doc = "Bit 0 - Enable Transmit FIFO"]
    #[inline(always)]
    pub fn enabletx(&self) -> ENABLETX_R {
        ENABLETX_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enable Receive FIFO"]
    #[inline(always)]
    pub fn enablerx(&self) -> ENABLERX_R {
        ENABLERX_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Transmit I2S Empty 0"]
    #[inline(always)]
    pub fn txi2se0(&self) -> TXI2SE0_R {
        TXI2SE0_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Packing Format 48-bit data"]
    #[inline(always)]
    pub fn pack48(&self) -> PACK48_R {
        PACK48_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:5 - FIFO Size Configuration"]
    #[inline(always)]
    pub fn size(&self) -> SIZE_R {
        SIZE_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bit 12 - DMA Transmit"]
    #[inline(always)]
    pub fn dmatx(&self) -> DMATX_R {
        DMATX_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - DMA Receive"]
    #[inline(always)]
    pub fn dmarx(&self) -> DMARX_R {
        DMARX_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Wake-up for Transmit FIFO Level"]
    #[inline(always)]
    pub fn waketx(&self) -> WAKETX_R {
        WAKETX_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Wake-up for Receive FIFO Level"]
    #[inline(always)]
    pub fn wakerx(&self) -> WAKERX_R {
        WAKERX_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 18 - Pop FIFO for Debug Reads"]
    #[inline(always)]
    pub fn popdbg(&self) -> POPDBG_R {
        POPDBG_R::new(((self.bits >> 18) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable Transmit FIFO"]
    #[inline(always)]
    #[must_use]
    pub fn enabletx(&mut self) -> ENABLETX_W<0> {
        ENABLETX_W::new(self)
    }
    #[doc = "Bit 1 - Enable Receive FIFO"]
    #[inline(always)]
    #[must_use]
    pub fn enablerx(&mut self) -> ENABLERX_W<1> {
        ENABLERX_W::new(self)
    }
    #[doc = "Bit 2 - Transmit I2S Empty 0"]
    #[inline(always)]
    #[must_use]
    pub fn txi2se0(&mut self) -> TXI2SE0_W<2> {
        TXI2SE0_W::new(self)
    }
    #[doc = "Bit 3 - Packing Format 48-bit data"]
    #[inline(always)]
    #[must_use]
    pub fn pack48(&mut self) -> PACK48_W<3> {
        PACK48_W::new(self)
    }
    #[doc = "Bit 12 - DMA Transmit"]
    #[inline(always)]
    #[must_use]
    pub fn dmatx(&mut self) -> DMATX_W<12> {
        DMATX_W::new(self)
    }
    #[doc = "Bit 13 - DMA Receive"]
    #[inline(always)]
    #[must_use]
    pub fn dmarx(&mut self) -> DMARX_W<13> {
        DMARX_W::new(self)
    }
    #[doc = "Bit 14 - Wake-up for Transmit FIFO Level"]
    #[inline(always)]
    #[must_use]
    pub fn waketx(&mut self) -> WAKETX_W<14> {
        WAKETX_W::new(self)
    }
    #[doc = "Bit 15 - Wake-up for Receive FIFO Level"]
    #[inline(always)]
    #[must_use]
    pub fn wakerx(&mut self) -> WAKERX_W<15> {
        WAKERX_W::new(self)
    }
    #[doc = "Bit 16 - Empty command for the transmit FIFO. When a 1 is written to this bit, the TX FIFO is emptied."]
    #[inline(always)]
    #[must_use]
    pub fn emptytx(&mut self) -> EMPTYTX_W<16> {
        EMPTYTX_W::new(self)
    }
    #[doc = "Bit 17 - Empty command for the receive FIFO. When a 1 is written to this bit, the RX FIFO is emptied."]
    #[inline(always)]
    #[must_use]
    pub fn emptyrx(&mut self) -> EMPTYRX_W<17> {
        EMPTYRX_W::new(self)
    }
    #[doc = "Bit 18 - Pop FIFO for Debug Reads"]
    #[inline(always)]
    #[must_use]
    pub fn popdbg(&mut self) -> POPDBG_W<18> {
        POPDBG_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "FIFO Configuration and Enable\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fifocfg](index.html) module"]
pub struct FIFOCFG_SPEC;
impl crate::RegisterSpec for FIFOCFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fifocfg::R](R) reader structure"]
impl crate::Readable for FIFOCFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fifocfg::W](W) writer structure"]
impl crate::Writable for FIFOCFG_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FIFOCFG to value 0"]
impl crate::Resettable for FIFOCFG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
