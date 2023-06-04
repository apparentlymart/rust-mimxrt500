#[doc = "Register `FIFOINTENSET` reader"]
pub struct R(crate::R<FIFOINTENSET_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FIFOINTENSET_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FIFOINTENSET_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FIFOINTENSET_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FIFOINTENSET` writer"]
pub struct W(crate::W<FIFOINTENSET_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FIFOINTENSET_SPEC>;
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
impl From<crate::W<FIFOINTENSET_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FIFOINTENSET_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TXERR` reader - Transmit Error Interrupt Enable"]
pub type TXERR_R = crate::BitReader<TXERR_A>;
#[doc = "Transmit Error Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TXERR_A {
    #[doc = "0: No interrupt will be generated for a transmit error."]
    DISABLED = 0,
    #[doc = "1: An interrupt will be generated when a transmit error occurs."]
    ENABLED = 1,
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
            false => TXERR_A::DISABLED,
            true => TXERR_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == TXERR_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == TXERR_A::ENABLED
    }
}
#[doc = "Field `TXERR` writer - Transmit Error Interrupt Enable"]
pub type TXERR_W<'a, const O: u8> = crate::BitWriter<'a, u32, FIFOINTENSET_SPEC, TXERR_A, O>;
impl<'a, const O: u8> TXERR_W<'a, O> {
    #[doc = "No interrupt will be generated for a transmit error."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(TXERR_A::DISABLED)
    }
    #[doc = "An interrupt will be generated when a transmit error occurs."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(TXERR_A::ENABLED)
    }
}
#[doc = "Field `RXERR` reader - Receive Error Interrupt Enable"]
pub type RXERR_R = crate::BitReader<RXERR_A>;
#[doc = "Receive Error Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RXERR_A {
    #[doc = "0: No interrupt will be generated for a receive error."]
    DISABLED = 0,
    #[doc = "1: An interrupt will be generated when a receive error occurs."]
    ENABLED = 1,
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
            false => RXERR_A::DISABLED,
            true => RXERR_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == RXERR_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == RXERR_A::ENABLED
    }
}
#[doc = "Field `RXERR` writer - Receive Error Interrupt Enable"]
pub type RXERR_W<'a, const O: u8> = crate::BitWriter<'a, u32, FIFOINTENSET_SPEC, RXERR_A, O>;
impl<'a, const O: u8> RXERR_W<'a, O> {
    #[doc = "No interrupt will be generated for a receive error."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(RXERR_A::DISABLED)
    }
    #[doc = "An interrupt will be generated when a receive error occurs."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(RXERR_A::ENABLED)
    }
}
#[doc = "Field `TXLVL` reader - Transmit FIFO Level Interrupt Enable"]
pub type TXLVL_R = crate::BitReader<TXLVL_A>;
#[doc = "Transmit FIFO Level Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TXLVL_A {
    #[doc = "0: No interrupt will be generated based on the TX FIFO level."]
    DISABLED = 0,
    #[doc = "1: If FIFOTRIG\\[TXLVLENA\\]
= 1, then an interrupt will be generated when the TX FIFO level decreases to the level specified by FIFOTRIG\\[TXLVL\\]"]
    ENABLED = 1,
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
            false => TXLVL_A::DISABLED,
            true => TXLVL_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == TXLVL_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == TXLVL_A::ENABLED
    }
}
#[doc = "Field `TXLVL` writer - Transmit FIFO Level Interrupt Enable"]
pub type TXLVL_W<'a, const O: u8> = crate::BitWriter<'a, u32, FIFOINTENSET_SPEC, TXLVL_A, O>;
impl<'a, const O: u8> TXLVL_W<'a, O> {
    #[doc = "No interrupt will be generated based on the TX FIFO level."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(TXLVL_A::DISABLED)
    }
    #[doc = "If FIFOTRIG\\[TXLVLENA\\]
= 1, then an interrupt will be generated when the TX FIFO level decreases to the level specified by FIFOTRIG\\[TXLVL\\]"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(TXLVL_A::ENABLED)
    }
}
#[doc = "Field `RXLVL` reader - Receive FIFO Level Interrupt Enable"]
pub type RXLVL_R = crate::BitReader<RXLVL_A>;
#[doc = "Receive FIFO Level Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RXLVL_A {
    #[doc = "0: No interrupt will be generated based on the RX FIFO level."]
    DISABLED = 0,
    #[doc = "1: If FIFOTRIG\\[RXLVLENA\\]
= 1, an interrupt will be generated when the when the RX FIFO level increases to the level specified by FIFOTRIG\\[RXLVL\\]."]
    ENABLED = 1,
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
            false => RXLVL_A::DISABLED,
            true => RXLVL_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == RXLVL_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == RXLVL_A::ENABLED
    }
}
#[doc = "Field `RXLVL` writer - Receive FIFO Level Interrupt Enable"]
pub type RXLVL_W<'a, const O: u8> = crate::BitWriter<'a, u32, FIFOINTENSET_SPEC, RXLVL_A, O>;
impl<'a, const O: u8> RXLVL_W<'a, O> {
    #[doc = "No interrupt will be generated based on the RX FIFO level."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(RXLVL_A::DISABLED)
    }
    #[doc = "If FIFOTRIG\\[RXLVLENA\\]
= 1, an interrupt will be generated when the when the RX FIFO level increases to the level specified by FIFOTRIG\\[RXLVL\\]."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(RXLVL_A::ENABLED)
    }
}
impl R {
    #[doc = "Bit 0 - Transmit Error Interrupt Enable"]
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
    #[doc = "Bit 0 - Transmit Error Interrupt Enable"]
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
#[doc = "FIFO Interrupt Enable\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fifointenset](index.html) module"]
pub struct FIFOINTENSET_SPEC;
impl crate::RegisterSpec for FIFOINTENSET_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fifointenset::R](R) reader structure"]
impl crate::Readable for FIFOINTENSET_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fifointenset::W](W) writer structure"]
impl crate::Writable for FIFOINTENSET_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FIFOINTENSET to value 0"]
impl crate::Resettable for FIFOINTENSET_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
