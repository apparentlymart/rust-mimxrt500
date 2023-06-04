#[doc = "Register `MINTSET` reader"]
pub struct R(crate::R<MINTSET_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MINTSET_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MINTSET_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MINTSET_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MINTSET` writer"]
pub struct W(crate::W<MINTSET_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MINTSET_SPEC>;
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
impl From<crate::W<MINTSET_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MINTSET_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SLVSTART` reader - Slave start interrupt enable"]
pub type SLVSTART_R = crate::BitReader<bool>;
#[doc = "Field `SLVSTART` writer - Slave start interrupt enable"]
pub type SLVSTART_W<'a, const O: u8> = crate::BitWriter<'a, u32, MINTSET_SPEC, bool, O>;
#[doc = "Field `MCTRLDONE` reader - Master control done interrupt enable"]
pub type MCTRLDONE_R = crate::BitReader<bool>;
#[doc = "Field `MCTRLDONE` writer - Master control done interrupt enable"]
pub type MCTRLDONE_W<'a, const O: u8> = crate::BitWriter<'a, u32, MINTSET_SPEC, bool, O>;
#[doc = "Field `COMPLETE` reader - Completed message interrupt enable"]
pub type COMPLETE_R = crate::BitReader<bool>;
#[doc = "Field `COMPLETE` writer - Completed message interrupt enable"]
pub type COMPLETE_W<'a, const O: u8> = crate::BitWriter<'a, u32, MINTSET_SPEC, bool, O>;
#[doc = "Field `RXPEND` reader - RX pending interrupt enable"]
pub type RXPEND_R = crate::BitReader<bool>;
#[doc = "Field `RXPEND` writer - RX pending interrupt enable"]
pub type RXPEND_W<'a, const O: u8> = crate::BitWriter<'a, u32, MINTSET_SPEC, bool, O>;
#[doc = "Field `TXNOTFULL` reader - TX buffer/FIFO is not full interrupt enable"]
pub type TXNOTFULL_R = crate::BitReader<bool>;
#[doc = "Field `TXNOTFULL` writer - TX buffer/FIFO is not full interrupt enable"]
pub type TXNOTFULL_W<'a, const O: u8> = crate::BitWriter<'a, u32, MINTSET_SPEC, bool, O>;
#[doc = "Field `IBIWON` reader - In-Band Interrupt (IBI) won interrupt enable"]
pub type IBIWON_R = crate::BitReader<bool>;
#[doc = "Field `IBIWON` writer - In-Band Interrupt (IBI) won interrupt enable"]
pub type IBIWON_W<'a, const O: u8> = crate::BitWriter<'a, u32, MINTSET_SPEC, bool, O>;
#[doc = "Field `ERRWARN` reader - Error or warning (ERRWARN) interrupt enable"]
pub type ERRWARN_R = crate::BitReader<bool>;
#[doc = "Field `ERRWARN` writer - Error or warning (ERRWARN) interrupt enable"]
pub type ERRWARN_W<'a, const O: u8> = crate::BitWriter<'a, u32, MINTSET_SPEC, bool, O>;
#[doc = "Field `NOWMASTER` reader - Now master (now this I3C module is a master) interrupt enable"]
pub type NOWMASTER_R = crate::BitReader<bool>;
#[doc = "Field `NOWMASTER` writer - Now master (now this I3C module is a master) interrupt enable"]
pub type NOWMASTER_W<'a, const O: u8> = crate::BitWriter<'a, u32, MINTSET_SPEC, bool, O>;
impl R {
    #[doc = "Bit 8 - Slave start interrupt enable"]
    #[inline(always)]
    pub fn slvstart(&self) -> SLVSTART_R {
        SLVSTART_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Master control done interrupt enable"]
    #[inline(always)]
    pub fn mctrldone(&self) -> MCTRLDONE_R {
        MCTRLDONE_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Completed message interrupt enable"]
    #[inline(always)]
    pub fn complete(&self) -> COMPLETE_R {
        COMPLETE_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - RX pending interrupt enable"]
    #[inline(always)]
    pub fn rxpend(&self) -> RXPEND_R {
        RXPEND_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - TX buffer/FIFO is not full interrupt enable"]
    #[inline(always)]
    pub fn txnotfull(&self) -> TXNOTFULL_R {
        TXNOTFULL_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - In-Band Interrupt (IBI) won interrupt enable"]
    #[inline(always)]
    pub fn ibiwon(&self) -> IBIWON_R {
        IBIWON_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 15 - Error or warning (ERRWARN) interrupt enable"]
    #[inline(always)]
    pub fn errwarn(&self) -> ERRWARN_R {
        ERRWARN_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 19 - Now master (now this I3C module is a master) interrupt enable"]
    #[inline(always)]
    pub fn nowmaster(&self) -> NOWMASTER_R {
        NOWMASTER_R::new(((self.bits >> 19) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 8 - Slave start interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn slvstart(&mut self) -> SLVSTART_W<8> {
        SLVSTART_W::new(self)
    }
    #[doc = "Bit 9 - Master control done interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn mctrldone(&mut self) -> MCTRLDONE_W<9> {
        MCTRLDONE_W::new(self)
    }
    #[doc = "Bit 10 - Completed message interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn complete(&mut self) -> COMPLETE_W<10> {
        COMPLETE_W::new(self)
    }
    #[doc = "Bit 11 - RX pending interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn rxpend(&mut self) -> RXPEND_W<11> {
        RXPEND_W::new(self)
    }
    #[doc = "Bit 12 - TX buffer/FIFO is not full interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn txnotfull(&mut self) -> TXNOTFULL_W<12> {
        TXNOTFULL_W::new(self)
    }
    #[doc = "Bit 13 - In-Band Interrupt (IBI) won interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn ibiwon(&mut self) -> IBIWON_W<13> {
        IBIWON_W::new(self)
    }
    #[doc = "Bit 15 - Error or warning (ERRWARN) interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn errwarn(&mut self) -> ERRWARN_W<15> {
        ERRWARN_W::new(self)
    }
    #[doc = "Bit 19 - Now master (now this I3C module is a master) interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn nowmaster(&mut self) -> NOWMASTER_W<19> {
        NOWMASTER_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Master Interrupt Set Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mintset](index.html) module"]
pub struct MINTSET_SPEC;
impl crate::RegisterSpec for MINTSET_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mintset::R](R) reader structure"]
impl crate::Readable for MINTSET_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mintset::W](W) writer structure"]
impl crate::Writable for MINTSET_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MINTSET to value 0"]
impl crate::Resettable for MINTSET_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
