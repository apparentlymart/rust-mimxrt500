#[doc = "Register `SINTSET` reader"]
pub struct R(crate::R<SINTSET_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SINTSET_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SINTSET_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SINTSET_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SINTSET` writer"]
pub struct W(crate::W<SINTSET_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SINTSET_SPEC>;
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
impl From<crate::W<SINTSET_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SINTSET_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `START` reader - Start interrupt enable"]
pub type START_R = crate::BitReader<bool>;
#[doc = "Field `START` writer - Start interrupt enable"]
pub type START_W<'a, const O: u8> = crate::BitWriter<'a, u32, SINTSET_SPEC, bool, O>;
#[doc = "Field `MATCHED` reader - Match interrupt enable"]
pub type MATCHED_R = crate::BitReader<bool>;
#[doc = "Field `MATCHED` writer - Match interrupt enable"]
pub type MATCHED_W<'a, const O: u8> = crate::BitWriter<'a, u32, SINTSET_SPEC, bool, O>;
#[doc = "Field `STOP` reader - Stop interrupt enable"]
pub type STOP_R = crate::BitReader<bool>;
#[doc = "Field `STOP` writer - Stop interrupt enable"]
pub type STOP_W<'a, const O: u8> = crate::BitWriter<'a, u32, SINTSET_SPEC, bool, O>;
#[doc = "Field `RXPEND` reader - Receive interrupt enable"]
pub type RXPEND_R = crate::BitReader<bool>;
#[doc = "Field `RXPEND` writer - Receive interrupt enable"]
pub type RXPEND_W<'a, const O: u8> = crate::BitWriter<'a, u32, SINTSET_SPEC, bool, O>;
#[doc = "Field `TXSEND` reader - Transmit interrupt enable"]
pub type TXSEND_R = crate::BitReader<bool>;
#[doc = "Field `TXSEND` writer - Transmit interrupt enable"]
pub type TXSEND_W<'a, const O: u8> = crate::BitWriter<'a, u32, SINTSET_SPEC, bool, O>;
#[doc = "Field `DACHG` reader - Dynamic address change interrupt enable"]
pub type DACHG_R = crate::BitReader<bool>;
#[doc = "Field `DACHG` writer - Dynamic address change interrupt enable"]
pub type DACHG_W<'a, const O: u8> = crate::BitWriter<'a, u32, SINTSET_SPEC, bool, O>;
#[doc = "Field `CCC` reader - Common Command Code (CCC) (that was not handled by I3C module) interrupt enable"]
pub type CCC_R = crate::BitReader<bool>;
#[doc = "Field `CCC` writer - Common Command Code (CCC) (that was not handled by I3C module) interrupt enable"]
pub type CCC_W<'a, const O: u8> = crate::BitWriter<'a, u32, SINTSET_SPEC, bool, O>;
#[doc = "Field `ERRWARN` reader - Error/warning interrupt enable"]
pub type ERRWARN_R = crate::BitReader<bool>;
#[doc = "Field `ERRWARN` writer - Error/warning interrupt enable"]
pub type ERRWARN_W<'a, const O: u8> = crate::BitWriter<'a, u32, SINTSET_SPEC, bool, O>;
#[doc = "Field `DDRMATCHED` reader - Double Data Rate (DDR) interrupt enable"]
pub type DDRMATCHED_R = crate::BitReader<bool>;
#[doc = "Field `DDRMATCHED` writer - Double Data Rate (DDR) interrupt enable"]
pub type DDRMATCHED_W<'a, const O: u8> = crate::BitWriter<'a, u32, SINTSET_SPEC, bool, O>;
#[doc = "Field `CHANDLED` reader - Common Command Code (CCC) (that was handled by I3C module) interrupt enable"]
pub type CHANDLED_R = crate::BitReader<bool>;
#[doc = "Field `CHANDLED` writer - Common Command Code (CCC) (that was handled by I3C module) interrupt enable"]
pub type CHANDLED_W<'a, const O: u8> = crate::BitWriter<'a, u32, SINTSET_SPEC, bool, O>;
#[doc = "Field `EVENT` reader - Event interrupt enable"]
pub type EVENT_R = crate::BitReader<bool>;
#[doc = "Field `EVENT` writer - Event interrupt enable"]
pub type EVENT_W<'a, const O: u8> = crate::BitWriter<'a, u32, SINTSET_SPEC, bool, O>;
impl R {
    #[doc = "Bit 8 - Start interrupt enable"]
    #[inline(always)]
    pub fn start(&self) -> START_R {
        START_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Match interrupt enable"]
    #[inline(always)]
    pub fn matched(&self) -> MATCHED_R {
        MATCHED_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Stop interrupt enable"]
    #[inline(always)]
    pub fn stop(&self) -> STOP_R {
        STOP_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Receive interrupt enable"]
    #[inline(always)]
    pub fn rxpend(&self) -> RXPEND_R {
        RXPEND_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Transmit interrupt enable"]
    #[inline(always)]
    pub fn txsend(&self) -> TXSEND_R {
        TXSEND_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Dynamic address change interrupt enable"]
    #[inline(always)]
    pub fn dachg(&self) -> DACHG_R {
        DACHG_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Common Command Code (CCC) (that was not handled by I3C module) interrupt enable"]
    #[inline(always)]
    pub fn ccc(&self) -> CCC_R {
        CCC_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Error/warning interrupt enable"]
    #[inline(always)]
    pub fn errwarn(&self) -> ERRWARN_R {
        ERRWARN_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Double Data Rate (DDR) interrupt enable"]
    #[inline(always)]
    pub fn ddrmatched(&self) -> DDRMATCHED_R {
        DDRMATCHED_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Common Command Code (CCC) (that was handled by I3C module) interrupt enable"]
    #[inline(always)]
    pub fn chandled(&self) -> CHANDLED_R {
        CHANDLED_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Event interrupt enable"]
    #[inline(always)]
    pub fn event(&self) -> EVENT_R {
        EVENT_R::new(((self.bits >> 18) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 8 - Start interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn start(&mut self) -> START_W<8> {
        START_W::new(self)
    }
    #[doc = "Bit 9 - Match interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn matched(&mut self) -> MATCHED_W<9> {
        MATCHED_W::new(self)
    }
    #[doc = "Bit 10 - Stop interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn stop(&mut self) -> STOP_W<10> {
        STOP_W::new(self)
    }
    #[doc = "Bit 11 - Receive interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn rxpend(&mut self) -> RXPEND_W<11> {
        RXPEND_W::new(self)
    }
    #[doc = "Bit 12 - Transmit interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn txsend(&mut self) -> TXSEND_W<12> {
        TXSEND_W::new(self)
    }
    #[doc = "Bit 13 - Dynamic address change interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn dachg(&mut self) -> DACHG_W<13> {
        DACHG_W::new(self)
    }
    #[doc = "Bit 14 - Common Command Code (CCC) (that was not handled by I3C module) interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn ccc(&mut self) -> CCC_W<14> {
        CCC_W::new(self)
    }
    #[doc = "Bit 15 - Error/warning interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn errwarn(&mut self) -> ERRWARN_W<15> {
        ERRWARN_W::new(self)
    }
    #[doc = "Bit 16 - Double Data Rate (DDR) interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn ddrmatched(&mut self) -> DDRMATCHED_W<16> {
        DDRMATCHED_W::new(self)
    }
    #[doc = "Bit 17 - Common Command Code (CCC) (that was handled by I3C module) interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn chandled(&mut self) -> CHANDLED_W<17> {
        CHANDLED_W::new(self)
    }
    #[doc = "Bit 18 - Event interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn event(&mut self) -> EVENT_W<18> {
        EVENT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Slave Interrupt Set Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sintset](index.html) module"]
pub struct SINTSET_SPEC;
impl crate::RegisterSpec for SINTSET_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sintset::R](R) reader structure"]
impl crate::Readable for SINTSET_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sintset::W](W) writer structure"]
impl crate::Writable for SINTSET_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SINTSET to value 0"]
impl crate::Resettable for SINTSET_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
