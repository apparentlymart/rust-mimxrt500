#[doc = "Register `FIFO_CTRL` reader"]
pub struct R(crate::R<FIFO_CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FIFO_CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FIFO_CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FIFO_CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FIFO_CTRL` writer"]
pub struct W(crate::W<FIFO_CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FIFO_CTRL_SPEC>;
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
impl From<crate::W<FIFO_CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FIFO_CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ENABLE` reader - FIFO Enable."]
pub type ENABLE_R = crate::BitReader<ENABLE_A>;
#[doc = "FIFO Enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ENABLE_A {
    #[doc = "0: Disabled."]
    DISABLED = 0,
    #[doc = "1: FIFO is enabled. The FIFO must be enabled in order for the CPU or DMA to read data from the DMIC via the FIFODATA register."]
    ENABLED = 1,
}
impl From<ENABLE_A> for bool {
    #[inline(always)]
    fn from(variant: ENABLE_A) -> Self {
        variant as u8 != 0
    }
}
impl ENABLE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ENABLE_A {
        match self.bits {
            false => ENABLE_A::DISABLED,
            true => ENABLE_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ENABLE_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ENABLE_A::ENABLED
    }
}
#[doc = "Field `ENABLE` writer - FIFO Enable."]
pub type ENABLE_W<'a, const O: u8> = crate::BitWriter<'a, u32, FIFO_CTRL_SPEC, ENABLE_A, O>;
impl<'a, const O: u8> ENABLE_W<'a, O> {
    #[doc = "Disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(ENABLE_A::DISABLED)
    }
    #[doc = "FIFO is enabled. The FIFO must be enabled in order for the CPU or DMA to read data from the DMIC via the FIFODATA register."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(ENABLE_A::ENABLED)
    }
}
#[doc = "Field `RESETN` reader - FIFO Reset"]
pub type RESETN_R = crate::BitReader<RESETN_A>;
#[doc = "FIFO Reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RESETN_A {
    #[doc = "0: Reset the FIFO. This must be cleared before resuming operation."]
    RESET = 0,
    #[doc = "1: Normal operation"]
    NORMAL = 1,
}
impl From<RESETN_A> for bool {
    #[inline(always)]
    fn from(variant: RESETN_A) -> Self {
        variant as u8 != 0
    }
}
impl RESETN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RESETN_A {
        match self.bits {
            false => RESETN_A::RESET,
            true => RESETN_A::NORMAL,
        }
    }
    #[doc = "Checks if the value of the field is `RESET`"]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == RESETN_A::RESET
    }
    #[doc = "Checks if the value of the field is `NORMAL`"]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == RESETN_A::NORMAL
    }
}
#[doc = "Field `RESETN` writer - FIFO Reset"]
pub type RESETN_W<'a, const O: u8> = crate::BitWriter<'a, u32, FIFO_CTRL_SPEC, RESETN_A, O>;
impl<'a, const O: u8> RESETN_W<'a, O> {
    #[doc = "Reset the FIFO. This must be cleared before resuming operation."]
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(RESETN_A::RESET)
    }
    #[doc = "Normal operation"]
    #[inline(always)]
    pub fn normal(self) -> &'a mut W {
        self.variant(RESETN_A::NORMAL)
    }
}
#[doc = "Field `INTEN` reader - Interrupt Enable."]
pub type INTEN_R = crate::BitReader<INTEN_A>;
#[doc = "Interrupt Enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum INTEN_A {
    #[doc = "0: FIFO level interrupts are not enabled."]
    DISABLED = 0,
    #[doc = "1: FIFO level interrupts are enabled."]
    ENABLED = 1,
}
impl From<INTEN_A> for bool {
    #[inline(always)]
    fn from(variant: INTEN_A) -> Self {
        variant as u8 != 0
    }
}
impl INTEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INTEN_A {
        match self.bits {
            false => INTEN_A::DISABLED,
            true => INTEN_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == INTEN_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == INTEN_A::ENABLED
    }
}
#[doc = "Field `INTEN` writer - Interrupt Enable."]
pub type INTEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, FIFO_CTRL_SPEC, INTEN_A, O>;
impl<'a, const O: u8> INTEN_W<'a, O> {
    #[doc = "FIFO level interrupts are not enabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(INTEN_A::DISABLED)
    }
    #[doc = "FIFO level interrupts are enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(INTEN_A::ENABLED)
    }
}
#[doc = "Field `DMAEN` reader - DMA Enable"]
pub type DMAEN_R = crate::BitReader<DMAEN_A>;
#[doc = "DMA Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DMAEN_A {
    #[doc = "0: DMA requests are not enabled."]
    DISABLED = 0,
    #[doc = "1: DMA requests based on FIFO level are enabled."]
    ENABLED = 1,
}
impl From<DMAEN_A> for bool {
    #[inline(always)]
    fn from(variant: DMAEN_A) -> Self {
        variant as u8 != 0
    }
}
impl DMAEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DMAEN_A {
        match self.bits {
            false => DMAEN_A::DISABLED,
            true => DMAEN_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == DMAEN_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == DMAEN_A::ENABLED
    }
}
#[doc = "Field `DMAEN` writer - DMA Enable"]
pub type DMAEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, FIFO_CTRL_SPEC, DMAEN_A, O>;
impl<'a, const O: u8> DMAEN_W<'a, O> {
    #[doc = "DMA requests are not enabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(DMAEN_A::DISABLED)
    }
    #[doc = "DMA requests based on FIFO level are enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(DMAEN_A::ENABLED)
    }
}
#[doc = "Field `TRIGLVL` reader - FIFO Trigger Level for Interrupt"]
pub type TRIGLVL_R = crate::FieldReader<u8, TRIGLVL_A>;
#[doc = "FIFO Trigger Level for Interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TRIGLVL_A {
    #[doc = "0: Trigger when the FIFO has received one entry (is no longer empty)."]
    ONEENTRY = 0,
    #[doc = "1: Trigger when the FIFO has received two entries."]
    TWOENTRIES = 1,
    #[doc = "14: Trigger when the FIFO has received 15 entries."]
    ENTRIES15 = 14,
    #[doc = "15: Trigger when the FIFO has received 16 entries (has become full)."]
    ENTRIES16 = 15,
}
impl From<TRIGLVL_A> for u8 {
    #[inline(always)]
    fn from(variant: TRIGLVL_A) -> Self {
        variant as _
    }
}
impl TRIGLVL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<TRIGLVL_A> {
        match self.bits {
            0 => Some(TRIGLVL_A::ONEENTRY),
            1 => Some(TRIGLVL_A::TWOENTRIES),
            14 => Some(TRIGLVL_A::ENTRIES15),
            15 => Some(TRIGLVL_A::ENTRIES16),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `ONEENTRY`"]
    #[inline(always)]
    pub fn is_oneentry(&self) -> bool {
        *self == TRIGLVL_A::ONEENTRY
    }
    #[doc = "Checks if the value of the field is `TWOENTRIES`"]
    #[inline(always)]
    pub fn is_twoentries(&self) -> bool {
        *self == TRIGLVL_A::TWOENTRIES
    }
    #[doc = "Checks if the value of the field is `ENTRIES15`"]
    #[inline(always)]
    pub fn is_entries15(&self) -> bool {
        *self == TRIGLVL_A::ENTRIES15
    }
    #[doc = "Checks if the value of the field is `ENTRIES16`"]
    #[inline(always)]
    pub fn is_entries16(&self) -> bool {
        *self == TRIGLVL_A::ENTRIES16
    }
}
#[doc = "Field `TRIGLVL` writer - FIFO Trigger Level for Interrupt"]
pub type TRIGLVL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, FIFO_CTRL_SPEC, u8, TRIGLVL_A, 5, O>;
impl<'a, const O: u8> TRIGLVL_W<'a, O> {
    #[doc = "Trigger when the FIFO has received one entry (is no longer empty)."]
    #[inline(always)]
    pub fn oneentry(self) -> &'a mut W {
        self.variant(TRIGLVL_A::ONEENTRY)
    }
    #[doc = "Trigger when the FIFO has received two entries."]
    #[inline(always)]
    pub fn twoentries(self) -> &'a mut W {
        self.variant(TRIGLVL_A::TWOENTRIES)
    }
    #[doc = "Trigger when the FIFO has received 15 entries."]
    #[inline(always)]
    pub fn entries15(self) -> &'a mut W {
        self.variant(TRIGLVL_A::ENTRIES15)
    }
    #[doc = "Trigger when the FIFO has received 16 entries (has become full)."]
    #[inline(always)]
    pub fn entries16(self) -> &'a mut W {
        self.variant(TRIGLVL_A::ENTRIES16)
    }
}
impl R {
    #[doc = "Bit 0 - FIFO Enable."]
    #[inline(always)]
    pub fn enable(&self) -> ENABLE_R {
        ENABLE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - FIFO Reset"]
    #[inline(always)]
    pub fn resetn(&self) -> RESETN_R {
        RESETN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Interrupt Enable."]
    #[inline(always)]
    pub fn inten(&self) -> INTEN_R {
        INTEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - DMA Enable"]
    #[inline(always)]
    pub fn dmaen(&self) -> DMAEN_R {
        DMAEN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 16:20 - FIFO Trigger Level for Interrupt"]
    #[inline(always)]
    pub fn triglvl(&self) -> TRIGLVL_R {
        TRIGLVL_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - FIFO Enable."]
    #[inline(always)]
    #[must_use]
    pub fn enable(&mut self) -> ENABLE_W<0> {
        ENABLE_W::new(self)
    }
    #[doc = "Bit 1 - FIFO Reset"]
    #[inline(always)]
    #[must_use]
    pub fn resetn(&mut self) -> RESETN_W<1> {
        RESETN_W::new(self)
    }
    #[doc = "Bit 2 - Interrupt Enable."]
    #[inline(always)]
    #[must_use]
    pub fn inten(&mut self) -> INTEN_W<2> {
        INTEN_W::new(self)
    }
    #[doc = "Bit 3 - DMA Enable"]
    #[inline(always)]
    #[must_use]
    pub fn dmaen(&mut self) -> DMAEN_W<3> {
        DMAEN_W::new(self)
    }
    #[doc = "Bits 16:20 - FIFO Trigger Level for Interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn triglvl(&mut self) -> TRIGLVL_W<16> {
        TRIGLVL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "FIFO Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fifo_ctrl](index.html) module"]
pub struct FIFO_CTRL_SPEC;
impl crate::RegisterSpec for FIFO_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fifo_ctrl::R](R) reader structure"]
impl crate::Readable for FIFO_CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fifo_ctrl::W](W) writer structure"]
impl crate::Writable for FIFO_CTRL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FIFO_CTRL to value 0"]
impl crate::Resettable for FIFO_CTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
