#[doc = "Register `INTENCLR` writer"]
pub struct W(crate::W<INTENCLR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INTENCLR_SPEC>;
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
impl From<crate::W<INTENCLR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INTENCLR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Master Pending interrupt clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MSTPENDINGCLR_AW {
    #[doc = "0: No effect on interrupt"]
    NO_EFFECT = 0,
    #[doc = "1: Clears the interrupt bit in INTENSET register"]
    CLEAR_MSTPENDING = 1,
}
impl From<MSTPENDINGCLR_AW> for bool {
    #[inline(always)]
    fn from(variant: MSTPENDINGCLR_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MSTPENDINGCLR` writer - Master Pending interrupt clear"]
pub type MSTPENDINGCLR_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, INTENCLR_SPEC, MSTPENDINGCLR_AW, O>;
impl<'a, const O: u8> MSTPENDINGCLR_W<'a, O> {
    #[doc = "No effect on interrupt"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(MSTPENDINGCLR_AW::NO_EFFECT)
    }
    #[doc = "Clears the interrupt bit in INTENSET register"]
    #[inline(always)]
    pub fn clear_mstpending(self) -> &'a mut W {
        self.variant(MSTPENDINGCLR_AW::CLEAR_MSTPENDING)
    }
}
#[doc = "Master Arbitration Loss interrupt clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MSTARBLOSSCLR_AW {
    #[doc = "0: No effect on interrupt"]
    NO_EFFECT = 0,
    #[doc = "1: Clears the interrupt bit in INTENSET register"]
    CLEAR_MSTARBLOSS = 1,
}
impl From<MSTARBLOSSCLR_AW> for bool {
    #[inline(always)]
    fn from(variant: MSTARBLOSSCLR_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MSTARBLOSSCLR` writer - Master Arbitration Loss interrupt clear"]
pub type MSTARBLOSSCLR_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, INTENCLR_SPEC, MSTARBLOSSCLR_AW, O>;
impl<'a, const O: u8> MSTARBLOSSCLR_W<'a, O> {
    #[doc = "No effect on interrupt"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(MSTARBLOSSCLR_AW::NO_EFFECT)
    }
    #[doc = "Clears the interrupt bit in INTENSET register"]
    #[inline(always)]
    pub fn clear_mstarbloss(self) -> &'a mut W {
        self.variant(MSTARBLOSSCLR_AW::CLEAR_MSTARBLOSS)
    }
}
#[doc = "Master Start/Stop Error interrupt clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MSTSTSTPERRCLR_AW {
    #[doc = "0: No effect on interrupt"]
    NO_EFFECT = 0,
    #[doc = "1: Clears the interrupt bit in INTENSET register"]
    CLEAR_MSTSTSTPERR = 1,
}
impl From<MSTSTSTPERRCLR_AW> for bool {
    #[inline(always)]
    fn from(variant: MSTSTSTPERRCLR_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MSTSTSTPERRCLR` writer - Master Start/Stop Error interrupt clear"]
pub type MSTSTSTPERRCLR_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, INTENCLR_SPEC, MSTSTSTPERRCLR_AW, O>;
impl<'a, const O: u8> MSTSTSTPERRCLR_W<'a, O> {
    #[doc = "No effect on interrupt"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(MSTSTSTPERRCLR_AW::NO_EFFECT)
    }
    #[doc = "Clears the interrupt bit in INTENSET register"]
    #[inline(always)]
    pub fn clear_mstststperr(self) -> &'a mut W {
        self.variant(MSTSTSTPERRCLR_AW::CLEAR_MSTSTSTPERR)
    }
}
#[doc = "Slave Pending interrupt clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SLVPENDINGCLR_AW {
    #[doc = "0: No effect on interrupt"]
    NO_EFFECT = 0,
    #[doc = "1: Clears the interrupt bit in INTENSET register"]
    CLEAR_SLVPENDING = 1,
}
impl From<SLVPENDINGCLR_AW> for bool {
    #[inline(always)]
    fn from(variant: SLVPENDINGCLR_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SLVPENDINGCLR` writer - Slave Pending interrupt clear"]
pub type SLVPENDINGCLR_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, INTENCLR_SPEC, SLVPENDINGCLR_AW, O>;
impl<'a, const O: u8> SLVPENDINGCLR_W<'a, O> {
    #[doc = "No effect on interrupt"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(SLVPENDINGCLR_AW::NO_EFFECT)
    }
    #[doc = "Clears the interrupt bit in INTENSET register"]
    #[inline(always)]
    pub fn clear_slvpending(self) -> &'a mut W {
        self.variant(SLVPENDINGCLR_AW::CLEAR_SLVPENDING)
    }
}
#[doc = "Slave Not Stretching interrupt clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SLVNOTSTRCLR_AW {
    #[doc = "0: No effect on interrupt"]
    NO_EFFECT = 0,
    #[doc = "1: Clears the interrupt bit in INTENSET register"]
    CLEAR_SLVNOTSTR = 1,
}
impl From<SLVNOTSTRCLR_AW> for bool {
    #[inline(always)]
    fn from(variant: SLVNOTSTRCLR_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SLVNOTSTRCLR` writer - Slave Not Stretching interrupt clear"]
pub type SLVNOTSTRCLR_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, INTENCLR_SPEC, SLVNOTSTRCLR_AW, O>;
impl<'a, const O: u8> SLVNOTSTRCLR_W<'a, O> {
    #[doc = "No effect on interrupt"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(SLVNOTSTRCLR_AW::NO_EFFECT)
    }
    #[doc = "Clears the interrupt bit in INTENSET register"]
    #[inline(always)]
    pub fn clear_slvnotstr(self) -> &'a mut W {
        self.variant(SLVNOTSTRCLR_AW::CLEAR_SLVNOTSTR)
    }
}
#[doc = "Slave Deselect interrupt clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SLVDESELCLR_AW {
    #[doc = "0: No effect on interrupt"]
    NO_EFFECT = 0,
    #[doc = "1: Clears the interrupt bit in INTENSET register"]
    CLEAR_SLVDESEL = 1,
}
impl From<SLVDESELCLR_AW> for bool {
    #[inline(always)]
    fn from(variant: SLVDESELCLR_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SLVDESELCLR` writer - Slave Deselect interrupt clear"]
pub type SLVDESELCLR_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, INTENCLR_SPEC, SLVDESELCLR_AW, O>;
impl<'a, const O: u8> SLVDESELCLR_W<'a, O> {
    #[doc = "No effect on interrupt"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(SLVDESELCLR_AW::NO_EFFECT)
    }
    #[doc = "Clears the interrupt bit in INTENSET register"]
    #[inline(always)]
    pub fn clear_slvdesel(self) -> &'a mut W {
        self.variant(SLVDESELCLR_AW::CLEAR_SLVDESEL)
    }
}
#[doc = "Monitor data Ready interrupt clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MONRDYCLR_AW {
    #[doc = "0: No effect on interrupt"]
    NO_EFFECT = 0,
    #[doc = "1: Clears the interrupt bit in INTENSET register"]
    CLEAR_MONRDY = 1,
}
impl From<MONRDYCLR_AW> for bool {
    #[inline(always)]
    fn from(variant: MONRDYCLR_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MONRDYCLR` writer - Monitor data Ready interrupt clear"]
pub type MONRDYCLR_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTENCLR_SPEC, MONRDYCLR_AW, O>;
impl<'a, const O: u8> MONRDYCLR_W<'a, O> {
    #[doc = "No effect on interrupt"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(MONRDYCLR_AW::NO_EFFECT)
    }
    #[doc = "Clears the interrupt bit in INTENSET register"]
    #[inline(always)]
    pub fn clear_monrdy(self) -> &'a mut W {
        self.variant(MONRDYCLR_AW::CLEAR_MONRDY)
    }
}
#[doc = "Monitor Overrun interrupt clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MONOVCLR_AW {
    #[doc = "0: No effect on interrupt"]
    NO_EFFECT = 0,
    #[doc = "1: Clears the interrupt bit in INTENSET register"]
    CLEAR_MONOV = 1,
}
impl From<MONOVCLR_AW> for bool {
    #[inline(always)]
    fn from(variant: MONOVCLR_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MONOVCLR` writer - Monitor Overrun interrupt clear"]
pub type MONOVCLR_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTENCLR_SPEC, MONOVCLR_AW, O>;
impl<'a, const O: u8> MONOVCLR_W<'a, O> {
    #[doc = "No effect on interrupt"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(MONOVCLR_AW::NO_EFFECT)
    }
    #[doc = "Clears the interrupt bit in INTENSET register"]
    #[inline(always)]
    pub fn clear_monov(self) -> &'a mut W {
        self.variant(MONOVCLR_AW::CLEAR_MONOV)
    }
}
#[doc = "Monitor Idle interrupt clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MONIDLECLR_AW {
    #[doc = "0: No effect on interrupt"]
    NO_EFFECT = 0,
    #[doc = "1: Clears the interrupt bit in INTENSET register"]
    CLEAR_MONIDLE = 1,
}
impl From<MONIDLECLR_AW> for bool {
    #[inline(always)]
    fn from(variant: MONIDLECLR_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MONIDLECLR` writer - Monitor Idle interrupt clear"]
pub type MONIDLECLR_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTENCLR_SPEC, MONIDLECLR_AW, O>;
impl<'a, const O: u8> MONIDLECLR_W<'a, O> {
    #[doc = "No effect on interrupt"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(MONIDLECLR_AW::NO_EFFECT)
    }
    #[doc = "Clears the interrupt bit in INTENSET register"]
    #[inline(always)]
    pub fn clear_monidle(self) -> &'a mut W {
        self.variant(MONIDLECLR_AW::CLEAR_MONIDLE)
    }
}
#[doc = "Event time-out interrupt clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EVENTTIMEOUTCLR_AW {
    #[doc = "0: No effect on interrupt"]
    NO_EFFECT = 0,
    #[doc = "1: Clears the interrupt bit in INTENSET register"]
    CLEAR_EVENTTIMEOUT = 1,
}
impl From<EVENTTIMEOUTCLR_AW> for bool {
    #[inline(always)]
    fn from(variant: EVENTTIMEOUTCLR_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EVENTTIMEOUTCLR` writer - Event time-out interrupt clear"]
pub type EVENTTIMEOUTCLR_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, INTENCLR_SPEC, EVENTTIMEOUTCLR_AW, O>;
impl<'a, const O: u8> EVENTTIMEOUTCLR_W<'a, O> {
    #[doc = "No effect on interrupt"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(EVENTTIMEOUTCLR_AW::NO_EFFECT)
    }
    #[doc = "Clears the interrupt bit in INTENSET register"]
    #[inline(always)]
    pub fn clear_eventtimeout(self) -> &'a mut W {
        self.variant(EVENTTIMEOUTCLR_AW::CLEAR_EVENTTIMEOUT)
    }
}
#[doc = "SCL time-out interrupt clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SCLTIMEOUTCLR_AW {
    #[doc = "0: No effect on interrupt"]
    NO_EFFECT = 0,
    #[doc = "1: Clears the interrupt bit in INTENSET register"]
    CLEAR_SCLTIMEOUT = 1,
}
impl From<SCLTIMEOUTCLR_AW> for bool {
    #[inline(always)]
    fn from(variant: SCLTIMEOUTCLR_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SCLTIMEOUTCLR` writer - SCL time-out interrupt clear"]
pub type SCLTIMEOUTCLR_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, INTENCLR_SPEC, SCLTIMEOUTCLR_AW, O>;
impl<'a, const O: u8> SCLTIMEOUTCLR_W<'a, O> {
    #[doc = "No effect on interrupt"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(SCLTIMEOUTCLR_AW::NO_EFFECT)
    }
    #[doc = "Clears the interrupt bit in INTENSET register"]
    #[inline(always)]
    pub fn clear_scltimeout(self) -> &'a mut W {
        self.variant(SCLTIMEOUTCLR_AW::CLEAR_SCLTIMEOUT)
    }
}
impl W {
    #[doc = "Bit 0 - Master Pending interrupt clear"]
    #[inline(always)]
    #[must_use]
    pub fn mstpendingclr(&mut self) -> MSTPENDINGCLR_W<0> {
        MSTPENDINGCLR_W::new(self)
    }
    #[doc = "Bit 4 - Master Arbitration Loss interrupt clear"]
    #[inline(always)]
    #[must_use]
    pub fn mstarblossclr(&mut self) -> MSTARBLOSSCLR_W<4> {
        MSTARBLOSSCLR_W::new(self)
    }
    #[doc = "Bit 6 - Master Start/Stop Error interrupt clear"]
    #[inline(always)]
    #[must_use]
    pub fn mstststperrclr(&mut self) -> MSTSTSTPERRCLR_W<6> {
        MSTSTSTPERRCLR_W::new(self)
    }
    #[doc = "Bit 8 - Slave Pending interrupt clear"]
    #[inline(always)]
    #[must_use]
    pub fn slvpendingclr(&mut self) -> SLVPENDINGCLR_W<8> {
        SLVPENDINGCLR_W::new(self)
    }
    #[doc = "Bit 11 - Slave Not Stretching interrupt clear"]
    #[inline(always)]
    #[must_use]
    pub fn slvnotstrclr(&mut self) -> SLVNOTSTRCLR_W<11> {
        SLVNOTSTRCLR_W::new(self)
    }
    #[doc = "Bit 15 - Slave Deselect interrupt clear"]
    #[inline(always)]
    #[must_use]
    pub fn slvdeselclr(&mut self) -> SLVDESELCLR_W<15> {
        SLVDESELCLR_W::new(self)
    }
    #[doc = "Bit 16 - Monitor data Ready interrupt clear"]
    #[inline(always)]
    #[must_use]
    pub fn monrdyclr(&mut self) -> MONRDYCLR_W<16> {
        MONRDYCLR_W::new(self)
    }
    #[doc = "Bit 17 - Monitor Overrun interrupt clear"]
    #[inline(always)]
    #[must_use]
    pub fn monovclr(&mut self) -> MONOVCLR_W<17> {
        MONOVCLR_W::new(self)
    }
    #[doc = "Bit 19 - Monitor Idle interrupt clear"]
    #[inline(always)]
    #[must_use]
    pub fn monidleclr(&mut self) -> MONIDLECLR_W<19> {
        MONIDLECLR_W::new(self)
    }
    #[doc = "Bit 24 - Event time-out interrupt clear"]
    #[inline(always)]
    #[must_use]
    pub fn eventtimeoutclr(&mut self) -> EVENTTIMEOUTCLR_W<24> {
        EVENTTIMEOUTCLR_W::new(self)
    }
    #[doc = "Bit 25 - SCL time-out interrupt clear"]
    #[inline(always)]
    #[must_use]
    pub fn scltimeoutclr(&mut self) -> SCLTIMEOUTCLR_W<25> {
        SCLTIMEOUTCLR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt Enable Clear Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intenclr](index.html) module"]
pub struct INTENCLR_SPEC;
impl crate::RegisterSpec for INTENCLR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [intenclr::W](W) writer structure"]
impl crate::Writable for INTENCLR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets INTENCLR to value 0"]
impl crate::Resettable for INTENCLR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
