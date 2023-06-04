#[doc = "Register `FIFOTRIG` reader"]
pub struct R(crate::R<FIFOTRIG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FIFOTRIG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FIFOTRIG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FIFOTRIG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FIFOTRIG` writer"]
pub struct W(crate::W<FIFOTRIG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FIFOTRIG_SPEC>;
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
impl From<crate::W<FIFOTRIG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FIFOTRIG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TXLVLENA` reader - Transmit FIFO Level Trigger Enable"]
pub type TXLVLENA_R = crate::BitReader<TXLVLENA_A>;
#[doc = "Transmit FIFO Level Trigger Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TXLVLENA_A {
    #[doc = "0: Transmit FIFO level does not generate a FIFO level trigger."]
    DISABLED = 0,
    #[doc = "1: An trigger generates if the transmit FIFO level reaches the value specified by the TXLVL field in this register."]
    ENABLED = 1,
}
impl From<TXLVLENA_A> for bool {
    #[inline(always)]
    fn from(variant: TXLVLENA_A) -> Self {
        variant as u8 != 0
    }
}
impl TXLVLENA_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TXLVLENA_A {
        match self.bits {
            false => TXLVLENA_A::DISABLED,
            true => TXLVLENA_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == TXLVLENA_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == TXLVLENA_A::ENABLED
    }
}
#[doc = "Field `TXLVLENA` writer - Transmit FIFO Level Trigger Enable"]
pub type TXLVLENA_W<'a, const O: u8> = crate::BitWriter<'a, u32, FIFOTRIG_SPEC, TXLVLENA_A, O>;
impl<'a, const O: u8> TXLVLENA_W<'a, O> {
    #[doc = "Transmit FIFO level does not generate a FIFO level trigger."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(TXLVLENA_A::DISABLED)
    }
    #[doc = "An trigger generates if the transmit FIFO level reaches the value specified by the TXLVL field in this register."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(TXLVLENA_A::ENABLED)
    }
}
#[doc = "Field `RXLVLENA` reader - Receive FIFO Level Trigger Enable"]
pub type RXLVLENA_R = crate::BitReader<RXLVLENA_A>;
#[doc = "Receive FIFO Level Trigger Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RXLVLENA_A {
    #[doc = "0: Receive FIFO level does not generate a FIFO level trigger."]
    DISABLED = 0,
    #[doc = "1: An trigger generates if the receive FIFO level reaches the value specified by the RXLVL."]
    ENABLED = 1,
}
impl From<RXLVLENA_A> for bool {
    #[inline(always)]
    fn from(variant: RXLVLENA_A) -> Self {
        variant as u8 != 0
    }
}
impl RXLVLENA_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RXLVLENA_A {
        match self.bits {
            false => RXLVLENA_A::DISABLED,
            true => RXLVLENA_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == RXLVLENA_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == RXLVLENA_A::ENABLED
    }
}
#[doc = "Field `RXLVLENA` writer - Receive FIFO Level Trigger Enable"]
pub type RXLVLENA_W<'a, const O: u8> = crate::BitWriter<'a, u32, FIFOTRIG_SPEC, RXLVLENA_A, O>;
impl<'a, const O: u8> RXLVLENA_W<'a, O> {
    #[doc = "Receive FIFO level does not generate a FIFO level trigger."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(RXLVLENA_A::DISABLED)
    }
    #[doc = "An trigger generates if the receive FIFO level reaches the value specified by the RXLVL."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(RXLVLENA_A::ENABLED)
    }
}
#[doc = "Field `TXLVL` reader - Transmit FIFO Level Trigger Point"]
pub type TXLVL_R = crate::FieldReader<u8, TXLVL_A>;
#[doc = "Transmit FIFO Level Trigger Point\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TXLVL_A {
    #[doc = "0: Trigger when the TX FIFO becomes empty."]
    EMPTY = 0,
    #[doc = "1: Trigger when the TX FIFO level decreases to one entry."]
    ONE_ENTRY = 1,
    #[doc = "15: Trigger when the TX FIFO level decreases to 15 entries (is no longer full)."]
    FIFTEEN_ENTRIES = 15,
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
            1 => Some(TXLVL_A::ONE_ENTRY),
            15 => Some(TXLVL_A::FIFTEEN_ENTRIES),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `EMPTY`"]
    #[inline(always)]
    pub fn is_empty(&self) -> bool {
        *self == TXLVL_A::EMPTY
    }
    #[doc = "Checks if the value of the field is `ONE_ENTRY`"]
    #[inline(always)]
    pub fn is_one_entry(&self) -> bool {
        *self == TXLVL_A::ONE_ENTRY
    }
    #[doc = "Checks if the value of the field is `FIFTEEN_ENTRIES`"]
    #[inline(always)]
    pub fn is_fifteen_entries(&self) -> bool {
        *self == TXLVL_A::FIFTEEN_ENTRIES
    }
}
#[doc = "Field `TXLVL` writer - Transmit FIFO Level Trigger Point"]
pub type TXLVL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FIFOTRIG_SPEC, u8, TXLVL_A, 4, O>;
impl<'a, const O: u8> TXLVL_W<'a, O> {
    #[doc = "Trigger when the TX FIFO becomes empty."]
    #[inline(always)]
    pub fn empty(self) -> &'a mut W {
        self.variant(TXLVL_A::EMPTY)
    }
    #[doc = "Trigger when the TX FIFO level decreases to one entry."]
    #[inline(always)]
    pub fn one_entry(self) -> &'a mut W {
        self.variant(TXLVL_A::ONE_ENTRY)
    }
    #[doc = "Trigger when the TX FIFO level decreases to 15 entries (is no longer full)."]
    #[inline(always)]
    pub fn fifteen_entries(self) -> &'a mut W {
        self.variant(TXLVL_A::FIFTEEN_ENTRIES)
    }
}
#[doc = "Field `RXLVL` reader - Receive FIFO Level Trigger Point"]
pub type RXLVL_R = crate::FieldReader<u8, RXLVL_A>;
#[doc = "Receive FIFO Level Trigger Point\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum RXLVL_A {
    #[doc = "0: Trigger when the RX FIFO has received 1 entry (the FIFO is no longer empty)."]
    ONE_ENTRY = 0,
    #[doc = "1: Trigger when the RX FIFO has received 2 entries."]
    TWO_ENTRIES = 1,
    #[doc = "15: Trigger when the RX FIFO has received 16 entries (the FIFO has become full)."]
    SIXTEEN_ENTRIES = 15,
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
            0 => Some(RXLVL_A::ONE_ENTRY),
            1 => Some(RXLVL_A::TWO_ENTRIES),
            15 => Some(RXLVL_A::SIXTEEN_ENTRIES),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `ONE_ENTRY`"]
    #[inline(always)]
    pub fn is_one_entry(&self) -> bool {
        *self == RXLVL_A::ONE_ENTRY
    }
    #[doc = "Checks if the value of the field is `TWO_ENTRIES`"]
    #[inline(always)]
    pub fn is_two_entries(&self) -> bool {
        *self == RXLVL_A::TWO_ENTRIES
    }
    #[doc = "Checks if the value of the field is `SIXTEEN_ENTRIES`"]
    #[inline(always)]
    pub fn is_sixteen_entries(&self) -> bool {
        *self == RXLVL_A::SIXTEEN_ENTRIES
    }
}
#[doc = "Field `RXLVL` writer - Receive FIFO Level Trigger Point"]
pub type RXLVL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FIFOTRIG_SPEC, u8, RXLVL_A, 4, O>;
impl<'a, const O: u8> RXLVL_W<'a, O> {
    #[doc = "Trigger when the RX FIFO has received 1 entry (the FIFO is no longer empty)."]
    #[inline(always)]
    pub fn one_entry(self) -> &'a mut W {
        self.variant(RXLVL_A::ONE_ENTRY)
    }
    #[doc = "Trigger when the RX FIFO has received 2 entries."]
    #[inline(always)]
    pub fn two_entries(self) -> &'a mut W {
        self.variant(RXLVL_A::TWO_ENTRIES)
    }
    #[doc = "Trigger when the RX FIFO has received 16 entries (the FIFO has become full)."]
    #[inline(always)]
    pub fn sixteen_entries(self) -> &'a mut W {
        self.variant(RXLVL_A::SIXTEEN_ENTRIES)
    }
}
impl R {
    #[doc = "Bit 0 - Transmit FIFO Level Trigger Enable"]
    #[inline(always)]
    pub fn txlvlena(&self) -> TXLVLENA_R {
        TXLVLENA_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Receive FIFO Level Trigger Enable"]
    #[inline(always)]
    pub fn rxlvlena(&self) -> RXLVLENA_R {
        RXLVLENA_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 8:11 - Transmit FIFO Level Trigger Point"]
    #[inline(always)]
    pub fn txlvl(&self) -> TXLVL_R {
        TXLVL_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - Receive FIFO Level Trigger Point"]
    #[inline(always)]
    pub fn rxlvl(&self) -> RXLVL_R {
        RXLVL_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Transmit FIFO Level Trigger Enable"]
    #[inline(always)]
    #[must_use]
    pub fn txlvlena(&mut self) -> TXLVLENA_W<0> {
        TXLVLENA_W::new(self)
    }
    #[doc = "Bit 1 - Receive FIFO Level Trigger Enable"]
    #[inline(always)]
    #[must_use]
    pub fn rxlvlena(&mut self) -> RXLVLENA_W<1> {
        RXLVLENA_W::new(self)
    }
    #[doc = "Bits 8:11 - Transmit FIFO Level Trigger Point"]
    #[inline(always)]
    #[must_use]
    pub fn txlvl(&mut self) -> TXLVL_W<8> {
        TXLVL_W::new(self)
    }
    #[doc = "Bits 16:19 - Receive FIFO Level Trigger Point"]
    #[inline(always)]
    #[must_use]
    pub fn rxlvl(&mut self) -> RXLVL_W<16> {
        RXLVL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "FIFO Trigger Settings\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fifotrig](index.html) module"]
pub struct FIFOTRIG_SPEC;
impl crate::RegisterSpec for FIFOTRIG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fifotrig::R](R) reader structure"]
impl crate::Readable for FIFOTRIG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fifotrig::W](W) writer structure"]
impl crate::Writable for FIFOTRIG_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FIFOTRIG to value 0"]
impl crate::Resettable for FIFOTRIG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
