#[doc = "Register `SDATACTRL` reader"]
pub struct R(crate::R<SDATACTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SDATACTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SDATACTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SDATACTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SDATACTRL` writer"]
pub struct W(crate::W<SDATACTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SDATACTRL_SPEC>;
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
impl From<crate::W<SDATACTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SDATACTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FLUSHTB` writer - Flush the to-bus buffer/FIFO"]
pub type FLUSHTB_W<'a, const O: u8> = crate::BitWriter<'a, u32, SDATACTRL_SPEC, bool, O>;
#[doc = "Field `FLUSHFB` writer - Flushes the from-bus buffer/FIFO"]
pub type FLUSHFB_W<'a, const O: u8> = crate::BitWriter<'a, u32, SDATACTRL_SPEC, bool, O>;
#[doc = "Field `UNLOCK` writer - Unlock"]
pub type UNLOCK_W<'a, const O: u8> = crate::BitWriter<'a, u32, SDATACTRL_SPEC, bool, O>;
#[doc = "Field `TXTRIG` reader - Trigger level for TX FIFO emptiness"]
pub type TXTRIG_R = crate::FieldReader<u8, TXTRIG_A>;
#[doc = "Trigger level for TX FIFO emptiness\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TXTRIG_A {
    #[doc = "0: Trigger on empty"]
    TRIGGREMPTY = 0,
    #[doc = "1: Trigger on full or less"]
    TRIGGRONEFOURTH = 1,
    #[doc = "2: Trigger on .5 full or less"]
    TRIGGRONEHALF = 2,
    #[doc = "3: Trigger on 1 less than full or less (Default)"]
    TRIGGRONELESS = 3,
}
impl From<TXTRIG_A> for u8 {
    #[inline(always)]
    fn from(variant: TXTRIG_A) -> Self {
        variant as _
    }
}
impl TXTRIG_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TXTRIG_A {
        match self.bits {
            0 => TXTRIG_A::TRIGGREMPTY,
            1 => TXTRIG_A::TRIGGRONEFOURTH,
            2 => TXTRIG_A::TRIGGRONEHALF,
            3 => TXTRIG_A::TRIGGRONELESS,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `TRIGGREMPTY`"]
    #[inline(always)]
    pub fn is_triggrempty(&self) -> bool {
        *self == TXTRIG_A::TRIGGREMPTY
    }
    #[doc = "Checks if the value of the field is `TRIGGRONEFOURTH`"]
    #[inline(always)]
    pub fn is_triggronefourth(&self) -> bool {
        *self == TXTRIG_A::TRIGGRONEFOURTH
    }
    #[doc = "Checks if the value of the field is `TRIGGRONEHALF`"]
    #[inline(always)]
    pub fn is_triggronehalf(&self) -> bool {
        *self == TXTRIG_A::TRIGGRONEHALF
    }
    #[doc = "Checks if the value of the field is `TRIGGRONELESS`"]
    #[inline(always)]
    pub fn is_triggroneless(&self) -> bool {
        *self == TXTRIG_A::TRIGGRONELESS
    }
}
#[doc = "Field `TXTRIG` writer - Trigger level for TX FIFO emptiness"]
pub type TXTRIG_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, SDATACTRL_SPEC, u8, TXTRIG_A, 2, O>;
impl<'a, const O: u8> TXTRIG_W<'a, O> {
    #[doc = "Trigger on empty"]
    #[inline(always)]
    pub fn triggrempty(self) -> &'a mut W {
        self.variant(TXTRIG_A::TRIGGREMPTY)
    }
    #[doc = "Trigger on full or less"]
    #[inline(always)]
    pub fn triggronefourth(self) -> &'a mut W {
        self.variant(TXTRIG_A::TRIGGRONEFOURTH)
    }
    #[doc = "Trigger on .5 full or less"]
    #[inline(always)]
    pub fn triggronehalf(self) -> &'a mut W {
        self.variant(TXTRIG_A::TRIGGRONEHALF)
    }
    #[doc = "Trigger on 1 less than full or less (Default)"]
    #[inline(always)]
    pub fn triggroneless(self) -> &'a mut W {
        self.variant(TXTRIG_A::TRIGGRONELESS)
    }
}
#[doc = "Field `RXTRIG` reader - Trigger level for RX FIFO fullness"]
pub type RXTRIG_R = crate::FieldReader<u8, RXTRIG_A>;
#[doc = "Trigger level for RX FIFO fullness\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum RXTRIG_A {
    #[doc = "0: Trigger on not empty"]
    TRIGGRNOTEMPTY = 0,
    #[doc = "1: Trigger on or more full"]
    TRIGGRONEFOURTH = 1,
    #[doc = "2: Trigger on .5 or more full"]
    TRIGGRONEHALF = 2,
    #[doc = "3: Trigger on 3/4 or more full"]
    TRIGGRTHREEFOURTHS = 3,
}
impl From<RXTRIG_A> for u8 {
    #[inline(always)]
    fn from(variant: RXTRIG_A) -> Self {
        variant as _
    }
}
impl RXTRIG_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RXTRIG_A {
        match self.bits {
            0 => RXTRIG_A::TRIGGRNOTEMPTY,
            1 => RXTRIG_A::TRIGGRONEFOURTH,
            2 => RXTRIG_A::TRIGGRONEHALF,
            3 => RXTRIG_A::TRIGGRTHREEFOURTHS,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `TRIGGRNOTEMPTY`"]
    #[inline(always)]
    pub fn is_triggrnotempty(&self) -> bool {
        *self == RXTRIG_A::TRIGGRNOTEMPTY
    }
    #[doc = "Checks if the value of the field is `TRIGGRONEFOURTH`"]
    #[inline(always)]
    pub fn is_triggronefourth(&self) -> bool {
        *self == RXTRIG_A::TRIGGRONEFOURTH
    }
    #[doc = "Checks if the value of the field is `TRIGGRONEHALF`"]
    #[inline(always)]
    pub fn is_triggronehalf(&self) -> bool {
        *self == RXTRIG_A::TRIGGRONEHALF
    }
    #[doc = "Checks if the value of the field is `TRIGGRTHREEFOURTHS`"]
    #[inline(always)]
    pub fn is_triggrthreefourths(&self) -> bool {
        *self == RXTRIG_A::TRIGGRTHREEFOURTHS
    }
}
#[doc = "Field `RXTRIG` writer - Trigger level for RX FIFO fullness"]
pub type RXTRIG_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, SDATACTRL_SPEC, u8, RXTRIG_A, 2, O>;
impl<'a, const O: u8> RXTRIG_W<'a, O> {
    #[doc = "Trigger on not empty"]
    #[inline(always)]
    pub fn triggrnotempty(self) -> &'a mut W {
        self.variant(RXTRIG_A::TRIGGRNOTEMPTY)
    }
    #[doc = "Trigger on or more full"]
    #[inline(always)]
    pub fn triggronefourth(self) -> &'a mut W {
        self.variant(RXTRIG_A::TRIGGRONEFOURTH)
    }
    #[doc = "Trigger on .5 or more full"]
    #[inline(always)]
    pub fn triggronehalf(self) -> &'a mut W {
        self.variant(RXTRIG_A::TRIGGRONEHALF)
    }
    #[doc = "Trigger on 3/4 or more full"]
    #[inline(always)]
    pub fn triggrthreefourths(self) -> &'a mut W {
        self.variant(RXTRIG_A::TRIGGRTHREEFOURTHS)
    }
}
#[doc = "Field `TXCOUNT` reader - Count of bytes in TX"]
pub type TXCOUNT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RXCOUNT` reader - Count of bytes in RX"]
pub type RXCOUNT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TXFULL` reader - TX is full"]
pub type TXFULL_R = crate::BitReader<TXFULL_A>;
#[doc = "TX is full\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TXFULL_A {
    #[doc = "0: TX is not full"]
    TXISNOTFULL = 0,
    #[doc = "1: TX is full"]
    TXISFULL = 1,
}
impl From<TXFULL_A> for bool {
    #[inline(always)]
    fn from(variant: TXFULL_A) -> Self {
        variant as u8 != 0
    }
}
impl TXFULL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TXFULL_A {
        match self.bits {
            false => TXFULL_A::TXISNOTFULL,
            true => TXFULL_A::TXISFULL,
        }
    }
    #[doc = "Checks if the value of the field is `TXISNOTFULL`"]
    #[inline(always)]
    pub fn is_txisnotfull(&self) -> bool {
        *self == TXFULL_A::TXISNOTFULL
    }
    #[doc = "Checks if the value of the field is `TXISFULL`"]
    #[inline(always)]
    pub fn is_txisfull(&self) -> bool {
        *self == TXFULL_A::TXISFULL
    }
}
#[doc = "Field `RXEMPTY` reader - RX is empty"]
pub type RXEMPTY_R = crate::BitReader<RXEMPTY_A>;
#[doc = "RX is empty\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RXEMPTY_A {
    #[doc = "0: RX is not empty"]
    RXISNOTEMPTY = 0,
    #[doc = "1: RX is empty"]
    RXISEMPTY = 1,
}
impl From<RXEMPTY_A> for bool {
    #[inline(always)]
    fn from(variant: RXEMPTY_A) -> Self {
        variant as u8 != 0
    }
}
impl RXEMPTY_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RXEMPTY_A {
        match self.bits {
            false => RXEMPTY_A::RXISNOTEMPTY,
            true => RXEMPTY_A::RXISEMPTY,
        }
    }
    #[doc = "Checks if the value of the field is `RXISNOTEMPTY`"]
    #[inline(always)]
    pub fn is_rxisnotempty(&self) -> bool {
        *self == RXEMPTY_A::RXISNOTEMPTY
    }
    #[doc = "Checks if the value of the field is `RXISEMPTY`"]
    #[inline(always)]
    pub fn is_rxisempty(&self) -> bool {
        *self == RXEMPTY_A::RXISEMPTY
    }
}
impl R {
    #[doc = "Bits 4:5 - Trigger level for TX FIFO emptiness"]
    #[inline(always)]
    pub fn txtrig(&self) -> TXTRIG_R {
        TXTRIG_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - Trigger level for RX FIFO fullness"]
    #[inline(always)]
    pub fn rxtrig(&self) -> RXTRIG_R {
        RXTRIG_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 16:20 - Count of bytes in TX"]
    #[inline(always)]
    pub fn txcount(&self) -> TXCOUNT_R {
        TXCOUNT_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bits 24:28 - Count of bytes in RX"]
    #[inline(always)]
    pub fn rxcount(&self) -> RXCOUNT_R {
        RXCOUNT_R::new(((self.bits >> 24) & 0x1f) as u8)
    }
    #[doc = "Bit 30 - TX is full"]
    #[inline(always)]
    pub fn txfull(&self) -> TXFULL_R {
        TXFULL_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - RX is empty"]
    #[inline(always)]
    pub fn rxempty(&self) -> RXEMPTY_R {
        RXEMPTY_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Flush the to-bus buffer/FIFO"]
    #[inline(always)]
    #[must_use]
    pub fn flushtb(&mut self) -> FLUSHTB_W<0> {
        FLUSHTB_W::new(self)
    }
    #[doc = "Bit 1 - Flushes the from-bus buffer/FIFO"]
    #[inline(always)]
    #[must_use]
    pub fn flushfb(&mut self) -> FLUSHFB_W<1> {
        FLUSHFB_W::new(self)
    }
    #[doc = "Bit 3 - Unlock"]
    #[inline(always)]
    #[must_use]
    pub fn unlock(&mut self) -> UNLOCK_W<3> {
        UNLOCK_W::new(self)
    }
    #[doc = "Bits 4:5 - Trigger level for TX FIFO emptiness"]
    #[inline(always)]
    #[must_use]
    pub fn txtrig(&mut self) -> TXTRIG_W<4> {
        TXTRIG_W::new(self)
    }
    #[doc = "Bits 6:7 - Trigger level for RX FIFO fullness"]
    #[inline(always)]
    #[must_use]
    pub fn rxtrig(&mut self) -> RXTRIG_W<6> {
        RXTRIG_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Slave Data Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sdatactrl](index.html) module"]
pub struct SDATACTRL_SPEC;
impl crate::RegisterSpec for SDATACTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sdatactrl::R](R) reader structure"]
impl crate::Readable for SDATACTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sdatactrl::W](W) writer structure"]
impl crate::Writable for SDATACTRL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SDATACTRL to value 0x8000_0030"]
impl crate::Resettable for SDATACTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0x8000_0030;
}
