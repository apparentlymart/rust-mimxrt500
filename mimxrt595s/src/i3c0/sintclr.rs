#[doc = "Register `SINTCLR` reader"]
pub struct R(crate::R<SINTCLR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SINTCLR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SINTCLR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SINTCLR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SINTCLR` writer"]
pub struct W(crate::W<SINTCLR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SINTCLR_SPEC>;
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
impl From<crate::W<SINTCLR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SINTCLR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `START` reader - START interrupt enable clear"]
pub type START_R = crate::BitReader<bool>;
#[doc = "Field `START` writer - START interrupt enable clear"]
pub type START_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, SINTCLR_SPEC, bool, O>;
#[doc = "Field `MATCHED` reader - MATCHED interrupt enable clear"]
pub type MATCHED_R = crate::BitReader<bool>;
#[doc = "Field `MATCHED` writer - MATCHED interrupt enable clear"]
pub type MATCHED_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, SINTCLR_SPEC, bool, O>;
#[doc = "Field `STOP` reader - STOP interrupt enable clear"]
pub type STOP_R = crate::BitReader<bool>;
#[doc = "Field `STOP` writer - STOP interrupt enable clear"]
pub type STOP_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, SINTCLR_SPEC, bool, O>;
#[doc = "Field `RXPEND` reader - RXPEND interrupt enable clear"]
pub type RXPEND_R = crate::BitReader<bool>;
#[doc = "Field `RXPEND` writer - RXPEND interrupt enable clear"]
pub type RXPEND_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, SINTCLR_SPEC, bool, O>;
#[doc = "Field `TXSEND` reader - TXSEND interrupt enable clear"]
pub type TXSEND_R = crate::BitReader<bool>;
#[doc = "Field `TXSEND` writer - TXSEND interrupt enable clear"]
pub type TXSEND_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, SINTCLR_SPEC, bool, O>;
#[doc = "Field `DACHG` reader - DACHG interrupt enable clear"]
pub type DACHG_R = crate::BitReader<bool>;
#[doc = "Field `DACHG` writer - DACHG interrupt enable clear"]
pub type DACHG_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, SINTCLR_SPEC, bool, O>;
#[doc = "Field `CCC` reader - CCC interrupt enable clear"]
pub type CCC_R = crate::BitReader<bool>;
#[doc = "Field `CCC` writer - CCC interrupt enable clear"]
pub type CCC_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, SINTCLR_SPEC, bool, O>;
#[doc = "Field `ERRWARN` reader - ERRWARN interrupt enable clear"]
pub type ERRWARN_R = crate::BitReader<bool>;
#[doc = "Field `ERRWARN` writer - ERRWARN interrupt enable clear"]
pub type ERRWARN_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, SINTCLR_SPEC, bool, O>;
#[doc = "Field `DDRMATCHED` reader - DDRMATCHED interrupt enable clear"]
pub type DDRMATCHED_R = crate::BitReader<bool>;
#[doc = "Field `DDRMATCHED` writer - DDRMATCHED interrupt enable clear"]
pub type DDRMATCHED_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, SINTCLR_SPEC, bool, O>;
#[doc = "Field `CHANDLED` reader - CHANDLED interrupt enable clear"]
pub type CHANDLED_R = crate::BitReader<bool>;
#[doc = "Field `CHANDLED` writer - CHANDLED interrupt enable clear"]
pub type CHANDLED_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, SINTCLR_SPEC, bool, O>;
#[doc = "Field `EVENT` reader - EVENT interrupt enable clear"]
pub type EVENT_R = crate::BitReader<bool>;
#[doc = "Field `EVENT` writer - EVENT interrupt enable clear"]
pub type EVENT_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, SINTCLR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 8 - START interrupt enable clear"]
    #[inline(always)]
    pub fn start(&self) -> START_R {
        START_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - MATCHED interrupt enable clear"]
    #[inline(always)]
    pub fn matched(&self) -> MATCHED_R {
        MATCHED_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - STOP interrupt enable clear"]
    #[inline(always)]
    pub fn stop(&self) -> STOP_R {
        STOP_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - RXPEND interrupt enable clear"]
    #[inline(always)]
    pub fn rxpend(&self) -> RXPEND_R {
        RXPEND_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - TXSEND interrupt enable clear"]
    #[inline(always)]
    pub fn txsend(&self) -> TXSEND_R {
        TXSEND_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - DACHG interrupt enable clear"]
    #[inline(always)]
    pub fn dachg(&self) -> DACHG_R {
        DACHG_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - CCC interrupt enable clear"]
    #[inline(always)]
    pub fn ccc(&self) -> CCC_R {
        CCC_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - ERRWARN interrupt enable clear"]
    #[inline(always)]
    pub fn errwarn(&self) -> ERRWARN_R {
        ERRWARN_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - DDRMATCHED interrupt enable clear"]
    #[inline(always)]
    pub fn ddrmatched(&self) -> DDRMATCHED_R {
        DDRMATCHED_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - CHANDLED interrupt enable clear"]
    #[inline(always)]
    pub fn chandled(&self) -> CHANDLED_R {
        CHANDLED_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - EVENT interrupt enable clear"]
    #[inline(always)]
    pub fn event(&self) -> EVENT_R {
        EVENT_R::new(((self.bits >> 18) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 8 - START interrupt enable clear"]
    #[inline(always)]
    #[must_use]
    pub fn start(&mut self) -> START_W<8> {
        START_W::new(self)
    }
    #[doc = "Bit 9 - MATCHED interrupt enable clear"]
    #[inline(always)]
    #[must_use]
    pub fn matched(&mut self) -> MATCHED_W<9> {
        MATCHED_W::new(self)
    }
    #[doc = "Bit 10 - STOP interrupt enable clear"]
    #[inline(always)]
    #[must_use]
    pub fn stop(&mut self) -> STOP_W<10> {
        STOP_W::new(self)
    }
    #[doc = "Bit 11 - RXPEND interrupt enable clear"]
    #[inline(always)]
    #[must_use]
    pub fn rxpend(&mut self) -> RXPEND_W<11> {
        RXPEND_W::new(self)
    }
    #[doc = "Bit 12 - TXSEND interrupt enable clear"]
    #[inline(always)]
    #[must_use]
    pub fn txsend(&mut self) -> TXSEND_W<12> {
        TXSEND_W::new(self)
    }
    #[doc = "Bit 13 - DACHG interrupt enable clear"]
    #[inline(always)]
    #[must_use]
    pub fn dachg(&mut self) -> DACHG_W<13> {
        DACHG_W::new(self)
    }
    #[doc = "Bit 14 - CCC interrupt enable clear"]
    #[inline(always)]
    #[must_use]
    pub fn ccc(&mut self) -> CCC_W<14> {
        CCC_W::new(self)
    }
    #[doc = "Bit 15 - ERRWARN interrupt enable clear"]
    #[inline(always)]
    #[must_use]
    pub fn errwarn(&mut self) -> ERRWARN_W<15> {
        ERRWARN_W::new(self)
    }
    #[doc = "Bit 16 - DDRMATCHED interrupt enable clear"]
    #[inline(always)]
    #[must_use]
    pub fn ddrmatched(&mut self) -> DDRMATCHED_W<16> {
        DDRMATCHED_W::new(self)
    }
    #[doc = "Bit 17 - CHANDLED interrupt enable clear"]
    #[inline(always)]
    #[must_use]
    pub fn chandled(&mut self) -> CHANDLED_W<17> {
        CHANDLED_W::new(self)
    }
    #[doc = "Bit 18 - EVENT interrupt enable clear"]
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
#[doc = "Slave Interrupt Clear Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sintclr](index.html) module"]
pub struct SINTCLR_SPEC;
impl crate::RegisterSpec for SINTCLR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sintclr::R](R) reader structure"]
impl crate::Readable for SINTCLR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sintclr::W](W) writer structure"]
impl crate::Writable for SINTCLR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0x0007_ff00;
}
#[doc = "`reset()` method sets SINTCLR to value 0"]
impl crate::Resettable for SINTCLR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
